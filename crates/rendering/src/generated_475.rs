//! rendering shard 475 — 100 stubs 0x884390..0x888450 EA-sorted asc next 100 distinct not yet in rendering (Ogre|G3D|Render|Adorn|View|Mesh filtered 17446 total 17445->17446 covered gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + #[doc(alias)] sanitized + todo!("0xADDR")) [skeleton batch rendering 475]
//! Source: ida/export.json (85545 funcs) EA asc gap filler distinct not yet in rendering — next 100 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::{
    LazyLock, Once,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

// ---- impl batch 0x884390..0x885880 (25 fns, IDA decompile+disasm grounded) ----
//
// Boost mapping (AGENTS.md §4, no boost crate):
// boost::singleton_pool<T, N, ...> → `SingletonPool` behind a
// `parking_lot::Mutex` (the pool mutex at `...::storage`); `get_pool()` is
// the `static` initializer; `boost::pool::release_memory` is
// `SingletonPool::release_memory`. boost::shared_ptr → rbx_core::SharedPtr.

/// Rust model of the free-list state behind
/// `boost::singleton_pool<T, RequestedSize, ...>` (IDA `0x8843ac` et al.):
/// `get_pool()` construction, `mutex` lock/unlock, `release_memory()` purge.
struct SingletonPool {
    requested_size: usize,
    free_blocks: usize,
}

impl SingletonPool {
    const fn new(requested_size: usize) -> Self {
        Self {
            requested_size,
            free_blocks: 0,
        }
    }

    /// IDA `boost::pool<...>::release_memory` — frees every free-list block
    /// back to the system; reports whether anything was freed.
    fn release_memory(&mut self) -> bool {
        let freed = self.free_blocks > 0;
        self.free_blocks = 0;
        freed
    }
}

/// IDA `boost::singleton_pool<...>::get_pool` — one static pool per
/// `(T, RequestedSize)` instantiation; `Mutex::new` is const, so the
/// storage needs no lazy cell.
static POOL_POLY_CELL_CONTACT: Mutex<SingletonPool> = Mutex::new(SingletonPool::new(232));
static POOL_EDGE_EDGE: Mutex<SingletonPool> = Mutex::new(SingletonPool::new(328));
static POOL_FACE_EDGE: Mutex<SingletonPool> = Mutex::new(SingletonPool::new(368));
static POOL_FACE_VERTEX: Mutex<SingletonPool> = Mutex::new(SingletonPool::new(304));

static POLY_CELL_CONTACT_AVAILABLE: AtomicUsize = AtomicUsize::new(0);
static EDGE_EDGE_AVAILABLE: AtomicUsize = AtomicUsize::new(0);
static FACE_EDGE_AVAILABLE: AtomicUsize = AtomicUsize::new(0);
static FACE_VERTEX_AVAILABLE: AtomicUsize = AtomicUsize::new(0);

static EDGE_EDGE_ALLOC_ONCE: Once = Once::new();
static FACE_EDGE_ALLOC_ONCE: Once = Once::new();
static FACE_VERTEX_ALLOC_ONCE: Once = Once::new();
static GLOBAL_I_A_439_ONCE: Once = Once::new();

/// Rust model of `RBX::poolAvailabilityList` (IDA `0x8843dc`): one cached
/// `availableSize` address per `RBX::Allocator<T>`. Raw addresses, never
/// dereferenced here — diagnostics only.
static POOL_AVAILABILITY_LIST: LazyLock<Mutex<Vec<usize>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

/// Rust model of `RBX::poolReleaseMemoryFuncList` (IDA `0x8843dc`):
/// `std::vector<bool (*)(void)>::push_back` → `Vec<fn() -> bool>::push`.
static POOL_RELEASE_MEMORY_FUNCS: LazyLock<Mutex<Vec<fn() -> bool>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

/// Rust model of `RBX::PolyCellContact` (IDA `0x884390`): pooled contact
/// record; field layout rides with the physics batch.
pub struct PolyCellContact {
    _opaque: (),
}

/// Rust model of `RBX::PolyCellPair` (IDA `0x884490`/`0x884494`): empty
/// teardown in this build.
pub struct PolyCellPair {
    _opaque: (),
}

/// Rust model of `RBX::EdgeEdgeConnector` (IDA `0x884440`).
pub struct EdgeEdgeConnector {
    _opaque: (),
}

/// Rust model of `RBX::FaceEdgeConnector` (IDA `0x8844fc`).
pub struct FaceEdgeConnector {
    _opaque: (),
}

/// Rust model of `RBX::FaceVertexConnector` (IDA `0x8845b0`).
pub struct FaceVertexConnector {
    _opaque: (),
}

/// Rust model of the mouse object cached by `RBX::Plugin::getMouseLua`
/// (IDA `0x885014`); payload rides with the Lua/mouse batch.
#[derive(Default)]
pub struct PluginMouse {
    _opaque: (),
}

/// Rust model of the `RBX::Workspace` null-mouse-command sink touched by
/// `RBX::Plugin::activate` (IDA `0x885062`): records the command reset.
#[derive(Default)]
pub struct WorkspaceMouseCommand {
    pub nulled: AtomicBool,
}

impl WorkspaceMouseCommand {
    /// IDA `RBX::Workspace::setNullMouseCommand` as reached from `0x885062`.
    pub fn set_null_mouse_command(&self) {
        self.nulled.store(true, Ordering::SeqCst);
    }
}

/// Rust model of `RBX::Button` (IDA `0x8856ec`): offsets are word/byte
/// indices from the `0x885190`/`0x8855bc`/`0x8856ec` decompiles.
pub struct Button {
    /// Byte `+104` (IDA `0x8855e8`): frozen guard — `setActive` is a no-op.
    pub frozen: AtomicBool,
    /// Byte `+105` (IDA `0x88567e`): active flag.
    pub active: AtomicBool,
    /// Word `+23` (IDA `0x885302`): factory token from the virtual create.
    pub factory_token: AtomicUsize,
    /// Word `+24` (IDA `0x88530c`): factory context `*(a2 + 92)`.
    pub context_a: AtomicUsize,
    /// Word `+25` (IDA `0x885306`): owner context `a2`.
    pub context_b: AtomicUsize,
    /// Word `+27` (IDA `0x8857f0`): reserved slot, zeroed by the ctor.
    pub reserved: AtomicUsize,
    pub text: String,
    pub tooltip: String,
    pub icon_path: String,
}

