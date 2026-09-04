//! RobloxView — iOS main view, mirrors Client/iOS/RobloxView.* + SharedCode/RobloxView.cpp
//! Batch 1: teleport/control-view leaves (IDA 0x25440..0x2c224) + RobloxView core
//! methods (IDA 0x37068..0x39674). `SharedPtr` = `rbx_core::SharedPtr` (`Arc`),
//! never `boost::shared_ptr`; `boost::bind`/`function` become closures/`Box<dyn Fn>`.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

pub use rbx_core::SharedPtr;

/// ObjC `id` (nullable object pointer); `None`/`NIL_ID` is `nil` (no host runtime here).
pub type ObjCId = usize;
pub const NIL_ID: ObjCId = 0;

/// `CGRect` carried through `setFrame:`/`bounds` (`objc_msgSend_stret`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl ViewRect {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, w: 0.0, h: 0.0 };
    pub fn is_empty(self) -> bool {
        self.w <= 0.0 || self.h <= 0.0
    }
}

/// Opaque `RBX::Game` handle (`rbx_core::SharedPtr<RBX::Game>` erases to this id;
/// the live `RBX::Game` lives in `rbx_datamodel`, out of slice).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GameHandle {
    pub id: u32,
}

/// `+[MainViewController sharedInstance]` slice state used by the teleport and
/// control-view helpers: presence replaces the nil-check, `ogre_subviews` the
/// `-[UIView subviews]` enumeration.
#[derive(Debug, Default)]
pub struct MainViewState {
    pub present: bool,
    pub ogre_subviews: parking_lot::Mutex<Vec<ObjCId>>,
    pub(crate) roblox_view: parking_lot::Mutex<Option<SharedPtr<RobloxView>>>,
}

impl MainViewState {
    pub fn new(present: bool, ogre_subviews: Vec<ObjCId>) -> Self {
        Self {
            present,
            ogre_subviews: parking_lot::Mutex::new(ogre_subviews),
            roblox_view: parking_lot::Mutex::new(None),
        }
    }
}

/// Wraps an opaque game id into a `SharedPtr` (`boost::shared_ptr<RBX::Game>`
/// construction from a held pointer; the live game lives out of slice).
pub fn wrap_game(id: u32) -> SharedPtr<GameHandle> {
    SharedPtr::new(GameHandle { id })
}

/// `PlaceLauncher::rbxView` ivar plus the `RobloxMemoryManager` free-memory
/// checker state that `deleteRobloxView` tears down with it.
#[derive(Debug, Default)]
pub struct PlaceLauncherViewSlot {
    view: parking_lot::Mutex<Option<SharedPtr<RobloxView>>>,
    free_memory_checker_running: AtomicBool,
}

impl PlaceLauncherViewSlot {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_view(&self, view: SharedPtr<RobloxView>) {
        *self.view.lock() = Some(view);
    }
    pub fn has_view(&self) -> bool {
        self.view.lock().is_some()
    }
    pub fn set_free_memory_checker_running(&self, running: bool) {
        self.free_memory_checker_running.store(running, Ordering::SeqCst);
    }
    pub fn free_memory_checker_running(&self) -> bool {
        self.free_memory_checker_running.load(Ordering::SeqCst)
    }
}

/// Minimal `RobloxView` counterpart (`SharedCode/RobloxView.cpp`): job slots,
/// bound game/workspace ids, and counters for the scheduler/signal/UIKit steps
/// the leaves below perform (render-system internals are out of slice).
#[derive(Debug, Default)]
pub struct RobloxView {
    width: AtomicU32,
    height: AtomicU32,
    ctor_str0: parking_lot::Mutex<String>,
    ctor_str1: parking_lot::Mutex<String>,
    ctor_str2: parking_lot::Mutex<String>,
    game: parking_lot::Mutex<Option<SharedPtr<GameHandle>>>,
    has_datamodel: AtomicBool,
    view_base: parking_lot::Mutex<Option<ObjCId>>,
    datamodel: parking_lot::Mutex<Option<ObjCId>>,
    overlay_datamodel: parking_lot::Mutex<Option<ObjCId>>,
    legacy_locks: AtomicU32,
    render_job_active: AtomicBool,
    view_update_job_active: AtomicBool,
    rendering_suspended: AtomicBool,
    stop_event_set: AtomicBool,
    run_service_stopped: AtomicBool,
    scheduler_adds: AtomicU32,
    scheduler_removes: AtomicU32,
    removed_blocking_calls: AtomicU32,
    removed_coordinators: AtomicU32,
    added_coordinators: AtomicU32,
    processed_messages: AtomicU32,
    window_listeners: AtomicU32,
    place_id_connection: parking_lot::Mutex<Option<u64>>,
    next_connection_id: AtomicU32,
    connection_disconnects: AtomicU32,
    main_dispatches: AtomicU32,
    pending_restart: AtomicBool,
    pending_resume: AtomicBool,
    ogre_view: parking_lot::Mutex<Option<ObjCId>>,
    ogre_window: parking_lot::Mutex<Option<ObjCId>>,
    control_view_present: AtomicBool,
    control_subview_adds: AtomicU32,
    bound_subview: parking_lot::Mutex<Option<ObjCId>>,
    bound_game_id: parking_lot::Mutex<Option<u32>>,
    teleport_animations: AtomicU32,
    last_animation: parking_lot::Mutex<Option<TeleportAnimation>>,
}

impl RobloxView {
    pub fn dims(&self) -> (u32, u32) {
        (self.width.load(Ordering::SeqCst), self.height.load(Ordering::SeqCst))
    }
    pub fn game_id(&self) -> Option<u32> {
        self.game.lock().as_ref().map(|g| g.id)
    }
    pub fn has_datamodel(&self) -> bool {
        self.has_datamodel.load(Ordering::SeqCst)
    }
    pub fn rendering_suspended(&self) -> bool {
        self.rendering_suspended.load(Ordering::SeqCst)
    }
    pub fn render_job_active(&self) -> bool {
        self.render_job_active.load(Ordering::SeqCst)
    }
    pub fn view_update_job_active(&self) -> bool {
        self.view_update_job_active.load(Ordering::SeqCst)
    }
    pub fn stop_event_set(&self) -> bool {
        self.stop_event_set.load(Ordering::SeqCst)
    }
    pub fn scheduler_adds(&self) -> u32 {
        self.scheduler_adds.load(Ordering::SeqCst)
    }
    pub fn scheduler_removes(&self) -> u32 {
        self.scheduler_removes.load(Ordering::SeqCst)
    }
    pub fn removed_blocking_calls(&self) -> u32 {
        self.removed_blocking_calls.load(Ordering::SeqCst)
    }
    pub fn processed_messages(&self) -> u32 {
        self.processed_messages.load(Ordering::SeqCst)
    }
    pub fn main_dispatches(&self) -> u32 {
        self.main_dispatches.load(Ordering::SeqCst)
    }
    pub fn control_subview_adds(&self) -> u32 {
        self.control_subview_adds.load(Ordering::SeqCst)
    }
    pub fn teleport_animations(&self) -> u32 {
        self.teleport_animations.load(Ordering::SeqCst)
    }
    pub fn last_animation(&self) -> Option<TeleportAnimation> {
        *self.last_animation.lock()
    }
    pub fn place_id_connection(&self) -> Option<u64> {
        *self.place_id_connection.lock()
    }
    fn set_game(&self, game: SharedPtr<GameHandle>) {
        *self.game.lock() = Some(game);
    }
    fn reconnect_place_id(&self) {
        // `connection::disconnect` + `operator=` + weak `release` on the old
        // slot (IDA 0x37b3c..0x37c10): drop-then-replace with a fresh id.
        if self.place_id_connection.lock().take().is_some() {
            self.connection_disconnects.fetch_add(1, Ordering::SeqCst);
        }
        let id = self.next_connection_id.fetch_add(1, Ordering::SeqCst) as u64 + 1;
        *self.place_id_connection.lock() = Some(id);
    }
    fn stop_render_jobs(&self) {
        // `CEvent::Set`, `removeBlocking` on both jobs, `ProcessMessages`,
        // then both `shared_ptr::reset` (IDA 0x37068..0x37360).
        self.stop_event_set.store(true, Ordering::SeqCst);
        self.removed_blocking_calls.fetch_add(2, Ordering::SeqCst);
        self.scheduler_removes.fetch_add(2, Ordering::SeqCst);
        self.processed_messages.fetch_add(1, Ordering::SeqCst);
        self.render_job_active.store(false, Ordering::SeqCst);
        self.view_update_job_active.store(false, Ordering::SeqCst);
    }
    fn start_render_jobs(&self) {
        // `new ViewUpdateJob` + `new RenderJob`, both `shared_ptr`-held and
        // `TaskScheduler::add`ed (IDA 0x37378..0x375f0).
        self.view_update_job_active.store(true, Ordering::SeqCst);
        self.render_job_active.store(true, Ordering::SeqCst);
        self.scheduler_adds.fetch_add(2, Ordering::SeqCst);
    }
    pub(crate) fn note_game_bound(&self, subview: ObjCId, game_id: u32) {
        *self.bound_subview.lock() = Some(subview);
        *self.bound_game_id.lock() = Some(game_id);
    }
    pub(crate) fn note_teleport_animation(&self, animation: TeleportAnimation) {
        *self.last_animation.lock() = Some(animation);
        self.teleport_animations.fetch_add(1, Ordering::SeqCst);
    }
    pub(crate) fn set_ogre_view(&self, view: ObjCId) {
        *self.ogre_view.lock() = Some(view);
    }
    pub(crate) fn set_ogre_window(&self, window: ObjCId) {
        *self.ogre_window.lock() = Some(window);
    }
    pub(crate) fn set_control_view_present(&self, present: bool) {
        self.control_view_present.store(present, Ordering::SeqCst);
    }
    pub(crate) fn add_control_subview(&self) {
        self.control_subview_adds.fetch_add(1, Ordering::SeqCst);
    }
    pub(crate) fn note_main_dispatch(&self) {
        self.main_dispatches.fetch_add(1, Ordering::SeqCst);
    }
}

// 0x37068 — __ZN10RobloxView37requestStopRenderingForBackgroundModeEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
// IDA 0x37068
impl RobloxView {
    #[doc(alias = "RobloxView::requestStopRenderingForBackgroundMode(void)")]
    #[doc = "RobloxView::requestStopRenderingForBackgroundMode(void)"]
    pub fn request_stop_rendering_for_background_mode(&self) {
        // Stop event, both jobs `removeBlocking`ed/`remove`d, messages pumped,
        // both slots `reset` (IDA 0x37068..0x37360).
        self.stop_render_jobs();
        self.rendering_suspended.store(true, Ordering::SeqCst);
    }
}

// 0x37378 — __ZN10RobloxView22requestResumeRenderingEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
// IDA 0x37378
impl RobloxView {
    #[doc(alias = "RobloxView::requestResumeRendering(void)")]
    #[doc = "RobloxView::requestResumeRendering(void)"]
    pub fn request_resume_rendering(&self) {
        // Recreates both jobs from the bound `ViewBase`/game and re-`add`s them
        // (IDA 0x37378..0x375f0).
        self.start_render_jobs();
        self.rendering_suspended.store(false, Ordering::SeqCst);
    }
}

// 0x37628 — __ZN10RobloxViewC2EjjSsSsSs
// type: void
// IDA 0x37628
impl RobloxView {
    #[doc(alias = "RobloxView::RobloxView(unsigned int,unsigned int,std::string,std::string,std::string)")]
    #[doc = "RobloxView::RobloxView(unsigned int,unsigned int,std::string,std::string,std::string)"]
    pub fn new_with_strings(
        width: u32,
        height: u32,
        s0: &str,
        s1: &str,
        s2: &str,
    ) -> Self {
        // `ViewBase::CreateView`, `addWindowEventListener`, `GetWindow`, and a
        // first `ViewUpdateJob` (IDA 0x37628..0x37b30); GL/view creation itself
        // is out of slice.
        Self {
            width: AtomicU32::new(width),
            height: AtomicU32::new(height),
            ctor_str0: parking_lot::Mutex::new(s0.to_owned()),
            ctor_str1: parking_lot::Mutex::new(s1.to_owned()),
            ctor_str2: parking_lot::Mutex::new(s2.to_owned()),
            view_update_job_active: AtomicBool::new(true),
            window_listeners: AtomicU32::new(1),
            ..Self::default()
        }
    }
}

// 0x37b3c — __ZN10RobloxView16completeViewPrepEN5boost10shared_ptrIN3RBX4GameEEE
// type: void
// IDA 0x37b3c
impl RobloxView {
    #[doc(alias = "RobloxView::completeViewPrep(rbx_core::SharedPtr<RBX::Game>)")]
    #[doc = "RobloxView::completeViewPrep(rbx_core::SharedPtr<RBX::Game>)"]
    pub fn complete_view_prep(&self, game: SharedPtr<GameHandle>) {
        // `game_ = game`, then (re)connects `onPlaceIDChanged` to the
        // datamodel-descriptor signal (IDA 0x37b3c..0x38098).
        self.set_game(game);
        self.reconnect_place_id();
    }
}

// 0x380a0 — __ZN10RobloxView16onPlaceIDChangedEPKN3RBX10Reflection18PropertyDescriptorE
// type: _DWORD __fastcall(RobloxView *__hidden this, const PropertyDescriptor *)
// IDA 0x380a0
impl RobloxView {
    #[doc(alias = "RobloxView::onPlaceIDChanged(RBX::Reflection::PropertyDescriptor const*)")]
    #[doc = "RobloxView::onPlaceIDChanged(RBX::Reflection::PropertyDescriptor const*)"]
    pub fn on_place_id_changed(&self) {
        // Empty body (IDA 0x380a0: single `;`).
    }
}

// 0x380a4 — __ZN10RobloxView13bindWorkspaceEN5boost10shared_ptrIN3RBX8ViewBaseEEENS1_INS2_9DataModelEEENS1_INS2_16OverlayDataModelEEE
// type: void
// IDA 0x380a4
impl RobloxView {
    #[doc(alias = "RobloxView::bindWorkspace(rbx_core::SharedPtr<RBX::ViewBase>,rbx_core::SharedPtr<RBX::DataModel>,rbx_core::SharedPtr<RBX::OverlayDataModel>)")]
    #[doc = "RobloxView::bindWorkspace(rbx_core::SharedPtr<RBX::ViewBase>,rbx_core::SharedPtr<RBX::DataModel>,rbx_core::SharedPtr<RBX::OverlayDataModel>)"]
    pub fn bind_workspace(&self, view_base: ObjCId, datamodel: ObjCId, overlay: ObjCId) {
        // `DataModel::LegacyLock` over the datamodel, then stores all three
        // bindings (IDA 0x380a4..0x382a8).
        self.legacy_locks.fetch_add(1, Ordering::SeqCst);
        *self.view_base.lock() = Some(view_base);
        *self.datamodel.lock() = Some(datamodel);
        *self.overlay_datamodel.lock() = Some(overlay);
    }
}

// 0x382b0 — __ZN10RobloxView22defineConcurrencyRulesEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
// IDA 0x382b0
impl RobloxView {
    #[doc(alias = "RobloxView::defineConcurrencyRules(void)")]
    #[doc = "RobloxView::defineConcurrencyRules(void)"]
    pub fn define_concurrency_rules(&self) -> bool {
        // `ReleaseAssert(renderJob)` (`RobloxView.cpp:555`, IDA 0x382b0..0x38310):
        // false models the abort without taking it.
        if !self.render_job_active.load(Ordering::SeqCst) {
            return false;
        }
        // Fresh `ExclusiveSequence` coordinator `add`ed to the render, view-
        // update, and physics (`RunService::getPhysicsJob`) jobs
        // (IDA 0x38310..0x386c0).
        self.added_coordinators.fetch_add(3, Ordering::SeqCst);
        true // IDA 0x386c4
    }
}

// 0x386d0 — __ZN10RobloxView16restartDataModelEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
// IDA 0x386d0
impl RobloxView {
    #[doc(alias = "RobloxView::restartDataModel(void)")]
    #[doc = "RobloxView::restartDataModel(void)"]
    pub fn restart_data_model(&self) {
        // `dispatch_async(main, doRestartDataModel-block)` (IDA 0x386d0..0x38718).
        self.pending_restart.store(true, Ordering::SeqCst);
        self.note_main_dispatch();
    }
}

// 0x38720 — __ZN10RobloxView15newGameDidStartEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
// IDA 0x38720
impl RobloxView {
    #[doc(alias = "RobloxView::newGameDidStart(void)")]
    #[doc = "RobloxView::newGameDidStart(void)"]
    pub fn new_game_did_start(&self) {
        // `dispatch_async(main, newGameDidStart-block)` (IDA 0x38720..0x38768).
        self.pending_resume.store(true, Ordering::SeqCst);
        self.note_main_dispatch();
    }
}

// 0x38770 — ____ZN10RobloxView18doRestartDataModelEv_block_invoke
// type: void
// IDA 0x38770
impl RobloxView {
    #[doc(alias = "____ZN10RobloxView18doRestartDataModelEv_block_invoke")]
    #[doc = "RobloxView::doRestartDataModel block"]
    pub fn do_restart_data_model(&self) {
        // `RunService::stopTasks`, stop event, both jobs `removeBlocking`ed,
        // physics coordinator removed, messages pumped, both jobs `reset`
        // (IDA 0x38770..0x38cc0).
        self.run_service_stopped.store(true, Ordering::SeqCst);
        self.stop_render_jobs();
        self.removed_coordinators.fetch_add(1, Ordering::SeqCst);
        self.pending_restart.store(false, Ordering::SeqCst);
    }
}

// 0x38cd0 — __ZN10RobloxView17setupNewDataModelEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
// IDA 0x38cd0
impl RobloxView {
    #[doc(alias = "RobloxView::setupNewDataModel(void)")]
    #[doc = "RobloxView::setupNewDataModel(void)"]
    pub fn setup_new_data_model(&self, game_id: u32) {
        // `DataModel::createDataModel`, `Game::setDataModel`, `LegacyLock`,
        // then (re)connects `onPlaceIDChanged` (IDA 0x38cd0..0x39010).
        self.set_game(SharedPtr::new(GameHandle { id: game_id }));
        self.has_datamodel.store(true, Ordering::SeqCst);
        self.legacy_locks.fetch_add(1, Ordering::SeqCst);
        self.reconnect_place_id();
    }
}

// 0x39018 — ____ZN10RobloxView15newGameDidStartEv_block_invoke
// type: void
// IDA 0x39018
impl RobloxView {
    #[doc(alias = "____ZN10RobloxView15newGameDidStartEv_block_invoke")]
    #[doc = "RobloxView::newGameDidStart block"]
    pub fn new_game_did_start_block(&self) {
        // Single call: `requestResumeRendering` (IDA 0x39018..0x3901e).
        self.request_resume_rendering();
        self.pending_resume.store(false, Ordering::SeqCst);
    }
}

// 0x39020 — __ZN10RobloxViewD1Ev
// type: void __fastcall(RobloxView *__hidden this)
// IDA 0x39020
impl RobloxView {
    #[doc(alias = "RobloxView::~RobloxView()")]
    #[doc = "RobloxView::~RobloxView() deleting destructor"]
    pub fn delete_dtor(&self) {
        // D1 tail-calls D2 then `operator delete` (IDA 0x39020..0x39024);
        // `Arc` drop is the delete.
        self.complete_dtor();
    }
}

// 0x39024 — __ZN10RobloxViewD2Ev
// type: void __fastcall(RobloxView *__hidden this)
// IDA 0x39024
impl RobloxView {
    #[doc(alias = "RobloxView::~RobloxView()")]
    #[doc = "RobloxView::~RobloxView() complete destructor"]
    pub fn complete_dtor(&self) {
        // Run-service stop, physics coordinator removal, stop event, both jobs
        // `removeBlocking`ed + `reset`, messages pumped, datamodel torn down
        // under `LegacyLock`, place-id slot disconnected (IDA 0x39024..0x3966c).
        self.run_service_stopped.store(true, Ordering::SeqCst);
        self.removed_coordinators.fetch_add(1, Ordering::SeqCst);
        self.stop_render_jobs();
        self.legacy_locks.fetch_add(1, Ordering::SeqCst);
        self.has_datamodel.store(false, Ordering::SeqCst);
        if self.place_id_connection.lock().take().is_some() {
            self.connection_disconnects.fetch_add(1, Ordering::SeqCst);
        }
        *self.game.lock() = None;
    }
}

// 0x39674 — __ZN10RobloxView11create_viewEN5boost10shared_ptrIN3RBX4GameEEEjjSsSsSs
// type: void
// IDA 0x39674
#[doc(alias = "RobloxView::create_view(rbx_core::SharedPtr<RBX::Game>,unsigned int,unsigned int,std::string,std::string,std::string)")]
#[doc = "RobloxView::create_view(rbx_core::SharedPtr<RBX::Game>,unsigned int,unsigned int,std::string,std::string,std::string)"]
pub fn create_view(
    game_id: u32,
    width: u32,
    height: u32,
    s0: &str,
    s1: &str,
    s2: &str,
) -> SharedPtr<RobloxView> {
    // `operator new` + `RobloxView::RobloxView(w, h, s0, s1, s2)` held in a fresh
    // `shared_ptr` (IDA 0x39674..0x39910); the game binds during `completeViewPrep`.
    let view = SharedPtr::new(RobloxView::new_with_strings(width, height, s0, s1, s2));
    view.note_game_bound(NIL_ID, game_id);
    view
}

/// Minimal `RobloxView::RenderJob` counterpart (`DataModelJob` with job name
/// `"Render"`): the bound view/marshaller ids, the weak datamodel slot
/// (`+496`), the wake `CEvent` (`+508`), the enable flag (`+628`), and the
/// prepare/perform block counters (`+632`/`+158`).
#[derive(Debug, Default)]
pub struct RenderJob {
    view_base: parking_lot::Mutex<Option<ObjCId>>,
    marshaller: parking_lot::Mutex<Option<ObjCId>>,
    datamodel: parking_lot::Mutex<Option<u32>>,
    event_set: AtomicBool,
    enabled_628: AtomicBool,
    prepare_blocks_632: AtomicU32,
    perform_blocks_158: AtomicU32,
    steps: AtomicU32,
    submitted_performs: AtomicU32,
    scheduled_prepares: AtomicU32,
    scheduled_performs: AtomicU32,
    wakes: AtomicU32,
    reschedules: AtomicU32,
    nominal_fps: parking_lot::Mutex<f64>,
    video_mem_mb: parking_lot::Mutex<f64>,
    avg_steps_per_second: parking_lot::Mutex<f64>,
    avg_duty_cycle: parking_lot::Mutex<f64>,
    avg_step_time: parking_lot::Mutex<f64>,
    antialiasing_on: AtomicBool,
    destroyed: AtomicBool,
}

/// Minimal `RobloxView::ViewUpdateJob` counterpart: its `ViewBase` /
/// `FunctionMarshaller` bindings (the ctor itself lives out of slice).
#[derive(Debug, Default)]
pub struct ViewUpdateJob {
    pub view_base: Option<ObjCId>,
    pub marshaller: Option<ObjCId>,
    pub(crate) steps: AtomicU32,
    pub(crate) error_sample: parking_lot::Mutex<f64>,
    pub(crate) view_update_pending: AtomicBool,
    pub(crate) update_passes: AtomicU32,
    pub(crate) present_passes: AtomicU32,
    pub(crate) destroyed: AtomicBool,
}

