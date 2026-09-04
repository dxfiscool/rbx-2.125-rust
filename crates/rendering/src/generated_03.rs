//! rendering next 100 — continuation EA-sorted Ogre|G3D|Gfx|Render (15058 total)
//! This shard: 0x3a8664..0x6d2d64 (100 stubs, 5674 prior filtered stubbed -> 5774, 9384 remaining before -> 9284 after)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, Weak};

/// Virtual `RBX::HandlesBase` view: `shouldRender2d` is vtable slot 36 (IDA 0x3a8664 `(*(this + 144))(this)`).
pub trait ShouldRender2d {
    fn should_render_2d(&self) -> bool;
}

/// Owner embedding the callable subobject 96 bytes past its head.
/// Models IDA `__ZThn96_*` thunks (`(char *)this - 96`, then tail-call).
pub trait Thunk96 {
    type Base: ?Sized;
    fn adjusted_base(&self) -> &Self::Base;
}

/// `boost::function<void(RBX::BillboardGui *, RBX::Adorn *)>` (IDA 0x3c042c).
/// Was boost; now a boxed closure over opaque object addresses.
pub type BillboardRenderFn = Box<dyn FnMut(usize, usize) + Send>;

/// The `+196` render-function slot written by `BillboardGui::setRenderFunction`.
#[derive(Default)]
pub struct BillboardRenderCell {
    pub render_fn: Option<BillboardRenderFn>,
}

/// Host view for `BillboardGui::shouldRender3dSortedAdorn` (IDA 0x3c04a8):
/// the `+212` enable flag plus the `getPart` → `DataModel::get(part, 1)` lookup chain.
pub trait SortedAdornHost {
    fn sorted_adorn_enabled(&self) -> bool;
    fn part_datamodel_hit(&self) -> bool;
}

/// `RBX::RenderStatsCommand` (IDA 0x3f6a90): a `RBX::Verb` named `"RenderStats"`
/// bound to its `DataModel` (vtable `off_11D7E58`, datamodel stored at field `+3`).
pub struct RenderStatsCommand {
    pub name: &'static str,
    pub data_model: usize,
}

/// `GuiItem` reached by `findConstFirstChildByName` + `ClassDescriptor::isA` (IDA 0x3f6be8).
/// `query_visible` is the virtual at vtable `+148`; `toggle_visible` is the
/// `v[132] = query() ^ 1` write at 0x3f6cf8/0x3f6d90.
pub struct GuiItemState {
    pub visible: Cell<bool>,
}

impl GuiItemState {
    pub fn new(visible: bool) -> Self {
        Self { visible: Cell::new(visible) }
    }
    pub fn query_visible(&self) -> bool {
        self.visible.get()
    }
    pub fn toggle_visible(&self) {
        self.visible.set(!self.visible.get());
    }
}

/// World lookup behind `RenderStatsCommand::doIt/isEnabled/isChecked`:
/// `findConstFirstChildByName(*(this + 3) + 2968, name)` + `GuiItem` isA-cast,
/// plus the `FFlag::DebugDisplayFPS` gate (IDA 0x3f6d04).
pub trait RenderStatsWorld {
    fn find_gui_item(&self, name: &str) -> Option<SharedPtr<GuiItemState>>;
    fn debug_display_fps(&self) -> bool;
}

/// Cursor decision inputs decoded from the 0x4252ec branch tree.
#[derive(Clone, Copy)]
pub struct CursorDecision {
    /// Byte at `[a2 + 0xBA8]` (mouse-lock request flag).
    pub mouse_lock_flag: bool,
    /// `[GameBasicSettings + 0x68]` mode word.
    pub settings_mode: u32,
    /// Byte at `[GameBasicSettings + 0x70]`.
    pub settings_feature_flag: bool,
    pub local_player_present: bool,
    pub adv_arrow_tool_enabled: bool,
    pub server_present: bool,
    /// Word at `[a2 + 0xB78]` feeding `Workspace::getCursor`.
    pub workspace_cursor: u32,
}