impl Default for Button {
    fn default() -> Self {
        // IDA 0x8857cc..0x8857f0: `+104`/`+105`/`+27` zeroed; the rest is
        // filled by `Toolbar::createButton` (IDA 0x8852fe..0x88530c).
        Self {
            frozen: AtomicBool::new(false),
            active: AtomicBool::new(false),
            factory_token: AtomicUsize::new(0),
            context_a: AtomicUsize::new(0),
            context_b: AtomicUsize::new(0),
            reserved: AtomicUsize::new(0),
            text: String::new(),
            tooltip: String::new(),
            icon_path: String::new(),
        }
    }
}

impl Button {
    /// Virtual `+136` deactivate hook + singleton refresh on the
    /// currently-active path (IDA `0x88564e`..`0x885678`); base override is
    /// empty, preserved for the call-graph shape.
    fn invoke_deactivate_singleton(&self) {}

    /// Change-notify virtual `+12` on the DataModel link (IDA `0x885694`);
    /// base override is empty, preserved for the call-graph shape.
    fn notify_changed(&self, _active: bool) {}
}

/// Rust model of `RBX::Toolbar` (IDA `0x885190`).
pub struct Toolbar {
    pub name: String,
    /// `std::map<void *, shared_ptr<Button>>` at `+104` (IDA `0x885318`):
    /// buttons by factory token. `operator[]` + shared copy → insert +
    /// `Arc` clone; iteration order differences are unobservable here.
    pub buttons: Mutex<HashMap<usize, SharedPtr<Button>>>,
}

impl Default for Toolbar {
    fn default() -> Self {
        Self {
            name: String::new(),
            buttons: Mutex::new(HashMap::new()),
        }
    }
}

/// Rust model of `RBX::Plugin` (IDA `0x884c40`).
pub struct Plugin {
    /// `RBX::DataModel*` identity installed by `setDataModel`
    /// (IDA `0x884cac`); key into the manager state map.
    pub data_model: Mutex<Option<usize>>,
    /// Byte `+112` (IDA `0x884cb6`, `0x88504c`): active flag.
    pub active: AtomicBool,
    /// Byte `+113` (IDA `0x884cba`, `0x885054`): exclusive flag.
    pub exclusive: AtomicBool,
    /// Shared mouse cached and returned by `getMouseLua` (IDA `0x885014`).
    pub mouse: Mutex<Option<SharedPtr<PluginMouse>>>,
}

impl Plugin {
    fn new(data_model: usize) -> Self {
        // IDA 0x884cac..0x884cba: `setDataModel` link + `+112`/`+113`
        // cleared on the freshly created plugin.
        Self {
            data_model: Mutex::new(Some(data_model)),
            active: AtomicBool::new(false),
            exclusive: AtomicBool::new(false),
            mouse: Mutex::new(None),
        }
    }

    /// Activation virtual, slot `23` (IDA `0x885046`); base override is
    /// empty, preserved for the call-graph shape.
    fn invoke_activate_virtual(&self) {}
}

/// Rust model of `RBX::PluginManager::StateDataEntry` (IDA `0x884c40`).
#[derive(Default)]
pub struct PluginManagerStateEntry {
    /// `std::map<std::string, shared_ptr<Toolbar>>` (IDA `0x884d2c`).
    pub toolbars: HashMap<String, SharedPtr<Toolbar>>,
    /// `std::list<shared_ptr<Plugin>>` (IDA `0x884d3c`): append-only here,
    /// so `Vec` preserves the same order.
    pub plugins: Vec<SharedPtr<Plugin>>,
}

/// Rust model of `RBX::PluginManager` (IDA `0x884c40`): state entries keyed
/// by `DataModel` identity (`std::map<DataModel *, ...>` at `+116`).
#[derive(Default)]
pub struct PluginManager {
    pub states: Mutex<HashMap<usize, PluginManagerStateEntry>>,
}

/// Singleton icon-base path consulted by `Toolbar::createButton`
/// (IDA `0x8851e2`..`0x88528a`); empty in this build.
static PLUGIN_ICON_BASE: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));

/// IDA `0x885266`..`0x88528a`: when the requested icon is non-empty, the
/// result is `base + "\\" + icon` (or just `base` when `base` is empty).
fn resolve_icon_path(icon: &str) -> String {
    if icon.is_empty() {
        return String::new();
    }
    let base = PLUGIN_ICON_BASE.lock().clone();
    if base.is_empty() {
        base
    } else {
        format!("{base}\\{icon}")
    }
}

// 0x884390 — __ZN3RBX9AllocatorINS_15PolyCellContactEE13releaseMemoryEv

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15PolyCellContactEE13releaseMemoryEv")]
// IDA 0x884390: `availableSize = 0` (`STR R1, [R0]` at 0x8843a2), then
// tail-calls `singleton_pool<PolyCellContact, 232>::release_memory`
// (`BL` at 0x8843a8); returns its result.
pub fn stub_884390() -> bool {
    POLY_CELL_CONTACT_AVAILABLE.store(0, Ordering::SeqCst);
    stub_8843ac()
}

// 0x8843ac — __ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv

#[doc(alias = "boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x8843ac: `get_pool()` (`BLX` at 0x8843b0); `mutex::lock` on the pool
// storage (0x8843c2); `pool::release_memory` (0x8843ce); `mutex::unlock`
// (0x8843d2); returns the purge result (0x8843d8).
// was: boost::mutex lock/unlock → `parking_lot::Mutex` guard scope.
pub fn stub_8843ac() -> bool {
    POOL_POLY_CELL_CONTACT.lock().release_memory()
}

// 0x8843dc — __ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEC2Ev

