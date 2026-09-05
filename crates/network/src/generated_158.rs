//! Auto-generated skeletons for rbx-network — global EA-sorted filler (RakNet|Network|Replicat|Socket filtered exhausted)
//! Filter: RakNet|Network|Replicat|Socket -> 5198 funcs (cs), 5282 (ci), 3 remaining before batch (next 0xecd6e8 _TFCreateCrashSocket); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x35ee4..0x3b724 | existing 17609 -> 17709 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::HashMap;

/// `rbx::signals` slot connection (IDA 0x3a278 et al.).
#[derive(Clone, Debug, Default)]
pub struct SignalSlotConn {
 pub id: u64,
 pub target: usize,
 pub live: bool,
}

/// `std::map<Name const*, ICreator const*>` (IDA 0x3aa30 et al.).
#[derive(Clone, Debug, Default)]
pub struct CreatorMap {
 pub entries: HashMap<String, usize>,
}

/// `RobloxInfo` cached URL strings (IDA 0x36918/0x369c0/0x36ab0: dword_130C460/64/68).
#[derive(Clone, Debug, Default)]
pub struct RobloxInfoUrls {
 pub base_url: Option<String>,
 pub api_base_url: Option<String>,
 pub domain: Option<String>,
}

/// Static-init state for `__GLOBAL__I_a_9` (IDA 0x36e80).
#[derive(Clone, Debug, Default)]
pub struct GlobalInitA9 {
 pub done: bool,
}
// 0x35ee4 — ___58+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]_block_invoke
// demangled: ___58+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]_block_invoke
// type: 
#[doc(alias = "___58+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]_block_invoke")]
pub fn stub_35ee4(show_alert: &mut dyn FnMut(&str)) {
    // IDA 0x35ee4: alloc UIAlertView; localized "RobloxWord" title; show.
    show_alert("RobloxWord");
}

// 0x35ffc — ___copy_helper_block_19
// demangled: ___copy_helper_block_19
// type: 
#[doc(alias = "___copy_helper_block_19")]
pub fn stub_35ffc(dst20: &mut usize, dst24: &mut usize, src20: usize, src24: usize, retain: &mut dyn FnMut(usize) -> usize) {
    // IDA 0x35ffc: _Block_object_assign(dst+20, src+20, 3); _Block_object_assign(dst+24, src+24, 3).
    *dst20 = retain(src20);
    *dst24 = retain(src24);
}

// 0x36020 — ___destroy_helper_block_20
// demangled: ___destroy_helper_block_20
// type: 
#[doc(alias = "___destroy_helper_block_20")]
pub fn stub_36020(slot20: &mut usize, slot24: &mut usize, release: &mut dyn FnMut(usize)) {
    // IDA 0x36020: _Block_object_dispose(slot+20, 3); _Block_object_dispose(slot+24, 3).
    release(*slot20);
    release(*slot24);
}

// 0x3603c — __Z18getUserAgentStringv
// demangled: getUserAgentString(void)
// type: id __fastcall()
#[doc(alias = "getUserAgentString(void)")]
pub fn stub_3603c(get: &mut dyn FnMut() -> String) -> String {
    // IDA 0x3603c: tail-calls +[RobloxInfo getUserAgentString].
    get()
}

// 0x36058 — +[RobloxInfo getDeviceType]
// demangled: +[RobloxInfo getDeviceType]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getDeviceType]")]
pub fn stub_36058(device_type: Option<&str>) -> Option<&str> {
    // IDA 0x36058: nil deviceType -> nil; "iPad" in it -> iPad; "iPhone" in it -> iPhone (tail below truncation).
    let dt = device_type?;
    if dt.contains("iPad") {
        Some("iPad")
    } else if dt.contains("iPhone") {
        Some("iPhone")
    } else {
        Some(dt)
    }
}

// 0x36114 — +[RobloxInfo getDeviceModelNumber]
// demangled: +[RobloxInfo getDeviceModelNumber]
// type: int __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getDeviceModelNumber]")]
pub fn stub_36114(is_tablet: bool, tablet_model: i32, phone_model: i32) -> i32 {
    // IDA 0x36114: tablet ? iPad-range model number : iPhone-range model number (below truncation).
    if is_tablet {
        tablet_model
    } else {
        phone_model
    }
}

// 0x3622c — +[RobloxInfo thisDeviceIsATablet]
// demangled: +[RobloxInfo thisDeviceIsATablet]
// type: char __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo thisDeviceIsATablet]")]
pub fn stub_3622c(supports_idiom: bool, idiom: i32) -> bool {
    // IDA 0x3622c: respondsToSelector(userInterfaceIdiom) ? userInterfaceIdiom == 1 (Pad) : NO.
    supports_idiom && idiom == 1
}

// 0x36290 — +[RobloxInfo deviceType]
// demangled: +[RobloxInfo deviceType]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo deviceType]")]
pub fn stub_36290(machine: &str) -> &str {
    // IDA 0x36290: sysctlbyname("hw.machine") -> stringWithUTF8String.
    machine
}

// 0x362fc — +[RobloxInfo deviceOSVersion]
// demangled: +[RobloxInfo deviceOSVersion]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo deviceOSVersion]")]
pub fn stub_362fc(version: &str) -> &str {
    // IDA 0x362fc: [[UIDevice currentDevice] systemVersion].
    version
}

// 0x36330 — +[RobloxInfo appVersion]
// demangled: +[RobloxInfo appVersion]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo appVersion]")]
pub fn stub_36330(version: &str) -> &str {
    // IDA 0x36330: mainBundle objectForInfoDictionaryKey CFBundleShortVersionString.
    version
}

