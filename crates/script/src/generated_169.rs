// Auto-generated skeletons for rbx-script — Lua|Script|CodeGen|Yield batch
// Filter: Lua|Script|CodeGen|Yield (4818 filtered, 0 remaining) -> global gap filler EA-sorted asc next 150 not yet in script crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x38770..0x419c8 EA-sorted asc next 150 global not yet in script crate (script 16383 -> 16533 distinct, global 85545->85545 distinct)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_script_gap_031bf0::RobloxViewState;
use std::sync::LazyLock;

/// Ogre `LogManager` singleton latch (IDA 0x39920: guarded once-alloc at
/// 0x3998c..0x399d2; the manager peers fold into the host).
static LOG_MANAGER_READY: LazyLock<bool> = LazyLock::new(|| true);
use std::collections::BTreeMap;

/// `signal<void()>` connection state for view callbacks (IDA 0x3a390: slot
/// alloc at 0x3a3a8..0x3a3e6 plus insert at 0x3a3ea).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ViewSignal {
    pub slots: u32,
    pub next_id: u32,
    pub fired: u32,
}

impl ViewSignal {
    /// `connect` (IDA 0x3a390): allocates the slot and answers the
    /// connection id.
    pub fn connect(&mut self) -> u32 {
        self.slots += 1;
        self.next_id += 1;
        self.next_id
    }

    /// `insert` (IDA 0x3be00): links a slot under the mutex.
    pub fn insert(&mut self) {
        self.slots += 1;
    }

    /// `remove` (IDA 0x3cf40): unlinks the slot (saturating).
    pub fn remove(&mut self) {
        self.slots = self.slots.saturating_sub(1);
    }

    /// Bind `operator()` (IDA 0x3cf28): the member call.
    pub fn fire(&mut self) {
        self.fired += 1;
    }
}

/// Opaque render-settings singleton handle (IDA 0x3a408: once-init folds
/// into the host).
static RENDER_SETTINGS_SINGLETON: LazyLock<u32> = LazyLock::new(|| 1);
/// `Name::doDeclare<sRunService>` singleton (IDA 0x3ae20: guarded once-init
/// at 0x3ae7c..0x3aea8 answering the static at 0x3aed6).
static RUN_SERVICE_NAME: LazyLock<&'static str> = LazyLock::new(|| "RunService");
/// `ServiceProvider::doGetClassIndex<RunService>` once-index (IDA 0x3af08:
/// guarded counter at 0x3af64..0x3afb2 folds into the host).
static RUNSERVICE_CLASS_INDEX: LazyLock<u32> = LazyLock::new(|| 1);
/// `Name::doDeclare<sControllerService>` singleton (IDA 0x3b828: guarded
/// once-init answering the static, same shape as 0x3ae20).
static CONTROLLER_SERVICE_NAME: LazyLock<&'static str> = LazyLock::new(|| "ControllerService");
/// `ServiceProvider::doGetClassIndex<ControllerService>` once-index (IDA
/// 0x3b910: same guarded-counter shape as 0x3af08).
static CONTROLLERSERVICE_CLASS_INDEX: LazyLock<u32> = LazyLock::new(|| 1);
/// Opaque `signal<void()>` static mutex handle (IDA 0x3c920: guarded
/// once-init, same shape as 0x3d5b0).
static SIGNAL_VOID_MUTEX: LazyLock<u32> = LazyLock::new(|| 1);
/// Opaque void-signal slot static mutex handle (IDA 0x3d030: same shape as
/// 0x3d938).
static SLOT_VOID_MUTEX: LazyLock<u32> = LazyLock::new(|| 1);
/// `RobloxView::ViewUpdateJob` observable state (IDA 0x403f0..0x406b4):
/// the ctor names the job "UpdateRbxView" (0x4045a, folds into the host);
/// liveness, steps, and the last error are observed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UpdateJobState {
    pub live: bool,
    pub steps: u32,
    pub last_error: f64,
}

/// RenderJob bind typeinfo (IDA 0x402a8, same manage shape as 0x3e030).
pub const RENDER_JOB_BIND_TYPEINFO: &str = "bind_t<RenderJob,ViewBase>";
/// `__GLOBAL__I_a_10` one-shot latch (IDA 0x4070c, same static-init shape
/// as `GLOBAL_A9_INIT`).
static GLOBAL_A10_INIT: LazyLock<u32> = LazyLock::new(|| 1);

/// `UserInfo` observable state (IDA 0x40984..0x419c8): credentials, login
/// persistence (`LastUserLoggedIn`), balances, and request latches. ObjC
/// peers and the cookie store fold into the host.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
    pub logged_in: bool,
    pub last_user: Option<String>,
    pub robux: i64,
    pub tix: i64,
    pub player_info_requested: bool,
    pub request_url: String,
    pub cookies_cleared: bool,
}

/// `convertToFriendlyString` (IDA 0x411a0): null answers "unknown"
/// (0x411b2..0x411b4); below 1000 plain "%d" (0x41214/0x41280); below
/// 1000000 (0xF4240) "%d,%03d" (0x4125a); otherwise "%d mil" (0x4120e).
pub fn friendly_string(value: Option<i64>) -> String {
    match value {
        None => "unknown".to_owned(),
        Some(v) if v < 1000 => format!("{v}"),
        Some(v) if v < 1_000_000 => format!("{},{:03}", v / 1000, v % 1000),
        Some(v) => format!("{} mil", v / 1_000_000),
    }
}
/// `RobloxView::RenderJob` observable state (IDA 0x3ecf0..0x3fb9c): the
/// ctor wires view/marshaller/datamodel peers (folds into the host);
/// wake/prepare/perform latches, the step count, the last error, metric
/// overrides, and the graphics mode are observed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderJobState {
    pub live: bool,
    pub awake: bool,
    pub prepared: bool,
    pub performed: bool,
    pub steps: u32,
    pub last_error: f64,
    pub metrics: Vec<(String, f32)>,
    pub graphics_mode: String,
}

/// Render-schedule bind typeinfo answers (IDA 0x40160/0x401f0, same manage
/// shape as 0x3e030).
pub const RENDER_PERFORM_BIND_TYPEINFO: &str = "bind_t<scheduleRenderPerform>";
pub const IMETRIC_BIND_TYPEINFO: &str = "bind_t<IMetric>";

/// Render-schedule invocation counter (IDA 0x401dc/0x40270/0x4027c).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RenderCallback {
    pub calls: u32,
}
/// `intrusive_ptr_target` strong/weak counters (IDA 0x3d240: zeroed at
/// 0x3d250/0x3d29c with alignment asserts, atomic.h:135, folding into the
/// host).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SlotCounts {
    pub strong: u32,
    pub weak: u32,
}

/// Task sequence step counter (IDA 0x3ebb0/0x3ebb4: thunks to
/// `SequenceBase::advance`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TaskSequence {
    pub steps: u32,
}

impl TaskSequence {
    pub fn advance(&mut self) -> u32 {
        self.steps += 1;
        self.steps
    }
}

/// Ogre `LogManager` scoped-slot latch (IDA 0x3ec30/0x3ec34: D1 forwards to
/// the deleting D2).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LogManagerSlot {
    pub live: bool,
}

/// Created service instance record (IDA 0x3a798/0x3b674: sized operator new
/// plus ctor plus shared_ptr wrap fold into the host; liveness observed).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreatedInstance {
    pub class: &'static str,
    pub live: bool,
}

/// `enable_shared_from_this` owner slot (IDA 0x3a930: accepts the owner
/// when no owner is held, 0x3a986; weak mechanics fold into `Arc`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OwnerSlot {
    pub has_owner: bool,
}

// 0x38770 — ____ZN10RobloxView18doRestartDataModelEv_block_invoke
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "____ZN10RobloxView18doRestartDataModelEv_block_invoke")]
pub fn stub_0x38770(state: &mut RobloxViewState) {
    // IDA 0x38770: `doRestartDataModel` block captures the game and runs
    // the restart on the main queue (cf. `restartDataModel` at 0x386d0 in
    // generated_script_gap_031bf0.rs). Queue hop folds into the caller.
    state.restart_queued = true;
}

// 0x38cd0 — __ZN10RobloxView17setupNewDataModelEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::setupNewDataModel(void)")]
pub fn stub_0x38cd0(state: &mut RobloxViewState) {
    // IDA 0x38cd0: `setupNewDataModel` builds fresh DataModel/Overlay
    // peers and binds them into the view (cf. `bindWorkspace` at 0x380a4).
    // Peers fold into host ownership; the bound triple latches.
    state.workspace_bound = true;
}