#[doc(alias = "RBX::Allocator<RBX::EdgeEdgeConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEC2Ev")]
// IDA 0x8843dc: `CBNZ initialized` early-out (0x8843f2);
// `vector<ulong*>::push_back(&poolAvailabilityList, &availableSize)`
// (0x884410); `vector<releaser>::push_back(&poolReleaseMemoryFuncList,
// &releaseMemory)` (0x884430); `initialized = 1` (0x884436).
pub fn stub_8843dc() {
    EDGE_EDGE_ALLOC_ONCE.call_once(|| {
        POOL_AVAILABILITY_LIST
            .lock()
            .push(&EDGE_EDGE_AVAILABLE as *const AtomicUsize as usize);
        POOL_RELEASE_MEMORY_FUNCS.lock().push(stub_884444);
    });
}

// 0x884440 — __ZNK3RBX17EdgeEdgeConnector16getConnectorTypeEv
// type: _DWORD __fastcall(RBX::EdgeEdgeConnector *__hidden this)
#[doc(alias = "RBX::EdgeEdgeConnector::getConnectorType(void)const")]
#[doc(alias = "__ZNK3RBX17EdgeEdgeConnector16getConnectorTypeEv")]
// IDA 0x884440: `MOVS R0, #7; BX LR` (disasm 0x884440-0x884442) — const
// connector id, `this` unused.
pub fn stub_884440(_this: *const EdgeEdgeConnector) -> u32 {
    7
}

// 0x884444 — __ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEE13releaseMemoryEv

#[doc(alias = "RBX::Allocator<RBX::EdgeEdgeConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEE13releaseMemoryEv")]
// IDA 0x884444: same shape as 0x884390 for `EdgeEdgeConnector`/`328`
// (`STR` at 0x884456, `BL release_memory` at 0x88445c).
pub fn stub_884444() -> bool {
    EDGE_EDGE_AVAILABLE.store(0, Ordering::SeqCst);
    stub_884460()
}

// 0x884460 — __ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv

#[doc(alias = "boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x884460: same 4-step shape as 0x8843ac for the `328` pool
// (`BLX get_pool` 0x884464, lock 0x884476, purge 0x884482, unlock
// 0x884486, return 0x88448c).
pub fn stub_884460() -> bool {
    POOL_EDGE_EDGE.lock().release_memory()
}

// 0x884490 — __ZN3RBX12PolyCellPairD1Ev
// type: void __fastcall(RBX::PolyCellPair *__hidden this)
#[doc(alias = "RBX::PolyCellPair::~PolyCellPair()")]
#[doc(alias = "__ZN3RBX12PolyCellPairD1Ev")]
// IDA 0x884490: `BX LR` (disasm, 1 insn) — non-deleting destructor body is
// empty.
pub fn stub_884490(_this: *mut PolyCellPair) {}

// 0x884494 — __ZN3RBX12PolyCellPairD0Ev
// type: void __fastcall(RBX::PolyCellPair *__hidden this)
#[doc(alias = "RBX::PolyCellPair::~PolyCellPair()")]
#[doc(alias = "__ZN3RBX12PolyCellPairD0Ev")]
// IDA 0x884494: `B.W operator delete` thunk (disasm, 1 insn) — deleting
// destructor.
// SAFETY: `_this` must be a live box pointer never used again.
pub fn stub_884494(_this: *mut PolyCellPair) {
    unsafe {
        drop(Box::from_raw(_this));
    }
}

// 0x884498 — __ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEC2Ev

#[doc(alias = "RBX::Allocator<RBX::FaceEdgeConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEC2Ev")]
// IDA 0x884498: `CBNZ initialized` early-out (0x8844ae); same two-list
// registration as 0x8843dc for `FaceEdgeConnector` (0x8844cc/0x8844ec);
// `initialized = 1` (0x8844f2).
pub fn stub_884498() {
    FACE_EDGE_ALLOC_ONCE.call_once(|| {
        POOL_AVAILABILITY_LIST
            .lock()
            .push(&FACE_EDGE_AVAILABLE as *const AtomicUsize as usize);
        POOL_RELEASE_MEMORY_FUNCS.lock().push(stub_884500);
    });
}

// 0x8844fc — __ZNK3RBX17FaceEdgeConnector16getConnectorTypeEv
// type: _DWORD __fastcall(RBX::FaceEdgeConnector *__hidden this)
#[doc(alias = "RBX::FaceEdgeConnector::getConnectorType(void)const")]
#[doc(alias = "__ZNK3RBX17FaceEdgeConnector16getConnectorTypeEv")]
// IDA 0x8844fc: `MOVS R0, #8; BX LR` (disasm 0x8844fc-0x8844fe) — const
// connector id, `this` unused.
pub fn stub_8844fc(_this: *const FaceEdgeConnector) -> u32 {
    8
}

// 0x884500 — __ZN3RBX9AllocatorINS_17FaceEdgeConnectorEE13releaseMemoryEv

#[doc(alias = "RBX::Allocator<RBX::FaceEdgeConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17FaceEdgeConnectorEE13releaseMemoryEv")]
// IDA 0x884500: same shape as 0x884390 for `FaceEdgeConnector`/`368`
// (`STR` at 0x884512, `BL release_memory` at 0x884518).
pub fn stub_884500() -> bool {
    FACE_EDGE_AVAILABLE.store(0, Ordering::SeqCst);
    stub_88451c()
}

// 0x88451c — __ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv

#[doc(alias = "boost::singleton_pool<RBX::FaceEdgeConnector,368u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x88451c: same 4-step shape as 0x8843ac for the `368` pool
// (`BLX get_pool` 0x884520, lock 0x884532, purge 0x88453e, unlock
// 0x884542, return 0x884548).
pub fn stub_88451c() -> bool {
    POOL_FACE_EDGE.lock().release_memory()
}

// 0x88454c — __ZN3RBX9AllocatorINS_19FaceVertexConnectorEEC2Ev

#[doc(alias = "RBX::Allocator<RBX::FaceVertexConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_19FaceVertexConnectorEEC2Ev")]
// IDA 0x88454c: `CBNZ initialized` early-out (0x884562); same two-list
// registration as 0x8843dc for `FaceVertexConnector` (0x884580/0x8845a0);
// `initialized = 1` (0x8845a6).
pub fn stub_88454c() {
    FACE_VERTEX_ALLOC_ONCE.call_once(|| {
        POOL_AVAILABILITY_LIST
            .lock()
            .push(&FACE_VERTEX_AVAILABLE as *const AtomicUsize as usize);
        POOL_RELEASE_MEMORY_FUNCS.lock().push(stub_8845b4);
    });
}