// 0x36370 — +[RobloxInfo friendlyDeviceName]
// demangled: +[RobloxInfo friendlyDeviceName]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo friendlyDeviceName]")]
pub fn stub_36370(device_type: &str, lookup: &mut dyn FnMut(&str) -> Option<String>) -> Option<String> {
    // IDA 0x36370: machine-id -> friendly name table (iPhone1,1 -> "iPhone 2G", ...).
    lookup(device_type)
}

// 0x3683c — +[RobloxInfo getUserAgentString]
// demangled: +[RobloxInfo getUserAgentString]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getUserAgentString]")]
pub fn stub_3683c(model: &str, device: &str, os: &str, app: &str, build: &mut dyn FnMut(&str, &str, &str, &str) -> String) -> String {
    // IDA 0x3683c: user-agent string from model/deviceType/systemVersion/appVersion (below truncation).
    build(model, device, os, app)
}

// 0x36918 — +[RobloxInfo getBaseUrl]
// demangled: +[RobloxInfo getBaseUrl]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getBaseUrl]")]
pub fn stub_36918(urls: &mut RobloxInfoUrls, is_tablet: bool, resolved: &str) -> String {
    // IDA 0x36918: cached base URL; else infoDictionary RbxBaseUrl (tablet) or phone key; cache + return.
    if let Some(u) = urls.base_url.clone() {
        return u;
    }
    let _ = is_tablet;
    let u = resolved.to_string();
    urls.base_url = Some(u.clone());
    u
}

// 0x369c0 — +[RobloxInfo getApiBaseUrl]
// demangled: +[RobloxInfo getApiBaseUrl]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getApiBaseUrl]")]
pub fn stub_369c0(urls: &mut RobloxInfoUrls, base_url: &str, derive: &mut dyn FnMut(&str) -> String) -> String {
    // IDA 0x369c0: cached api URL; else derive from base URL host (below truncation).
    if let Some(u) = urls.api_base_url.clone() {
        return u;
    }
    let u = derive(base_url);
    urls.api_base_url = Some(u.clone());
    u
}

// 0x36ab0 — +[RobloxInfo getDomainString]
// demangled: +[RobloxInfo getDomainString]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getDomainString]")]
pub fn stub_36ab0(urls: &mut RobloxInfoUrls, base_url: &str) -> String {
    // IDA 0x36ab0: cached domain; empty base -> empty; else strip "http://" (tail below truncation).
    if let Some(d) = urls.domain.clone() {
        return d;
    }
    let d = if base_url.is_empty() {
        String::new()
    } else {
        base_url.strip_prefix("http://").unwrap_or(base_url).trim_end_matches('/').to_string()
    };
    urls.domain = Some(d.clone());
    d
}

// 0x36bc8 — +[RobloxInfo getBaseUrlChangedNotification]
// demangled: +[RobloxInfo getBaseUrlChangedNotification]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getBaseUrlChangedNotification]")]
pub fn stub_36bc8() -> &'static str {
    // IDA 0x36bc8: return "RBXBaseUrlChangedNotifier".
    "RBXBaseUrlChangedNotifier"
}

// 0x36bd4 — +[RobloxInfo setBaseUrl:]
// demangled: +[RobloxInfo setBaseUrl:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxInfo setBaseUrl:]")]
pub fn stub_36bd4(urls: &mut RobloxInfoUrls, url: String, notify: &mut dyn FnMut(&str)) {
    // IDA 0x36bd4: store base URL (trailing-slash fixup); post RBXBaseUrlChangedNotifier (tail below truncation).
    let mut u = url;
    if !u.ends_with('/') {
        u.push('/');
    }
    urls.base_url = Some(u);
    notify("RBXBaseUrlChangedNotifier");
}

// 0x36de4 — ___25+[RobloxInfo setBaseUrl:]_block_invoke
// demangled: ___25+[RobloxInfo setBaseUrl:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___25+[RobloxInfo setBaseUrl:]_block_invoke")]
pub fn stub_36de4(refresh: &mut dyn FnMut(bool)) {
    // IDA 0x36de4: getiOSSettingsServiceWithForcedReadFromWeb:NO.
    refresh(false);
}

// 0x36e04 — +[RobloxInfo searchUrl]
// demangled: +[RobloxInfo searchUrl]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo searchUrl]")]
pub fn stub_36e04<'a>(is_tablet: bool, phone: &'a str, tablet: &'a str) -> &'a str {
    // IDA 0x36e04: settings search URL (phone var31 / tablet var30).
    if is_tablet {
        tablet
    } else {
        phone
    }
}

// 0x36e80 — __GLOBAL__I_a_9
// demangled: global constructor keyed to_a_9
// type: 
#[doc(alias = "global constructor keyed to_a_9")]
pub fn stub_36e80(state: &mut GlobalInitA9, init: &mut dyn FnMut()) {
    // IDA 0x36e80: boost error categories + ios_base::Init + FFlag::DisblePlayButtonForNonBC registration.
    if !state.done {
        init();
        state.done = true;
    }
}

// 0x37068 — __ZN10RobloxView37requestStopRenderingForBackgroundModeEv
// demangled: RobloxView::requestStopRenderingForBackgroundMode(void)
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::requestStopRenderingForBackgroundMode(void)")]
pub fn stub_37068(cleanup_in_background: bool, stop: &mut dyn FnMut(bool)) {
    // IDA 0x37068: FFlag::RenderCleanupInBackground gates the RenderJob stop (below truncation).
    stop(cleanup_in_background);
}

