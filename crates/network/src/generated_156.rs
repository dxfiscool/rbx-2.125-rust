//! Auto-generated skeletons for rbx-network — global EA-sorted filler (RakNet|Network|Replicat|Socket filtered exhausted)
//! Filter: RakNet|Network|Replicat|Socket -> 5198 funcs (cs), 5282 (ci), 3 remaining before batch (next 0xecd6e8 _TFCreateCrashSocket); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x2ba40..0x312cc | existing 17409 -> 17509 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

/// ObjC block captured-object slot index for the +20 byte field (word 5).
pub const BLOCK_CAPTURE_WORD: usize = 5;
/// `_Block_object_assign` / `_Block_object_dispose` kind for retained objects (BLOCK_FIELD_IS_OBJECT).
pub const BLOCK_FIELD_IS_OBJECT: usize = 3;

/// `boost::shared_ptr` value slot: raw pointer plus its counted control word.
#[derive(Clone, Copy, Debug, Default)]
pub struct SharedSlot {
    pub ptr: usize,
    pub counted: usize,
}

/// `boost::_bi::bind_t` capture: target fn plus bound `(cstr, game)` pair (IDA 0x2ca7c).
#[derive(Clone, Debug, Default)]
pub struct BindCstrGame {
    pub target: usize,
    pub cstr: usize,
    pub game: SharedSlot,
}

/// `boost::_bi::bind_t` capture: target fn plus `(int, cstr, game)` triple (IDA 0x2cb64).
#[derive(Clone, Debug, Default)]
pub struct BindIntCstrGame {
    pub target: usize,
    pub arg0: i32,
    pub cstr: usize,
    pub game: SharedSlot,
}

/// `boost::_bi::bind_t` capture: target fn plus `(int, game, JoinGameRequest)` triple (IDA 0x2cc54).
#[derive(Clone, Debug, Default)]
pub struct BindIntGameJoin {
    pub target: usize,
    pub arg0: i32,
    pub game: SharedSlot,
    pub join_request: usize,
}

/// `boost::_bi::bind_t` capture: target fn plus `(int, game)` pair (IDA 0x2cd44).
#[derive(Clone, Debug, Default)]
pub struct BindIntGame {
    pub target: usize,
    pub arg0: i32,
    pub game: SharedSlot,
}

/// `boost::_bi::bind_t` capture: target fn plus `(string x3, page, game)` list5 (IDA 0x2ce2c).
#[derive(Clone, Debug, Default)]
pub struct BindStringsPageGame {
    pub target: usize,
    pub s0: String,
    pub s1: String,
    pub s2: String,
    pub page: usize,
    pub game: SharedSlot,
}

/// `boost::_bi::bind_t` capture: target fn plus `(RobloxView, game, FunctionMarshaller)` (IDA 0x2d280).
#[derive(Clone, Debug, Default)]
pub struct BindViewGameMarshaller {
    pub target: usize,
    pub view: usize,
    pub game: SharedSlot,
    pub marshaller: usize,
}

/// `boost::function<void(RBX::DataModel *)>` holding one bound functor (IDA 0x2d370).
#[derive(Clone, Debug, Default)]
pub struct DataModelCallback {
    pub bound: Option<BindViewGameMarshaller>,
}

/// `rbx::signals::connection` slot for `signal<void(std::string)>` (IDA 0x2c8c0).
#[derive(Clone, Debug, Default)]
pub struct StringSignalConn {
    pub id: u64,
    pub live: bool,
}

/// `ControlView` construction result of `initControlViewHelper` (IDA 0x2c224).
#[derive(Clone, Copy, Debug, Default)]
pub struct ControlViewInit {
    pub ogre_view: usize,
    pub ogre_window: usize,
    pub control_view: usize,
}
// 0x2ba40 — ___copy_helper_block_429
// demangled: ___copy_helper_block_429
// type: 
#[doc(alias = "___copy_helper_block_429")]
pub fn stub_2ba40(dst: &mut [usize], src: &[usize]) {
    // IDA 0x2ba40: _Block_object_assign(dst + 20, src[20], BLOCK_FIELD_IS_OBJECT).
    let retained = src.get(BLOCK_CAPTURE_WORD).copied().unwrap_or(0);
    if let Some(slot) = dst.get_mut(BLOCK_CAPTURE_WORD) {
        *slot = retained;
    }
}