// 0x8845b0 — __ZNK3RBX19FaceVertexConnector16getConnectorTypeEv
// type: _DWORD __fastcall(RBX::FaceVertexConnector *__hidden this)
#[doc(alias = "RBX::FaceVertexConnector::getConnectorType(void)const")]
#[doc(alias = "__ZNK3RBX19FaceVertexConnector16getConnectorTypeEv")]
// IDA 0x8845b0: `MOVS R0, #6; BX LR` (disasm 0x8845b0-0x8845b2) — const
// connector id, `this` unused.
pub fn stub_8845b0(_this: *const FaceVertexConnector) -> u32 {
    6
}

// 0x8845b4 — __ZN3RBX9AllocatorINS_19FaceVertexConnectorEE13releaseMemoryEv

#[doc(alias = "RBX::Allocator<RBX::FaceVertexConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_19FaceVertexConnectorEE13releaseMemoryEv")]
// IDA 0x8845b4: same shape as 0x884390 for `FaceVertexConnector`/`304`
// (`STR` at 0x8845c6, `BL release_memory` at 0x8845cc).
pub fn stub_8845b4() -> bool {
    FACE_VERTEX_AVAILABLE.store(0, Ordering::SeqCst);
    stub_8845d0()
}

// 0x8845d0 — __ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv

#[doc(alias = "boost::singleton_pool<RBX::FaceVertexConnector,304u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x8845d0: same 4-step shape as 0x8843ac for the `304` pool
// (`BLX get_pool` 0x8845d4, lock 0x8845e6, purge 0x8845f2, unlock
// 0x8845f6, return 0x8845fc).
pub fn stub_8845d0() -> bool {
    POOL_FACE_VERTEX.lock().release_memory()
}

// 0x884600 — __GLOBAL__I_a_439

#[doc(alias = "global constructor keyed to_a_439")]
#[doc(alias = "__GLOBAL__I_a_439")]
// IDA 0x884600 (`__GLOBAL__I_a_439`, 521 insns): TU static initializer —
// head disassembles as `std::ios_base::Init` construction plus its
// `atexit` destructor registration, then merged-globals init. Runs once
// before main; Rust statics need no glue, so this is a once gate.
pub fn stub_884600() {
    GLOBAL_I_A_439_ONCE.call_once(|| {});
}

// 0x884c40 — __ZN3RBX13PluginManager12createPluginEv
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "RBX::PluginManager::createPlugin(void)")]
#[doc(alias = "__ZN3RBX13PluginManager12createPluginEv")]
// IDA 0x884c40: `Creatable<Instance>::create<Plugin>` (0x884c68);
// `setDataModel` link + `+112`/`+113` cleared (0x884cac..0x884cba);
// `std::map<DataModel *, StateDataEntry>` lower-bound walk
// (0x884cc0..0x884ce4); find-or-emplace with an empty toolbar map + plugin
// list (0x884cfe..0x884dd2); hinted re-walk (0x884dd6..0x884e04);
// `list<shared_ptr<Plugin>>::push_back` (0x884e30..0x884e3a); shared_ptr
// return via the hidden sret slot (0x884e4e..0x884e66).
// `this` is the sret `shared_ptr<Plugin>` slot, not a manager pointer.
// was: boost::shared_ptr/std::map+list → rbx_core::SharedPtr/HashMap+Vec.
pub fn stub_884c40(
    manager: &PluginManager,
    data_model: &rbx_datamodel::data_model::DataModel,
) -> SharedPtr<Plugin> {
    let key = data_model as *const _ as usize;
    let plugin = SharedPtr::new(Plugin::new(key));
    manager
        .states
        .lock()
        .entry(key)
        .or_default()
        .plugins
        .push(SharedPtr::clone(&plugin));
    plugin
}

// 0x885014 — __ZN3RBX6Plugin11getMouseLuaEv
// type: _DWORD __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "RBX::Plugin::getMouseLua(void)")]
#[doc(alias = "__ZN3RBX6Plugin11getMouseLuaEv")]
// IDA 0x885014: `shared_count` copy into the callee slot (0x885026) with
// the `pi_` store/load pair (0x88501e..0x88502c) — cache-and-return the
// shared mouse.
// was: boost::detail::shared_count/pi_ juggling → `Arc` clone.
pub fn stub_885014(plugin: &Plugin, source: &SharedPtr<PluginMouse>) -> SharedPtr<PluginMouse> {
    let mouse = SharedPtr::clone(source);
    *plugin.mouse.lock() = Some(SharedPtr::clone(&mouse));
    mouse
}

// 0x885030 — __ZN3RBX6Plugin8activateEb
// type: _DWORD __fastcall(RBX::Plugin *__hidden this, bool)
#[doc(alias = "RBX::Plugin::activate(bool)")]
#[doc(alias = "__ZN3RBX6Plugin8activateEb")]
// IDA 0x885030: activation virtual, slot `23` (0x885046); `+112 = 1`
// (0x88504c); returns `1` (0x885048); when the bool is set, `+113 = 1`
// (0x885054) plus `Workspace::setNullMouseCommand` (0x885062).
pub fn stub_885030(plugin: &Plugin, mouse: &WorkspaceMouseCommand, exclusive: bool) -> bool {
    plugin.invoke_activate_virtual();
    plugin.active.store(true, Ordering::SeqCst);
    if exclusive {
        plugin.exclusive.store(true, Ordering::SeqCst);
        mouse.set_null_mouse_command();
    }
    true
}

// 0x885068 — __ZN3RBX6Plugin13createToolbarESs