// 0x39018 — ____ZN10RobloxView15newGameDidStartEv_block_invoke
#[doc(alias = "____ZN10RobloxView15newGameDidStartEv_block_invoke")]
pub fn stub_0x39018(state: &mut RobloxViewState) {
    // IDA 0x39018: `newGameDidStart` block calls `requestResumeRendering`
    // (0x3901a, cf. 0x37378). The suspension latch clears.
    state.rendering_suspended = false;
}

// 0x39020 — __ZN10RobloxViewD1Ev
// type: void __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::~RobloxView()")]
pub fn stub_0x39020() {
    // IDA 0x39020: D1 dtor forwards to the D2 dtor (thunk); drop glue
    // covers it — no-op.
}

// 0x39024 — __ZN10RobloxViewD2Ev
// type: void __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::~RobloxView() [0x39024]")]
pub fn stub_0x39024() {
    // IDA 0x39024: D2 dtor tears down mutexes, jobs, and peers; all fold
    // into host ownership — no-op.
}

// 0x39674 — __ZN10RobloxView11create_viewEN5boost10shared_ptrIN3RBX4GameEEEjjSsSsSs
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, std::string *, std::string *)
#[doc(alias = "RobloxView::create_view(rbx_core::SharedPtr<RBX::Game>,unsigned int,unsigned int,std::string,std::string,std::string)")]
pub fn stub_0x39674(
    width: u32,
    height: u32,
    a: &str,
    b: &str,
    c: &str,
    has_game: bool,
) -> RobloxViewState {
    // IDA 0x39674: `create_view` builds the view like the ctor at 0x37628
    // and preps it with the game like 0x37b3c (both in
    // generated_script_gap_031bf0.rs); construction plumbing folds into
    // host ownership.
    RobloxViewState {
        width,
        height,
        params: [a.to_owned(), b.to_owned(), c.to_owned()],
        view_prepped: has_game,
        ..RobloxViewState::default()
    }
}

// 0x39920 — __ZL14initLogManagerv
// type: _DWORD __fastcall()
#[doc(alias = "initLogManager(void)")]
pub fn stub_0x39920(bundle_path: &str) -> String {
    // IDA 0x39920: `initLogManager` resolves the bundle path (0x3993e, cf.
    // `macBundlePath` at 0x375b4) and one-shots the Ogre LogManager
    // (0x3998c..0x399d2). The path is the observed input.
    let _ = *LOG_MANAGER_READY;
    bundle_path.to_owned()
}

// 0x39be0 — __ZNSt12domain_errorD0Ev
// type: void __cdecl(std::domain_error *__hidden this)
#[doc(alias = "std::domain_error::~domain_error()")]
pub fn stub_0x39be0() {
    // IDA 0x39be0: D0 dtor runs the base dtor (0x39be6) plus `operator
    // delete` (0x39bf0); both fold into drop glue — no-op.
}

// 0x39bf8 — __ZNSt12domain_errorD2Ev
// type: void __cdecl(std::domain_error *__hidden this)
#[doc(alias = "std::domain_error::~domain_error() [0x39bf8]")]
pub fn stub_0x39bf8() {
    // IDA 0x39bf8: D2 dtor runs the base dtor; drop glue covers it —
    // no-op.
}

// 0x39c00 — __ZNSt16invalid_argumentD1Ev
// type: void __cdecl(std::invalid_argument *__hidden this)
#[doc(alias = "std::invalid_argument::~invalid_argument()")]
pub fn stub_0x39c00() {
    // IDA 0x39c00: D1 dtor runs the base dtor; drop glue covers it —
    // no-op.
}

// 0x39c08 — __ZNSt12length_errorD0Ev
// type: void __cdecl(std::length_error *__hidden this)
#[doc(alias = "std::length_error::~length_error()")]
pub fn stub_0x39c08() {
    // IDA 0x39c08: D0 dtor (base dtor plus delete, same shape as 0x39be0)
    // — no-op.
}

// 0x39c20 — __ZNSt12out_of_rangeD1Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
pub fn stub_0x39c20() {
    // IDA 0x39c20: D1 dtor runs the base dtor; drop glue covers it —
    // no-op.
}

// 0x39c28 — __ZNSt11range_errorD0Ev
// type: void __cdecl(std::range_error *__hidden this)
#[doc(alias = "std::range_error::~range_error()")]
pub fn stub_0x39c28() {
    // IDA 0x39c28: D0 dtor (same shape as 0x39be0) — no-op.
}

// 0x39c40 — __ZNSt11range_errorD2Ev
// type: void __cdecl(std::range_error *__hidden this)
#[doc(alias = "std::range_error::~range_error() [0x39c40]")]
pub fn stub_0x39c40() {
    // IDA 0x39c40: D2 dtor runs the base dtor; drop glue covers it —
    // no-op.
}

// 0x39c48 — __ZNSt14overflow_errorD1Ev
// type: void __cdecl(std::overflow_error *__hidden this)
#[doc(alias = "std::overflow_error::~overflow_error()")]
pub fn stub_0x39c48() {
    // IDA 0x39c48: D1 dtor runs the base dtor; drop glue covers it —
    // no-op.
}

// 0x39c50 — __ZNSt15underflow_errorD0Ev
// type: void __cdecl(std::underflow_error *__hidden this)
#[doc(alias = "std::underflow_error::~underflow_error()")]
pub fn stub_0x39c50() {
    // IDA 0x39c50: D0 dtor (same shape as 0x39be0) — no-op.
}

// 0x39c68 — __ZNSt15underflow_errorD2Ev
// type: void __cdecl(std::underflow_error *__hidden this)
#[doc(alias = "std::underflow_error::~underflow_error() [0x39c68]")]
pub fn stub_0x39c68() {
    // IDA 0x39c68: D2 dtor runs the base dtor; drop glue covers it —
    // no-op.
}

// 0x39d7c — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEE5resetEv
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::reset(void)")]
pub fn stub_0x39d7c() {
    // IDA 0x39d7c: `shared_ptr<RenderJob>::reset()` nulls the pointer
    // (0x39da2..0x39da8) and releases (0x39dca..0x39dd2, same shape as
    // 0x3a660); the drop folds into `Arc` — no-op.
}

// 0x39e10 — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv
#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::reset(void)")]
pub fn stub_0x39e10() {
    // IDA 0x39e10: `shared_ptr<ViewUpdateJob>::reset()` — same null plus
    // release shape as 0x39d7c; `Arc` glue covers it — no-op.
}

// 0x39ea8 — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_
#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::operator=(rbx_core::SharedPtr<RobloxView::ViewUpdateJob>&&)")]
pub fn stub_0x39ea8() {
    // IDA 0x39ea8: move-assign steals the source pair, nulling it
    // (0x39ed6..0x39ed8), installs it (0x39edc..0x39ee6), and releases the
    // old count (0x39f02..0x39f0a); move glue covers it — no-op.
}

// 0x39f4c — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::shared_ptr<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
pub fn stub_0x39f4c() {
    // IDA 0x39f4c: ctor-from-pointer installs the pointer (0x39f7c..
    // 0x39f84) and builds the count block (0x39faa); `Arc` construction
    // glue covers it — no-op.
}

// 0x3a030 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEaSEOS3_
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::operator=(rbx_core::SharedPtr<RobloxView::RenderJob>&&)")]
pub fn stub_0x3a030() {
    // IDA 0x3a030: move-assign for RenderJob (same steal shape as 0x39ea8)
    // — no-op.
}

// 0x3a0d4 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEC1IS2_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::shared_ptr<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
pub fn stub_0x3a0d4() {
    // IDA 0x3a0d4: ctor-from-pointer for RenderJob (same shape as 0x39f4c)
    // — no-op.
}

// 0x3a1b8 — __ZN17QuitEventListenerD1Ev
// type: void __fastcall(QuitEventListener *__hidden this)
#[doc(alias = "QuitEventListener::~QuitEventListener()")]
pub fn stub_0x3a1b8() {
    // IDA 0x3a1b8: D1 dtor has an empty body; drop glue covers it — no-op.
}

// 0x3a2ec — __ZN5boost10shared_ptrIN3RBX9DataModelEEaSINS1_16OverlayDataModelEEERS3_ONS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>& rbx_core::SharedPtr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(rbx_core::SharedPtr<RBX::OverlayDataModel> &&)")]
pub fn stub_0x3a2ec() {
    // IDA 0x3a2ec: move-assign from OverlayDataModel (same steal shape as
    // 0x39ea8) — no-op.
}

// 0x3a390 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>> const&)")]
pub fn stub_0x3a390(sig: &mut ViewSignal) -> u32 {
    // IDA 0x3a390: `connect` for the `RobloxView` member bind — see
    // `ViewSignal::connect`.
    sig.connect()
}

// 0x3a408 — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEE9singletonEv")]
pub fn stub_0x3a408() -> u32 {
    // IDA 0x3a408: settings-item singleton — see
    // `RENDER_SETTINGS_SINGLETON`.
    *RENDER_SETTINGS_SINGLETON
}

