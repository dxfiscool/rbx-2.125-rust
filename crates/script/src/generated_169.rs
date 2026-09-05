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
pub fn stub_0x3d240() -> ! {
    todo!("0x3d240 rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0>::counts::counts(void)")
}

// 0x3dc58 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_deleter(std::type_info const&)")]
pub fn stub_0x3dc58() -> ! {
    todo!("0x3dc58 boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_deleter(std::type_info const&)")
}

// 0x3dc60 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(rbx_core::SharedPtr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const")]
pub fn stub_0x3dc60() -> ! {
    todo!("0x3dc60 void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(rbx_core::SharedPtr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const")
}

// 0x3dd34 — __ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
pub fn stub_0x3dd34() -> ! {
    todo!("0x3dd34 boost::detail::shared_count::shared_count<RobloxView::RenderJob>(RobloxView::RenderJob *)")
}

// 0x3de28 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
pub fn stub_0x3de28() -> ! {
    todo!("0x3de28 boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")
}

// 0x3de2c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p() [0x3de2c]")]
pub fn stub_0x3de2c() -> ! {
    todo!("0x3de2c boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")
}

// 0x3de30 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::dispose(void)")]
pub fn stub_0x3de30() -> ! {
    todo!("0x3de30 boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::dispose(void)")
}

// 0x3de40 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_deleter(std::type_info const&)")]
pub fn stub_0x3de40() -> ! {
    todo!("0x3de40 boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_deleter(std::type_info const&)")
}

// 0x3de44 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_untyped_deleter(void)")]
pub fn stub_0x3de44() -> ! {
    todo!("0x3de44 boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_untyped_deleter(void)")
}

// 0x3de48 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(rbx_core::SharedPtr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const")]
pub fn stub_0x3de48() -> ! {
    todo!("0x3de48 void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(rbx_core::SharedPtr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const")
}

// 0x3df1c — __ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
pub fn stub_0x3df1c() -> ! {
    todo!("0x3df1c boost::detail::shared_count::shared_count<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")
}

// 0x3e010 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()")]
pub fn stub_0x3e010() -> ! {
    todo!("0x3e010 boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()")
}

// 0x3e014 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p() [0x3e014]")]
pub fn stub_0x3e014() -> ! {
    todo!("0x3e014 boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()")
}

// 0x3e018 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::dispose(void)")]
pub fn stub_0x3e018() -> ! {
    todo!("0x3e018 boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::dispose(void)")
}

// 0x3e028 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_deleter(std::type_info const&)")]
pub fn stub_0x3e028() -> ! {
    todo!("0x3e028 boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_deleter(std::type_info const&)")
}

// 0x3e02c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_untyped_deleter(void)")]
pub fn stub_0x3e02c() -> ! {
    todo!("0x3e02c boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_untyped_deleter(void)")
}

// 0x3e0b0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI19CRenderSettingsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<CRenderSettingsItem,CRenderSettingsItem>(rbx_core::SharedPtr<CRenderSettingsItem> const*,CRenderSettingsItem *)const")]
pub fn stub_0x3e0b0() -> ! {
    todo!("0x3e0b0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<CRenderSettingsItem,CRenderSettingsItem>(rbx_core::SharedPtr<CRenderSettingsItem> const*,CRenderSettingsItem *)const")
}