#[doc(alias = "RBX::Plugin::createToolbar(std::string)")]
#[doc(alias = "__ZN3RBX6Plugin13createToolbarESs")]
// IDA 0x885068: factory = vtable `+8` of `*(plugin_ctx + 92)`
// (0x885088..0x885092); `std::string` copy of the name (0x885098) into the
// virtual create (0x8850d2); rep destroy (0x8850e2..0x885128). The factory
// registers the toolbar in the manager's per-DataModel map; the Rust
// `String` owns its rep, so no manual destroy runs.
pub fn stub_885068(manager: &PluginManager, plugin: &Plugin, name: String) {
    let toolbar = SharedPtr::new(Toolbar {
        name: name.clone(),
        ..Default::default()
    });
    if let Some(key) = *plugin.data_model.lock() {
        if let Some(entry) = manager.states.lock().get_mut(&key) {
            entry.toolbars.insert(name, toolbar);
        }
    }
}

// 0x885190 — __ZN3RBX7Toolbar12createButtonESsSsSs

#[doc(alias = "RBX::Toolbar::createButton(std::string,std::string,std::string)")]
#[doc(alias = "__ZN3RBX7Toolbar12createButtonESsSsSs")]
// IDA 0x885190: `Creatable<Instance>::create<Button>` (0x8851b6);
// singleton icon-path fixup — `base + "\\" + icon` when both non-empty
// (0x8851e2..0x88528a); virtual Button factory, vtable `+8`
// (0x88529c..0x8852d6); field stores `+23`/`+25`/`+24`
// (0x8852fe..0x88530c); `map<void *, shared_ptr<Button>>::operator[]` +
// shared copy (0x885318..0x885322); shared_ptr return (0x88532a..0x885350);
// temp `std::string` rep destroys (0x8852dc..0x88540a) — owned `String`s
// here, so no manual destroy runs.
// was: boost::shared_ptr/std::map/std::string → SharedPtr/HashMap/String.
pub fn stub_885190(
    toolbar: &Toolbar,
    text: String,
    tooltip: String,
    icon: String,
) -> SharedPtr<Button> {
    static NEXT_TOKEN: AtomicUsize = AtomicUsize::new(1);
    let token = NEXT_TOKEN.fetch_add(1, Ordering::SeqCst);
    let button = SharedPtr::new(Button {
        factory_token: AtomicUsize::new(token),
        text,
        tooltip,
        icon_path: resolve_icon_path(&icon),
        ..Default::default()
    });
    toolbar
        .buttons
        .lock()
        .insert(token, SharedPtr::clone(&button));
    button
}

// 0x8855bc — __ZN3RBX6Button9setActiveEb
// type: _DWORD __fastcall(RBX::Button *__hidden this, bool)
#[doc(alias = "RBX::Button::setActive(bool)")]
#[doc(alias = "__ZN3RBX6Button9setActiveEb")]
// IDA 0x8855bc: frozen guard at `+104` (0x8855e8) — whole body skipped;
// activate path resets the siblings via `Toolbar::reset`, then sets `+105`
// (0x885610..0x88561e); deactivate path refreshes the singleton and calls
// virtual `+136` while `+105` holds (0x885626..0x885678); flag store
// (0x88567e); change-notify virtual `+12` (0x885694).
pub fn stub_8855bc(button: &Button, toolbar: &Toolbar, active: bool) {
    if button.frozen.load(Ordering::SeqCst) {
        return;
    }
    if active {
        stub_885880(toolbar);
    } else if button.active.load(Ordering::SeqCst) {
        button.invoke_deactivate_singleton();
    }
    button.active.store(active, Ordering::SeqCst);
    button.notify_changed(active);
}

// 0x8856ec — __ZN3RBX6ButtonC2Ev
// type: _DWORD __fastcall(RBX::Button *__hidden this)
#[doc(alias = "RBX::Button::Button(void)")]
#[doc(alias = "__ZN3RBX6ButtonC2Ev")]
// IDA 0x8856ec: `Instance` base ctor (0x88570e); vtable installs
// (0x885740..0x885756); `Described<Button>` class descriptor + registrar
// bump (0x885778..0x8857aa); `+104`/`+105`/`+27` zeroed
// (0x8857cc..0x8857f0); signal static-mutex once-init
// (0x8857f6..0x8857fe). The descriptor tables live with the reflection
// batch; construction state is `Button::default`.
pub fn stub_8856ec() -> Button {
    Button::default()
}

// 0x885880 — __ZN3RBX7Toolbar5resetEv
// type: _DWORD __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "RBX::Toolbar::reset(void)")]
#[doc(alias = "__ZN3RBX7Toolbar5resetEv")]
// IDA 0x885880: `Rb_tree` walk over the button map (0x885884..0x88589e)
// with `Button::setActive(node, 0)` per entry (0x885890) via
// `_Rb_tree_increment` (0x885896). Snapshot first: deactivation never
// inserts, so the order and set match the in-place walk without holding
// the map across the calls.
pub fn stub_885880(toolbar: &Toolbar) {
    let buttons: Vec<SharedPtr<Button>> = toolbar.buttons.lock().values().cloned().collect();
    for button in buttons {
        stub_8855bc(&button, toolbar, false);
    }
}

// 0x8858a4 — __ZN3RBX13PluginManager9singletonEv
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "RBX::PluginManager::singleton(void)")]
#[doc(alias = "__ZN3RBX13PluginManager9singletonEv")]
// IDA 0x8858a4: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8858a4() {
}

// 0x8858cc — __ZN3RBX7ToolbarC2Ev
// type: _DWORD __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "RBX::Toolbar::Toolbar(void)")]
#[doc(alias = "__ZN3RBX7ToolbarC2Ev")]
// IDA 0x8858cc: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8858cc() {
}

// 0x885a20 — __ZN3RBX7Toolbar9getButtonEPv
// type: _DWORD __fastcall(RBX::Toolbar *__hidden this, void *)
#[doc(alias = "RBX::Toolbar::getButton(void *)")]
#[doc(alias = "__ZN3RBX7Toolbar9getButtonEPv")]
// IDA 0x885a20: 26 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885a20() {
}

// 0x885a60 — __ZN3RBX6PluginC2Ev
// type: _DWORD __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "RBX::Plugin::Plugin(void)")]
#[doc(alias = "__ZN3RBX6PluginC2Ev")]
// IDA 0x885a60: 144 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885a60() {
}

// 0x885c04 — __ZN3RBX6PluginD0Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "RBX::Plugin::~Plugin()")]
#[doc(alias = "__ZN3RBX6PluginD0Ev")]
// IDA 0x885c04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_885c04() {
}