// 0x3a790 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev")]
pub fn stub_0x3a790() {
    // IDA 0x3a790: `Creator` D1 dtor; drop glue covers it — no-op.
}

// 0x3a798 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6CameraEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)")]
pub fn stub_0x3a798() -> CreatedInstance {
    // IDA 0x3a798: `create<Camera>` news 0x1DC bytes (0x3a7ce), runs the
    // ctor (0x3a7f2), and wraps (0x3a800).
    CreatedInstance { class: "Camera", live: true }
}

// 0x3a930 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6CameraES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Camera,RBX::Camera>(rbx_core::SharedPtr<RBX::Camera> const*,RBX::Camera *)const")]
pub fn stub_0x3a930(slot: &mut OwnerSlot) {
    // IDA 0x3a930: `_internal_accept_owner` — see `OwnerSlot` (the
    // use-count gate at 0x3a986 folds into the latch).
    slot.has_owner = true;
}

// 0x3aa10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3aa10() {
    // IDA 0x3aa10: D0 dtor (teardown plus delete); drop glue covers it —
    // no-op.
}

// 0x3aa18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x3aa18() -> u32 {
    // IDA 0x3aa18: `get_deleter` answers non-null only on exact deleter
    // typeinfo match (0x3a2a..0x3aa2e); the host holds no deleters.
    0
}

// 0x3aa30 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_ESH_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>)")]
pub fn stub_0x3aa30(map: &mut BTreeMap<u32, u32>) {
    // IDA 0x3aa30 `_Rb_tree<Name, ICreator>::erase(first,last)`: full-range
    // erase clears (0x3aa4a..0x3aa88). Host has no tree nodes; granularity
    // collapses to the owning map.
    map.clear();
}

// 0x3aa90 — __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEED1Ev
#[doc(alias = "std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::~map()")]
pub fn stub_0x3aa90(map: &mut BTreeMap<u32, u32>) {
    // IDA 0x3aa90 `map<Name, ICreator>::~map()`: erases all (0x3aa98);
    // drop glue covers the nodes.
    map.clear();
}

// 0x3aaa0 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev")]
pub fn stub_0x3aaa0() {
    // IDA 0x3aaa0: `Creator` C2 ctor; construction plumbing folds into the
    // host — no-op.
}

// 0x3acc8 — __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x3acc8(map: &mut BTreeMap<u32, u32>, key: u32) -> u32 {
    // IDA 0x3acc8 `map::operator[]`: lower-bound probe with insert-default
    // on miss (0x3acd2..0x3ad14, cf. 0x23a04 in generated_110.rs). Host has
    // no nodes; the default creator id is 0.
    *map.entry(key).or_insert(0)
}

// 0x3ad20 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::pair<RBX::Name const* const,RBX::ICreator const*> const&)")]
pub fn stub_0x3ad20(map: &mut BTreeMap<u32, u32>, key: u32, value: u32) -> bool {
    // IDA 0x3ad20 `_M_insert_unique`: inserts on miss (cf. 0x243b0 in
    // generated_110.rs). Answers inserted (true) vs already present.
    if map.contains_key(&key) {
        false
    } else {
        map.insert(key, value);
        true
    }
}

// 0x3add8 — __ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v")]
pub fn stub_0x3add8() -> &'static str {
    // IDA 0x3add8: `Name::declare<sRunService>` thunk forwarding to the
    // `doDeclare` shim (same shape as 0x26a4f8).
    stub_0x3ae20()
}

// 0x3ae20 — __ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v")]
pub fn stub_0x3ae20() -> &'static str {
    // IDA 0x3ae20: `doDeclare<sRunService>` — see `RUN_SERVICE_NAME`.
    *RUN_SERVICE_NAME
}

// 0x3af08 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10RunServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RunService>(void)")]
pub fn stub_0x3af08() -> u32 {
    // IDA 0x3af08: `doGetClassIndex<RunService>` — see
    // `RUNSERVICE_CLASS_INDEX`.
    *RUNSERVICE_CLASS_INDEX
}

// 0x3afe0 — __ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3afe0() {
    // IDA 0x3afe0: `shared_ptr` ctor installs the pointer (0x3afe6),
    // builds the count (0x3afec), and accepts the owner (0x3affc); `Arc`
    // construction glue covers it — no-op.
}

// 0x3b008 — __ZN5boost6detail12shared_countC2IPN3RBX10RunServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3b008() {
    // IDA 0x3b008: `shared_count` ctor — same control block shape as
    // 0x3b14c; `Arc` glue covers it — no-op.
}

// 0x3b108 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x3b108]")]
pub fn stub_0x3b108() {
    // IDA 0x3b108: D1 dtor has an empty body; drop glue covers it — no-op.
}

// 0x3b110 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x3b110() {
    // IDA 0x3b110: `dispose` (same shape as 0x3b278) — no-op.
}

// 0x3b130 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x3b130() -> u32 {
    // IDA 0x3b130: `get_deleter` answers null without an exact deleter
    // match (same shape as 0x3aa18).
    0
}

// 0x3b148 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x3b148() -> u32 {
    // IDA 0x3b148: `get_untyped_deleter` answers null (same shape as
    // 0x3b330).
    0
}

// 0x3b268 — __ZN3RBX5Tasks11Coordinator9onPreStepEPNS_13TaskScheduler3JobE
// type: void()
#[doc(alias = "RBX::Tasks::Coordinator::onPreStep(RBX::TaskScheduler::Job *)")]
pub fn stub_0x3b268() {
    // IDA 0x3b268: `Coordinator::onPreStep` has an empty body — no-op.
}

// 0x3b26c — __ZN3RBX5Tasks11Coordinator10onPostStepEPNS_13TaskScheduler3JobE
// type: void()
#[doc(alias = "RBX::Tasks::Coordinator::onPostStep(RBX::TaskScheduler::Job *)")]
pub fn stub_0x3b26c() {
    // IDA 0x3b26c: `Coordinator::onPostStep` has an empty body — no-op.
}

// 0x3b518 — __ZNK3RBX15ServiceProvider4findINS_17ControllerServiceEEEPT_v
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::find<RBX::ControllerService>(void)const")]
pub fn stub_0x3b518(present: bool) -> Option<u32> {
    // IDA 0x3b518: `find<ControllerService>` walks the provider tables
    // (folds into the host) and answers the service or null.
    present.then_some(1)
}

// 0x3b674 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17ControllerServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)")]
pub fn stub_0x3b674() -> CreatedInstance {
    // IDA 0x3b674: `create<ControllerService>` news 0x64 bytes (0x3b6a8),
    // runs the ctor (0x3b6cc), and wraps (0x3b6da).
    CreatedInstance { class: "ControllerService", live: true }
}

// 0x3b724 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17ControllerServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ControllerService>(rbx_core::SharedPtr<RBX::ControllerService> const&)")]
pub fn stub_0x3b724() {
    // IDA 0x3b724: `shared_ptr<Instance>::operator=(const ControllerService
    // ref)` add-refs, swaps, and releases (same shape as 0x3a1bc); `Arc`
    // glue covers it — no-op.
}

// 0x3b7e0 — __ZN3RBX4Name7declareILZNS_18sControllerServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_18sControllerServiceEEEERKS0_v")]
pub fn stub_0x3b7e0() -> &'static str {
    // IDA 0x3b7e0: `Name::declare<sControllerService>` thunk forwarding to
    // the `doDeclare` shim (same shape as 0x26a4f8).
    stub_0x3b828()
}

// 0x3b828 — __ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v")]
pub fn stub_0x3b828() -> &'static str {
    // IDA 0x3b828: `doDeclare<sControllerService>` — see
    // `CONTROLLER_SERVICE_NAME`.
    *CONTROLLER_SERVICE_NAME
}

// 0x3b910 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17ControllerServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ControllerService>(void)")]
pub fn stub_0x3b910() -> u32 {
    // IDA 0x3b910: `doGetClassIndex<ControllerService>` — see
    // `CONTROLLERSERVICE_CLASS_INDEX`.
    *CONTROLLERSERVICE_CLASS_INDEX
}

// 0x3b9e8 — __ZN5boost10shared_ptrIN3RBX17ControllerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3b9e8() {
    // IDA 0x3b9e8: `shared_ptr` ctor installs the pointer, builds the
    // count, and accepts the owner (same shape as 0x3afe0); `Arc` glue
    // covers it — no-op.
}

// 0x3ba10 — __ZN5boost6detail12shared_countC2IPN3RBX17ControllerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3ba10() {
    // IDA 0x3ba10: `shared_count` ctor (same shape as 0x3b14c); `Arc` glue
    // covers it — no-op.
}

// 0x3bb10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3bb10() {
    // IDA 0x3bb10: D1 dtor has an empty body; drop glue covers it — no-op.
}

