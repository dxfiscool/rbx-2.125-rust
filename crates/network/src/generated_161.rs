//! Auto-generated skeletons for rbx-network — global EA-sorted filler (RakNet|Network|Replicat|Socket filtered exhausted)
//! Filter: RakNet|Network|Replicat|Socket -> 5198 funcs (cs), 5282 (ci), 0 remaining before batch; filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x41cf0..0x4642c | existing 17909 -> 18009 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

/// `signal<void(bool, void*, UIEvent)>` slot connection (IDA 0x4546c et al.).
#[derive(Clone, Debug, Default)]
pub struct UiEventSlot {
 pub id: u64,
 pub target: usize,
 pub live: bool,
}

/// `std::deque<function<void()>*>` queue (IDA 0x44564 et al.).
#[derive(Clone, Debug, Default)]
pub struct FunctorQueue {
 pub items: Vec<usize>,
}

/// Static-init state for `__GLOBAL__I_a_14` (IDA 0x44924).
#[derive(Clone, Debug, Default)]
pub struct GlobalInitA14 {
 pub done: bool,
}

/// CameraControl touch state (IDA 0x44d04 et al.).
#[derive(Clone, Debug, Default)]
pub struct CameraTouch {
 pub has_touch: bool,
 pub begin: (f32, f32),
 pub has_rotated: bool,
}

/// `iOSSettingsService` settings + handler map (IDA 0x43180 et al.).
#[derive(Clone, Debug, Default)]
pub struct SettingsService {
 pub values: HashMap<String, String>,
 pub search_url: String,
 pub api_url: String,
}

/// `FunctionMarshaller` window registry (IDA 0x43c78 et al.: map<uint, marshaller*>).
#[derive(Clone, Debug, Default)]
pub struct MarshallerRegistry {
 pub entries: HashMap<u32, usize>,
}

/// Static-init state for `__GLOBAL__I_a_13` (IDA 0x43394).
#[derive(Clone, Debug, Default)]
pub struct GlobalInitA13 {
 pub done: bool,
}
use std::collections::HashMap;

/// Google Analytics tracker state (IDA 0x41f74 et al.).
#[derive(Clone, Debug, Default)]
pub struct GaTracker {
 pub initialized: bool,
 pub queued_views: Vec<String>,
 pub queued_events: Vec<(String, String, String, i32)>,
 pub queued_vars: Vec<(String, String)>,
}

/// `RobloxWebUtility` cached settings (IDA 0x427c0 et al.).
#[derive(Clone, Debug, Default)]
pub struct WebUtility {
 pub shared: bool,
 pub cached_settings: Option<usize>,
 pub last_request_time: f64,
}

/// Static-init state for `__GLOBAL__I_a_12` (IDA 0x42580).
#[derive(Clone, Debug, Default)]
pub struct GlobalInitA12 {
 pub done: bool,
}

// 0x41cf0 — ___35+[RobloxGoogleAnalytics initialize]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___35+[RobloxGoogleAnalytics initialize]_block_invoke")]
pub fn stub_41cf0(setup: &mut dyn FnMut()) {
    // IDA 0x41cf0: analytics init block — GAI tracker setup from settings (below truncation).
    setup();
}

// 0x41f28 — +[RobloxGoogleAnalytics release]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[RobloxGoogleAnalytics release]")]
pub fn stub_41f28() {
    // IDA 0x41f28: empty release body.
}

// 0x41f2c — +[RobloxGoogleAnalytics callBackPageTracking:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxGoogleAnalytics callBackPageTracking:]")]
pub fn stub_41f2c(url: Option<&str>, track: &mut dyn FnMut(&str)) {
    // IDA 0x41f2c: page-tracking callback forwards dict url.
    if let Some(u) = url {
        track(u);
    }
}

// 0x41f74 — +[RobloxGoogleAnalytics setPageViewTracking:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxGoogleAnalytics setPageViewTracking:]")]
pub fn stub_41f74(state: &mut GaTracker, url: String, send: &mut dyn FnMut(&str)) {
    // IDA 0x41f74: initialized ? sendView : queue for later.
    if state.initialized {
        send(&url);
    } else {
        state.queued_views.push(url);
    }
}

// 0x4203c — +[RobloxGoogleAnalytics callBackEventTracking:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxGoogleAnalytics callBackEventTracking:]")]
pub fn stub_4203c(category: &str, action: &str, label: &str, value: i32, track: &mut dyn FnMut(&str, &str, &str, i32)) {
    // IDA 0x4203c: event-tracking callback unpacks dict fields.
    track(category, action, label, value);
}

// 0x420e4 — +[RobloxGoogleAnalytics setEventTracking:withAction:withLabel:withValue:]
// type: void __cdecl(id, SEL, id, id, id, int)
#[doc(alias = "+[RobloxGoogleAnalytics setEventTracking:withAction:withLabel:withValue:]")]
pub fn stub_420e4(state: &mut GaTracker, category: String, action: String, label: String, value: i32, send: &mut dyn FnMut(&str, &str, &str, i32)) {
    // IDA 0x420e4: initialized ? sendEvent : queue for later.
    if state.initialized {
        send(&category, &action, &label, value);
    } else {
        state.queued_events.push((category, action, label, value));
    }
}