/// `Render FPS` / `Render Duty` / `Render Job Time` / `Render Nominal FPS` /
/// `Video Memory MB` are computed locally; the breakdown names delegate to the
/// bound view's `IMetric` (out of slice); anything else reads `0.0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderMetric {
    Fps,
    Duty,
    JobTime,
    NominalFps,
    VideoMemoryMb,
    ViewDelegated,
    Unknown,
}

impl RenderMetric {
    pub fn classify(name: &str) -> Self {
        match name {
            "Render FPS" => Self::Fps,
            "Render Duty" => Self::Duty,
            "Render Job Time" => Self::JobTime,
            "Render Nominal FPS" => Self::NominalFps,
            "Video Memory MB" => Self::VideoMemoryMb,
            "Delta Between Renders" | "Ogre" | "Total Render" | "Present Time" | "GPU Delay" | "Render Prepare" => Self::ViewDelegated,
            _ => Self::Unknown,
        }
    }
}

// 0x3ecf0 — __ZN10RobloxView9RenderJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerEN5boost10shared_ptrINS1_9DataModelEEE
// type: void
// IDA 0x3ecf0
impl RenderJob {
    #[doc(alias = "RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)")]
    #[doc = "RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)"]
    pub fn new(view_base: ObjCId, marshaller: ObjCId, datamodel: Option<u32>) -> Self {
        // `DataModelJob("Render", category 2, ...)` base, vtables, weak
        // datamodel at `+496`, `viewBase` at `+504`, fresh `CEvent` at `+508`,
        // `+628 = 1`, `+632 = 0` (IDA 0x3ecf0..0x3ee70).
        Self {
            view_base: parking_lot::Mutex::new(Some(view_base)),
            marshaller: parking_lot::Mutex::new(Some(marshaller)),
            datamodel: parking_lot::Mutex::new(datamodel),
            enabled_628: AtomicBool::new(true),
            nominal_fps: parking_lot::Mutex::new(60.0),
            ..Self::default()
        }
    }
    pub fn steps(&self) -> u32 {
        self.steps.load(Ordering::SeqCst)
    }
    pub fn is_destroyed(&self) -> bool {
        self.destroyed.load(Ordering::SeqCst)
    }
}

// 0x3ee80 — __ZN10RobloxView9RenderJobD1Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
// IDA 0x3ee80
impl RenderJob {
    #[doc(alias = "RobloxView::RenderJob::~RenderJob()")]
    #[doc = "RobloxView::RenderJob::~RenderJob() complete destructor"]
    pub fn destroy_d1(&self) {
        // Restores vtables, `~CEvent` at `+508`, `weak_release` of the `+496`
        // weak slot, then `Job::~Job` (IDA 0x3ee80..0x3ef30); `Arc` drop is D0.
        self.event_set.store(false, Ordering::SeqCst);
        *self.datamodel.lock() = None;
        self.destroyed.store(true, Ordering::SeqCst);
    }
}

// 0x3ef40 — __ZN10RobloxView9RenderJobD0Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
// IDA 0x3ef40
impl RenderJob {
    #[doc(alias = "RobloxView::RenderJob::~RenderJob()")]
    #[doc = "RobloxView::RenderJob::~RenderJob() deleting destructor"]
    pub fn delete_d0(&self) {
        // D0 runs D1 then `operator delete` (IDA 0x3ef40..0x3ef60);
        // `Arc` drop is the delete.
        self.destroy_d1();
    }
}

// 0x3f008 — __ZN10RobloxView9RenderJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
// IDA 0x3f008
impl RenderJob {
    #[doc(alias = "RobloxView::RenderJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
    #[doc = "RobloxView::RenderJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)"]
    pub fn sleep_time_secs(&self, stats_throttled: bool) -> f64 {
        // Throttled (`stats+0x274 != 0`): `computeStandardSleepTime(stats, 60.0)`
        // (IDA 0x3f008..0x3f034); the averaging core is out of slice, the
        // nominal 60fps quantum is exact.
        // Unthrottled: stores `+Inf` (`0x7FEFFFFFFFFFFFFF`, IDA 0x3f036..0x3f044).
        if stats_throttled {
            1.0 / 60.0
        } else {
            f64::INFINITY // IDA 0x3f036
        }
    }
}

// 0x3f058 — __ZN10RobloxView9RenderJob5errorERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
// IDA 0x3f058
impl RenderJob {
    #[doc(alias = "RobloxView::RenderJob::error(RBX::TaskScheduler::Job::Stats const&)")]
    #[doc = "RobloxView::RenderJob::error(RBX::TaskScheduler::Job::Stats const&)"]
    pub fn record_error(&self, stats_throttled: bool, sample: f64) {
        // Throttled: `computeStandardError(stats, 30.0)` (IDA 0x3f058..0x3f080).
        // Unthrottled: zeroes the 9-byte sample (`IDA 0x3f084..0x3f08a`).
        if stats_throttled {
            *self.avg_step_time.lock() = sample / 30.0;
        } else {
            *self.avg_steps_per_second.lock() = 0.0; // IDA 0x3f084
            *self.avg_duty_cycle.lock() = 0.0; // IDA 0x3f086
            *self.avg_step_time.lock() = 0.0; // IDA 0x3f088
        }
    }
}

// 0x3f094 — __ZN10RobloxView9RenderJob16stepDataModelJobERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
// IDA 0x3f094
impl RenderJob {
    #[doc(alias = "RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
    #[doc = "RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)"]
    pub fn step_data_model_job(&self) -> bool {
        // Binds `scheduleRenderPerform(job, view, dt)` and `Submit`s it to the
        // marshaller, logs `Finished renderPerform` under `RenderBreakdown`,
        // returns 1 (IDA 0x3f094..0x3f590).
        self.steps.fetch_add(1, Ordering::SeqCst);
        self.submitted_performs.fetch_add(1, Ordering::SeqCst);
        true // IDA: v29 = 1
    }
}

// 0x3f598 — __ZNK10RobloxView9RenderJob14getMetricValueERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
// IDA 0x3f598
impl RenderJob {
    #[doc(alias = "RobloxView::RenderJob::getMetricValue(std::string const&)const")]
    #[doc = "RobloxView::RenderJob::getMetricValue(std::string const&)const"]
    pub fn metric_value(&self, name: &str) -> f64 {
        match RenderMetric::classify(name) {
            // IDA 0x3f598..0x3f5f0: Job averages.
            RenderMetric::Fps => *self.avg_steps_per_second.lock(),
            RenderMetric::Duty => *self.avg_duty_cycle.lock(),
            RenderMetric::JobTime => *self.avg_step_time.lock(),
            // `1000.0 / GetRenderTimeAverage` (IDA 0x3f5f0..0x3f660).
            RenderMetric::NominalFps => *self.nominal_fps.lock(),
            // `GetDXVideoMemorySize(0) / 1e6` (IDA 0x3f6a0..0x3f6f0).
            RenderMetric::VideoMemoryMb => *self.video_mem_mb.lock(),
            // Delegated breakdown names + unknown names read `0.0`
            // (IDA 0x3f660..0x3f6a0, LABEL_22).
            RenderMetric::ViewDelegated | RenderMetric::Unknown => 0.0,
        }
    }
}

// 0x3f700 — __ZNK10RobloxView9RenderJob9getMetricERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
// IDA 0x3f700
impl RenderJob {
    #[doc(alias = "RobloxView::RenderJob::getMetric(std::string const&)const")]
    #[doc = "RobloxView::RenderJob::getMetric(std::string const&)const"]
    pub fn metric_text(&self, name: &str) -> String {
        // Null view renders `"No View"` (IDA 0x3f700..0x3f7c0); the AA branch
        // renders `"On"`/`"Off"` from `getAntialiasingMode() == 1`, unknown
        // names render `""` (IDA 0x3f7c0..0x3f8f8).
        if self.view_base.lock().is_none() {
            return "No View".to_owned();
        }
        match name {
            "Antialiasing" => {
                if self.antialiasing_on.load(Ordering::SeqCst) {
                    "On".to_owned()
                } else {
                    "Off".to_owned()
                }
            }
            _ => String::new(),
        }
    }
}

// 0x3f904 — __ZThn480_N10RobloxView9RenderJobD1Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
// IDA 0x3f904
impl RenderJob {
    #[doc(alias = "non-virtual thunk toRobloxView::RenderJob::~RenderJob()")]
    #[doc = "non-virtual thunk toRobloxView::RenderJob::~RenderJob() D1"]
    pub fn thunk_destroy_d1(&self) {
        // `this -= 480` adjustor then D1 (IDA 0x3f904..0x3f910); layout adjust
        // is a no-op in Rust.
        self.destroy_d1();
    }
}

// 0x3f9c8 — __ZThn480_N10RobloxView9RenderJobD0Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
// IDA 0x3f9c8
impl RenderJob {
    #[doc(alias = "non-virtual thunk toRobloxView::RenderJob::~RenderJob()")]
    #[doc = "non-virtual thunk toRobloxView::RenderJob::~RenderJob() D0"]
    pub fn thunk_delete_d0(&self) {
        // `this -= 480` adjustor then D0 (IDA 0x3f9c8..0x3f9d4).
        self.delete_d0();
    }
}

// 0x3fa94 — __ZThn480_NK10RobloxView9RenderJob9getMetricERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
// IDA 0x3fa94
impl RenderJob {
    #[doc(alias = "non-virtual thunk toRobloxView::RenderJob::getMetric(std::string const&)const")]
    #[doc = "non-virtual thunk toRobloxView::RenderJob::getMetric(std::string const&)const"]
    pub fn thunk_metric_text(&self, name: &str) -> String {
        // `this -= 480` adjustor then `getMetric` (IDA 0x3fa94..0x3faa0).
        self.metric_text(name)
    }
}

// 0x3faa4 — __ZThn480_NK10RobloxView9RenderJob14getMetricValueERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
// IDA 0x3faa4
impl RenderJob {
    #[doc(alias = "non-virtual thunk toRobloxView::RenderJob::getMetricValue(std::string const&)const")]
    #[doc = "non-virtual thunk toRobloxView::RenderJob::getMetricValue(std::string const&)const"]
    pub fn thunk_metric_value(&self, name: &str) -> f64 {
        // `this -= 480` adjustor then `getMetricValue` (IDA 0x3faa4..0x3faac).
        self.metric_value(name)
    }
}

// 0x3faac — __ZN10RobloxView9RenderJob21scheduleRenderPrepareEPS0_PN3RBX8ViewBaseE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, RenderJob *, ViewBase *)
// IDA 0x3faac
impl RenderJob {
    #[doc(alias = "RobloxView::RenderJob::scheduleRenderPrepare(RobloxView::RenderJob*,RBX::ViewBase *)")]
    #[doc = "RobloxView::RenderJob::scheduleRenderPrepare(RobloxView::RenderJob*,RBX::ViewBase *)"]
    pub fn schedule_render_prepare(&self) -> bool {
        // `if (!this[632]) return peer->VF32(this + 480); return this`
        // (IDA 0x3faac..0x3fac2): unblocked prepares delegate onward.
        if self.prepare_blocks_632.load(Ordering::SeqCst) == 0 {
            self.scheduled_prepares.fetch_add(1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }
}

// 0x3fac4 — __ZN10RobloxView9RenderJob21scheduleRenderPerformEPS0_PN3RBX8ViewBaseEd
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, RobloxView::RenderJob *, RBX::ViewBase *, double)
// IDA 0x3fac4
impl RenderJob {
    #[doc(alias = "RobloxView::RenderJob::scheduleRenderPerform(RobloxView::RenderJob*,RBX::ViewBase *,double)")]
    #[doc = "RobloxView::RenderJob::scheduleRenderPerform(RobloxView::RenderJob*,RBX::ViewBase *,double)"]
    pub fn schedule_render_perform(&self, peer_present: bool, dt: f64) -> bool {
        // Locks the `+496` weak datamodel; with a live model and
        // `this[158] == 0`, calls the peer's VF36 then `wake`
        // (IDA 0x3fac4..0x3fb90). `dt` schedules the perform quantum.
        let _ = dt;
        if self.datamodel.lock().is_some()
            && self.perform_blocks_158.load(Ordering::SeqCst) == 0
        {
            self.scheduled_performs.fetch_add(1, Ordering::SeqCst);
            if peer_present {
                self.wake();
            }
            true
        } else {
            false
        }
    }
}

// 0x3fb9c — __ZN10RobloxView9RenderJob4wakeEv
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this)
// IDA 0x3fb9c
impl RenderJob {
    #[doc(alias = "RobloxView::RenderJob::wake(void)")]
    #[doc = "RobloxView::RenderJob::wake(void)"]
    pub fn wake(&self) -> bool {
        // Locks the scheduler-job weak slot; an expired weak throws
        // `bad_weak_ptr` (IDA 0x3fb9c..0x3fc10) — false models the throw
        // without taking it. Otherwise `TaskScheduler::reschedule`
        // (IDA 0x3fc10..0x3fc30).
        if self.destroyed.load(Ordering::SeqCst) {
            return false;
        }
        self.wakes.fetch_add(1, Ordering::SeqCst);
        self.reschedules.fetch_add(1, Ordering::SeqCst);
        true
    }
    pub fn reschedules(&self) -> u32 {
        self.reschedules.load(Ordering::SeqCst)
    }
}

// 0x39d7c — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEE5resetEv
// type: void
// IDA 0x39d7c
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::reset(void)")]
pub fn reset_render_job_slot(slot: &mut Option<SharedPtr<RenderJob>>) {
    // Releases the owned job and nulls the slot (IDA 0x39d7c..0x39e08);
    // `Arc` drop is the `sp_counted_base::release`.
    *slot = None;
}

// 0x39e10 — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv
// type: void
// IDA 0x39e10
#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::reset(void)")]
pub fn reset_view_update_job_slot(slot: &mut Option<SharedPtr<ViewUpdateJob>>) {
    // Same reset shape for the view-update slot (IDA 0x39e10..0x39e9c).
    *slot = None;
}

// 0x39ea8 — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_
// type: void
// IDA 0x39ea8
#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::operator=(rbx_core::SharedPtr<RobloxView::ViewUpdateJob>&&)")]
pub fn move_assign_view_update_job_slot(
    slot: &mut Option<SharedPtr<ViewUpdateJob>>,
    job: SharedPtr<ViewUpdateJob>,
) {
    // Move-assigns, releasing the previous owner (IDA 0x39ea8..0x39f42).
    *slot = Some(job);
}

// 0x39f4c — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_
// type: void
// IDA 0x39f4c
#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::shared_ptr<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
pub fn wrap_view_update_job(job: ViewUpdateJob) -> SharedPtr<ViewUpdateJob> {
    // Takes ownership of the raw job pointer into a fresh control block
    // (`shared_countC2` + `enable_shared_from_this::accept_owner`,
    // IDA 0x39f4c..0x3a028); `Arc::new` is both.
    SharedPtr::new(job)
}

// 0x3a030 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEaSEOS3_
// type: void
// IDA 0x3a030
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::operator=(rbx_core::SharedPtr<RobloxView::RenderJob>&&)")]
pub fn move_assign_render_job_slot(
    slot: &mut Option<SharedPtr<RenderJob>>,
    job: SharedPtr<RenderJob>,
) {
    // Move-assigns, releasing the previous owner (IDA 0x3a030..0x3a0ca).
    *slot = Some(job);
}

// 0x3a0d4 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEC1IS2_EEPT_
// type: void
// IDA 0x3a0d4
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::shared_ptr<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
pub fn wrap_render_job(job: RenderJob) -> SharedPtr<RenderJob> {
    // Fresh control block + owner acceptance (IDA 0x3a0d4..0x3a1b0).
    SharedPtr::new(job)
}

// 0x3dc60 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void
// IDA 0x3dc60
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(rbx_core::SharedPtr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const")]
pub fn accept_render_job_owner() {
    // `Arc` values are always owned; the weak-owner handshake is a no-op
    // (IDA 0x3dc60: owner already held, nothing to link).
}

// 0x3de48 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void
// IDA 0x3de48
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(rbx_core::SharedPtr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const")]
pub fn accept_view_update_job_owner() {
    // Same no-op handshake for the view-update job (IDA 0x3de48).
}

// 0x3dd34 — __ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_
// type: void
// IDA 0x3dd34
#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
pub fn alloc_render_job_control_block(job: RenderJob) -> SharedPtr<RenderJob> {
    // Allocates the `sp_counted_impl_p` control block (`operator new`, IDA
    // 0x3dd34..0x3de20); `Arc::new` fuses object and control block.
    SharedPtr::new(job)
}

// 0x3df1c — __ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_
// type: void
// IDA 0x3df1c
#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
pub fn alloc_view_update_job_control_block(job: ViewUpdateJob) -> SharedPtr<ViewUpdateJob> {
    // Same fused allocation for the view-update job (IDA 0x3df1c..0x3e008).
    SharedPtr::new(job)
}

/// `rbx::signals::signal` slot registry for the two RobloxView-bound signals:
/// `void()` and `void(PropertyDescriptor const*)`. Each `connect` allocates a
/// `callable_slot` holding the `bind_t` functor and `insert`s it (IDA 0x3a278 /
/// 0x3a390); `boost::bind`/`function` erase to the stored closures, and
/// `intrusive_ptr` ownership erases to the map entry.
#[derive(Default)]
pub struct ViewSignalRegistry {
    next_id: AtomicU32,
    void_slots: parking_lot::Mutex<std::collections::HashMap<u64, Box<dyn Fn() + Send>>>,
    desc_slots: parking_lot::Mutex<std::collections::HashMap<u64, Box<dyn Fn(ObjCId) + Send>>>,
}

impl ViewSignalRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    fn fresh_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst) as u64 + 1
    }
    pub fn void_slot_count(&self) -> usize {
        self.void_slots.lock().len()
    }
    pub fn desc_slot_count(&self) -> usize {
        self.desc_slots.lock().len()
    }
    fn insert_void(&self, handler: Box<dyn Fn() + Send>) -> u64 {
        let id = self.fresh_id();
        self.void_slots.lock().insert(id, handler);
        id
    }
    fn insert_desc(&self, handler: Box<dyn Fn(ObjCId) + Send>) -> u64 {
        let id = self.fresh_id();
        self.desc_slots.lock().insert(id, handler);
        id
    }
    fn remove_void(&self, id: u64) -> bool {
        self.void_slots.lock().remove(&id).is_some()
    }
    fn remove_desc(&self, id: u64) -> bool {
        self.desc_slots.lock().remove(&id).is_some()
    }
}

// 0x3a278 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
// IDA 0x3a278
impl ViewSignalRegistry {
    #[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>> const&)")]
    #[doc = "rbx::signals::signal<void(RBX::Reflection::PropertyDescriptor const*)>::connect RobloxView mf1 slot"]
    pub fn connect_place_id_slot(&self, handler: Box<dyn Fn(ObjCId) + Send>) -> u64 {
        // `new callable_slot` over the `bind_t(mf1 onPlaceIDChanged, view, _1)`
        // + `signal::insert` + weak `add_ref` (IDA 0x3a278..0x3a2ea); the map
        // entry is the slot, the id the connection.
        self.insert_desc(handler)
    }
}

// 0x3a390 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_
// type: int(void)
// IDA 0x3a390
impl ViewSignalRegistry {
    #[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>> const&)")]
    #[doc = "rbx::signals::signal<void()>::connect RobloxView mf0 slot"]
    pub fn connect_void_slot(&self, handler: Box<dyn Fn() + Send>) -> u64 {
        // Same slot-allocate + insert shape for the `mf0` bind (IDA 0x3a390..0x3b002).
        self.insert_void(handler)
    }
}

// 0x3cdb8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED1Ev
// type: void
// IDA 0x3cdb8
impl ViewSignalRegistry {
    #[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()")]
    #[doc = "rbx::signals::signal<void()>::callable_slot RobloxView mf0 destructor D1"]
    pub fn drop_void_slot_d1(&self, id: u64) {
        // D1: `intrusive_ptr_release` on the slot (IDA 0x3cdb8..0x3cd54);
        // map removal is the release.
        self.remove_void(id);
    }
}

// 0x3ce64 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED0Ev
// type: void
// IDA 0x3ce64
impl ViewSignalRegistry {
    #[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()")]
    #[doc = "rbx::signals::signal<void()>::callable_slot RobloxView mf0 destructor D0"]
    pub fn drop_void_slot_d0(&self, id: u64) {
        // D0: D1 then `operator delete` (IDA 0x3ce64..0x3cf10); dropping the
        // boxed closure is the delete.
        self.drop_void_slot_d1(id);
    }
}

// 0x3cf18 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv
// type: void
// IDA 0x3cf18
impl ViewSignalRegistry {
    #[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)")]
    #[doc = "rbx::callable void() RobloxView mf0 slot call"]
    pub fn call_void_slot(&self, id: u64) -> bool {
        // Forwards to the stored `bind_t::operator()` (IDA 0x3cf18..0x3cf1e);
        // false when the slot is already gone.
        match self.void_slots.lock().get(&id) {
            Some(handler) => {
                handler();
                true
            }
            None => false,
        }
    }
}

// 0x3cf20 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv
// type: void
// IDA 0x3cf20
impl ViewSignalRegistry {
    #[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)")]
    #[doc = "non-virtual thunk to rbx::callable void() RobloxView mf0 slot call"]
    pub fn thunk_call_void_slot(&self, id: u64) -> bool {
        // `this -= 4` adjustor then `call` (IDA 0x3cf20..0x3cf26).
        self.call_void_slot(id)
    }
}

// 0x3cf28 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv
// type: int(void)
// IDA 0x3cf28
impl ViewSignalRegistry {
    #[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>::operator()(void)")]
    #[doc = "boost::bind_t void() RobloxView mf0 call operator"]
    pub fn invoke_void_bind(&self, id: u64) -> bool {
        // Unwraps the `mf0` member pointer and calls it on the bound view
        // (IDA 0x3cf28..0x3cf3e); the closure already binds both.
        self.call_void_slot(id)
    }
}

// 0x3d0e4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED1Ev
// type: void
// IDA 0x3d0e4
impl ViewSignalRegistry {
    #[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()")]
    #[doc = "rbx::callable void() RobloxView mf0 destructor D1"]
    pub fn drop_void_callable_d1(&self, id: u64) {
        // D1 releases the slot (IDA 0x3d0e4..0x3d180).
        self.drop_void_slot_d1(id);
    }
}

// 0x3d190 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED0Ev
// type: void
// IDA 0x3d190
impl ViewSignalRegistry {
    #[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()")]
    #[doc = "rbx::callable void() RobloxView mf0 destructor D0"]
    pub fn drop_void_callable_d0(&self, id: u64) {
        // D0: D1 then `operator delete` (IDA 0x3d190..0x3d23c).
        self.drop_void_callable_d1(id);
    }
}

// 0x3d6a8 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED1Ev
// type: void
// IDA 0x3d6a8
impl ViewSignalRegistry {
    #[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>::~callable_slot()")]
    #[doc = "rbx::signals::signal<void(PropertyDescriptor const*)>::callable_slot RobloxView mf1 destructor D1"]
    pub fn drop_desc_slot_d1(&self, id: u64) {
        // D1: `intrusive_ptr_release` (IDA 0x3d6a8..0x3d744).
        self.remove_desc(id);
    }
}

// 0x3d754 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED0Ev
// type: void
// IDA 0x3d754
impl ViewSignalRegistry {
    #[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>::~callable_slot()")]
    #[doc = "rbx::signals::signal<void(PropertyDescriptor const*)>::callable_slot RobloxView mf1 destructor D0"]
    pub fn drop_desc_slot_d0(&self, id: u64) {
        // D0: D1 then `operator delete` (IDA 0x3d754..0x3d800).
        self.drop_desc_slot_d1(id);
    }
}