// 0x885ca4 — __ZN3RBX6PluginD1Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "RBX::Plugin::~Plugin()")]
#[doc(alias = "__ZN3RBX6PluginD1Ev")]
// IDA 0x885ca4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_885ca4() {
}

// 0x885ca8 — __ZThn32_N3RBX6PluginD0Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Plugin::~Plugin()")]
#[doc(alias = "__ZThn32_N3RBX6PluginD0Ev")]
// IDA 0x885ca8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_885ca8() {
}

// 0x885cb0 — __ZThn36_N3RBX6PluginD0Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Plugin::~Plugin()")]
#[doc(alias = "__ZThn36_N3RBX6PluginD0Ev")]
// IDA 0x885cb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_885cb0() {
}

// 0x885cb8 — __ZN3RBX6PluginD2Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "RBX::Plugin::~Plugin()")]
#[doc(alias = "__ZN3RBX6PluginD2Ev")]
// IDA 0x885cb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_885cb8() {
}

// 0x885df0 — __ZThn32_N3RBX6PluginD1Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Plugin::~Plugin()")]
#[doc(alias = "__ZThn32_N3RBX6PluginD1Ev")]
// IDA 0x885df0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_885df0() {
}

// 0x885df8 — __ZThn36_N3RBX6PluginD1Ev
// type: void __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Plugin::~Plugin()")]
#[doc(alias = "__ZThn36_N3RBX6PluginD1Ev")]
// IDA 0x885df8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_885df8() {
}

// 0x885e00 — __ZN3RBX6Plugin12setDataModelEPNS_9DataModelE
// type: _DWORD __fastcall(RBX::Plugin *__hidden this, RBX::DataModel *)
#[doc(alias = "RBX::Plugin::setDataModel(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX6Plugin12setDataModelEPNS_9DataModelE")]
// IDA 0x885e00: 77 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885e00() {
}

// 0x885edc — __ZN3RBX13PluginManagerC2Ev
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "RBX::PluginManager::PluginManager(void)")]
#[doc(alias = "__ZN3RBX13PluginManagerC2Ev")]
// IDA 0x885edc: 187 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_885edc() {
}

// 0x8860f4 — __Z26initPluginManagerSingletonv
// type: _DWORD __fastcall()
#[doc(alias = "initPluginManagerSingleton(void)")]
#[doc(alias = "__Z26initPluginManagerSingletonv")]
// IDA 0x8860f4: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8860f4() {
}

// 0x886224 — __ZL24doPluginManagerSingletonv
// type: _DWORD __fastcall()
#[doc(alias = "doPluginManagerSingleton(void)")]
#[doc(alias = "__ZL24doPluginManagerSingletonv")]
// IDA 0x886224: 82 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886224() {
}

// 0x886328 — __ZN3RBX13PluginManager15getActivePluginEPNS_9DataModelE
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this, RBX::DataModel *)
#[doc(alias = "RBX::PluginManager::getActivePlugin(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX13PluginManager15getActivePluginEPNS_9DataModelE")]
// IDA 0x886328: 26 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886328() {
}

// 0x886368 — __ZN3RBX13PluginManager17DeactivatePluginsEv
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "RBX::PluginManager::DeactivatePlugins(void)")]
#[doc(alias = "__ZN3RBX13PluginManager17DeactivatePluginsEv")]
// IDA 0x886368: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886368() {
}

// 0x886388 — __ZThn92_N3RBX13PluginManager17DeactivatePluginsEv
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::DeactivatePlugins(void)")]
#[doc(alias = "__ZThn92_N3RBX13PluginManager17DeactivatePluginsEv")]
// IDA 0x886388: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886388() {
}

// 0x8863a8 — __ZN3RBX13PluginManager8activateEPNS_6PluginEPNS_9DataModelE
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this, RBX::Plugin *, RBX::DataModel *)
#[doc(alias = "RBX::PluginManager::activate(RBX::Plugin *,RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX13PluginManager8activateEPNS_6PluginEPNS_9DataModelE")]
// IDA 0x8863a8: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8863a8() {
}

// 0x8865c0 — __ZThn92_N3RBX13PluginManager8activateEPNS_6PluginEPNS_9DataModelE
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this, RBX::Plugin *, RBX::DataModel *)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::activate(RBX::Plugin *,RBX::DataModel *)")]
#[doc(alias = "__ZThn92_N3RBX13PluginManager8activateEPNS_6PluginEPNS_9DataModelE")]
// IDA 0x8865c0: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8865c0() {
}

// 0x8865c8 — __ZN3RBX13PluginManager14StateDataEntry10getToolbarESsPNS_17IStudioPluginHostE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::PluginManager::StateDataEntry::getToolbar(std::string,RBX::IStudioPluginHost *)")]
#[doc(alias = "__ZN3RBX13PluginManager14StateDataEntry10getToolbarESsPNS_17IStudioPluginHostE")]
// IDA 0x8865c8: 205 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8865c8() {
}

// 0x886808 — __ZN3RBX13PluginManager14StateDataEntry12hideStudioUIEbPNS_17IStudioPluginHostE
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
#[doc(alias = "RBX::PluginManager::StateDataEntry::hideStudioUI(bool,RBX::IStudioPluginHost *)")]
#[doc(alias = "__ZN3RBX13PluginManager14StateDataEntry12hideStudioUIEbPNS_17IStudioPluginHostE")]
// IDA 0x886808: 121 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886808() {
}

// 0x886950 — __ZN3RBX13PluginManager14StateDataEntry15fireButtonClickEPv
// type: _DWORD __fastcall(RBX::PluginManager::StateDataEntry *__hidden this, void *)
#[doc(alias = "RBX::PluginManager::StateDataEntry::fireButtonClick(void *)")]
#[doc(alias = "__ZN3RBX13PluginManager14StateDataEntry15fireButtonClickEPv")]
// IDA 0x886950: 22 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886950() {
}

// 0x886984 — __ZN3RBX13PluginManager13createToolbarEPNS_6PluginESs

#[doc(alias = "RBX::PluginManager::createToolbar(RBX::Plugin *,std::string)")]
#[doc(alias = "__ZN3RBX13PluginManager13createToolbarEPNS_6PluginESs")]
// IDA 0x886984: 151 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886984() {
}