// 0x42230 — +[RobloxGoogleAnalytics callbackCustomVariableTracking:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxGoogleAnalytics callbackCustomVariableTracking:]")]
pub fn stub_42230(label: &str, value: &str, set: &mut dyn FnMut(&str, &str)) {
    // IDA 0x42230: custom-variable callback unpacks label/value.
    set(label, value);
}

// 0x42298 — +[RobloxGoogleAnalytics setCustomVariableWithLabel:withValue:]
// type: void __cdecl(id, SEL, id, id)
#[doc(alias = "+[RobloxGoogleAnalytics setCustomVariableWithLabel:withValue:]")]
pub fn stub_42298(state: &mut GaTracker, label: String, value: String, send: &mut dyn FnMut(&str, &str)) {
    // IDA 0x42298: initialized ? set:value : queue for later.
    if state.initialized {
        send(&label, &value);
    } else {
        state.queued_vars.push((label, value));
    }
}

// 0x42374 — +[RobloxGoogleAnalytics debugCountersPrint]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[RobloxGoogleAnalytics debugCountersPrint]")]
pub fn stub_42374(print: &mut dyn FnMut()) {
    // IDA 0x42374: debugCountersPrint — read debug_* defaults + log (below truncation).
    print();
}

// 0x424cc — +[RobloxGoogleAnalytics debugCounterIncrement:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxGoogleAnalytics debugCounterIncrement:]")]
pub fn stub_424cc(counters: &mut HashMap<String, i64>, name: &str) {
    // IDA 0x424cc: debug_<name> counter +1 in user defaults.
    *counters.entry(format!("debug_{}", name)).or_insert(0) += 1;
}

// 0x42580 — __GLOBAL__I_a_12
// demangled: global constructor keyed to_a_12
#[doc(alias = "global constructor keyed to_a_12")]
pub fn stub_42580(state: &mut GlobalInitA12, init: &mut dyn FnMut()) {
    // IDA 0x42580: boost error categories + ios_base::Init + bad_alloc static exception object.
    if !state.done {
        init();
        state.done = true;
    }
}

// 0x42718 — +[RobloxWebUtility sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxWebUtility sharedInstance]")]
pub fn stub_42718(slot: &mut Option<usize>, alloc: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x42718: dispatch_once sharedInstance.
    if let Some(v) = *slot {
        return v;
    }
    let v = alloc();
    *slot = Some(v);
    v
}

// 0x42774 — ___34+[RobloxWebUtility sharedInstance]_block_invoke
#[doc(alias = "___34+[RobloxWebUtility sharedInstance]_block_invoke")]
pub fn stub_42774(alloc: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x42774: sharedInstance block — alloc + init.
    alloc()
}

// 0x427a8 — ___copy_helper_block__7
#[doc(alias = "___copy_helper_block__7")]
pub fn stub_427a8(dst20: &mut usize, src20: usize, retain: &mut dyn FnMut(usize) -> usize) {
    // IDA 0x427a8: _Block_object_assign(dst+20, src+20, 3).
    *dst20 = retain(src20);
}

// 0x427b4 — ___destroy_helper_block__7
#[doc(alias = "___destroy_helper_block__7")]
pub fn stub_427b4(slot20: &mut usize, release: &mut dyn FnMut(usize)) {
    // IDA 0x427b4: _Block_object_dispose(slot+20, 3).
    release(*slot20);
}

// 0x427c0 — -[RobloxWebUtility init]
// type: RobloxWebUtility *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility init]")]
pub fn stub_427c0(ok: bool, util: &mut WebUtility) -> bool {
    // IDA 0x427c0: super init; epoch lastSettingsRequestTime; create queues (below truncation).
    if !ok {
        return false;
    }
    util.last_request_time = 0.0;
    true
}

// 0x42880 — -[RobloxWebUtility dealloc]
// type: void __cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility dealloc]")]
pub fn stub_42880(util: &mut WebUtility, teardown: &mut dyn FnMut()) {
    // IDA 0x42880: release cached objects + queues; super dealloc (below truncation).
    util.cached_settings = None;
    util.last_request_time = 0.0;
    teardown();
}

// 0x4290c — -[RobloxWebUtility getiOSLogQueue]
// type: dispatch_queue_s *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getiOSLogQueue]")]
pub fn stub_4290c(queue: usize) -> usize {
    // IDA 0x4290c: return iOSLogQueue.
    queue
}

// 0x4291c — -[RobloxWebUtility getiOSSettingsQueue]
// type: dispatch_queue_s *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getiOSSettingsQueue]")]
pub fn stub_4291c(queue: usize) -> usize {
    // IDA 0x4291c: return iOSSettingsQueue.
    queue
}

// 0x4292c — -[RobloxWebUtility setCachediOSSettings:]
// type: void __cdecl(RobloxWebUtility *self, SEL, iOSSettingsService *)
#[doc(alias = "-[RobloxWebUtility setCachediOSSettings:]")]
pub fn stub_4292c(util: &mut WebUtility, settings: usize) {
    // IDA 0x4292c: store cachediOSSettings.
    util.cached_settings = Some(settings);
}

// 0x4293c — -[RobloxWebUtility getCachediOSSettings]
// type: iOSSettingsService *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getCachediOSSettings]")]
pub fn stub_4293c(util: &WebUtility) -> Option<usize> {
    // IDA 0x4293c: return cachediOSSettings.
    util.cached_settings
}