// 0x2ba4c — ___destroy_helper_block_430
// demangled: ___destroy_helper_block_430
// type: 
#[doc(alias = "___destroy_helper_block_430")]
pub fn stub_2ba4c(block: &[usize], release: &mut dyn FnMut(usize)) {
    // IDA 0x2ba4c: _Block_object_dispose(block[20], BLOCK_FIELD_IS_OBJECT).
    release(block.get(BLOCK_CAPTURE_WORD).copied().unwrap_or(0));
}

// 0x2ba54 — __ZL16executeUrlScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
// demangled: executeUrlScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "executeUrlScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
pub fn stub_2ba54(
    game: usize,
    url: &str,
    enter_play_identity: &mut dyn FnMut(),
    is_url: &mut dyn FnMut(usize, &str) -> bool,
    fetch_body: &mut dyn FnMut(usize, &str) -> Option<String>,
    run_signed: &mut dyn FnMut(usize, &str),
) {
    // IDA 0x2ba54: Impersonator(Identities 7); if ContentProvider::isUrl: LegacyLock, ContentId +
    // getContent streamed into a string, then executeSignedScript; else just teardown.
    enter_play_identity();
    if !is_url(game, url) {
        return;
    }
    if let Some(body) = fetch_body(game, url) {
        run_signed(game, &body);
    }
}

// 0x2bdb0 — __ZL19executeSignedScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
// demangled: executeSignedScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "executeSignedScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
pub fn stub_2bdb0(
    game: usize,
    script: &str,
    verify_signature: &mut dyn FnMut(usize, &str) -> String,
    run: &mut dyn FnMut(usize, &str),
) {
    // IDA 0x2bdb0: verified = ContentProvider::verifyScriptSignature; assign to string; executeScript.
    let verified = verify_signature(game, script);
    run(game, &verified);
}

// 0x2bf74 — __ZL13executeScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
// demangled: executeScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "executeScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
pub fn stub_2bf74(
    game: usize,
    script: &str,
    script_execution_enabled: bool,
    execute_in_new_thread: &mut dyn FnMut(usize, &str),
) {
    // IDA 0x2bf74: LegacyLock; if byte at DataModel + 3005: ProtectedString::fromTrustedSource then
    // ScriptContext::executeInNewThread(7, ...).
    if script_execution_enabled {
        execute_in_new_thread(game, script);
    }
}

// 0x2c138 — ____ZL15presentGameViewv_block_invoke
// demangled: ____ZL15presentGameViewv_block_invoke
// type: void __cdecl(id)
#[doc(alias = "____ZL15presentGameViewv_block_invoke")]
pub fn stub_2c138(
    shared_instance: &mut dyn FnMut() -> usize,
    ogre_view_controller: &mut dyn FnMut(usize) -> usize,
    last_non_game_controller: &mut dyn FnMut(usize) -> usize,
    presented_view_controller: &mut dyn FnMut(usize) -> usize,
    present_view_controller: &mut dyn FnMut(usize, usize),
) {
    // IDA 0x2c138: shared MainViewController; if its presented controller is not the Ogre
    // controller, present it (animated 0) with the handleStartGameSuccess completion block.
    let mvc = shared_instance();
    if mvc == 0 {
        return;
    }
    let ogre = ogre_view_controller(mvc);
    if ogre == 0 {
        return;
    }
    let host = last_non_game_controller(mvc);
    if host == 0 {
        return;
    }
    if presented_view_controller(host) != ogre {
        present_view_controller(host, ogre);
    }
}

// 0x2c1f8 — ____ZL15presentGameViewv_block_invoke_2
// demangled: ____ZL15presentGameViewv_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "____ZL15presentGameViewv_block_invoke_2")]
pub fn stub_2c1f8(block: &[usize], handle_start_game_success: &mut dyn FnMut(usize) -> usize) -> usize {
    // IDA 0x2c1f8: target = block[20]; return target ? [target handleStartGameSuccess] : target.
    let target = block.get(BLOCK_CAPTURE_WORD).copied().unwrap_or(0);
    if target != 0 {
        handle_start_game_success(target)
    } else {
        target
    }
}