// 0x37378 — __ZN10RobloxView22requestResumeRenderingEv
// demangled: RobloxView::requestResumeRendering(void)
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::requestResumeRendering(void)")]
pub fn stub_37378(alloc: &mut dyn FnMut(usize) -> usize, start: &mut dyn FnMut(usize)) {
    // IDA 0x37378: alloc render job (0x1E8) + resume (below truncation).
    let job = alloc(0x1E8);
    start(job);
}

// 0x375b4 — __Z13macBundlePathv
// demangled: macBundlePath(void)
// type: _DWORD __fastcall()
#[doc(alias = "macBundlePath(void)")]
pub fn stub_375b4(bundle_path: &str) -> String {
    // IDA 0x375b4: main bundle POSIX path (1024-byte buffer) into std::string.
    bundle_path.to_string()
}

// 0x37628 — __ZN10RobloxViewC2EjjSsSsSs
// demangled: RobloxView::RobloxView(unsigned int,unsigned int,std::string,std::string,std::string)
// type: 
#[doc(alias = "RobloxView::RobloxView(unsigned int,unsigned int,std::string,std::string,std::string)")]
pub fn stub_37628(view: usize, a: u32, b: u32, s1: &str, s2: &str, s3: &str, init: &mut dyn FnMut(usize, u32, u32, &str, &str, &str)) -> usize {
    // IDA 0x37628: RobloxView::RobloxView — string members + render setup (below truncation).
    init(view, a, b, s1, s2, s3);
    view
}

// 0x37b3c — __ZN10RobloxView16completeViewPrepEN5boost10shared_ptrIN3RBX4GameEEE
// demangled: RobloxView::completeViewPrep(boost::shared_ptr<RBX::Game>)
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, 
// was: boost::shared_ptr
#[doc(alias = "RobloxView::completeViewPrep(rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_37b3c(game: usize, store: &mut dyn FnMut(usize), prep: &mut dyn FnMut()) {
    // IDA 0x37b3c: completeViewPrep — store Game shared_ptr + view prep (below truncation).
    store(game);
    prep();
}

// 0x380a0 — __ZN10RobloxView16onPlaceIDChangedEPKN3RBX10Reflection18PropertyDescriptorE
// demangled: RobloxView::onPlaceIDChanged(RBX::Reflection::PropertyDescriptor const*)
// type: _DWORD __fastcall(RobloxView *__hidden this, const PropertyDescriptor *)
#[doc(alias = "RobloxView::onPlaceIDChanged(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_380a0() {
    // IDA 0x380a0: empty onPlaceIDChanged body.
}

// 0x380a4 — __ZN10RobloxView13bindWorkspaceEN5boost10shared_ptrIN3RBX8ViewBaseEEENS1_INS2_9DataModelEEENS1_INS2_16OverlayDataModelEEE
// demangled: RobloxView::bindWorkspace(boost::shared_ptr<RBX::ViewBase>,boost::shared_ptr<RBX::DataModel>,boost::shared_ptr<RBX::OverlayDataModel>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "RobloxView::bindWorkspace(rbx_core::SharedPtr<RBX::ViewBase>,rbx_core::SharedPtr<RBX::DataModel>,rbx_core::SharedPtr<RBX::OverlayDataModel>)")]
pub fn stub_380a4(bind: &mut dyn FnMut()) {
    // IDA 0x380a4: RobloxView::bindWorkspace — workspace shared_ptr wiring (below truncation).
    bind();
}

// 0x382b0 — __ZN10RobloxView22defineConcurrencyRulesEv
// demangled: RobloxView::defineConcurrencyRules(void)
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::defineConcurrencyRules(void)")]
pub fn stub_382b0(define: &mut dyn FnMut()) {
    // IDA 0x382b0: RobloxView::defineConcurrencyRules — ViewUpdateJob + mutex rules (below truncation).
    define();
}

// 0x386d0 — __ZN10RobloxView16restartDataModelEv
// demangled: RobloxView::restartDataModel(void)
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::restartDataModel(void)")]
pub fn stub_386d0(restart: &mut dyn FnMut(), dispatch: &mut dyn FnMut(&mut dyn FnMut())) {
    // IDA 0x386d0: dispatch_async(main, ^{ doRestartDataModel(); }).
    dispatch(restart);
}

// 0x38720 — __ZN10RobloxView15newGameDidStartEv
// demangled: RobloxView::newGameDidStart(void)
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::newGameDidStart(void)")]
pub fn stub_38720(start: &mut dyn FnMut(), dispatch: &mut dyn FnMut(&mut dyn FnMut())) {
    // IDA 0x38720: dispatch_async(main, ^{ newGameDidStart body }).
    dispatch(start);
}

// 0x38770 — ____ZN10RobloxView18doRestartDataModelEv_block_invoke
// demangled: ____ZN10RobloxView18doRestartDataModelEv_block_invoke
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detai
// was: boost::shared_ptr
#[doc(alias = "____ZN10RobloxView18doRestartDataModelEv_block_invoke")]
pub fn stub_38770(view: usize, restart: &mut dyn FnMut(usize)) {
    // IDA 0x38770: doRestartDataModel block — runs on main (below truncation).
    restart(view);
}