// 0x4294c — -[RobloxWebUtility getLastSettingsRequestTime]
// type: id __cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getLastSettingsRequestTime]")]
pub fn stub_4294c(util: &WebUtility) -> f64 {
    // IDA 0x4294c: return lastSettingsRequestTime.
    util.last_request_time
}

// 0x4295c — -[RobloxWebUtility getiOSSettingsServiceFromWeb]
// type: iOSSettingsService *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getiOSSettingsServiceFromWeb]")]
pub fn stub_4295c(util: &mut WebUtility, alloc: &mut dyn FnMut(usize) -> usize, fetch: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x4295c: new iOSSettingsService; fetch client settings; cache (below truncation).
    let s = alloc(0xB4);
    fetch(s);
    util.cached_settings = Some(s);
    s
}

// 0x42a98 — +[RobloxWebUtility getiOSSettingsServiceWithForcedReadFromWeb:]
// type: iOSSettingsService *__cdecl(id, SEL, char)
#[doc(alias = "+[RobloxWebUtility getiOSSettingsServiceWithForcedReadFromWeb:]")]
pub fn stub_42a98(util: &WebUtility, forced: bool, fetch: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x42a98: forced ? fetch from web : cached (below truncation).
    if forced {
        fetch()
    } else {
        util.cached_settings.unwrap_or(0)
    }
}

// 0x42bc8 — ___63+[RobloxWebUtility getiOSSettingsServiceWithForcedReadFromWeb:]_block_invoke
// type: iOSSettingsService *__fastcall(int)
#[doc(alias = "___63+[RobloxWebUtility getiOSSettingsServiceWithForcedReadFromWeb:]_block_invoke")]
pub fn stub_42bc8(assemble: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x42bc8: settings-service block — date + cached flags assembly (below truncation).
    assemble()
}

// 0x42dd8 — ___copy_helper_block_65
#[doc(alias = "___copy_helper_block_65")]
pub fn stub_42dd8(dst20: &mut usize, src20: usize, retain: &mut dyn FnMut(usize) -> usize) {
    // IDA 0x42dd8: _Block_object_assign(dst+20, src+20, 8).
    *dst20 = retain(src20);
}

// 0x42de4 — ___destroy_helper_block_66
#[doc(alias = "___destroy_helper_block_66")]
pub fn stub_42de4(slot20: &mut usize, release: &mut dyn FnMut(usize)) {
    // IDA 0x42de4: _Block_object_dispose(slot+20, 8).
    release(*slot20);
}

// 0x42dec — +[RobloxWebUtility getUrlForButtonTag:recordPageView:query:]
// type: id __cdecl(id, SEL, int, char, id)
#[doc(alias = "+[RobloxWebUtility getUrlForButtonTag:recordPageView:query:]")]
pub fn stub_42dec(tag: i32, record: bool, query: &str, build: &mut dyn FnMut(i32, bool, &str) -> String) -> String {
    // IDA 0x42dec: button-tag URL builder (below truncation).
    build(tag, record, query)
}

// 0x43180 — __ZN18iOSSettingsServiceC2Ev
// demangled: iOSSettingsService::iOSSettingsService(void)
// type: iOSSettingsService *__fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::iOSSettingsService(void)")]
pub fn stub_43180(svc: &mut SettingsService) {
    // IDA 0x43180: iOSSettingsService ctor — empty map + empty strings.
    svc.values.clear();
    svc.search_url.clear();
    svc.api_url.clear();
}

// 0x432b0 — __ZN18iOSSettingsServiceD1Ev
// demangled: iOSSettingsService::~iOSSettingsService()
// type: void __fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::~iOSSettingsService()")]
pub fn stub_432b0(destroy: &mut dyn FnMut()) {
    // IDA 0x432b0: D1 thunk tail-calls D2.
    destroy();
}

// 0x432b4 — __ZN18iOSSettingsServiceD0Ev
// demangled: iOSSettingsService::~iOSSettingsService()
// type: void __fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::~iOSSettingsService()")]
pub fn stub_432b4(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x432b4: D0: body + operator delete.
    destroy();
    free();
}

// 0x432c8 — __ZN18iOSSettingsServiceD2Ev
// demangled: iOSSettingsService::~iOSSettingsService()
// type: void __fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::~iOSSettingsService()")]
pub fn stub_432c8(destroy: &mut dyn FnMut()) {
    // IDA 0x432c8: D2 — vtable + string/map destroys (below truncation).
    destroy();
}

// 0x43314 — __ZN10SimpleJSOND1Ev
// demangled: SimpleJSON::~SimpleJSON()
// type: void __fastcall(SimpleJSON *__hidden this)
#[doc(alias = "SimpleJSON::~SimpleJSON()")]
pub fn stub_43314(map: &mut HashMap<String, usize>) {
    // IDA 0x43314: SimpleJSON D2 — erase handler map.
    map.clear();
}

// 0x43338 — __ZN10SimpleJSOND0Ev
// demangled: SimpleJSON::~SimpleJSON()
// type: void __fastcall(SimpleJSON *__hidden this)
#[doc(alias = "SimpleJSON::~SimpleJSON()")]
pub fn stub_43338(map: &mut HashMap<String, usize>, free: &mut dyn FnMut()) {
    // IDA 0x43338: SimpleJSON D0 — erase map + delete.
    map.clear();
    free();
}