/// What `DataModel::getRenderMouseCursor` fills into the out `ContentId`.
pub enum MouseCursorContent {
    Assets(&'static str),
    Workspace(u32),
}
/// `RBX::RenderHooksService` (IDA 0x44e308: `operator new(0xAC)` then ctor).
pub struct RenderHooksService {
    /// Raw instance words (`0xAC` bytes = 43 words); layout recovered on demand.
    pub words: [u32; 43],
}

impl RenderHooksService {
    pub fn new() -> Self {
        Self { words: [0; 43] }
    }
}

impl Default for RenderHooksService {
    fn default() -> Self {
        Self::new()
    }
}

/// Service-provider slot operations behind `create<RenderHooksService>` (IDA 0x435a28).
pub trait RenderHooksServiceHost {
    fn find_render_hooks_service(&self) -> Option<SharedPtr<RenderHooksService>>;
    fn create_render_hooks_service(&mut self) -> SharedPtr<RenderHooksService>;
    /// `Instance::setAndLockParent(svc, provider)` (IDA 0x435a96).
    fn lock_service_parent(&mut self, svc: &SharedPtr<RenderHooksService>);
    /// `call_once` class-index init (IDA 0x435ab6..0x435abe).
    fn init_service_class_index(&mut self);
    /// `shared_ptr<Instance>::operator=` into the provider slot table (IDA 0x435ad0).
    fn publish_render_hooks_service(&mut self, svc: &SharedPtr<RenderHooksService>);
    /// `FLog::Asserts` service-map membership check (IDA 0x435af0..0x435b38).
    fn debug_assert_service_registered(&self, name: &str);
}

static CLASS_INDEX_NEXT: AtomicUsize = AtomicUsize::new(1);
/// Class-index cell behind `doGetClassIndex<RenderHooksService>` (IDA 0x44e51c:
/// guard-variable once init via `ServiceProvider::newIndex(1)`).
static RENDER_HOOKS_CLASS_INDEX: LazyLock<usize> =
    LazyLock::new(|| CLASS_INDEX_NEXT.fetch_add(1, Ordering::Relaxed));

/// Name cell behind `Name::doDeclare<sRenderHooksService>` (IDA 0x44e434:
/// `Name::declare(sRenderHooksService, 1)` under `__cxa_guard`).
static RENDER_HOOKS_SERVICE_NAME: LazyLock<&'static str> = LazyLock::new(|| "RenderHooksService");

// 0x3a8664 — __ZNK3RBX11HandlesBase14shouldRender2dEv
#[doc(alias = "RBX::HandlesBase::shouldRender2d(void)const")]
// was: RBX::HandlesBase::shouldRender2d(void)const
// IDA 0x3a8664: virtual dispatch `(*(this + 144))(this)` — vtable slot 36.
pub fn stub_3a8664(target: &dyn ShouldRender2d) -> bool {
    target.should_render_2d()
}

// 0x3a87dc — __ZThn96_NK3RBX11HandlesBase14shouldRender2dEv
#[doc(alias = "non-virtual thunk to RBX::HandlesBase::shouldRender2d(void)const")]
// was: non-virtual thunk to RBX::HandlesBase::shouldRender2d(void)const
// IDA 0x3a87dc: `this - 96` adjustment, then tail-calls shouldRender2d.
pub fn stub_3a87dc<T: Thunk96<Base = dyn ShouldRender2d> + ?Sized>(obj: &T) -> bool {
    stub_3a8664(obj.adjusted_base())
}

// 0x3c042c — __ZN3RBX12BillboardGui17setRenderFunctionEN5boost8functionIFvPS0_PNS_5AdornEEEE
#[doc(alias = "RBX::BillboardGui::setRenderFunction(boost::function<void ()(RBX::BillboardGui*,RBX::Adorn *)>)")]
// was: RBX::BillboardGui::setRenderFunction(boost::function<void ()(RBX::BillboardGui*,RBX::Adorn *)>)
// IDA 0x3c042c: `boost::function::operator=` into the `+196` slot; was boost, now `BillboardRenderFn`.
pub fn stub_3c042c(cell: &mut BillboardRenderCell, render_fn: BillboardRenderFn) {
    cell.render_fn = Some(render_fn);
}

// 0x3c04a8 — __ZNK3RBX12BillboardGui25shouldRender3dSortedAdornEv
#[doc(alias = "RBX::BillboardGui::shouldRender3dSortedAdorn(void)const")]
// was: RBX::BillboardGui::shouldRender3dSortedAdorn(void)const
// IDA 0x3c04a8: `getPart` shared_ptr + `+212` flag; true iff the flag is set and
// IDA 0x3c04a8: `DataModel::get(part, 1) != 0` (0x3c04d8..0x3c0512); temp released on exit.
pub fn stub_3c04a8(host: &dyn SortedAdornHost) -> bool {
    if !host.sorted_adorn_enabled() {
        return false;
    }
    host.part_datamodel_hit()
}

// 0x3c066c — __ZThn96_NK3RBX12BillboardGui25shouldRender3dSortedAdornEv
#[doc(alias = "non-virtual thunk to RBX::BillboardGui::shouldRender3dSortedAdorn(void)const")]
// was: non-virtual thunk to RBX::BillboardGui::shouldRender3dSortedAdorn(void)const
// IDA 0x3c066c: `this - 96` adjustment, then tail-calls shouldRender3dSortedAdorn.
pub fn stub_3c066c<T: Thunk96<Base = dyn SortedAdornHost> + ?Sized>(obj: &T) -> bool {
    stub_3c04a8(obj.adjusted_base())
}

// 0x3f1c00 — __ZNK3RBX13ClickDetector19shouldRender3dAdornEv
#[doc(alias = "RBX::ClickDetector::shouldRender3dAdorn(void)const")]
// was: RBX::ClickDetector::shouldRender3dAdorn(void)const
// IDA 0x3f1c00: `MOVS R0, #1; BX LR` — always renders its 3d adorn.
pub fn stub_3f1c00() -> bool {
    true
}

// 0x3f1c34 — __ZThn92_NK3RBX13ClickDetector19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk to RBX::ClickDetector::shouldRender3dAdorn(void)const")]
// was: non-virtual thunk to RBX::ClickDetector::shouldRender3dAdorn(void)const
// IDA 0x3f1c34: `MOVS R0, #1; BX LR` — thunk body is the same constant.
pub fn stub_3f1c34() -> bool {
    stub_3f1c00()
}

// 0x3f6a8c — __ZN3RBX18RenderStatsCommandC1EPNS_9DataModelE
#[doc(alias = "RBX::RenderStatsCommand::RenderStatsCommand(RBX::DataModel *)")]
// was: RBX::RenderStatsCommand::RenderStatsCommand(RBX::DataModel *)
// IDA 0x3f6a8c: C1 complete-object ctor thunk — tail-calls C2 at 0x3f6a90.
pub fn stub_3f6a8c(data_model: usize) -> RenderStatsCommand {
    stub_3f6a90(data_model)
}

// 0x3f6a90 — __ZN3RBX18RenderStatsCommandC2EPNS_9DataModelE
#[doc(alias = "RBX::RenderStatsCommand::RenderStatsCommand(RBX::DataModel *)")]
// was: RBX::RenderStatsCommand::RenderStatsCommand(RBX::DataModel *)
// IDA 0x3f6a90: `Verb::Verb(this, datamodel ? datamodel + 144 : 0, "RenderStats")`
// IDA 0x3f6a90: (0x3f6af8..0x3f6b02), vtable `off_11D7E58`, datamodel at field `+3`.
pub fn stub_3f6a90(data_model: usize) -> RenderStatsCommand {
    RenderStatsCommand { name: "RenderStats", data_model }
}

// 0x3f6be8 — __ZN3RBX18RenderStatsCommand4doItEPNS_10IDataStateE
#[doc(alias = "RBX::RenderStatsCommand::doIt(RBX::IDataState *)")]
// was: RBX::RenderStatsCommand::doIt(RBX::IDataState *)
// IDA 0x3f6be8: FastLog "Gui:RenderStats" (0x3f6c54), find child "RenderStats", GuiItem
// IDA 0x3f6be8: isA-cast, toggle `v[132] = isVisible() ^ 1` (0x3f6cf8); when
// IDA 0x3f6be8: FFlag::DebugDisplayFPS also toggles the "FPS" child (0x3f6d04..0x3f6d90).
pub fn stub_3f6be8(cmd: &RenderStatsCommand, world: &dyn RenderStatsWorld) {
    let _ = cmd;
    if let Some(item) = world.find_gui_item("RenderStats") {
        item.toggle_visible();
        if world.debug_display_fps() {
            if let Some(fps) = world.find_gui_item("FPS") {
                fps.toggle_visible();
            }
        }
    }
}

// 0x3f6eb0 — __ZNK3RBX18RenderStatsCommand9isEnabledEv
#[doc(alias = "RBX::RenderStatsCommand::isEnabled(void)const")]
// was: RBX::RenderStatsCommand::isEnabled(void)const
// IDA 0x3f6eb0: true iff child "StatsHud1" exists and isA-casts to GuiItem (0x3f6f1c..0x3f6f5a).
pub fn stub_3f6eb0(cmd: &RenderStatsCommand, world: &dyn RenderStatsWorld) -> bool {
    let _ = cmd;
    world.find_gui_item("StatsHud1").is_some()
}

// 0x3f702c — __ZNK3RBX18RenderStatsCommand9isCheckedEv
#[doc(alias = "RBX::RenderStatsCommand::isChecked(void)const")]
// was: RBX::RenderStatsCommand::isChecked(void)const
// IDA 0x3f702c: child "RenderStats" GuiItem-cast; virtual `+148` when found, else 0
// IDA 0x3f702c: (0x3f70f0..0x3f7122).
pub fn stub_3f702c(cmd: &RenderStatsCommand, world: &dyn RenderStatsWorld) -> bool {
    let _ = cmd;
    world
        .find_gui_item("RenderStats")
        .map(|item| item.query_visible())
        .unwrap_or(false)
}

// 0x3fe43c — __ZN3RBX18RenderStatsCommandD1Ev
#[doc(alias = "RBX::RenderStatsCommand::~RenderStatsCommand()")]
// was: RBX::RenderStatsCommand::~RenderStatsCommand()
// IDA 0x3fe43c: D1 thunk — `RBX::Verb::~Verb(this)`; storage retained, so by-value drop.
pub fn stub_3fe43c(cmd: RenderStatsCommand) {
    drop(cmd);
}

// 0x3fe440 — __ZN3RBX18RenderStatsCommandD0Ev
#[doc(alias = "RBX::RenderStatsCommand::~RenderStatsCommand()")]
// was: RBX::RenderStatsCommand::~RenderStatsCommand()
// IDA 0x3fe440: D0 deleting dtor — `Verb::~Verb` + `operator delete` (0x3fe490..0x3fe496);
// IDA 0x3fe440: boxed drop runs the dtor and frees the storage.
pub fn stub_3fe440(cmd: Box<RenderStatsCommand>) {
    drop(cmd);
}

// 0x4252ec — __ZN3RBX9DataModel20getRenderMouseCursorEv
#[doc(alias = "RBX::DataModel::getRenderMouseCursor(void)")]
// was: RBX::DataModel::getRenderMouseCursor(void)
// IDA 0x4252ec: fills the out ContentId — MouseLocked when mode == 1, feature flag set
// IDA 0x4252ec: and a local player exists; advCursor when adv-arrow tooling applies without a
// IDA 0x4252ec: local player or server; plain arrow otherwise; Workspace::getCursor off the
// IDA 0x4252ec: mouse-lock path (0x4252fe..0x425386).
pub fn stub_4252ec(ctx: &CursorDecision) -> MouseCursorContent {
    if ctx.mouse_lock_flag {
        if ctx.settings_mode == 1 && ctx.settings_feature_flag && ctx.local_player_present {
            return MouseCursorContent::Assets("Textures/MouseLockedCursor.png");
        }
        if ctx.adv_arrow_tool_enabled && !ctx.local_player_present && !ctx.server_present {
            return MouseCursorContent::Assets("Textures/advCursor-default.png");
        }
        return MouseCursorContent::Assets("Textures/ArrowCursor.png");
    }
    if ctx.settings_mode == 1 && ctx.settings_feature_flag && ctx.local_player_present {
        return MouseCursorContent::Assets("Textures/MouseLockedCursor.png");
    }
    MouseCursorContent::Workspace(ctx.workspace_cursor)
}

// 0x435a28 — __ZNK3RBX15ServiceProvider6createINS_18RenderHooksServiceEEEPT_v
#[doc(alias = "RBX::RenderHooksService * RBX::ServiceProvider::create<RBX::RenderHooksService>(void)const")]
// was: RBX::RenderHooksService * RBX::ServiceProvider::create<RBX::RenderHooksService>(void)const
// IDA 0x435a28: `find` fast path (0x435a4e); else `Creatable::create` + `setAndLockParent`
// IDA 0x435a28: (0x435a86..0x435a96), `call_once` class-index init, `operator=` into the
// IDA 0x435a28: provider slot table, and the `FLog::Asserts` service-map check
// IDA 0x435a28: (0x435ab6..0x435b38); temp released before returning the lookup.
pub fn stub_435a28(host: &mut dyn RenderHooksServiceHost) -> Option<SharedPtr<RenderHooksService>> {
    if let Some(existing) = host.find_render_hooks_service() {
        return Some(existing);
    }
    let svc = host.create_render_hooks_service();
    host.lock_service_parent(&svc);
    host.init_service_class_index();
    host.publish_render_hooks_service(&svc);
    host.debug_assert_service_registered("RenderHooksService");
    host.find_render_hooks_service()
}

// 0x44e308 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18RenderHooksServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::RenderHooksService> RBX::Creatable<RBX::Instance>::create<RBX::RenderHooksService>(void)")]
// was: boost::shared_ptr<RBX::RenderHooksService> RBX::Creatable<RBX::Instance>::create<RBX::RenderHooksService>(void)
// IDA 0x44e308: `operator new(0xAC)` + `RenderHooksService` ctor, wrapped in a
// IDA 0x44e308: `shared_ptr` with `Creatable<Instance>::Deleter` (0x44e33c..0x44e36e).
// IDA 0x44e308: Was boost::shared_ptr; now rbx_core::SharedPtr (Arc — deleter is Drop).
pub fn stub_44e308() -> SharedPtr<RenderHooksService> {
    SharedPtr::new(RenderHooksService::new())
}

// 0x44e3b8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_18RenderHooksServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::RenderHooksService>(rbx_core::SharedPtr<RBX::RenderHooksService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::RenderHooksService>(boost::shared_ptr<RBX::RenderHooksService> const&)
// IDA 0x44e3b8: templated `operator=<RenderHooksService>` — copy-and-swap of the
// IDA 0x44e3b8: shared_count (addref new, swap pointers, release old). The cross-type
// IDA 0x44e3b8: upcast is a runtime no-op in Rust; the Arc clone is the whole effect.
pub fn stub_44e3b8(dst: &mut SharedPtr<RenderHooksService>, src: &SharedPtr<RenderHooksService>) {
    *dst = SharedPtr::clone(src);
}

// 0x44e3ec — __ZN3RBX4Name7declareILZNS_19sRenderHooksServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sRenderHooksServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_19sRenderHooksServiceEEEERKS0_v
// IDA 0x44e3ec: null class string → `getNullName()` (0x44e426); else `call_once`
// IDA 0x44e3ec: `callDoDeclare` and return the declared name (0x44e402..0x44e422).
pub fn stub_44e3ec(class_name: Option<&'static str>) -> &'static str {
    match class_name {
        Some(_) => stub_44e430(),
        None => "",
    }
}

// 0x44e430 — __ZN3RBX4Name13callDoDeclareILZNS_19sRenderHooksServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sRenderHooksServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_19sRenderHooksServiceEEEEvv
// IDA 0x44e430: thunk — tail-calls the `doDeclare` shim.
pub fn stub_44e430() -> &'static str {
    stub_44e434()
}

// 0x44e434 — __ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v
// IDA 0x44e434: `Name::declare(sRenderHooksService, 1)` under a `__cxa_guard` once-init
// IDA 0x44e434: (0x44e490..0x44e4bc); the `LazyLock` cell below is the Rust once-guard.
pub fn stub_44e434() -> &'static str {
    *RENDER_HOOKS_SERVICE_NAME
}