// 0x38cd0 — __ZN10RobloxView17setupNewDataModelEv
// demangled: RobloxView::setupNewDataModel(void)
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::setupNewDataModel(void)")]
pub fn stub_38cd0(setup: &mut dyn FnMut()) {
    // IDA 0x38cd0: RobloxView::setupNewDataModel — Game wiring (below truncation).
    setup();
}

// 0x39018 — ____ZN10RobloxView15newGameDidStartEv_block_invoke
// demangled: ____ZN10RobloxView15newGameDidStartEv_block_invoke
// type: 
#[doc(alias = "____ZN10RobloxView15newGameDidStartEv_block_invoke")]
pub fn stub_39018(view: usize, resume: &mut dyn FnMut(usize)) {
    // IDA 0x39018: requestResumeRendering(captured view).
    resume(view);
}

// 0x39020 — __ZN10RobloxViewD1Ev
// demangled: RobloxView::~RobloxView()
// type: void __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::~RobloxView()")]
pub fn stub_39020(destroy: &mut dyn FnMut()) {
    // IDA 0x39020: D1 thunk tail-calls D2.
    destroy();
}

// 0x39024 — __ZN10RobloxViewD2Ev
// demangled: RobloxView::~RobloxView()
// type: void __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::~RobloxView()")]
pub fn stub_39024(destroy: &mut dyn FnMut()) {
    // IDA 0x39024: ~RobloxView D2 — RenderJob/FunctionMarshaller releases (below truncation).
    destroy();
}

// 0x39674 — __ZN10RobloxView11create_viewEN5boost10shared_ptrIN3RBX4GameEEEjjSsSsSs
// demangled: RobloxView::create_view(boost::shared_ptr<RBX::Game>,unsigned int,unsigned int,std::string,std::string,std::string)
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, std::string *, std::string *)
// was: boost::shared_ptr
#[doc(alias = "RobloxView::create_view(rbx_core::SharedPtr<RBX::Game>,unsigned int,unsigned int,std::string,std::string,std::string)")]
pub fn stub_39674(create: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x39674: RobloxView::create_view — alloc + construct (below truncation).
    create()
}

// 0x39920 — __ZL14initLogManagerv
// demangled: initLogManager(void)
// type: _DWORD __fastcall()
#[doc(alias = "initLogManager(void)")]
pub fn stub_39920(init: &mut dyn FnMut()) {
    // IDA 0x39920: initLogManager — Ogre::LogManager setup (below truncation).
    init();
}

// 0x39be0 — __ZNSt12domain_errorD0Ev
// demangled: std::domain_error::~domain_error()
// type: void __cdecl(std::domain_error *__hidden this)
#[doc(alias = "std::domain_error::~domain_error()")]
pub fn stub_39be0(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x39be0: domain_error D0: logic_error dtor + operator delete.
    destroy();
    free();
}

// 0x39bf8 — __ZNSt12domain_errorD2Ev
// demangled: std::domain_error::~domain_error()
// type: void __cdecl(std::domain_error *__hidden this)
#[doc(alias = "std::domain_error::~domain_error()")]
pub fn stub_39bf8(destroy: &mut dyn FnMut()) {
    // IDA 0x39bf8: domain_error D1 thunk -> logic_error dtor.
    destroy();
}

// 0x39c00 — __ZNSt16invalid_argumentD1Ev
// demangled: std::invalid_argument::~invalid_argument()
// type: void __cdecl(std::invalid_argument *__hidden this)
#[doc(alias = "std::invalid_argument::~invalid_argument()")]
pub fn stub_39c00(destroy: &mut dyn FnMut()) {
    // IDA 0x39c00: invalid_argument D1 thunk -> logic_error dtor.
    destroy();
}

// 0x39c08 — __ZNSt12length_errorD0Ev
// demangled: std::length_error::~length_error()
// type: void __cdecl(std::length_error *__hidden this)
#[doc(alias = "std::length_error::~length_error()")]
pub fn stub_39c08(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x39c08: length_error D0: logic_error dtor + operator delete.
    destroy();
    free();
}

// 0x39c20 — __ZNSt12out_of_rangeD1Ev
// demangled: std::out_of_range::~out_of_range()
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
pub fn stub_39c20(destroy: &mut dyn FnMut()) {
    // IDA 0x39c20: out_of_range D1 thunk -> logic_error dtor.
    destroy();
}

// 0x39c28 — __ZNSt11range_errorD0Ev
// demangled: std::range_error::~range_error()
// type: void __cdecl(std::range_error *__hidden this)
#[doc(alias = "std::range_error::~range_error()")]
pub fn stub_39c28(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x39c28: range_error D0: runtime_error dtor + operator delete.
    destroy();
    free();
}

// 0x39c40 — __ZNSt11range_errorD2Ev
// demangled: std::range_error::~range_error()
// type: void __cdecl(std::range_error *__hidden this)
#[doc(alias = "std::range_error::~range_error()")]
pub fn stub_39c40(destroy: &mut dyn FnMut()) {
    // IDA 0x39c40: range_error D1 thunk -> runtime_error dtor.
    destroy();
}

// 0x39c48 — __ZNSt14overflow_errorD1Ev
// demangled: std::overflow_error::~overflow_error()
// type: void __cdecl(std::overflow_error *__hidden this)
#[doc(alias = "std::overflow_error::~overflow_error()")]
pub fn stub_39c48(destroy: &mut dyn FnMut()) {
    // IDA 0x39c48: overflow_error D1 thunk -> runtime_error dtor.
    destroy();
}