// 0x43360 — __ZN10SimpleJSON14DefaultHandlerERKSsS1_
// demangled: SimpleJSON::DefaultHandler(std::string const&,std::string const&)
#[doc(alias = "SimpleJSON::DefaultHandler(std::string const&,std::string const&)")]
pub fn stub_43360() -> i32 {
    // IDA 0x43360: DefaultHandler returns 0.
    0
}

// 0x43364 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,void (*)(char const*)>> *)
// type: int(void)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,void (*)(char const*)>> *)")]
pub fn stub_43364(map: &mut HashMap<String, usize>, keys: &[String]) {
    // IDA 0x43364: _Rb_tree range erase.
    for k in keys {
        map.remove(k);
    }
}

// 0x43394 — __GLOBAL__I_a_13
// demangled: global constructor keyed to_a_13
#[doc(alias = "global constructor keyed to_a_13")]
pub fn stub_43394(state: &mut GlobalInitA13, init: &mut dyn FnMut()) {
    // IDA 0x43394: boost error categories + ios_base::Init + bad_alloc static exception object.
    if !state.done {
        init();
        state.done = true;
    }
}

// 0x4352c — __ZN3RBX18FunctionMarshallerC2Ej
// demangled: RBX::FunctionMarshaller::FunctionMarshaller(unsigned int)
// type: int __fastcall(RBX::FunctionMarshaller *this, int, int, int)
#[doc(alias = "RBX::FunctionMarshaller::FunctionMarshaller(unsigned int)")]
pub fn stub_4352c(job: usize, init: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x4352c: FunctionMarshaller::FunctionMarshaller (below truncation).
    init(job);
    job
}

// 0x43624 — __ZN3RBX18FunctionMarshaller9GetWindowEv
// demangled: RBX::FunctionMarshaller::GetWindow(void)
// type: int __fastcall(RBX::FunctionMarshaller *this, int, int, int)
#[doc(alias = "RBX::FunctionMarshaller::GetWindow(void)")]
pub fn stub_43624(reg: &MarshallerRegistry, key: u32) -> Option<usize> {
    // IDA 0x43624: FunctionMarshaller::GetWindow — registry lookup (below truncation).
    reg.entries.get(&key).copied()
}

// 0x43804 — __ZN3RBX18FunctionMarshaller13ReleaseWindowEPS0_
// demangled: RBX::FunctionMarshaller::ReleaseWindow(RBX::FunctionMarshaller*)
// type: void __fastcall(RBX::FunctionMarshaller *this, RBX::FunctionMarshaller *, int, int)
#[doc(alias = "RBX::FunctionMarshaller::ReleaseWindow(RBX::FunctionMarshaller*)")]
pub fn stub_43804(reg: &mut MarshallerRegistry, key: u32, release: &mut dyn FnMut(usize)) {
    // IDA 0x43804: FunctionMarshaller::ReleaseWindow — erase + release (below truncation).
    if let Some(m) = reg.entries.remove(&key) {
        release(m);
    }
}

// 0x43930 — __ZN3RBX18FunctionMarshaller14handleAppEventEPv
// demangled: RBX::FunctionMarshaller::handleAppEvent(void *)
// type: void __fastcall(RBX::FunctionMarshaller *this, void *)
#[doc(alias = "RBX::FunctionMarshaller::handleAppEvent(void *)")]
pub fn stub_43930(run: &mut dyn FnMut(), clear: &mut dyn FnMut(), free: &mut dyn FnMut(), signal: &mut dyn FnMut()) {
    // IDA 0x43930: run functor; clear + delete it; delete this; signal event.
    run();
    clear();
    free();
    signal();
}

// 0x43a98 — __ZN3RBX18FunctionMarshaller7ExecuteEN5boost8functionIFvvEEEPNS_6CEventE
// demangled: RBX::FunctionMarshaller::Execute(boost::function<void ()(void)>,RBX::CEvent *)
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::FunctionMarshaller::Execute(boost::function<void ()(void)>,RBX::CEvent *)")]
pub fn stub_43a98(same_thread: bool, run: &mut dyn FnMut(), post: &mut dyn FnMut()) {
    // IDA 0x43a98: same thread ? run directly : post app event.
    if same_thread {
        run();
    } else {
        post();
    }
}

// 0x43b98 — __ZN3RBX18FunctionMarshaller6SubmitEN5boost8functionIFvvEEE
// demangled: RBX::FunctionMarshaller::Submit(boost::function<void ()(void)>)
// type: void __fastcall(int, int)
#[doc(alias = "RBX::FunctionMarshaller::Submit(boost::function<void ()(void)>)")]
pub fn stub_43b98(post: &mut dyn FnMut()) {
    // IDA 0x43b98: Submit — wrap functor + postAppEvent (below truncation).
    post();
}

// 0x43c70 — __ZN3RBX18FunctionMarshaller15ProcessMessagesEv
// demangled: RBX::FunctionMarshaller::ProcessMessages(void)
// type: CFRunLoopRunResult __fastcall(Roblox *this)
#[doc(alias = "RBX::FunctionMarshaller::ProcessMessages(void)")]
pub fn stub_43c70(process: &mut dyn FnMut() -> i32) -> i32 {
    // IDA 0x43c70: thunk tail-calls processAppEvents.
    process()
}

