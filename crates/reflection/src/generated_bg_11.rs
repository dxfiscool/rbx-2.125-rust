//! reflection — generated_bg_11 — 120 stubs EA-sorted asc global gap filler 0x427b4..0x481cc not yet in crates/reflection (global 85545 funcs, 71511 gaps reflection before; 14035->14155 distinct)
//! Source: ida/export.json (85545 funcs) global EA asc not in crates/reflection/src — next 120 uncovered for reflection-bg sorted asc after 0x427b3
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// `RobloxWebUtility` + `iOSSettingsService` state (IDA 0x427c0-0x43180):
/// init count, cached-settings flag, last fetch time and fetch count.
/// Queues, the settings map and the JSON fetch live out of slice.
pub(crate) static WEBUTILITY_INITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static IOS_SETTINGS_CACHED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static IOS_SETTINGS_TIME: std::sync::LazyLock<
    parking_lot::Mutex<f64>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(0.0));
pub(crate) static IOS_SETTINGS_FETCHES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static IOS_SETTINGS_THISPTR: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
/// `iOSSettingsService` construction state (IDA 0x43180): zeroed entry
/// map + empty strings, then `Init`. The map contents are read by the
/// `ReadValue*` family (platform side); the entry count records here.
#[derive(Debug, Clone, Default)]
pub struct IosSettingsState {
    pub entries: u32,
}
/// `FunctionMarshaller` thread windows (IDA 0x43624/0x43804): static
/// `map<thread, marshaller>` with refcounts; a miss constructs +
/// registers, a final release erases. Queue submit/execute/drain
/// counters record the pump traffic.
pub(crate) static MARSHALLER_WINDOWS: std::sync::LazyLock<
    parking_lot::Mutex<std::collections::HashMap<usize, (u32, u32)>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));
pub(crate) static MARSHALLER_NEXT: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(1);
pub(crate) static MARSHALLER_SUBMITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static MARSHALLER_EXECUTES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static MARSHALLER_DRAINS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// `FunctionMarshaller` construction state (IDA 0x4352c): deque +
/// mutex init, zeroed fields, capacity stored at +92.
#[derive(Debug, Clone, Default)]
pub struct MarshallerInit {
    pub capacity: u32,
}
/// `FunctionMarshaller::staticData` one-shot flag (IDA 0x441a8/0x441ac).
pub(crate) static MARSHALLER_STATICDATA: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
/// `CameraControl` touch state (IDA 0x44abc-0x44e58): pan-active flag,
/// live touch count and rotated flag. Positions/sensitivity math lives
/// out of slice; the fixed rotate sensitivity is a constant.
pub const CAM_ROTATE_SENSITIVITY: f32 = 0.025;
pub(crate) static CAMERA_PAN_ACTIVE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static CAMERA_TOUCHES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static CAMERA_ROTATED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
/// `CameraControl` init args (IDA 0x44abc): frame + multi-touch flag;
/// `hasRotated` starts clear, sensitivity is fixed, touch origin and
/// screen scale start at -1.
#[derive(Debug, Clone, Default)]
pub struct CameraControlInit {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub multitouch: bool,
}
/// `cameraTouch` tracking + `UIEvent` signal state (IDA 0x450a0-0x46464):
/// whether a touch is captured for the pan, the signal/slot mutex
/// handles and the slot-connected flag. Touch sets and signal payloads
/// live out of slice.
pub(crate) static CAMERA_TOUCH_SET: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static SIGNAL_UIEVENT_MUTEX: std::sync::LazyLock<u32> =
    std::sync::LazyLock::new(|| 1);
pub(crate) static SIGNAL_UIEVENT_SLOT_MUTEX: std::sync::LazyLock<u32> =
    std::sync::LazyLock::new(|| 1);
pub(crate) static UI_SLOT_CONNECTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
/// typeinfo name for the managed `bind_t<objc_object*,objc_selector*,
/// bool,void*,UIEvent>` (IDA 0x463cc, cf. 0x2d644).
pub const BIND_UIEVENT_OBJC_TYPEINFO: &str =
    "bind_t<objc_object*,objc_selector*,bool,void*,UIEvent>";