// 0x2c210 — ___copy_helper_block_499
// demangled: ___copy_helper_block_499
// type: 
#[doc(alias = "___copy_helper_block_499")]
pub fn stub_2c210(dst: &mut [usize], src: &[usize]) {
    // IDA 0x2c210: _Block_object_assign(dst + 20, src[20], BLOCK_FIELD_IS_OBJECT).
    let retained = src.get(BLOCK_CAPTURE_WORD).copied().unwrap_or(0);
    if let Some(slot) = dst.get_mut(BLOCK_CAPTURE_WORD) {
        *slot = retained;
    }
}

// 0x2c21c — ___destroy_helper_block_500
// demangled: ___destroy_helper_block_500
// type: 
#[doc(alias = "___destroy_helper_block_500")]
pub fn stub_2c21c(block: &[usize], release: &mut dyn FnMut(usize)) {
    // IDA 0x2c21c: _Block_object_dispose(block[20], BLOCK_FIELD_IS_OBJECT).
    release(block.get(BLOCK_CAPTURE_WORD).copied().unwrap_or(0));
}

// 0x2c224 — __ZL21initControlViewHelperP10RobloxViewa
// demangled: initControlViewHelper(RobloxView *,signed char)
// type: _DWORD __fastcall(RobloxView *, signed __int8)
#[doc(alias = "initControlViewHelper(RobloxView *,signed char)")]
pub fn stub_2c224(
    has_controller: bool,
    has_render_window: bool,
    present_on_main: bool,
    build_views: &mut dyn FnMut() -> ControlViewInit,
    dispatch_main: &mut dyn FnMut(),
) -> ControlViewInit {
    // IDA 0x2c224: if MainViewController.sharedInstance && RobloxView.var7: resolve the VIEW/WINDOW
    // render targets, setOgreView/setOgreWindow, alloc ControlView initWithGame(mainScreen.bounds)
    // autorelease + addSubview twice. If a2: dispatch_async(main, block_global505).
    let mut out = ControlViewInit::default();
    if has_controller && has_render_window {
        out = build_views();
    }
    if present_on_main {
        dispatch_main();
    }
    out
}

// 0x2c5b0 — __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv
// demangled: __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv")]
pub fn stub_2c5b0(cell: &mut Option<usize>, create_settings: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x2c5b0: fast path returns sing; else GlobalAdvancedSettings::singleton, mutex lock,
    // create TaskSchedulerSettings + setParentInternal, ReleaseAssert(s.get() == sing), unlock.
    if let Some(sing) = *cell {
        return sing;
    }
    let created = create_settings();
    if cell.is_none() {
        *cell = Some(created);
    }
    debug_assert_eq!(*cell, Some(created));
    created
}

// 0x2c764 — __ZNK3RBX15ServiceProvider4findINS_10GuiServiceEEEPT_v
// demangled: RBX::GuiService * RBX::ServiceProvider::find<RBX::GuiService>(void)const
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "RBX::GuiService * RBX::ServiceProvider::find<RBX::GuiService>(void)const")]
pub fn stub_2c764(
    cache: &mut Vec<usize>,
    class_index: &mut dyn FnMut() -> usize,
    find_service_by_class_name: &mut dyn FnMut() -> usize,
) -> usize {
    // IDA 0x2c764: call_once(doGetClassIndex<GuiService>); cached slot hit returns early (vector
    // resized when the index is out of range); else findServiceByClassName, store, return.
    let idx = class_index();
    if idx + 1 <= cache.len() {
        let hit = cache[idx];
        if hit != 0 {
            return hit;
        }
    } else {
        cache.resize(idx + 1, 0);
    }
    let found = find_service_by_class_name();
    if found != 0 {
        cache[idx] = found;
    }
    found
}

// 0x2c8c0 — __ZN3rbx7signals6signalIFvSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)
// type: int __fastcall(char, boost::mutex *, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")]
pub fn stub_2c8c0(slots: &mut Vec<StringSignalConn>) -> u64 {
    // IDA 0x2c8c0: operator new islot(32); callable<slot, boost::function> ctor; signal::insert;
    // the returned connection holds a weak ref to the slot.
    let id = slots.len() as u64;
    slots.push(StringSignalConn { id, live: true });
    id
}

// 0x2c9a8 — __ZN5boost10shared_ptrIN3RBX4GameEEC1INS1_16SecurePlayerGameEEEPT_
// demangled: boost::shared_ptr<RBX::Game>::shared_ptr<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::Game>::shared_ptr<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")]
pub fn stub_2c9a8(slot: &mut SharedSlot, raw: usize, counted: usize) {
    // IDA 0x2c9a8: shared_ptr<Game> from SecurePlayerGame*: px = raw, shared_count ctor, swap in.
    slot.ptr = raw;
    slot.counted = counted;
}