// 0x3e190 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3e190() -> ! {
    todo!("0x3e190 boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3e1e8 — __ZN5boost14singleton_poolIN3RBX16OnDemandInstanceELj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x3e1e8() -> ! {
    todo!("0x3e1e8 boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x3ebb0 — __ZN3RBX5Tasks8Sequence9onPreStepEPNS_13TaskScheduler3JobE
// type: int __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::Sequence::onPreStep(RBX::TaskScheduler::Job *)")]
pub fn stub_0x3ebb0() -> ! {
    todo!("0x3ebb0 RBX::Tasks::Sequence::onPreStep(RBX::TaskScheduler::Job *)")
}

// 0x3ebb4 — __ZN3RBX5Tasks17ExclusiveSequence10onPostStepEPNS_13TaskScheduler3JobE
// type: int __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::ExclusiveSequence::onPostStep(RBX::TaskScheduler::Job *)")]
pub fn stub_0x3ebb4() -> ! {
    todo!("0x3ebb4 RBX::Tasks::ExclusiveSequence::onPostStep(RBX::TaskScheduler::Job *)")
}

// 0x3ec30 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED1Ev
#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")]
pub fn stub_0x3ec30() -> ! {
    todo!("0x3ec30 boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")
}

// 0x3ec34 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED2Ev
#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr() [0x3ec34]")]
pub fn stub_0x3ec34() -> ! {
    todo!("0x3ec34 boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")
}

// 0x3eccc — __ZN17QuitEventListenerD0Ev
// type: void __fastcall(QuitEventListener *__hidden this)
#[doc(alias = "QuitEventListener::~QuitEventListener() [0x3eccc]")]
pub fn stub_0x3eccc() -> ! {
    todo!("0x3eccc QuitEventListener::~QuitEventListener()")
}

// 0x3ecd0 — __ZN4Ogre19WindowEventListener11windowMovedEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)")]
pub fn stub_0x3ecd0() -> ! {
    todo!("0x3ecd0 Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)")
}

// 0x3ecd4 — __ZN4Ogre19WindowEventListener13windowResizedEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)")]
pub fn stub_0x3ecd4() -> ! {
    todo!("0x3ecd4 Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)")
}

// 0x3ecd8 — __ZN4Ogre19WindowEventListener13windowClosingEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)")]
pub fn stub_0x3ecd8() -> ! {
    todo!("0x3ecd8 Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)")
}

// 0x3ecdc — __ZN17QuitEventListener12windowClosedEPN4Ogre12RenderWindowE
// type: _DWORD __fastcall(QuitEventListener *__hidden this, RenderWindow *)
#[doc(alias = "QuitEventListener::windowClosed(Ogre::RenderWindow *)")]
pub fn stub_0x3ecdc() -> ! {
    todo!("0x3ecdc QuitEventListener::windowClosed(Ogre::RenderWindow *)")
}

// 0x3ecec — __ZN4Ogre19WindowEventListener17windowFocusChangeEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)")]
pub fn stub_0x3ecec() -> ! {
    todo!("0x3ecec Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)")
}

// 0x3ecf0 — __ZN10RobloxView9RenderJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerEN5boost10shared_ptrINS1_9DataModelEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
#[doc(alias = "RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)")]
pub fn stub_0x3ecf0() -> ! {
    todo!("0x3ecf0 RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)")
}

// 0x3ee80 — __ZN10RobloxView9RenderJobD1Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "RobloxView::RenderJob::~RenderJob()")]
pub fn stub_0x3ee80() -> ! {
    todo!("0x3ee80 RobloxView::RenderJob::~RenderJob()")
}

// 0x3ef40 — __ZN10RobloxView9RenderJobD0Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "RobloxView::RenderJob::~RenderJob() [0x3ef40]")]
pub fn stub_0x3ef40() -> ! {
    todo!("0x3ef40 RobloxView::RenderJob::~RenderJob()")
}

// 0x3f008 — __ZN10RobloxView9RenderJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x3f008() -> ! {
    todo!("0x3f008 RobloxView::RenderJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x3f058 — __ZN10RobloxView9RenderJob5errorERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x3f058() -> ! {
    todo!("0x3f058 RobloxView::RenderJob::error(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x3f090 — __ZNK3RBX13TaskScheduler3Job26getDesiredConcurrencyCountEv
// type: int __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::getDesiredConcurrencyCount(void)const")]
pub fn stub_0x3f090() -> ! {
    todo!("0x3f090 RBX::TaskScheduler::Job::getDesiredConcurrencyCount(void)const")
}

// 0x3f094 — __ZN10RobloxView9RenderJob16stepDataModelJobERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x3f094() -> ! {
    todo!("0x3f094 RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x3f598 — __ZNK10RobloxView9RenderJob14getMetricValueERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "RobloxView::RenderJob::getMetricValue(std::string const&)const")]
pub fn stub_0x3f598() -> ! {
    todo!("0x3f598 RobloxView::RenderJob::getMetricValue(std::string const&)const")
}

// 0x3f700 — __ZNK10RobloxView9RenderJob9getMetricERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "RobloxView::RenderJob::getMetric(std::string const&)const")]
pub fn stub_0x3f700() -> ! {
    todo!("0x3f700 RobloxView::RenderJob::getMetric(std::string const&)const")
}

// 0x3f904 — __ZThn480_N10RobloxView9RenderJobD1Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::~RenderJob()")]
pub fn stub_0x3f904() -> ! {
    todo!("0x3f904 non-virtual thunk toRobloxView::RenderJob::~RenderJob()")
}

// 0x3f9c8 — __ZThn480_N10RobloxView9RenderJobD0Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::~RenderJob() [0x3f9c8]")]
pub fn stub_0x3f9c8() -> ! {
    todo!("0x3f9c8 non-virtual thunk toRobloxView::RenderJob::~RenderJob()")
}

// 0x3fa94 — __ZThn480_NK10RobloxView9RenderJob9getMetricERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::getMetric(std::string const&)const")]
pub fn stub_0x3fa94() -> ! {
    todo!("0x3fa94 non-virtual thunk toRobloxView::RenderJob::getMetric(std::string const&)const")
}

// 0x3faa4 — __ZThn480_NK10RobloxView9RenderJob14getMetricValueERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::getMetricValue(std::string const&)const")]
pub fn stub_0x3faa4() -> ! {
    todo!("0x3faa4 non-virtual thunk toRobloxView::RenderJob::getMetricValue(std::string const&)const")
}

// 0x3faac — __ZN10RobloxView9RenderJob21scheduleRenderPrepareEPS0_PN3RBX8ViewBaseE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, RenderJob *, ViewBase *)
#[doc(alias = "RobloxView::RenderJob::scheduleRenderPrepare(RobloxView::RenderJob*,RBX::ViewBase *)")]
pub fn stub_0x3faac() -> ! {
    todo!("0x3faac RobloxView::RenderJob::scheduleRenderPrepare(RobloxView::RenderJob*,RBX::ViewBase *)")
}

// 0x3fac4 — __ZN10RobloxView9RenderJob21scheduleRenderPerformEPS0_PN3RBX8ViewBaseEd
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, RobloxView::RenderJob *, RBX::ViewBase *, double)
#[doc(alias = "RobloxView::RenderJob::scheduleRenderPerform(RobloxView::RenderJob*,RBX::ViewBase *,double)")]
pub fn stub_0x3fac4() -> ! {
    todo!("0x3fac4 RobloxView::RenderJob::scheduleRenderPerform(RobloxView::RenderJob*,RBX::ViewBase *,double)")
}

// 0x3fb9c — __ZN10RobloxView9RenderJob4wakeEv
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "RobloxView::RenderJob::wake(void)")]
pub fn stub_0x3fb9c() -> ! {
    todo!("0x3fb9c RobloxView::RenderJob::wake(void)")
}

// 0x40160 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x40160() -> ! {
    todo!("0x40160 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x401dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x401dc() -> ! {
    todo!("0x401dc boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x401f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x401f0() -> ! {
    todo!("0x401f0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x40270 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x40270() -> ! {
    todo!("0x40270 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x4027c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double> &,boost::_bi::list0 &,int)")]
pub fn stub_0x4027c() -> ! {
    todo!("0x4027c void boost::_bi::list3<boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double> &,boost::_bi::list0 &,int)")
}

// 0x402a8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x402a8() -> ! {
    todo!("0x402a8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x40308 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x40308() -> ! {
    todo!("0x40308 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x40318 — __ZN5boost8weak_ptrIN3RBX9DataModelEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::Weak<RBX::DataModel>::weak_ptr<RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)")]
pub fn stub_0x40318() -> ! {
    todo!("0x40318 rbx_core::Weak<RBX::DataModel>::weak_ptr<RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)")
}

// 0x403f0 — __ZN10RobloxView13ViewUpdateJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerE
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this, RBX::ViewBase *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RobloxView::ViewUpdateJob::ViewUpdateJob(RBX::ViewBase *,RBX::FunctionMarshaller *)")]
pub fn stub_0x403f0() -> ! {
    todo!("0x403f0 RobloxView::ViewUpdateJob::ViewUpdateJob(RBX::ViewBase *,RBX::FunctionMarshaller *)")
}

// 0x404f0 — __ZN10RobloxView13ViewUpdateJobD1Ev
// type: void __fastcall(RobloxView::ViewUpdateJob *__hidden this)
#[doc(alias = "RobloxView::ViewUpdateJob::~ViewUpdateJob()")]
pub fn stub_0x404f0() -> ! {
    todo!("0x404f0 RobloxView::ViewUpdateJob::~ViewUpdateJob()")
}

// 0x4059c — __ZN10RobloxView13ViewUpdateJobD0Ev
// type: void __fastcall(RobloxView::ViewUpdateJob *__hidden this)
#[doc(alias = "RobloxView::ViewUpdateJob::~ViewUpdateJob() [0x4059c]")]
pub fn stub_0x4059c() -> ! {
    todo!("0x4059c RobloxView::ViewUpdateJob::~ViewUpdateJob()")
}

// 0x40650 — __ZN10RobloxView13ViewUpdateJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::ViewUpdateJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x40650() -> ! {
    todo!("0x40650 RobloxView::ViewUpdateJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x40680 — __ZN10RobloxView13ViewUpdateJob5errorERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::ViewUpdateJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x40680() -> ! {
    todo!("0x40680 RobloxView::ViewUpdateJob::error(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x406a8 — __ZN10RobloxView13ViewUpdateJob17getPriorityFactorEv
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this)
#[doc(alias = "RobloxView::ViewUpdateJob::getPriorityFactor(void)")]
pub fn stub_0x406a8() -> ! {
    todo!("0x406a8 RobloxView::ViewUpdateJob::getPriorityFactor(void)")
}

// 0x406b4 — __ZN10RobloxView13ViewUpdateJob4stepERKN3RBX13TaskScheduler3Job5StatsE
#[doc(alias = "RobloxView::ViewUpdateJob::step(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x406b4() -> ! {
    todo!("0x406b4 RobloxView::ViewUpdateJob::step(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x4070c — __GLOBAL__I_a_10
#[doc(alias = "global constructor keyed to_a_10")]
pub fn stub_0x4070c() -> ! {
    todo!("0x4070c global constructor keyed to_a_10")
}

// 0x40984 — -[UserInfo init]
// type: UserInfo *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo init]")]
pub fn stub_0x40984() -> ! {
    todo!("0x40984 -[UserInfo init]")
}

// 0x409b0 — -[UserInfo setUserLoggedIn:]
// type: void __cdecl(UserInfo *self, SEL, char)
#[doc(alias = "-[UserInfo setUserLoggedIn:]")]
pub fn stub_0x409b0() -> ! {
    todo!("0x409b0 -[UserInfo setUserLoggedIn:]")
}

// 0x40ab4 — -[UserInfo userLoggedIn]
// type: char __cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo userLoggedIn]")]
pub fn stub_0x40ab4() -> ! {
    todo!("0x40ab4 -[UserInfo userLoggedIn]")
}

// 0x40ac4 — -[UserInfo UpdatePlayerInfo]
// type: void __cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo UpdatePlayerInfo]")]
pub fn stub_0x40ac4() -> ! {
    todo!("0x40ac4 -[UserInfo UpdatePlayerInfo]")
}

// 0x40c58 — ___28-[UserInfo UpdatePlayerInfo]_block_invoke
#[doc(alias = "___28-[UserInfo UpdatePlayerInfo]_block_invoke")]
pub fn stub_0x40c58() -> ! {
    todo!("0x40c58 ___28-[UserInfo UpdatePlayerInfo]_block_invoke")
}

// 0x41104 — ___copy_helper_block__6
#[doc(alias = "___copy_helper_block__6")]
pub fn stub_0x41104() -> ! {
    todo!("0x41104 ___copy_helper_block__6")
}

// 0x41128 — ___destroy_helper_block__6
#[doc(alias = "___destroy_helper_block__6")]
pub fn stub_0x41128() -> ! {
    todo!("0x41128 ___destroy_helper_block__6")
}

// 0x41144 — +[UserInfo CurrentPlayer]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[UserInfo CurrentPlayer]")]
pub fn stub_0x41144() -> ! {
    todo!("0x41144 +[UserInfo CurrentPlayer]")
}

// 0x4118c — -[UserInfo Robux]
// type: id __cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo Robux]")]
pub fn stub_0x4118c() -> ! {
    todo!("0x4118c -[UserInfo Robux]")
}

// 0x411a0 — __Z23convertToFriendlyStringP8NSNumber
// type: _DWORD __fastcall(id)
#[doc(alias = "convertToFriendlyString(NSNumber *)")]
pub fn stub_0x411a0() -> ! {
    todo!("0x411a0 convertToFriendlyString(NSNumber *)")
}

// 0x41288 — -[UserInfo Tix]
// type: id __cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo Tix]")]
pub fn stub_0x41288() -> ! {
    todo!("0x41288 -[UserInfo Tix]")
}

// 0x4129c — +[UserInfo clearAllRobloxCookie]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[UserInfo clearAllRobloxCookie]")]
pub fn stub_0x4129c() -> ! {
    todo!("0x4129c +[UserInfo clearAllRobloxCookie]")
}

// 0x41580 — +[UserInfo printCookies]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[UserInfo printCookies]")]
pub fn stub_0x41580() -> ! {
    todo!("0x41580 +[UserInfo printCookies]")
}

// 0x419c8 — +[UserInfo logout]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[UserInfo logout]")]
pub fn stub_0x419c8() -> ! {
    todo!("0x419c8 +[UserInfo logout]")
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