// 0x3bb18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x3bb18() {
    // IDA 0x3bb18: `dispose` (same shape as 0x3b278) — no-op.
}

// 0x3bb38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x3bb38() -> u32 {
    // IDA 0x3bb38: `get_deleter` answers null without an exact deleter
    // match (same shape as 0x3aa18).
    0
}

// 0x3bb50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x3bb50() -> u32 {
    // IDA 0x3bb50: `get_untyped_deleter` answers null (same shape as
    // 0x3b330).
    0
}

// 0x3bb58 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv")]
pub fn stub_0x3bb58(class_name: Option<&str>) -> bool {
    // IDA 0x3bb58: `isNullClassName` asserts the empty/name-null
    // correspondence (object.h:360, 0x3bb7a..0x3bbac) and answers whether
    // the class name is null (0x3bbf4).
    class_name.is_none()
}

// 0x3bbf8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>::operator=(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x3bbf8() {
    // IDA 0x3bbf8: `shared_ptr<Instance>::operator=` (same shape as
    // 0x3a1bc); `Arc` glue covers it — no-op.
}

// 0x3bcb8 — __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0>::operator delete(void *)")]
pub fn stub_0x3bcb8(strong: u32) {
    // IDA 0x3bcb8: slot-target `operator delete` release-asserts a zero
    // strong count (intrusive_ptr_target.h:133, 0x3bcf6..0x3bd64) before
    // freeing (folds into drop).
    if strong != 0 {
        panic!("c->strong == 0 file: ../Base/include/rbx/intrusive_ptr_target.h line: 133");
    }
}

// 0x3be00 — __ZN3rbx7signals6signalIFvvEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::insert(rbx::signals::signal<void ()(void)>::slot *)")]
pub fn stub_0x3be00(sig: &mut ViewSignal) {
    // IDA 0x3be00: void-signal `insert` — see `ViewSignal::insert`.
    sig.insert();
}

// 0x3c920 — __ZN3rbx7signals6signalIFvvEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(void)>::safe_static_init_mutex(void)")]
pub fn stub_0x3c920() -> u32 {
    // IDA 0x3c920: void-signal `safe_static_init_mutex` — see
    // `SIGNAL_VOID_MUTEX`.
    *SIGNAL_VOID_MUTEX
}

// 0x3cdb8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()")]
pub fn stub_0x3cdb8() {
    // IDA 0x3cdb8: `callable_slot` D1 dtor; drop glue covers it — no-op.
}

// 0x3ce64 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot() [0x3ce64]")]
pub fn stub_0x3ce64() {
    // IDA 0x3ce64: `callable_slot` D0 dtor; drop glue covers it — no-op.
}

// 0x3cf18 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)")]
pub fn stub_0x3cf18(sig: &mut ViewSignal) {
    // IDA 0x3cf18: void-flavor `callable::call` invoking the bound member;
    // the call folds into the fire count.
    sig.fire();
}

// 0x3cf20 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)")]
pub fn stub_0x3cf20(sig: &mut ViewSignal) {
    // IDA 0x3cf20: thn4 `call` adjusts `this` and forwards.
    stub_0x3cf18(sig);
}

// 0x3cf28 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>::operator()(void)")]
pub fn stub_0x3cf28(sig: &mut ViewSignal) {
    // IDA 0x3cf28: bind `operator()` — see `ViewSignal::fire`.
    sig.fire();
}

// 0x3cf40 — __ZN3rbx7signals6signalIFvvEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::remove(rbx::signals::signal<void ()(void)>::slot *)")]
pub fn stub_0x3cf40(sig: &mut ViewSignal) {
    // IDA 0x3cf40: void-signal `remove` — see `ViewSignal::remove`.
    sig.remove();
}

// 0x3d030 — __ZN3rbx7signals6signalIFvvEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x3d030() -> u32 {
    // IDA 0x3d030: void-signal slot `safe_static_init_mutex` — see
    // `SLOT_VOID_MUTEX`.
    *SLOT_VOID_MUTEX
}

// 0x3d038 — __ZN3rbx7signals6signalIFvvEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::~slot()")]
pub fn stub_0x3d038() {
    // IDA 0x3d038: void-signal `slot` D1 dtor; drop glue covers it —
    // no-op.
}

// 0x3d0e4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()")]
pub fn stub_0x3d0e4() {
    // IDA 0x3d0e4: `callable` D1 dtor; drop glue covers it — no-op.
}

// 0x3d190 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable() [0x3d190]")]
pub fn stub_0x3d190() {
    // IDA 0x3d190: `callable` D0 dtor; drop glue covers it — no-op.
}

// 0x3d240 — __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EE6countsC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0>::counts::counts(void)")]
pub fn stub_0x3d240() -> SlotCounts {
    // IDA 0x3d240: `counts` ctor — see `SlotCounts`.
    SlotCounts::default()
}

// 0x3dc58 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_deleter(std::type_info const&)")]
pub fn stub_0x3dc58() -> u32 {
    // IDA 0x3dc58: `get_deleter` answers null without an exact deleter
    // match (same shape as 0x3aa18).
    0
}

// 0x3dc60 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(rbx_core::SharedPtr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const")]
pub fn stub_0x3dc60(slot: &mut OwnerSlot) {
    // IDA 0x3dc60: `_internal_accept_owner` for RenderJob (same use-count
    // gate shape as 0x3a930).
    slot.has_owner = true;
}

// 0x3dd34 — __ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
pub fn stub_0x3dd34() {
    // IDA 0x3dd34: `shared_count` ctor (same shape as 0x3b14c); `Arc` glue
    // covers it — no-op.
}

// 0x3de28 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
pub fn stub_0x3de28() {
    // IDA 0x3de28: D1 dtor has an empty body; drop glue covers it — no-op.
}

// 0x3de2c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p() [0x3de2c]")]
pub fn stub_0x3de2c() {
    // IDA 0x3de2c: D0 dtor (teardown plus delete); drop glue covers it —
    // no-op.
}

// 0x3de30 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::dispose(void)")]
pub fn stub_0x3de30() {
    // IDA 0x3de30: `dispose` (same shape as 0x3b278) — no-op.
}

// 0x3de40 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_deleter(std::type_info const&)")]
pub fn stub_0x3de40() -> u32 {
    // IDA 0x3de40: `get_deleter` answers null (same shape as 0x3aa18).
    0
}

// 0x3de44 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_untyped_deleter(void)")]
pub fn stub_0x3de44() -> u32 {
    // IDA 0x3de44: `get_untyped_deleter` answers null (same shape as
    // 0x3b330).
    0
}

// 0x3de48 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(rbx_core::SharedPtr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const")]
pub fn stub_0x3de48(slot: &mut OwnerSlot) {
    // IDA 0x3de48: `_internal_accept_owner` for ViewUpdateJob (same shape
    // as 0x3dc60).
    slot.has_owner = true;
}

// 0x3df1c — __ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
pub fn stub_0x3df1c() {
    // IDA 0x3df1c: `shared_count` ctor (same shape as 0x3b14c); `Arc` glue
    // covers it — no-op.
}

// 0x3e010 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()")]
pub fn stub_0x3e010() {
    // IDA 0x3e010: D1 dtor has an empty body; drop glue covers it — no-op.
}

// 0x3e014 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p() [0x3e014]")]
pub fn stub_0x3e014() {
    // IDA 0x3e014: D0 dtor (teardown plus delete); drop glue covers it —
    // no-op.
}

// 0x3e018 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::dispose(void)")]
pub fn stub_0x3e018() {
    // IDA 0x3e018: `dispose` (same shape as 0x3b278) — no-op.
}

// 0x3e028 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_deleter(std::type_info const&)")]
pub fn stub_0x3e028() -> u32 {
    // IDA 0x3e028: `get_deleter` answers null (same shape as 0x3aa18).
    0
}

// 0x3e02c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_untyped_deleter(void)")]
pub fn stub_0x3e02c() -> u32 {
    // IDA 0x3e02c: `get_untyped_deleter` answers null (same shape as
    // 0x3b330).
    0
}

// 0x3e0b0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI19CRenderSettingsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<CRenderSettingsItem,CRenderSettingsItem>(rbx_core::SharedPtr<CRenderSettingsItem> const*,CRenderSettingsItem *)const")]
pub fn stub_0x3e0b0(slot: &mut OwnerSlot) {
    // IDA 0x3e0b0: `_internal_accept_owner` for RenderSettingsItem (same
    // shape as 0x3a930).
    slot.has_owner = true;
}

// 0x3e190 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3e190() {
    // IDA 0x3e190: D0 dtor (teardown plus delete); drop glue covers it —
    // no-op.
}