/// `CharacterMove` movement-signal connections (IDA 0x46704 wires
/// `localCharacterMovementEnabledChange:` when the input service
/// exists).
pub(crate) static CHARACTER_MOVE_CONNS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// `CharacterMove::init:` frame (IDA 0x466cc supers to
/// `ThumbStickControl`).
#[derive(Debug, Clone, Default)]
pub struct CharacterMoveInit {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

// 0x427b4 — ___destroy_helper_block__7
#[doc(alias = "___destroy_helper_block__7")]
pub fn stub_0x427b4() {
    // IDA 0x427b4: `__destroy_helper_block__7` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x427c0 — -[RobloxWebUtility init]
// type: RobloxWebUtility *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility init]")]
pub fn stub_0x427c0() -> usize {
    // IDA 0x427c0: `RobloxWebUtility::init` creates the log + settings
    // queues. Queue handles are opaque; the init records here as a
    // nonzero handle.
    WEBUTILITY_INITS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    1
}

// 0x42880 — -[RobloxWebUtility dealloc]
// type: void __cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility dealloc]")]
pub fn stub_0x42880() {
    // IDA 0x42880: `dealloc` drops the queues, cached settings and
    // request time. Release is drop glue; the cached flag resets here.
    IOS_SETTINGS_CACHED.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x4290c — -[RobloxWebUtility getiOSLogQueue]
// type: dispatch_queue_s *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getiOSLogQueue]")]
pub fn stub_0x4290c() -> usize {
    // IDA 0x4290c: `getiOSLogQueue` returns the log queue handle.
    // Opaque dispatch handle; nonzero when initialized.
    1
}

// 0x4291c — -[RobloxWebUtility getiOSSettingsQueue]
// type: dispatch_queue_s *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getiOSSettingsQueue]")]
pub fn stub_0x4291c() -> usize {
    // IDA 0x4291c: `getiOSSettingsQueue` returns the settings queue
    // handle (distinct from the log queue above).
    2
}

// 0x4292c — -[RobloxWebUtility setCachediOSSettings:]
// type: void __cdecl(RobloxWebUtility *self, SEL, iOSSettingsService *)
#[doc(alias = "-[RobloxWebUtility setCachediOSSettings:]")]
pub fn stub_0x4292c(cached: bool) {
    // IDA 0x4292c: `setCachediOSSettings:` retains the service. The
    // retain is drop glue; presence records here.
    IOS_SETTINGS_CACHED.store(cached, std::sync::atomic::Ordering::SeqCst);
}

// 0x4293c — -[RobloxWebUtility getCachediOSSettings]
// type: iOSSettingsService *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getCachediOSSettings]")]
pub fn stub_0x4293c() -> bool {
    // IDA 0x4293c: `getCachediOSSettings` returns the cached service
    // presence.
    IOS_SETTINGS_CACHED.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x4294c — -[RobloxWebUtility getLastSettingsRequestTime]
// type: id __cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getLastSettingsRequestTime]")]
pub fn stub_0x4294c() -> f64 {
    // IDA 0x4294c: `getLastSettingsRequestTime` returns the stamp.
    *IOS_SETTINGS_TIME.lock()
}

// 0x4295c — -[RobloxWebUtility getiOSSettingsServiceFromWeb]
// type: iOSSettingsService *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getiOSSettingsServiceFromWeb]")]
pub fn stub_0x4295c(now: f64) -> bool {
    // IDA 0x4295c: `getiOSSettingsServiceFromWeb` news the service,
    // fetches "iOSAppSettings" into it (0x4298a-0x429d6), caches it
    // (0x429ee) and stamps the request time (0x42a00-0x42a4a). The
    // fetch records here.
    IOS_SETTINGS_CACHED.store(true, std::sync::atomic::Ordering::SeqCst);
    *IOS_SETTINGS_TIME.lock() = now;
    IOS_SETTINGS_FETCHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    true
}

// 0x42a98 — +[RobloxWebUtility getiOSSettingsServiceWithForcedReadFromWeb:]
// type: iOSSettingsService *__cdecl(id, SEL, char)
#[doc(alias = "+[RobloxWebUtility getiOSSettingsServiceWithForcedReadFromWeb:]")]
pub fn stub_0x42a98(forced: bool, stale: bool, now: f64) -> bool {
    // IDA 0x42a98: `getiOSSettingsServiceWithForcedReadFromWeb:`
    // returns the cache unless a forced read or a stale stamp demands
    // a web fetch (which runs in the 0x42bc8 block). The decision
    // reports here.
    if forced || stale {
        return stub_0x4295c(now);
    }
    stub_0x4293c()
}

// 0x42bc8 — ___63+[RobloxWebUtility getiOSSettingsServiceWithForcedReadFromWeb:]_block_invoke
// type: iOSSettingsService *__fastcall(int)
#[doc(alias = "___63+[RobloxWebUtility getiOSSettingsServiceWithForcedReadFromWeb:]_block_invoke")]
pub fn stub_0x42bc8(now: f64) {
    // IDA 0x42bc8: the forced-read block performs the web fetch and
    // caches it (same store path as 0x4295c). It sequences the fetch
    // here.
    stub_0x4295c(now);
}

// 0x42dd8 — ___copy_helper_block_65
#[doc(alias = "___copy_helper_block_65")]
pub fn stub_0x42dd8() {
    // IDA 0x42dd8: `__copy_helper_block_65` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x42de4 — ___destroy_helper_block_66
#[doc(alias = "___destroy_helper_block_66")]
pub fn stub_0x42de4() {
    // IDA 0x42de4: `__destroy_helper_block_66` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x42dec — +[RobloxWebUtility getUrlForButtonTag:recordPageView:query:]
// type: id __cdecl(id, SEL, int, char, id)
#[doc(alias = "+[RobloxWebUtility getUrlForButtonTag:recordPageView:query:]")]
pub fn stub_0x42dec(
    tag: i32,
    record_page_view: bool,
    query: &str,
    is_tablet: bool,
    base_url: &str,
    search_url: &str,
) -> String {
    // IDA 0x42dec: `getUrlForButtonTag:recordPageView:query:` builds
    // the button URL (same table as `HomeViewController`,
    // `crate::generated_bg_3::stub_0x1cc54`). Delegate to keep one
    // source of truth.
    crate::generated_bg_3::stub_0x1cc54(tag, record_page_view, query, is_tablet, base_url, search_url)
}

// 0x43180 — __ZN18iOSSettingsServiceC2Ev
// type: iOSSettingsService *__fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::iOSSettingsService(void)")]
#[doc(alias = "__ZN18iOSSettingsServiceC2Ev")]
pub fn stub_0x43180() -> IosSettingsState {
    // IDA 0x43180: `iOSSettingsService::iOSSettingsService` zeroes the
    // entry map + strings (0x431a8-0x43208), publishes `_thisPtr`
    // (0x4320e) and runs `Init` (0x43236). The publish records here.
    IOS_SETTINGS_THISPTR.store(true, std::sync::atomic::Ordering::SeqCst);
    IosSettingsState::default()
}

// 0x432b0 — __ZN18iOSSettingsServiceD1Ev
// type: void __fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::~iOSSettingsService()")]
#[doc(alias = "__ZN18iOSSettingsServiceD1Ev")]
pub fn stub_0x432b0() {
    // IDA 0x432b0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x432b4 — __ZN18iOSSettingsServiceD0Ev
// type: void __fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::~iOSSettingsService()")]
#[doc(alias = "__ZN18iOSSettingsServiceD0Ev")]
pub fn stub_0x432b4() {
    // IDA 0x432b4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x432c8 — __ZN18iOSSettingsServiceD2Ev
// type: void __fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::~iOSSettingsService()")]
#[doc(alias = "__ZN18iOSSettingsServiceD2Ev")]
pub fn stub_0x432c8() {
    // IDA 0x432c8: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x43314 — __ZN10SimpleJSOND1Ev
// type: void __fastcall(SimpleJSON *__hidden this)
#[doc(alias = "SimpleJSON::~SimpleJSON()")]
#[doc(alias = "__ZN10SimpleJSOND1Ev")]
pub fn stub_0x43314() {
    // IDA 0x43314: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x43338 — __ZN10SimpleJSOND0Ev
// type: void __fastcall(SimpleJSON *__hidden this)
#[doc(alias = "SimpleJSON::~SimpleJSON()")]
#[doc(alias = "__ZN10SimpleJSOND0Ev")]
pub fn stub_0x43338() {
    // IDA 0x43338: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x43360 — __ZN10SimpleJSON14DefaultHandlerERKSsS1_
#[doc(alias = "SimpleJSON::DefaultHandler(std::string const&,std::string const&)")]
#[doc(alias = "__ZN10SimpleJSON14DefaultHandlerERKSsS1_")]
pub fn stub_0x43360() {
    // IDA 0x43360: `SimpleJSON::DefaultHandler` handles unregistered
    // keys by ignoring them. Default-ignore glue; no explicit body.
}

// 0x43364 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,void (*)(char const*)>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
pub fn stub_0x43364() {
    // IDA 0x43364: `_Rb_tree<string,...>::_M_erase` destroys the
    // handler-map subtree. Map glue covers it; no explicit body.
}

// 0x43394 — __GLOBAL__I_a_13
#[doc(alias = "global constructor keyed to_a_13")]
#[doc(alias = "__GLOBAL__I_a_13")]
pub fn stub_0x43394() {
    // IDA 0x43394: `__GLOBAL__I_a_13` runs the `a_13`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

// 0x4352c — __ZN3RBX18FunctionMarshallerC2Ej
// type: int __fastcall(RBX::FunctionMarshaller *this, int, int, int)
#[doc(alias = "RBX::FunctionMarshaller::FunctionMarshaller(unsigned int)")]
#[doc(alias = "__ZN3RBX18FunctionMarshallerC2Ej")]
pub fn stub_0x4352c(capacity: u32) -> MarshallerInit {
    // IDA 0x4352c: `FunctionMarshaller::FunctionMarshaller` inits the
    // queue deque + mutex (0x43596-0x435b4), zeroes the fields and
    // stores the capacity at +92 (0x435bc-0x435c6).
    MarshallerInit { capacity }
}

// 0x43624 — __ZN3RBX18FunctionMarshaller9GetWindowEv
// type: int __fastcall(RBX::FunctionMarshaller *this, int, int, int)
#[doc(alias = "RBX::FunctionMarshaller::GetWindow(void)")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller9GetWindowEv")]
pub fn stub_0x43624(thread: u32) -> usize {
    // IDA 0x43624: `GetWindow` looks the thread up in the static
    // marshaller map (0x436d4-0x43704); a miss constructs + registers
    // a marshaller and bumps its +88 count (0x43740-0x43776), a hit
    // bumps the count (0x4372e). The handle reports here.
    let mut windows = MARSHALLER_WINDOWS.lock();
    if let Some(handle) = windows
        .iter()
        .find_map(|(h, (t, _))| (*t == thread).then_some(*h))
    {
        if let Some((_, refs)) = windows.get_mut(&handle) {
            *refs += 1;
        }
        return handle;
    }
    let handle = MARSHALLER_NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst) as usize;
    windows.insert(handle, (thread, 1));
    handle
}

// 0x43804 — __ZN3RBX18FunctionMarshaller13ReleaseWindowEPS0_
// type: void __fastcall(RBX::FunctionMarshaller *this, RBX::FunctionMarshaller *, int, int)
#[doc(alias = "RBX::FunctionMarshaller::ReleaseWindow(RBX::FunctionMarshaller*)")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller13ReleaseWindowEPS0_")]
pub fn stub_0x43804(handle: usize) -> bool {
    // IDA 0x43804: `ReleaseWindow` decrements the +88 count (0x43878)
    // and erases the map entry at zero (0x4389a-0x438bc). Liveness
    // reports here.
    let mut windows = MARSHALLER_WINDOWS.lock();
    let alive = match windows.get_mut(&handle) {
        Some((_, refs)) => {
            *refs = refs.saturating_sub(1);
            *refs != 0
        }
        None => false,
    };
    if !alive {
        windows.remove(&handle);
    }
    alive
}

// 0x43930 — __ZN3RBX18FunctionMarshaller14handleAppEventEPv
// type: void __fastcall(RBX::FunctionMarshaller *this, void *)
#[doc(alias = "RBX::FunctionMarshaller::handleAppEvent(void *)")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller14handleAppEventEPv")]
pub fn stub_0x43930() {
    // IDA 0x43930: `handleAppEvent` pumps app events into the
    // marshaller (platform-owned dispatch). Event-pump glue; no
    // explicit body.
}

// 0x43a98 — __ZN3RBX18FunctionMarshaller7ExecuteEN5boost8functionIFvvEEEPNS_6CEventE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::FunctionMarshaller::Execute(boost::function<void ()(void)>,RBX::CEvent *)")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller7ExecuteEN5boost8functionIFvvEEEPNS_6CEventE")]
pub fn stub_0x43a98() {
    // IDA 0x43a98: `Execute` runs the closure and signals the event.
    // Scheduler glue; the dispatch records here.
    MARSHALLER_EXECUTES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x43b98 — __ZN3RBX18FunctionMarshaller6SubmitEN5boost8functionIFvvEEE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::FunctionMarshaller::Submit(boost::function<void ()(void)>)")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller6SubmitEN5boost8functionIFvvEEE")]
pub fn stub_0x43b98() {
    // IDA 0x43b98: `Submit` queues the closure (same dispatch shape as
    // 0x335f8). The submit records here.
    MARSHALLER_SUBMITS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x43c70 — __ZN3RBX18FunctionMarshaller15ProcessMessagesEv
// type: CFRunLoopRunResult __fastcall(Roblox *this)
#[doc(alias = "RBX::FunctionMarshaller::ProcessMessages(void)")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller15ProcessMessagesEv")]
pub fn stub_0x43c70() {
    // IDA 0x43c70: `ProcessMessages` drains the queue (same call shape
    // as 0x371fe). The drain records here.
    MARSHALLER_DRAINS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x43c74 — __ZN3RBX18FunctionMarshaller10StaticDataD1Ev
// type: void __fastcall(RBX::FunctionMarshaller::StaticData *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::StaticData::~StaticData()")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller10StaticDataD1Ev")]
pub fn stub_0x43c74() {
    // IDA 0x43c74: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x43c78 — __ZN3RBX18FunctionMarshaller10StaticDataD2Ev
// type: void __fastcall(RBX::FunctionMarshaller::StaticData *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::StaticData::~StaticData()")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller10StaticDataD2Ev")]
pub fn stub_0x43c78() {
    // IDA 0x43c78: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x43d14 — __ZNSt3mapIjPN3RBX18FunctionMarshallerESt4lessIjESaISt4pairIKjS2_EEEixERS6_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<unsigned int,RBX::FunctionMarshaller *,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::operator[](unsigned int const&)")]
#[doc(alias = "__ZNSt3mapIjPN3RBX18FunctionMarshallerESt4lessIjESaISt4pairIKjS2_EEEixERS6_")]
pub fn stub_0x43d14() {
    // IDA 0x43d14: `map<uint,Marshaller*>::operator[]` fetches-or-creates
    // the thread slot. Window effects record at `GetWindow`/`ReleaseWindow`
    // (0x43624/0x43804); map mechanics are drop glue here.
}

// 0x43d6c — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseERS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(unsigned int const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseERS1_")]
pub fn stub_0x43d6c() {
    // IDA 0x43d6c: `_Rb_tree<uint,...>::erase(key)` removes the slot.
    // Window effects record at 0x43804; drop glue here.
}

// 0x43d94 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE11equal_rangeERS1_
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::equal_range(unsigned int const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE11equal_rangeERS1_")]
pub fn stub_0x43d94() {
    // IDA 0x43d94: `_Rb_tree<uint,...>::equal_range(key)` locates the
    // slot range. Lookup glue; no explicit body.
}

// 0x43de0 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_")]
pub fn stub_0x43de0() {
    // IDA 0x43de0: `_Rb_tree<uint,...>::erase(first, last)` removes
    // the slot range (same shape as 0x43d6c). Drop glue here.
}

// 0x43e40 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,RBX::FunctionMarshaller *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
pub fn stub_0x43e40() {
    // IDA 0x43e40: `_Rb_tree<uint,...>::_M_erase(node)` destroys the
    // subtree. Map glue covers it; no explicit body.
}

// 0x43e68 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
pub fn stub_0x43e68() {
    // IDA 0x43e68: `_Rb_tree<uint,...>::_M_insert_unique(pos, value)`
    // inserts the slot. Window effects record at 0x43624; drop glue
    // here.
}

// 0x43f1c — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
pub fn stub_0x43f1c() {
    // IDA 0x43f1c: `_Rb_tree<uint,...>::_M_insert` links the node
    // (same shape as 0x43e68). Drop glue here.
}

// 0x43f74 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_")]
pub fn stub_0x43f74() {
    // IDA 0x43f74: `_Rb_tree<uint,...>::_M_insert_unique(value)`
    // inserts the slot (same shape as 0x43e68). Drop glue here.
}

// 0x43fdc — __ZN5boost11unique_lockINS_15recursive_mutexEE4lockEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::unique_lock<boost::recursive_mutex>::lock(void)")]
#[doc(alias = "__ZN5boost11unique_lockINS_15recursive_mutexEE4lockEv")]
pub fn stub_0x43fdc() {
    // IDA 0x43fdc: `unique_lock<recursive_mutex>::lock` takes the
    // lock. Lock glue; no explicit body.
}

// 0x441a8 — __ZN3RBX18FunctionMarshaller27safe_static_init_staticDataEv
// type: _DWORD __fastcall(RBX::FunctionMarshaller *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::safe_static_init_staticData(void)")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller27safe_static_init_staticDataEv")]
pub fn stub_0x441a8() {
    // IDA 0x441a8: `safe_static_init_staticData` one-shots the static
    // data. One-shot init glue; the flag reads at 0x441ac.
}

// 0x441ac — __ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv
// type: void *__fastcall(RBX::FunctionMarshaller *this)
#[doc(alias = "RBX::FunctionMarshaller::safe_static_do_get_staticData(void)")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv")]
pub fn stub_0x441ac() -> bool {
    // IDA 0x441ac: `safe_static_do_get_staticData` returns the static
    // data presence.
    MARSHALLER_STATICDATA.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x442bc — __ZN5boost15recursive_mutexC2Ev
// type: _DWORD __fastcall(boost::recursive_mutex *__hidden this)
#[doc(alias = "boost::recursive_mutex::recursive_mutex(void)")]
#[doc(alias = "__ZN5boost15recursive_mutexC2Ev")]
pub fn stub_0x442bc() {
    // IDA 0x442bc: `recursive_mutex::recursive_mutex` constructs the
    // mutex. Construction glue; no explicit body.
}

// 0x44564 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EED2Ev")]
pub fn stub_0x44564() {
    // IDA 0x44564: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x44590 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE17_M_initialize_mapEm")]
pub fn stub_0x44590() {
    // IDA 0x44590: `_Deque_base<function*>::_M_initialize_map`
    // allocates the queue map. Deque glue covers it; no explicit body.
}

// 0x446e8 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_allocate_mapEm
// type: int(void)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_allocate_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_allocate_mapEm")]
pub fn stub_0x446e8() {
    // IDA 0x446e8: `_Deque_base<function*>::_M_allocate_map` allocates
    // the map storage (same shape as 0x44590). Deque glue covers it;
    // no explicit body.
}

// 0x44700 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_create_nodesEPPS4_S8_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_create_nodes(boost::function<void ()(void)> ***,boost::function<void ()(void)> ***)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_create_nodesEPPS4_S8_")]
pub fn stub_0x44700() {
    // IDA 0x44700: `_Deque_base<function*>::_M_create_nodes` builds
    // the nodes. Deque glue covers it; no explicit body.
}

// 0x447f4 — __ZNSt5dequeIPN5boost8functionIFvvEEESaIS4_EEC2ERKS6_
// type: int __fastcall(int)
#[doc(alias = "std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::deque(std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>> const&)")]
#[doc(alias = "__ZNSt5dequeIPN5boost8functionIFvvEEESaIS4_EEC2ERKS6_")]
pub fn stub_0x447f4() {
    // IDA 0x447f4: `deque<function*>::deque(const&)` copies the queue.
    // Deque glue covers it; no explicit body.
}

// 0x44888 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN5boost8functionIFvvEEERKS8_PS9_ES3_IS8_RS8_PS8_EEET0_T_SH_SG_
#[doc(alias = "std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>>(std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN5boost8functionIFvvEEERKS8_PS9_ES3_IS8_RS8_PS8_EEET0_T_SH_SG_")]
pub fn stub_0x44888() {
    // IDA 0x44888: `__copy<...>::copy(deque iterators)` copies the
    // range. Deque glue covers it; no explicit body.
}

// 0x44924 — __GLOBAL__I_a_14
#[doc(alias = "global constructor keyed to_a_14")]
#[doc(alias = "__GLOBAL__I_a_14")]
pub fn stub_0x44924() {
    // IDA 0x44924: `__GLOBAL__I_a_14` runs the `a_14`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

// 0x44abc — -[CameraControl init:delegate:]
// type: id __cdecl(CameraControl *self, SEL, CGRect, id)
#[doc(alias = "-[CameraControl init:delegate:]")]
pub fn stub_0x44abc(x: f32, y: f32, width: f32, height: f32) -> CameraControlInit {
    // IDA 0x44abc: `CameraControl::init:delegate:` sets the frame
    // (0x44b10), enables multi-touch (0x44b24), clears `hasRotated`
    // (0x44b40), stores the delegate (0x44b56) and fixes sensitivity
    // 0.025 with touch origin + scale at -1 (0x44b66-0x44b82). The
    // frame records here; the rest are constants above.
    CameraControlInit { x, y, width, height, multitouch: true }
}

// 0x44b90 — -[CameraControl dealloc]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl dealloc]")]
pub fn stub_0x44b90() {
    // IDA 0x44b90: `dealloc` drops the control. Release is drop glue;
    // the pan state resets here.
    CAMERA_PAN_ACTIVE.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x44bbc — -[CameraControl setupPostMouseEventConnection]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl setupPostMouseEventConnection]")]
pub fn stub_0x44bbc() {
    // IDA 0x44bbc: `setupPostMouseEventConnection` binds
    // `postMouseEventProcessed:inputObject:event:` to the input
    // service signal (0x44bec-0x44c5e). Closure + slot glue; no
    // explicit body.
}

// 0x44cd4 — -[CameraControl postMouseEventProcessed:inputObject:event:]
// type: void __cdecl(CameraControl *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[CameraControl postMouseEventProcessed:inputObject:event:]")]
pub fn stub_0x44cd4(consumed: bool, is_camera_touch: bool) {
    // IDA 0x44cd4: `postMouseEventProcessed:` ends the camera pan when
    // a consumed input matches `cameraTouch` (0x44cee-0x44cfe). It
    // sequences the pan end here.
    if consumed && is_camera_touch {
        stub_0x44dec();
    }
}

// 0x44d04 — -[CameraControl doCameraPanTouchBegan]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchBegan]")]
pub fn stub_0x44d04() {
    // IDA 0x44d04: `doCameraPanTouchBegan` starts the pan. It records
    // here.
    CAMERA_PAN_ACTIVE.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x44dec — -[CameraControl doCameraPanTouchEnded]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchEnded]")]
pub fn stub_0x44dec() {
    // IDA 0x44dec: `doCameraPanTouchEnded` ends the pan. It records
    // here.
    CAMERA_PAN_ACTIVE.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x44e58 — -[CameraControl doCameraPanTouchMove]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchMove]")]
pub fn stub_0x44e58() {
    // IDA 0x44e58: `doCameraPanTouchMove` rotates by the touch delta
    // (position math out of slice) and marks rotated. The mark
    // records here.
    CAMERA_ROTATED.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x450a0 — -[CameraControl touchesBegan:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesBegan:withEvent:]")]
pub fn stub_0x450a0(touch_count: u32) {
    // IDA 0x450a0: `touchesBegan:` captures the single touch as
    // `cameraTouch` and begins the pan when none is captured
    // (0x450d2-0x450f6), then forwards to the delegate (0x45120).
    // The forward is drop glue; the capture records here.
    if !CAMERA_TOUCH_SET.load(std::sync::atomic::Ordering::SeqCst) && touch_count == 1 {
        CAMERA_TOUCH_SET.store(true, std::sync::atomic::Ordering::SeqCst);
        stub_0x44d04();
    }
    CAMERA_TOUCHES.fetch_add(touch_count, std::sync::atomic::Ordering::SeqCst);
}

// 0x45124 — -[CameraControl touchesEnded:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesEnded:withEvent:]")]
pub fn stub_0x45124(matching: bool, ended: u32) {
    // IDA 0x45124: `touchesEnded:` clears a matching `cameraTouch` and
    // ends the pan, then forwards to the delegate (same shape as
    // 0x450a0 tail). The clear records here.
    if matching {
        CAMERA_TOUCH_SET.store(false, std::sync::atomic::Ordering::SeqCst);
        stub_0x44dec();
    }
    CAMERA_TOUCHES.fetch_sub(ended.min(CAMERA_TOUCHES.load(std::sync::atomic::Ordering::SeqCst)), std::sync::atomic::Ordering::SeqCst);
}

// 0x45234 — -[CameraControl touchesCancelled:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesCancelled:withEvent:]")]
pub fn stub_0x45234(matching: bool, cancelled: u32) {
    // IDA 0x45234: `touchesCancelled:` clears a matching `cameraTouch`
    // and ends the pan (same shape as 0x45124). The clear records
    // here.
    if matching {
        CAMERA_TOUCH_SET.store(false, std::sync::atomic::Ordering::SeqCst);
        stub_0x44dec();
    }
    CAMERA_TOUCHES.fetch_sub(cancelled.min(CAMERA_TOUCHES.load(std::sync::atomic::Ordering::SeqCst)), std::sync::atomic::Ordering::SeqCst);
}

// 0x45344 — -[CameraControl touchesMoved:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesMoved:withEvent:]")]
pub fn stub_0x45344(camera_in_set: bool) {
    // IDA 0x45344: `touchesMoved:` pans when `cameraTouch` is in the
    // set (0x453e8-0x45418), then forwards to the delegate (0x4541c).
    // The pan records here.
    if camera_in_set {
        stub_0x44e58();
    }
}

// 0x45454 — -[CameraControl .cxx_construct]
// type: id __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl .cxx_construct]")]
pub fn stub_0x45454() {
    // IDA 0x45454: `.cxx_construct` runs member constructors in place.
    // Construction glue; no explicit body.
}

// 0x4546c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")]
pub fn stub_0x4546c() {
    // IDA 0x4546c: `signal<bool,void*,UIEvent>::connect<function<...>>`
    // installs the slot (same shape as 0x3a278). Closure + slot glue;
    // the install records here.
    UI_SLOT_CONNECTED.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x45554 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::insert(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6insertEPNS6_4slotE")]
pub fn stub_0x45554() {
    // IDA 0x45554: `signal<bool,void*,UIEvent>::insert(slot *)`
    // appends the slot (same shape as 0x3d2f4). The install records
    // here.
    UI_SLOT_CONNECTED.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x45764 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_")]
pub fn stub_0x45764() {
    // IDA 0x45764: `intrusive_ptr<slot>::operator=(slot*)`
    // copy-assigns the slot (same shape as 0x3c0c8). `Arc` clone glue
    // covers it; no explicit body.
}

// 0x45808 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_")]
pub fn stub_0x45808() {
    // IDA 0x45808: `intrusive_ptr<slot>::operator=(const&)`
    // copy-assigns the slot (same shape as 0x3d508). `Arc` clone glue
    // covers it; no explicit body.
}

// 0x458ac — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv")]
pub fn stub_0x458ac() -> u32 {
    // IDA 0x458ac: `signal<bool,void*,UIEvent>::safe_static_do_get_mutex`
    // one-shots the static signal mutex (same shape as 0x3d5b0). The
    // opaque handle records once.
    *SIGNAL_UIEVENT_MUTEX
}

// 0x459a4 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_")]
pub fn stub_0x459a4() {
    // IDA 0x459a4: `callable<slot,function<...>>::callable` wraps the
    // bound slot (same shape as 0x46de8). Closure-wrapping glue; no
    // explicit body.
}

// 0x45aa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev")]
pub fn stub_0x45aa0() {
    // IDA 0x45aa0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x45b74 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED0Ev")]
pub fn stub_0x45b74() {
    // IDA 0x45b74: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x45c4c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot10disconnectEv")]
pub fn stub_0x45c4c() {
    // IDA 0x45c4c: `signal<bool,void*,UIEvent>::slot::disconnect`
    // detaches the slot. The detach records here.
    UI_SLOT_CONNECTED.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x45d5c — __ZNK3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot9connectedEv")]
pub fn stub_0x45d5c() -> bool {
    // IDA 0x45d5c: `signal<bool,void*,UIEvent>::slot::connected`
    // reports the attach state.
    UI_SLOT_CONNECTED.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x45d68 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_")]
pub fn stub_0x45d68() {
    // IDA 0x45d68: `callable<slot,function3<...>>::call` invokes the
    // stored target on the event args. Closure-call glue; no explicit
    // body.
}

// 0x45d98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_")]
pub fn stub_0x45d98() {
    // IDA 0x45d98: non-virtual thunk to `"'rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *` (IDA demangle) -- this/arg-adjust + tail-call. Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x45dc8 — __ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::operator()(bool,void *,RBX::UIEvent)const")]
#[doc(alias = "__ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_")]
pub fn stub_0x45dc8() {
    // IDA 0x45dc8: `function3<void,bool,void*,UIEvent>::operator()`
    // runs the stored target. Closure-call glue; no explicit body.
}

// 0x45eb0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::remove(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6removeEPNS6_4slotE")]
pub fn stub_0x45eb0() {
    // IDA 0x45eb0: `signal<bool,void*,UIEvent>::remove(slot *)`
    // detaches the slot (same shape as 0x3d848). The detach records
    // here.
    UI_SLOT_CONNECTED.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x45fa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x45fa0() {
    // IDA 0x45fa0: `signal<bool,void*,UIEvent>::slot::
    // safe_static_init_mutex` one-shots the static slot mutex (same
    // shape as 0x3c920). One-shot init glue; no explicit body.
}

// 0x45fa4 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0x45fa4() -> u32 {
    // IDA 0x45fa4: `signal<bool,void*,UIEvent>::slot::
    // safe_static_do_get_mutex` one-shots the static slot mutex (same
    // shape as 0x458ac). The opaque handle records once.
    *SIGNAL_UIEVENT_SLOT_MUTEX
}

// 0x46094 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev")]
pub fn stub_0x46094() {
    // IDA 0x46094: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x46168 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev")]
pub fn stub_0x46168() {
    // IDA 0x46168: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x46240 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD1Ev")]
pub fn stub_0x46240() {
    // IDA 0x46240: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x462ec — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD0Ev")]
pub fn stub_0x462ec() {
    // IDA 0x462ec: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4639c — __ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_
// type: int(void)
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::assign_to_own(boost::function3<void,bool,void *,RBX::UIEvent> const&)")]
#[doc(alias = "__ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_")]
pub fn stub_0x4639c() {
    // IDA 0x4639c: `function3<void,bool,void*,UIEvent>::assign_to_own`
    // copy-assigns the function (same shape as 0x3e288). `Box<dyn Fn>`
    // assignment glue; no explicit body.
}

// 0x463cc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x463cc(get_typeinfo: bool) -> &'static str {
    // IDA 0x463cc: `functor_manager<bind_t<objc_object*,objc_selector*,
    // bool,void*,UIEvent>>::manage` answers op 4 with the `bind_t`
    // typeinfo (same shape as 0x4a21c). Other ops are vtable glue.
    if get_typeinfo { BIND_UIEVENT_OBJC_TYPEINFO } else { "" }
}

// 0x4642c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEvbS8_SA_E6invokeERNS1_15function_bufferEbS8_SA_
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>,void,bool,objc_selector *,RBX>::invoke(boost::detail::function::function_buffer &,bool,objc_selector *,RBX)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEvbS8_SA_E6invokeERNS1_15function_bufferEbS8_SA_")]
pub fn stub_0x4642c() {
    // IDA 0x4642c: `void_function_obj_invoker3<bind_t<objc...>>::invoke`
    // runs the bound slot on the event args (same shape as 0x4a27c).
    // Closure-call glue; no explicit body.
}

// 0x46464 — __ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv
// type: int(void)
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::clear(void)")]
#[doc(alias = "__ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv")]
pub fn stub_0x46464() {
    // IDA 0x46464: `function3<void,bool,void*,UIEvent>::clear` drops
    // the stored target (same shape as 0x4bfdc). `Box<dyn Fn>` drop
    // glue covers it; no explicit body.
}

// 0x46490 — __GLOBAL__I_a_15
#[doc(alias = "global constructor keyed to_a_15")]
#[doc(alias = "__GLOBAL__I_a_15")]
pub fn stub_0x46490() {
    // IDA 0x46490: `__GLOBAL__I_a_15` runs the `a_15`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

// 0x466cc — -[CharacterMove init:]
// type: id __cdecl(CharacterMove *self, SEL, CGRect)
#[doc(alias = "-[CharacterMove init:]")]
pub fn stub_0x466cc(x: f32, y: f32, width: f32, height: f32) -> CharacterMoveInit {
    // IDA 0x466cc: `CharacterMove::init:` supers to
    // `ThumbStickControl::init:` on the frame (0x466e6-0x46702). The
    // frame records here.
    CharacterMoveInit { x, y, width, height }
}

// 0x46704 — -[CharacterMove setupCharacterMoveConnection]
// type: void __cdecl(CharacterMove *self, SEL)
#[doc(alias = "-[CharacterMove setupCharacterMoveConnection]")]
pub fn stub_0x46704(service_present: bool) {
    // IDA 0x46704: `setupCharacterMoveConnection` connects
    // `localCharacterMovementEnabledChange:` to the input service
    // signal when the service exists (0x46738-0x4679c, same connect
    // shape as 0x46c18). The install records here.
    if service_present {
        CHARACTER_MOVE_CONNS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x467e8 — -[CharacterMove localCharacterMovementEnabledChange:]
// type: void __cdecl(CharacterMove *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[CharacterMove localCharacterMovementEnabledChange:]")]
pub fn stub_0x467e8() -> ! {
    todo!("0x467e8 -[CharacterMove localCharacterMovementEnabledChange:]")
}

// 0x467ec — -[CharacterMove touchesEnded:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesEnded:withEvent:]")]
pub fn stub_0x467ec() -> ! {
    todo!("0x467ec -[CharacterMove touchesEnded:withEvent:]")
}

// 0x468bc — -[CharacterMove touchesCancelled:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesCancelled:withEvent:]")]
pub fn stub_0x468bc() -> ! {
    todo!("0x468bc -[CharacterMove touchesCancelled:withEvent:]")
}

// 0x4698c — -[CharacterMove cancelMovement]
// type: void __cdecl(CharacterMove *self, SEL)
#[doc(alias = "-[CharacterMove cancelMovement]")]
pub fn stub_0x4698c() -> ! {
    todo!("0x4698c -[CharacterMove cancelMovement]")
}

// 0x469e8 — -[CharacterMove touchesMoved:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesMoved:withEvent:]")]
pub fn stub_0x469e8() -> ! {
    todo!("0x469e8 -[CharacterMove touchesMoved:withEvent:]")
}

// 0x46f64 — __GLOBAL__I_a_16
#[doc(alias = "global constructor keyed to_a_16")]
#[doc(alias = "__GLOBAL__I_a_16")]
pub fn stub_0x46f64() -> ! {
    todo!("0x46f64 global constructor keyed to_a_16")
}

// 0x47178 — -[ControlComponent init]
// type: ControlComponent *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent init]")]
pub fn stub_0x47178() -> ! {
    todo!("0x47178 -[ControlComponent init]")
}

// 0x471c0 — -[ControlComponent findControlView]
// type: id __cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent findControlView]")]
pub fn stub_0x471c0() -> ! {
    todo!("0x471c0 -[ControlComponent findControlView]")
}

// 0x47274 — -[ControlComponent getGameFromControlView]
// type: Game *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent getGameFromControlView]")]
pub fn stub_0x47274() -> ! {
    todo!("0x47274 -[ControlComponent getGameFromControlView]")
}

// 0x47338 — -[ControlComponent getUserInputServiceForGameDataModel]
// type: UserInputService *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent getUserInputServiceForGameDataModel]")]
pub fn stub_0x47338() -> ! {
    todo!("0x47338 -[ControlComponent getUserInputServiceForGameDataModel]")
}

// 0x47424 — __GLOBAL__I_a_17
#[doc(alias = "global constructor keyed to_a_17")]
#[doc(alias = "__GLOBAL__I_a_17")]
pub fn stub_0x47424() -> ! {
    todo!("0x47424 global constructor keyed to_a_17")
}

// 0x47638 — -[ControlView init:withGame:]
// type: id __cdecl(ControlView *self, SEL, CGRect, shared_ptr<RBX::Game>)
#[doc(alias = "-[ControlView init:withGame:]")]
pub fn stub_0x47638() -> ! {
    todo!("0x47638 -[ControlView init:withGame:]")
}

// 0x47904 — -[ControlView dealloc]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView dealloc]")]
pub fn stub_0x47904() -> ! {
    todo!("0x47904 -[ControlView dealloc]")
}

// 0x479f8 — -[ControlView setGame:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::Game>)
#[doc(alias = "-[ControlView setGame:]")]
pub fn stub_0x479f8() -> ! {
    todo!("0x479f8 -[ControlView setGame:]")
}

// 0x47aec — -[ControlView gotStartLeaveGameNotification:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView gotStartLeaveGameNotification:]")]
pub fn stub_0x47aec() -> ! {
    todo!("0x47aec -[ControlView gotStartLeaveGameNotification:]")
}

// 0x47afc — -[ControlView dataModelChanged:]
// type: void __cdecl(ControlView *self, SEL, DataModel *)
#[doc(alias = "-[ControlView dataModelChanged:]")]
pub fn stub_0x47afc() -> ! {
    todo!("0x47afc -[ControlView dataModelChanged:]")
}

// 0x47b38 — -[ControlView setControlVisibility:]
// type: void __cdecl(ControlView *self, SEL, char)
#[doc(alias = "-[ControlView setControlVisibility:]")]
pub fn stub_0x47b38() -> ! {
    todo!("0x47b38 -[ControlView setControlVisibility:]")
}

// 0x47b90 — ___36-[ControlView setControlVisibility:]_block_invoke
#[doc(alias = "___36-[ControlView setControlVisibility:]_block_invoke")]
pub fn stub_0x47b90() -> ! {
    todo!("0x47b90 ___36-[ControlView setControlVisibility:]_block_invoke")
}

// 0x47c04 — ___copy_helper_block__8
#[doc(alias = "___copy_helper_block__8")]
pub fn stub_0x47c04() -> ! {
    todo!("0x47c04 ___copy_helper_block__8")
}

// 0x47c10 — ___destroy_helper_block__8
#[doc(alias = "___destroy_helper_block__8")]
pub fn stub_0x47c10() -> ! {
    todo!("0x47c10 ___destroy_helper_block__8")
}

// 0x47c18 — -[ControlView showControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView showControls]")]
pub fn stub_0x47c18() -> ! {
    todo!("0x47c18 -[ControlView showControls]")
}

// 0x47c2c — -[ControlView hideControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView hideControls]")]
pub fn stub_0x47c2c() -> ! {
    todo!("0x47c2c -[ControlView hideControls]")
}

// 0x47c40 — -[ControlView postMouseEventProcessedFromOverlay:inputObject:event:]
// type: void __cdecl(ControlView *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[ControlView postMouseEventProcessedFromOverlay:inputObject:event:]")]
pub fn stub_0x47c40() -> ! {
    todo!("0x47c40 -[ControlView postMouseEventProcessedFromOverlay:inputObject:event:]")
}

// 0x47d48 — -[ControlView postMouseEventProcessed:inputObject:event:]
// type: void __cdecl(ControlView *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[ControlView postMouseEventProcessed:inputObject:event:]")]
pub fn stub_0x47d48() -> ! {
    todo!("0x47d48 -[ControlView postMouseEventProcessed:inputObject:event:]")
}

// 0x47d78 — -[ControlView setupLocalPlayerConnections]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupLocalPlayerConnections]")]
pub fn stub_0x47d78() -> ! {
    todo!("0x47d78 -[ControlView setupLocalPlayerConnections]")
}

// 0x47d7c — -[ControlView textBoxFocusGained:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::TextBox>)
#[doc(alias = "-[ControlView textBoxFocusGained:]")]
pub fn stub_0x47d7c() -> ! {
    todo!("0x47d7c -[ControlView textBoxFocusGained:]")
}

// 0x47ea4 — -[ControlView getGame]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, ControlView *self, SEL)
#[doc(alias = "-[ControlView getGame]")]
pub fn stub_0x47ea4() -> ! {
    todo!("0x47ea4 -[ControlView getGame]")
}

// 0x47f48 — -[ControlView setupEvents]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupEvents]")]
pub fn stub_0x47f48() -> ! {
    todo!("0x47f48 -[ControlView setupEvents]")
}

// 0x4818c — -[ControlView disconnectEvents]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView disconnectEvents]")]
pub fn stub_0x4818c() -> ! {
    todo!("0x4818c -[ControlView disconnectEvents]")
}

// 0x481cc — -[ControlView bindToUserInputService:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::DataModel>)
#[doc(alias = "-[ControlView bindToUserInputService:]")]
pub fn stub_0x481cc() -> ! {
    todo!("0x481cc -[ControlView bindToUserInputService:]")
}