// 0x3d808 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1Iv10RobloxViewS7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void
// IDA 0x3d808
impl ViewSignalRegistry {
    #[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
    #[doc = "rbx::callable void(PropertyDescriptor const*) RobloxView mf1 slot call"]
    pub fn call_desc_slot(&self, id: u64, descriptor: ObjCId) -> bool {
        // Forwards the descriptor to the stored `bind_t::operator()`
        // (IDA 0x3d808..0x3d81a).
        match self.desc_slots.lock().get(&id) {
            Some(handler) => {
                handler(descriptor);
                true
            }
            None => false,
        }
    }
}

// 0x3d81c — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1Iv10RobloxViewS7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void
// IDA 0x3d81c
impl ViewSignalRegistry {
    #[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
    #[doc = "non-virtual thunk to rbx::callable void(PropertyDescriptor const*) RobloxView mf1 slot call"]
    pub fn thunk_call_desc_slot(&self, id: u64, descriptor: ObjCId) -> bool {
        // `this -= 4` adjustor then `call` (IDA 0x3d81c..0x3d828).
        self.call_desc_slot(id, descriptor)
    }
}

// 0x3d830 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1Iv10RobloxViewPKN3RBX10Reflection18PropertyDescriptorEEENS0_5list2INS0_5valueIPS4_EENS_3argILi1EEEEEEclIS9_EEvRT_
// type: int(void)
// IDA 0x3d830
impl ViewSignalRegistry {
    #[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>::operator()<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* &)")]
    #[doc = "boost::bind_t void(RobloxView, PropertyDescriptor const*) call operator"]
    pub fn invoke_desc_bind(&self, id: u64, descriptor: ObjCId) -> bool {
        // Unwraps the `mf1` member pointer, applies the bound view plus the
        // forwarded `arg<1>` descriptor (IDA 0x3d830..0x3d9e0).
        self.call_desc_slot(id, descriptor)
    }
}

// 0x3d9f0 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1Iv10RobloxViewS7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// type: void
// IDA 0x3d9f0
impl ViewSignalRegistry {
    #[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
    #[doc = "rbx::callable void(PropertyDescriptor const*) RobloxView mf1 destructor D1"]
    pub fn drop_desc_callable_d1(&self, id: u64) {
        // D1 releases the slot (IDA 0x3d9f0..0x3da8c).
        self.drop_desc_slot_d1(id);
    }
}

// 0x3da9c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1Iv10RobloxViewS7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// type: void
// IDA 0x3da9c
impl ViewSignalRegistry {
    #[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
    #[doc = "rbx::callable void(PropertyDescriptor const*) RobloxView mf1 destructor D0"]
    pub fn drop_desc_callable_d0(&self, id: u64) {
        // D0: D1 then `operator delete` (IDA 0x3da9c..0x3dbf0).
        self.drop_desc_callable_d1(id);
    }
}

// 0x3d8e0 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE6removeEPNS8_4slotE
// type: void
// IDA 0x3d8e0
impl ViewSignalRegistry {
    #[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::remove(rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot*)")]
    #[doc = "rbx::signals::signal<void(PropertyDescriptor const*)>::remove slot"]
    pub fn remove_desc_slot(&self, id: u64) -> bool {
        // `ReleaseAssert`s slot ownership, unlinks it, then drops the
        // `intrusive_ptr` (IDA 0x3d8e0..0x3df6c); false models the assert
        // firing on an unknown slot without taking it.
        self.remove_desc(id)
    }
}

// 0x3de28 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED1Ev
// type: void
// IDA 0x3de28
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
pub fn drop_render_job_control_block_d1(job: SharedPtr<RenderJob>) {
    // D1 disposes: destroys the owned job (IDA 0x3de28..0x3de2c);
    // dropping the `Arc` destroys object and fused control block together.
    drop(job);
}

// 0x3de2c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED0Ev
// type: void
// IDA 0x3de2c
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
pub fn drop_render_job_control_block_d0(job: SharedPtr<RenderJob>) {
    // D0: D1 then deletes the control block itself (IDA 0x3de2c..0x3de30).
    drop_render_job_control_block_d1(job);
}

// 0x3de30 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE7disposeEv
// type: void
// IDA 0x3de30
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::dispose(void)")]
pub fn dispose_render_job(job: SharedPtr<RenderJob>) {
    // `dispose` runs the job destructor while the block outlives it
    // (IDA 0x3de30..0x3de40); first D1 half of the fused drop.
    job.destroy_d1();
    drop(job);
}

// 0x3de40 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE11get_deleterERKSt9type_info
// type: void
// IDA 0x3de40
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_deleter(std::type_info const&)")]
pub fn render_job_deleter() -> Option<ObjCId> {
    // `sp_counted_impl_p` stores no deleter: always null unless the queried
    // `type_info` matches, which no caller does (IDA 0x3de40..0x3de44).
    None
}

// 0x3de44 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE19get_untyped_deleterEv
// type: void
// IDA 0x3de44
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_untyped_deleter(void)")]
pub fn render_job_untyped_deleter() -> Option<ObjCId> {
    // Same null deleter through the untyped path (IDA 0x3de44..0x3de48).
    None
}

// 0x3e010 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED1Ev
// type: void
// IDA 0x3e010
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()")]
pub fn drop_view_update_job_control_block_d1(job: SharedPtr<ViewUpdateJob>) {
    // D1 disposes the owned view-update job (IDA 0x3e010..0x3e014).
    drop(job);
}

// 0x3e014 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED0Ev
// type: void
// IDA 0x3e014
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()")]
pub fn drop_view_update_job_control_block_d0(job: SharedPtr<ViewUpdateJob>) {
    // D0: D1 then control-block delete (IDA 0x3e014..0x3e018).
    drop_view_update_job_control_block_d1(job);
}

// 0x3e018 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE7disposeEv
// type: void
// IDA 0x3e018
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::dispose(void)")]
pub fn dispose_view_update_job(job: SharedPtr<ViewUpdateJob>) {
    // `dispose` destroys the job object (IDA 0x3e018..0x3e028).
    drop(job);
}

// 0x3e028 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE11get_deleterERKSt9type_info
// type: void
// IDA 0x3e028
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_deleter(std::type_info const&)")]
pub fn view_update_job_deleter() -> Option<ObjCId> {
    // Null deleter, same as the render-job twin (IDA 0x3e028..0x3e02c).
    None
}

// 0x3e02c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE19get_untyped_deleterEv
// type: void
// IDA 0x3e02c
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_untyped_deleter(void)")]
pub fn view_update_job_untyped_deleter() -> Option<ObjCId> {
    // Null deleter through the untyped path (IDA 0x3e02c..0x3e030).
    None
}

// 0x25440 — -[PlaceLauncher deleteRobloxView]
// type: void __cdecl(PlaceLauncher *self, SEL)
// IDA 0x25440
impl PlaceLauncherViewSlot {
    #[doc(alias = "-[PlaceLauncher deleteRobloxView]")]
    #[doc = "-[PlaceLauncher deleteRobloxView]"]
    pub fn delete_roblox_view(&self) {
        // `rbxView = self->rbxView; if (rbxView)` (IDA 0x25440..0x25448): the
        // checker stops only when a view existed.
        if self.view.lock().take().is_some() {
            // `self->rbxView = 0` then `~RobloxView` + `operator delete`
            // (IDA 0x2544a..0x25458); `Arc` drop is the delete.
            // `-[RobloxMemoryManager stopFreeMemoryChecker]` (IDA 0x2545c..0x25464).
            self.free_memory_checker_running.store(false, Ordering::SeqCst);
        }
    }
}

/// Outcome of `initControlViewHelper`: which UIKit/render-system steps ran.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ControlInitOutcome {
    pub set_ogre_view: bool,
    pub created_control_view: bool,
    pub subview_adds: u32,
    pub set_ogre_window: bool,
    pub dispatched_main: bool,
}

// 0x2643c — __ZL15initControlViewP10RobloxViewaPN3RBX18FunctionMarshallerE
// type: _DWORD __fastcall(RobloxView *, signed __int8, RBX::FunctionMarshaller *)
// IDA 0x2643c
#[doc(alias = "initControlView(RobloxView *,signed char,RBX::FunctionMarshaller *)")]
pub fn init_control_view(
    view: &RobloxView,
    main: &MainViewState,
    render_window_present: bool,
    restart: bool,
) -> ControlInitOutcome {
    // Binds `initControlViewHelper(view, restart)` into `function0<void>` and
    // runs it via `FunctionMarshaller::Execute(..., 0)` then `clear`s it
    // (IDA 0x2643c..0x2647c); the bound closure runs inline here.
    init_control_view_helper(view, main, render_window_present, restart)
}

// 0x2aba4 — __ZL14finishTeleportP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEEPNS3_18FunctionMarshallerE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
// IDA 0x2aba4
#[doc(alias = "finishTeleport(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *)")]
pub fn finish_teleport(
    view: &RobloxView,
    main: &MainViewState,
    game: &SharedPtr<GameHandle>,
) -> bool {
    // Binds `finishTeleportHelper(view, game)` into `function0<void>`,
    // `Execute`s it, `clear`s, then releases both shared counts
    // (IDA 0x2aba4..0x2ac1c); `Arc` drops are the releases.
    finish_teleport_helper(view, main, game)
}

// 0x2b754 — __ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE
// type: void
// IDA 0x2b754
#[doc(alias = "finishTeleportHelper(RobloxView *,rbx_core::SharedPtr<RBX::Game>)")]
pub fn finish_teleport_helper(
    view: &RobloxView,
    main: &MainViewState,
    game: &SharedPtr<GameHandle>,
) -> bool {
    // `MainViewController sharedInstance`; the whole body is under `if (v11)`
    // (IDA 0x2b754..0x2b78c): nil controller skips everything.
    if !main.present {
        return false;
    }
    // Enumerates `-[UIView subviews]` of the ogre view controller's view and
    // keeps the first (`v15`, IDA 0x2b78c..0x2b7f0); empty enumeration skips
    // both `setGame:` and the animation below.
    let first = main.ogre_subviews.lock().first().copied();
    match first {
        None => false,
        Some(subview) => {
            // `-[subview setGame:game]` (IDA 0x2b7f0..0x2b8a0).
            view.note_game_bound(subview, game.id);
            // `+[UIView animateWithDuration:0.5 delay:0 options:0 ...]` with the
            // frame block (0x2b980) and the clips block (0x2ba14)
            // (IDA 0x2b8a0..0x2b97c).
            view.note_teleport_animation(TeleportAnimation {
                duration: 0.5,
                delay: 0.0,
                options: 0,
            });
            true
        }
    }
}

/// `+[UIView animateWithDuration:...]` parameters captured by `finishTeleportHelper`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TeleportAnimation {
    pub duration: f64,
    pub delay: f64,
    pub options: u32,
}

// 0x2b980 — ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke
// type: void
// IDA 0x2b980
#[doc(alias = "____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke")]
pub fn finish_teleport_animation_frame(screen_bounds: Option<ViewRect>) -> ViewRect {
    // Animation block: `setFrame:` to `-[UIScreen mainScreen] bounds`
    // (IDA 0x2b980..0x2b9a0); nil screen falls back to the zero rect.
    screen_bounds.unwrap_or(ViewRect::ZERO) // IDA 0x2b998
}

// 0x2ba14 — ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428
// type: void
// IDA 0x2ba14
#[doc(alias = "____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428")]
pub fn finish_teleport_completion_clips_to_bounds() -> bool {
    // Completion block: `setClipsToBounds:0` (IDA 0x2ba14..0x2ba24).
    false // IDA 0x2ba20
}

// 0x2c224 — __ZL21initControlViewHelperP10RobloxViewa
// type: _DWORD __fastcall(RobloxView *, signed __int8)
// IDA 0x2c224
#[doc(alias = "initControlViewHelper(RobloxView *,signed char)")]
pub fn init_control_view_helper(
    view: &RobloxView,
    main: &MainViewState,
    render_window_present: bool,
    flag: bool,
) -> ControlInitOutcome {
    // Skips everything when the controller is nil or `view->var7` (render
    // window) is nil (IDA 0x2c224..0x2c27c).
    if !main.present || !render_window_present {
        return ControlInitOutcome::default();
    }
    // `renderWindow->get("VIEW")` then `-[MainViewController setOgreView:]`
    // (IDA 0x2c27c..0x2c330).
    view.set_ogre_view(main.ogre_subviews.lock().first().copied().unwrap_or(NIL_ID));
    // `-[ControlView alloc]` + `init:withGame:` with the main-screen bounds
    // (zero when `mainScreen` is nil), `autorelease` (IDA 0x2c330..0x2c3c0).
    view.set_control_view_present(true);
    // BUG: original adds the control view twice (`addSubview:v25` at both
    // IDA 0x2c3c0 and 0x2c430); preserved here as two counted adds.
    view.add_control_subview();
    view.add_control_subview();
    // `renderWindow->get("WINDOW")` then `setOgreWindow:` (IDA 0x2c3c8..0x2c428).
    view.set_ogre_window(1);
    // `if (a2) dispatch_async(main, block505)` (IDA 0x2c438..0x2c448).
    if flag {
        view.note_main_dispatch();
    }
    ControlInitOutcome {
        set_ogre_view: true,
        created_control_view: true,
        subview_adds: 2,
        set_ogre_window: true,
        dispatched_main: flag,
    }
}

/// `bind(initControlView, view, restart, marshaller)` argument triple.
#[derive(Debug, Clone, Default)]
pub struct InitControlArgs {
    pub view: Option<ObjCId>,
    pub restart: bool,
    pub marshaller: Option<ObjCId>,
}

/// `bind(initControlViewHelper, view, restart)` argument pair.
#[derive(Debug, Clone, Default)]
pub struct InitControlPair {
    pub view: Option<ObjCId>,
    pub restart: bool,
}

/// `boost::function<void(DataModel*)>` holding the marshalled finish-teleport
/// triple; the call operator ignores its argument (the game is bound).
#[derive(Debug, Clone, Default)]
pub struct DatamodelCallback {
    pub args: FinishTeleportMarshalledArgs,
}

/// `boost::function<void()>` holding the finish-teleport-helper pair.
#[derive(Debug, Clone, Default)]
pub struct VoidCallback {
    pub args: FinishTeleportArgs,
}

// 0x2d280 — __ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
// type: void
// IDA 0x2d280
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list_av_3<RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *>::type> boost::bind<void,RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *,RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *>(void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *)")]
pub fn bind_finish_teleport(view: ObjCId, game: u32, marshaller: ObjCId) -> FinishTeleportMarshalledArgs {
    // Captures `(finishTeleport, view, game, marshaller)` into the triple
    // (IDA 0x2d280..0x2d36c); the closure captures the same three values.
    jump_build_finish_teleport_marshalled_args(view, game, marshaller)
}

// 0x2d370 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: void
// IDA 0x2d370
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn wrap_finish_teleport_callback(args: FinishTeleportMarshalledArgs) -> DatamodelCallback {
    // `function<void(DataModel*)>` ctor from the triple bind
    // (IDA 0x2d370..0x2d450); the invocable holds the triple.
    DatamodelCallback { args }
}

// 0x2d458 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: void
// IDA 0x2d458
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn wrap_finish_teleport_callback1(args: FinishTeleportMarshalledArgs) -> DatamodelCallback {
    // `function1` single-argument ctor, same capture (IDA 0x2d458..0x2d538).
    DatamodelCallback { args }
}

// 0x2d544 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_
// type: void
// IDA 0x2d544
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)")]
pub fn assign_finish_teleport_callback(
    slot: &mut Option<DatamodelCallback>,
    args: FinishTeleportMarshalledArgs,
) {
    // Stores the triple functor into the `function1` buffer
    // (IDA 0x2d544..0x2d63c).
    *slot = Some(DatamodelCallback { args });
}

// 0x2d644 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// type: void
// IDA 0x2d644
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn manage_finish_teleport_fn(op: FunctorOp, slot: &mut Option<FinishTeleportMarshalledArgs>) -> bool {
    // Clone/destroy over the marshalled triple (IDA 0x2d644..0x2d65c).
    manage_boxed_slot(op, slot)
}

// 0x2d660 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEvPNS8_9DataModelEE6invokeERNS1_15function_bufferESN_
// type: void
// IDA 0x2d660
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn invoke_finish_teleport_fn(
    view: &RobloxView,
    main: &MainViewState,
    args: &FinishTeleportMarshalledArgs,
) -> bool {
    // Invokes the triple with the forwarded `DataModel*` ignored — the game
    // is already bound (IDA 0x2d660..0x2d678).
    jump_apply_finish_teleport_marshalled_args(view, main, args)
}

// 0x2d67c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// type: void
// IDA 0x2d67c
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const")]
pub fn vtable_assign_finish_teleport_fn(
    slot: &mut Option<DatamodelCallback>,
    args: FinishTeleportMarshalledArgs,
) -> bool {
    // `basic_vtable1::assign_to` without tag: stores and reports success
    // (IDA 0x2d67c..0x2d760).
    *slot = Some(DatamodelCallback { args });
    true
}

// 0x2d768 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: void
// IDA 0x2d768
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn vtable_assign_tagged_finish_teleport_fn(
    slot: &mut Option<DatamodelCallback>,
    args: FinishTeleportMarshalledArgs,
) -> bool {
    // Tagged `assign_to` overload: same store-and-true (IDA 0x2d768..0x2d87c).
    *slot = Some(DatamodelCallback { args });
    true
}

// 0x2d884 — __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// type: void
// IDA 0x2d884
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
pub fn apply_finish_teleport_list(
    view: &RobloxView,
    main: &MainViewState,
    args: &FinishTeleportMarshalledArgs,
) -> bool {
    // Unpacks `(view, game, marshaller)` and calls `finishTeleport`
    // (IDA 0x2d884..0x2d95c).
    jump_apply_finish_teleport_marshalled_args(view, main, args)
}

// 0x2d964 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void
// IDA 0x2d964
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn manager_small_finish_teleport_fn(op: FunctorOp, slot: &mut Option<FinishTeleportMarshalledArgs>) -> bool {
    // Small-object (`mpl::false_`) manager: same clone/destroy
    // (IDA 0x2d964..0x2da94).
    manage_boxed_slot(op, slot)
}

// 0x2da9c — __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
// type: void
// IDA 0x2da9c
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::list3(boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")]
pub fn build_finish_teleport_list(view: ObjCId, game: u32, marshaller: ObjCId) -> FinishTeleportMarshalledArgs {
    // `list3` value-triple ctor (IDA 0x2da9c..0x2db4c).
    FinishTeleportMarshalledArgs { view: Some(view), game: Some(game), marshaller: Some(marshaller) }
}

// 0x2db54 — __ZN5boost3_bi8storage3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
// type: void
// IDA 0x2db54
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::storage3(boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")]
pub fn build_finish_teleport_storage(view: ObjCId, game: u32, marshaller: ObjCId) -> FinishTeleportMarshalledArgs {
    // `storage3` wraps the same triple (IDA 0x2db54..0x2dc10).
    build_finish_teleport_list(view, game, marshaller)
}

// 0x312d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// type: void
// IDA 0x312d0
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn manage_init_control_fn(op: FunctorOp, slot: &mut Option<InitControlArgs>) -> bool {
    // Clone/destroy over the `(view, restart, marshaller)` triple
    // (IDA 0x312d0..0x31340).
    manage_boxed_slot(op, slot)
}

// 0x31348 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_
// type: void
// IDA 0x31348
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn invoke_init_control_fn(
    view: &RobloxView,
    main: &MainViewState,
    render_window_present: bool,
    args: &InitControlArgs,
) -> ControlInitOutcome {
    // Invokes `initControlView` with the bound triple (IDA 0x31348..0x31356).
    init_control_view(
        view,
        main,
        render_window_present,
        args.restart,
    )
}

// 0x327d4 — __ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEES2_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// type: void
// IDA 0x327d4
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<RobloxView *,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,RobloxView *,rbx_core::SharedPtr<RBX::Game>,RobloxView *,rbx_core::SharedPtr<RBX::Game>>(void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),RobloxView *,rbx_core::SharedPtr<RBX::Game>)")]
pub fn bind_finish_teleport_helper(view: ObjCId, game: u32) -> FinishTeleportArgs {
    // Captures `(finishTeleportHelper, view, game)` (IDA 0x327d4..0x328b4).
    jump_build_finish_teleport_args(view, game)
}

// 0x328bc — __ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
// type: void
// IDA 0x328bc
#[doc(alias = "boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn build_finish_teleport_helper_list(view: ObjCId, game: u32) -> FinishTeleportArgs {
    // `list2` value-pair ctor (IDA 0x328bc..0x3297c).
    FinishTeleportArgs { view: Some(view), game: Some(game) }
}

// 0x32984 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: void
// IDA 0x32984
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn wrap_finish_teleport_helper_callback(args: FinishTeleportArgs) -> VoidCallback {
    // `function<void()>` ctor from the pair bind (IDA 0x32984..0x32a60).
    VoidCallback { args }
}

// 0x32a68 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: void
// IDA 0x32a68
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn wrap_finish_teleport_helper_callback0(args: FinishTeleportArgs) -> VoidCallback {
    // `function0` ctor, same capture (IDA 0x32a68..0x32b48).
    VoidCallback { args }
}

// 0x32b50 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEEvT_
// type: void
// IDA 0x32b50
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
pub fn assign_finish_teleport_helper_callback(
    slot: &mut Option<VoidCallback>,
    args: FinishTeleportArgs,
) {
    // Stores the pair functor into the `function0` buffer (IDA 0x32b50..0x32c40).
    *slot = Some(VoidCallback { args });
}

// 0x32c48 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// type: void
// IDA 0x32c48
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn manage_finish_teleport_helper_fn(op: FunctorOp, slot: &mut Option<FinishTeleportArgs>) -> bool {
    // Clone/destroy over the `(view, game)` pair (IDA 0x32c48..0x32c60).
    manage_boxed_slot(op, slot)
}

// 0x32c64 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
// type: void
// IDA 0x32c64
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn invoke_finish_teleport_helper_fn(
    view: &RobloxView,
    main: &MainViewState,
    args: &FinishTeleportArgs,
) -> bool {
    // Invokes `finishTeleportHelper` from the pair (IDA 0x32c64..0x32c74).
    jump_apply_finish_teleport_args(view, main, args)
}

// 0x32c78 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// type: void
// IDA 0x32c78
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
pub fn vtable_assign_finish_teleport_helper_fn(
    slot: &mut Option<VoidCallback>,
    args: FinishTeleportArgs,
) -> bool {
    // Untagged `assign_to`: store-and-true (IDA 0x32c78..0x32d58).
    *slot = Some(VoidCallback { args });
    true
}

// 0x32d60 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: void
// IDA 0x32d60
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn vtable_assign_tagged_finish_teleport_helper_fn(
    slot: &mut Option<VoidCallback>,
    args: FinishTeleportArgs,
) -> bool {
    // Tagged `assign_to` overload: same store-and-true (IDA 0x32d60..0x32e6c).
    *slot = Some(VoidCallback { args });
    true
}

// 0x32e74 — __ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: void
// IDA 0x32e74
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn apply_finish_teleport_helper_list(
    view: &RobloxView,
    main: &MainViewState,
    args: &FinishTeleportArgs,
) -> bool {
    // Unpacks `(view, game)` and calls `finishTeleportHelper`
    // (IDA 0x32e74..0x32f44).
    jump_apply_finish_teleport_args(view, main, args)
}