// 0x44e518 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_18RenderHooksServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::RenderHooksService>(void)")]
// was: void RBX::ServiceProvider::callDoGetClassIndex<RBX::RenderHooksService>(void)
// IDA 0x44e518: thunk — tail-calls `doGetClassIndex<RenderHooksService>`.
pub fn stub_44e518() -> usize {
    stub_44e51c()
}

// 0x44e51c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18RenderHooksServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RenderHooksService>(void)")]
// was: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RenderHooksService>(void)
// IDA 0x44e51c: guard-variable once init via `ServiceProvider::newIndex(1)`
// IDA 0x44e51c: (0x44e578..0x44e598); the `LazyLock` cell below is the Rust once-guard.
pub fn stub_44e51c() -> usize {
    *RENDER_HOOKS_CLASS_INDEX
}

// 0x44e5f4 — __ZN5boost10shared_ptrIN3RBX18RenderHooksServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RenderHooksService>::shared_ptr<RBX::RenderHooksService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::RenderHooksService>::shared_ptr<RBX::RenderHooksService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter)
// IDA 0x44e5f4: `shared_ptr(ptr, deleter)` — stores the pointer, builds the
// IDA 0x44e5f4: `shared_count`, then `_internal_accept_owner` when non-null
// IDA 0x44e5f4: (0x44e614..0x44e65a). Was boost; the Arc control block subsumes the
// IDA 0x44e5f4: count, and Drop is the deleter.
pub fn stub_44e5f4(service: Box<RenderHooksService>) -> SharedPtr<RenderHooksService> {
    SharedPtr::new(*service)
}

