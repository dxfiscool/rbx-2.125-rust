//! reflection — generated_bg_8 — 150 stubs EA-sorted asc global gap filler 0x3219c..0x3b008 not yet in crates/reflection (global 85545 funcs, 63831 gaps reflection before; 21716->21866 distinct)
//! Source: ida/export.json (85545 funcs) global EA asc not in crates/reflection/src — next 150 uncovered for reflection-bg sorted asc after 0x32194
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// Gap-filler GuiService/LoginService factory + `finishTeleportHelper`
/// bind glue (IDA 0x32270-0x32194 tail). Only `functor_manager` typeinfo
/// answers and factory class names carry values; the rest is
/// closure/`Arc`/static-init glue.
/// typeinfo names for the managed `bind_t`s (cf. 0x2d644).
pub const BIND_OPEN_URL_TYPEINFO: &str = "bind_t<openUrlWindow,id,SEL,string>";
pub const BIND_FINISH_HELPER_TYPEINFO: &str = "bind_t<finishTeleportHelper,RobloxView*,SharedPtr<Game>>";
pub const BIND_VIEW_CHAR_TYPEINFO: &str = "bind_t<RobloxView*,signed char>";
pub const BIND_OBJC_SEL_TYPEINFO: &str = "bind_t<objc_object*,objc_selector*>";
pub const BIND_TELEPORT_IMPL_TYPEINFO: &str =
    "bind_t<teleportImpl,PlaceLauncher*,string,string,string>";
/// `RBX::Http` request state (IDA 0x33368): the URL string copied from the
/// `char const*`, header map zeroed, body left empty.
#[derive(Debug, Clone, Default)]
pub struct HttpRequestState {
    pub url: String,
    pub body: String,
}
/// `Teleporter` pending/dispatched request triples (IDA 0x33550/0x33d00):
/// `doTeleport` binds `teleportImpl` + submits to the marshaller, and
/// `teleportImpl` forwards the three strings to the launcher
/// `teleport:withAuthentication:withScript:`.
#[derive(Debug, Clone, Default)]
pub struct TeleportRequest {
    pub first: String,
    pub second: String,
    pub third: String,
}
pub(crate) static TELEPORT_SUBMITTED: std::sync::LazyLock<
    parking_lot::Mutex<Vec<TeleportRequest>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(Vec::new()));
pub(crate) static TELEPORT_DISPATCHED: std::sync::LazyLock<
    parking_lot::Mutex<Vec<TeleportRequest>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(Vec::new()));
/// `Reachability` notifier + factory state (IDA 0x3588c-0x35ce4): the
/// run-loop notifier flag and a handle counter for the
/// `reachabilityWith*` constructors. Flag/status queries
/// (`localWiFiStatusForFlags`, `networkStatusForFlags`,
/// `connectionRequired`, `currentReachabilityStatus`) are pure functions
/// of the `SCNetworkReachability` flags below.
pub(crate) static REACHABILITY_NOTIFIER: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static REACHABILITY_NEXT_HANDLE: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(1);
/// Last `RobloxAlert` message (IDA 0x35d3c/0x35e90): the factories
/// `dispatch_async` a block to the main queue that shows a `UIAlertView`
/// with this message plus an `Ok` button (0x35d8c); the show/release is
/// UIKit glue, the message records here.
pub(crate) static ROBLOX_ALERT_MESSAGE: std::sync::LazyLock<
    parking_lot::Mutex<String>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));
/// `RobloxInfo` URL state (IDA 0x36918-0x36e04): cached base/API/domain
/// URLs plus set/post/refresh counters. Mirrors the canonical
/// `rbx_platform::RobloxInfo` shapes (which owns the plist/settings
/// reads); the caches record here with matching normalization.
pub(crate) static INFO_BASE_URL: std::sync::LazyLock<
    parking_lot::Mutex<Option<String>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(None));
pub(crate) static INFO_API_BASE_URL: std::sync::LazyLock<
    parking_lot::Mutex<Option<String>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(None));
pub(crate) static INFO_DOMAIN: std::sync::LazyLock<
    parking_lot::Mutex<Option<String>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(None));
pub(crate) static INFO_BASE_URL_SETS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static INFO_BASE_URL_POSTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static INFO_SETTINGS_REFRESHES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// `https://api` + first-dot suffix of `base` (IDA 0x36a18-0x36a9e);
/// empty base stays nil (IDA 0x36a10).
fn info_api_base_url_for(base: &str) -> Option<String> {
    if base.is_empty() {
        return None;
    }
    let trimmed = base.trim_end_matches('/');
    let dot = trimmed.find('.')?;
    Some(format!("https://api{}", &trimmed[dot..]))
}
/// First-dot suffix of `base` minus scheme and `/` (IDA 0x36b30-0x36bb0);
/// empty base stays nil (IDA 0x36b06).
fn info_domain_string_for(base: &str) -> Option<String> {
    if base.is_empty() {
        return None;
    }
    let no_scheme = base.strip_prefix("http://").unwrap_or(base);
    let dot = no_scheme.find('.')?;
    Some(no_scheme[dot..].replace('/', ""))
}
/// Digit right after `token` (`characterAtIndex:loc+len`, non-digit
/// reads as 0, IDA 0x361fa-0x36228); no token reads 0.
fn info_digit_after(haystack: &str, token: &str) -> i32 {
    let pos = match haystack.find(token) {
        Some(pos) => pos + token.len(),
        None => return 0,
    };
    haystack[pos..].chars().next().and_then(|c| c.to_digit(10)).unwrap_or(0) as i32
}
/// `RobloxView` render state (IDA 0x37068/0x37378/0x37b3c): whether the
/// render + view-update jobs are currently scheduled, plus completed
/// view-prep count. Job add/remove traffic is scheduler glue.
pub(crate) static ROBLOXVIEW_RENDERING: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(true);
pub(crate) static ROBLOXVIEW_PREPS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// `RobloxView` constructor args (IDA 0x37628): dimensions plus the
/// three copied strings (place/auth/script); one-shot log/plugin init,
/// Ogre view creation and the update-job install are engine glue.
#[derive(Debug, Clone, Default)]
pub struct RobloxViewCreate {
    pub width: u32,
    pub height: u32,
    pub first: String,
    pub second: String,
    pub third: String,
}


/// `RobloxView` workspace/bind/restart state (IDA 0x380a4-0x39674):
/// bind/restart/start/dispatch counters plus the one-shot log-manager
/// flag. Scheduler/signal/lock traffic is engine glue.
pub(crate) static ROBLOXVIEW_BINDS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static ROBLOXVIEW_RULES: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static ROBLOXVIEW_RESTARTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static ROBLOXVIEW_DID_RESTART: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static ROBLOXVIEW_NEWGAMES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static ROBLOXVIEW_STARTED: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static ROBLOXVIEW_DATAMODELS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static ROBLOXVIEW_CREATES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOG_MANAGER_INIT: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
/// `Name -> ICreator` registry behind the `FactoryProduct` creators
/// (IDA 0x3aa30/0x3acc8/0x3ad20): class names in registration order.
/// `std::map`/`_Rb_tree` traffic is drop glue.
pub(crate) static CREATOR_NAMES: std::sync::LazyLock<
    parking_lot::Mutex<Vec<String>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(Vec::new()));
/// `RunService` declaration + class index (IDA 0x3ae20/0x3af08, cf.
/// `LOGIN_CLASS_INDEX` in bg_7).
pub(crate) static RUNSERVICE_DECLARED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static RUNSERVICE_CLASS_INDEX: std::sync::LazyLock<usize> =
    std::sync::LazyLock::new(|| 1);

// 0x3219c — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev")]
pub fn stub_0x3219c() {
    // IDA 0x3219c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x32270 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x32270(get_typeinfo: bool) -> &'static str {
    // IDA 0x32270: `functor_manager<bind_t<openUrl...>>::manage` answers op
    // 4 with the `bind_t` typeinfo (40 insns, `strcmp` traffic, same shape
    // as 0x2d644). Other ops are vtable glue.
    if get_typeinfo {
        BIND_OPEN_URL_TYPEINFO
    } else {
        ""
    }
}

// 0x322d0 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs")]
pub fn stub_0x322d0() {
    // IDA 0x322d0: `void_function_obj_invoker1<bind_t<openUrl...>>::invoke`
    // runs the bound `openUrlWindow:` slot (11 insns). Closure-call glue;
    // no explicit body.
}

// 0x322e8 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_SsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int, int, int *), const std::string **)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,std::string),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,std::string) &,boost::_bi::list1<std::string &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_SsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x322e8() {
    // IDA 0x322e8: `list3<id,SEL,arg<1>>::operator()` copies the string arg
    // and invokes `openUrlWindow:` (99 insns, cf. 0x25e00 GuiService
    // wiring). Closure-call glue; no explicit body.
}

// 0x32408 — __ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv")]
pub fn stub_0x32408() {
    // IDA 0x32408: `Name::callDoDeclare<sGuiService>` forwards to
    // `doDeclare` (1 insn). Trampoline glue; no explicit body.
}

// 0x3240c — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10GuiServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GuiService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10GuiServiceEEEvv")]
pub fn stub_0x3240c() {
    // IDA 0x3240c: `ServiceProvider::callDoGetClassIndex<GuiService>`
    // forwards to `doGetClassIndex` (1 insn). Trampoline glue; no explicit
    // body.
}