// 0x32f4c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void
// IDA 0x32f4c
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn manager_small_finish_teleport_helper_fn(op: FunctorOp, slot: &mut Option<FinishTeleportArgs>) -> bool {
    // Small-object manager over the pair (IDA 0x32f4c..0x33468).
    manage_boxed_slot(op, slot)
}

// 0x33470 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewaENS3_5list2INS3_5valueIS6_EENSA_IaEEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// type: void
// IDA 0x33470
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn manage_init_control_pair(op: FunctorOp, slot: &mut Option<InitControlPair>) -> bool {
    // Clone/destroy over the `(view, restart)` pair (IDA 0x33470..0x334c8).
    manage_boxed_slot(op, slot)
}

// 0x334d0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP10RobloxViewaENS3_5list2INS3_5valueIS6_EENSA_IaEEEEEEvE6invokeERNS1_15function_bufferE
// type: void
// IDA 0x334d0
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn invoke_init_control_pair(
    view: &RobloxView,
    main: &MainViewState,
    render_window_present: bool,
    args: &InitControlPair,
) -> ControlInitOutcome {
    // Invokes `initControlViewHelper` from the pair (IDA 0x334d0..0x334f0).
    init_control_view_helper(view, main, render_window_present, args.restart)
}






/// `functor_manager::manage` operation (`functor_manager_operation_type`):
/// clone allocates a copy, destroy drops in place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctorOp {
    Clone,
    Destroy,
}

/// Manages an optionally-held boxed functor: clone duplicates the holder
/// (`Arc` bumps are the `shared_count` copies), destroy empties it.
pub fn manage_boxed_slot<T>(op: FunctorOp, slot: &mut Option<T>) -> bool
where
    T: Clone,
{
    match op {
        FunctorOp::Clone => slot.is_some(),
        FunctorOp::Destroy => slot.take().is_some(),
    }
}

/// `bind(scheduleRenderPerform, job, view, dt)` argument triple.
#[derive(Debug, Clone, Default)]
pub struct RenderPerformClosure {
    pub job: Option<SharedPtr<RenderJob>>,
    pub view: Option<ObjCId>,
    pub dt: f64,
}

/// `bind(scheduleRenderPrepare, job, view)` argument pair.
#[derive(Debug, Clone, Default)]
pub struct RenderPrepareClosure {
    pub job: Option<SharedPtr<RenderJob>>,
    pub view: Option<ObjCId>,
}

/// `bind(mf2 ViewBase::metric, view, job, dt)` argument triple.
#[derive(Debug, Clone, Default)]
pub struct MetricForwardClosure {
    pub view: Option<ObjCId>,
    pub job: Option<SharedPtr<RenderJob>>,
    pub dt: f64,
}

// 0x40160 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// type: void
// IDA 0x40160
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn manage_render_perform_closure(op: FunctorOp, slot: &mut Option<RenderPerformClosure>) -> bool {
    // Clone/destroy over the `(job, view, dt)` triple (IDA 0x40160..0x401d8).
    manage_boxed_slot(op, slot)
}

// 0x401dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEvE6invokeERNS1_15function_bufferE
// type: void
// IDA 0x401dc
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn invoke_render_perform_closure(slot: &Option<RenderPerformClosure>) -> bool {
    // Calls `scheduleRenderPerform(job, view, dt)` from the triple
    // (IDA 0x401dc..0x401ee).
    match slot {
        Some(bound) => match &bound.job {
            Some(job) => job.schedule_render_perform(true, bound.dt),
            None => false,
        },
        None => false,
    }
}

// 0x402a8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// type: void
// IDA 0x402a8
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn manage_render_prepare_closure(op: FunctorOp, slot: &mut Option<RenderPrepareClosure>) -> bool {
    // Clone/destroy over the `(job, view)` pair (IDA 0x402a8..0x40300).
    manage_boxed_slot(op, slot)
}

// 0x40308 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
// type: void
// IDA 0x40308
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<RBX::ViewBase *>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn invoke_render_prepare_closure(slot: &Option<RenderPrepareClosure>) -> bool {
    // Calls `scheduleRenderPrepare(job, view)` from the pair
    // (IDA 0x40308..0x403e8, via `list2::operator()`).
    match slot {
        Some(bound) => match &bound.job {
            Some(job) => job.schedule_render_prepare(),
            None => false,
        },
        None => false,
    }
}

// 0x401f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// type: void
// IDA 0x401f0
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn manage_metric_forward_closure(op: FunctorOp, slot: &mut Option<MetricForwardClosure>) -> bool {
    // Clone/destroy over the `(view, job, dt)` triple (IDA 0x401f0..0x40268).
    manage_boxed_slot(op, slot)
}

// 0x40270 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEvE6invokeERNS1_15function_bufferE
// type: void
// IDA 0x40270
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn invoke_metric_forward_closure(
    job: &RenderJob,
    slot: &Option<MetricForwardClosure>,
    metric: ObjCId,
) -> bool {
    // Applies the `mf2` metric hook on the bound view with `(metric, dt)`
    // (IDA 0x40270..0x4027a, via `list3::operator()`); the view half is out
    // of slice, so the forward is recorded on the job.
    if slot.is_some() {
        job.scheduled_performs.fetch_add(1, Ordering::SeqCst);
        let _ = metric;
        true
    } else {
        false
    }
}

// 0x4027c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
// IDA 0x4027c
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double> &,boost::_bi::list0 &,int)")]
pub fn apply_metric_forward_list(
    job: &RenderJob,
    slot: &Option<MetricForwardClosure>,
    metric: ObjCId,
) -> bool {
    // Unpacks `(view, job, dt)` and applies `mf2` (IDA 0x4027c..0x402a6).
    invoke_metric_forward_closure(job, slot, metric)
}

// 0x403f0 — __ZN10RobloxView13ViewUpdateJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerE
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this, RBX::ViewBase *, struct _Unwind_Exception *lpuexcpt)
// IDA 0x403f0
impl ViewUpdateJob {
    #[doc(alias = "RobloxView::ViewUpdateJob::ViewUpdateJob(RBX::ViewBase *,RBX::FunctionMarshaller *)")]
    #[doc = "RobloxView::ViewUpdateJob::ViewUpdateJob(RBX::ViewBase *,RBX::FunctionMarshaller *)"]
    pub fn new(view_base: ObjCId, marshaller: ObjCId) -> Self {
        // `Job::Job("UpdateRbxView", ...)`, vtable, zeroed counter slots
        // (IDA 0x403f0..0x404e8); GL/view creation is out of slice.
        Self { view_base: Some(view_base), marshaller: Some(marshaller), ..Self::default() }
    }
    pub fn steps(&self) -> u32 {
        self.steps.load(Ordering::SeqCst)
    }
    pub fn is_destroyed(&self) -> bool {
        self.destroyed.load(Ordering::SeqCst)
    }
}

// 0x404f0 — __ZN10RobloxView13ViewUpdateJobD1Ev
// type: void __fastcall(RobloxView::ViewUpdateJob *__hidden this)
// IDA 0x404f0
impl ViewUpdateJob {
    #[doc(alias = "RobloxView::ViewUpdateJob::~ViewUpdateJob()")]
    #[doc = "RobloxView::ViewUpdateJob::~ViewUpdateJob() complete destructor"]
    pub fn destroy_d1(&self) {
        // Restores the vtable then `Job::~Job` (IDA 0x404f0..0x40590).
        self.destroyed.store(true, Ordering::SeqCst);
    }
}

// 0x4059c — __ZN10RobloxView13ViewUpdateJobD0Ev
// type: void __fastcall(RobloxView::ViewUpdateJob *__hidden this)
// IDA 0x4059c
impl ViewUpdateJob {
    #[doc(alias = "RobloxView::ViewUpdateJob::~ViewUpdateJob()")]
    #[doc = "RobloxView::ViewUpdateJob::~ViewUpdateJob() deleting destructor"]
    pub fn delete_d0(&self) {
        // D0 runs D1 then `operator delete` (IDA 0x4059c..0x40648).
        self.destroy_d1();
    }
}

// 0x40650 — __ZN10RobloxView13ViewUpdateJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
// IDA 0x40650
impl ViewUpdateJob {
    #[doc(alias = "RobloxView::ViewUpdateJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
    #[doc = "RobloxView::ViewUpdateJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)"]
    pub fn sleep_time_secs(&self) -> f64 {
        // Unconditionally `computeStandardSleepTime(stats, 60.0)`
        // (IDA 0x40650..0x40674); the averaging core is out of slice, the
        // nominal 60fps quantum is exact.
        1.0 / 60.0
    }
}

// 0x40680 — __ZN10RobloxView13ViewUpdateJob5errorERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
// IDA 0x40680
impl ViewUpdateJob {
    #[doc(alias = "RobloxView::ViewUpdateJob::error(RBX::TaskScheduler::Job::Stats const&)")]
    #[doc = "RobloxView::ViewUpdateJob::error(RBX::TaskScheduler::Job::Stats const&)"]
    pub fn record_error(&self, sample: f64) {
        // Unconditionally `computeStandardError(stats, 30.0)`
        // (IDA 0x40680..0x406a4).
        *self.error_sample.lock() = sample / 30.0;
    }
}

// 0x406a8 — __ZN10RobloxView13ViewUpdateJob17getPriorityFactorEv
// type: _DWORD __fastcall(RobloxView::ViewUpdateJob *__hidden this)
// IDA 0x406a8
impl ViewUpdateJob {
    #[doc(alias = "RobloxView::ViewUpdateJob::getPriorityFactor(void)")]
    #[doc = "RobloxView::ViewUpdateJob::getPriorityFactor(void)"]
    pub fn priority_factor(&self) -> f64 {
        1.0 // IDA 0x406a8..0x406b2: `return 1.0`
    }
}

// 0x406b4 — __ZN10RobloxView13ViewUpdateJob4stepERKN3RBX13TaskScheduler3Job5StatsE
// type: void
// IDA 0x406b4
impl ViewUpdateJob {
    #[doc(alias = "RobloxView::ViewUpdateJob::step(RBX::TaskScheduler::Job::Stats const&)")]
    #[doc = "RobloxView::ViewUpdateJob::step(RBX::TaskScheduler::Job::Stats const&)"]
    pub fn step(&self) -> bool {
        // `if (view->needsUpdate() == 1) view->update(); view->present();`
        // then `return 1` (IDA 0x406b4..0x40710).
        if self.view_update_pending.load(Ordering::SeqCst) {
            self.update_passes.fetch_add(1, Ordering::SeqCst);
            self.view_update_pending.store(false, Ordering::SeqCst);
        }
        self.present_passes.fetch_add(1, Ordering::SeqCst);
        self.steps.fetch_add(1, Ordering::SeqCst);
        true // IDA: return 1
    }
    pub fn mark_view_update_pending(&self) {
        self.view_update_pending.store(true, Ordering::SeqCst);
    }
}

// 0x51f80 — -[MainViewController setRobloxView:]
// type: void __cdecl(MainViewController *self, SEL, RobloxView *)
// IDA 0x51f80
impl MainViewState {
    #[doc(alias = "-[MainViewController setRobloxView:]")]
    #[doc = "-[MainViewController setRobloxView:]"]
    pub fn set_roblox_view(&self, view: SharedPtr<RobloxView>) {
        // Retains the new view over the old ivar (IDA 0x51f80..0x51f8e);
        // `Arc` clone-then-store is the retain.
        *self.roblox_view.lock() = Some(view);
    }
}

// 0x51f90 — -[MainViewController getRobloxView]
// type: RobloxView *__cdecl(MainViewController *self, SEL)
// IDA 0x51f90
impl MainViewState {
    #[doc(alias = "-[MainViewController getRobloxView]")]
    #[doc = "-[MainViewController getRobloxView]"]
    pub fn roblox_view(&self) -> Option<SharedPtr<RobloxView>> {
        // Autoreleased ivar load (IDA 0x51f90..0x519a0); the clone extends the
        // borrow past the autorelease pool.
        self.roblox_view.lock().clone()
    }
}

/// `bind(finishTeleportHelper, view, game)` argument pair.
#[derive(Debug, Clone, Default)]
pub struct FinishTeleportArgs {
    pub view: Option<ObjCId>,
    pub game: Option<u32>,
}

/// `bind(finishTeleport, view, game, marshaller)` argument triple.
#[derive(Debug, Clone, Default)]
pub struct FinishTeleportMarshalledArgs {
    pub view: Option<ObjCId>,
    pub game: Option<u32>,
    pub marshaller: Option<ObjCId>,
}

// 0xf1f1c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: void
// IDA 0xf1f1c8
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn shim_manage_finish_teleport_fn(op: FunctorOp, slot: &mut Option<FinishTeleportMarshalledArgs>) -> bool {
    // Tail-call trampoline into the real `manager` (IDA 0xf1f1c8: `B` to the target).
    manage_boxed_slot(op, slot)
}

// 0xf1f270 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: void
// IDA 0xf1f270
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn shim_manage_finish_teleport_helper_fn(op: FunctorOp, slot: &mut Option<FinishTeleportArgs>) -> bool {
    // Tail-call trampoline into the `(view, game)` manager (IDA 0xf1f270).
    manage_boxed_slot(op, slot)
}

// 0xf1f2f4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv$shim
// type: void
// IDA 0xf1f2f4
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv$shim")]
pub fn shim_invoke_void_bind(registry: &ViewSignalRegistry, id: u64) -> bool {
    // Tail-call trampoline into `bind_t::operator()` (IDA 0xf1f2f4).
    registry.invoke_void_bind(id)
}

// 0xf1f348 — __ZNK10RobloxView9RenderJob14getMetricValueERKSs$shim
// type: void
// IDA 0xf1f348
#[doc(alias = "__ZNK10RobloxView9RenderJob14getMetricValueERKSs$shim")]
pub fn shim_metric_value(job: &RenderJob, name: &str) -> f64 {
    // Tail-call trampoline into `getMetricValue` (IDA 0xf1f348).
    job.metric_value(name)
}

// 0xf1f360 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i$shim
// type: void
// IDA 0xf1f360
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i$shim")]
pub fn shim_apply_metric_forward_list(
    job: &RenderJob,
    slot: &Option<MetricForwardClosure>,
    metric: ObjCId,
) -> bool {
    // Tail-call trampoline into `list3::operator()` (IDA 0xf1f360).
    apply_metric_forward_list(job, slot, metric)
}

// 0xf267d4 — j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
// IDA 0xf267d4
#[doc(alias = "boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn jump_build_finish_teleport_args(view: ObjCId, game: u32) -> FinishTeleportArgs {
    // Import trampoline (`j_`): constructs the `(view, game)` pair.
    FinishTeleportArgs { view: Some(view), game: Some(game) }
}

// 0xf267e4 — j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// IDA 0xf267e4
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn jump_apply_finish_teleport_args(
    view: &RobloxView,
    main: &MainViewState,
    args: &FinishTeleportArgs,
) -> bool {
    // Import trampoline: applies `finishTeleportHelper` to the pair.
    match (args.view, args.game) {
        (Some(_), Some(game_id)) => {
            finish_teleport_helper(view, main, &SharedPtr::new(GameHandle { id: game_id }))
        }
        _ => false,
    }
}

// 0xf26834 — j___ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
// IDA 0xf26834
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::list3(boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")]
pub fn jump_build_finish_teleport_marshalled_args(
    view: ObjCId,
    game: u32,
    marshaller: ObjCId,
) -> FinishTeleportMarshalledArgs {
    // Import trampoline: constructs the `(view, game, marshaller)` triple.
    FinishTeleportMarshalledArgs { view: Some(view), game: Some(game), marshaller: Some(marshaller) }
}

// 0xf26844 — j___ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// IDA 0xf26844
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
pub fn jump_apply_finish_teleport_marshalled_args(
    view: &RobloxView,
    main: &MainViewState,
    args: &FinishTeleportMarshalledArgs,
) -> bool {
    // Import trampoline: applies `finishTeleport` to the triple.
    match (args.view, args.game) {
        (Some(_), Some(game_id)) => {
            finish_teleport(view, main, &SharedPtr::new(GameHandle { id: game_id }))
        }
        _ => false,
    }
}

// 0xf26904 — j___ZN5boost3_bi8storage3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
// IDA 0xf26904
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::storage3(boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")]
pub fn jump_build_finish_teleport_storage(
    view: ObjCId,
    game: u32,
    marshaller: ObjCId,
) -> FinishTeleportMarshalledArgs {
    // Import trampoline into the `storage3` ctor; storage wraps the same triple.
    jump_build_finish_teleport_marshalled_args(view, game, marshaller)
}

// 0xf26da4 — j___ZN10RobloxView13ViewUpdateJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerE
// type: int __fastcall(RobloxView::ViewUpdateJob *this, RBX::ViewBase *, struct _Unwind_Exception *lpuexcpt)
// IDA 0xf26da4
#[doc(alias = "RobloxView::ViewUpdateJob::ViewUpdateJob(RBX::ViewBase *,RBX::FunctionMarshaller *)")]
pub fn jump_new_view_update_job(view_base: ObjCId, marshaller: ObjCId) -> ViewUpdateJob {
    // Import trampoline into the `ViewUpdateJob` ctor.
    ViewUpdateJob::new(view_base, marshaller)
}

// 0xf26db4 — j___ZN10RobloxView9RenderJob4wakeEv
// type: int __fastcall(RobloxView::RenderJob *this)
// IDA 0xf26db4
#[doc(alias = "RobloxView::RenderJob::wake(void)")]
pub fn jump_wake_render_job(job: &RenderJob) -> bool {
    // Import trampoline into `RenderJob::wake`.
    job.wake()
}

// 0xf27264 — j___ZNK10RobloxView9RenderJob14getMetricValueERKSs
// type: int __fastcall(RobloxView::RenderJob *this, const std::string *)
// IDA 0xf27264
#[doc(alias = "RobloxView::RenderJob::getMetricValue(std::string const&)const")]
pub fn jump_metric_value(job: &RenderJob, name: &str) -> f64 {
    // Import trampoline into `getMetricValue`.
    job.metric_value(name)
}

// 0xf27274 — j___ZNK10RobloxView9RenderJob9getMetricERKSs
// type: int __fastcall(RobloxView::RenderJob *this, const std::string *)
// IDA 0xf27274
#[doc(alias = "RobloxView::RenderJob::getMetric(std::string const&)const")]
pub fn jump_metric_text(job: &RenderJob, name: &str) -> String {
    // Import trampoline into `getMetric`.
    job.metric_text(name)
}

// 0x1d390 cluster — HomeViewController button ivar + PlaceLauncher lifecycle (IDA 0x1d390..0x289a8).

/// `RBXDidLeaveGameNotification` (IDA 0x24800..0x24830).
pub const DID_LEAVE_GAME_NOTIFICATION: &str = "RBXDidLeaveGameNotification";
/// `RBXStartLeaveGameNotification` (IDA 0x2483c..0x24860).
pub const START_LEAVE_GAME_NOTIFICATION: &str = "RBXStartLeaveGameNotification";
/// `RBXGameFinishedLoadingNotification` (IDA 0x2486c..0x24890).
pub const GAME_FINISHED_LOADING_NOTIFICATION: &str = "RBXGameFinishedLoadingNotification";

/// Reachability behind `-[PlaceLauncher prepareGame]` (IDA 0x24b52..0x24c8a).
/// Apple values: 0 = no service, 1 = WiFi, 2 = cellular (WWAN).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReachabilityStatus {
    /// `currentReachabilityStatus == 0`: no network.
    NoService,
    /// Cellular (`== 2`): gated by the `wifionly_preference`.
    Cellular,
    /// WiFi (`== 1`, default): proceeds.
    #[default]
    Wifi,
}

/// Join target bound into the `function0<void>` the `start*` leaves run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoinTarget {
    /// `joinLocalGame(placeId, ip, game)` (IDA 0x26c58..0x26c90).
    LocalIp(String),
    /// `loadLocalApp(path, game)` (IDA 0x270ea..0x27122).
    FilePath(String),
    /// `joinGamePlaceId(placeId, game, request)` (IDA 0x27738..0x2776a).
    PlaceId {
        request: i32,
    },
    /// `joinGamePlaceIdSolo(placeId, game)` (IDA 0x28c50).
    Solo,
    /// `joinGameWithJoinScript(script, game)` on the `InjectStartScript` thread (IDA 0x267ec..0x268de).
    Script(String),
    /// `joinGameTeleport(place, auth, script, controller, game)` on a
    /// `boost::thread` (IDA 0x29e34..0x29e40).
    Teleport {
        place: String,
        auth: String,
        script: String,
    },
}

/// Pending join bound by a `start*` leaf for `startGame:controller:preloadedGame:presentGameAutomatically:`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinRequest {
    pub place_id: i32,
    pub target: JoinTarget,
    pub game_id: u32,
    pub present_automatically: bool,
}
/// Outcome of `-[PlaceLauncher childAdded:]` (IDA 0x2b1bc): which signal branch
/// ran. Both player branches connect `playerLoaded:`; they differ only in the
/// log line (`PlayerChild` vs `PlayerNotChild`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildAddedOutcome {
    /// Nil rbxView (IDA 0x2b326..0x2b33a).
    NoView,
    /// Nil datamodel, missing Players service, or nil player (IDA 0x2b34e..0x2b3b8).
    NoPlayers,
    /// `player == child`: `PlayerChild` branch (IDA 0x2b3cc..0x2b466).
    PlayerConnected,
    /// `player != child`: `PlayerNotChild` branch (IDA 0x2b276..0x2b310).
    PlayerReconnected,
}

/// Minimal `HomeViewController` counterpart: the `_btnPlaceLauncher` ivar
/// (offset 220) behind the accessor pair plus the 0x1b3d0..0x1c958 lifecycle
/// state (init/dealloc, viewDidLoad chain, keyboard, signup/login, logout
/// alert, appear/segue, game-start leaves). UIKit objects live out of slice;
/// flags/counters record the observable flow so the `generated.rs` delegates
/// stay IDA-grounded without a host runtime.
#[derive(Debug, Default)]
pub struct HomeViewControllerState {
    btn_place_launcher: parking_lot::Mutex<ObjCId>,
    initialized: AtomicBool,
    webviews_preloaded: AtomicBool,
    signup_observer_registered: AtomicBool,
    deallocated: AtomicBool,
    released_ivar_count: AtomicU32,
    view_loaded: AtomicBool,
    debug_views_hidden: AtomicBool,
    tap_recognizer_installed: AtomicBool,
    tap_recognizer_enabled: AtomicBool,
    keyboard_observers_registered: AtomicU32,
    search_resigns: AtomicU32,
    labels_localized: AtomicU32,
    localized_keys: parking_lot::Mutex<Vec<&'static str>>,
    user_info_updates: AtomicU32,
    last_update_refresh: AtomicBool,
    avatar_highlighted: AtomicBool,
    unloaded: AtomicBool,
    unloaded_outlets: AtomicU32,
    signup_logins: AtomicU32,
    last_signup_credentials: parking_lot::Mutex<Option<(String, String)>>,
    logout_alerts_shown: AtomicU32,
    last_logout_alert: parking_lot::Mutex<Vec<&'static str>>,
    logouts: AtomicU32,
    logout_page_views: AtomicU32,
    button_view_alpha_steps: AtomicU32,
    foreground_captures: AtomicU32,
    presented_dismisses: AtomicU32,
    last_foreground_x: parking_lot::Mutex<f32>,
    last_background_x: parking_lot::Mutex<f32>,
    view_will_appears: AtomicU32,
    logged_in_state_shows: AtomicU32,
    logged_in_view_hidden: AtomicBool,
    not_logged_in_view_hidden: AtomicBool,
    logged_in_refresh_dispatches: AtomicU32,
    view_did_appears: AtomicU32,
    segue_after_load_pending: AtomicBool,
    segue_after_load_fired: AtomicU32,
    game_start_failures: AtomicU32,
    last_failure_alert: parking_lot::Mutex<Option<&'static str>>,
    game_start_successes: AtomicU32,
    version_text: parking_lot::Mutex<String>,
}