// 0x2ca7c — __ZN5boost4bindIvRKSsNS_10shared_ptrIN3RBX4GameEEEPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// demangled: boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<char const*,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,std::string const&,boost::shared_ptr<RBX::Game>,char const*,boost::shared_ptr<RBX::Game>>(void (*)(std::string const&,boost::shared_ptr<RBX::Game>),char const*,boost::shared_ptr<RBX::Game>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<char const*,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,std::string const&,rbx_core::SharedPtr<RBX::Game>,char const*,rbx_core::SharedPtr<RBX::Game>>(void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),char const*,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_2ca7c(target: usize, cstr: usize, game: SharedSlot) -> BindCstrGame {
    // IDA 0x2ca7c: list2<value<char const*>, value<shared_ptr<Game>>> ctor; bind_t{target, args}.
    BindCstrGame { target, cstr, game }
}

// 0x2cb64 — __ZN5boost4bindIviRKSsNS_10shared_ptrIN3RBX4GameEEEiPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
// demangled: boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_3<int,char const*,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,int,std::string const&,boost::shared_ptr<RBX::Game>,int,char const*,boost::shared_ptr<RBX::Game>>(void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),int,char const*,boost::shared_ptr<RBX::Game>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_3<int,char const*,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,int,std::string const&,rbx_core::SharedPtr<RBX::Game>,int,char const*,rbx_core::SharedPtr<RBX::Game>>(void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),int,char const*,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_2cb64(target: usize, arg0: i32, cstr: usize, game: SharedSlot) -> BindIntCstrGame {
    // IDA 0x2cb64: list3<value<int>, value<char const*>, value<shared_ptr<Game>>> ctor; bind_t pack.
    BindIntCstrGame { target, arg0, cstr, game }
}

// 0x2cc54 — __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestiS4_S5_EENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_
// demangled: boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list_av_3<int,boost::shared_ptr<RBX::Game>,JoinGameRequest>::type> boost::bind<void,int,boost::shared_ptr<RBX::Game>,JoinGameRequest,int,boost::shared_ptr<RBX::Game>,JoinGameRequest>(void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),int,boost::shared_ptr<RBX::Game>,JoinGameRequest)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list_av_3<int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest>::type> boost::bind<void,int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest,int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest>(void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest)")]
pub fn stub_2cc54(target: usize, arg0: i32, game: SharedSlot, join_request: usize) -> BindIntGameJoin {
    // IDA 0x2cc54: list3<value<int>, value<shared_ptr<Game>>, value<JoinGameRequest>> ctor; pack.
    BindIntGameJoin { target, arg0, game, join_request }
}

// 0x2cd44 — __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEEiS4_EENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
// demangled: boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<int,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,int,boost::shared_ptr<RBX::Game>,int,boost::shared_ptr<RBX::Game>>(void (*)(int,boost::shared_ptr<RBX::Game>),int,boost::shared_ptr<RBX::Game>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<int,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,int,rbx_core::SharedPtr<RBX::Game>,int,rbx_core::SharedPtr<RBX::Game>>(void (*)(int,rbx_core::SharedPtr<RBX::Game>),int,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_2cd44(target: usize, arg0: i32, game: SharedSlot) -> BindIntGame {
    // IDA 0x2cd44: list2<value<int>, value<shared_ptr<Game>>> ctor; bind_t pack.
    BindIntGame { target, arg0, game }
}

// 0x2ce2c — __ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
// demangled: boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>,std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>(void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Gam
// type: int __fastcall(int, int, std::string *, int, std::string *, int, int)
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>,std::string,std::string,std::string,RobloxPageViewController *,boost::share")]
pub fn stub_2ce2c(
    target: usize,
    s0: &str,
    s1: &str,
    s2: &str,
    page: usize,
    game: SharedSlot,
) -> BindStringsPageGame {
    // IDA 0x2ce2c: three std::string copies + shared_count copy into list5; bind_t pack; temps released.
    BindStringsPageGame {
        target,
        s0: s0.to_owned(),
        s1: s1.to_owned(),
        s2: s2.to_owned(),
        page,
        game,
    }
}