// 0x3e1e8 — __ZN5boost14singleton_poolIN3RBX16OnDemandInstanceELj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x3e1e8() -> u32 {
    // IDA 0x3e1e8: `singleton_pool<OnDemandInstance,20>::get_pool` — same
    // once-storage shape as 0x3e198 with block size 20 (0x3e21e).
    20
}

// 0x3ebb0 — __ZN3RBX5Tasks8Sequence9onPreStepEPNS_13TaskScheduler3JobE
// type: int __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::Sequence::onPreStep(RBX::TaskScheduler::Job *)")]
pub fn stub_0x3ebb0(seq: &mut TaskSequence) -> u32 {
    // IDA 0x3ebb0: `Sequence::onPreStep` thunk to `advance` — see
    // `TaskSequence::advance`.
    seq.advance()
}

// 0x3ebb4 — __ZN3RBX5Tasks17ExclusiveSequence10onPostStepEPNS_13TaskScheduler3JobE
// type: int __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::ExclusiveSequence::onPostStep(RBX::TaskScheduler::Job *)")]
pub fn stub_0x3ebb4(seq: &mut TaskSequence) -> u32 {
    // IDA 0x3ebb4: `ExclusiveSequence::onPostStep` thunk to `advance` —
    // same shape as 0x3ebb0.
    seq.advance()
}

// 0x3ec30 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED1Ev
#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")]
pub fn stub_0x3ec30(slot: &mut LogManagerSlot) {
    // IDA 0x3ec30: D1 thunk forwarding to the deleting D2.
    stub_0x3ec34(slot);
}

// 0x3ec34 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED2Ev
#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr() [0x3ec34]")]
pub fn stub_0x3ec34(slot: &mut LogManagerSlot) {
    // IDA 0x3ec34: D2 dtor deletes the manager; drop glue covers it and
    // the slot is marked dead.
    slot.live = false;
}

// 0x3eccc — __ZN17QuitEventListenerD0Ev
// type: void __fastcall(QuitEventListener *__hidden this)
#[doc(alias = "QuitEventListener::~QuitEventListener() [0x3eccc]")]
pub fn stub_0x3eccc() {
    // IDA 0x3eccc: D0 thunk deleting only (empty D1 at 0x3a1b8); drop glue
    // covers it — no-op.
}

// 0x3ecd0 — __ZN4Ogre19WindowEventListener11windowMovedEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)")]
pub fn stub_0x3ecd0() {
    // IDA 0x3ecd0: `windowMoved` has an empty body — no-op.
}

// 0x3ecd4 — __ZN4Ogre19WindowEventListener13windowResizedEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)")]
pub fn stub_0x3ecd4() {
    // IDA 0x3ecd4: `windowResized` has an empty body — no-op.
}

// 0x3ecd8 — __ZN4Ogre19WindowEventListener13windowClosingEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)")]
pub fn stub_0x3ecd8() -> i32 {
    // IDA 0x3ecd8: `windowClosing` answers 1 (0x3ecda).
    1
}

// 0x3ecdc — __ZN17QuitEventListener12windowClosedEPN4Ogre12RenderWindowE
// type: _DWORD __fastcall(QuitEventListener *__hidden this, RenderWindow *)
#[doc(alias = "QuitEventListener::windowClosed(Ogre::RenderWindow *)")]
pub fn stub_0x3ecdc(log: &mut Vec<String>) -> i32 {
    // IDA 0x3ecdc: `windowClosed` logs the close request (puts shim) and
    // answers success; the stream folds into the log.
    log.push("Request to close OGRE render window received".to_owned());
    1
}

// 0x3ecec — __ZN4Ogre19WindowEventListener17windowFocusChangeEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)")]
pub fn stub_0x3ecec() {
    // IDA 0x3ecec: `windowFocusChange` has an empty body — no-op.
}

// 0x3ecf0 — __ZN10RobloxView9RenderJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerEN5boost10shared_ptrINS1_9DataModelEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
#[doc(alias = "RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)")]
pub fn stub_0x3ecf0() -> RenderJobState {
    // IDA 0x3ecf0: `RenderJob` ctor wires the view, marshaller, and
    // datamodel peers (folds into host ownership); the job starts live.
    RenderJobState { live: true, ..RenderJobState::default() }
}

// 0x3ee80 — __ZN10RobloxView9RenderJobD1Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "RobloxView::RenderJob::~RenderJob()")]
pub fn stub_0x3ee80(job: &mut RenderJobState) {
    // IDA 0x3ee80: D1 dtor tears down; drop glue covers it and the job is
    // marked dead.
    job.live = false;
}

// 0x3ef40 — __ZN10RobloxView9RenderJobD0Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "RobloxView::RenderJob::~RenderJob() [0x3ef40]")]
pub fn stub_0x3ef40(job: &mut RenderJobState) {
    // IDA 0x3ef40: D0 dtor (teardown plus delete); drop glue covers it and
    // the job is marked dead.
    job.live = false;
}

// 0x3f008 — __ZN10RobloxView9RenderJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x3f008(enabled: bool, standard: f64) -> f64 {
    // IDA 0x3f008: `sleepTime` stores +Inf when disabled (0x3f036..
    // 0x3f046, 0x7FEFFFFFFFFFFFFF) and otherwise answers
    // `computeStandardSleepTime(stats, 60.0)` (0x3f01a..0x3f02e, folds into
    // the input).
    if enabled {
        standard
    } else {
        f64::INFINITY
    }
}

// 0x3f058 — __ZN10RobloxView9RenderJob5errorERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x3f058(job: &mut RenderJobState, enabled: bool, standard: f64) {
    // IDA 0x3f058: `error` zeroes the error words when disabled
    // (0x3f084..0x3f08a) and otherwise stores
    // `computeStandardError(stats, 30.0)` (0x3f06a..0x3f07c, folds into the
    // input).
    job.last_error = if enabled { standard } else { 0.0 };
}

// 0x3f090 — __ZNK3RBX13TaskScheduler3Job26getDesiredConcurrencyCountEv
// type: int __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::getDesiredConcurrencyCount(void)const")]
pub fn stub_0x3f090() -> u32 {
    // IDA 0x3f090: `getDesiredConcurrencyCount` answers 1 (0x3f092).
    1
}

// 0x3f094 — __ZN10RobloxView9RenderJob16stepDataModelJobERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x3f094(job: &mut RenderJobState) -> u32 {
    // IDA 0x3f094: `stepDataModelJob` steps cameras and the datamodel
    // (folds into the host); the step count is observed.
    job.steps += 1;
    job.steps
}

// 0x3f598 — __ZNK10RobloxView9RenderJob14getMetricValueERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "RobloxView::RenderJob::getMetricValue(std::string const&)const")]
pub fn stub_0x3f598(job: &RenderJobState, name: &str) -> f32 {
    // IDA 0x3f598: `getMetricValue` dispatches "Render FPS", "Render Duty",
    // "Render Job Time", "Render Nominal FPS", and siblings (0x3f5c2..) to
    // job average queries; the engine queries fold into the metric table.
    job.metrics.iter().find(|(n, _)| n == name).map(|(_, v)| *v).unwrap_or(0.0)
}

// 0x3f700 — __ZNK10RobloxView9RenderJob9getMetricERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "RobloxView::RenderJob::getMetric(std::string const&)const")]
pub fn stub_0x3f700(job: &RenderJobState, name: &str) -> String {
    // IDA 0x3f700: `getMetric` formats "Graphics Mode" and siblings into
    // the out string; formatting folds into the stored mode.
    if name == "Graphics Mode" {
        job.graphics_mode.clone()
    } else {
        String::new()
    }
}

// 0x3f904 — __ZThn480_N10RobloxView9RenderJobD1Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::~RenderJob()")]
pub fn stub_0x3f904(job: &mut RenderJobState) {
    // IDA 0x3f904: thn480 D1 (adjust plus base dtor); drop glue covers it
    // and the job is marked dead.
    job.live = false;
}

// 0x3f9c8 — __ZThn480_N10RobloxView9RenderJobD0Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::~RenderJob() [0x3f9c8]")]
pub fn stub_0x3f9c8(job: &mut RenderJobState) {
    // IDA 0x3f9c8: thn480 D0 (adjust, teardown, delete); drop glue covers
    // it and the job is marked dead.
    job.live = false;
}

// 0x3fa94 — __ZThn480_NK10RobloxView9RenderJob9getMetricERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::getMetric(std::string const&)const")]
pub fn stub_0x3fa94(job: &RenderJobState, name: &str) -> f32 {
    // IDA 0x3fa94: thn480 `getMetricValue` (adjust plus dispatch, same
    // shape as 0x3f598).
    stub_0x3f598(job, name)
}