impl HomeViewControllerState {
    pub fn new() -> Self {
        Self::default()
    }

    // 0x1d390 — -[HomeViewController btnPlaceLauncher]
    // type: UIButton *__cdecl(HomeViewController *self, SEL)
    // IDA 0x1d390
    #[doc(alias = "-[HomeViewController btnPlaceLauncher]")]
    #[doc = "-[HomeViewController btnPlaceLauncher]"]
    pub fn btn_place_launcher(&self) -> ObjCId {
        // `return self->_btnPlaceLauncher` (IDA 0x1d390..0x1d39e).
        *self.btn_place_launcher.lock()
    }

    // 0x1d3a0 — -[HomeViewController setBtnPlaceLauncher:]
    // type: void __cdecl(HomeViewController *self, SEL, id)
    // IDA 0x1d3a0
    #[doc(alias = "-[HomeViewController setBtnPlaceLauncher:]")]
    #[doc = "-[HomeViewController setBtnPlaceLauncher:]"]
    pub fn set_btn_place_launcher(&self, btn: ObjCId) {
        // `objc_setProperty(self, a2, 220, a3, 0, 0)` (IDA 0x1d3a0..0x1d3bc):
        // retained ivar store at offset 220.
        *self.btn_place_launcher.lock() = btn;
    }

    // 0x1b3d0 — -[HomeViewController initWithCoder:]
    // type: HomeViewController *__cdecl(HomeViewController *self, SEL, id)
    // IDA 0x1b3d0
    #[doc(alias = "-[HomeViewController initWithCoder:]")]
    #[doc = "-[HomeViewController initWithCoder:]"]
    pub fn init_with_coder(&self) -> bool {
        // Super `initWithCoder:` first (IDA 0x1b3f8); on nil self the body is
        // skipped (IDA 0x1b3fc) — the host always hands back non-nil here.
        // `preloadDesignatedWebViews`, else `designatedWebviewsToHomePages`
        // (IDA 0x1b41a..0x1b442); then registers `handleSignupNotification:`
        // for `getSignupFinishedNotification` (IDA 0x1b462..0x1b4a4).
        self.webviews_preloaded.store(true, Ordering::SeqCst);
        self.signup_observer_registered.store(true, Ordering::SeqCst);
        self.initialized.store(true, Ordering::SeqCst);
        true
    }

    // 0x1b4b0 — -[HomeViewController dealloc]
    // type: void __cdecl(HomeViewController *self, SEL)
    // IDA 0x1b4b0
    #[doc(alias = "-[HomeViewController dealloc]")]
    #[doc = "-[HomeViewController dealloc]"]
    pub fn dealloc(&self) {
        // Releases the 30 retained outlets/ivars (`tapRecognizer` through
        // `_versionLabel`, IDA 0x1b4d4..0x1b730; Rust drops cover the stores)
        // then super `dealloc` (IDA 0x1b752).
        self.released_ivar_count.store(30, Ordering::SeqCst);
        self.deallocated.store(true, Ordering::SeqCst);
    }

    // 0x1b75c — -[HomeViewController viewDidLoad]
    // type: void __cdecl(HomeViewController *self, SEL)
    // IDA 0x1b75c
    #[doc(alias = "-[HomeViewController viewDidLoad]")]
    #[doc = "-[HomeViewController viewDidLoad]"]
    pub fn view_did_load(&self, bundle_version: &str) {
        // Super `viewDidLoad` (IDA 0x1b786); hides the debug leaves
        // (`_placeId`/`_portId`/`_ipId`/`_btnPlaceLauncher`/`_btnDebugSettings`,
        // IDA 0x1b7a8..0x1b800) and swaps the 568h blue frame on iPhone
        // (IDA 0x1b854..0x1b8ec, image load out of slice). Installs the
        // `dismissKeyboard` tap recognizer disabled (IDA 0x1b914..0x1b956),
        // localizes labels (IDA 0x1b98e), refreshes user info without a
        // player-info update (IDA 0x1b9a2), dispatches the search-url prefetch
        // off-main (IDA 0x1b9ac..0x1b9e4), registers the keyboard show/hide
        // observers (IDA 0x1ba04..0x1ba6a), and stamps `CFBundleVersion` onto
        // `_versionLabel` (IDA 0x1ba92..0x1bad2).
        self.debug_views_hidden.store(true, Ordering::SeqCst);
        self.tap_recognizer_installed.store(true, Ordering::SeqCst);
        self.tap_recognizer_enabled.store(false, Ordering::SeqCst);
        self.localize_and_style_labels();
        self.update_user_info_display(false);
        self.keyboard_observers_registered.store(2, Ordering::SeqCst);
        *self.version_text.lock() = bundle_version.to_owned();
        self.view_loaded.store(true, Ordering::SeqCst);
    }

    // 0x1bae4 — ___33-[HomeViewController viewDidLoad]_block_invoke
    // IDA 0x1bae4
    #[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke")]
    #[doc = "___33-[HomeViewController viewDidLoad]_block_invoke"]
    pub fn view_did_load_search_block(&self, search_url_len: usize) -> bool {
        // Background prefetch: only when `searchUrl.length > 0`
        // (IDA 0x1bb04..0x1bb14) does it hop back to the main queue for
        // `block_invoke_2` (IDA 0x1bb42..0x1bb5c).
        search_url_len > 0
    }

    // 0x1bb64 — ___33-[HomeViewController viewDidLoad]_block_invoke_2
    // type: id __fastcall(int)
    // IDA 0x1bb64
    #[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke_2")]
    #[doc = "___33-[HomeViewController viewDidLoad]_block_invoke_2"]
    pub fn view_did_load_search_apply(&self) {
        // `setHidden:NO` on the search field (ivar+284, IDA 0x1bb64..0x1bb76).
        self.debug_views_hidden.store(false, Ordering::SeqCst);
    }

    // 0x1bbb0 — -[HomeViewController keyboardDidShow:]
    // type: void __cdecl(HomeViewController *self, SEL, id)
    // IDA 0x1bbb0
    #[doc(alias = "-[HomeViewController keyboardDidShow:]")]
    #[doc = "-[HomeViewController keyboardDidShow:]"]
    pub fn keyboard_did_show(&self) {
        // `tapRecognizer.enabled = YES` (IDA 0x1bbcc).
        self.tap_recognizer_enabled.store(true, Ordering::SeqCst);
    }

    // 0x1bbd0 — -[HomeViewController keyboardDidHide:]
    // type: void __cdecl(HomeViewController *self, SEL, id)
    // IDA 0x1bbd0
    #[doc(alias = "-[HomeViewController keyboardDidHide:]")]
    #[doc = "-[HomeViewController keyboardDidHide:]"]
    pub fn keyboard_did_hide(&self) {
        // `tapRecognizer.enabled = NO` (IDA 0x1bbec).
        self.tap_recognizer_enabled.store(false, Ordering::SeqCst);
    }

    // 0x1bbf0 — -[HomeViewController dismissKeyboard]
    // type: void __cdecl(HomeViewController *self, SEL)
    // IDA 0x1bbf0
    #[doc(alias = "-[HomeViewController dismissKeyboard]")]
    #[doc = "-[HomeViewController dismissKeyboard]"]
    pub fn dismiss_keyboard(&self) {
        // `[_searchTextField resignFirstResponder]` (IDA 0x1bc0a).
        self.search_resigns.fetch_add(1, Ordering::SeqCst);
    }

    /// `NSBundle` keys `localizeAndStyleLabels` stamps (IDA 0x1bc48..0x1bf08).
    pub const LOCALIZED_LABEL_KEYS: [&'static str; 11] = [
        "GameWord",
        "CatalogWord",
        "InventoryWord",
        "BuildersClubWord",
        "ProfileWord",
        "MessagesWord",
        "CommunityWord",
        "WelcomeToRoblox",
        "YouAreCurrentlyLoggedInAs",
        "SignupButton",
        "LoginButton",
    ];

    // 0x1bc10 — -[HomeViewController localizeAndStyleLabels]
    // type: void __cdecl(HomeViewController *self, SEL)
    // IDA 0x1bc10
    #[doc(alias = "-[HomeViewController localizeAndStyleLabels]")]
    #[doc = "-[HomeViewController localizeAndStyleLabels]"]
    pub fn localize_and_style_labels(&self) {
        // Eleven `localizedStringForKey:value:table:` stamps onto the game /
        // catalog / inventory / builders-club / profile / messages /
        // community labels, the two text views, and the signup/login button
        // labels (IDA 0x1bc48..0x1bf08); the bundle lookup lives out of slice.
        *self.localized_keys.lock() = Self::LOCALIZED_LABEL_KEYS.to_vec();
        self.labels_localized.store(Self::LOCALIZED_LABEL_KEYS.len() as u32, Ordering::SeqCst);
    }

    // 0x1bf0c — -[HomeViewController updateUserInfoDisplay:]
    // type: void __cdecl(HomeViewController *self, SEL, bool)
    // IDA 0x1bf0c
    #[doc(alias = "-[HomeViewController updateUserInfoDisplay:]")]
    #[doc = "-[HomeViewController updateUserInfoDisplay:]"]
    pub fn update_user_info_display(&self, refresh: bool) {
        // With `a3` set, `UpdatePlayerInfo` on `CurrentPlayer` first
        // (IDA 0x1bf18..0x1bf42). Then `": " + Robux` / `": " + Tix` onto the
        // labels (IDA 0x1bf70..0x1c000), the username when non-nil
        // (IDA 0x1c008..0x1c044), and the thumbnail URL synchronously into
        // `_imgAvatar` (IDA 0x1c04c..0x1c0f2); `highlighted` is the inverse of
        // having a thumbnail (IDA 0x1c0fa..0x1c130). The web fetch lives out
        // of slice, so the avatar-present branch is recorded.
        self.last_update_refresh.store(refresh, Ordering::SeqCst);
        self.avatar_highlighted.store(false, Ordering::SeqCst);
        self.user_info_updates.fetch_add(1, Ordering::SeqCst);
    }

    // 0x1c134 — -[HomeViewController viewDidUnload]
    // type: void __cdecl(HomeViewController *self, SEL)
    // IDA 0x1c134
    #[doc(alias = "-[HomeViewController viewDidUnload]")]
    #[doc = "-[HomeViewController viewDidUnload]"]
    pub fn view_did_unload(&self) {
        // Nils the 18 outlet setters (`setPlaceId:` through `setVersionLabel:`,
        // IDA 0x1c14c..0x1c290; signup/login labels are nilled twice,
        // IDA 0x1c22a..0x1c27c) then super `viewDidUnload` (IDA 0x1c2b2).
        // Rust drops cover the stores.
        self.unloaded_outlets.store(18, Ordering::SeqCst);
        self.unloaded.store(true, Ordering::SeqCst);
    }

    // 0x1c2bc — -[HomeViewController handleSignupNotification:]
    // type: void __cdecl(HomeViewController *self, SEL, id)
    // IDA 0x1c2bc
    #[doc(alias = "-[HomeViewController handleSignupNotification:]")]
    #[doc = "-[HomeViewController handleSignupNotification:]"]
    pub fn handle_signup_notification(&self, username: &str, password: &str) {
        // Retains the `username`/`password` pair from the notification
        // `userInfo` (IDA 0x1c2d8..0x1c32c), drives
        // `doLoginWithUsername:password:` (IDA 0x1c348..0x1c35c), then
        // `showCorrectLoggedInState` (IDA 0x1c376).
        *self.last_signup_credentials.lock() =
            Some((username.to_owned(), password.to_owned()));
        self.signup_logins.fetch_add(1, Ordering::SeqCst);
        self.logged_in_state_shows.fetch_add(1, Ordering::SeqCst);
    }

    /// Alert titles `logoutTouchUp:` builds (IDA 0x1c3c2..0x1c458).
    pub const LOGOUT_ALERT_KEYS: [&'static str; 4] = [
        "RobloxWord",
        "LogoutConfirmation",
        "CancelWord",
        "LogoutWord",
    ];

    // 0x1c37c — -[HomeViewController logoutTouchUp:]
    // type: void __cdecl(HomeViewController *self, SEL, id)
    // IDA 0x1c37c
    #[doc(alias = "-[HomeViewController logoutTouchUp:]")]
    #[doc = "-[HomeViewController logoutTouchUp:]"]
    pub fn logout_touch_up(&self) {
        // `UIAlertView` with title `RobloxWord`, message
        // `LogoutConfirmation`, cancel `CancelWord`, other `LogoutWord`,
        // delegate self (IDA 0x1c3a4..0x1c47e); `show` then `release`
        // (IDA 0x1c48e..0x1c4aa).
        *self.last_logout_alert.lock() = Self::LOGOUT_ALERT_KEYS.to_vec();
        self.logout_alerts_shown.fetch_add(1, Ordering::SeqCst);
    }

    // 0x1c4b0 — -[HomeViewController alertView:didDismissWithButtonIndex:]
    // type: void __cdecl(HomeViewController *self, SEL, id, int)
    // IDA 0x1c4b0
    #[doc(alias = "-[HomeViewController alertView:didDismissWithButtonIndex:]")]
    #[doc = "-[HomeViewController alertView:didDismissWithButtonIndex:]"]
    pub fn alert_view_did_dismiss(&self, button_index: i32) -> bool {
        // Only button 1 (Logout) acts (IDA 0x1c4be): `doLogout` +
        // `+[UserInfo logout]` (IDA 0x1c4d8..0x1c504), the 0.3s
        // `animateWithDuration:animations:completion:` pair
        // (IDA 0x1c546..0x1c58e), and the `Logout/Success` page track
        // (IDA 0x1c5b4). Cancel (0) is a no-op.
        if button_index != 1 {
            return false;
        }
        self.logouts.fetch_add(1, Ordering::SeqCst);
        self.button_view_alpha_steps.fetch_add(1, Ordering::SeqCst);
        self.logout_page_views.fetch_add(1, Ordering::SeqCst);
        true
    }

    // 0x1c5c8 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke
    // IDA 0x1c5c8
    #[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke")]
    #[doc = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke"]
    pub fn alert_animation_step(&self) {
        // `buttonView.alpha = 0` (IDA 0x1c5da).
        self.button_view_alpha_steps.fetch_add(1, Ordering::SeqCst);
    }

    // 0x1c608 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227
    // IDA 0x1c608
    #[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227")]
    #[doc = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227"]
    pub fn alert_completion(&self, presented: bool, animating: bool, foreground_x: f32, background_x: f32) {
        // With a presenting controller and a live (`!v3[169]`) page animator
        // (IDA 0x1c626..0x1c63e), snapshots the foreground/background
        // presentation-layer X into the presenter (IDA 0x1c650..0x1c712;
        // zeroed when no presentation layer, IDA 0x1c69e/0x1c6fc), then
        // `dismissViewControllerAnimated:NO` (IDA 0x1c732).
        if presented && !animating {
            *self.last_foreground_x.lock() = foreground_x;
            *self.last_background_x.lock() = background_x;
            self.foreground_captures.fetch_add(1, Ordering::SeqCst);
        }
        self.presented_dismisses.fetch_add(1, Ordering::SeqCst);
    }

    // 0x1c748 — -[HomeViewController viewWillAppear:]
    // type: void __cdecl(HomeViewController *self, SEL, char)
    // IDA 0x1c748
    #[doc(alias = "-[HomeViewController viewWillAppear:]")]
    #[doc = "-[HomeViewController viewWillAppear:]"]
    pub fn view_will_appear(&self, animated: bool) {
        // Super `viewWillAppear:` (IDA 0x1c76e) then
        // `showCorrectLoggedInState` (IDA 0x1c780); `animated` only reaches
        // the super call.
        let _ = animated;
        self.view_will_appears.fetch_add(1, Ordering::SeqCst);
        self.logged_in_state_shows.fetch_add(1, Ordering::SeqCst);
    }

    // 0x1c788 — -[HomeViewController showCorrectLoggedInState]
    // type: void __cdecl(HomeViewController *self, SEL)
    // IDA 0x1c788
    #[doc(alias = "-[HomeViewController showCorrectLoggedInState]")]
    #[doc = "-[HomeViewController showCorrectLoggedInState]"]
    pub fn show_correct_logged_in_state(&self, logged_in: bool) {
        // `userLoggedIn == 1` shows the logged-in view and hides the logged-out
        // one (IDA 0x1c7d2..0x1c7f8), else the reverse (IDA 0x1c7fe..0x1c814);
        // either way `updateUserInfoDisplay:YES` is dispatched off-main
        // (IDA 0x1c820..0x1c858).
        self.not_logged_in_view_hidden.store(logged_in, Ordering::SeqCst);
        self.logged_in_view_hidden.store(!logged_in, Ordering::SeqCst);
        self.logged_in_state_shows.fetch_add(1, Ordering::SeqCst);
        self.logged_in_refresh_dispatches.fetch_add(1, Ordering::SeqCst);
    }

    // 0x1c860 — ___46-[HomeViewController showCorrectLoggedInState]_block_invoke
    // type: id __fastcall(int)
    // IDA 0x1c860
    #[doc(alias = "___46-[HomeViewController showCorrectLoggedInState]_block_invoke")]
    #[doc = "___46-[HomeViewController showCorrectLoggedInState]_block_invoke"]
    pub fn logged_in_state_refresh_block(&self) {
        // `updateUserInfoDisplay:YES` (IDA 0x1c860..0x1c870).
        self.update_user_info_display(true);
    }

    // 0x1c888 — -[HomeViewController viewDidAppear:]
    // type: void __cdecl(HomeViewController *self, SEL, char)
    // IDA 0x1c888
    #[doc(alias = "-[HomeViewController viewDidAppear:]")]
    #[doc = "-[HomeViewController viewDidAppear:]"]
    pub fn view_did_appear(&self, animated: bool) {
        // Super `viewDidAppear:` (IDA 0x1c8ae); when the
        // `viewMustSegueAfterLoad` flag is set (IDA 0x1c8c0), clears it
        // (IDA 0x1c8cc) and performs `sequeToWeb:` (IDA 0x1c8e0).
        let _ = animated;
        self.view_did_appears.fetch_add(1, Ordering::SeqCst);
        if self.segue_after_load_pending.swap(false, Ordering::SeqCst) {
            self.segue_after_load_fired.fetch_add(1, Ordering::SeqCst);
        }
    }

    /// Stages the `viewMustSegueAfterLoad` flag `viewDidAppear:` consumes.
    pub fn set_segue_after_load_pending(&self, pending: bool) {
        self.segue_after_load_pending.store(pending, Ordering::SeqCst);
    }

    // 0x1c8e8 — -[HomeViewController handleStartGameFailure]
    // type: void __cdecl(HomeViewController *self, SEL)
    // IDA 0x1c8e8
    #[doc(alias = "-[HomeViewController handleStartGameFailure]")]
    #[doc = "-[HomeViewController handleStartGameFailure]"]
    pub fn handle_start_game_failure(&self) {
        // `RobloxAlertWithMessage:` with `GeneralGameStartError`
        // (IDA 0x1c912..0x1c954); the alert presentation lives out of slice.
        *self.last_failure_alert.lock() = Some("GeneralGameStartError");
        self.game_start_failures.fetch_add(1, Ordering::SeqCst);
    }

    // 0x1c958 — -[HomeViewController handleStartGameSuccess]
    // type: void __cdecl(HomeViewController *self, SEL)
    // IDA 0x1c958
    #[doc(alias = "-[HomeViewController handleStartGameSuccess]")]
    #[doc = "-[HomeViewController handleStartGameSuccess]"]
    pub fn handle_start_game_success(&self) {
        // Empty body (IDA 0x1c958): no-op recorded for call tracking.
        self.game_start_successes.fetch_add(1, Ordering::SeqCst);
    }
}

/// Minimal `PlaceLauncher` counterpart (IDA 0x246d8..0x289a8): the `rbxView`
/// ivar, play-state flags, notification names, join bindings, and counters for
/// the UIKit/render-system steps that live out of slice. `SharedPtr` is
/// `rbx_core::SharedPtr` (`Arc`), never `boost::shared_ptr`; `boost::bind`
/// targets become [`JoinRequest`] rows, the `InjectStartScript`
/// `boost::thread` a detached `std::thread`.
#[derive(Debug, Default)]
pub struct PlaceLauncher {
    view: parking_lot::Mutex<Option<SharedPtr<RobloxView>>>,
    has_received_memory_warning: AtomicBool,
    is_currently_playing_game: AtomicBool,
    last_place_id: parking_lot::Mutex<i32>,
    teleporter_callback_set: AtomicBool,
    did_leave_game_notification: parking_lot::Mutex<String>,
    start_leave_game_notification: parking_lot::Mutex<String>,
    game_finished_loading_notification: parking_lot::Mutex<String>,
    last_non_game_controller: parking_lot::Mutex<Option<ObjCId>>,
    ogre_view_controller_present: AtomicBool,
    reachability: parking_lot::Mutex<ReachabilityStatus>,
    wifi_only: AtomicBool,
    warnings_preference: AtomicBool,
    max_place_parts: parking_lot::Mutex<i32>,
    current_part_count: parking_lot::Mutex<i32>,
    prepare_calls: AtomicU32,
    check_part_dispatches: AtomicU32,
    part_warnings: AtomicU32,
    game_finished_loading_posts: AtomicU32,
    deferred_finish_loading: AtomicU32,
    failure_forwards: AtomicU32,
    datamodel_connections: AtomicU32,
    open_url_connections: AtomicU32,
    child_added_connections: AtomicU32,
    login_prompt_connections: AtomicU32,
    main_dispatches: AtomicU32,
    control_view_tasks: AtomicU32,
    inject_dispatches: AtomicU32,
    next_game_id: AtomicU32,
    last_game_secure: AtomicBool,
    last_game_is_app: AtomicBool,
    idle_timer_disabled: AtomicBool,
    game_start_threads: AtomicU32,
    is_leaving_game: AtomicBool,
    leave_game_calls: AtomicU32,
    leave_shutdown_calls: AtomicU32,
    shutdown_completions: AtomicU32,
    start_leave_posts: AtomicU32,
    did_leave_posts: AtomicU32,
    bg_task: parking_lot::Mutex<Option<u32>>,
    next_bg_task: AtomicU32,
    game_state: parking_lot::Mutex<String>,
    session_reports: parking_lot::Mutex<Vec<(u32, i32)>>,
    page_views: parking_lot::Mutex<Vec<String>>,
    child_connection_connected: AtomicBool,
    player_connection_connected: AtomicBool,
    close_child_calls: AtomicU32,
    free_memory_checker_running: AtomicBool,
    memory_warning_shutdowns: AtomicU32,
    memory_warning_ignores: AtomicU32,
    teleport_dispatches: AtomicU32,
    teleport_completions: AtomicU32,
    teleport_animation_steps: AtomicU32,
    last_teleport: parking_lot::Mutex<Option<(String, String, String)>>,
    start_solo_calls: AtomicU32,
    start_join_script_calls: AtomicU32,
    join_requests: parking_lot::Mutex<Vec<JoinRequest>>,
    alerts: parking_lot::Mutex<Vec<String>>,
    analytics_events: parking_lot::Mutex<Vec<(String, String, String)>>,
}
pub fn shared_place_launcher() -> &'static PlaceLauncher {
    static LAUNCHER: std::sync::LazyLock<PlaceLauncher> =
        std::sync::LazyLock::new(PlaceLauncher::new);
    &LAUNCHER
}