// 0x44e6bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18RenderHooksServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RenderHooksService,RBX::RenderHooksService>(rbx_core::SharedPtr<RBX::RenderHooksService> const*,RBX::RenderHooksService *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RenderHooksService,RBX::RenderHooksService>(boost::shared_ptr<RBX::RenderHooksService> const*,RBX::RenderHooksService *)const
// IDA 0x44e6bc: links the weak back-pointer only when the weak use_count is zero
// IDA 0x44e6bc: (0x44e6e4..0x44e74c); the `+36` owner fixup folds into the Arc downgrade.
pub fn stub_44e6bc(
    owner: &mut Option<Weak<RenderHooksService>>,
    shared: &SharedPtr<RenderHooksService>,
) {
    if owner.as_ref().and_then(|weak| weak.upgrade()).is_none() {
        *owner = Some(SharedPtr::downgrade(shared));
    }
}

// 0x44e7a8 — __ZN5boost6detail12shared_countC2IPN3RBX18RenderHooksServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter)
// IDA 0x44e7a8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44e7a8() {
}

// 0x44e8b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18RenderHooksServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// IDA 0x44e8b0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_44e8b0() {
}

// 0x44e8b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18RenderHooksServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// IDA 0x44e8b4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44e8b4() {
}

// 0x44e8b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18RenderHooksServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// IDA 0x44e8b8: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44e8b8() {
}