// 0x2d280 — __ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
// demangled: boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list_av_3<RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *>::type> boost::bind<void,RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *,RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *>(void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list_av_3<RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *>::type> boost::bind<void,RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *,RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *>(void (*)(RobloxView *,rbx_core::SharedPtr<RBX::G")]
pub fn stub_2d280(target: usize, view: usize, game: SharedSlot, marshaller: usize) -> BindViewGameMarshaller {
    // IDA 0x2d280: list3<value<RobloxView*>, value<shared_ptr<Game>>, value<FunctionMarshaller*>> ctor; pack.
    BindViewGameMarshaller { target, view, game, marshaller }
}

// 0x2d370 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// demangled: __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2d370(bound: BindViewGameMarshaller) -> DataModelCallback {
    // IDA 0x2d370: function<void(DataModel*)> ctor: bind_t copied to a temp, forwarded to function1 ctor.
    DataModelCallback { bound: Some(bound) }
}

// 0x2d458 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// demangled: __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2d458(bound: BindViewGameMarshaller) -> DataModelCallback {
    // IDA 0x2d458: function1 ctor: *a1 = 0, then assign_to with the bind_t copy; temp released.
    let mut cb = DataModelCallback::default();
    stub_2d544(&mut cb, bound);
    cb
}

// 0x2d544 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_
// demangled: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Gam
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshall")]
pub fn stub_2d544(cb: &mut DataModelCallback, bound: BindViewGameMarshaller) {
    // IDA 0x2d544: basic_vtable1::assign_to(stored_vtable, buffer a1 + 4, functor copy).
    cb.bound = Some(bound);
}

// 0x2d644 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail")]
pub fn stub_2d644(op: i32, out_type: &mut usize, out_flags: &mut u16) -> usize {
    // IDA 0x2d644: op != 4 (get_type): tail-call functor_manager::manager table; else store the
    // bind_t typeinfo, clear the small-object flags word, return the typeinfo.
    const MANAGER_TABLE: usize = 0x2d648;
    const BIND_T_TYPEINFO: usize = 0x2d65a;
    if op != 4 {
        return MANAGER_TABLE;
    }
    *out_type = BIND_T_TYPEINFO;
    *out_flags = 0;
    BIND_T_TYPEINFO
}

// 0x2d660 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEvPNS8_9DataModelEE6invokeERNS1_15function_bufferESN_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
// type: int __fastcall(int, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn stub_2d660(bound: &BindViewGameMarshaller, invoke: &mut dyn FnMut(usize, usize, usize)) {
    // IDA 0x2d660: void_function_obj_invoker1::invoke: functor f from the buffer, DataModel* arg in
    // list1; list3::operator() calls f(view, game, marshaller).
    invoke(bound.view, bound.game.ptr, bound.marshaller);
}

// 0x2d67c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boos
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>")]
pub fn stub_2d67c() -> ! {
    todo!("0x2d67c bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),")
}

// 0x2d768 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boos
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>")]
pub fn stub_2d768() -> ! {
    todo!("0x2d768 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),")
}

// 0x2d884 — __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RB")]
pub fn stub_2d884() -> ! {
    todo!("0x2d884 void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView")
}

// 0x2d964 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detai")]
pub fn stub_2d964() -> ! {
    todo!("0x2d964 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<R")
}

// 0x2da9c — __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
// demangled: boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::list3(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::list3(boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")]
pub fn stub_2da9c() -> ! {
    todo!("0x2da9c boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::list3(boost::_bi::value<RobloxView ")
}

// 0x2db54 — __ZN5boost3_bi8storage3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
// demangled: boost::_bi::storage3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::storage3(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::storage3(boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")]
pub fn stub_2db54() -> ! {
    todo!("0x2db54 boost::_bi::storage3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::storage3(boost::_bi::value<Roblo")
}

// 0x2dc24 — __ZN5boost6threadC2INS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEEOT_
// demangled: boost::thread::thread<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,b
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::thread::thread<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,std")]
pub fn stub_2dc24() -> ! {
    todo!("0x2dc24 boost::thread::thread<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>")
}