// 0x32410 — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x32410(constructed: bool) -> &'static str {
    // IDA 0x32410: `FactoryProduct<TaskSchedulerSettings,...>::Creator::
    // getClassName` asserts `wasConstructed()` (debug glue) and returns the
    // declared `TaskSchedulerSettings` name (decompiled 0x32410-0x32470).
    assert!(constructed, "wasConstructed() (IDA 0x32410)");
    "TaskSchedulerSettings"
}

// 0x3247c — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x3247c(constructed: bool, create_ok: bool) -> bool {
    // IDA 0x3247c: `FactoryProduct<TaskSchedulerSettings,...>::Creator::
    // create` asserts `wasConstructed()`, runs `Creatable::create` and
    // returns the new instance (decompiled 0x3247c-0x324fa). Factory glue;
    // presence collapses to `bool`.
    assert!(constructed, "wasConstructed() (IDA 0x3247c)");
    create_ok
}

// 0x324fc — __ZN5boost10shared_ptrIN3RBX21TaskSchedulerSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX21TaskSchedulerSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x324fc() {
    // IDA 0x324fc: `shared_ptr<TaskSchedulerSettings>::shared_ptr<...,
    // Creatable::Deleter>` stores the pointer + deleter (14 insns).
    // `Arc` construction glue covers it; no explicit body.
}

// 0x325fc — __ZN5boost6detail12shared_countC2IPN3RBX21TaskSchedulerSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX21TaskSchedulerSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x325fc() {
    // IDA 0x325fc: `shared_count::shared_count<TaskSchedulerSettings*,...>`
    // allocates the control block (`operator new`, 58 insns). `Arc`
    // construction glue covers it; no explicit body.
}

// 0x326fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x326fc() {
    // IDA 0x326fc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x32700 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x32700() {
    // IDA 0x32700: `sp_counted_impl_pd<TaskSchedulerSettings*,...>::
    // dispose` runs `Instance::predelete` then deletes (13 insns, same
    // shape as 0x31bf4). `Arc` drop glue covers it; no explicit body.
}

// 0x32720 — __ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v")]
pub fn stub_0x32720() {
    // IDA 0x32720: `Name::declare<sTaskSchedulerSettings>` one-shots the
    // class-name declaration (`call_once`, 20 insns). Idempotent declare
    // glue; no explicit body.
}

// 0x32764 — __ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv")]
pub fn stub_0x32764() {
    // IDA 0x32764: `Name::callDoDeclare<sTaskSchedulerSettings>` forwards
    // to `doDeclare` (1 insn). Trampoline glue; no explicit body.
}

// 0x32768 — __ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv")]
pub fn stub_0x32768(constructed: bool) -> &'static str {
    // IDA 0x32768: `FactoryProduct<ScriptContext,...>::Creator::getClassName`
    // asserts `wasConstructed()` and returns the declared `ScriptContext`
    // name (decompiled 0x32768-0x327c8, same shape as 0x32410).
    assert!(constructed, "wasConstructed() (IDA 0x32768)");
    "ScriptContext"
}

// 0x327d4 — __ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEES2_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<RobloxView *,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,RobloxView *,rbx_core::SharedPtr<RBX::Game>,RobloxView *,rbx_core::SharedPtr<RBX::Game>>(void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),RobloxView *,rbx_core::SharedPtr<RBX::Game>)")]
#[doc(alias = "__ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEES2_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_")]
pub fn stub_0x327d4() {
    // IDA 0x327d4: `bind(finishTeleportHelper, view, game)` packs the
    // `RobloxView*` + `SharedPtr<Game>` argument list (87 insns, cf.
    // 0x2aba4). `bind_t` packing is closure glue; no explicit body.
}

// 0x328bc — __ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_")]
pub fn stub_0x328bc() {
    // IDA 0x328bc: `list2<value<RobloxView*>,value<SharedPtr<Game>>>::list2`
    // copies the view + game control block (75 insns). Closure packing
    // glue; no explicit body.
}

// 0x32984 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x32984() {
    // IDA 0x32984: `function<void()>::function<bind_t<helper...>>` copies
    // the bindable into the function object (87 insns). `Box<dyn Fn>`
    // construction glue; no explicit body.
}

// 0x32a68 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x32a68() {
    // IDA 0x32a68: `function0<void>::function0<bind_t<helper...>>` copies
    // the bindable and stores it via `assign_to` (89 insns, same shape as
    // 0x2f0f0). `Box<dyn Fn()>` construction glue; no explicit body.
}

// 0x32b50 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
#[doc(alias = "__ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEEvT_")]
pub fn stub_0x32b50() {
    // IDA 0x32b50: `function0<void>::assign_to<bind_t<helper...>>` copies
    // the bindable into the vtable-managed buffer (94 insns, same shape as
    // 0x2f1d8). `Box<dyn Fn()>` assignment glue; no explicit body.
}

// 0x32c48 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x32c48(get_typeinfo: bool) -> &'static str {
    // IDA 0x32c48: `functor_manager<bind_t<helper...>>::manage` answers op 4
    // with the `bind_t` typeinfo (11 insns, same shape as 0x2f2d0). Other
    // ops are vtable glue.
    if get_typeinfo {
        BIND_FINISH_HELPER_TYPEINFO
    } else {
        ""
    }
}

// 0x32c64 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_0x32c64() {
    // IDA 0x32c64: `void_function_obj_invoker0<bind_t<helper...>>::invoke`
    // runs the bound helper (9 insns). Closure-call glue; no explicit body.
}

// 0x32c78 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x32c78() -> bool {
    // IDA 0x32c78: `basic_vtable0<void>::assign_to<bind_t<helper...>>`
    // copies the bindable into the caller's buffer and delegates to the
    // heap variant (0x32d60, 89 insns, same shape as 0x2f300). True.
    true
}

// 0x32d60 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x32d60() -> bool {
    // IDA 0x32d60: `basic_vtable0<void>::assign_to<bind_t<helper...>>
    // (function_obj_tag)` heap-clones the bindable (`operator new`, 109
    // insns, same shape as 0x2f3e8). `Box::new` always fits; true.
    true
}

// 0x32e74 — __ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x32e74() {
    // IDA 0x32e74: `list2<RobloxView*,SharedPtr<Game>>::operator()` unwraps
    // view + game and calls `finishTeleportHelper` (81 insns, cf. 0x2b754).
    // Closure-call glue; no explicit body.
}

// 0x32f4c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x32f4c(get_typeinfo: bool) -> &'static str {
    // IDA 0x32f4c: `functor_manager<bind_t<helper...>>::manager` clones /
    // destroys through the buffer vtable and answers the typeinfo query
    // (121 insns, same shape as 0x2f5d4). Other ops are vtable glue.
    if get_typeinfo {
        BIND_FINISH_HELPER_TYPEINFO
    } else {
        ""
    }
}

// 0x33080 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEENS1_26device_close_all_operationIS5_EENSD_ISB_EEEENS1_14execute_traitsIT_NS_9result_ofIFSH_vEE4typeEE11result_typeESH_T0_T1_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>)")]
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEENS1_26device_close_all_operationIS5_EENSD_ISB_EEEENS1_14execute_traitsIT_NS_9result_ofIFSH_vEE4typeEE11result_typeESH_T0_T1_")]
pub fn stub_0x33080() {
    // IDA 0x33080: `iostreams::execute_all<copy_operation<istream,
    // ostringstream>>` pumps the fetched URL bytes into the script buffer
    // with nested try/catch (101 insns, cf. 0x2ba54). Stream-copy glue for
    // the content fetch; no explicit body.
}

// 0x33188 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEENS1_26device_close_all_operationIS5_EEEENS1_14execute_traitsIT_NS_9result_ofIFSG_vEE4typeEE11result_typeESG_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>)")]
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEENS1_26device_close_all_operationIS5_EEEENS1_14execute_traitsIT_NS_9result_ofIFSG_vEE4typeEE11result_typeESG_T0_")]
pub fn stub_0x33188() {
    // IDA 0x33188: `iostreams::execute_all<copy_operation<istream,
    // ostringstream>, device_close_all_operation>` overload pumps the
    // fetched bytes into the script buffer (same shape as 0x33080).
    // Stream-copy glue for the content fetch; no explicit body.
}

// 0x33250 — __ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESH_
// type: int __fastcall(int, int, unsigned int, int, int, void *, int, int, int, int)
#[doc(alias = "int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>(boost::reference_wrapper<std::istream> &,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESH_")]
pub fn stub_0x33250() {
    // IDA 0x33250: `iostreams::copy_impl<istream, ostringstream>` reads the
    // source in chunks (`operator new` buffer, 0x33282-0x33332) and writes
    // each chunk into the destination. Stream-copy glue; no explicit body.
}

// 0x33368 — __ZN3RBX4HttpC2EPKc
// type: RBX::Http *__fastcall(RBX::Http *this, const char *)
#[doc(alias = "RBX::Http::Http(char const*)")]
#[doc(alias = "__ZN3RBX4HttpC2EPKc")]
pub fn stub_0x33368(url: &str) -> HttpRequestState {
    // IDA 0x33368: `RBX::Http::Http(char const*)` stores the default API
    // endpoint, copies the URL string (0x333e6) and zeroes the header map
    // with an empty body (0x333f4-0x33406).
    HttpRequestState { url: url.to_owned(), body: String::new() }
}