// 0x43c74 — __ZN3RBX18FunctionMarshaller10StaticDataD1Ev
// demangled: RBX::FunctionMarshaller::StaticData::~StaticData()
// type: void __fastcall(RBX::FunctionMarshaller::StaticData *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::StaticData::~StaticData()")]
pub fn stub_43c74(destroy: &mut dyn FnMut()) {
    // IDA 0x43c74: StaticData D1 thunk tail-calls D2.
    destroy();
}

// 0x43c78 — __ZN3RBX18FunctionMarshaller10StaticDataD2Ev
// demangled: RBX::FunctionMarshaller::StaticData::~StaticData()
// type: void __fastcall(RBX::FunctionMarshaller::StaticData *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::StaticData::~StaticData()")]
pub fn stub_43c78(reg: &mut MarshallerRegistry, teardown: &mut dyn FnMut()) {
    // IDA 0x43c78: StaticData D2 — mutex destroy + registry erase.
    reg.entries.clear();
    teardown();
}

// 0x43d14 — __ZNSt3mapIjPN3RBX18FunctionMarshallerESt4lessIjESaISt4pairIKjS2_EEEixERS6_
// demangled: std::map<unsigned int,RBX::FunctionMarshaller *,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::operator[](unsigned int const&)
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<unsigned int,RBX::FunctionMarshaller *,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::operator[](unsigned int const&)")]
pub fn stub_43d14(reg: &mut MarshallerRegistry, key: u32) -> usize {
    // IDA 0x43d14: map operator[] — find or default-insert.
    *reg.entries.entry(key).or_insert(0)
}

// 0x43d6c — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseERS1_
// demangled: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(unsigned int const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(unsigned int const&)")]
pub fn stub_43d6c(reg: &mut MarshallerRegistry, key: u32) -> bool {
    // IDA 0x43d6c: erase by key.
    reg.entries.remove(&key).is_some()
}

// 0x43d94 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE11equal_rangeERS1_
// demangled: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::equal_range(unsigned int const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::equal_range(unsigned int const&)")]
pub fn stub_43d94(reg: &MarshallerRegistry, key: u32) -> bool {
    // IDA 0x43d94: equal_range — key present.
    reg.entries.contains_key(&key)
}

// 0x43de0 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// demangled: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>)")]
pub fn stub_43de0(reg: &mut MarshallerRegistry, keys: &[u32]) {
    // IDA 0x43de0: _Rb_tree range erase.
    for k in keys {
        reg.entries.remove(k);
    }
}

// 0x43e40 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// demangled: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,RBX::FunctionMarshaller *>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,RBX::FunctionMarshaller *>> *)")]
pub fn stub_43e40(reg: &mut MarshallerRegistry, keys: &[u32]) {
    // IDA 0x43e40: _Rb_tree node erase.
    for k in keys {
        reg.entries.remove(k);
    }
}

// 0x43e68 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// demangled: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
pub fn stub_43e68(reg: &mut MarshallerRegistry, key: u32, value: usize) -> bool {
    // IDA 0x43e68: hinted unique insert; false when present.
    if reg.entries.contains_key(&key) {
        return false;
    }
    reg.entries.insert(key, value);
    true
}

// 0x43f1c — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// demangled: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
pub fn stub_43f1c(reg: &mut MarshallerRegistry, key: u32, value: usize) {
    // IDA 0x43f1c: hinted insert.
    reg.entries.insert(key, value);
}

// 0x43f74 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_
// demangled: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
pub fn stub_43f74(reg: &mut MarshallerRegistry, key: u32, value: usize) -> bool {
    // IDA 0x43f74: unique insert; false when present.
    if reg.entries.contains_key(&key) {
        return false;
    }
    reg.entries.insert(key, value);
    true
}

// 0x43fdc — __ZN5boost11unique_lockINS_15recursive_mutexEE4lockEv
// demangled: boost::unique_lock<boost::recursive_mutex>::lock(void)
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::unique_lock<boost::recursive_mutex>::lock(void)")]
pub fn stub_43fdc(lock: &mut dyn FnMut()) {
    // IDA 0x43fdc: unique_lock::lock (below truncation).
    lock();
}

// 0x441a8 — __ZN3RBX18FunctionMarshaller27safe_static_init_staticDataEv
// demangled: RBX::FunctionMarshaller::safe_static_init_staticData(void)
// type: _DWORD __fastcall(RBX::FunctionMarshaller *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::safe_static_init_staticData(void)")]
pub fn stub_441a8(get: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x441a8: thunk tail-calls safe_static_do_get_staticData.
    get()
}

// 0x441ac — __ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv
// demangled: RBX::FunctionMarshaller::safe_static_do_get_staticData(void)
// type: void *__fastcall(RBX::FunctionMarshaller *this)
#[doc(alias = "RBX::FunctionMarshaller::safe_static_do_get_staticData(void)")]
pub fn stub_441ac(guard: &mut bool, init: &mut dyn FnMut()) {
    // IDA 0x441ac: guarded one-time StaticData init.
    if !*guard {
        init();
        *guard = true;
    }
}