// 0x3faa4 — __ZThn480_NK10RobloxView9RenderJob14getMetricValueERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::getMetricValue(std::string const&)const")]
pub fn stub_0x3faa4(job: &RenderJobState, name: &str) -> String {
    // IDA 0x3faa4: thn480 `getMetric` (adjust plus format, same shape as
    // 0x3f700).
    stub_0x3f700(job, name)
}

// 0x3faac — __ZN10RobloxView9RenderJob21scheduleRenderPrepareEPS0_PN3RBX8ViewBaseE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, RenderJob *, ViewBase *)
#[doc(alias = "RobloxView::RenderJob::scheduleRenderPrepare(RobloxView::RenderJob*,RBX::ViewBase *)")]
pub fn stub_0x3faac(job: &mut RenderJobState) {
    // IDA 0x3faac: `scheduleRenderPrepare` schedules unless already queued
    // (flag at +632, 0x3faac..0x3fac2, folds into the latch).
    job.prepared = true;
}

// 0x3fac4 — __ZN10RobloxView9RenderJob21scheduleRenderPerformEPS0_PN3RBX8ViewBaseEd
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, RobloxView::RenderJob *, RBX::ViewBase *, double)
#[doc(alias = "RobloxView::RenderJob::scheduleRenderPerform(RobloxView::RenderJob*,RBX::ViewBase *,double)")]
pub fn stub_0x3fac4(job: &mut RenderJobState, has_model: bool) {
    // IDA 0x3fac4: `scheduleRenderPerform` schedules the perform against
    // the live datamodel (0x3fb02..0x3fb38, folds into the host).
    job.performed = has_model;
}

// 0x3fb9c — __ZN10RobloxView9RenderJob4wakeEv
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "RobloxView::RenderJob::wake(void)")]
pub fn stub_0x3fb9c(job: &mut RenderJobState) {
    // IDA 0x3fb9c: `wake` raises the wake flag (0x3fbbe) through the
    // scheduler singleton (folds into the host).
    job.awake = true;
}

// 0x40160 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x40160(_op: crate::generated_110::FunctorOp) -> &'static str {
    // IDA 0x40160: `functor_manager::manage` for the scheduleRenderPerform
    // bind — same op dispatch as 0x3e030.
    RENDER_PERFORM_BIND_TYPEINFO
}

// 0x401dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x401dc(cb: &mut RenderCallback) {
    // IDA 0x401dc: invoker thunk forwarding to the bind call (same shape
    // as 0x3e090).
    cb.calls += 1;
}

// 0x401f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x401f0(_op: crate::generated_110::FunctorOp) -> &'static str {
    // IDA 0x401f0: `functor_manager::manage` for the IMetric bind — same
    // op dispatch as 0x3e030.
    IMETRIC_BIND_TYPEINFO
}

// 0x40270 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x40270(cb: &mut RenderCallback) {
    // IDA 0x40270: invoker thunk forwarding to the bind call (same shape
    // as 0x3e090).
    cb.calls += 1;
}

// 0x4027c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double> &,boost::_bi::list0 &,int)")]
pub fn stub_0x4027c(cb: &mut RenderCallback) {
    // IDA 0x4027c: `list3` bind call invoking the member (virtual-aware at
    // 0x4029a..0x4029e, same shape as 0x3e094).
    cb.calls += 1;
}

// 0x402a8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x402a8(_op: crate::generated_110::FunctorOp) -> &'static str {
    // IDA 0x402a8: `functor_manager::manage` for the RenderJob bind — same
    // op dispatch as 0x3e030.
    RENDER_JOB_BIND_TYPEINFO
}

// 0x40308 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x40308(cb: &mut RenderCallback) {
    // IDA 0x40308: invoker thunk forwarding to the bind call (same shape
    // as 0x3e090).
    cb.calls += 1;
}

// 0x40318 — __ZN5boost8weak_ptrIN3RBX9DataModelEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::Weak<RBX::DataModel>::weak_ptr<RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)")]
pub fn stub_0x40318() {
    // IDA 0x40318: `weak_ptr` ctor from shared (convertible) — `Weak`
    // construction glue covers it — no-op.
}

// 0x403f0 — __ZN10RobloxView13ViewUpdateJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerE
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this, RBX::ViewBase *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RobloxView::ViewUpdateJob::ViewUpdateJob(RBX::ViewBase *,RBX::FunctionMarshaller *)")]
pub fn stub_0x403f0() -> UpdateJobState {
    // IDA 0x403f0: `ViewUpdateJob` ctor names the job and wires peers;
    // construction folds into host ownership; the job starts live.
    UpdateJobState { live: true, ..UpdateJobState::default() }
}

// 0x404f0 — __ZN10RobloxView13ViewUpdateJobD1Ev
// type: void __fastcall(RobloxView::ViewUpdateJob *__hidden this)
#[doc(alias = "RobloxView::ViewUpdateJob::~ViewUpdateJob()")]
pub fn stub_0x404f0(job: &mut UpdateJobState) {
    // IDA 0x404f0: D1 dtor tears down; drop glue covers it and the job is
    // marked dead.
    job.live = false;
}

// 0x4059c — __ZN10RobloxView13ViewUpdateJobD0Ev
// type: void __fastcall(RobloxView::ViewUpdateJob *__hidden this)
#[doc(alias = "RobloxView::ViewUpdateJob::~ViewUpdateJob() [0x4059c]")]
pub fn stub_0x4059c(job: &mut UpdateJobState) {
    // IDA 0x4059c: D0 dtor (teardown plus delete); drop glue covers it and
    // the job is marked dead.
    job.live = false;
}

// 0x40650 — __ZN10RobloxView13ViewUpdateJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::ViewUpdateJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x40650(enabled: bool, standard: f64) -> f64 {
    // IDA 0x40650: `sleepTime` — same settings-gated standard shape as
    // 0x3f008.
    if enabled {
        standard
    } else {
        f64::INFINITY
    }
}

// 0x40680 — __ZN10RobloxView13ViewUpdateJob5errorERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::ViewUpdateJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x40680(job: &mut UpdateJobState, enabled: bool, standard: f64) {
    // IDA 0x40680: `error` — same gated-error shape as 0x3f058.
    job.last_error = if enabled { standard } else { 0.0 };
}

// 0x406a8 — __ZN10RobloxView13ViewUpdateJob17getPriorityFactorEv
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this)
#[doc(alias = "RobloxView::ViewUpdateJob::getPriorityFactor(void)")]
pub fn stub_0x406a8() -> f64 {
    // IDA 0x406a8: `getPriorityFactor` answers 1.0 (0x406b0).
    1.0
}

// 0x406b4 — __ZN10RobloxView13ViewUpdateJob4stepERKN3RBX13TaskScheduler3Job5StatsE
#[doc(alias = "RobloxView::ViewUpdateJob::step(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x406b4(job: &mut UpdateJobState) -> u32 {
    // IDA 0x406b4: `step` updates and presents the view (0x406c6..0x406da,
    // folds into the host) and answers 1 (0x406de).
    job.steps += 1;
    1
}

// 0x4070c — __GLOBAL__I_a_10
#[doc(alias = "global constructor keyed to_a_10")]
pub fn stub_0x4070c() -> u32 {
    // IDA 0x4070c: `__GLOBAL__I_a_10` — see `GLOBAL_A10_INIT`.
    *GLOBAL_A10_INIT
}

// 0x40984 — -[UserInfo init]
// type: UserInfo *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo init]")]
pub fn stub_0x40984() -> UserInfo {
    // IDA 0x40984: `init` chains to super (0x4099e..0x409ae, folds into
    // the host); the record starts default.
    UserInfo::default()
}

// 0x409b0 — -[UserInfo setUserLoggedIn:]
// type: void __cdecl(UserInfo *self, SEL, char)
#[doc(alias = "-[UserInfo setUserLoggedIn:]")]
pub fn stub_0x409b0(user: &mut UserInfo, logged_in: bool) {
    // IDA 0x409b0: stores the flag (0x409c6); when set, persists the
    // username as LastUserLoggedIn (0x409e6..0x40a14); otherwise clears
    // credentials (0x40a34..0x40a48) and the key (0x40a64..0x40a7e); always
    // synchronizes (0x40a9a..0x40aae, folds into the host).
    user.logged_in = logged_in;
    if logged_in {
        user.last_user = Some(user.username.clone());
    } else {
        user.username.clear();
        user.password.clear();
        user.last_user = None;
    }
}

// 0x40ab4 — -[UserInfo userLoggedIn]
// type: char __cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo userLoggedIn]")]
pub fn stub_0x40ab4(user: &UserInfo) -> bool {
    // IDA 0x40ab4: `userLoggedIn` answers the flag (0x40ac2).
    user.logged_in
}