// 0x33454 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x33454() {
    // IDA 0x33454: `sp_counted_impl_pd<Players*,Creatable::Deleter>::
    // get_deleter` answers the deleter query by `type_info` (vtable glue,
    // same shape as 0x31bf4 family). `Arc` drop glue covers it; no
    // explicit body.
}

// 0x3346c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x3346c() {
    // IDA 0x3346c: `sp_counted_impl_pd<Players*,Creatable::Deleter>::
    // get_untyped_deleter` returns the untyped deleter address (vtable
    // glue). `Arc` drop glue covers it; no explicit body.
}

// 0x33470 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewaENS3_5list2INS3_5valueIS6_EENSA_IaEEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewaENS3_5list2INS3_5valueIS6_EENSA_IaEEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x33470(get_typeinfo: bool) -> &'static str {
    // IDA 0x33470: `functor_manager<bind_t<RobloxView*,signed char>>::
    // manage` answers op 4 with the `bind_t` typeinfo (same shape as
    // 0x32c48). Other ops are vtable glue.
    if get_typeinfo { BIND_VIEW_CHAR_TYPEINFO } else { "" }
}

// 0x334d0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP10RobloxViewaENS3_5list2INS3_5valueIS6_EENSA_IaEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP10RobloxViewaENS3_5list2INS3_5valueIS6_EENSA_IaEEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_0x334d0() {
    // IDA 0x334d0: `void_function_obj_invoker0<bind_t<RobloxView*,signed
    // char>>::invoke` runs the bound slot (same shape as 0x32c64).
    // Closure-call glue; no explicit body.
}

// 0x334dc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorENS3_5list2INS3_5valueIS6_EENSB_IS7_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *),boost::_bi::list2<boost::_bi::value<objc_object *>,boost::_bi::list2<objc_selector>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *),boost::_bi::list2<boost::_bi::value<objc_object *>,boost::_bi::list2<objc_selector>>>>&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorENS3_5list2INS3_5valueIS6_EENSB_IS7_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x334dc(get_typeinfo: bool) -> &'static str {
    // IDA 0x334dc: `functor_manager<bind_t<objc_object*,objc_selector*>>::
    // manage` answers op 4 with the `bind_t` typeinfo (same shape as
    // 0x32270). Other ops are vtable glue.
    if get_typeinfo { BIND_OBJC_SEL_TYPEINFO } else { "" }
}

// 0x3353c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorENS3_5list2INS3_5valueIS6_EENSB_IS7_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *),boost::_bi::list2<boost::_bi::value<objc_object *>,boost::_bi::list2<objc_selector>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorENS3_5list2INS3_5valueIS6_EENSB_IS7_EEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_0x3353c() {
    // IDA 0x3353c: `void_function_obj_invoker0<bind_t<objc_object*,
    // objc_selector*>>::invoke` runs the bound slot. Closure-call glue;
    // no explicit body.
}

// 0x33548 — __ZN10TeleporterD1Ev
// type: void __fastcall(Teleporter *__hidden this)
#[doc(alias = "Teleporter::~Teleporter()")]
#[doc(alias = "__ZN10TeleporterD1Ev")]
pub fn stub_0x33548() {
    // IDA 0x33548: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3354c — __ZN10TeleporterD0Ev
// type: void __fastcall(Teleporter *__hidden this)
#[doc(alias = "Teleporter::~Teleporter()")]
#[doc(alias = "__ZN10TeleporterD0Ev")]
pub fn stub_0x3354c() {
    // IDA 0x3354c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x33550 — __ZN10Teleporter10doTeleportERKSsS1_S1_
// type: _DWORD __fastcall(Teleporter *__hidden this, const std::string *, const std::string *, const std::string *)
#[doc(alias = "Teleporter::doTeleport(std::string const&,std::string const&,std::string const&)")]
#[doc(alias = "__ZN10Teleporter10doTeleportERKSsS1_S1_")]
pub fn stub_0x33550(first: &str, second: &str, third: &str) {
    // IDA 0x33550: `Teleporter::doTeleport` copies the three strings
    // (0x3357a-0x335be), binds `teleportImpl` with the launcher
    // (0x335e0) and submits it to the marshaller (0x335f8); the tail
    // releases the temporaries. The submit records here.
    TELEPORT_SUBMITTED.lock().push(TeleportRequest {
        first: first.to_owned(),
        second: second.to_owned(),
        third: third.to_owned(),
    });
}

// 0x33920 — __ZNK10Teleporter17isTeleportEnabledEv
// type: _DWORD __fastcall(Teleporter *__hidden this)
#[doc(alias = "Teleporter::isTeleportEnabled(void)const")]
#[doc(alias = "__ZNK10Teleporter17isTeleportEnabledEv")]
pub fn stub_0x33920() -> bool {
    // IDA 0x33920: `Teleporter::isTeleportEnabled` returns 1 (0x33922).
    true
}

// 0x33924 — __ZN5boost4bindIvP13PlaceLauncherSsSsSsS2_SsSsSsEENS_3_bi6bind_tIT_PFS5_T0_T1_T2_T3_ENS3_9list_av_4IT4_T5_T6_T7_E4typeEEESB_SD_SE_SF_SG_
// type: int __fastcall(int, int, int, std::string *, std::string *, std::string *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list_av_4<PlaceLauncher *,std::string,std::string,std::string>::type> boost::bind<void,PlaceLauncher *,std::string,std::string,std::string,PlaceLauncher *,std::string,std::string,std::string>(void (*)(PlaceLauncher *,std::string,std::string,std::string),PlaceLauncher *,std::string,std::string,std::string)")]
#[doc(alias = "__ZN5boost4bindIvP13PlaceLauncherSsSsSsS2_SsSsSsEENS_3_bi6bind_tIT_PFS5_T0_T1_T2_T3_ENS3_9list_av_4IT4_T5_T6_T7_E4typeEEESB_SD_SE_SF_SG_")]
pub fn stub_0x33924() {
    // IDA 0x33924: `bind(teleportImpl, launcher, strings...)` packs the
    // `PlaceLauncher*` + three-string argument list (0x33924-0x33cf8).
    // `bind_t` packing is closure glue; no explicit body.
}

// 0x33d00 — __ZN10Teleporter12teleportImplEP13PlaceLauncherSsSsSs
#[doc(alias = "Teleporter::teleportImpl(PlaceLauncher *,std::string,std::string,std::string)")]
#[doc(alias = "__ZN10Teleporter12teleportImplEP13PlaceLauncherSsSsSs")]
pub fn stub_0x33d00(first: &str, second: &str, third: &str) {
    // IDA 0x33d00: `Teleporter::teleportImpl` converts the three strings
    // to `NSString` (0x33d32-0x33d8a) and forwards them to the launcher
    // `teleport:withAuthentication:withScript:` (0x33dac). The forward
    // records here; the `NSString` traffic is drop glue.
    TELEPORT_DISPATCHED.lock().push(TeleportRequest {
        first: first.to_owned(),
        second: second.to_owned(),
        third: third.to_owned(),
    });
}

// 0x33db0 — __ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EC2ES5_S6_S6_S6_
// type: int __fastcall(int, int, std::string *, int, std::string *)
#[doc(alias = "boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::list4(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EC2ES5_S6_S6_S6_")]
pub fn stub_0x33db0() {
    // IDA 0x33db0: `list4<value<PlaceLauncher*>,value<string>x3>::list4`
    // copies the launcher + three strings into the bind argument list.
    // Closure packing glue; no explicit body.
}

// 0x33fe0 — __ZN5boost3_bi8storage4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EC2ES5_S6_S6_S6_
// type: int __fastcall(int, int, std::string *, int, std::string *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage4(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EC2ES5_S6_S6_S6_")]
pub fn stub_0x33fe0() {
    // IDA 0x33fe0: `storage4<value<PlaceLauncher*>,value<string>x3>::
    // storage4` copies the four values plus the string control blocks.
    // Closure packing glue; no explicit body.
}

// 0x341ac — __ZN5boost3_bi8storage3INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_EC2ES5_S6_S6_
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_EC2ES5_S6_S6_")]
pub fn stub_0x341ac() {
    // IDA 0x341ac: `storage3<value<PlaceLauncher*>,value<string>x2>::
    // storage3` copies the three values plus the string control blocks
    // (partial-bind path). Closure packing glue; no explicit body.
}

// 0x342f4 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x342f4() {
    // IDA 0x342f4: `function<void()>::function<bind_t<teleportImpl...>>`
    // copies the bindable into the function object (same shape as
    // 0x32984). `Box<dyn Fn>` construction glue; no explicit body.
}

// 0x345b0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x345b0() {
    // IDA 0x345b0: `function0<void>::function0<bind_t<teleportImpl...>>`
    // copies the bindable and stores it via `assign_to` (same shape as
    // 0x32a68). `Box<dyn Fn()>` construction glue; no explicit body.
}

// 0x34870 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEEvT_
// type: int(void)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>)")]
#[doc(alias = "__ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEEvT_")]
pub fn stub_0x34870() {
    // IDA 0x34870: `function0<void>::assign_to<bind_t<teleportImpl...>>`
    // copies the bindable into the vtable-managed buffer (same shape as
    // 0x32b50). `Box<dyn Fn()>` assignment glue; no explicit body.
}