// 0x2dfac — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEC2EOSK_
// demangled: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::va
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(std::stri")]
pub fn stub_2dfac() -> ! {
    todo!("0x2dfac boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::st")
}

// 0x2e0f4 — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEED1Ev
// demangled: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::~thread_data()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::~thread_data()")]
pub fn stub_2e0f4() -> ! {
    todo!("0x2e0f4 boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::st")
}

// 0x2e1bc — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEED0Ev
// demangled: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::~thread_data()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::~thread_data()")]
pub fn stub_2e1bc() -> ! {
    todo!("0x2e1bc boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::st")
}

// 0x2e284 — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEE3runEv
// demangled: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::run(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::run(void)")]
pub fn stub_2e284() -> ! {
    todo!("0x2e284 boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::st")
}

// 0x2e2a0 — __ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvSsSsSsP8NSObjectSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
// type: int __fastcall(std::string *)
// was: boost::shared_ptr
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string,std::string,std::string,NSOb")]
pub fn stub_2e2a0() -> ! {
    todo!("0x2e2a0 void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<")
}

// 0x2e518 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS6_5list5INS6_5valueISsEESI_SI_NSH_IP24RobloxPageViewControllerEENSH_ISD_EEEEEEEEEEvPKNSA_IT_EEPT0_
// demangled: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::shared_ptr<boost::deta
// type: 
// was: boost::shared_ptr
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPag")]
pub fn stub_2e518() -> ! {
    todo!("0x2e518 void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void")
}

// 0x2e5ec — __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS4_5list5INS4_5valueISsEESG_SG_NSF_IP24RobloxPageViewControllerEENSF_ISB_EEEEEEEEEEPT_
// demangled: boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>(boost::detai")]
pub fn stub_2e5ec() -> ! {
    todo!("0x2e5ec boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),b")
}

// 0x2e6e0 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEED1Ev
// demangled: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::~sp_counted_impl_p()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::~sp_counted_impl_p()")]
pub fn stub_2e6e0() -> ! {
    todo!("0x2e6e0 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi")
}

// 0x2e6e4 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEED0Ev
// demangled: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::~sp_counted_impl_p()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::~sp_counted_impl_p()")]
pub fn stub_2e6e4() -> ! {
    todo!("0x2e6e4 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi")
}

// 0x2e6e8 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE7disposeEv
// demangled: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::dispose(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::dispose(void)")]
pub fn stub_2e6e8() -> ! {
    todo!("0x2e6e8 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi")
}

// 0x2e6f8 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::get_deleter(std::type_info const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::get_deleter(std::typ")]
pub fn stub_2e6f8() -> ! {
    todo!("0x2e6f8 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi")
}

// 0x2e6fc — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::get_untyped_deleter(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::get_untyped_deleter(")]
pub fn stub_2e6fc() -> ! {
    todo!("0x2e6fc boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi")
}

// 0x2e700 — __ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
// demangled: boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_2e700() -> ! {
    todo!("0x2e700 boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost")
}

// 0x2e970 — __ZN5boost3_bi8storage5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
// demangled: boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::G")]
pub fn stub_2e970() -> ! {
    todo!("0x2e970 boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<bo")
}

// 0x2ebbc — __ZN5boost3_bi8storage4INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEEEC2ES3_S3_S3_S6_
// demangled: boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>)")]
pub fn stub_2ebbc() -> ! {
    todo!("0x2ebbc boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>>::storage4(boost::_b")
}

// 0x2edec — __ZN5boost3_bi8storage3INS0_5valueISsEES3_S3_EC2ES3_S3_S3_
// demangled: boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)
// type: int(void)
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_2edec() -> ! {
    todo!("0x2edec boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::s")
}

// 0x2efb4 — __ZN5boost3_bi8storage2INS0_5valueISsEES3_EC2ES3_S3_
// demangled: boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_2efb4() -> ! {
    todo!("0x2efb4 boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)")
}

// 0x2f0f0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
// demangled: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2f0f0() -> ! {
    todo!("0x2f0f0 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integ")
}

// 0x2f1d8 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEEvT_
// demangled: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
pub fn stub_2f1d8() -> ! {
    todo!("0x2f1d8 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_p")
}

// 0x2f2d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_2f2d0() -> ! {
    todo!("0x2f2d0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared")
}

// 0x2f2ec — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEvE6invokeERNS1_15function_bufferE
// demangled: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_2f2ec() -> ! {
    todo!("0x2f2ec boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<bo")
}

// 0x2f300 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIiEENSE_ISA_EEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::functio")]
pub fn stub_2f300() -> ! {
    todo!("0x2f300 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi:")
}

// 0x2f3e8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIiEENSE_ISA_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::functio")]
pub fn stub_2f3e8() -> ! {
    todo!("0x2f3e8 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi:")
}