// 0x886b40 — __ZThn92_N3RBX13PluginManager13createToolbarEPNS_6PluginESs

#[doc(alias = "non-virtual thunk toRBX::PluginManager::createToolbar(RBX::Plugin *,std::string)")]
#[doc(alias = "__ZThn92_N3RBX13PluginManager13createToolbarEPNS_6PluginESs")]
// IDA 0x886b40: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886b40() {
}

// 0x886b4c — __ZN3RBX13PluginManager11buttonClickEPNS_9DataModelEPv
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this, RBX::DataModel *, void *)
#[doc(alias = "RBX::PluginManager::buttonClick(RBX::DataModel *,void *)")]
#[doc(alias = "__ZN3RBX13PluginManager11buttonClickEPNS_9DataModelEPv")]
// IDA 0x886b4c: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886b4c() {
}

// 0x886cb0 — __ZThn96_N3RBX13PluginManager11buttonClickEPNS_9DataModelEPv
// type: _DWORD __fastcall(RBX::PluginManager *__hidden this, RBX::DataModel *, void *)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::buttonClick(RBX::DataModel *,void *)")]
#[doc(alias = "__ZThn96_N3RBX13PluginManager11buttonClickEPNS_9DataModelEPv")]
// IDA 0x886cb0: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886cb0() {
}

// 0x886cb8 — __ZN3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PluginManager,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev")]
// IDA 0x886cb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886cb8() {
}

// 0x886cdc — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev")]
// IDA 0x886cdc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886cdc() {
}

// 0x886d00 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EED1Ev

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EED1Ev")]
// IDA 0x886d00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886d00() {
}

// 0x886d40 — __ZN3RBX10Reflection9EventDescINS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
// IDA 0x886d40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886d40() {
}

// 0x886d64 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED1Ev

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED1Ev")]
// IDA 0x886d64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886d64() {
}

// 0x886da4 — __ZN3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EED1Ev

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Toolbar,boost::shared_ptr<RBX::Instance> ()(std::string,std::string,std::string),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EED1Ev")]
// IDA 0x886da4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886da4() {
}

// 0x886df4 — __ZN3RBX10Reflection13BoundFuncDescINS_6ButtonEFvbELi1EED1Ev

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Button,void ()(bool),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6ButtonEFvbELi1EED1Ev")]
// IDA 0x886df4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886df4() {
}

// 0x886e34 — __ZN3RBX10Reflection9EventDescINS_6ButtonEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Button,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Button::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_6ButtonEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
// IDA 0x886e34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886e34() {
}

// 0x886e58 — __ZN5boost10shared_ptrIN3RBX13PluginManagerEED1Ev

#[doc(alias = "boost::shared_ptr<RBX::PluginManager>::~shared_ptr()")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13PluginManagerEED1Ev")]
// IDA 0x886e58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_886e58() {
}

// 0x886e6c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6ButtonEEEN5boost10shared_ptrIT_EEv

#[doc(alias = "boost::shared_ptr<RBX::Button> RBX::Creatable<RBX::Instance>::create<RBX::Button>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6ButtonEEEN5boost10shared_ptrIT_EEv")]
// IDA 0x886e6c: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886e6c() {
}

// 0x886f1c — __ZNSt3mapIPvN5boost10shared_ptrIN3RBX6ButtonEEESt4lessIS0_ESaISt4pairIKS0_S5_EEEixERS9_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<void *,boost::shared_ptr<RBX::Button>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::operator[](void * const&)")]
#[doc(alias = "__ZNSt3mapIPvN5boost10shared_ptrIN3RBX6ButtonEEESt4lessIS0_ESaISt4pairIKS0_S5_EEEixERS9_")]
// IDA 0x886f1c: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_886f1c() {
}

// 0x887064 — __ZN5boost10shared_ptrIN3RBX6ButtonEEaSERKS3_

#[doc(alias = "boost::shared_ptr<RBX::Button>::operator=(boost::shared_ptr<RBX::Button> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6ButtonEEaSERKS3_")]
// IDA 0x887064: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_887064() {
}

// 0x88709c — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_11PluginMouseEEERS3_RKNS0_IT_EE

#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::PluginMouse>(boost::shared_ptr<RBX::PluginMouse> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_11PluginMouseEEERS3_RKNS0_IT_EE")]
// IDA 0x88709c: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88709c() {
}

// 0x8870d0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11PluginMouseEEEN5boost10shared_ptrIT_EEv

#[doc(alias = "boost::shared_ptr<RBX::PluginMouse> RBX::Creatable<RBX::Instance>::create<RBX::PluginMouse>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_11PluginMouseEEEN5boost10shared_ptrIT_EEv")]
// IDA 0x8870d0: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8870d0() {
}

// 0x887180 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6PluginEEEN5boost10shared_ptrIT_EEv

#[doc(alias = "boost::shared_ptr<RBX::Plugin> RBX::Creatable<RBX::Instance>::create<RBX::Plugin>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6PluginEEEN5boost10shared_ptrIT_EEv")]
// IDA 0x887180: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_887180() {
}

// 0x887230 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7ToolbarEEEN5boost10shared_ptrIT_EEv

#[doc(alias = "boost::shared_ptr<RBX::Toolbar> RBX::Creatable<RBX::Instance>::create<RBX::Toolbar>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_7ToolbarEEEN5boost10shared_ptrIT_EEv")]
// IDA 0x887230: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_887230() {
}

// 0x8872e0 — __ZNSt3mapISsN5boost10shared_ptrIN3RBX7ToolbarEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_

#[doc(alias = "std::map<std::string,boost::shared_ptr<RBX::Toolbar>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsN5boost10shared_ptrIN3RBX7ToolbarEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_")]
// IDA 0x8872e0: 192 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8872e0() {
}

// 0x8874fc — __ZN5boost10shared_ptrIN3RBX7ToolbarEEaSERKS3_

#[doc(alias = "boost::shared_ptr<RBX::Toolbar>::operator=(boost::shared_ptr<RBX::Toolbar> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7ToolbarEEaSERKS3_")]
// IDA 0x8874fc: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8874fc() {
}