// 0x39c50 — __ZNSt15underflow_errorD0Ev
// demangled: std::underflow_error::~underflow_error()
// type: void __cdecl(std::underflow_error *__hidden this)
#[doc(alias = "std::underflow_error::~underflow_error()")]
pub fn stub_39c50(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x39c50: underflow_error D0: runtime_error dtor + operator delete.
    destroy();
    free();
}

// 0x39c68 — __ZNSt15underflow_errorD2Ev
// demangled: std::underflow_error::~underflow_error()
// type: void __cdecl(std::underflow_error *__hidden this)
#[doc(alias = "std::underflow_error::~underflow_error()")]
pub fn stub_39c68(destroy: &mut dyn FnMut()) {
    // IDA 0x39c68: underflow_error D1 thunk -> runtime_error dtor.
    destroy();
}

// 0x39c6c — __ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEENS1_8functionIFvvEEE
// demangled: RBX::TaskScheduler::removeBlocking(boost::shared_ptr<RBX::TaskScheduler::Job>,boost::function<void ()(void)>)
// type: void __fastcall(int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "RBX::TaskScheduler::removeBlocking(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,boost::function<void ()(void)>)")]
pub fn stub_39c6c(remove: &mut dyn FnMut()) {
    // IDA 0x39c6c: TaskScheduler::removeBlocking (below truncation).
    remove();
}

// 0x39d7c — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEE5resetEv
// demangled: boost::shared_ptr<RobloxView::RenderJob>::reset(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::reset(void)")]
pub fn stub_39d7c(job: &mut Option<usize>, release: &mut dyn FnMut(usize)) {
    // IDA 0x39d7c: shared_ptr<RenderJob>::reset — clear + release.
    if let Some(p) = job.take() {
        release(p);
    }
}

// 0x39e10 — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv
// demangled: boost::shared_ptr<RobloxView::ViewUpdateJob>::reset(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::reset(void)")]
pub fn stub_39e10(job: &mut Option<usize>, release: &mut dyn FnMut(usize)) {
    // IDA 0x39e10: shared_ptr<ViewUpdateJob>::reset — clear + release.
    if let Some(p) = job.take() {
        release(p);
    }
}

// 0x39ea8 — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_
// demangled: boost::shared_ptr<RobloxView::ViewUpdateJob>::operator=(boost::shared_ptr<RobloxView::ViewUpdateJob>&&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::operator=(rbx_core::SharedPtr<RobloxView::ViewUpdateJob>&&)")]
pub fn stub_39ea8(dst: &mut Option<usize>, src: &mut Option<usize>, release: &mut dyn FnMut(usize)) {
    // IDA 0x39ea8: move-assign — steal src pair (src zeroed); release old dst.
    let old = std::mem::replace(dst, src.take());
    if let Some(p) = old {
        release(p);
    }
}

// 0x39f4c — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_
// demangled: boost::shared_ptr<RobloxView::ViewUpdateJob>::shared_ptr<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)
// type: int __fastcall(int, void *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::shared_ptr<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
pub fn stub_39f4c(make: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x39f4c: shared_ptr<ViewUpdateJob> construct (below truncation).
    make()
}

// 0x3a030 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEaSEOS3_
// demangled: boost::shared_ptr<RobloxView::RenderJob>::operator=(boost::shared_ptr<RobloxView::RenderJob>&&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::operator=(rbx_core::SharedPtr<RobloxView::RenderJob>&&)")]
pub fn stub_3a030(dst: &mut Option<usize>, src: &mut Option<usize>, release: &mut dyn FnMut(usize)) {
    // IDA 0x3a030: move-assign — steal src pair (src zeroed); release old dst.
    let old = std::mem::replace(dst, src.take());
    if let Some(p) = old {
        release(p);
    }
}

// 0x3a0d4 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEC1IS2_EEPT_
// demangled: boost::shared_ptr<RobloxView::RenderJob>::shared_ptr<RobloxView::RenderJob>(RobloxView::RenderJob *)
// type: int __fastcall(int, void *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::shared_ptr<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
pub fn stub_3a0d4(make: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x3a0d4: shared_ptr<RenderJob> construct (below truncation).
    make()
}

// 0x3a1b8 — __ZN17QuitEventListenerD1Ev
// demangled: QuitEventListener::~QuitEventListener()
// type: void __fastcall(QuitEventListener *__hidden this)
#[doc(alias = "QuitEventListener::~QuitEventListener()")]
pub fn stub_3a1b8() {
    // IDA 0x3a1b8: empty QuitEventListener dtor body.
}

// 0x3a1bc — __ZN5boost10shared_ptrIN3RBX4GameEEaSERKS3_
// demangled: boost::shared_ptr<RBX::Game>::operator=(boost::shared_ptr<RBX::Game> const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::Game>::operator=(rbx_core::SharedPtr<RBX::Game> const&)")]
pub fn stub_3a1bc(dst: &mut Option<usize>, src: Option<usize>, retain: &mut dyn FnMut(usize), release: &mut dyn FnMut(usize)) {
    // IDA 0x3a1bc: shared_ptr<Game> copy-assign — retain src; release old (below truncation).
    if let Some(s) = src {
        retain(s);
    }
    let old = std::mem::replace(dst, src);
    if let Some(p) = old {
        release(p);
    }
}

// 0x3a278 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>> const&)
// type: int(void)
// was: boost::shared_ptr
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost:")]
pub fn stub_3a278(slots: &mut Vec<SignalSlotConn>, target: usize) -> u64 {
    // IDA 0x3a278: operator new islot; callable ctor; signal::insert; connection (mf1 flavor).
    let id = slots.len() as u64;
    slots.push(SignalSlotConn { id, target, live: true });
    id
}