// 0x2f4fc — __ZN5boost3_bi5list2INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFviS7_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn stub_2f4fc() -> ! {
    todo!("0x2f4fc void boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi:")
}

// 0x2f5d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_2f5d4() -> ! {
    todo!("0x2f5d4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared")
}

// 0x2f708 — __ZN5boost3_bi5list2INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S8_
// demangled: boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_2f708() -> ! {
    todo!("0x2f708 boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")
}

// 0x2f7d0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// demangled: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2f7d0() -> ! {
    todo!("0x2f7d0 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_t")
}

// 0x2f8bc — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEEvT_
// demangled: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>)
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::sh")]
pub fn stub_2f8bc() -> ! {
    todo!("0x2f8bc void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value")
}

// 0x2f9bc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_")]
pub fn stub_2f9bc() -> ! {
    todo!("0x2f9bc boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::val")
}

// 0x2f9d8 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEvE6invokeERNS1_15function_bufferE
// demangled: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,void>::invoke(boost::detail::function::function_buffer &)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_2f9d8() -> ! {
    todo!("0x2f9d8 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boos")
}

// 0x2f9ec — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS5_5list3INS5_5valueIiEENSF_ISA_EENSF_ISB_EEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::funct
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost")]
pub fn stub_2f9ec() -> ! {
    todo!("0x2f9ec bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<")
}

// 0x2fad8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS5_5list3INS5_5valueIiEENSF_ISA_EENSF_ISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::funct
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost")]
pub fn stub_2fad8() -> ! {
    todo!("0x2fad8 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<")
}

// 0x2fbf4 — __ZN5boost3_bi5list3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEclIPFviS7_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::operator()<void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest) &,boost::_bi::list0 &,int)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::operator()<void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest) &,boost::_bi::list0 &,int)")]
pub fn stub_2fbf4() -> ! {
    todo!("0x2fbf4 void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::operator()<void (*)(int,boost::shared_ptr<RBX::Ga")
}

// 0x2fcd4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation")]
pub fn stub_2fcd4() -> ! {
    todo!("0x2fcd4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::val")
}

// 0x2fe0c — __ZN5boost3_bi5list3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEC2ES3_S8_SA_
// demangled: boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::list3(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::list3(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)")]
pub fn stub_2fe0c() -> ! {
    todo!("0x2fe0c boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::list3(boost::_bi::value<int>,boost::_bi::value<boost::")
}

// 0x2fec4 — __ZN5boost3_bi8storage3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEC2ES3_S8_SA_
// demangled: boost::_bi::storage3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::storage3(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::storage3(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)")]
pub fn stub_2fec4() -> ! {
    todo!("0x2fec4 boost::_bi::storage3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::storage3(boost::_bi::value<int>,boost::_bi::value<b")
}

// 0x2ff94 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// demangled: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: boost::detail::sp_counted_base *__fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2ff94() -> ! {
    todo!("0x2ff94 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_not")
}

// 0x30080 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEEvT_
// demangled: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char co")]
pub fn stub_30080() -> ! {
    todo!("0x30080 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::va")
}

// 0x3017c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_t")]
pub fn stub_3017c() -> ! {
    todo!("0x3017c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::")
}

// 0x30198 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
// demangled: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_30198() -> ! {
    todo!("0x30198 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,b")
}

// 0x301ac — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list3INS5_5valueIiEENSG_IPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::functio
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boo")]
pub fn stub_301ac() -> ! {
    todo!("0x301ac bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::val")
}

// 0x30298 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list3INS5_5valueIiEENSG_IPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::functio
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boo")]
pub fn stub_30298() -> ! {
    todo!("0x30298 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::val")
}

// 0x303b8 — __ZN5boost3_bi5list3INS0_5valueIiEENS2_IPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFviRKSsSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
// type: int(void)
// was: boost::shared_ptr
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn stub_303b8() -> ! {
    todo!("0x303b8 void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(int,std::string const&,boost::sha")
}

// 0x30534 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_")]
pub fn stub_30534() -> ! {
    todo!("0x30534 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::")
}

// 0x3066c — __ZN5boost3_bi5list3INS0_5valueIiEENS2_IPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S6_SB_
// demangled: boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list3(boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list3(boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_3066c() -> ! {
    todo!("0x3066c boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list3(boost::_bi::value<int>,boost::_bi::value<char const*")
}