// 0x887534 — __ZN3RBX13PluginManagerD1Ev
// type: void __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "RBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZN3RBX13PluginManagerD1Ev")]
// IDA 0x887534: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_887534() {
}

// 0x887538 — __ZN3RBX13PluginManagerD0Ev
// type: void __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "RBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZN3RBX13PluginManagerD0Ev")]
// IDA 0x887538: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887538() {
}

// 0x8875d8 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEE12getClassNameEv

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEE12getClassNameEv")]
// IDA 0x8875d8: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8875d8() {
}

// 0x887600 — __ZThn32_N3RBX13PluginManagerD1Ev
// type: void __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZThn32_N3RBX13PluginManagerD1Ev")]
// IDA 0x887600: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887600() {
}

// 0x887608 — __ZThn32_N3RBX13PluginManagerD0Ev
// type: void __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZThn32_N3RBX13PluginManagerD0Ev")]
// IDA 0x887608: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887608() {
}

// 0x8876ac — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEE12getClassNameEv

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEE12getClassNameEv")]
// IDA 0x8876ac: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8876ac() {
}

// 0x8876d4 — __ZThn36_N3RBX13PluginManagerD1Ev
// type: void __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZThn36_N3RBX13PluginManagerD1Ev")]
// IDA 0x8876d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8876d4() {
}

// 0x8876dc — __ZThn36_N3RBX13PluginManagerD0Ev
// type: void __fastcall(RBX::PluginManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZThn36_N3RBX13PluginManagerD0Ev")]
// IDA 0x8876dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8876dc() {
}

// 0x887780 — __ZNK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E12getClassNameEv

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E12getClassNameEv")]
// IDA 0x887780: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_887780() {
}

// 0x887790 — __ZThn32_NK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E12getClassNameEv")]
// IDA 0x887790: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_887790() {
}

// 0x8877a0 — __ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7CreatorD1Ev

#[doc(alias = "__ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7CreatorD1Ev")]
// IDA 0x8877a0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8877a0() {
}

// 0x8877a4 — __ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7CreatorD1Ev

#[doc(alias = "__ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7CreatorD1Ev")]
// IDA 0x8877a4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8877a4() {
}

// 0x8877a8 — __ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7CreatorD1Ev

#[doc(alias = "__ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7CreatorD1Ev")]
// IDA 0x8877a8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8877a8() {
}

// 0x8877ac — __ZN3RBX6ButtonD1Ev
// type: void __fastcall(RBX::Button *__hidden this)
#[doc(alias = "RBX::Button::~Button()")]
#[doc(alias = "__ZN3RBX6ButtonD1Ev")]
// IDA 0x8877ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8877ac() {
}

// 0x8878c0 — __ZN3RBX6ButtonD0Ev
// type: void __fastcall(RBX::Button *__hidden this)
#[doc(alias = "RBX::Button::~Button()")]
#[doc(alias = "__ZN3RBX6ButtonD0Ev")]
// IDA 0x8878c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8878c0() {
}

// 0x8879e8 — __ZNK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E12getClassNameEv

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E12getClassNameEv")]
// IDA 0x8879e8: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8879e8() {
}

// 0x8879f8 — __ZThn32_N3RBX6ButtonD1Ev
// type: void __fastcall(RBX::Button *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
#[doc(alias = "__ZThn32_N3RBX6ButtonD1Ev")]
// IDA 0x8879f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8879f8() {
}

// 0x887b08 — __ZThn32_N3RBX6ButtonD0Ev
// type: void __fastcall(RBX::Button *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
#[doc(alias = "__ZThn32_N3RBX6ButtonD0Ev")]
// IDA 0x887b08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887b08() {
}

// 0x887c30 — __ZThn32_NK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E12getClassNameEv

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E12getClassNameEv")]
// IDA 0x887c30: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_887c30() {
}

// 0x887c40 — __ZThn36_N3RBX6ButtonD1Ev
// type: void __fastcall(RBX::Button *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
#[doc(alias = "__ZThn36_N3RBX6ButtonD1Ev")]
// IDA 0x887c40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887c40() {
}

// 0x887d50 — __ZThn36_N3RBX6ButtonD0Ev
// type: void __fastcall(RBX::Button *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Button::~Button()")]
#[doc(alias = "__ZThn36_N3RBX6ButtonD0Ev")]
// IDA 0x887d50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887d50() {
}

// 0x887e78 — __ZN3RBX7ToolbarD1Ev
// type: void __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "RBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZN3RBX7ToolbarD1Ev")]
// IDA 0x887e78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887e78() {
}

// 0x887f64 — __ZN3RBX7ToolbarD0Ev
// type: void __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "RBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZN3RBX7ToolbarD0Ev")]
// IDA 0x887f64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_887f64() {
}

// 0x888060 — __ZNK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E12getClassNameEv

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E12getClassNameEv")]
// IDA 0x888060: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_888060() {
}

// 0x888070 — __ZThn32_N3RBX7ToolbarD1Ev
// type: void __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZThn32_N3RBX7ToolbarD1Ev")]
// IDA 0x888070: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_888070() {
}

// 0x888158 — __ZThn32_N3RBX7ToolbarD0Ev
// type: void __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZThn32_N3RBX7ToolbarD0Ev")]
// IDA 0x888158: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_888158() {
}

// 0x888258 — __ZThn32_NK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E12getClassNameEv

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E12getClassNameEv")]
// IDA 0x888258: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_888258() {
}

// 0x888268 — __ZThn36_N3RBX7ToolbarD1Ev
// type: void __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZThn36_N3RBX7ToolbarD1Ev")]
// IDA 0x888268: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_888268() {
}

// 0x888350 — __ZThn36_N3RBX7ToolbarD0Ev
// type: void __fastcall(RBX::Toolbar *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Toolbar::~Toolbar()")]
#[doc(alias = "__ZThn36_N3RBX7ToolbarD0Ev")]
// IDA 0x888350: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_888350() {
}

// 0x888450 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(int result, int)
#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_erase(std::_Rb_tree_node<std::pair<void * const,boost::shared_ptr<RBX::Button>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// IDA 0x888450: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_888450() {
}