// 0x442bc — __ZN5boost15recursive_mutexC2Ev
// demangled: boost::recursive_mutex::recursive_mutex(void)
// type: _DWORD __fastcall(boost::recursive_mutex *__hidden this)
#[doc(alias = "boost::recursive_mutex::recursive_mutex(void)")]
pub fn stub_442bc(init: &mut dyn FnMut()) {
    // IDA 0x442bc: recursive_mutex ctor (below truncation).
    init();
}

// 0x44564 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EED2Ev
// demangled: std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::~_Deque_base()
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::~_Deque_base()")]
pub fn stub_44564(q: &mut FunctorQueue, free: &mut dyn FnMut(usize)) {
    // IDA 0x44564: _Deque_base dtor — delete nodes.
    for item in q.items.drain(..) {
        free(item);
    }
}

// 0x44590 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE17_M_initialize_mapEm
// demangled: std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_initialize_map(unsigned long)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_initialize_map(unsigned long)")]
pub fn stub_44590(q: &mut FunctorQueue, cap: usize) {
    // IDA 0x44590: deque map initialize — reserve.
    q.items.reserve(cap);
}

// 0x446e8 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_allocate_mapEm
// demangled: std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_allocate_map(unsigned long)
// type: int(void)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_allocate_map(unsigned long)")]
pub fn stub_446e8(count: usize, alloc: &mut dyn FnMut(usize) -> usize) -> usize {
    // IDA 0x446e8: throw bad_alloc when huge else operator new.
    if count >= 0x40000000 {
        panic!("std::bad_alloc");
    }
    alloc(count)
}

// 0x44700 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_create_nodesEPPS4_S8_
// demangled: std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_create_nodes(boost::function<void ()(void)> ***,boost::function<void ()(void)> ***)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_create_nodes(boost::function<void ()(void)> ***,boost::function<void ()(void)> ***)")]
pub fn stub_44700(q: &mut FunctorQueue, count: usize, alloc: &mut dyn FnMut() -> usize) {
    // IDA 0x44700: create nodes (0x200 each).
    for _ in 0..count {
        q.items.push(alloc());
    }
}

// 0x447f4 — __ZNSt5dequeIPN5boost8functionIFvvEEESaIS4_EEC2ERKS6_
// demangled: std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::deque(std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>> const&)
// type: int __fastcall(int)
#[doc(alias = "std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::deque(std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>> const&)")]
pub fn stub_447f4(dst: &mut FunctorQueue, src: &[usize]) {
    // IDA 0x447f4: deque range construct.
    dst.items.extend_from_slice(src);
}

// 0x44888 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN5boost8functionIFvvEEERKS8_PS9_ES3_IS8_RS8_PS8_EEET0_T_SH_SG_
// demangled: std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>>(std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>)
#[doc(alias = "std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>>(std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>)")]
pub fn stub_44888(dst: &mut Vec<usize>, src: &[usize]) {
    // IDA 0x44888: deque copy.
    dst.extend_from_slice(src);
}

// 0x44924 — __GLOBAL__I_a_14
// demangled: global constructor keyed to_a_14
#[doc(alias = "global constructor keyed to_a_14")]
pub fn stub_44924(state: &mut GlobalInitA14, init: &mut dyn FnMut()) {
    // IDA 0x44924: boost error categories + ios_base::Init + bad_alloc static exception object.
    if !state.done {
        init();
        state.done = true;
    }
}

// 0x44abc — -[CameraControl init:delegate:]
// type: id __cdecl(CameraControl *self, SEL, CGRect, id)
#[doc(alias = "-[CameraControl init:delegate:]")]
pub fn stub_44abc(ok: bool, enable_touch: &mut dyn FnMut()) -> bool {
    // IDA 0x44abc: super init; set frame; multitouch on (below truncation).
    if !ok {
        return false;
    }
    enable_touch();
    true
}

// 0x44b90 — -[CameraControl dealloc]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl dealloc]")]
pub fn stub_44b90(teardown: &mut dyn FnMut()) {
    // IDA 0x44b90: super dealloc.
    teardown();
}

// 0x44bbc — -[CameraControl setupPostMouseEventConnection]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl setupPostMouseEventConnection]")]
pub fn stub_44bbc(connect: &mut dyn FnMut()) {
    // IDA 0x44bbc: setup post-mouse-event connection (below truncation).
    connect();
}

// 0x44cd4 — -[CameraControl postMouseEventProcessed:inputObject:event:]
// type: void __cdecl(CameraControl *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[CameraControl postMouseEventProcessed:inputObject:event:]")]
pub fn stub_44cd4(has_touch: bool, matches: bool, end_pan: &mut dyn FnMut()) {
    // IDA 0x44cd4: touch matches cameraTouch and processed -> pan ended.
    if has_touch && matches {
        end_pan();
    }
}

// 0x44d04 — -[CameraControl doCameraPanTouchBegan]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchBegan]")]
pub fn stub_44d04(begin: &mut dyn FnMut()) {
    // IDA 0x44d04: hasRotated=0; capture touch begin (below truncation).
    begin();
}