// 0x3a2ec — __ZN5boost10shared_ptrIN3RBX9DataModelEEaSINS1_16OverlayDataModelEEERS3_ONS0_IT_EE
// demangled: boost::shared_ptr<RBX::DataModel>& boost::shared_ptr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(boost::shared_ptr<RBX::OverlayDataModel> &&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>& rbx_core::SharedPtr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(rbx_core::SharedPtr<RBX::OverlayDataModel> &&)")]
pub fn stub_3a2ec(dst: &mut Option<usize>, src: &mut Option<usize>, release: &mut dyn FnMut(usize)) {
    // IDA 0x3a2ec: move-assign — steal src pair (src zeroed); release old dst.
    let old = std::mem::replace(dst, src.take());
    if let Some(p) = old {
        release(p);
    }
}

// 0x3a390 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>> const&)
// type: int(void)
// was: boost::shared_ptr
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>> const&)")]
pub fn stub_3a390(slots: &mut Vec<SignalSlotConn>, target: usize) -> u64 {
    // IDA 0x3a390: operator new islot; callable ctor; signal::insert; connection (mf0 flavor).
    let id = slots.len() as u64;
    slots.push(SignalSlotConn { id, target, live: true });
    id
}

// 0x3a408 — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEE9singletonEv
// demangled: __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEE9singletonEv")]
pub fn stub_3a408(slot: &mut Option<usize>, init: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x3a408: CRenderSettingsItem singleton (below truncation).
    if let Some(v) = *slot {
        return v;
    }
    let v = init();
    *slot = Some(v);
    v
}

// 0x3a5bc — __ZN5boost10shared_ptrIN3RBX5Tasks8SequenceEE5resetIS3_EEvPT_
// demangled: void boost::shared_ptr<RBX::Tasks::Sequence>::reset<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)
// type: 
// was: boost::shared_ptr
#[doc(alias = "void rbx_core::SharedPtr<RBX::Tasks::Sequence>::reset<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)")]
pub fn stub_3a5bc(slot: &mut Option<usize>, value: Option<usize>, release: &mut dyn FnMut(usize)) {
    // IDA 0x3a5bc: shared_ptr<Sequence>::reset — release old, store new (below truncation).
    if let Some(p) = std::mem::replace(slot, value) {
        release(p);
    }
}

// 0x3a660 — __ZN5boost10shared_ptrIN3RBX8ViewBaseEE5resetEv
// demangled: boost::shared_ptr<RBX::ViewBase>::reset(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::ViewBase>::reset(void)")]
pub fn stub_3a660(job: &mut Option<usize>, release: &mut dyn FnMut(usize)) {
    // IDA 0x3a660: shared_ptr<ViewBase>::reset — clear + release.
    if let Some(p) = job.take() {
        release(p);
    }
}

// 0x3a6f8 — __ZN5boost13exception_ptrD1Ev
// demangled: boost::exception_ptr::~exception_ptr()
// type: void __fastcall(boost::exception_ptr *__hidden this)
// was: boost::shared_ptr
#[doc(alias = "boost::exception_ptr::~exception_ptr()")]
pub fn stub_3a6f8(slot: &mut Option<usize>, release: &mut dyn FnMut(usize)) {
    // IDA 0x3a6f8: exception_ptr dtor — release counted base.
    if let Some(p) = slot.take() {
        release(p);
    }
}

// 0x3a790 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev
// demangled: __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev")]
pub fn stub_3a790(destroy: &mut dyn FnMut()) {
    // IDA 0x3a790: Creator<Camera> D1 thunk tail-calls D2.
    destroy();
}

// 0x3a798 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6CameraEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)")]
pub fn stub_3a798(alloc: &mut dyn FnMut(usize) -> usize, construct: &mut dyn FnMut(usize), share: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x3a798: new(0x1DC); Camera::Camera; shared_ptr attach.
    let p = alloc(0x1DC);
    construct(p);
    share(p);
    p
}

// 0x3a850 — __ZN5boost6detail15sp_counted_base12weak_releaseEv
// demangled: boost::detail::sp_counted_base::weak_release(void)
// type: _DWORD __fastcall(boost::detail::sp_counted_base *__hidden this)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_base::weak_release(void)")]
pub fn stub_3a850(weak: &mut u32, destroy: &mut dyn FnMut()) {
    // IDA 0x3a850: spinlock; weak_refs--; dispose when it hits zero from one.
    let old = *weak;
    *weak = old.wrapping_sub(1);
    if old == 1 {
        destroy();
    }
}

// 0x3a930 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6CameraES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Camera,RBX::Camera>(boost::shared_ptr<RBX::Camera> const*,RBX::Camera *)const
// type: 
// was: boost::shared_ptr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Camera,RBX::Camera>(rbx_core::SharedPtr<RBX::Camera> const*,RBX::Camera *)const")]
pub fn stub_3a930(use_count: u32, adopt: &mut dyn FnMut(), share: &mut dyn FnMut()) {
    // IDA 0x3a930: weak_count::use_count gates the weak_this store (below truncation).
    if use_count == 0 {
        adopt();
    } else {
        share();
    }
}

// 0x3aa10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3aa10(block: usize, free: &mut dyn FnMut(usize)) {
    // IDA 0x3aa10: D0 thunk tail-calls operator delete.
    free(block);
}