// 0x44e8d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18RenderHooksServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// IDA 0x44e8d8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44e8d8() {
}

// 0x44e8f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18RenderHooksServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// IDA 0x44e8f0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44e8f0() {
}

// 0x4a1350 — __ZNK3RBX9Explosion19shouldRender3dAdornEv
#[doc(alias = "RBX::Explosion::shouldRender3dAdorn(void)const")]
// was: RBX::Explosion::shouldRender3dAdorn(void)const
// IDA 0x4a1350: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a1350() {
}

// 0x4a1368 — __ZThn92_NK3RBX9Explosion19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk to RBX::Explosion::shouldRender3dAdorn(void)const")]
// was: non-virtual thunk to RBX::Explosion::shouldRender3dAdorn(void)const
// IDA 0x4a1368: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a1368() {
}

// 0x4a6868 — __ZNK3RBX10IAdornable19shouldRender3dAdornEv
#[doc(alias = "RBX::IAdornable::shouldRender3dAdorn(void)const")]
// was: RBX::IAdornable::shouldRender3dAdorn(void)const
// IDA 0x4a6868: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6868() {
}

// 0x4bcbac — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting> const>::initSingleton(void)
// IDA 0x4bcbac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4bcbac() {
}

// 0x4bcbb0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting> const>::doGetSingleton(void)
// IDA 0x4bcbb0: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bcbb0() {
}

// 0x4bcca0 — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()
// IDA 0x4bcca0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4bcca0() {
}

// 0x4bcca4 — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()
// IDA 0x4bcca4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4bcca4() {
}

// 0x4bce78 — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()
// IDA 0x4bce78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4bce78() {
}

// 0x4bcf18 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::lookup(char const*)const
// IDA 0x4bcf18: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bcf18() {
}

// 0x4bcf48 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4bcf48: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bcf48() {
}

// 0x4bcf68 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4bcf68: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bcf68() {
}

// 0x4bcfc4 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToString(unsigned long,std::string &)const
// IDA 0x4bcfc4: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bcfc4() {
}

// 0x4bd108 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToString(RBX::GameBasicSettings::RenderQualitySetting const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToString(RBX::GameBasicSettings::RenderQualitySetting const&)const
// IDA 0x4bd108: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bd108() {
}

// 0x4bd2a8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17GameBasicSettings20RenderQualitySettingEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameBasicSettings::RenderQualitySetting>(RBX::GameBasicSettings::RenderQualitySetting const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameBasicSettings::RenderQualitySetting>(RBX::GameBasicSettings::RenderQualitySetting const&)
// IDA 0x4bd2a8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bd2a8() {
}

// 0x4bd2f8 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::singleton(void)
// IDA 0x4bd2f8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bd2f8() {
}

// 0x4bd364 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::construct_func(char const*,char *)
// IDA 0x4bd364: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bd364() {
}

// 0x4bd370 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::destruct_func(char *)
// IDA 0x4bd370: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4bd370() {
}

// 0x4bd374 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToItem(RBX::GameBasicSettings::RenderQualitySetting const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToItem(RBX::GameBasicSettings::RenderQualitySetting const&)const
// IDA 0x4bd374: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bd374() {
}

// 0x4bd440 — __ZN3rbx8any_castIRKN3RBX17GameBasicSettings20RenderQualitySettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::GameBasicSettings::RenderQualitySetting const& rbx::any_cast<RBX::GameBasicSettings::RenderQualitySetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::GameBasicSettings::RenderQualitySetting const& rbx::any_cast<RBX::GameBasicSettings::RenderQualitySetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4bd440: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bd440() {
}