// 0x44dec — -[CameraControl doCameraPanTouchEnded]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchEnded]")]
pub fn stub_44dec(touch: &mut CameraTouch, reset_input: &mut dyn FnMut()) {
    // IDA 0x44dec: touchBeginPos = -1; cameraTouch = nil; reset input when NewCameraControls.
    touch.begin = (-1.0, -1.0);
    touch.has_touch = false;
    reset_input();
}

// 0x44e58 — -[CameraControl doCameraPanTouchMove]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchMove]")]
pub fn stub_44e58(pan: &mut dyn FnMut()) {
    // IDA 0x44e58: doCameraPanTouchMove (below truncation).
    pan();
}

// 0x450a0 — -[CameraControl touchesBegan:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesBegan:withEvent:]")]
pub fn stub_450a0(has_touch: bool, count: usize, capture: &mut dyn FnMut(), began: &mut dyn FnMut(), forward: &mut dyn FnMut()) {
    // IDA 0x450a0: first touch captured + began; forward to delegate.
    if !has_touch && count == 1 {
        capture();
        began();
    }
    forward();
}

// 0x45124 — -[CameraControl touchesEnded:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesEnded:withEvent:]")]
pub fn stub_45124(ended_ours: bool, end_pan: &mut dyn FnMut(), forward: &mut dyn FnMut()) {
    // IDA 0x45124: touchesEnded — end pan when ours ends; forward (below truncation).
    if ended_ours {
        end_pan();
    }
    forward();
}

// 0x45234 — -[CameraControl touchesCancelled:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesCancelled:withEvent:]")]
pub fn stub_45234(cancelled_ours: bool, end_pan: &mut dyn FnMut(), forward: &mut dyn FnMut()) {
    // IDA 0x45234: touchesCancelled — end pan when ours cancels; forward (below truncation).
    if cancelled_ours {
        end_pan();
    }
    forward();
}

// 0x45344 — -[CameraControl touchesMoved:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesMoved:withEvent:]")]
pub fn stub_45344(moved_ours: bool, move_pan: &mut dyn FnMut(), forward: &mut dyn FnMut()) {
    // IDA 0x45344: touchesMoved — move pan when ours moves; forward (below truncation).
    if moved_ours {
        move_pan();
    }
    forward();
}

// 0x45454 — -[CameraControl .cxx_construct]
// type: id __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl .cxx_construct]")]
pub fn stub_45454(touch: &mut CameraTouch) {
    // IDA 0x45454: cxx_construct — touchBeginPos = 0.
    touch.begin = (0.0, 0.0);
}

// 0x4546c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)")]
pub fn stub_4546c(slots: &mut Vec<UiEventSlot>, target: usize) -> u64 {
    // IDA 0x4546c: operator new islot; callable ctor; signal connect (below truncation).
    let id = slots.len() as u64;
    slots.push(UiEventSlot { id, target, live: true });
    id
}

// 0x45554 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6insertEPNS6_4slotE
// demangled: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::insert(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::insert(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
pub fn stub_45554(slots: &mut Vec<UiEventSlot>, target: usize) -> u64 {
    // IDA 0x45554: signal::insert — new islot; insert (below truncation).
    let id = slots.len() as u64;
    slots.push(UiEventSlot { id, target, live: true });
    id
}

// 0x45764 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)")]
pub fn stub_45764(slot: &mut Option<usize>, value: Option<usize>, add_ref: &mut dyn FnMut(usize), release: &mut dyn FnMut(usize)) -> Option<usize> {
    // IDA 0x45764: add_ref(new); store; release(old).
    if let Some(v) = value {
        add_ref(v);
    }
    let old = std::mem::replace(slot, value);
    if let Some(o) = old {
        release(o);
    }
    *slot
}

// 0x45808 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)")]
pub fn stub_45808(slot: &mut Option<usize>, value: Option<usize>, add_ref: &mut dyn FnMut(usize), release: &mut dyn FnMut(usize)) -> Option<usize> {
    // IDA 0x45808: add_ref(new); store; release(old).
    if let Some(v) = value {
        add_ref(v);
    }
    let old = std::mem::replace(slot, value);
    if let Some(o) = old {
        release(o);
    }
    *slot
}

// 0x458ac — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_do_get_mutex(void)")]
pub fn stub_458ac(guard: &mut bool, slot: &mut Option<usize>, alloc: &mut dyn FnMut(usize) -> usize, init: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x458ac: guarded one-time mutex alloc (0x2C) + construct.
    if !*guard {
        let m = alloc(0x2C);
        init(m);
        *slot = Some(m);
        *guard = true;
    }
    slot.unwrap_or(0)
}

// 0x459a4 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
// demangled: rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)")]
pub fn stub_459a4(slot: usize, init: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x459a4: callable ctor — vtable + functor assign (below truncation).
    init(slot);
    slot
}

// 0x45aa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev
// demangled: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
pub fn stub_45aa0(slots: &mut Vec<UiEventSlot>, id: u64, clear: &mut dyn FnMut(u64), release: &mut dyn FnMut(u64)) {
    // IDA 0x45aa0: D1: function clear; vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        clear(s.id);
        release(s.id);
    }
}

// 0x45b74 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED0Ev
// demangled: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
pub fn stub_45b74(slots: &mut Vec<UiEventSlot>, id: u64, clear: &mut dyn FnMut(u64), release: &mut dyn FnMut(u64)) {
    // IDA 0x45b74: D0: function clear; vtable resets; release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        clear(s.id);
        release(s.id);
    }
}