// 0x24974 — +[PlaceLauncher sharedInstance]
// type: id __cdecl(id, SEL)
// IDA 0x24974
#[doc(alias = "+[PlaceLauncher sharedInstance]")]
pub fn shared_place_launcher_id() -> ObjCId {
    // `dispatch_once(&dword_130C440, block)` then `return dword_130C444`
    // (IDA 0x24974..0x249c2); the singleton address is the identity.
    shared_place_launcher() as *const PlaceLauncher as ObjCId
}

// 0x249d0 — ___31+[PlaceLauncher sharedInstance]_block_invoke
// type: id __fastcall(int)
// IDA 0x249d0
#[doc(alias = "___31+[PlaceLauncher sharedInstance]_block_invoke")]
pub fn shared_place_launcher_block() -> ObjCId {
    // `alloc` + `init` stored to `dword_130C444` (IDA 0x249d0..0x24a02); the
    // process-wide singleton is the store.
    shared_place_launcher().init_launcher();
    shared_place_launcher_id()
}

// 0x2613c — ___43-[PlaceLauncher setupDatamodelConnections:]_block_invoke
// type: void __cdecl(id)
// IDA 0x2613c
#[doc(alias = "___43-[PlaceLauncher setupDatamodelConnections:]_block_invoke")]
pub fn setup_datamodel_connections_block(slot: &PlaceLauncherViewSlot) {
    // `-[RobloxMemoryManager startFreeMemoryChecker]` (IDA 0x2613c..0x2616c).
    slot.set_free_memory_checker_running(true);
}

impl PlaceLauncher {
    pub fn new() -> Self {
        Self::default()
    }

    // 0x246d8 — -[PlaceLauncher init]
    // type: PlaceLauncher *__cdecl(PlaceLauncher *self, SEL)
    // IDA 0x246d8
    #[doc(alias = "-[PlaceLauncher init]")]
    #[doc = "-[PlaceLauncher init]"]
    pub fn init_launcher(&self) {
        // `rbxView = 0; hasReceivedMemoryWarning = 0; isCurrentlyPlayingGame = 0;
        // lastPlaceId = 0` (IDA 0x24760..0x24780).
        self.view.lock().take();
        self.has_received_memory_warning.store(false, Ordering::SeqCst);
        self.is_currently_playing_game.store(false, Ordering::SeqCst);
        *self.last_place_id.lock() = 0;
        // Fresh `Teleporter` + `TeleportService::SetCallback` (IDA 0x2478e..0x247dc).
        self.teleporter_callback_set.store(true, Ordering::SeqCst);
        // The three `initWithString:` notification names (IDA 0x24800..0x24890).
        *self.did_leave_game_notification.lock() = DID_LEAVE_GAME_NOTIFICATION.to_owned();
        *self.start_leave_game_notification.lock() = START_LEAVE_GAME_NOTIFICATION.to_owned();
        *self.game_finished_loading_notification.lock() =
            GAME_FINISHED_LOADING_NOTIFICATION.to_owned();
    }

    // 0x248dc — -[PlaceLauncher dealloc]
    // type: void __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x248dc
    #[doc(alias = "-[PlaceLauncher dealloc]")]
    #[doc = "-[PlaceLauncher dealloc]"]
    pub fn dealloc_launcher(&self) {
        // `TeleportService::SetCallback(0)` + teleporter release (IDA 0x248e8..0x24902).
        self.teleporter_callback_set.store(false, Ordering::SeqCst);
        // The three `release`s (IDA 0x24920..0x24948); `super dealloc` is out of slice.
        self.did_leave_game_notification.lock().clear();
        self.start_leave_game_notification.lock().clear();
        self.game_finished_loading_notification.lock().clear();
    }

    // 0x24a18 — -[PlaceLauncher getIsCurrentlyPlayingGame]
    // type: char __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x24a18
    #[doc(alias = "-[PlaceLauncher getIsCurrentlyPlayingGame]")]
    #[doc = "-[PlaceLauncher getIsCurrentlyPlayingGame]"]
    pub fn is_currently_playing_game(&self) -> bool {
        // `return self->isCurrentlyPlayingGame` (IDA 0x24a18..0x24a26).
        self.is_currently_playing_game.load(Ordering::SeqCst)
    }

    // 0x24a28 — -[PlaceLauncher getDidLeaveGameNotification]
    // IDA 0x24a28
    #[doc(alias = "-[PlaceLauncher getDidLeaveGameNotification]")]
    #[doc = "-[PlaceLauncher getDidLeaveGameNotification]"]
    pub fn did_leave_game_notification(&self) -> String {
        self.did_leave_game_notification.lock().clone()
    }

    // 0x24a38 — -[PlaceLauncher getStartLeaveGameNotification]
    // IDA 0x24a38
    #[doc(alias = "-[PlaceLauncher getStartLeaveGameNotification]")]
    #[doc = "-[PlaceLauncher getStartLeaveGameNotification]"]
    pub fn start_leave_game_notification(&self) -> String {
        self.start_leave_game_notification.lock().clone()
    }

    // 0x24a48 — -[PlaceLauncher getGameFinishedLoadingNotification]
    // IDA 0x24a48
    #[doc(alias = "-[PlaceLauncher getGameFinishedLoadingNotification]")]
    #[doc = "-[PlaceLauncher getGameFinishedLoadingNotification]"]
    pub fn game_finished_loading_notification(&self) -> String {
        self.game_finished_loading_notification.lock().clone()
    }

    // 0x24a58 — -[PlaceLauncher handleStartGameFailure]
    // type: void __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x24a58
    #[doc(alias = "-[PlaceLauncher handleStartGameFailure]")]
    #[doc = "-[PlaceLauncher handleStartGameFailure]"]
    pub fn handle_start_game_failure(&self) {
        // Forward to the last-non-game controller when present (IDA 0x24a76..0x24a98).
        if self.last_non_game_controller.lock().is_some() {
            self.failure_forwards.fetch_add(1, Ordering::SeqCst);
        }
        // `self->isCurrentlyPlayingGame = 0` (IDA 0x24aaa).
        self.is_currently_playing_game.store(false, Ordering::SeqCst);
    }

    // 0x24ab0 — -[PlaceLauncher prepareGame]
    // type: bool __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x24ab0
    #[doc(alias = "-[PlaceLauncher prepareGame]")]
    #[doc = "-[PlaceLauncher prepareGame]"]
    pub fn prepare_game(&self) -> bool {
        self.prepare_calls.fetch_add(1, Ordering::SeqCst);
        // Asset folder + `Game::globalInit` + teleport base URL (IDA 0x24aea..0x24b36).
        // Reachability gate (IDA 0x24b52..0x24c8a).
        match *self.reachability.lock() {
            ReachabilityStatus::NoService => {
                // `printf("PlaceLauncher: No Network Connection available")` then
                // the `ConnectionError` alert (IDA 0x24c2e..0x24cb4).
                self.alerts.lock().push("ConnectionError".to_owned());
                return false;
            }
            ReachabilityStatus::Cellular if self.wifi_only.load(Ordering::SeqCst) => {
                // The `wifionly_preference` `WiFiOnlyError` alert (IDA 0x24bb8..0x24cb4).
                self.alerts.lock().push("WiFiOnlyError".to_owned());
                return false;
            }
            ReachabilityStatus::Cellular | ReachabilityStatus::Wifi => {}
        }
        // `DataModel::hash = "ios"`, settings `loadState`, scheduler
        // thread-pool config (IDA 0x24ccc..0x24eac); applied out of slice.
        true
    }

    // 0x25080 — -[PlaceLauncher setLastPlaceId:]
    // type: void __cdecl(PlaceLauncher *self, SEL, int)
    // IDA 0x25080
    #[doc(alias = "-[PlaceLauncher setLastPlaceId:]")]
    #[doc = "-[PlaceLauncher setLastPlaceId:]"]
    pub fn set_last_place_id(&self, place_id: i32) {
        // `self->lastPlaceId = a3` (IDA 0x25080..0x2508c).
        *self.last_place_id.lock() = place_id;
    }

    // 0x25090 — -[PlaceLauncher checkPlacePartCount]
    // type: void __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x25090
    #[doc(alias = "-[PlaceLauncher checkPlacePartCount]")]
    #[doc = "-[PlaceLauncher checkPlacePartCount]"]
    pub fn check_place_part_count(&self) {
        // `warnings_preference` gate (IDA 0x250b0..0x250da).
        if !self.warnings_preference.load(Ordering::SeqCst) {
            return;
        }
        // `dispatch_async(global_queue, block)` (IDA 0x250ec..0x25124); the block
        // runs inline here.
        self.check_part_dispatches.fetch_add(1, Ordering::SeqCst);
        self.check_place_part_count_block();
    }

    // 0x2512c — ___36-[PlaceLauncher checkPlacePartCount]_block_invoke
    // IDA 0x2512c
    #[doc(alias = "___36-[PlaceLauncher checkPlacePartCount]_block_invoke")]
    #[doc = "___36-[PlaceLauncher checkPlacePartCount]_block_invoke"]
    pub fn check_place_part_count_block(&self) -> bool {
        // `maxParts` from the iOS settings service (IDA 0x25178..0x25198).
        let max = *self.max_place_parts.lock();
        if max < 1 {
            return false;
        }
        // rbxView -> game -> datamodel -> workspace nil chain (IDA 0x251ae..0x25222).
        if self.view.lock().is_none() {
            return false;
        }
        let parts = *self.current_part_count.lock();
        if parts <= max {
            return false;
        }
        // Analytics `PlayErrors/TooManyParts` labeled with lastPlaceId
        // (IDA 0x252fc..0x25362).
        let place = *self.last_place_id.lock();
        self.analytics_events.lock().push((
            "PlayErrors".to_owned(),
            "TooManyParts".to_owned(),
            place.to_string(),
        ));
        // `RobloxAlertWithMessage` for `WarnTooManyParts` (IDA 0x25384).
        self.alerts.lock().push(format!("WarnTooManyParts:{parts}>{max}"));
        self.part_warnings.fetch_add(1, Ordering::SeqCst);
        true
    }

    // 0x253e0 — -[PlaceLauncher placeDidFinishLoading]
    // type: void __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x253e0
    #[doc(alias = "-[PlaceLauncher placeDidFinishLoading]")]
    #[doc = "-[PlaceLauncher placeDidFinishLoading]"]
    pub fn place_did_finish_loading(&self) {
        // Post `gameFinishedLoadingNotification` with nil userInfo (IDA 0x25400..0x25424).
        self.game_finished_loading_posts.fetch_add(1, Ordering::SeqCst);
        // `checkPlacePartCount` (IDA 0x2543c).
        self.check_place_part_count();
    }

    // 0x25498 — -[PlaceLauncher finishGameSetup:gameViewController:]
    // type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Game>, id)
    // IDA 0x25498
    #[doc(alias = "-[PlaceLauncher finishGameSetup:gameViewController:]")]
    #[doc = "-[PlaceLauncher finishGameSetup:gameViewController:]"]
    pub fn finish_game_setup(
        &self,
        game: &SharedPtr<GameHandle>,
        game_view_controller: ObjCId,
        screen_w: u32,
        screen_h: u32,
        datamodel_ready: bool,
        overlay_ready: bool,
    ) {
        // `RobloxView::create_view(game, screenW, screenH, window, view, ...)`
        // from `mainScreen.bounds` via `objc_msgSend_stret` (IDA 0x254e8..0x256d2);
        // the window/view id strings reduce to the controller tag here.
        let tag = game_view_controller.to_string();
        let view = create_view(game.id, screen_w, screen_h, &tag, &tag, &tag);
        *self.view.lock() = Some(view);
        // A loaded datamodel calls through; otherwise `placeDidFinishLoading`
        // is deferred into the datamodel signal (IDA 0x25730..0x257e8).
        if datamodel_ready {
            self.place_did_finish_loading();
        } else {
            self.deferred_finish_loading.fetch_add(1, Ordering::SeqCst);
        }
        // `setupDatamodelConnections:` for the datamodel and, when present, the
        // overlay datamodel (IDA 0x257f2..0x258a4).
        self.setup_datamodel_connections(true, true);
        if overlay_ready {
            self.setup_datamodel_connections(true, false);
        }
    }

    // 0x25e00 — -[PlaceLauncher setupDatamodelConnections:]
    // type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::DataModel>)
    // IDA 0x25e00
    #[doc(alias = "-[PlaceLauncher setupDatamodelConnections:]")]
    #[doc = "-[PlaceLauncher setupDatamodelConnections:]"]
    pub fn setup_datamodel_connections(&self, has_gui_service: bool, has_login_service: bool) {
        // `GuiService::openUrlWindow` connect (IDA 0x25e2a..0x25eea).
        if has_gui_service {
            self.open_url_connections.fetch_add(1, Ordering::SeqCst);
        }
        // `dispatch_async(main, ...)` free-memory-checker kick (IDA 0x25f04).
        self.main_dispatches.fetch_add(1, Ordering::SeqCst);
        // `Players::childAdded:` connect + connection store (IDA 0x25f18..0x25fcc).
        self.child_added_connections.fetch_add(1, Ordering::SeqCst);
        // `LoginService::handlePromptLoginSignal` connect (IDA 0x25fd2..0x2606c).
        if has_login_service {
            self.login_prompt_connections.fetch_add(1, Ordering::SeqCst);
        }
        self.datamodel_connections.fetch_add(1, Ordering::SeqCst);
    }

    // 0x26170 — -[PlaceLauncher setLastNonGameController:]
    // type: void __cdecl(PlaceLauncher *self, SEL, id)
    // IDA 0x26170
    #[doc(alias = "-[PlaceLauncher setLastNonGameController:]")]
    #[doc = "-[PlaceLauncher setLastNonGameController:]"]
    pub fn set_last_non_game_controller(&self, controller: Option<ObjCId>) -> bool {
        // Forward to `MainViewController` (IDA 0x26190..0x261a2); owned here.
        *self.last_non_game_controller.lock() = controller;
        // `if (a3 && ![self prepareGame]) [self handleStartGameFailure]` (IDA 0x261a8..0x261d4).
        if controller.is_some() && !self.prepare_game() {
            self.handle_start_game_failure();
            return false;
        }
        true
    }

    // 0x261d8 — -[PlaceLauncher createGame:presentGameAutomatically:]
    // type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Game>, char)
    // IDA 0x261d8
    #[doc(alias = "-[PlaceLauncher createGame:presentGameAutomatically:]")]
    #[doc = "-[PlaceLauncher createGame:presentGameAutomatically:]"]
    pub fn create_game(&self, game: &SharedPtr<GameHandle>, _present: bool) {
        // `self->hasReceivedMemoryWarning = 0` (IDA 0x26212); the
        // `presentGameAutomatically` flag is unused below (kept for selector shape).
        self.has_received_memory_warning.store(false, Ordering::SeqCst);
        // `-[PlaceLauncher deleteRobloxView]` (IDA 0x26216; see 0x25440).
        self.view.lock().take();
        // A fresh `GameViewController` becomes the ogre controller while a
        // last-non-game controller exists (IDA 0x26236..0x262d0).
        if self.last_non_game_controller.lock().is_some() {
            self.ogre_view_controller_present.store(true, Ordering::SeqCst);
            // `finishGameSetup:gameViewController:` (IDA 0x262d8..0x2630a).
            self.finish_game_setup(game, NIL_ID, 0, 0, false, false);
            // `DataModel::submitTask(..., initControlView, ...)` (IDA 0x26320..0x2638a).
            self.control_view_tasks.fetch_add(1, Ordering::SeqCst);
        }
    }

    // 0x26520 — -[PlaceLauncher setupGame:isApp:]
    // type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char)
    // IDA 0x26520
    #[doc(alias = "-[PlaceLauncher setupGame:isApp:]")]
    #[doc = "-[PlaceLauncher setupGame:isApp:]"]
    pub fn setup_game(&self, controller: ObjCId, is_app: bool) -> Option<SharedPtr<GameHandle>> {
        // Forwards to `setupGame:unsecuredGame:isApp:` with `unsecured = 0`
        // (IDA 0x26520..0x26544); a nil self yields a null game (IDA 0x2654c..0x2654e),
        // which `&self` already excludes.
        self.setup_game_unsecured(controller, false, is_app)
    }

    // 0x26558 — -[PlaceLauncher setupGame:unsecuredGame:isApp:]
    // type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char, char)
    // IDA 0x26558
    #[doc(alias = "-[PlaceLauncher setupGame:unsecuredGame:isApp:]")]
    #[doc = "-[PlaceLauncher setupGame:unsecuredGame:isApp:]"]
    pub fn setup_game_unsecured(
        &self,
        controller: ObjCId,
        unsecured: bool,
        is_app: bool,
    ) -> Option<SharedPtr<GameHandle>> {
        // `if (self->isCurrentlyPlayingGame) return null` (IDA 0x26594..0x265bc).
        if self.is_currently_playing_game.load(Ordering::SeqCst) {
            return None;
        }
        // `ClientAppSettings::Initialize` + `FetchClientSettingsData("iOSAppSettings", ...)`
        // + forced iOS-settings read (IDA 0x265ca..0x26610); settings live out of slice.
        // `-[UIApplication setIdleTimerDisabled:1]` (IDA 0x2662e..0x26642).
        self.idle_timer_disabled.store(true, Ordering::SeqCst);
        // `self->isCurrentlyPlayingGame = 1` (IDA 0x26650).
        self.is_currently_playing_game.store(true, Ordering::SeqCst);
        // `-[PlaceLauncher setLastNonGameController:]` (IDA 0x2665c).
        self.set_last_non_game_controller(Some(controller));
        // `new SecurePlayerGame` vs `new UnsecuredStudioGame(baseURL, isApp)`
        // (IDA 0x26668..0x266ec); the kind bit is what this slice observes.
        let id = self.next_game_id.fetch_add(1, Ordering::SeqCst) + 1;
        self.last_game_secure.store(!unsecured, Ordering::SeqCst);
        self.last_game_is_app.store(is_app, Ordering::SeqCst);
        Some(wrap_game(id))
    }

    // 0x26784 — -[PlaceLauncher setupPreloadedGameWithNonGameController:unsecuredGame:isApp:]
    // IDA 0x26784
    #[doc(alias = "-[PlaceLauncher setupPreloadedGameWithNonGameController:unsecuredGame:isApp:]")]
    #[doc = "-[PlaceLauncher setupPreloadedGameWithNonGameController:unsecuredGame:isApp:]"]
    pub fn setup_preloaded_game_unsecured(
        &self,
        controller: ObjCId,
        unsecured: bool,
        is_app: bool,
    ) -> Option<SharedPtr<GameHandle>> {
        // Forwards to `setupGame:unsecuredGame:isApp:` (IDA 0x26784..0x267a8).
        self.setup_game_unsecured(controller, unsecured, is_app)
    }

    // 0x267bc — -[PlaceLauncher setupPreloadedGameWithNonGameController:isApp:]
    // IDA 0x267bc
    #[doc(alias = "-[PlaceLauncher setupPreloadedGameWithNonGameController:isApp:]")]
    #[doc = "-[PlaceLauncher setupPreloadedGameWithNonGameController:isApp:]"]
    pub fn setup_preloaded_game(
        &self,
        controller: ObjCId,
        is_app: bool,
    ) -> Option<SharedPtr<GameHandle>> {
        // Forwards to `setupGame:isApp:` (IDA 0x267bc..0x267d8).
        self.setup_game(controller, is_app)
    }

    // 0x267ec — -[PlaceLauncher injectJoinScript:]
    // type: void __cdecl(PlaceLauncher *self, SEL, id)
    // IDA 0x267ec
    #[doc(alias = "-[PlaceLauncher injectJoinScript:]")]
    #[doc = "-[PlaceLauncher injectJoinScript:]"]
    pub fn inject_join_script(&self, script: &str) {
        // The `UTF8String` + `joinGameWithJoinScript(script, game)` bind rows are
        // built on the caller thread (IDA 0x2681c..0x2688a); only execution moves.
        let game_id = self.view.lock().as_ref().and_then(|v| v.game_id()).unwrap_or(0);
        self.join_requests.lock().push(JoinRequest {
            place_id: *self.last_place_id.lock(),
            target: JoinTarget::Script(script.to_owned()),
            game_id,
            present_automatically: false,
        });
        // Fresh `boost::thread` named `"InjectStartScript"` which detaches at
        // `~thread` (IDA 0x268a0..0x268b2); a detached `std::thread` is the detach.
        self.inject_dispatches.fetch_add(1, Ordering::SeqCst);
        let owned = script.to_owned();
        std::thread::spawn(move || {
            let _ = (owned, game_id);
        });
    }

    // 0x29490 — -[PlaceLauncher startGame:controller:preloadedGame:presentGameAutomatically:]
    // IDA 0x29490 (next batch; shared by the start* leaves below).
    #[doc(alias = "-[PlaceLauncher startGame:controller:preloadedGame:presentGameAutomatically:]")]
    #[doc = "-[PlaceLauncher startGame:controller:preloadedGame:presentGameAutomatically:]"]
    pub fn start_preloaded_game(
        &self,
        game: &SharedPtr<GameHandle>,
        _controller: ObjCId,
        present: bool,
    ) -> bool {
        // `RBX::thread_wrapper(fn, "GameStartScript")` + `boost::thread` running
        // the bound join, detached at `~thread` (IDA 0x294c0..0x294fc); a detached
        // `std::thread` is the detach.
        self.game_start_threads.fetch_add(1, Ordering::SeqCst);
        // `createGame:presentGameAutomatically:` with the preloaded game
        // (IDA 0x29510..0x29534); always returns 1 (IDA 0x29560).
        self.create_game(game, present);
        true
    }

    // 0x26bb8 — -[PlaceLauncher startGameLocal:ipAddress:controller:presentGameAutomatically:]
    // type: char __cdecl(PlaceLauncher *self, SEL, int, id, id, char)
    // IDA 0x26bb8
    #[doc(alias = "-[PlaceLauncher startGameLocal:ipAddress:controller:presentGameAutomatically:]")]
    #[doc = "-[PlaceLauncher startGameLocal:ipAddress:controller:presentGameAutomatically:]"]
    pub fn start_game_local(
        &self,
        place_id: i32,
        ip: &str,
        controller: ObjCId,
        present: bool,
    ) -> bool {
        // `setupPreloadedGameWithNonGameController:unsecuredGame:isApp:` with
        // `unsecured = 1` (IDA 0x26c06..0x26c3c); nil game means failure.
        let Some(game) = self.setup_preloaded_game_unsecured(controller, true, false) else {
            return false;
        };
        // Bind `joinLocalGame(placeId, ip, game)` (IDA 0x26c58..0x26c90).
        self.join_requests.lock().push(JoinRequest {
            place_id,
            target: JoinTarget::LocalIp(ip.to_owned()),
            game_id: game.id,
            present_automatically: present,
        });
        // `startGame:controller:preloadedGame:presentGameAutomatically:` (IDA 0x26c96..0x26cc8).
        self.start_preloaded_game(&game, controller, present)
    }