// 0x4bd530 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToValue(RBX::Name const&,RBX::GameBasicSettings::RenderQualitySetting&)const")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToValue(RBX::Name const&,RBX::GameBasicSettings::RenderQualitySetting&)const
// IDA 0x4bd530: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bd530() {
}

// 0x4bd5ac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>> *)
// IDA 0x4bd5ac: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bd5ac() {
}

// 0x4dcbd8 — __ZN3RBX10Reflection9DescribedINS_18RenderHooksServiceELZNS_19sRenderHooksServiceEENS_17NonFactoryProductINS_8InstanceELZNS_19sRenderHooksServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18RenderHooksServiceELZNS_19sRenderHooksServiceEENS_17NonFactoryProductINS_8InstanceELZNS_19sRenderHooksServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_18RenderHooksServiceELZNS_19sRenderHooksServiceEENS_17NonFactoryProductINS_8InstanceELZNS_19sRenderHooksServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x4dcbd8: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4dcbd8() {
}

// 0x4e7160 — __ZNK3RBX7Feature19shouldRender3dAdornEv
#[doc(alias = "RBX::Feature::shouldRender3dAdorn(void)const")]
// was: RBX::Feature::shouldRender3dAdorn(void)const
// IDA 0x4e7160: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4e7160() {
}

// 0x4e7190 — __ZThn92_NK3RBX7Feature19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk to RBX::Feature::shouldRender3dAdorn(void)const")]
// was: non-virtual thunk to RBX::Feature::shouldRender3dAdorn(void)const
// IDA 0x4e7190: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4e7190() {
}

// 0x4f8834 — __ZNK3RBX10ForceField19shouldRender3dAdornEv
#[doc(alias = "RBX::ForceField::shouldRender3dAdorn(void)const")]
// was: RBX::ForceField::shouldRender3dAdorn(void)const
// IDA 0x4f8834: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f8834() {
}

// 0x4f8868 — __ZThn92_NK3RBX10ForceField19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk to RBX::ForceField::shouldRender3dAdorn(void)const")]
// was: non-virtual thunk to RBX::ForceField::shouldRender3dAdorn(void)const
// IDA 0x4f8868: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f8868() {
}

// 0x5156a4 — __ZN3RBX10GuiBuilder16buildRenderStatsEv
#[doc(alias = "RBX::GuiBuilder::buildRenderStats(void)")]
// was: RBX::GuiBuilder::buildRenderStats(void)
// IDA 0x5156a4: 1196 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5156a4() {
}

// 0x52986c — __ZNK3RBX9GuiObject25getRenderBackgroundColor4Ev
#[doc(alias = "RBX::GuiObject::getRenderBackgroundColor4(void)const")]
// was: RBX::GuiObject::getRenderBackgroundColor4(void)const
// IDA 0x52986c: 18 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52986c() {
}

// 0x59fad8 — __ZNK3RBX13JointInstance19shouldRender3dAdornEv
#[doc(alias = "RBX::JointInstance::shouldRender3dAdorn(void)const")]
// was: RBX::JointInstance::shouldRender3dAdorn(void)const
// IDA 0x59fad8: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_59fad8() {
}

// 0x59fae8 — __ZThn92_NK3RBX13JointInstance19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk to RBX::JointInstance::shouldRender3dAdorn(void)const")]
// was: non-virtual thunk to RBX::JointInstance::shouldRender3dAdorn(void)const
// IDA 0x59fae8: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_59fae8() {
}

// 0x5a3aec — __ZNK3RBX10ManualWeld19shouldRender3dAdornEv
#[doc(alias = "RBX::ManualWeld::shouldRender3dAdorn(void)const")]
// was: RBX::ManualWeld::shouldRender3dAdorn(void)const
// IDA 0x5a3aec: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a3aec() {
}

// 0x5a3c58 — __ZThn92_NK3RBX10ManualWeld19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk to RBX::ManualWeld::shouldRender3dAdorn(void)const")]
// was: non-virtual thunk to RBX::ManualWeld::shouldRender3dAdorn(void)const
// IDA 0x5a3c58: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a3c58() {
}

// 0x5a3d10 — __ZNK3RBX10ManualGlue19shouldRender3dAdornEv
#[doc(alias = "RBX::ManualGlue::shouldRender3dAdorn(void)const")]
// was: RBX::ManualGlue::shouldRender3dAdorn(void)const
// IDA 0x5a3d10: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a3d10() {
}

// 0x5a3e7c — __ZThn92_NK3RBX10ManualGlue19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk to RBX::ManualGlue::shouldRender3dAdorn(void)const")]
// was: non-virtual thunk to RBX::ManualGlue::shouldRender3dAdorn(void)const
// IDA 0x5a3e7c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a3e7c() {
}

// 0x5c8a7c — __ZNK3RBX7Message14shouldRender2dEv
#[doc(alias = "RBX::Message::shouldRender2d(void)const")]
// was: RBX::Message::shouldRender2d(void)const
// IDA 0x5c8a7c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c8a7c() {
}

// 0x5c8e84 — __ZThn92_NK3RBX7Message14shouldRender2dEv
#[doc(alias = "non-virtual thunk to RBX::Message::shouldRender2d(void)const")]
// was: non-virtual thunk to RBX::Message::shouldRender2d(void)const
// IDA 0x5c8e84: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c8e84() {
}