// 0x3073c — __ZN5boost6threadC2INS_9function0IvEEEEOT_
// demangled: boost::thread::thread<boost::function0<void>>(boost::function0<void> &&)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, void *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "boost::thread::thread<boost::function0<void>>(boost::function0<void> &&)")]
pub fn stub_3073c() -> ! {
    todo!("0x3073c boost::thread::thread<boost::function0<void>>(boost::function0<void> &&)")
}

// 0x30878 — __ZN5boost6detail11thread_dataINS_9function0IvEEEC2EOS3_
// demangled: boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>&&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>&&)")]
pub fn stub_30878() -> ! {
    todo!("0x30878 boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>&&)")
}

// 0x3093c — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// demangled: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_3093c() -> ! {
    todo!("0x3093c __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_")
}

// 0x30a24 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEEvT_
// demangled: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>")]
pub fn stub_30a24() -> ! {
    todo!("0x30a24 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi")
}

// 0x30b1c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_30b1c() -> ! {
    todo!("0x30b1c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_")
}

// 0x30b38 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
// demangled: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_30b38() -> ! {
    todo!("0x30b38 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const")
}

// 0x30b40 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::s")]
pub fn stub_30b40() -> ! {
    todo!("0x30b40 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<c")
}

// 0x30c28 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::s")]
pub fn stub_30c28() -> ! {
    todo!("0x30c28 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<c")
}

// 0x30d3c — __ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvRKSsS9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string const&,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
// type: int(void)
// was: boost::shared_ptr
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn stub_30d3c() -> ! {
    todo!("0x30d3c void boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::")
}

// 0x30eac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_30eac() -> ! {
    todo!("0x30eac boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_")
}

// 0x30fe0 — __ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
// demangled: boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_30fe0() -> ! {
    todo!("0x30fe0 boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Gam")
}

// 0x310a8 — __ZN5boost6detail12shared_countC2IN3RBX16SecurePlayerGameEEEPT_
// demangled: boost::detail::shared_count::shared_count<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")]
pub fn stub_310a8() -> ! {
    todo!("0x310a8 boost::detail::shared_count::shared_count<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")
}

// 0x3119c — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED1Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")]
pub fn stub_3119c() -> ! {
    todo!("0x3119c boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")
}

// 0x311a0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED0Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")]
pub fn stub_311a0() -> ! {
    todo!("0x311a0 boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")
}

// 0x311a4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE7disposeEv
// demangled: boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::dispose(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::dispose(void)")]
pub fn stub_311a4() -> ! {
    todo!("0x311a4 boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::dispose(void)")
}

// 0x311b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_deleter(std::type_info const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_deleter(std::type_info const&)")]
pub fn stub_311b4() -> ! {
    todo!("0x311b4 boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_deleter(std::type_info const&)")
}

// 0x311b8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_untyped_deleter(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_untyped_deleter(void)")]
pub fn stub_311b8() -> ! {
    todo!("0x311b8 boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_untyped_deleter(void)")
}

// 0x311bc — __ZN5boost6detail12shared_countC2IN3RBX19UnsecuredStudioGameEEEPT_
// demangled: boost::detail::shared_count::shared_count<RBX::UnsecuredStudioGame>(RBX::UnsecuredStudioGame *)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::UnsecuredStudioGame>(RBX::UnsecuredStudioGame *)")]
pub fn stub_311bc() -> ! {
    todo!("0x311bc boost::detail::shared_count::shared_count<RBX::UnsecuredStudioGame>(RBX::UnsecuredStudioGame *)")
}

// 0x312b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEED1Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()")]
pub fn stub_312b0() -> ! {
    todo!("0x312b0 boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()")
}

// 0x312b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEED0Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()")]
pub fn stub_312b4() -> ! {
    todo!("0x312b4 boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()")
}

// 0x312b8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE7disposeEv
// demangled: boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::dispose(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::dispose(void)")]
pub fn stub_312b8() -> ! {
    todo!("0x312b8 boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::dispose(void)")
}

// 0x312c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_deleter(std::type_info const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_deleter(std::type_info const&)")]
pub fn stub_312c8() -> ! {
    todo!("0x312c8 boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_deleter(std::type_info const&)")
}

// 0x312cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_untyped_deleter(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_untyped_deleter(void)")]
pub fn stub_312cc() -> ! {
    todo!("0x312cc boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_untyped_deleter(void)")
}