// 0x34b40 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x34b40(get_typeinfo: bool) -> &'static str {
    // IDA 0x34b40: `functor_manager<bind_t<teleportImpl...>>::manage`
    // answers op 4 with the `bind_t` typeinfo (same shape as 0x32c48).
    // Other ops are vtable glue.
    if get_typeinfo { BIND_TELEPORT_IMPL_TYPEINFO } else { "" }
}

// 0x34b5c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_0x34b5c() {
    // IDA 0x34b5c: `void_function_obj_invoker0<bind_t<teleportImpl...>>::
    // invoke` runs the bound teleport (same shape as 0x32c64).
    // Closure-call glue; no explicit body.
}

// 0x34b70 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEbT_RNS1_15function_bufferE
// type: int(void)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x34b70() -> bool {
    // IDA 0x34b70: `basic_vtable0<void>::assign_to<bind_t<teleportImpl...>>`
    // copies the bindable into the caller's buffer (same shape as
    // 0x32c78). True.
    true
}

// 0x34e30 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x34e30() -> bool {
    // IDA 0x34e30: `basic_vtable0<void>::assign_to<bind_t<teleportImpl...>>
    // (function_obj_tag)` heap-clones the bindable (same shape as
    // 0x32d60). `Box::new` always fits; true.
    true
}

// 0x350ec — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x350ec() {
    // IDA 0x350ec: `basic_vtable0<void>::assign_functor<bind_t<
    // teleportImpl...>>` stores the functor for the small-object path.
    // `Box<dyn Fn>` assignment glue; no explicit body.
}

// 0x35200 — __ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EclIPFvS4_SsSsSsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::operator()<void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(PlaceLauncher *,std::string,std::string,std::string) &,boost::_bi::list0 &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EclIPFvS4_SsSsSsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x35200() {
    // IDA 0x35200: `list4<PlaceLauncher*,string,string,string>::operator()`
    // unwraps the launcher + three strings and calls `teleportImpl`
    // (same shape as 0x32e74). Closure-call glue; no explicit body.
}

// 0x35438 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE7managerERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE7managerERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x35438(get_typeinfo: bool) -> &'static str {
    // IDA 0x35438: `functor_manager<bind_t<teleportImpl...>>::manager`
    // clones/destroys through the buffer vtable and answers the typeinfo
    // query (same shape as 0x32f4c). Other ops are vtable glue.
    if get_typeinfo { BIND_TELEPORT_IMPL_TYPEINFO } else { "" }
}

// 0x355c8 — __GLOBAL__I_a_8
#[doc(alias = "global constructor keyed to_a_8")]
#[doc(alias = "__GLOBAL__I_a_8")]
pub fn stub_0x355c8() {
    // IDA 0x355c8: `__GLOBAL__I_a_8` runs the `a_8` translation-unit static
    // initializers. Static-init glue; no explicit body.
}

// 0x3588c — -[Reachability startNotifier]
// type: char __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability startNotifier]")]
pub fn stub_0x3588c() -> bool {
    // IDA 0x3588c: `startNotifier` installs `ReachabilityCallback` via
    // `SCNetworkReachabilitySetCallback` and schedules the ref on the
    // current run loop (0x358ba-0x358e4), failing (0) when either call
    // fails. The schedule records here; the success path returns true.
    REACHABILITY_NOTIFIER.store(true, std::sync::atomic::Ordering::SeqCst);
    true
}

// 0x358ec — _ReachabilityCallback
// type: id __fastcall(int, int, int)
#[doc(alias = "_ReachabilityCallback")]
pub fn stub_0x358ec() {
    // IDA 0x358ec: `ReachabilityCallback` posts the
    // `kReachabilityChangedNotification` on flag changes. Notification
    // glue; no explicit body.
}