// 0x5cd254 — __ZNK3RBX13ModelInstance19shouldRender3dAdornEv
#[doc(alias = "RBX::ModelInstance::shouldRender3dAdorn(void)const")]
// was: RBX::ModelInstance::shouldRender3dAdorn(void)const
// IDA 0x5cd254: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cd254() {
}

// 0x5cd274 — __ZThn96_NK3RBX13ModelInstance19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk to RBX::ModelInstance::shouldRender3dAdorn(void)const")]
// was: non-virtual thunk to RBX::ModelInstance::shouldRender3dAdorn(void)const
// IDA 0x5cd274: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cd274() {
}

// 0x5d9d60 — __ZNK3RBX12PartInstance27getRenderingCoordinateFrameEv
#[doc(alias = "RBX::PartInstance::getRenderingCoordinateFrame(void)const")]
// was: RBX::PartInstance::getRenderingCoordinateFrame(void)const
// IDA 0x5d9d60: 51 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d9d60() {
}

// 0x5db794 — __ZN3RBX12PartInstance28calcRenderingCoordinateFrameEv
#[doc(alias = "RBX::PartInstance::calcRenderingCoordinateFrame(void)")]
// was: RBX::PartInstance::calcRenderingCoordinateFrame(void)
// IDA 0x5db794: 54 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5db794() {
}

// 0x5dbcfc — __ZNK3RBX12PartInstance19shouldRender3dAdornEv
#[doc(alias = "RBX::PartInstance::shouldRender3dAdorn(void)const")]
// was: RBX::PartInstance::shouldRender3dAdorn(void)const
// IDA 0x5dbcfc: 107 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5dbcfc() {
}

// 0x5dbe44 — __ZThn108_NK3RBX12PartInstance19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk to RBX::PartInstance::shouldRender3dAdorn(void)const")]
// was: non-virtual thunk to RBX::PartInstance::shouldRender3dAdorn(void)const
// IDA 0x5dbe44: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5dbe44() {
}

// 0x5dd96c — __ZN3RBX12PartInstance31computeRenderingCoordinateFrameEPS0_
#[doc(alias = "RBX::PartInstance::computeRenderingCoordinateFrame(RBX::PartInstance*)")]
// was: RBX::PartInstance::computeRenderingCoordinateFrame(RBX::PartInstance*)
// IDA 0x5dd96c: 230 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5dd96c() {
}

// 0x5ddc40 — __ZNK3RBX12PartInstance39getLastComputedRenderingCoordinateFrameEPKS0_
#[doc(alias = "RBX::PartInstance::getLastComputedRenderingCoordinateFrame(RBX::PartInstance const*)const")]
// was: RBX::PartInstance::getLastComputedRenderingCoordinateFrame(RBX::PartInstance const*)const
// IDA 0x5ddc40: 205 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ddc40() {
}

// 0x5dea38 — __ZN3RBX12PartInstance17setRenderMaterialENS_8MaterialE
#[doc(alias = "RBX::PartInstance::setRenderMaterial(RBX::Material)")]
// was: RBX::PartInstance::setRenderMaterial(RBX::Material)
// IDA 0x5dea38: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5dea38() {
}

// 0x5e0dbc — __ZNK3RBX12PartInstance17getRenderMaterialEv
#[doc(alias = "RBX::PartInstance::getRenderMaterial(void)const")]
// was: RBX::PartInstance::getRenderMaterial(void)const
// IDA 0x5e0dbc: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e0dbc() {
}

// 0x5e2b1c — __ZN3RBX12PartInstance17getRenderLocationEv
#[doc(alias = "RBX::PartInstance::getRenderLocation(void)")]
// was: RBX::PartInstance::getRenderLocation(void)
// IDA 0x5e2b1c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2b1c() {
}

// 0x5e2b28 — __ZN3RBX12PartInstance13getRenderSizeEv
#[doc(alias = "RBX::PartInstance::getRenderSize(void)")]
// was: RBX::PartInstance::getRenderSize(void)
// IDA 0x5e2b28: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2b28() {
}

// 0x5e2b98 — __ZNK3RBX10IAdornable14shouldRender2dEv
#[doc(alias = "RBX::IAdornable::shouldRender2d(void)const")]
// was: RBX::IAdornable::shouldRender2d(void)const
// IDA 0x5e2b98: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2b98() {
}

// 0x5e2c08 — __ZThn132_N3RBX12PartInstance17getRenderLocationEv
#[doc(alias = "non-virtual thunk to RBX::PartInstance::getRenderLocation(void)")]
// was: non-virtual thunk to RBX::PartInstance::getRenderLocation(void)
// IDA 0x5e2c08: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2c08() {
}

// 0x5e2c14 — __ZThn132_N3RBX12PartInstance13getRenderSizeEv
#[doc(alias = "non-virtual thunk to RBX::PartInstance::getRenderSize(void)")]
// was: non-virtual thunk to RBX::PartInstance::getRenderSize(void)
// IDA 0x5e2c14: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e2c14() {
}

// 0x61054c — __ZNK3RBX9ScreenGui23isAncestorRenderableGuiEv
#[doc(alias = "RBX::ScreenGui::isAncestorRenderableGui(void)const")]
// was: RBX::ScreenGui::isAncestorRenderableGui(void)const
// IDA 0x61054c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61054c() {
}

// 0x610f4c — __ZNK3RBX9ScreenGui14shouldRender2dEv
#[doc(alias = "RBX::ScreenGui::shouldRender2d(void)const")]
// was: RBX::ScreenGui::shouldRender2d(void)const
// IDA 0x610f4c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_610f4c() {
}