// 0x3aa18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_3aa18(block: usize, type_name: &str) -> usize {
    // IDA 0x3aa18: match "N3RBX9CreatableINS_8InstanceEE7DeleterE" -> block + 16, else 0.
    if type_name == "N3RBX9CreatableINS_8InstanceEE7DeleterE" {
        block + 16
    } else {
        0
    }
}

// 0x3aa30 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_ESH_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::_Rb_tree_iterator<std::pair<RBX::Name const* const,")]
pub fn stub_3aa30(map: &mut CreatorMap, keys: &[String]) {
    // IDA 0x3aa30: _Rb_tree range erase.
    for k in keys {
        map.entries.remove(k);
    }
}

// 0x3aa90 — __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEED1Ev
// demangled: std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::~map()
// type: 
#[doc(alias = "std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::~map()")]
pub fn stub_3aa90(map: &mut CreatorMap) {
    // IDA 0x3aa90: ~map — erase all nodes.
    map.entries.clear();
}

// 0x3aaa0 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev
// demangled: __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev")]
pub fn stub_3aaa0(slot: usize, init: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x3aaa0: Creator<Camera> C2 — vtable + registration (below truncation).
    init(slot);
    slot
}

// 0x3acc8 — __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_
// demangled: std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::operator[](RBX::Name const* const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::operator[](RBX::Name const* const&)")]
pub fn stub_3acc8(map: &mut CreatorMap, key: String, make: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x3acc8: map operator[] — find or default-insert.
    if let Some(&v) = map.entries.get(&key) {
        return v;
    }
    let v = make();
    map.entries.insert(key, v);
    v
}

// 0x3ad20 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::pair<RBX::Name const* const,RBX::ICreator const*> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::pair<RBX::Name const* const,RBX::ICreato")]
pub fn stub_3ad20(map: &mut CreatorMap, key: String, value: usize) -> bool {
    // IDA 0x3ad20: unique insert; false when present.
    if map.entries.contains_key(&key) {
        return false;
    }
    map.entries.insert(key, value);
    true
}

// 0x3add8 — __ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v
// demangled: __ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v")]
pub fn stub_3add8(has_name: bool, null_name: usize, once: &mut dyn FnMut(), declared: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x3add8: null sRunService -> getNullName; else call_once(callDoDeclare) + doDeclare.
    if !has_name {
        return null_name;
    }
    once();
    declared()
}

// 0x3ae20 — __ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v
// demangled: __ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v
// type: 
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v")]
pub fn stub_3ae20(guard: &mut bool, cached: &mut usize, declare: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x3ae20: cxa_guard one-time Name::declare(sRunService).
    if !*guard {
        *cached = declare();
        *guard = true;
    }
    *cached
}

// 0x3af08 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10RunServiceEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RunService>(void)
// type: 
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RunService>(void)")]
pub fn stub_3af08(guard: &mut bool, index: &mut usize, alloc: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x3af08: guarded one-time ServiceProvider::newIndex<RunService>.
    if !*guard {
        *index = alloc();
        *guard = true;
    }
    *index
}

// 0x3afe0 — __ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_3afe0(slot: &mut Option<usize>, ptr: usize, make_count: &mut dyn FnMut(usize), accept: &mut dyn FnMut(usize)) {
    // IDA 0x3afe0: px store; shared_count attach; _internal_accept_owner when px set.
    *slot = Some(ptr);
    make_count(ptr);
    if ptr != 0 {
        accept(ptr);
    }
}

// 0x3b008 — __ZN5boost6detail12shared_countC2IPN3RBX10RunServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_3b008(alloc: &mut dyn FnMut(usize) -> usize, px: usize, init: &mut dyn FnMut(usize, usize)) -> usize {
    // IDA 0x3b008: operator new(0x14); use=weak=1; store px.
    let block = alloc(0x14);
    init(block, px);
    block
}

// 0x3b108 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3b108() {
    // IDA 0x3b108: empty sp_counted_impl_pd<RunService> D2 body.
}

// 0x3b110 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_3b110(px: usize, predelete: &mut dyn FnMut(usize) -> i32, destroy: &mut dyn FnMut(usize) -> i32) -> i32 {
    // IDA 0x3b110: predelete; null px -> result else virtual destroy.
    let r = predelete(px);
    if px != 0 {
        destroy(px)
    } else {
        r
    }
}

// 0x3b130 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_3b130(block: usize, type_name: &str) -> usize {
    // IDA 0x3b130: match "N3RBX9CreatableINS_8InstanceEE7DeleterE" -> block + 16, else 0.
    if type_name == "N3RBX9CreatableINS_8InstanceEE7DeleterE" {
        block + 16
    } else {
        0
    }
}

// 0x3b148 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_3b148(block: usize) -> usize {
    // IDA 0x3b148: return block + 16.
    block + 16
}

// 0x3b14c — __ZN5boost6detail12shared_countC2IN3RBX5Tasks8SequenceEEEPT_
// demangled: boost::detail::shared_count::shared_count<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)")]
pub fn stub_3b14c(alloc: &mut dyn FnMut(usize) -> usize, px: usize, init: &mut dyn FnMut(usize, usize)) -> usize {
    // IDA 0x3b14c: operator new(0x10); use=weak=1; store px.
    let block = alloc(0x10);
    init(block, px);
    block
}

// 0x3b268 — __ZN3RBX5Tasks11Coordinator9onPreStepEPNS_13TaskScheduler3JobE
// demangled: RBX::Tasks::Coordinator::onPreStep(RBX::TaskScheduler::Job *)
// type: void()
#[doc(alias = "RBX::Tasks::Coordinator::onPreStep(RBX::TaskScheduler::Job *)")]
pub fn stub_3b268() {
    // IDA 0x3b268: empty Coordinator::onPreStep body.
}