// 0x35970 — -[Reachability stopNotifier]
// type: void __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability stopNotifier]")]
pub fn stub_0x35970() {
    // IDA 0x35970: `stopNotifier` unschedules the ref from the run loop
    // and clears the callback. The unschedule records here.
    REACHABILITY_NOTIFIER.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x359a8 — -[Reachability dealloc]
// type: void __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability dealloc]")]
pub fn stub_0x359a8() {
    // IDA 0x359a8: `dealloc` stops the notifier, releases the
    // `SCNetworkReachabilityRef` and supers. Release is drop glue; the
    // notifier reset records here.
    REACHABILITY_NOTIFIER.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x35a00 — +[Reachability reachabilityWithHostName:]
// type: id __cdecl(id, SEL, id)
#[doc(alias = "+[Reachability reachabilityWithHostName:]")]
pub fn stub_0x35a00(_host: &str) -> usize {
    // IDA 0x35a00: `reachabilityWithHostName:` creates the
    // `SCNetworkReachabilityRef` for the hostname and inits a
    // `Reachability` with it. Allocation is drop glue; the handle
    // records here.
    REACHABILITY_NEXT_HANDLE.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

// 0x35a80 — +[Reachability reachabilityWithAddress:]
// type: id __cdecl(id, SEL, const sockaddr_in *)
#[doc(alias = "+[Reachability reachabilityWithAddress:]")]
pub fn stub_0x35a80() -> usize {
    // IDA 0x35a80: `reachabilityWithAddress:` inits a `Reachability`
    // with the given `sockaddr_in`. Allocation is drop glue; the handle
    // records here.
    REACHABILITY_NEXT_HANDLE.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

// 0x35af8 — +[Reachability reachabilityForInternetConnection]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[Reachability reachabilityForInternetConnection]")]
pub fn stub_0x35af8() -> usize {
    // IDA 0x35af8: `reachabilityForInternetConnection` inits a
    // `Reachability` with a zero `sockaddr_in`. Allocation is drop glue;
    // the handle records here.
    REACHABILITY_NEXT_HANDLE.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

// 0x35b44 — +[Reachability reachabilityForLocalWiFi]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[Reachability reachabilityForLocalWiFi]")]
pub fn stub_0x35b44() -> usize {
    // IDA 0x35b44: `reachabilityForLocalWiFi` inits a `Reachability`
    // with the link-local address plus a `localWiFiRef`. Allocation is
    // drop glue; the handle records here.
    REACHABILITY_NEXT_HANDLE.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

// 0x35ba8 — -[Reachability localWiFiStatusForFlags:]
// type: int __cdecl(Reachability *self, SEL, unsigned int)
#[doc(alias = "-[Reachability localWiFiStatusForFlags:]")]
pub fn stub_0x35ba8(flags: u32) -> u32 {
    // IDA 0x35ba8: `localWiFiStatusForFlags:` returns WiFi (1) only when
    // reachable (`kSCNetworkReachabilityFlagsReachable`) and direct
    // (`kSCNetworkReachabilityFlagsIsDirect`), i.e. `(flags &
    // 0x20002) == 0x20002` (0x35bcc).
    u32::from(flags & 0x20002 == 0x20002)
}

// 0x35bd0 — _PrintReachabilityFlags
#[doc(alias = "_PrintReachabilityFlags")]
pub fn stub_0x35bd0() {
    // IDA 0x35bd0: `PrintReachabilityFlags` logs the flag word. Log
    // glue; no explicit body.
}

// 0x35c6c — -[Reachability networkStatusForFlags:]
// type: int __cdecl(Reachability *self, SEL, unsigned int)
#[doc(alias = "-[Reachability networkStatusForFlags:]")]
pub fn stub_0x35c6c(flags: u32) -> u32 {
    // IDA 0x35c6c: `networkStatusForFlags:` maps the flag word to
    // `NotReachable` (0) / `ReachableViaWiFi` (1) / `ReachableViaWWAN`
    // (2): unreachable without bit 1 (0x35c88), WWAN with bit 18
    // (0x35cae), else reachable unless a connection is required without
    // an on-demand/on-traffic bypass (0x35c94-0x35cb2).
    if flags & 2 == 0 {
        return 0;
    }
    let mut reachable = flags & 4 == 0;
    if flags & 0x28 != 0 && flags & 0x10 == 0 {
        reachable = true;
    }
    if flags & 0x40000 != 0 {
        return 2;
    }
    u32::from(reachable)
}

// 0x35cb8 — -[Reachability connectionRequired]
// type: char __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability connectionRequired]")]
pub fn stub_0x35cb8(flags_ok: bool, flags: u32) -> bool {
    // IDA 0x35cb8: `connectionRequired` fetches the flags
    // (`SCNetworkReachabilityGetFlags`, 0x35cd2) and reports bit 2
    // (`kSCNetworkReachabilityFlagsConnectionRequired`, 0x35cdc);
    // a failed fetch reports false (0x35cd4).
    flags_ok && flags & 4 != 0
}

// 0x35ce4 — -[Reachability currentReachabilityStatus]
// type: int __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability currentReachabilityStatus]")]
pub fn stub_0x35ce4(flags_ok: bool, flags: u32, local_wifi: bool) -> u32 {
    // IDA 0x35ce4: `currentReachabilityStatus` fetches the flags
    // (0x35d00, 0 without them) and dispatches to
    // `localWiFiStatusForFlags:` when a `localWiFiRef` exists, else to
    // `networkStatusForFlags:` (0x35d14-0x35d32).
    if !flags_ok {
        return 0;
    }
    if local_wifi { stub_0x35ba8(flags) } else { stub_0x35c6c(flags) }
}

// 0x35d3c — +[RobloxAlert RobloxAlertWithMessage:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxAlert RobloxAlertWithMessage:]")]
pub fn stub_0x35d3c(message: &str) {
    // IDA 0x35d3c: `RobloxAlertWithMessage:` captures the message in a
    // stack block (0x35d70-0x35d80) and `dispatch_async`s it to the main
    // queue (0x35d82). The message records here; the block shows the
    // alert (0x35d8c).
    *ROBLOX_ALERT_MESSAGE.lock() = message.to_owned();
}

// 0x35d8c — ___38+[RobloxAlert RobloxAlertWithMessage:]_block_invoke
#[doc(alias = "___38+[RobloxAlert RobloxAlertWithMessage:]_block_invoke")]
pub fn stub_0x35d8c() {
    // IDA 0x35d8c: the `RobloxAlertWithMessage:` block builds a
    // `UIAlertView` with the localized title, the captured message and
    // an `Ok` button, shows it and releases it (0x35db4-0x35e5c).
    // UIKit show/release glue; the message already records at 0x35d3c.
}

// 0x35e7c — ___copy_helper_block__5
#[doc(alias = "___copy_helper_block__5")]
pub fn stub_0x35e7c() {
    // IDA 0x35e7c: `__copy_helper_block__5` retains the captured message
    // for the heap-promoted block. Retain is drop glue; no explicit body.
}

// 0x35e88 — ___destroy_helper_block__5
#[doc(alias = "___destroy_helper_block__5")]
pub fn stub_0x35e88() {
    // IDA 0x35e88: `__destroy_helper_block__5` releases the captured
    // message. Release is drop glue; no explicit body.
}

// 0x35e90 — +[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]
// type: void __cdecl(id, SEL, id, id)
#[doc(alias = "+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]")]
pub fn stub_0x35e90(message: &str) {
    // IDA 0x35e90: `RobloxAlertWithMessageAndDelegate:Delegate:`
    // captures the message plus delegate in a block and dispatches it
    // to the main queue (same shape as 0x35d3c). The message records
    // here; the delegate capture is drop glue.
    *ROBLOX_ALERT_MESSAGE.lock() = message.to_owned();
}

// 0x35ee4 — ___58+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]_block_invoke
#[doc(alias = "___58+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]_block_invoke")]
pub fn stub_0x35ee4() {
    // IDA 0x35ee4: the `...AndDelegate:` block shows the alert (same
    // shape as 0x35d8c). UIKit glue; the message already records at
    // 0x35e90.
}

// 0x35ffc — ___copy_helper_block_19
#[doc(alias = "___copy_helper_block_19")]
pub fn stub_0x35ffc() {
    // IDA 0x35ffc: `__copy_helper_block_19` retains the captured
    // message/delegate. Retain is drop glue; no explicit body.
}

// 0x36020 — ___destroy_helper_block_20
#[doc(alias = "___destroy_helper_block_20")]
pub fn stub_0x36020() {
    // IDA 0x36020: `__destroy_helper_block_20` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x3603c — __Z18getUserAgentStringv
// type: id __fastcall()
#[doc(alias = "getUserAgentString(void)")]
#[doc(alias = "__Z18getUserAgentStringv")]
pub fn stub_0x3603c(user_agent: &str) -> &str {
    // IDA 0x3603c: `getUserAgentString()` forwards to
    // `+[RobloxInfo getUserAgentString]` (0x3683c) and returns its
    // `NSString`. Forwarder glue; the assembled value passes through.
    user_agent
}

// 0x36058 — +[RobloxInfo getDeviceType]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getDeviceType]")]
pub fn stub_0x36058(device_type: Option<&str>) -> &'static str {
    // IDA 0x36058: `getDeviceType` maps `deviceType` to `iPad` /
    // `iPhone` / `iPod` / `Unknown` via `rangeOfString:`
    // (0x360a2-0x360f4); a nil `deviceType` returns `iPad`
    // (0x36104-0x36108).
    match device_type {
        None => "iPad",
        Some(t) if t.contains("iPad") => "iPad",
        Some(t) if t.contains("iPhone") => "iPhone",
        Some(t) if t.contains("iPod") => "iPod",
        Some(_) => "Unknown",
    }
}

// 0x36114 — +[RobloxInfo getDeviceModelNumber]
// type: int __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getDeviceModelNumber]")]
pub fn stub_0x36114(device_type: Option<&str>, tablet: bool) -> i32 {
    // IDA 0x36114: tablet reads `atoi` past `iPad` (-1 without it,
    // 0x3615e-0x36180); phone tries `iPod` first (0x36198-0x361a4),
    // else past `iPhone` (-1 without it, 0x361b6-0x361c2). A nil
    // `deviceType` reads 0 through the nil receiver (0x361e0-0x36208).
    let Some(device) = device_type else {
        return 0;
    };
    if tablet {
        if !device.contains("iPad") {
            return -1;
        }
        return info_digit_after(device, "iPad");
    }
    if device.contains("iPod") {
        return info_digit_after(device, "iPod");
    }
    if !device.contains("iPhone") {
        return -1;
    }
    info_digit_after(device, "iPhone")
}

// 0x3622c — +[RobloxInfo thisDeviceIsATablet]
// type: char __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo thisDeviceIsATablet]")]
pub fn stub_0x3622c(supports_idiom: bool, idiom: i32) -> bool {
    // IDA 0x3622c: `thisDeviceIsATablet` gates on
    // `respondsToSelector:userInterfaceIdiom` (0x3626c-0x36274); the Pad
    // idiom (1) survives the `!= 1 -> 0` fold (0x36282-0x3628a).
    supports_idiom && idiom == 1
}

// 0x36290 — +[RobloxInfo deviceType]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo deviceType]")]
pub fn stub_0x36290(machine: &str) -> String {
    // IDA 0x36290: `deviceType` wraps `sysctlbyname("hw.machine")` in a
    // string (0x362b2-0x362fa); the sysctl itself lives out of slice.
    machine.to_owned()
}

// 0x362fc — +[RobloxInfo deviceOSVersion]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo deviceOSVersion]")]
pub fn stub_0x362fc(version: &str) -> String {
    // IDA 0x362fc: `deviceOSVersion` returns `UIDevice.systemVersion`
    // (0x36318-0x36322).
    version.to_owned()
}

// 0x36330 — +[RobloxInfo appVersion]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo appVersion]")]
pub fn stub_0x36330(version: &str) -> String {
    // IDA 0x36330: `appVersion` returns
    // `objectForInfoDictionaryKey:CFBundleShortVersionString`
    // (0x3634c-0x36356).
    version.to_owned()
}

// 0x36370 — +[RobloxInfo friendlyDeviceName]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo friendlyDeviceName]")]
pub fn stub_0x36370(machine: &str) -> &'static str {
    // IDA 0x36370: `friendlyDeviceName` ladders `isEqualToString:` over
    // `hw.machine` (0x36390-0x36836).
    match machine {
        "iPhone1,1" => "iPhone 2G",
        "iPhone1,2" => "iPhone 3G",
        "iPhone2,1" => "iPhone 3GS",
        "iPhone3,1" | "iPhone3,2" => "iPhone 4",
        "iPhone3,3" => "iPhone 4 (CDMA)",
        "iPhone4,1" => "iPhone 4S",
        "iPhone5,1" => "iPhone 5",
        "iPhone5,2" => "iPhone 5 (GSM+CDMA)",
        "iPod1,1" => "iPod Touch (1 Gen)",
        "iPod2,1" => "iPod Touch (2 Gen)",
        "iPod3,1" => "iPod Touch (3 Gen)",
        "iPod4,1" => "iPod Touch (4 Gen)",
        "iPod5,1" => "iPod Touch (5 Gen)",
        "iPad1,1" => "iPad",
        "iPad1,2" => "iPad 3G",
        "iPad2,1" => "iPad 2 (WiFi)",
        "iPad2,2" | "iPad2,4" => "iPad 2",
        "iPad2,3" => "iPad 2 (CDMA)",
        "iPad2,5" => "iPad Mini (WiFi)",
        "iPad2,6" => "iPad Mini",
        "iPad2,7" => "iPad Mini (GSM+CDMA)",
        "iPad3,1" => "iPad 3 (WiFi)",
        "iPad3,2" => "iPad 3 (GSM+CDMA)",
        "iPad3,3" => "iPad 3",
        "iPad3,4" => "iPad 4 (WiFi)",
        "iPad3,5" => "iPad 4",
        "iPad3,6" => "iPad 4 (GSM+CDMA)",
        "i386" => "Simulator 32 bit intel",
        "x86_64" => "Simulator 64 bit intel",
        _ => "Unknown",
    }
}

// 0x3683c — +[RobloxInfo getUserAgentString]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getUserAgentString]")]
pub fn stub_0x3683c(model: &str, device_type: &str, os_version: &str, app_version: &str) -> String {
    // IDA 0x3683c: `getUserAgentString` formats `model`, `deviceType`,
    // `systemVersion` and `CFBundleShortVersionString` into the
    // Mozilla/5.0 template (0x36870-0x36914).
    format!(
        "Mozilla/5.0 ({model}; {device_type}; CPU iPhone OS {os_version} like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Mobile/9B176 ROBLOX iOS App {app_version}"
    )
}

// 0x36918 — +[RobloxInfo getBaseUrl]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getBaseUrl]")]
pub fn stub_0x36918(plist_url: &str) -> String {
    // IDA 0x36918: `getBaseUrl` returns the cached base URL
    // (`dword_130C460`, 0x36926-0x3692c); on a miss it stores the
    // `RbxBaseUrl`/`RbxBaseMobileUrl` plist value via `setBaseUrl:`
    // (0x36988-0x369b6). The plist value crosses as a parameter here.
    if let Some(cached) = INFO_BASE_URL.lock().clone() {
        return cached;
    }
    stub_0x36bd4(plist_url)
}

// 0x369c0 — +[RobloxInfo getApiBaseUrl]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getApiBaseUrl]")]
pub fn stub_0x369c0(base: &str) -> Option<String> {
    // IDA 0x369c0: `getApiBaseUrl` returns the cached API URL
    // (`dword_130C464`, 0x369d4-0x36aac), derived as `https://api` +
    // first-dot suffix on a miss.
    if let Some(cached) = INFO_API_BASE_URL.lock().clone() {
        return Some(cached);
    }
    let url = info_api_base_url_for(base)?;
    *INFO_API_BASE_URL.lock() = Some(url.clone());
    Some(url)
}

// 0x36ab0 — +[RobloxInfo getDomainString]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getDomainString]")]
pub fn stub_0x36ab0(base: &str) -> Option<String> {
    // IDA 0x36ab0: `getDomainString` returns the cached domain
    // (`dword_130C468`, 0x36aca-0x36bc6), derived as the first-dot
    // suffix on a miss.
    if let Some(cached) = INFO_DOMAIN.lock().clone() {
        return Some(cached);
    }
    let domain = info_domain_string_for(base)?;
    *INFO_DOMAIN.lock() = Some(domain.clone());
    Some(domain)
}

// 0x36bc8 — +[RobloxInfo getBaseUrlChangedNotification]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getBaseUrlChangedNotification]")]
pub fn stub_0x36bc8() -> &'static str {
    // IDA 0x36bc8: `getBaseUrlChangedNotification` returns the
    // `RBXBaseUrlChangedNotifier` name (0x36bd2).
    "RBXBaseUrlChangedNotifier"
}

// 0x36bd4 — +[RobloxInfo setBaseUrl:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxInfo setBaseUrl:]")]
pub fn stub_0x36bd4(url: &str) -> String {
    // IDA 0x36bd4: `setBaseUrl:` stores the base URL (0x36c08),
    // normalizing a trailing `/` (0x36c48-0x36c70), pushes it through
    // `SetBaseURL` (0x36c86-0x36c9e), dispatches the settings refresh
    // (0x36cce), posts `RBXBaseUrlChangedNotifier` (0x36cf0-0x36d12)
    // and initializes analytics (0x36d30). The store + counters record
    // here; the UTF-8 rep dance is drop glue.
    let normalized = if url.ends_with('/') {
        url.to_owned()
    } else {
        format!("{url}/")
    };
    *INFO_BASE_URL.lock() = Some(normalized.clone());
    INFO_BASE_URL_SETS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    stub_0x36de4();
    INFO_BASE_URL_POSTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    normalized
}

// 0x36de4 — ___25+[RobloxInfo setBaseUrl:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___25+[RobloxInfo setBaseUrl:]_block_invoke")]
pub fn stub_0x36de4() {
    // IDA 0x36de4: the `setBaseUrl:` block refreshes the iOS settings
    // service without a forced web read
    // (`getiOSSettingsServiceWithForcedReadFromWeb:NO`, 0x36dfe). The
    // refresh records here; the service read lives out of slice.
    INFO_SETTINGS_REFRESHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x36e04 — +[RobloxInfo searchUrl]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo searchUrl]")]
pub fn stub_0x36e04(tablet: bool, phone_url: &str, pad_url: &str) -> String {
    // IDA 0x36e04: `searchUrl` refreshes the settings service without a
    // forced web read (0x36e2a-0x36e58); the tablet flag picks the pad
    // URL over the phone URL (0x36e68-0x36e6a).
    stub_0x36de4();
    if tablet { pad_url.to_owned() } else { phone_url.to_owned() }
}

// 0x36e80 — __GLOBAL__I_a_9
#[doc(alias = "global constructor keyed to_a_9")]
#[doc(alias = "__GLOBAL__I_a_9")]
pub fn stub_0x36e80() {
    // IDA 0x36e80: `__GLOBAL__I_a_9` runs the `a_9` translation-unit static
    // initializers. Static-init glue; no explicit body.
}

// 0x37068 — __ZN10RobloxView37requestStopRenderingForBackgroundModeEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::requestStopRenderingForBackgroundMode(void)")]
#[doc(alias = "__ZN10RobloxView37requestStopRenderingForBackgroundModeEv")]
pub fn stub_0x37068(cleanup_in_background: bool) {
    // IDA 0x37068: `requestStopRenderingForBackgroundMode` signals the
    // render event and removes + resets the render job (0x370d8-0x37220),
    // then removes the view-update job (0x37164-0x37266); with
    // `RenderCleanupInBackground` both removals go through
    // `removeBlocking` + `ProcessMessages` (0x370f6-0x37204). Either way
    // both jobs end unscheduled, which records here.
    let _ = cleanup_in_background;
    ROBLOXVIEW_RENDERING.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x37378 — __ZN10RobloxView22requestResumeRenderingEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::requestResumeRendering(void)")]
#[doc(alias = "__ZN10RobloxView22requestResumeRenderingEv")]
pub fn stub_0x37378() {
    // IDA 0x37378: `requestResumeRendering` creates the view-update job
    // (0x373aa-0x373fc) and the render job (0x3741a-0x37468) and adds
    // both to the scheduler (0x37490-0x374f0). Both jobs end scheduled,
    // which records here.
    ROBLOXVIEW_RENDERING.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x375b4 — __Z13macBundlePathv
// type: _DWORD __fastcall()
#[doc(alias = "macBundlePath(void)")]
#[doc(alias = "__Z13macBundlePathv")]
pub fn stub_0x375b4(bundle_path: &str) -> String {
    // IDA 0x375b4: `macBundlePath` copies the main bundle's POSIX path
    // (`CFBundleGetMainBundle`/`CFBundleCopyBundleURL`/
    // `CFURLCopyFileSystemPath`, 0x375d4-0x375f4) into the out string
    // (0x3760a). The CoreFoundation traffic is drop glue.
    bundle_path.to_owned()
}

// 0x37628 — __ZN10RobloxViewC2EjjSsSsSs
#[doc(alias = "RobloxView::RobloxView(unsigned int,unsigned int,std::string,std::string,std::string)")]
#[doc(alias = "__ZN10RobloxViewC2EjjSsSsSs")]
pub fn stub_0x37628(width: u32, height: u32, first: &str, second: &str, third: &str) -> RobloxViewCreate {
    // IDA 0x37628: `RobloxView::RobloxView` copies the three strings
    // (0x3764e-0x37694), one-shots the log manager + plugin modules
    // (0x376b4-0x376d4), creates the Ogre view (0x3770c-0x37758) and
    // installs the view-update job (0x37800-0x37822). The args record
    // here; init + view creation are engine glue.
    RobloxViewCreate {
        width,
        height,
        first: first.to_owned(),
        second: second.to_owned(),
        third: third.to_owned(),
    }
}

// 0x37b3c — __ZN10RobloxView16completeViewPrepEN5boost10shared_ptrIN3RBX4GameEEE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, void *, char, int, int, int, int)
#[doc(alias = "RobloxView::completeViewPrep(rbx_core::SharedPtr<RBX::Game>)")]
#[doc(alias = "__ZN10RobloxView16completeViewPrepEN5boost10shared_ptrIN3RBX4GameEEE")]
pub fn stub_0x37b3c(game_present: bool) -> bool {
    // IDA 0x37b3c: `completeViewPrep` stores the game, connects
    // `onPlaceIDChanged` (0x37b60-0x37c00), binds the workspace
    // (0x37cc4), creates the render job + concurrency rules and adds
    // both jobs (0x37cf8-0x37dd6), then wires `restartDataModel` /
    // `newGameDidStart` when watched (0x37e04-0x37eaa). Completion
    // records here; signal/job traffic is scheduler glue.
    if game_present {
        ROBLOXVIEW_PREPS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    game_present
}

// 0x380a4 — __ZN10RobloxView13bindWorkspaceEN5boost10shared_ptrIN3RBX8ViewBaseEEENS1_INS2_9DataModelEEENS1_INS2_16OverlayDataModelEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int)
#[doc(alias = "RobloxView::bindWorkspace(rbx_core::SharedPtr<RBX::ViewBase>,rbx_core::SharedPtr<RBX::DataModel>,rbx_core::SharedPtr<RBX::OverlayDataModel>)")]
#[doc(alias = "__ZN10RobloxView13bindWorkspaceEN5boost10shared_ptrIN3RBX8ViewBaseEEENS1_INS2_9DataModelEEENS1_INS2_16OverlayDataModelEEE")]
pub fn stub_0x380a4(overlay_present: bool) -> bool {
    // IDA 0x380a4: `bindWorkspace` binds the overlay datamodel into the
    // view when present (LegacyLock + `operator=` under the lock,
    // 0x380d2-0x3815c), then binds the main datamodel the same way and
    // refreshes (0x38166-0x381da). Lock/`Arc` traffic is drop glue; the
    // bind records here.
    if overlay_present {
        ROBLOXVIEW_BINDS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    overlay_present
}

// 0x382b0 — __ZN10RobloxView22defineConcurrencyRulesEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::defineConcurrencyRules(void)")]
#[doc(alias = "__ZN10RobloxView22defineConcurrencyRulesEv")]
pub fn stub_0x382b0(render_present: bool, update_present: bool) {
    // IDA 0x382b0: `defineConcurrencyRules` release-asserts both jobs
    // (`RobloxView.cpp:555-556`, 0x382ea-0x3839a), adds an
    // `ExclusiveSequence` coordinator to each (0x383b0-0x3849a) and,
    // when render settings allow, a `Sequence` shared with the physics
    // job (0x384b4-0x385c4).
    assert!(render_present, "renderJob (IDA 0x382b0)");
    assert!(update_present, "viewUpdateJob (IDA 0x382b0)");
    ROBLOXVIEW_RULES.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x386d0 — __ZN10RobloxView16restartDataModelEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::restartDataModel(void)")]
#[doc(alias = "__ZN10RobloxView16restartDataModelEv")]
pub fn stub_0x386d0() {
    // IDA 0x386d0: `restartDataModel` captures `this` in a stack block
    // and `dispatch_async`s `doRestartDataModel` to the main queue
    // (0x38706-0x38718). The dispatch records here; the block runs at
    // 0x38770.
    ROBLOXVIEW_RESTARTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x38720 — __ZN10RobloxView15newGameDidStartEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::newGameDidStart(void)")]
#[doc(alias = "__ZN10RobloxView15newGameDidStartEv")]
pub fn stub_0x38720() {
    // IDA 0x38720: `newGameDidStart` captures `this` in a stack block
    // and `dispatch_async`s its block to the main queue
    // (0x38756-0x38768). The dispatch records here; the block runs at
    // 0x39018.
    ROBLOXVIEW_NEWGAMES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x38770 — ____ZN10RobloxView18doRestartDataModelEv_block_invoke
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "____ZN10RobloxView18doRestartDataModelEv_block_invoke")]
pub fn stub_0x38770() {
    // IDA 0x38770: the `doRestartDataModel` block runs the main-queue
    // restart (teardown + `setupNewDataModel`). It sequences the same
    // path as 0x38cd0 with no datamodel yet installed.
    stub_0x38cd0(false);
    ROBLOXVIEW_DID_RESTART.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x38cd0 — __ZN10RobloxView17setupNewDataModelEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::setupNewDataModel(void)")]
#[doc(alias = "__ZN10RobloxView17setupNewDataModelEv")]
pub fn stub_0x38cd0(datamodel_present: bool) -> bool {
    // IDA 0x38cd0: `setupNewDataModel` returns early when a datamodel
    // exists (0x38d42); else it creates one, sets it on the game,
    // attaches the view, connects `onPlaceIDChanged` and shuts the
    // overlay down (0x38d4a-0x38ee0). Creation records here.
    if datamodel_present {
        return false;
    }
    ROBLOXVIEW_DATAMODELS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    true
}

// 0x39018 — ____ZN10RobloxView15newGameDidStartEv_block_invoke
#[doc(alias = "____ZN10RobloxView15newGameDidStartEv_block_invoke")]
pub fn stub_0x39018() {
    // IDA 0x39018: the `newGameDidStart` block finishes the main-queue
    // start (same dispatch shape as 0x38770). Completion records here.
    ROBLOXVIEW_STARTED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x39020 — __ZN10RobloxViewD1Ev
// type: void __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::~RobloxView()")]
#[doc(alias = "__ZN10RobloxViewD1Ev")]
pub fn stub_0x39020() {
    // IDA 0x39020: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39024 — __ZN10RobloxViewD2Ev
// type: void __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::~RobloxView()")]
#[doc(alias = "__ZN10RobloxViewD2Ev")]
pub fn stub_0x39024() {
    // IDA 0x39024: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x39674 — __ZN10RobloxView11create_viewEN5boost10shared_ptrIN3RBX4GameEEEjjSsSsSs
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, std::string *, std::string *)
#[doc(alias = "RobloxView::create_view(rbx_core::SharedPtr<RBX::Game>,unsigned int,unsigned int,std::string,std::string,std::string)")]
#[doc(alias = "__ZN10RobloxView11create_viewEN5boost10shared_ptrIN3RBX4GameEEEjjSsSsSs")]
pub fn stub_0x39674(game_present: bool, width: u32, height: u32, first: &str, second: &str, third: &str) -> RobloxViewCreate {
    // IDA 0x39674: `create_view` news the view, runs the `RobloxView`
    // constructor on the copied strings (0x396a8-0x39702) and finishes
    // with `completeViewPrep` on the game (0x39736-0x3974c). Creation
    // records here; the prep counts at 0x37b3c.
    if game_present {
        ROBLOXVIEW_CREATES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    stub_0x37628(width, height, first, second, third)
}

// 0x39920 — __ZL14initLogManagerv
// type: _DWORD __fastcall()
#[doc(alias = "initLogManager(void)")]
#[doc(alias = "__ZL14initLogManagerv")]
pub fn stub_0x39920(bundle_path: &str) -> String {
    // IDA 0x39920: `initLogManager` one-shots the Ogre `LogManager`
    // (`__cxa_guard_acquire`, 0x3998c-0x399dc) and creates
    // `<bundle>/ogre.log` (0x399e8-0x39a26, `macBundlePath` at 0x3993e).
    // The manager is engine glue; the log path records here.
    LOG_MANAGER_INIT.store(true, std::sync::atomic::Ordering::SeqCst);
    format!("{bundle_path}/ogre.log")
}

// 0x39be0 — __ZNSt12domain_errorD0Ev
// type: void __cdecl(std::domain_error *__hidden this)
#[doc(alias = "std::domain_error::~domain_error()")]
#[doc(alias = "__ZNSt12domain_errorD0Ev")]
pub fn stub_0x39be0() {
    // IDA 0x39be0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39bf8 — __ZNSt12domain_errorD2Ev
// type: void __cdecl(std::domain_error *__hidden this)
#[doc(alias = "std::domain_error::~domain_error()")]
#[doc(alias = "__ZNSt12domain_errorD2Ev")]
pub fn stub_0x39bf8() {
    // IDA 0x39bf8: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x39c00 — __ZNSt16invalid_argumentD1Ev
// type: void __cdecl(std::invalid_argument *__hidden this)
#[doc(alias = "std::invalid_argument::~invalid_argument()")]
#[doc(alias = "__ZNSt16invalid_argumentD1Ev")]
pub fn stub_0x39c00() {
    // IDA 0x39c00: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39c08 — __ZNSt12length_errorD0Ev
// type: void __cdecl(std::length_error *__hidden this)
#[doc(alias = "std::length_error::~length_error()")]
#[doc(alias = "__ZNSt12length_errorD0Ev")]
pub fn stub_0x39c08() {
    // IDA 0x39c08: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39c20 — __ZNSt12out_of_rangeD1Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
#[doc(alias = "__ZNSt12out_of_rangeD1Ev")]
pub fn stub_0x39c20() {
    // IDA 0x39c20: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39c28 — __ZNSt11range_errorD0Ev
// type: void __cdecl(std::range_error *__hidden this)
#[doc(alias = "std::range_error::~range_error()")]
#[doc(alias = "__ZNSt11range_errorD0Ev")]
pub fn stub_0x39c28() {
    // IDA 0x39c28: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39c40 — __ZNSt11range_errorD2Ev
// type: void __cdecl(std::range_error *__hidden this)
#[doc(alias = "std::range_error::~range_error()")]
#[doc(alias = "__ZNSt11range_errorD2Ev")]
pub fn stub_0x39c40() {
    // IDA 0x39c40: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x39c48 — __ZNSt14overflow_errorD1Ev
// type: void __cdecl(std::overflow_error *__hidden this)
#[doc(alias = "std::overflow_error::~overflow_error()")]
#[doc(alias = "__ZNSt14overflow_errorD1Ev")]
pub fn stub_0x39c48() {
    // IDA 0x39c48: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39c50 — __ZNSt15underflow_errorD0Ev
// type: void __cdecl(std::underflow_error *__hidden this)
#[doc(alias = "std::underflow_error::~underflow_error()")]
#[doc(alias = "__ZNSt15underflow_errorD0Ev")]
pub fn stub_0x39c50() {
    // IDA 0x39c50: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39c68 — __ZNSt15underflow_errorD2Ev
// type: void __cdecl(std::underflow_error *__hidden this)
#[doc(alias = "std::underflow_error::~underflow_error()")]
#[doc(alias = "__ZNSt15underflow_errorD2Ev")]
pub fn stub_0x39c68() {
    // IDA 0x39c68: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x39c6c — __ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEENS1_8functionIFvvEEE
// type: void __fastcall(int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::removeBlocking(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,boost::function<void ()(void)>)")]
#[doc(alias = "__ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEENS1_8functionIFvvEEE")]
pub fn stub_0x39c6c() {
    // IDA 0x39c6c: `TaskScheduler::removeBlocking` removes the job and
    // runs the follow-up closure. Scheduler glue; no explicit body.
}

// 0x39d7c — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEE5resetEv
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::reset(void)")]
#[doc(alias = "__ZN5boost10shared_ptrIN10RobloxView9RenderJobEE5resetEv")]
pub fn stub_0x39d7c() {
    // IDA 0x39d7c: `shared_ptr<RenderJob>::reset` drops the job.
    // `Arc` drop glue covers it; no explicit body.
}

// 0x39e10 — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv
#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::reset(void)")]
#[doc(alias = "__ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv")]
pub fn stub_0x39e10() {
    // IDA 0x39e10: `shared_ptr<ViewUpdateJob>::reset` drops the job.
    // `Arc` drop glue covers it; no explicit body.
}

// 0x39ea8 — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_
#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::operator=(rbx_core::SharedPtr<RobloxView::ViewUpdateJob>&&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_")]
pub fn stub_0x39ea8() {
    // IDA 0x39ea8: `shared_ptr<ViewUpdateJob>::operator=(&&)` move-assigns
    // the job. `Arc` move glue covers it; no explicit body.
}

// 0x39f4c — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::shared_ptr<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_")]
pub fn stub_0x39f4c() {
    // IDA 0x39f4c: `shared_ptr<ViewUpdateJob>::shared_ptr<...>(ptr)`
    // wraps the raw job. `Arc` construction glue covers it; no explicit
    // body.
}

// 0x3a030 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEaSEOS3_
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::operator=(rbx_core::SharedPtr<RobloxView::RenderJob>&&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN10RobloxView9RenderJobEEaSEOS3_")]
pub fn stub_0x3a030() {
    // IDA 0x3a030: `shared_ptr<RenderJob>::operator=(&&)` move-assigns
    // the job. `Arc` move glue covers it; no explicit body.
}

// 0x3a0d4 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEC1IS2_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::shared_ptr<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN10RobloxView9RenderJobEEC1IS2_EEPT_")]
pub fn stub_0x3a0d4() {
    // IDA 0x3a0d4: `shared_ptr<RenderJob>::shared_ptr<...>(ptr)` wraps
    // the raw job. `Arc` construction glue covers it; no explicit body.
}

// 0x3a1b8 — __ZN17QuitEventListenerD1Ev
// type: void __fastcall(QuitEventListener *__hidden this)
#[doc(alias = "QuitEventListener::~QuitEventListener()")]
#[doc(alias = "__ZN17QuitEventListenerD1Ev")]
pub fn stub_0x3a1b8() {
    // IDA 0x3a1b8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a1bc — __ZN5boost10shared_ptrIN3RBX4GameEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::Game>::operator=(rbx_core::SharedPtr<RBX::Game> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4GameEEaSERKS3_")]
pub fn stub_0x3a1bc() {
    // IDA 0x3a1bc: `shared_ptr<Game>::operator=(const&)` copy-assigns
    // the game. `Arc` clone glue covers it; no explicit body.
}

// 0x3a2ec — __ZN5boost10shared_ptrIN3RBX9DataModelEEaSINS1_16OverlayDataModelEEERS3_ONS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>& rbx_core::SharedPtr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(rbx_core::SharedPtr<RBX::OverlayDataModel> &&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9DataModelEEaSINS1_16OverlayDataModelEEERS3_ONS0_IT_EE")]
pub fn stub_0x3a2ec() {
    // IDA 0x3a2ec: `shared_ptr<DataModel>::operator=<OverlayDataModel>`
    // cross-assigns the overlay. `Arc` conversion glue covers it; no
    // explicit body.
}

// 0x3a390 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x3a390() {
    // IDA 0x3a390: `signal<void()>::connect<bind_t<mf0<RobloxView>>>`
    // binds the view method and installs the connection. Closure + slot
    // glue; no explicit body.
}

// 0x3a408 — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEE9singletonEv")]
pub fn stub_0x3a408(present: bool) -> bool {
    // IDA 0x3a408: `GlobalAdvancedSettingsItem<CRenderSettingsItem,
    // sRenderSettings>::singleton` returns the settings singleton,
    // consumed at 0x382b0 via its +188 flag. Presence collapses to
    // `bool`.
    present
}

// 0x3a5bc — __ZN5boost10shared_ptrIN3RBX5Tasks8SequenceEE5resetIS3_EEvPT_
#[doc(alias = "void rbx_core::SharedPtr<RBX::Tasks::Sequence>::reset<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5Tasks8SequenceEE5resetIS3_EEvPT_")]
pub fn stub_0x3a5bc() {
    // IDA 0x3a5bc: `shared_ptr<Sequence>::reset<Sequence>(ptr)` wraps
    // the sequence. `Arc` construction glue covers it; no explicit body.
}

// 0x3a660 — __ZN5boost10shared_ptrIN3RBX8ViewBaseEE5resetEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ViewBase>::reset(void)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8ViewBaseEE5resetEv")]
pub fn stub_0x3a660() {
    // IDA 0x3a660: `shared_ptr<ViewBase>::reset` drops the view.
    // `Arc` drop glue covers it; no explicit body.
}

// 0x3a6f8 — __ZN5boost13exception_ptrD1Ev
// type: void __fastcall(boost::exception_ptr *__hidden this)
#[doc(alias = "boost::exception_ptr::~exception_ptr()")]
#[doc(alias = "__ZN5boost13exception_ptrD1Ev")]
pub fn stub_0x3a6f8() {
    // IDA 0x3a6f8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a790 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev")]
pub fn stub_0x3a790() {
    // IDA 0x3a790: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a798 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6CameraEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6CameraEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x3a798(create_ok: bool) -> bool {
    // IDA 0x3a798: `Creatable<Instance>::create<Camera>` runs the
    // factory and returns the new instance (same shape as 0x3247c).
    // Factory glue; presence collapses to `bool`.
    create_ok
}

// 0x3a850 — __ZN5boost6detail15sp_counted_base12weak_releaseEv
// type: _DWORD __fastcall(boost::detail::sp_counted_base *__hidden this)
#[doc(alias = "boost::detail::sp_counted_base::weak_release(void)")]
#[doc(alias = "__ZN5boost6detail15sp_counted_base12weak_releaseEv")]
pub fn stub_0x3a850() {
    // IDA 0x3a850: `sp_counted_base::weak_release` drops a weak ref.
    // `Arc` downgrade glue covers it; no explicit body.
}

// 0x3aa10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x3aa10() {
    // IDA 0x3aa10: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3aa18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x3aa18() {
    // IDA 0x3aa18: `sp_counted_impl_pd<Camera*,Creatable::Deleter>::
    // get_deleter` answers the deleter query by `type_info` (same shape
    // as 0x33454). `Arc` drop glue covers it; no explicit body.
}

// 0x3aa30 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_ESH_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_ESH_")]
pub fn stub_0x3aa30(name: &str) {
    // IDA 0x3aa30: `_Rb_tree<Name const*,...>::erase(first, last)`
    // unregisters the creator range. The unregister records here.
    CREATOR_NAMES.lock().retain(|n| n != name);
}

// 0x3aa90 — __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEED1Ev
#[doc(alias = "std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::~map()")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEED1Ev")]
pub fn stub_0x3aa90() {
    // IDA 0x3aa90: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3aaa0 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev")]
pub fn stub_0x3aaa0() {
    // IDA 0x3aaa0: `FactoryProduct<Camera,...>::Creator::Creator`
    // constructs the creator (registers the factory product).
    // Construction glue; no explicit body.
}

// 0x3acc8 — __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_")]
pub fn stub_0x3acc8(name: &str) -> bool {
    // IDA 0x3acc8: `map<Name const*,ICreator const*>::operator[]`
    // fetches-or-creates the creator slot. Presence reports here.
    let mut names = CREATOR_NAMES.lock();
    if !names.iter().any(|n| n == name) {
        names.push(name.to_owned());
        return false;
    }
    true
}

// 0x3ad20 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::pair<RBX::Name const* const,RBX::ICreator const*> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")]
pub fn stub_0x3ad20(name: &str) -> bool {
    // IDA 0x3ad20: `_Rb_tree<Name const*,...>::_M_insert_unique`
    // registers the creator unless present. The register records here.
    let mut names = CREATOR_NAMES.lock();
    if names.iter().any(|n| n == name) {
        return false;
    }
    names.push(name.to_owned());
    true
}

// 0x3add8 — __ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v")]
pub fn stub_0x3add8() {
    // IDA 0x3add8: `Name::declare<sRunService>` one-shots the class-name
    // declaration (same shape as 0x32720). Idempotent declare glue; no
    // explicit body.
}

// 0x3ae20 — __ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v")]
pub fn stub_0x3ae20() {
    // IDA 0x3ae20: `Name::doDeclare<sRunService>` performs the
    // declaration. It records here.
    RUNSERVICE_DECLARED.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x3af08 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10RunServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RunService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_10RunServiceEEEmv")]
pub fn stub_0x3af08() -> usize {
    // IDA 0x3af08: `ServiceProvider::doGetClassIndex<RunService>`
    // returns the service class index.
    *RUNSERVICE_CLASS_INDEX
}

// 0x3afe0 — __ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x3afe0() {
    // IDA 0x3afe0: `shared_ptr<RunService>::shared_ptr<...,
    // Creatable::Deleter>` stores the pointer + deleter (same shape as
    // 0x324fc). `Arc` construction glue covers it; no explicit body.
}

// 0x3b008 — __ZN5boost6detail12shared_countC2IPN3RBX10RunServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10RunServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x3b008() {
    // IDA 0x3b008: `shared_count::shared_count<RunService*,...>`
    // allocates the control block (same shape as 0x325fc). `Arc`
    // construction glue covers it; no explicit body.
}