// 0x61149c — __ZThn96_NK3RBX9ScreenGui14shouldRender2dEv
#[doc(alias = "non-virtual thunk to RBX::ScreenGui::shouldRender2d(void)const")]
// was: non-virtual thunk to RBX::ScreenGui::shouldRender2d(void)const
// IDA 0x61149c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61149c() {
}

// 0x61ee4c — __ZNK3RBX14SelectionLasso19shouldRender3dAdornEv
#[doc(alias = "RBX::SelectionLasso::shouldRender3dAdorn(void)const")]
// was: RBX::SelectionLasso::shouldRender3dAdorn(void)const
// IDA 0x61ee4c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61ee4c() {
}

// 0x61ee7c — __ZThn96_NK3RBX14SelectionLasso19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk to RBX::SelectionLasso::shouldRender3dAdorn(void)const")]
// was: non-virtual thunk to RBX::SelectionLasso::shouldRender3dAdorn(void)const
// IDA 0x61ee7c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61ee7c() {
}

// 0x61f4b4 — __ZNK3RBX18SelectionPartLasso19shouldRender3dAdornEv
#[doc(alias = "RBX::SelectionPartLasso::shouldRender3dAdorn(void)const")]
// was: RBX::SelectionPartLasso::shouldRender3dAdorn(void)const
// IDA 0x61f4b4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61f4b4() {
}

// 0x61f4e4 — __ZThn96_NK3RBX18SelectionPartLasso19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk to RBX::SelectionPartLasso::shouldRender3dAdorn(void)const")]
// was: non-virtual thunk to RBX::SelectionPartLasso::shouldRender3dAdorn(void)const
// IDA 0x61f4e4: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61f4e4() {
}

// 0x6290fc — __ZNK3RBX18SkateboardPlatform14shouldRender2dEv
#[doc(alias = "RBX::SkateboardPlatform::shouldRender2d(void)const")]
// was: RBX::SkateboardPlatform::shouldRender2d(void)const
// IDA 0x6290fc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6290fc() {
}

// 0x629100 — __ZThn108_NK3RBX18SkateboardPlatform14shouldRender2dEv
#[doc(alias = "non-virtual thunk to RBX::SkateboardPlatform::shouldRender2d(void)const")]
// was: non-virtual thunk to RBX::SkateboardPlatform::shouldRender2d(void)const
// IDA 0x629100: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_629100() {
}

// 0x668d28 — __ZNK3RBX9GuiBase2d14shouldRender2dEv
#[doc(alias = "RBX::GuiBase2d::shouldRender2d(void)const")]
// was: RBX::GuiBase2d::shouldRender2d(void)const
// IDA 0x668d28: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_668d28() {
}

// 0x668ef8 — __ZThn96_NK3RBX9GuiBase2d14shouldRender2dEv
#[doc(alias = "non-virtual thunk to RBX::GuiBase2d::shouldRender2d(void)const")]
// was: non-virtual thunk to RBX::GuiBase2d::shouldRender2d(void)const
// IDA 0x668ef8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_668ef8() {
}

// 0x6bd00c — __ZNK3RBX11VehicleSeat14shouldRender2dEv
#[doc(alias = "RBX::VehicleSeat::shouldRender2d(void)const")]
// was: RBX::VehicleSeat::shouldRender2d(void)const
// IDA 0x6bd00c: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6bd00c() {
}

// 0x6bd020 — __ZThn108_NK3RBX11VehicleSeat14shouldRender2dEv
#[doc(alias = "non-virtual thunk to RBX::VehicleSeat::shouldRender2d(void)const")]
// was: non-virtual thunk to RBX::VehicleSeat::shouldRender2d(void)const
// IDA 0x6bd020: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6bd020() {
}

// 0x6cd8e0 — __ZN3RBX9Workspace27selectAllTopLevelRenderableEv
#[doc(alias = "RBX::Workspace::selectAllTopLevelRenderable(void)")]
// was: RBX::Workspace::selectAllTopLevelRenderable(void)
// IDA 0x6cd8e0: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6cd8e0() {
}

// 0x6d2cfc — __ZN3RBX13ModelInstance17getRenderLocationEv
#[doc(alias = "RBX::ModelInstance::getRenderLocation(void)")]
// was: RBX::ModelInstance::getRenderLocation(void)
// IDA 0x6d2cfc: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6d2cfc() {
}

// 0x6d2d0c — __ZN3RBX13ModelInstance13getRenderSizeEv
#[doc(alias = "RBX::ModelInstance::getRenderSize(void)")]
// was: RBX::ModelInstance::getRenderSize(void)
// IDA 0x6d2d0c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6d2d0c() {
}

// 0x6d2d48 — __ZNK3RBX10IAdornable25shouldRender3dSortedAdornEv
#[doc(alias = "RBX::IAdornable::shouldRender3dSortedAdorn(void)const")]
// was: RBX::IAdornable::shouldRender3dSortedAdorn(void)const
// IDA 0x6d2d48: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6d2d48() {
}

// 0x6d2d64 — __ZThn120_N3RBX13ModelInstance17getRenderLocationEv
#[doc(alias = "non-virtual thunk to RBX::ModelInstance::getRenderLocation(void)")]
// was: non-virtual thunk to RBX::ModelInstance::getRenderLocation(void)
// IDA 0x6d2d64: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6d2d64() {
}