// 0x3b26c — __ZN3RBX5Tasks11Coordinator10onPostStepEPNS_13TaskScheduler3JobE
// demangled: RBX::Tasks::Coordinator::onPostStep(RBX::TaskScheduler::Job *)
// type: void()
#[doc(alias = "RBX::Tasks::Coordinator::onPostStep(RBX::TaskScheduler::Job *)")]
pub fn stub_3b26c() {
    // IDA 0x3b26c: empty Coordinator::onPostStep body.
}

// 0x3b270 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED1Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::~sp_counted_impl_p()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::~sp_counted_impl_p()")]
pub fn stub_3b270() {
    // IDA 0x3b270: empty sp_counted_impl_p<Sequence> D2 body.
}

// 0x3b274 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED0Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::~sp_counted_impl_p()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::~sp_counted_impl_p()")]
pub fn stub_3b274(block: usize, free: &mut dyn FnMut(usize)) {
    // IDA 0x3b274: D0 thunk tail-calls operator delete.
    free(block);
}

// 0x3b278 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE7disposeEv
// demangled: boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::dispose(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::dispose(void)")]
pub fn stub_3b278(px: usize, teardown: &mut dyn FnMut(usize)) {
    // IDA 0x3b278: Sequence dispose — vtable reset, member delete, mutex destroy (below truncation).
    if px != 0 {
        teardown(px);
    }
}

// 0x3b32c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::get_deleter(std::type_info const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::get_deleter(std::type_info const&)")]
pub fn stub_3b32c() -> usize {
    // IDA 0x3b32c: plain impl_p has no deleter -> 0.
    0
}

// 0x3b330 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::get_untyped_deleter(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::get_untyped_deleter(void)")]
pub fn stub_3b330() -> usize {
    // IDA 0x3b330: plain impl_p has no untyped deleter -> 0.
    0
}

// 0x3b334 — __ZN5boost6detail12shared_countC2IN3RBX5Tasks17ExclusiveSequenceEEEPT_
// demangled: boost::detail::shared_count::shared_count<RBX::Tasks::ExclusiveSequence>(RBX::Tasks::ExclusiveSequence *)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tasks::ExclusiveSequence>(RBX::Tasks::ExclusiveSequence *)")]
pub fn stub_3b334(alloc: &mut dyn FnMut(usize) -> usize, px: usize, init: &mut dyn FnMut(usize, usize)) -> usize {
    // IDA 0x3b334: operator new(0x10); use=weak=1; store px.
    let block = alloc(0x10);
    init(block, px);
    block
}

// 0x3b450 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED1Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::~sp_counted_impl_p()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::~sp_counted_impl_p()")]
pub fn stub_3b450() {
    // IDA 0x3b450: empty sp_counted_impl_p<ExclusiveSequence> D2 body.
}

// 0x3b454 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED0Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::~sp_counted_impl_p()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::~sp_counted_impl_p()")]
pub fn stub_3b454(block: usize, free: &mut dyn FnMut(usize)) {
    // IDA 0x3b454: D0 thunk tail-calls operator delete.
    free(block);
}

// 0x3b458 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE7disposeEv
// demangled: boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::dispose(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::dispose(void)")]
pub fn stub_3b458(px: usize, teardown: &mut dyn FnMut(usize)) {
    // IDA 0x3b458: ExclusiveSequence dispose — vtable reset, member delete, mutex destroy (below truncation).
    if px != 0 {
        teardown(px);
    }
}

// 0x3b50c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::get_deleter(std::type_info const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::get_deleter(std::type_info const&)")]
pub fn stub_3b50c() -> usize {
    // IDA 0x3b50c: plain impl_p has no deleter -> 0.
    0
}

// 0x3b510 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::get_untyped_deleter(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::get_untyped_deleter(void)")]
pub fn stub_3b510() -> usize {
    // IDA 0x3b510: plain impl_p has no untyped deleter -> 0.
    0
}

// 0x3b518 — __ZNK3RBX15ServiceProvider4findINS_17ControllerServiceEEEPT_v
// demangled: RBX::ControllerService * RBX::ServiceProvider::find<RBX::ControllerService>(void)const
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::find<RBX::ControllerService>(void)const")]
pub fn stub_3b518(found: Option<usize>) -> Option<usize> {
    // IDA 0x3b518: ServiceProvider::find<ControllerService> (below truncation).
    found
}

// 0x3b674 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17ControllerServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)")]
pub fn stub_3b674(alloc: &mut dyn FnMut(usize) -> usize, construct: &mut dyn FnMut(usize), share: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x3b674: new(0x64); ControllerService::ControllerService; shared_ptr attach.
    let p = alloc(0x64);
    construct(p);
    share(p);
    p
}

// 0x3b724 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17ControllerServiceEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ControllerService>(boost::shared_ptr<RBX::ControllerService> const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ControllerService>(rbx_core::SharedPtr<RBX::ControllerService> const&)")]
pub fn stub_3b724(dst: &mut Option<usize>, src: Option<usize>, retain: &mut dyn FnMut(usize), release: &mut dyn FnMut(usize)) {
    // IDA 0x3b724: shared_ptr<Instance> copy-assign — retain src; release old (below truncation).
    if let Some(s) = src {
        retain(s);
    }
    let old = std::mem::replace(dst, src);
    if let Some(p) = old {
        release(p);
    }
}