// 0x40ac4 — -[UserInfo UpdatePlayerInfo]
// type: void __cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo UpdatePlayerInfo]")]
pub fn stub_0x40ac4(user: &mut UserInfo, base_url: &str) {
    // IDA 0x40ac4: `UpdatePlayerInfo` GETs `{base}mobileapi/userinfo` over
    // https (0x40ae8..0x40b2a) with the User-Agent header (0x40b8a..
    // 0x40bc4) and queues the op. Request plumbing folds into the host;
    // the queued fetch is observed.
    let url = format!("{base_url}mobileapi/userinfo").replacen("http:", "https:", 1);
    user.request_url = url;
    user.player_info_requested = true;
}

// 0x40c58 — ___28-[UserInfo UpdatePlayerInfo]_block_invoke
#[doc(alias = "___28-[UserInfo UpdatePlayerInfo]_block_invoke")]
pub fn stub_0x40c58(user: &mut UserInfo, robux: i64, tix: i64, username: &str) {
    // IDA 0x40c58: the fetch block parses the info dictionary into the
    // balances and identity (JSON parsing folds into the inputs).
    user.robux = robux;
    user.tix = tix;
    user.username = username.to_owned();
}

// 0x41104 — ___copy_helper_block__6
#[doc(alias = "___copy_helper_block__6")]
pub fn stub_0x41104() {
    // IDA 0x41104: block copy helper retains the two captures (0x41114..
    // 0x41124, folds into `Clone`) — no-op.
}

// 0x41128 — ___destroy_helper_block__6
#[doc(alias = "___destroy_helper_block__6")]
pub fn stub_0x41128() {
    // IDA 0x41128: block destroy helper releases the captures (0x41132..
    // 0x4113e, folds into drop) — no-op.
}

// 0x41144 — +[UserInfo CurrentPlayer]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[UserInfo CurrentPlayer]")]
pub fn stub_0x41144() -> UserInfo {
    // IDA 0x41144: `CurrentPlayer` lazily allocs and inits the singleton
    // (0x41152..0x41186); singleton storage folds into the host.
    UserInfo::default()
}

// 0x4118c — -[UserInfo Robux]
// type: id __cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo Robux]")]
pub fn stub_0x4118c(user: &UserInfo) -> String {
    // IDA 0x4118c: `Robux` formats the balance (folds into
    // `convertToFriendlyString`).
    friendly_string(Some(user.robux))
}

// 0x411a0 — __Z23convertToFriendlyStringP8NSNumber
// type: _DWORD __fastcall(id)
#[doc(alias = "convertToFriendlyString(NSNumber *)")]
pub fn stub_0x411a0(value: Option<i64>) -> String {
    // IDA 0x411a0: `convertToFriendlyString` — see `friendly_string`.
    friendly_string(value)
}

// 0x41288 — -[UserInfo Tix]
// type: id __cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo Tix]")]
pub fn stub_0x41288(user: &UserInfo) -> String {
    // IDA 0x41288: `Tix` formats the balance (same shape as 0x4118c).
    friendly_string(Some(user.tix))
}

// 0x4129c — +[UserInfo clearAllRobloxCookie]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[UserInfo clearAllRobloxCookie]")]
pub fn stub_0x4129c(user: &mut UserInfo) {
    // IDA 0x4129c: `clearAllRobloxCookie` drains the shared cookie storage
    // (folds into the host); the clear is observed.
    user.cookies_cleared = true;
}

// 0x41580 — +[UserInfo printCookies]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[UserInfo printCookies]")]
pub fn stub_0x41580(cookies: &[String]) -> Vec<String> {
    // IDA 0x41580: `printCookies` logs each stored cookie; the walk folds
    // into an echo of the logged set.
    cookies.to_vec()
}

// 0x419c8 — +[UserInfo logout]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[UserInfo logout]")]
pub fn stub_0x419c8(current: &mut Option<UserInfo>) {
    // IDA 0x419c8: `logout` releases the current player (0x419ea) and
    // zeroes it (0x419f0).
    *current = None;
}

#[cfg(test)]
mod roblox_view_batch_tests {
    use super::*;

    #[test]
    fn view_lifecycle_latches() {
        let mut view = RobloxViewState::default();
        stub_0x38770(&mut view);
        assert!(view.restart_queued);
        let mut view = RobloxViewState::default();
        stub_0x38cd0(&mut view);
        assert!(view.workspace_bound);
        let mut view = RobloxViewState {
            rendering_suspended: true,
            ..RobloxViewState::default()
        };
        stub_0x39018(&mut view);
        assert!(!view.rendering_suspended);
        stub_0x39020();
        stub_0x39024();
    }

    #[test]
    fn create_view_and_log() {
        let view = stub_0x39674(800, 600, "a", "b", "c", true);
        assert_eq!(view.width, 800);
        assert_eq!(view.height, 600);
        assert_eq!(view.params, ["a".to_owned(), "b".to_owned(), "c".to_owned()]);
        assert!(view.view_prepped);
        let bare = stub_0x39674(1, 1, "a", "b", "c", false);
        assert!(!bare.view_prepped);
        assert_eq!(stub_0x39920("/Applications/Test.app"), "/Applications/Test.app");
    }

    #[test]
    fn error_and_ptr_glue() {
        stub_0x39be0();
        stub_0x39bf8();
        stub_0x39c00();
        stub_0x39c08();
        stub_0x39c20();
        stub_0x39c28();
        stub_0x39c40();
        stub_0x39c48();
        stub_0x39c50();
        stub_0x39c68();
        stub_0x39d7c();
        stub_0x39e10();
        stub_0x39ea8();
        stub_0x39f4c();
        stub_0x3a030();
        stub_0x3a0d4();
        stub_0x3a1b8();
        stub_0x3a2ec();
    }
}

#[cfg(test)]
mod factory_batch_tests {
    use super::*;

    #[test]
    fn signal_and_singletons() {
        let mut sig = ViewSignal::default();
        assert_eq!(stub_0x3a390(&mut sig), 1);
        assert_eq!(stub_0x3a390(&mut sig), 2);
        assert_eq!(stub_0x3a408(), 1);
        assert_eq!(stub_0x3ae20(), "RunService");
        assert_eq!(stub_0x3add8(), "RunService");
        assert_eq!(stub_0x3af08(), 1);
    }

    #[test]
    fn create_and_own() {
        assert_eq!(stub_0x3a798(), CreatedInstance { class: "Camera", live: true });
        assert_eq!(stub_0x3b674(), CreatedInstance { class: "ControllerService", live: true });
        let mut slot = OwnerSlot::default();
        stub_0x3a930(&mut slot);
        assert!(slot.has_owner);
        assert_eq!(stub_0x3b518(true), Some(1));
        assert_eq!(stub_0x3b518(false), None);
    }

    #[test]
    fn creator_maps() {
        let mut map = BTreeMap::new();
        assert!(stub_0x3ad20(&mut map, 7, 70));
        assert!(!stub_0x3ad20(&mut map, 7, 71));
        assert_eq!(map[&7], 70);
        assert_eq!(stub_0x3acc8(&mut map, 7), 70);
        assert_eq!(stub_0x3acc8(&mut map, 9), 0);
        stub_0x3aa30(&mut map);
        assert!(map.is_empty());
        map.insert(1, 10);
        stub_0x3aa90(&mut map);
        assert!(map.is_empty());
        stub_0x3aaa0();
        assert_eq!(stub_0x3aa18(), 0);
        assert_eq!(stub_0x3b130(), 0);
        assert_eq!(stub_0x3b148(), 0);
    }

    #[test]
    fn glue_noops() {
        stub_0x3a790();
        stub_0x3aa10();
        stub_0x3afe0();
        stub_0x3b008();
        stub_0x3b108();
        stub_0x3b110();
        stub_0x3b268();
        stub_0x3b26c();
    }
}

#[cfg(test)]
mod void_signal_batch_tests {
    use super::*;

    #[test]
    fn controller_names_and_index() {
        assert_eq!(stub_0x3b7e0(), "ControllerService");
        assert_eq!(stub_0x3b828(), "ControllerService");
        assert_eq!(stub_0x3b910(), 1);
        assert_eq!(stub_0x3af08(), 1);
    }

    #[test]
    fn void_signal_flow() {
        let mut sig = ViewSignal::default();
        stub_0x3be00(&mut sig);
        assert_eq!(sig.slots, 1);
        stub_0x3cf28(&mut sig);
        assert_eq!(sig.fired, 1);
        stub_0x3cf18(&mut sig);
        assert_eq!(sig.fired, 2);
        stub_0x3cf20(&mut sig);
        assert_eq!(sig.fired, 3);
        stub_0x3cf40(&mut sig);
        assert_eq!(sig.slots, 0);
        stub_0x3cf40(&mut sig);
        assert_eq!(sig.slots, 0);
        assert_eq!(stub_0x3c920(), 1);
        assert_eq!(stub_0x3d030(), 1);
    }