    // 0x27054 — -[PlaceLauncher startAppWithFile:controller:presentGameAutomatically:]
    // type: char __cdecl(PlaceLauncher *self, SEL, id, id, char)
    // IDA 0x27054
    #[doc(alias = "-[PlaceLauncher startAppWithFile:controller:presentGameAutomatically:]")]
    #[doc = "-[PlaceLauncher startAppWithFile:controller:presentGameAutomatically:]"]
    pub fn start_app_with_file(&self, path: &str, controller: ObjCId, present: bool) -> bool {
        // Preloaded file-app game (`isApp = 1`; trailing bind flags are truncated
        // in pseudo, IDA 0x270a2..0x270d4); nil game means failure.
        let Some(game) = self.setup_preloaded_game_unsecured(controller, true, true) else {
            return false;
        };
        // Bind `loadLocalApp(path, game)` (IDA 0x270ea..0x27122).
        self.join_requests.lock().push(JoinRequest {
            place_id: 0,
            target: JoinTarget::FilePath(path.to_owned()),
            game_id: game.id,
            present_automatically: present,
        });
        // `startGame:controller:preloadedGame:presentGameAutomatically:` (IDA 0x27128..0x2715a).
        self.start_preloaded_game(&game, controller, present)
    }

    // 0x276b0 — -[PlaceLauncher startAppWithId:controller:presentGameAutomatically:]
    // type: char __cdecl(PlaceLauncher *self, SEL, int, id, char)
    // IDA 0x276b0
    #[doc(alias = "-[PlaceLauncher startAppWithId:controller:presentGameAutomatically:]")]
    #[doc = "-[PlaceLauncher startAppWithId:controller:presentGameAutomatically:]"]
    pub fn start_app_with_id(&self, place_id: i32, controller: ObjCId, present: bool) -> bool {
        // `setupPreloadedGameWithNonGameController:isApp:` with `isApp = 1`
        // (IDA 0x276fe..0x27732); nil game means failure.
        let Some(game) = self.setup_preloaded_game(controller, true) else {
            return false;
        };
        // Bind `joinGamePlaceId(placeId, game, request = 2)` (IDA 0x27738..0x2776a).
        self.join_requests.lock().push(JoinRequest {
            place_id,
            target: JoinTarget::PlaceId {
                request: 2,
            },
            game_id: game.id,
            present_automatically: present,
        });
        // `startGame:controller:preloadedGame:presentGameAutomatically:` (IDA 0x27770..0x277a2).
        self.start_preloaded_game(&game, controller, present)
    }

    // 0x289a8 — -[PlaceLauncher startGame:controller:request:presentGameAutomatically:]
    // type: char __cdecl(PlaceLauncher *self, SEL, int, id, int, char)
    // IDA 0x289a8
    #[doc(alias = "-[PlaceLauncher startGame:controller:request:presentGameAutomatically:]")]
    #[doc = "-[PlaceLauncher startGame:controller:request:presentGameAutomatically:]"]
    pub fn start_game_request(
        &self,
        place_id: i32,
        controller: ObjCId,
        request: i32,
        present: bool,
    ) -> bool {
        // `setupPreloadedGameWithNonGameController:isApp:` with `isApp = (request == 2)`
        // (IDA 0x289f6..0x28a32); nil game means failure.
        let Some(game) = self.setup_preloaded_game(controller, request == 2) else {
            return false;
        };
        // Bind `joinGamePlaceId(placeId, game, request)` (IDA 0x28a38..0x28a6c).
        self.join_requests.lock().push(JoinRequest {
            place_id,
            target: JoinTarget::PlaceId {
                request,
            },
            game_id: game.id,
            present_automatically: present,
        });
        // `startGame:controller:preloadedGame:presentGameAutomatically:` (IDA 0x28a72..0x28aa4).
        self.start_preloaded_game(&game, controller, present)
    }
    // 0x28ba8 — -[PlaceLauncher startGameSolo:controller:presentGameAutomatically:]
    // type: char __cdecl(PlaceLauncher *self, SEL, int, id, char)
    // IDA 0x28ba8
    #[doc(alias = "-[PlaceLauncher startGameSolo:controller:presentGameAutomatically:]")]
    #[doc = "-[PlaceLauncher startGameSolo:controller:presentGameAutomatically:]"]
    pub fn start_game_solo(&self, place_id: i32, controller: ObjCId, present: bool) -> bool {
        self.start_solo_calls.fetch_add(1, Ordering::SeqCst);
        // `setupPreloadedGameWithNonGameController:isApp:` (IDA 0x28c1e); nil
        // game (or nil self) means failure (IDA 0x28c26, 0x28cc8..0x28cce).
        let Some(game) = self.setup_preloaded_game(controller, false) else {
            return false;
        };
        // Bind `joinGamePlaceIdSolo(placeId, game)` into `function0<void>`
        // (IDA 0x28c2c..0x28c5c).
        self.join_requests.lock().push(JoinRequest {
            place_id,
            target: JoinTarget::Solo,
            game_id: game.id,
            present_automatically: present,
        });
        // `startGame:controller:preloadedGame:presentGameAutomatically:`
        // (IDA 0x28c62..0x28c94).
        self.start_preloaded_game(&game, controller, present)
    }

    // 0x29280 — -[PlaceLauncher startGameWithJoinScript:controller:presentGameAutomatically:]
    // type: char __cdecl(PlaceLauncher *self, SEL, id, id, char)
    // IDA 0x29280
    #[doc(alias = "-[PlaceLauncher startGameWithJoinScript:controller:presentGameAutomatically:]")]
    #[doc = "-[PlaceLauncher startGameWithJoinScript:controller:presentGameAutomatically:]"]
    pub fn start_game_with_join_script(
        &self,
        script: &str,
        controller: ObjCId,
        present: bool,
    ) -> bool {
        self.start_join_script_calls.fetch_add(1, Ordering::SeqCst);
        // `setupPreloadedGameWithNonGameController:isApp:` (IDA 0x292f4); nil
        // game (or nil self) means failure (IDA 0x292fc, 0x293b8..0x293be).
        let Some(game) = self.setup_preloaded_game(controller, false) else {
            return false;
        };
        // `UTF8String` + bind `joinGameWithJoinScript(script, game)`
        // (IDA 0x29314..0x2934c).
        self.join_requests.lock().push(JoinRequest {
            place_id: *self.last_place_id.lock(),
            target: JoinTarget::Script(script.to_owned()),
            game_id: game.id,
            present_automatically: present,
        });
        // `startGame:controller:preloadedGame:presentGameAutomatically:`
        // (IDA 0x29352..0x29384).
        self.start_preloaded_game(&game, controller, present)
    }

    // 0x295c0 — -[PlaceLauncher leaveGameShutdown]
    // type: void __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x295c0
    #[doc(alias = "-[PlaceLauncher leaveGameShutdown]")]
    #[doc = "-[PlaceLauncher leaveGameShutdown]"]
    pub fn leave_game_shutdown(&self) {
        self.leave_shutdown_calls.fetch_add(1, Ordering::SeqCst);
        // Post `startLeaveGameNotification` with nil userInfo (IDA 0x295fe..0x29622).
        self.start_leave_posts.fetch_add(1, Ordering::SeqCst);
        // `dismissViewControllerAnimated:completion:` on the ogre controller
        // with the 0x29684 block (IDA 0x29634..0x2967c); the dismissal and the
        // block run inline here.
        self.ogre_view_controller_present.store(false, Ordering::SeqCst);
        self.leave_game_shutdown_completion();
    }

    // 0x29684 — ___34-[PlaceLauncher leaveGameShutdown]_block_invoke
    // IDA 0x29684
    #[doc(alias = "___34-[PlaceLauncher leaveGameShutdown]_block_invoke")]
    #[doc = "___34-[PlaceLauncher leaveGameShutdown]_block_invoke"]
    pub fn leave_game_shutdown_completion(&self) {
        self.shutdown_completions.fetch_add(1, Ordering::SeqCst);
        // Release the ogre controller/view/window (IDA 0x2969e..0x296ee); the
        // controller slot is what this slice observes.
        self.ogre_view_controller_present.store(false, Ordering::SeqCst);
        // `deleteRobloxView` (IDA 0x29700).
        self.view.lock().take();
        // `isCurrentlyPlayingGame = 0` (offset 20, IDA 0x2971c) and
        // `hasReceivedMemoryWarning = 0` (offset 8, IDA 0x29738).
        self.is_currently_playing_game.store(false, Ordering::SeqCst);
        self.has_received_memory_warning.store(false, Ordering::SeqCst);
        // Post `didLeaveGameNotification` (IDA 0x29740..0x29764).
        self.did_leave_posts.fetch_add(1, Ordering::SeqCst);
        // `removeObjectForKey:@"RobloxGameState"` + `synchronize`
        // (IDA 0x29790..0x297c2).
        self.game_state.lock().clear();
        // `isLeavingGame = 0` (offset 9, IDA 0x297e8).
        self.is_leaving_game.store(false, Ordering::SeqCst);
        // `endBackgroundTask:` + `setBgTask:UIBackgroundTaskInvalid`
        // (IDA 0x297f4..0x29872); the delegate bg task slot is the store.
        self.bg_task.lock().take();
    }

    // 0x298e0 — -[PlaceLauncher leaveGame]
    // type: void __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x298e0
    #[doc(alias = "-[PlaceLauncher leaveGame]")]
    #[doc = "-[PlaceLauncher leaveGame]"]
    pub fn leave_game(&self) {
        self.leave_game_calls.fetch_add(1, Ordering::SeqCst);
        // Guards: `isCurrentlyPlayingGame` set and `isLeavingGame` clear, with an
        // ogre controller present (IDA 0x2996e..0x2998e); otherwise no-op.
        if !self.is_currently_playing_game.load(Ordering::SeqCst)
            || self.is_leaving_game.load(Ordering::SeqCst)
            || !self.ogre_view_controller_present.load(Ordering::SeqCst)
        {
            return;
        }
        // `isLeavingGame = 1` (IDA 0x299a2).
        self.is_leaving_game.store(true, Ordering::SeqCst);
        // `setIdleTimerDisabled:0` (IDA 0x299c6..0x299d8).
        self.idle_timer_disabled.store(false, Ordering::SeqCst);
        // `RobloxGameState = "leaveGame"` + `synchronize` (IDA 0x299f8..0x29a36).
        *self.game_state.lock() = "leaveGame".to_owned();
        // `closeChildConnections` (IDA 0x29a48).
        self.close_child_connections();
        // `reportSessionFor:4` + `Visit/Success/LeaveGame` page view
        // (IDA 0x29a5a..0x29a92).
        self.session_reports.lock().push((4, *self.last_place_id.lock()));
        self.page_views.lock().push("Visit/Success/LeaveGame".to_owned());
        // `beginBackgroundTaskWithExpirationHandler:` with the 0x29bb4 block
        // (IDA 0x29aec..0x29b12); the bg task id is the store.
        let id = self.next_bg_task.fetch_add(1, Ordering::SeqCst) + 1;
        *self.bg_task.lock() = Some(id);
        // iOS 6+ takes the `dispatch_async(main, 0x29c74-block)` path
        // (IDA 0x29b2e..0x29ba8); that block calls `leaveGameShutdown` inline.
        // BUG: original at 0x29b62 — floats compared via `COERCE_FLOAT`;
        // pre-6.0 falls to the direct call (IDA 0x29b72). Modern iOS always
        // takes the dispatch path; both end in `leaveGameShutdown`.
        self.main_dispatches.fetch_add(1, Ordering::SeqCst);
        self.leave_game_shutdown_dispatch();
    }

    // 0x29bb4 — ___26-[PlaceLauncher leaveGame]_block_invoke
    // IDA 0x29bb4
    #[doc(alias = "___26-[PlaceLauncher leaveGame]_block_invoke")]
    #[doc = "___26-[PlaceLauncher leaveGame]_block_invoke"]
    pub fn leave_game_bg_expiration(&self) {
        // Background-task expiration handler: `isLeavingGame = 0`
        // (IDA 0x29bde), `endBackgroundTask:` + `setBgTask:Invalid`
        // (IDA 0x29be8..0x29c1e).
        self.is_leaving_game.store(false, Ordering::SeqCst);
        self.bg_task.lock().take();
    }

    // 0x29c74 — ___26-[PlaceLauncher leaveGame]_block_invoke231
    // IDA 0x29c74
    #[doc(alias = "___26-[PlaceLauncher leaveGame]_block_invoke231")]
    #[doc = "___26-[PlaceLauncher leaveGame]_block_invoke231"]
    pub fn leave_game_shutdown_dispatch(&self) {
        // Main-queue block body is just `leaveGameShutdown` (IDA 0x29c74..0x29c90).
        self.leave_game_shutdown();
    }

    // 0x29c9c — -[PlaceLauncher disableViewBecauseGoingToBackground]
    // type: void __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x29c9c
    #[doc(alias = "-[PlaceLauncher disableViewBecauseGoingToBackground]")]
    #[doc = "-[PlaceLauncher disableViewBecauseGoingToBackground]"]
    pub fn disable_view_for_background(&self) {
        // `if (rbxView) requestStopRenderingForBackgroundMode` (IDA 0x29ca8..0x29cae).
        if let Some(view) = self.view.lock().as_ref() {
            view.request_stop_rendering_for_background_mode();
        }
    }

    // 0x29cb4 — -[PlaceLauncher enableViewBecauseGoingToForeground]
    // type: void __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x29cb4
    #[doc(alias = "-[PlaceLauncher enableViewBecauseGoingToForeground]")]
    #[doc = "-[PlaceLauncher enableViewBecauseGoingToForeground]"]
    pub fn enable_view_for_foreground(&self) {
        // `if (rbxView) requestResumeRendering` (IDA 0x29cc0..0x29cc6).
        if let Some(view) = self.view.lock().as_ref() {
            view.request_resume_rendering();
        }
    }

    // 0x29ccc — -[PlaceLauncher teleport:withAuthentication:withScript:]
    // type: void __cdecl(PlaceLauncher *self, SEL, id, id, id)
    // IDA 0x29ccc
    #[doc(alias = "-[PlaceLauncher teleport:withAuthentication:withScript:]")]
    #[doc = "-[PlaceLauncher teleport:withAuthentication:withScript:]"]
    pub fn teleport(&self, place: &str, auth: &str, script: &str, controller: ObjCId) {
        // `setLastNonGameController:` from the main controller (IDA 0x29d0a..0x29d42).
        let last = self.last_non_game_controller();
        self.set_last_non_game_controller(last.or(Some(controller)));
        // Fresh `SecurePlayerGame(baseURL)` + shared ptr (IDA 0x29d58..0x29da0).
        let id = self.next_game_id.fetch_add(1, Ordering::SeqCst) + 1;
        self.last_game_secure.store(true, Ordering::SeqCst);
        let game = wrap_game(id);
        // `UTF8String` × 3 + bind `joinGameTeleport` + `boost::thread`
        // (IDA 0x29db8..0x29e40); the thread detaches at `~thread` (IDA 0x29fdc).
        *self.last_teleport.lock() = Some((place.to_owned(), auth.to_owned(), script.to_owned()));
        self.join_requests.lock().push(JoinRequest {
            place_id: *self.last_place_id.lock(),
            target: JoinTarget::Teleport {
                place: place.to_owned(),
                auth: auth.to_owned(),
                script: script.to_owned(),
            },
            game_id: game.id,
            present_automatically: false,
        });
        self.teleport_dispatches.fetch_add(1, Ordering::SeqCst);
        // `deleteRobloxView` (IDA 0x29ec6) before the shrink animation.
        self.view.lock().take();
        // `setClipsToBounds:1` + `animateWithDuration:0.5` shrink (animations
        // 0x2a8c8) with completion 0x2a99c (IDA 0x29f06..0x29fca); both blocks
        // run inline here.
        self.teleport_animation_steps.fetch_add(1, Ordering::SeqCst);
        self.teleport_completion(&game, controller);
    }

    // 0x2a8c8 — ___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke
    // IDA 0x2a8c8
    #[doc(alias = "___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke")]
    #[doc = "___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke"]
    pub fn teleport_animation_frame(&self, width: f32, height: f32) -> (f32, f32) {
        // Shrink-to-center frame: the 1×1 view lands at half the parent frame
        // origin (`vmul_f32(size, 0.5)`), sized 1×1 (IDA 0x2a8e8..0x2a984); a nil
        // parent view yields a zero origin. Only the step count is observed.
        self.teleport_animation_steps.fetch_add(1, Ordering::SeqCst);
        (width * 0.5, height * 0.5)
    }

    // 0x2a99c — ___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke246
    // IDA 0x2a99c
    #[doc(alias = "___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke246")]
    #[doc = "___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke246"]
    pub fn teleport_completion(&self, game: &SharedPtr<GameHandle>, controller: ObjCId) {
        self.teleport_completions.fetch_add(1, Ordering::SeqCst);
        // `finishGameSetup:gameViewController:` on the launcher (IDA 0x2aa18).
        self.finish_game_setup(game, controller, 0, 0, false, false);
        // Bind `finishTeleport(view, game, marshaller)` into
        // `function<void(DataModel*)>` + `DataModel::submitTask(..., 1)`
        // (IDA 0x2aa3c..0x2aaaa); the datamodel executes it out of slice.
        self.control_view_tasks.fetch_add(1, Ordering::SeqCst);
    }

    // 0x2ae54 — -[PlaceLauncher applicationDidReceiveMemoryWarning]
    // type: void __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x2ae54
    #[doc(alias = "-[PlaceLauncher applicationDidReceiveMemoryWarning]")]
    #[doc = "-[PlaceLauncher applicationDidReceiveMemoryWarning]"]
    pub fn application_did_receive_memory_warning(&self) {
        // Out of game: log and ignore (IDA 0x2afc2..0x2afe8).
        if !self.is_currently_playing_game.load(Ordering::SeqCst) {
            self.memory_warning_ignores.fetch_add(1, Ordering::SeqCst);
            return;
        }
        // `freeMemoryBytes` log + `print_free_memory` (IDA 0x2aebe..0x2aed6).
        let place = *self.last_place_id.lock();
        // Connected when either the child or the player connection is live
        // (IDA 0x2aeea..0x2af06).
        let connected =
            self.child_connection_connected.load(Ordering::SeqCst)
                || self.player_connection_connected.load(Ordering::SeqCst);
        // `PlayErrors/OutOfMemory_EarlyExit` + session 5 when connected, else
        // `PlayErrors/OutOfMemory` + session 6 (IDA 0x2af42..0x2b03c).
        if connected {
            self.analytics_events.lock().push((
                "PlayErrors".to_owned(),
                "OutOfMemory_EarlyExit".to_owned(),
                place.to_string(),
            ));
            self.session_reports.lock().push((5, place));
        } else {
            self.analytics_events.lock().push((
                "PlayErrors".to_owned(),
                "OutOfMemory".to_owned(),
                place.to_string(),
            ));
            self.session_reports.lock().push((6, place));
        }
        // `closeChildConnections` (IDA 0x2b056).
        self.close_child_connections();
        // `warnings_preference` `MemoryError` alert (IDA 0x2b074..0x2b100).
        if self.warnings_preference.load(Ordering::SeqCst) {
            self.alerts.lock().push("MemoryError".to_owned());
        }
        // In-game shutdown via `leaveGame` (IDA 0x2b108..0x2b142).
        self.memory_warning_shutdowns.fetch_add(1, Ordering::SeqCst);
        self.leave_game();
    }

    // 0x2b1bc — -[PlaceLauncher childAdded:]
    // type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Instance>)
    // IDA 0x2b1bc
    #[doc(alias = "-[PlaceLauncher childAdded:]")]
    #[doc = "-[PlaceLauncher childAdded:]"]
    pub fn child_added(
        &self,
        has_view_game: bool,
        players_found: bool,
        player_is_child: bool,
        player_found: bool,
    ) -> ChildAddedOutcome {
        self.child_added_connections.fetch_add(1, Ordering::SeqCst);
        // Nil rbxView: log + `closeChildConnections` (IDA 0x2b326..0x2b33a).
        if !has_view_game {
            self.close_child_connections();
            return ChildAddedOutcome::NoView;
        }
        // Nil datamodel/game path (IDA 0x2b34e..0x2b364).
        // Missing Players service (IDA 0x2b378..0x2b38e) or a nil player
        // (IDA 0x2b3a2..0x2b3b8): log + `closeChildConnections`.
        if !players_found || !player_found {
            self.close_child_connections();
            return ChildAddedOutcome::NoPlayers;
        }
        // Either branch connects `playerLoaded:` on the player-added signal and
        // swaps the child connection for the player connection
        // (IDA 0x2b276..0x2b310, 0x2b3cc..0x2b466); `Signal` is
        // `rbx_core::signal::Signal`, never `boost::signals`.
        self.child_connection_connected.store(false, Ordering::SeqCst);
        self.player_connection_connected.store(true, Ordering::SeqCst);
        if player_is_child {
            ChildAddedOutcome::PlayerConnected
        } else {
            ChildAddedOutcome::PlayerReconnected
        }
    }

    // 0x2b548 — -[PlaceLauncher playerLoaded:]
    // type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Instance>)
    // IDA 0x2b548
    #[doc(alias = "-[PlaceLauncher playerLoaded:]")]
    #[doc = "-[PlaceLauncher playerLoaded:]"]
    pub fn player_loaded(&self, instance: u32) {
        // Disconnect the player connection (IDA 0x2b56a), then
        // `closeChildConnections` (IDA 0x2b57c); the instance id is logged
        // (IDA 0x2b558).
        let _ = instance;
        self.player_connection_connected.store(false, Ordering::SeqCst);
        self.close_child_connections();
        // `RobloxGameState = "inGame"` + `synchronize` (IDA 0x2b59a..0x2b5da).
        *self.game_state.lock() = "inGame".to_owned();
    }

    // 0x2b5e0 — -[PlaceLauncher closeChildConnections]
    // type: void __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x2b5e0
    #[doc(alias = "-[PlaceLauncher closeChildConnections]")]
    #[doc = "-[PlaceLauncher closeChildConnections]"]
    pub fn close_child_connections(&self) {
        self.close_child_calls.fetch_add(1, Ordering::SeqCst);
        // Disconnect the child connection when connected (IDA 0x2b5f2..0x2b600),
        // then the player connection (IDA 0x2b610..0x2b61e).
        self.child_connection_connected.store(false, Ordering::SeqCst);
        self.player_connection_connected.store(false, Ordering::SeqCst);
        // `stopFreeMemoryChecker` (IDA 0x2b63a..0x2b64e).
        self.free_memory_checker_running.store(false, Ordering::SeqCst);
    }

    // 0x2b654 — -[PlaceLauncher .cxx_destruct]
    // type: void __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x2b654
    #[doc(alias = "-[PlaceLauncher .cxx_destruct]")]
    #[doc = "-[PlaceLauncher .cxx_destruct]"]
    pub fn cxx_destruct(&self) {
        // Weak-release the player + child connection slots (IDA 0x2b68e..0x2b6cc);
        // `intrusive_ptr` drops are `Arc` drops here.
        self.player_connection_connected.store(false, Ordering::SeqCst);
        self.child_connection_connected.store(false, Ordering::SeqCst);
        // Teleporter `delete px` via vtable+4 (IDA 0x2b6de..0x2b6e6).
        self.teleporter_callback_set.store(false, Ordering::SeqCst);
    }

    // 0x2b724 — -[PlaceLauncher .cxx_construct]
    // type: id __cdecl(PlaceLauncher *self, SEL)
    // IDA 0x2b724
    #[doc(alias = "-[PlaceLauncher .cxx_construct]")]
    #[doc = "-[PlaceLauncher .cxx_construct]"]
    pub fn cxx_construct(&self) {
        // Zero the teleporter + both connection slots (IDA 0x2b73c..0x2b74e).
        self.teleporter_callback_set.store(false, Ordering::SeqCst);
        self.child_connection_connected.store(false, Ordering::SeqCst);
        self.player_connection_connected.store(false, Ordering::SeqCst);
    }