// 0x45c4c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot10disconnectEv
// demangled: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::disconnect(void)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::disconnect(void)")]
pub fn stub_45c4c(slots: &mut Vec<UiEventSlot>, id: u64, disconnect: &mut dyn FnMut(u64)) {
    // IDA 0x45c4c: slot::disconnect (below truncation).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        disconnect(s.id);
    }
}

// 0x45d5c — __ZNK3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot9connectedEv
// demangled: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::connected(void)const
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::connected(void)const")]
pub fn stub_45d5c(live: bool) -> bool {
    // IDA 0x45d5c: connected = slot word != 0.
    live
}

// 0x45d68 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
// demangled: rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
pub fn stub_45d68(target: usize, processed: bool, input: usize, event: usize, invoke: &mut dyn FnMut(usize, bool, usize, usize)) {
    // IDA 0x45d68: callable::call forwards to function3::operator().
    invoke(target, processed, input, event);
}

// 0x45d98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
// demangled: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
pub fn stub_45d98(target: usize, processed: bool, input: usize, event: usize, invoke: &mut dyn FnMut(usize, bool, usize, usize)) {
    // IDA 0x45d98: non-virtual thunk adjusts inward then tail-calls the operator().
    invoke(target, processed, input, event);
}

// 0x45dc8 — __ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_
// demangled: boost::function3<void,bool,void *,RBX::UIEvent>::operator()(bool,void *,RBX::UIEvent)const
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::operator()(bool,void *,RBX::UIEvent)const")]
pub fn stub_45dc8(has_fn: bool, invoke: &mut dyn FnMut()) {
    // IDA 0x45dc8: function3::operator() — empty call throws (below truncation).
    if !has_fn {
        panic!("bad_function_call");
    }
    invoke();
}

// 0x45eb0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6removeEPNS6_4slotE
// demangled: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::remove(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::remove(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
pub fn stub_45eb0(slots: &mut Vec<UiEventSlot>, id: u64, expired: bool, remove: &mut dyn FnMut(u64)) {
    // IDA 0x45eb0: ReleaseAssert(!expired); remove slot.
    assert!(!expired, "!boost::intrusive_ptr_expired(item)");
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        remove(s.id);
    }
}

// 0x45fa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_init_mutex(void)")]
pub fn stub_45fa0(get: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x45fa0: thunk tail-calls safe_static_do_get_mutex.
    get()
}

// 0x45fa4 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_45fa4(guard: &mut bool, slot: &mut Option<usize>, alloc: &mut dyn FnMut(usize) -> usize, init: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x45fa4: guarded one-time mutex alloc + construct.
    if !*guard {
        let m = alloc(0x2C);
        init(m);
        *slot = Some(m);
        *guard = true;
    }
    slot.unwrap_or(0)
}

// 0x46094 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
pub fn stub_46094(slots: &mut Vec<UiEventSlot>, id: u64, clear: &mut dyn FnMut(u64), release: &mut dyn FnMut(u64)) {
    // IDA 0x46094: D1: function clear; vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        clear(s.id);
        release(s.id);
    }
}

// 0x46168 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
pub fn stub_46168(slots: &mut Vec<UiEventSlot>, id: u64, clear: &mut dyn FnMut(u64), release: &mut dyn FnMut(u64)) {
    // IDA 0x46168: D0: function clear; vtable resets; release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        clear(s.id);
        release(s.id);
    }
}

// 0x46240 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD1Ev
// demangled: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
pub fn stub_46240(slots: &mut Vec<UiEventSlot>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x46240: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x462ec — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD0Ev
// demangled: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
pub fn stub_462ec(slots: &mut Vec<UiEventSlot>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x462ec: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x4639c — __ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_
// demangled: boost::function3<void,bool,void *,RBX::UIEvent>::assign_to_own(boost::function3<void,bool,void *,RBX::UIEvent> const&)
// type: int(void)
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::assign_to_own(boost::function3<void,bool,void *,RBX::UIEvent> const&)")]
pub fn stub_4639c(dst: usize, has_src: bool, is_small: bool, copy: &mut dyn FnMut(usize, bool)) -> usize {
    // IDA 0x4639c: function3::assign_to_own — inline small copy else heap clone; return dst.
    if has_src {
        copy(dst, is_small);
    }
    dst
}

// 0x463cc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_463cc(op: u32, manage: &mut dyn FnMut(u32) -> usize) -> usize {
    // IDA 0x463cc: functor_manager::manage — clone/move/destroy by op (below truncation).
    manage(op)
}

// 0x4642c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEvbS8_SA_E6invokeERNS1_15function_bufferEbS8_SA_
// demangled: boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>,void,bool,objc_selector *,RBX>::invoke(boost::detail::function::function_buffer &,bool,objc_selector *,RBX)
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>,void,bool,objc_selector *,RBX>::invoke(boost::detail::function::function_buffer &,bool,objc_selector *,RBX)")]
pub fn stub_4642c(target: usize, sel: usize, a: bool, b: usize, c: usize, invoke: &mut dyn FnMut(usize, usize, bool, usize, usize)) {
    // IDA 0x4642c: invoker forwards objc msgSend(target, sel, processed, input, event).
    invoke(target, sel, a, b, c);
}