    #[test]
    fn null_name_and_delete() {
        assert!(stub_0x3bb58(None));
        assert!(!stub_0x3bb58(Some("ControllerService")));
        stub_0x3bcb8(0);
    }

    #[test]
    #[should_panic(expected = "c->strong == 0")]
    fn delete_asserts_live() {
        stub_0x3bcb8(3);
    }

    #[test]
    fn glue_noops_and_nulls() {
        stub_0x3b724();
        stub_0x3b9e8();
        stub_0x3ba10();
        stub_0x3bb10();
        stub_0x3bb18();
        stub_0x3bbf8();
        stub_0x3cdb8();
        stub_0x3ce64();
        stub_0x3d0e4();
        stub_0x3d190();
        stub_0x3d038();
        assert_eq!(stub_0x3bb38(), 0);
        assert_eq!(stub_0x3bb50(), 0);
    }
}

#[cfg(test)]
mod step_count_batch_tests {
    use super::*;

    #[test]
    fn counts_and_owners() {
        assert_eq!(stub_0x3d240(), SlotCounts { strong: 0, weak: 0 });
        let mut slot = OwnerSlot::default();
        stub_0x3dc60(&mut slot);
        stub_0x3de48(&mut slot);
        stub_0x3e0b0(&mut slot);
        assert!(slot.has_owner);
        assert_eq!(stub_0x3dc58(), 0);
        assert_eq!(stub_0x3de40(), 0);
        assert_eq!(stub_0x3de44(), 0);
        assert_eq!(stub_0x3e028(), 0);
        assert_eq!(stub_0x3e02c(), 0);
    }

    #[test]
    fn sequence_advances() {
        let mut seq = TaskSequence::default();
        assert_eq!(stub_0x3ebb0(&mut seq), 1);
        assert_eq!(stub_0x3ebb4(&mut seq), 2);
        assert_eq!(seq.steps, 2);
        assert_eq!(stub_0x3e1e8(), 20);
    }

    #[test]
    fn log_slot_dies() {
        let mut slot = LogManagerSlot { live: true };
        stub_0x3ec30(&mut slot);
        assert!(!slot.live);
    }

    #[test]
    fn glue_noops() {
        stub_0x3dd34();
        stub_0x3de28();
        stub_0x3de2c();
        stub_0x3de30();
        stub_0x3df1c();
        stub_0x3e010();
        stub_0x3e014();
        stub_0x3e018();
        stub_0x3e190();
        stub_0x3eccc();
        stub_0x3ecd0();
    }
}

#[cfg(test)]
mod render_job_batch_tests {
    use super::*;
    use crate::generated_110::FunctorOp;

    #[test]
    fn window_events() {
        stub_0x3ecd4();
        assert_eq!(stub_0x3ecd8(), 1);
        let mut log = Vec::new();
        assert_eq!(stub_0x3ecdc(&mut log), 1);
        assert_eq!(log, vec!["Request to close OGRE render window received".to_owned()]);
        stub_0x3ecec();
    }

    #[test]
    fn render_job_lifecycle() {
        let mut job = stub_0x3ecf0();
        assert!(job.live);
        assert!(!job.awake && !job.prepared && !job.performed);
        stub_0x3fb9c(&mut job);
        assert!(job.awake);
        stub_0x3faac(&mut job);
        assert!(job.prepared);
        stub_0x3fac4(&mut job, false);
        assert!(!job.performed);
        stub_0x3fac4(&mut job, true);
        assert!(job.performed);
        assert_eq!(stub_0x3f094(&mut job), 1);
        assert_eq!(stub_0x3f094(&mut job), 2);
        stub_0x3ee80(&mut job);
        assert!(!job.live);
        let mut job = stub_0x3ecf0();
        stub_0x3ef40(&mut job);
        assert!(!job.live);
        let mut job = stub_0x3ecf0();
        stub_0x3f904(&mut job);
        assert!(!job.live);
        let mut job = stub_0x3ecf0();
        stub_0x3f9c8(&mut job);
        assert!(!job.live);
    }

    #[test]
    fn sleep_error_and_concurrency() {
        assert_eq!(stub_0x3f008(false, 0.016), f64::INFINITY);
        assert_eq!(stub_0x3f008(true, 0.016), 0.016);
        let mut job = stub_0x3ecf0();
        stub_0x3f058(&mut job, false, 0.5);
        assert_eq!(job.last_error, 0.0);
        stub_0x3f058(&mut job, true, 0.5);
        assert_eq!(job.last_error, 0.5);
        assert_eq!(stub_0x3f090(), 1);
    }

    #[test]
    fn metrics() {
        let job = RenderJobState {
            metrics: vec![("Render FPS".to_owned(), 60.0)],
            graphics_mode: "OpenGL".to_owned(),
            ..RenderJobState::default()
        };
        assert_eq!(stub_0x3f598(&job, "Render FPS"), 60.0);
        assert_eq!(stub_0x3f598(&job, "Nope"), 0.0);
        assert_eq!(stub_0x3fa94(&job, "Render FPS"), 60.0);
        assert_eq!(stub_0x3f700(&job, "Graphics Mode"), "OpenGL");
        assert_eq!(stub_0x3f700(&job, "Nope"), "");
        assert_eq!(stub_0x3faa4(&job, "Graphics Mode"), "OpenGL");
    }

    #[test]
    fn render_functor_glue() {
        assert_eq!(stub_0x40160(FunctorOp::GetType), RENDER_PERFORM_BIND_TYPEINFO);
        assert_eq!(stub_0x401f0(FunctorOp::Destroy), IMETRIC_BIND_TYPEINFO);
        let mut cb = RenderCallback::default();
        stub_0x401dc(&mut cb);
        stub_0x40270(&mut cb);
        stub_0x4027c(&mut cb);
        assert_eq!(cb.calls, 3);
    }
}

#[cfg(test)]
mod update_user_batch_tests {
    use super::*;
    use crate::generated_110::FunctorOp;

    #[test]
    fn update_job_lifecycle() {
        let mut job = stub_0x403f0();
        assert!(job.live);
        assert_eq!(stub_0x40650(false, 0.1), f64::INFINITY);
        assert_eq!(stub_0x40650(true, 0.1), 0.1);
        stub_0x40680(&mut job, false, 0.5);
        assert_eq!(job.last_error, 0.0);
        stub_0x40680(&mut job, true, 0.5);
        assert_eq!(job.last_error, 0.5);
        assert_eq!(stub_0x406a8(), 1.0);
        assert_eq!(stub_0x406b4(&mut job), 1);
        assert_eq!(job.steps, 1);
        stub_0x404f0(&mut job);
        assert!(!job.live);
        let mut job = stub_0x403f0();
        stub_0x4059c(&mut job);
        assert!(!job.live);
        assert_eq!(stub_0x402a8(FunctorOp::GetType), RENDER_JOB_BIND_TYPEINFO);
        let mut cb = RenderCallback::default();
        stub_0x40308(&mut cb);
        assert_eq!(cb.calls, 1);
    }

    #[test]
    fn user_login_flow() {
        let mut user = stub_0x40984();
        assert!(!stub_0x40ab4(&user));
        user.username = "builder".to_owned();
        stub_0x409b0(&mut user, true);
        assert!(stub_0x40ab4(&user));
        assert_eq!(user.last_user, Some("builder".to_owned()));
        stub_0x409b0(&mut user, false);
        assert!(!stub_0x40ab4(&user));
        assert_eq!(user.username, "");
        assert_eq!(user.last_user, None);
    }

    #[test]
    fn player_info_and_balances() {
        let mut user = stub_0x40984();
        stub_0x40ac4(&mut user, "http://www.roblox.com/");
        assert!(user.player_info_requested);
        assert_eq!(user.request_url, "https://www.roblox.com/mobileapi/userinfo");
        stub_0x40c58(&mut user, 1500, 2200, "builder");
        assert_eq!(user.robux, 1500);
        assert_eq!(user.tix, 2200);
        assert_eq!(stub_0x4118c(&user), "1,500");
        assert_eq!(stub_0x41288(&user), "2,200");
        assert_eq!(stub_0x411a0(None), "unknown");
        assert_eq!(stub_0x411a0(Some(999)), "999");
        assert_eq!(stub_0x411a0(Some(1_500_000)), "1 mil");
        let current = stub_0x41144();
        assert_eq!(current, UserInfo::default());
    }

    #[test]
    fn cookies_and_logout() {
        let mut user = stub_0x40984();
        stub_0x4129c(&mut user);
        assert!(user.cookies_cleared);
        assert_eq!(
            stub_0x41580(&["a=1".to_owned()]),
            vec!["a=1".to_owned()]
        );
        let mut current = Some(user);
        stub_0x419c8(&mut current);
        assert_eq!(current, None);
        stub_0x41104();
        stub_0x41128();
    }
}