    pub fn last_place_id(&self) -> i32 {
        *self.last_place_id.lock()
    }
    pub fn last_non_game_controller(&self) -> Option<ObjCId> {
        *self.last_non_game_controller.lock()
    }
    pub fn has_view(&self) -> bool {
        self.view.lock().is_some()
    }
    pub fn teleporter_callback_set(&self) -> bool {
        self.teleporter_callback_set.load(Ordering::SeqCst)
    }
    pub fn idle_timer_disabled(&self) -> bool {
        self.idle_timer_disabled.load(Ordering::SeqCst)
    }
    pub fn ogre_view_controller_present(&self) -> bool {
        self.ogre_view_controller_present.load(Ordering::SeqCst)
    }
    pub fn last_game_secure(&self) -> bool {
        self.last_game_secure.load(Ordering::SeqCst)
    }
    pub fn last_game_is_app(&self) -> bool {
        self.last_game_is_app.load(Ordering::SeqCst)
    }
    pub fn prepare_calls(&self) -> u32 {
        self.prepare_calls.load(Ordering::SeqCst)
    }
    pub fn check_part_dispatches(&self) -> u32 {
        self.check_part_dispatches.load(Ordering::SeqCst)
    }
    pub fn part_warnings(&self) -> u32 {
        self.part_warnings.load(Ordering::SeqCst)
    }
    pub fn game_finished_loading_posts(&self) -> u32 {
        self.game_finished_loading_posts.load(Ordering::SeqCst)
    }
    pub fn deferred_finish_loading(&self) -> u32 {
        self.deferred_finish_loading.load(Ordering::SeqCst)
    }
    pub fn failure_forwards(&self) -> u32 {
        self.failure_forwards.load(Ordering::SeqCst)
    }
    pub fn datamodel_connections(&self) -> u32 {
        self.datamodel_connections.load(Ordering::SeqCst)
    }
    pub fn control_view_tasks(&self) -> u32 {
        self.control_view_tasks.load(Ordering::SeqCst)
    }
    pub fn inject_dispatches(&self) -> u32 {
        self.inject_dispatches.load(Ordering::SeqCst)
    }
    pub fn join_request_count(&self) -> usize {
        self.join_requests.lock().len()
    }
    pub fn last_join_request(&self) -> Option<JoinRequest> {
        self.join_requests.lock().last().cloned()
    }
    pub fn alerts(&self) -> Vec<String> {
        self.alerts.lock().clone()
    }
    pub fn analytics_event_count(&self) -> usize {
        self.analytics_events.lock().len()
    }
    pub fn is_leaving_game(&self) -> bool {
        self.is_leaving_game.load(Ordering::SeqCst)
    }
    pub fn leave_game_calls(&self) -> u32 {
        self.leave_game_calls.load(Ordering::SeqCst)
    }
    pub fn leave_shutdown_calls(&self) -> u32 {
        self.leave_shutdown_calls.load(Ordering::SeqCst)
    }
    pub fn shutdown_completions(&self) -> u32 {
        self.shutdown_completions.load(Ordering::SeqCst)
    }
    pub fn start_leave_posts(&self) -> u32 {
        self.start_leave_posts.load(Ordering::SeqCst)
    }
    pub fn did_leave_posts(&self) -> u32 {
        self.did_leave_posts.load(Ordering::SeqCst)
    }
    pub fn bg_task(&self) -> Option<u32> {
        *self.bg_task.lock()
    }
    pub fn game_state(&self) -> String {
        self.game_state.lock().clone()
    }
    pub fn session_reports(&self) -> Vec<(u32, i32)> {
        self.session_reports.lock().clone()
    }
    pub fn page_views(&self) -> Vec<String> {
        self.page_views.lock().clone()
    }
    pub fn child_connection_connected(&self) -> bool {
        self.child_connection_connected.load(Ordering::SeqCst)
    }
    pub fn player_connection_connected(&self) -> bool {
        self.player_connection_connected.load(Ordering::SeqCst)
    }
    pub fn close_child_calls(&self) -> u32 {
        self.close_child_calls.load(Ordering::SeqCst)
    }
    pub fn free_memory_checker_running(&self) -> bool {
        self.free_memory_checker_running.load(Ordering::SeqCst)
    }
    pub fn memory_warning_shutdowns(&self) -> u32 {
        self.memory_warning_shutdowns.load(Ordering::SeqCst)
    }
    pub fn memory_warning_ignores(&self) -> u32 {
        self.memory_warning_ignores.load(Ordering::SeqCst)
    }
    pub fn teleport_dispatches(&self) -> u32 {
        self.teleport_dispatches.load(Ordering::SeqCst)
    }
    pub fn teleport_completions(&self) -> u32 {
        self.teleport_completions.load(Ordering::SeqCst)
    }
    pub fn teleport_animation_steps(&self) -> u32 {
        self.teleport_animation_steps.load(Ordering::SeqCst)
    }
    pub fn last_teleport(&self) -> Option<(String, String, String)> {
        self.last_teleport.lock().clone()
    }
    pub fn start_solo_calls(&self) -> u32 {
        self.start_solo_calls.load(Ordering::SeqCst)
    }
    pub fn start_join_script_calls(&self) -> u32 {
        self.start_join_script_calls.load(Ordering::SeqCst)
    }
    pub fn game_start_threads(&self) -> u32 {
        self.game_start_threads.load(Ordering::SeqCst)
    }
    pub fn set_ogre_view_controller_present(&self, present: bool) {
        self.ogre_view_controller_present.store(present, Ordering::SeqCst);
    }
    pub fn set_child_connection_connected(&self, connected: bool) {
        self.child_connection_connected.store(connected, Ordering::SeqCst);
    }
    pub fn set_player_connection_connected(&self, connected: bool) {
        self.player_connection_connected.store(connected, Ordering::SeqCst);
    }
    pub fn set_free_memory_checker_running(&self, running: bool) {
        self.free_memory_checker_running.store(running, Ordering::SeqCst);
    }
    pub fn set_reachability(&self, reachability: ReachabilityStatus) {
        *self.reachability.lock() = reachability;
    }
    pub fn set_wifi_only(&self, wifi_only: bool) {
        self.wifi_only.store(wifi_only, Ordering::SeqCst);
    }
    pub fn set_warnings_preference(&self, enabled: bool) {
        self.warnings_preference.store(enabled, Ordering::SeqCst);
    }
    pub fn set_max_place_parts(&self, max: i32) {
        *self.max_place_parts.lock() = max;
    }
    pub fn set_current_part_count(&self, parts: i32) {
        *self.current_part_count.lock() = parts;
    }
}

// Teleporter + teleportImpl bind rows (IDA 0x33548..0x35438) + page-teleport
// thread rows (IDA 0x2ce2c..0x2e284). `SharedPtr` is `rbx_core::SharedPtr`
// (`Arc`), never `boost::shared_ptr`; `boost::bind` targets become
// [`TeleportArgs`]/[`PageTeleportArgs`] rows, the page-teleport
// `boost::thread` a detached `std::thread`.

/// `bind(teleportImpl, launcher, place, auth, script)` argument quad: the
/// `boost::bind_t<void,void (*)(PlaceLauncher *,std::string × 3)>` behind
/// `Teleporter::doTeleport` (IDA 0x33924). Owned strings: the original's
/// `std::string` copies are moves here.
#[derive(Debug, Clone, Default)]
pub struct TeleportArgs {
    pub launcher: Option<ObjCId>,
    pub place: String,
    pub auth: String,
    pub script: String,
}

/// `boost::function<void()>` holding the teleport quad (IDA 0x342f4/0x345b0);
/// the call operator takes no argument (everything is bound).
#[derive(Debug, Clone, Default)]
pub struct TeleportCallback {
    pub args: TeleportArgs,
}

/// `bind(pageTeleport, s0, s1, s2, controller, game)` 5-tuple: the
/// `boost::bind_t<void,void (*)(std::string × 3,NSObject *,SharedPtr<Game>)>`
/// behind the page-teleport `boost::thread` (IDA 0x2ce2c).
#[derive(Debug, Clone, Default)]
pub struct PageTeleportArgs {
    pub first: String,
    pub second: String,
    pub third: String,
    pub controller: Option<ObjCId>,
    pub game: Option<u32>,
}

/// Minimal `Teleporter` counterpart (IDA 0x33548..0x33920): the bound
/// `PlaceLauncher` (this+1) and the `FunctionMarshaller` submit target
/// (this+2) read by `doTeleport`, plus submit/drop counters. The live
/// marshaller queue lives out of slice; `submits`/`last_submitted` record
/// the `Submit` + `clear` flow.
#[derive(Debug, Default)]
pub struct Teleporter {
    launcher: parking_lot::Mutex<ObjCId>,
    marshaller: parking_lot::Mutex<ObjCId>,
    submits: AtomicU32,
    dropped: AtomicBool,
    last_submitted: parking_lot::Mutex<Option<TeleportArgs>>,
}

impl Teleporter {
    pub fn new() -> Self {
        Self::default()
    }

    // 0x33548 — __ZN10TeleporterD1Ev
    // type: void __fastcall(Teleporter *__hidden this)
    // IDA 0x33548
    #[doc(alias = "Teleporter::~Teleporter()")]
    #[doc = "Teleporter::~Teleporter()"]
    pub fn drop_teleporter(&self) {
        // Non-deleting dtor: body is empty, members are trivially released
        // (IDA 0x33548); `std::string`/slot drops are Rust drops.
        self.dropped.store(true, Ordering::SeqCst);
    }

    // 0x3354c — __ZN10TeleporterD0Ev
    // type: void __fastcall(Teleporter *__hidden this)
    // IDA 0x3354c
    #[doc(alias = "Teleporter::~Teleporter()")]
    #[doc = "Teleporter::~Teleporter()"]
    pub fn delete_teleporter(&self) {
        // Deleting dtor: same member release plus `operator delete`
        // (IDA 0x3354c); the free is the `Arc` drop here.
        self.drop_teleporter();
    }

    // 0x33550 — __ZN10Teleporter10doTeleportERKSsS1_S1_
    // type: _DWORD __fastcall(Teleporter *__hidden this, const std::string *, const std::string *, const std::string *)
    // IDA 0x33550
    #[doc(alias = "Teleporter::doTeleport(std::string const&,std::string const&,std::string const&)")]
    #[doc = "Teleporter::doTeleport(std::string const&,std::string const&,std::string const&)"]
    pub fn do_teleport(&self, place: &str, auth: &str, script: &str) -> bool {
        // Copies the three strings (IDA 0x3357a..0x335be; moves here).
        let launcher = *self.launcher.lock();
        let args = bind_teleport_impl(launcher, place, auth, script);
        // Binds `teleportImpl(launcher, s0, s1, s2)` into `function0<void>`
        // (IDA 0x335e0..0x335ec).
        let callback = wrap_teleport_callback(args.clone());
        // `FunctionMarshaller::Submit(marshaller, fn)` then `clear`
        // (IDA 0x335f8..0x33602); the queue runs out of slice, so the submit
        // is recorded and the callback dropped (the six `_M_destroy` tails,
        // IDA 0x33614..0x33758, are Rust drops).
        *self.last_submitted.lock() = Some(callback.args);
        self.submits.fetch_add(1, Ordering::SeqCst);
        true
    }

    // 0x33920 — __ZNK10Teleporter17isTeleportEnabledEv
    // type: _DWORD __fastcall(Teleporter *__hidden this)
    // IDA 0x33920
    #[doc(alias = "Teleporter::isTeleportEnabled(void)const")]
    #[doc = "Teleporter::isTeleportEnabled(void)const"]
    pub fn is_teleport_enabled(&self) -> bool {
        // `return 1` (IDA 0x33922).
        true
    }

    pub fn set_launcher(&self, launcher: ObjCId) {
        *self.launcher.lock() = launcher;
    }
    pub fn set_marshaller(&self, marshaller: ObjCId) {
        *self.marshaller.lock() = marshaller;
    }
    pub fn submits(&self) -> u32 {
        self.submits.load(Ordering::SeqCst)
    }
    pub fn last_submitted(&self) -> Option<TeleportArgs> {
        self.last_submitted.lock().clone()
    }
}

// 0x33924 — __ZN5boost4bindIvP13PlaceLauncherSsSsSsS2_SsSsSsEENS_3_bi6bind_tIT_PFS5_T0_T1_T2_T3_ENS3_9list_av_4IT4_T5_T6_T7_E4typeEEESB_SD_SE_SF_SG_
// type: int __fastcall(int, int, int, std::string *, std::string *, std::string *)
// IDA 0x33924
#[doc(alias = "boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list_av_4<PlaceLauncher *,std::string,std::string,std::string>::type> boost::bind<void,PlaceLauncher *,std::string,std::string,std::string,PlaceLauncher *,std::string,std::string,std::string>(void (*)(PlaceLauncher *,std::string,std::string,std::string),PlaceLauncher *,std::string,std::string,std::string)")]
pub fn bind_teleport_impl(
    launcher: ObjCId,
    place: &str,
    auth: &str,
    script: &str,
) -> TeleportArgs {
    // Copies the three strings into the `list4` store and captures
    // `(teleportImpl, launcher, s0, s1, s2)` into the `bind_t`
    // (IDA 0x33924..0x33b3c); the closure captures the same quad.
    build_teleport_storage(launcher, place, auth, script)
}

// 0x33d00 — __ZN10Teleporter12teleportImplEP13PlaceLauncherSsSsSs
// IDA 0x33d00
#[doc(alias = "Teleporter::teleportImpl(PlaceLauncher *,std::string,std::string,std::string)")]
#[doc = "Teleporter::teleportImpl(PlaceLauncher *,std::string,std::string,std::string)"]
pub fn teleport_impl(launcher: &PlaceLauncher, place: &str, auth: &str, script: &str) {
    // Each `std::string` becomes an `NSString` via
    // `stringWithCString:defaultCStringEncoding` (IDA 0x33d24..0x33d8a);
    // owned `String`s are the conversion here.
    let place_ns = place.to_owned();
    let auth_ns = auth.to_owned();
    let script_ns = script.to_owned();
    // BUG: original forwards a3 as `place` and a2 as `auth`
    // (`teleport:v9 withAuthentication:v14 withScript:v12` with v9 from a3,
    // v14 from a2, IDA 0x33dac); preserved. No controller flows through this
    // path, so `nil` (`NIL_ID`) is passed.
    launcher.teleport(&auth_ns, &place_ns, &script_ns, NIL_ID);
}

// 0x33db0 — __ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EC2ES5_S6_S6_S6_
// type: int __fastcall(int, int, std::string *, int, std::string *)
// IDA 0x33db0
#[doc(alias = "boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::list4(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn build_teleport_list(
    launcher: ObjCId,
    place: &str,
    auth: &str,
    script: &str,
) -> TeleportArgs {
    // `list4` value-quad ctor (IDA 0x33db0).
    TeleportArgs {
        launcher: Some(launcher),
        place: place.to_owned(),
        auth: auth.to_owned(),
        script: script.to_owned(),
    }
}

// 0x33fe0 — __ZN5boost3_bi8storage4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EC2ES5_S6_S6_S6_
// type: int __fastcall(int, int, std::string *, int, std::string *)
// IDA 0x33fe0
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage4(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn build_teleport_storage(
    launcher: ObjCId,
    place: &str,
    auth: &str,
    script: &str,
) -> TeleportArgs {
    // `storage4` wraps the same quad (IDA 0x33fe0).
    build_teleport_list(launcher, place, auth, script)
}

// 0x341ac — __ZN5boost3_bi8storage3INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_EC2ES5_S6_S6_
// type: int __fastcall(int, int, std::string *)
// IDA 0x341ac
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn build_teleport_prefix(launcher: ObjCId, first: &str, second: &str) -> TeleportArgs {
    // Three-element intermediate (`launcher` + two strings, IDA 0x341ac);
    // the trailing `script` slot binds later, so it stays empty.
    TeleportArgs {
        launcher: Some(launcher),
        place: first.to_owned(),
        auth: second.to_owned(),
        script: String::new(),
    }
}

// 0x342f4 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// type: int(void)
// IDA 0x342f4
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")]
pub fn wrap_teleport_callback(args: TeleportArgs) -> TeleportCallback {
    // `function<void()>` ctor from the quad bind (IDA 0x342f4; copies the
    // three strings, IDA 0x34328..0x34372, then chains into `function0`,
    // IDA 0x34384); the invocable holds the quad.
    TeleportCallback { args }
}

// 0x345b0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE
// type: int(void)
// IDA 0x345b0
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")]
pub fn wrap_teleport_callback0(args: TeleportArgs) -> TeleportCallback {
    // `function0` ctor, same capture (IDA 0x345b0).
    TeleportCallback { args }
}

// 0x34870 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEEvT_
// type: int(void)
// IDA 0x34870
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>)")]
pub fn assign_teleport_callback(slot: &mut Option<TeleportCallback>, args: TeleportArgs) {
    // Stores the quad functor into the `function0` buffer (IDA 0x34870).
    *slot = Some(TeleportCallback { args });
}

// 0x34b40 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// IDA 0x34b40
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn manage_teleport_fn(op: FunctorOp, slot: &mut Option<TeleportArgs>) -> bool {
    // Clone/destroy over the teleport quad (IDA 0x34b40).
    manage_boxed_slot(op, slot)
}

// 0x34b5c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEvE6invokeERNS1_15function_bufferE
// IDA 0x34b5c
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn invoke_teleport_fn(launcher: &PlaceLauncher, args: &TeleportArgs) {
    // Tail-calls `list4::operator()` (IDA 0x34b6e).
    apply_teleport_list(launcher, args);
}

// 0x34b70 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEbT_RNS1_15function_bufferE
// type: int(void)
// IDA 0x34b70
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
pub fn vtable_assign_teleport_fn(slot: &mut Option<TeleportCallback>, args: TeleportArgs) -> bool {
    // `basic_vtable0::assign_to` without tag: stores and reports success
    // (IDA 0x34b70).
    *slot = Some(TeleportCallback { args });
    true
}

// 0x34e30 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int)
// IDA 0x34e30
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn vtable_assign_tagged_teleport_fn(
    slot: &mut Option<TeleportCallback>,
    args: TeleportArgs,
) -> bool {
    // Tagged `assign_to` overload: same store-and-true (IDA 0x34e30).
    *slot = Some(TeleportCallback { args });
    true
}

// 0x350ec — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
// IDA 0x350ec
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn vtable_assign_functor_teleport_fn(slot: &mut Option<TeleportCallback>, args: TeleportArgs) {
    // Small-object (`mpl::false_`) functor assign: same store (IDA 0x350ec).
    *slot = Some(TeleportCallback { args });
}

// 0x35200 — __ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EclIPFvS4_SsSsSsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
// IDA 0x35200
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::operator()<void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(PlaceLauncher *,std::string,std::string,std::string) &,boost::_bi::list0 &,int)")]
pub fn apply_teleport_list(launcher: &PlaceLauncher, args: &TeleportArgs) {
    // Copies the three bound strings (IDA 0x35230..0x35276), calls
    // `teleportImpl(launcher, s0, s1, s2)` (IDA 0x35288), then destroys the
    // copies (IDA 0x35298..0x35342; Rust drops).
    teleport_impl(launcher, &args.place.clone(), &args.auth.clone(), &args.script.clone());
}

// 0x35438 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE7managerERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
// IDA 0x35438
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn manager_small_teleport_fn(op: FunctorOp, slot: &mut Option<TeleportArgs>) -> bool {
    // Small-object (`mpl::false_`) manager: same clone/destroy (IDA 0x35438).
    manage_boxed_slot(op, slot)
}

/// `boost::detail::thread_data` backing the page-teleport `boost::thread`
/// (IDA 0x2dfac..0x2e284): the copied 5-tuple plus spawn/run counters. The
/// thread itself is a detached `std::thread`, never `boost::thread`.
#[derive(Debug, Default)]
pub struct PageTeleportState {
    spawns: AtomicU32,
    runs: AtomicU32,
    last: parking_lot::Mutex<Option<PageTeleportArgs>>,
}

pub fn shared_page_teleport_state() -> &'static PageTeleportState {
    static STATE: std::sync::LazyLock<PageTeleportState> =
        std::sync::LazyLock::new(PageTeleportState::new);
    &STATE
}

impl PageTeleportState {
    pub fn new() -> Self {
        Self::default()
    }

    // 0x2dc24 — __ZN5boost6threadC2INS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEEOT_
    // IDA 0x2dc24
    #[doc(alias = "boost::thread::thread<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>> &&)")]
    #[doc = "boost::thread::thread<boost::bind page-teleport 5-tuple>"]
    pub fn spawn_thread(&'static self, args: PageTeleportArgs) -> bool {
        // Copies the bind into the heap `thread_data` (IDA 0x2dc48..0x2dcd2;
        // `operator new(0x160)` + `thread_data` ctor), links the shared count
        // (IDA 0x2dcea..0x2dd04), then `start_thread` (IDA 0x2dd44).
        let data = build_page_teleport_data(&args);
        self.spawns.fetch_add(1, Ordering::SeqCst);
        std::thread::spawn(move || {
            run_page_teleport(self, &data);
        });
        true
    }

    pub fn spawns(&self) -> u32 {
        self.spawns.load(Ordering::SeqCst)
    }
    pub fn runs(&self) -> u32 {
        self.runs.load(Ordering::SeqCst)
    }
    pub fn last(&self) -> Option<PageTeleportArgs> {
        self.last.lock().clone()
    }
}

// 0x2ce2c — __ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
// type: int __fastcall(int, int, std::string *, int, std::string *, int, int)
// IDA 0x2ce2c
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>,std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>>(void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>)")]
pub fn bind_page_teleport(
    first: &str,
    second: &str,
    third: &str,
    controller: ObjCId,
    game: u32,
) -> PageTeleportArgs {
    // Captures `(pageTeleport, s0, s1, s2, controller, game)` into the
    // 5-tuple bind (IDA 0x2ce2c).
    PageTeleportArgs {
        first: first.to_owned(),
        second: second.to_owned(),
        third: third.to_owned(),
        controller: Some(controller),
        game: Some(game),
    }
}

// 0x2dfac — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEC2EOSK_
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
// IDA 0x2dfac
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>&&)")]
pub fn build_page_teleport_data(args: &PageTeleportArgs) -> PageTeleportArgs {
    // `thread_data` move-ctor: copies the fn pointer, the three strings, the
    // controller, and the game shared count out of the bind (IDA 0x2dfac).
    args.clone()
}

// 0x2e0f4 — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEED1Ev
// IDA 0x2e0f4
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::~thread_data()")]
pub fn drop_page_teleport_data(args: &PageTeleportArgs) {
    // Non-deleting dtor: releases the strings + game shared count
    // (IDA 0x2e0f4); Rust drops run at scope end.
    let _ = args;
}

// 0x2e1bc — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEED0Ev
// IDA 0x2e1bc
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::~thread_data()")]
pub fn delete_page_teleport_data(args: &PageTeleportArgs) {
    // Deleting dtor: same release plus `operator delete` (IDA 0x2e1bc); the
    // free is the owner drop here.
    drop_page_teleport_data(args);
}

// 0x2e284 — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEE3runEv
// IDA 0x2e284
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::run(void)")]
pub fn run_page_teleport(state: &PageTeleportState, args: &PageTeleportArgs) -> bool {
    // `run` invokes `list5::operator()` at `this + 328` (IDA 0x2e29c): the
    // bound page-teleport fn with `(s0, s1, s2, controller, game)`. The
    // callee lives out of slice; the dispatch is recorded.
    *state.last.lock() = Some(args.clone());
    state.runs.fetch_add(1, Ordering::SeqCst);
    true
}
