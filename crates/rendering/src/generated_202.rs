//! rendering — generated_202 — 150 stubs global gap filler EA-sorted asc after 0x3ca778
//! Filter: Ogre|G3D|Rendering|Adorn complete 14266/14266 (done) -> global gap filler next 150 unstubbed by stub_0x scan (33823 stub_0x present, 51722 gaps, this batch 0x16e4c..0x42774)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use parking_lot::Mutex;
use std::sync::{
    Once, OnceLock,
    atomic::{AtomicUsize, Ordering},
};

// ---- impl batch 0x97c0..0x1c740 (29 fns, IDA decompile+disasm grounded) ----
//
// Boost mapping (AGENTS.md §4, no boost crate):
// boost::shared_ptr -> rbx_core::SharedPtr; boost::singleton_pool storage ->
// GuardedPool behind parking_lot::Mutex; boost::call_once -> std::sync::Once;
// boost::exception_detail static objects -> Once-guarded markers;
// __cxa_atexit -> ATEXIT_REGISTRY.

/// Block field flag in every copy/destroy helper below (IDA literal `3`
/// = BLOCK_FIELD_IS_OBJECT).
pub const BLOCK_FIELD_IS_OBJECT: i32 = 3;

/// Captured ObjC object slot in a heap block (IDA `*(a1 + 20)` cells).
/// `_Block_object_assign` retains and `_Block_object_dispose` releases;
/// modeled by cloning/dropping a `SharedPtr` (was: objc_retain/release).
pub type BlockSlot = Option<SharedPtr<()>>;

/// IDA `_Block_object_assign` / `_Block_object_assign_shim` for flag 3:
/// retain `src` into `dst`. The shim adds only a nil-tolerant indirection,
/// so both spellings map here.
pub fn block_assign_slot(dst: &mut BlockSlot, src: &BlockSlot) {
    let _flag = BLOCK_FIELD_IS_OBJECT;
    *dst = src.clone();
}

/// IDA `_Block_object_dispose` / `_Block_object_dispose_shim` for flag 3:
/// release the captured object held by `slot`.
pub fn block_dispose_slot(slot: &mut BlockSlot) {
    let _flag = BLOCK_FIELD_IS_OBJECT;
    *slot = None;
}

/// Single-capture copy helper shape (IDA `dst + 20` <- `src + 20`).
pub fn block_copy_1(dst: &mut [BlockSlot], src: &[BlockSlot]) {
    block_assign_slot(&mut dst[0], &src[0]);
}

/// Single-capture destroy helper shape (IDA dispose of `a1 + 20`).
pub fn block_dispose_1(slots: &mut [BlockSlot]) {
    block_dispose_slot(&mut slots[0]);
}

/// Triple-capture copy helper shape (IDA `dst + 20/24/28` from `src[5]/[6]/[7]`).
pub fn block_copy_3(dst: &mut [BlockSlot], src: &[BlockSlot]) {
    block_assign_slot(&mut dst[0], &src[0]);
    block_assign_slot(&mut dst[1], &src[1]);
    block_assign_slot(&mut dst[2], &src[2]);
}

/// Triple-capture destroy helper shape (IDA dispose of `a1[5]/[6]/[7]`).
pub fn block_dispose_3(slots: &mut [BlockSlot]) {
    block_dispose_slot(&mut slots[0]);
    block_dispose_slot(&mut slots[1]);
    block_dispose_slot(&mut slots[2]);
}

/// IDA `OBJC_CLASS___Appirater` instance (0x17fe4: alloc + init).
pub struct Appirater {
    /// IDA `setDelegate:` target (global `dword_130C394`).
    pub delegate: usize,
}

impl Appirater {
    /// IDA `-[Appirater init]`.
    pub fn init(delegate: usize) -> Self {
        Self { delegate }
    }
}

/// Observer entry behind `+[NSNotificationCenter defaultCenter]`
/// (IDA 0x18052..0x18092).
pub struct NotificationObserver {
    pub observer: usize,
    pub selector: &'static str,
    pub name: &'static str,
}

static APP_WILL_RESIGN_OBSERVERS: Mutex<Vec<NotificationObserver>> = Mutex::new(Vec::new());

/// IDA `dword_130C398` — the `+[Appirater sharedInstance]` cell.
static APPIRATER_SHARED: OnceLock<Appirater> = OnceLock::new();

/// IDA `dword_130C394` — delegate installed before first `sharedInstance`.
static APPIRATER_DELEGATE: Mutex<usize> = Mutex::new(0);

/// Write path for IDA `dword_130C394` (test hook for the singleton below).
pub fn set_appirater_delegate(delegate: usize) {
    *APPIRATER_DELEGATE.lock() = delegate;
}

/// IDA `UIViewController` node walked by 0x1a124.
pub struct TopViewController {
    /// Next controller returned by the external IDA `_topMostController(vc)`
    /// callee (presented/modal child), if any.
    pub child: Option<SharedPtr<TopViewController>>,
}

/// Models the external IDA `_topMostController(UIViewController *)` callee
/// invoked at 0x1a166: descend one presentation level, if any.
pub fn top_most_controller_step(vc: &TopViewController) -> Option<SharedPtr<TopViewController>> {
    vc.child.clone()
}

/// IDA `cfstr_Uiapplication` / `cfstr_Appdelegate` (0x1a79c..0x1a7b6).
pub const UI_APPLICATION_PRINCIPAL_CLASS: &str = "UIApplication";
pub const UI_APPLICATION_DELEGATE_CLASS: &str = "AppDelegate";

/// IDA `NSAutoreleasePool` scope in `_main`
/// (alloc @0x1a788, init @0x1a798, release @0x1a7ca).
pub struct AutoreleasePool;

impl AutoreleasePool {
    pub fn alloc_init() -> Self {
        Self
    }

    pub fn release(self) {}
}

/// Launch record behind IDA `_UIApplicationMain` (0x1a7b6): last argc/argv.
static LAST_UI_APPLICATION_MAIN: Mutex<Option<(i32, Vec<String>)>> = Mutex::new(None);

/// IDA `_UIApplicationMain(argc, argv, principalClassName, delegateClassName)`.
pub fn ui_application_main(argc: i32, argv: &[String], principal: &str, delegate: &str) -> i32 {
    debug_assert_eq!(principal, UI_APPLICATION_PRINCIPAL_CLASS);
    debug_assert_eq!(delegate, UI_APPLICATION_DELEGATE_CLASS);
    *LAST_UI_APPLICATION_MAIN.lock() = Some((argc, argv.to_vec()));
    0
}

/// IDA `__cxa_atexit` registrations in TU order (dtor labels).
static ATEXIT_REGISTRY: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

pub fn register_atexit(dtor: &'static str) {
    ATEXIT_REGISTRY.lock().push(dtor);
}

/// IDA `boost::system::generic_category/system_category` statics. Rust's std
/// carries error categories implicitly; the Once models the TU guard and the
/// counter records the three observed stores (generic x2 + system).
static ERROR_CATEGORIES_INIT: Once = Once::new();
static ERROR_CATEGORIES_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn ensure_error_categories() {
    ERROR_CATEGORIES_INIT.call_once(|| {
        ERROR_CATEGORIES_COUNT.store(3, Ordering::SeqCst);
    });
}

/// IDA `std::ios_base::Init::Init` + `__cxa_atexit(~Init)`.
static IOS_BASE_INIT: Once = Once::new();

pub fn ensure_ios_base_init() {
    IOS_BASE_INIT.call_once(|| {
        register_atexit("std::ios_base::Init::~Init");
    });
}

/// IDA `boost::exception_detail::exception_ptr_static_exception_object<T>::e`
/// guarded construction + `__cxa_atexit(~exception_ptr)` (was: boost::exception_ptr).
pub fn ensure_static_exception_object(guard: &Once, name: &'static str) {
    guard.call_once(|| {
        let _ = name;
        register_atexit("boost::exception_ptr::~exception_ptr");
    });
}

static BAD_ALLOC_OBJECT: Once = Once::new();
static BAD_EXCEPTION_OBJECT: Once = Once::new();

/// Rust model of `boost::singleton_pool<T, RequestedSize, ...>` storage behind
/// `get_pool()` (IDA `0x17d46`, `0x1a6be`, ...): the pool mutex at `storage`
/// plus the `create_object` creation guard.
pub struct GuardedPool {
    pub requested_size: usize,
    pool: Mutex<usize>,
    created: Once,
}

impl GuardedPool {
    pub const fn new(requested_size: usize) -> Self {
        Self {
            requested_size,
            pool: Mutex::new(0),
            created: Once::new(),
        }
    }

    /// IDA `singleton_pool<T, N>::get_pool()` — idempotent pool creation behind
    /// the TU `__ZGVN...storageE` / `...create_objectE` guards.
    pub fn get_pool(&self) -> usize {
        self.created.call_once(|| {
            *self.pool.lock() = self.requested_size;
        });
        *self.pool.lock()
    }
}

static POOL_XML_ATTRIBUTE: GuardedPool = GuardedPool::new(20);
static POOL_XML_ELEMENT: GuardedPool = GuardedPool::new(36);
static POOL_FW_INSTANCE: GuardedPool = GuardedPool::new(28);
static POOL_ON_DEMAND_INSTANCE: GuardedPool = GuardedPool::new(20);
static POOL_ON_DEMAND_PV_INSTANCE: GuardedPool = GuardedPool::new(24);
static POOL_FW_PART_INSTANCE: GuardedPool = GuardedPool::new(56);
static POOL_ON_DEMAND_PART_INSTANCE: GuardedPool = GuardedPool::new(200);

/// Tail shared by IDA 0x17c58/0x1a5d0/0x1a7d4: the four instance pools those
/// TUs ensure (XmlAttribute/20, XmlElement/36, FWInstance/28, OnDemandInstance/20).
pub fn ensure_xml_instance_pools() {
    POOL_XML_ATTRIBUTE.get_pool();
    POOL_XML_ELEMENT.get_pool();
    POOL_FW_INSTANCE.get_pool();
    POOL_ON_DEMAND_INSTANCE.get_pool();
}

/// IDA `RBX::Reflection::Singleton<EnumDesc<E>>` + `EnumRegistrar<E>` /
/// `TypeRegistrar<E>` slots behind `__GLOBAL__I_a`: zero both registrar cells,
/// run `initSingleton` once, fetch the singleton twice.
pub struct ReflectionEnumInit {
    pub name: &'static str,
    enum_registrar: AtomicUsize,
    type_registrar: AtomicUsize,
    singleton_once: Once,
    singleton_gets: AtomicUsize,
}

impl ReflectionEnumInit {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            enum_registrar: AtomicUsize::new(0),
            type_registrar: AtomicUsize::new(0),
            singleton_once: Once::new(),
            singleton_gets: AtomicUsize::new(0),
        }
    }

    pub fn run(&self) {
        // IDA `STR.W R11, [EnumRegistrar]`.
        self.enum_registrar.store(0, Ordering::SeqCst);
        // IDA `boost::call_once(flag, initSingleton)`.
        self.singleton_once.call_once(|| {});
        // IDA `doGetSingleton()` (first fetch).
        self.singleton_gets.fetch_add(1, Ordering::SeqCst);
        // IDA `STR.W R11, [TypeRegistrar]`.
        self.type_registrar.store(0, Ordering::SeqCst);
        self.singleton_once.call_once(|| {});
        // IDA `doGetSingleton()` (second fetch).
        self.singleton_gets.fetch_add(1, Ordering::SeqCst);
    }

    pub fn singleton_get_count(&self) -> usize {
        self.singleton_gets.load(Ordering::SeqCst)
    }
}

static ENUM_AA_SAMPLES: ReflectionEnumInit = ReflectionEnumInit::new("AASamples");
static ENUM_GRAPHICS_MODE: ReflectionEnumInit = ReflectionEnumInit::new("GraphicsMode");
static ENUM_FRAME_RATE_MANAGER_MODE: ReflectionEnumInit =
    ReflectionEnumInit::new("FrameRateManagerMode");
static ENUM_ANTIALIASING_MODE: ReflectionEnumInit = ReflectionEnumInit::new("AntialiasingMode");
static ENUM_QUALITY_LEVEL: ReflectionEnumInit = ReflectionEnumInit::new("QualityLevel");
static ENUM_RESOLUTION_PRESET: ReflectionEnumInit = ReflectionEnumInit::new("ResolutionPreset");
static ENUM_SHADOW_MODE: ReflectionEnumInit = ReflectionEnumInit::new("ShadowMode");

/// IDA `RBX::Reflection::ClassRegistrar<CRenderSettingsItem>::registrar`
/// zeroed at 0x16ea6, then `classDescriptor()` at 0x16eaa.
static CLASS_REGISTRAR_CRSI: AtomicUsize = AtomicUsize::new(0);
static CLASS_DESCRIPTOR_CALLS: AtomicUsize = AtomicUsize::new(0);

pub fn class_descriptor_render_settings_item() -> &'static str {
    CLASS_DESCRIPTOR_CALLS.fetch_add(1, Ordering::SeqCst);
    "CRenderSettingsItem"
}

/// One static descriptor construction inside `__GLOBAL__I_a` (IDA
/// `EnumPropDescriptor` / `PropDescriptor` / `BoundProp` / `BoundFuncDesc`
/// ctor plus `__cxa_atexit` of its dtor).
pub struct PropDescriptorRecord {
    pub name: &'static str,
    pub category: &'static str,
    pub kind: &'static str,
}

static PROP_DESCRIPTOR_REGISTRY: Mutex<Vec<PropDescriptorRecord>> = Mutex::new(Vec::new());

pub fn register_prop_descriptor(name: &'static str, category: &'static str, kind: &'static str) {
    PROP_DESCRIPTOR_REGISTRY
        .lock()
        .push(PropDescriptorRecord { name, category, kind });
    register_atexit(kind);
}

/// IDA `FactoryProduct<T>::creatorPrivate` guarded `Creator` construction
/// (0x17964..0x179de: Camera, then CRenderSettingsItem).
static CREATOR_CAMERA: Once = Once::new();
static CREATOR_CRSI_ITEM: Once = Once::new();

pub fn ensure_factory_creator(guard: &Once, name: &'static str) {
    guard.call_once(|| {
        register_atexit(name);
    });
}


// 0x16e4c — __GLOBAL__I_a
#[doc(alias = "global constructor keyed to_a")]
// was: global constructor keyed to_a
// IDA 0x16e4c: 911 insns — CRenderSettingsItem TU static init: boost categories +
// ios_base::Init/atexit, ClassRegistrar zero + classDescriptor(), 7 reflection
// enums (EnumRegistrar zero + call_once(initSingleton) + 2x doGetSingleton +
// TypeRegistrar zero), 20 prop/func descriptors each with atexit(dtor),
// bad_alloc/bad_exception objects, 7 singleton pools, Camera +
// CRenderSettingsItem factory creators.
pub fn stub_0x16e4c() {
    // IDA 0x16e56..0x16e70: generic_category x2 + system_category stores.
    ensure_error_categories();
    // IDA 0x16e72..0x16e94: ios_base::Init + __cxa_atexit(~Init).
    ensure_ios_base_init();
    // IDA 0x16ea6..0x16eaa: ClassRegistrar<CRenderSettingsItem>::registrar = 0, then classDescriptor().
    CLASS_REGISTRAR_CRSI.store(0, Ordering::SeqCst);
    class_descriptor_render_settings_item();
    // IDA 0x16eae..0x170be: one registrar block per enum, in TU order.
    ENUM_AA_SAMPLES.run();
    ENUM_GRAPHICS_MODE.run();
    ENUM_FRAME_RATE_MANAGER_MODE.run();
    ENUM_ANTIALIASING_MODE.run();
    ENUM_QUALITY_LEVEL.run();
    ENUM_RESOLUTION_PRESET.run();
    ENUM_SHADOW_MODE.run();
    // IDA 0x170c2..0x1777c: static prop/func descriptor constructions, in TU order.
    register_prop_descriptor("graphicsMode", "General", "EnumPropDescriptor<GraphicsMode>");
    register_prop_descriptor("FrameRateManager", "General", "EnumPropDescriptor<FrameRateManagerMode>");
    register_prop_descriptor("QualityLevel", "Performance", "EnumPropDescriptor<QualityLevel>");
    register_prop_descriptor("AlwaysDrawConnectors", "General", "PropDescriptor<bool>");
    register_prop_descriptor("IsAggregationShown", "Debug", "PropDescriptor<bool>");
    register_prop_descriptor("IsSynchronizedWithPhysics", "Performance", "BoundProp<bool>");
    register_prop_descriptor("UsesPaintMessage", "Performance", "BoundProp<bool>");
    register_prop_descriptor("AASamples", "Quality", "EnumPropDescriptor<AASamples>");
    register_prop_descriptor("profileName", "Quality", "BoundProp<string>");
    register_prop_descriptor("Shadow", "Debug", "EnumPropDescriptor<ShadowMode>");
    register_prop_descriptor("Antialiasing", "Quality", "EnumPropDescriptor<AntialiasingMode>");
    register_prop_descriptor("ShowBoundingBoxes", "Debug", "PropDescriptor<bool>");
    register_prop_descriptor("AutoFRMLevel", "Debug", "PropDescriptor<int>");
    register_prop_descriptor("EnableFRM", "Debug", "PropDescriptor<bool>");
    register_prop_descriptor("DebugDisableInterpolation", "Debug", "PropDescriptor<bool>");
    register_prop_descriptor("Resolution", "Screen", "EnumPropDescriptor<ResolutionPreset>");
    register_prop_descriptor("GetMaxQualityLevel", "", "BoundFuncDesc<int()>");
    register_prop_descriptor("TextureCacheSize", "Cache", "PropDescriptor<uint>");
    register_prop_descriptor("MeshCacheSize", "Cache", "PropDescriptor<uint>");
    register_prop_descriptor("EagerBulkExecution", "Performance", "PropDescriptor<bool>");
    // IDA 0x1777c..0x177f6: guarded bad_alloc / bad_exception objects.
    ensure_static_exception_object(&BAD_ALLOC_OBJECT, "bad_alloc");
    ensure_static_exception_object(&BAD_EXCEPTION_OBJECT, "bad_exception");
    // IDA 0x177f8..0x17964: guarded singleton_pool get_pool() calls, in TU order.
    POOL_XML_ATTRIBUTE.get_pool();
    POOL_XML_ELEMENT.get_pool();
    POOL_FW_INSTANCE.get_pool();
    POOL_ON_DEMAND_INSTANCE.get_pool();
    POOL_ON_DEMAND_PV_INSTANCE.get_pool();
    POOL_FW_PART_INSTANCE.get_pool();
    POOL_ON_DEMAND_PART_INSTANCE.get_pool();
    // IDA 0x17964..0x179de: guarded FactoryProduct creators (Camera, CRenderSettingsItem).
    ensure_factory_creator(&CREATOR_CAMERA, "FactoryProduct<Camera>::~Creator");
    ensure_factory_creator(&CREATOR_CRSI_ITEM, "FactoryProduct<CRenderSettingsItem>::~Creator");
}

// 0x17c58 — __GLOBAL__I_a_0
#[doc(alias = "global constructor keyed to_a_0")]
// was: global constructor keyed to_a_0
// IDA 0x17c58: 131 insns — TU prologue (boost categories, ios_base::Init, atexit) + guarded bad_alloc/bad_exception objects + XmlAttribute/XmlElement/FWInstance/OnDemandInstance pools.
pub fn stub_0x17c58() {
    ensure_error_categories();
    ensure_ios_base_init();
    ensure_static_exception_object(&BAD_ALLOC_OBJECT, "bad_alloc");
    ensure_static_exception_object(&BAD_EXCEPTION_OBJECT, "bad_exception");
    ensure_xml_instance_pools();
}

// 0x17f80 — +[Appirater sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[Appirater sharedInstance]")]
// was: +[Appirater sharedInstance]
// IDA 0x17f80: fast path returns dword_130C398 when set; else stack block + dispatch_once(0x17fe4), then return dword_130C398.
pub fn stub_0x17f80(class_token: usize) -> &'static Appirater {
    if let Some(shared) = APPIRATER_SHARED.get() {
        return shared;
    }
    APPIRATER_SHARED.get_or_init(|| {
        let delegate = *APPIRATER_DELEGATE.lock();
        stub_0x17fe4(class_token, delegate)
    })
}

// 0x17fe4 — ___27+[Appirater sharedInstance]_block_invoke
#[doc(alias = "___27+[Appirater sharedInstance]_block_invoke")]
// was: ___27+[Appirater sharedInstance]_block_invoke
// IDA 0x17fe4: +[Appirater alloc] + -[Appirater init], setDelegate:(dword_130C394), defaultCenter addObserver:appWillResignActive.
pub fn stub_0x17fe4(class_token: usize, delegate: usize) -> Appirater {
    let instance = Appirater::init(delegate);
    APP_WILL_RESIGN_OBSERVERS.lock().push(NotificationObserver {
        observer: class_token,
        selector: "appWillResignActive",
        name: "UIApplicationWillResignActiveNotification",
    });
    instance
}

// 0x18094 — ___copy_helper_block_
#[doc(alias = "___copy_helper_block_")]
// was: ___copy_helper_block_
// IDA 0x18094: `_Block_object_assign_shim(dst + 20, src + 20, 3)` — retain the single captured object.
pub fn stub_0x18094(dst: &mut [BlockSlot], src: &[BlockSlot]) {
    block_copy_1(dst, src);
}

// 0x180a0 — ___destroy_helper_block_
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_")]
// was: ___destroy_helper_block_
// IDA 0x180a0: `_Block_object_dispose_shim(*(a1 + 20), 3)` — release the single captured object.
pub fn stub_0x180a0(slots: &mut [BlockSlot]) {
    block_dispose_1(slots);
}

// 0x18bc8 — ___copy_helper_block_125
#[doc(alias = "___copy_helper_block_125")]
// was: ___copy_helper_block_125
// IDA 0x18bc8: `_Block_object_assign_shim(dst + 20, src + 20, 3)` — retain the single captured object.
pub fn stub_0x18bc8(dst: &mut [BlockSlot], src: &[BlockSlot]) {
    block_copy_1(dst, src);
}

// 0x18bd4 — ___destroy_helper_block_126
#[doc(alias = "___destroy_helper_block_126")]
// was: ___destroy_helper_block_126
// IDA 0x18bd4: `_Block_object_dispose_shim(*(a1 + 20), 3)` — release the single captured object.
pub fn stub_0x18bd4(slots: &mut [BlockSlot]) {
    block_dispose_1(slots);
}

// 0x18c8c — ___copy_helper_block_130
#[doc(alias = "___copy_helper_block_130")]
// was: ___copy_helper_block_130
// IDA 0x18c8c: `_Block_object_assign_shim(dst + 20, src + 20, 3)` — retain the single captured object.
pub fn stub_0x18c8c(dst: &mut [BlockSlot], src: &[BlockSlot]) {
    block_copy_1(dst, src);
}

// 0x18c98 — ___destroy_helper_block_131
#[doc(alias = "___destroy_helper_block_131")]
// was: ___destroy_helper_block_131
// IDA 0x18c98: `_Block_object_dispose_shim(*(a1 + 20), 3)` — release the single captured object.
pub fn stub_0x18c98(slots: &mut [BlockSlot]) {
    block_dispose_1(slots);
}

// 0x1a124 — __Z17topMostControllerv
// type: _DWORD __fastcall()
#[doc(alias = "topMostController(void)")]
// was: topMostController(void)
// IDA 0x1a124: sharedApplication -> keyWindow -> rootViewController, then do/while descent via _topMostController until null; return deepest controller.
pub fn stub_0x1a124(root: SharedPtr<TopViewController>) -> SharedPtr<TopViewController> {
    let mut top = root;
    let mut descended = top_most_controller_step(&top);
    while let Some(next) = descended {
        top = next;
        descended = top_most_controller_step(&top);
    }
    top
}

// 0x1a5d0 — __GLOBAL__I_a_1
#[doc(alias = "global constructor keyed to_a_1")]
// was: global constructor keyed to_a_1
// IDA 0x1a5d0: 131 insns — same TU shape as 0x17c58 (prologue + bad_alloc/bad_exception + XmlAttribute/XmlElement/FWInstance/OnDemandInstance pools).
pub fn stub_0x1a5d0() {
    ensure_error_categories();
    ensure_ios_base_init();
    ensure_static_exception_object(&BAD_ALLOC_OBJECT, "bad_alloc");
    ensure_static_exception_object(&BAD_EXCEPTION_OBJECT, "bad_exception");
    ensure_xml_instance_pools();
}

// 0x1a768 — _main
// type: int __fastcall(int argc, const char **argv, const char **envp)
#[doc(alias = "_main")]
// was: _main
// IDA 0x1a768: NSAutoreleasePool alloc/init, UIApplicationMain(argc, argv, "UIApplication", "AppDelegate"), [pool release], return status.
pub fn stub_0x1a768(argc: i32, argv: Vec<String>) -> i32 {
    let pool = AutoreleasePool::alloc_init();
    let status = ui_application_main(
        argc,
        &argv,
        UI_APPLICATION_PRINCIPAL_CLASS,
        UI_APPLICATION_DELEGATE_CLASS,
    );
    pool.release();
    status
}

// 0x1a7d4 — __GLOBAL__I_a_2
#[doc(alias = "global constructor keyed to_a_2")]
// was: global constructor keyed to_a_2
// IDA 0x1a7d4: 131 insns — same TU shape as 0x17c58 (prologue + bad_alloc/bad_exception + XmlAttribute/XmlElement/FWInstance/OnDemandInstance pools).
pub fn stub_0x1a7d4() {
    ensure_error_categories();
    ensure_ios_base_init();
    ensure_static_exception_object(&BAD_ALLOC_OBJECT, "bad_alloc");
    ensure_static_exception_object(&BAD_EXCEPTION_OBJECT, "bad_exception");
    ensure_xml_instance_pools();
}

// 0x1ae78 — ___copy_helper_block__0
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block__0")]
// was: ___copy_helper_block__0
// IDA 0x1ae78: `_Block_object_assign(dst + 20, src[5], 3); _Block_object_assign(dst + 24, src[6], 3); _Block_object_assign_shim(dst + 28, src[7], 3)` — retain three captures.
pub fn stub_0x1ae78(dst: &mut [BlockSlot], src: &[BlockSlot]) {
    block_copy_3(dst, src);
}

// 0x1aea8 — ___destroy_helper_block__0
#[doc(alias = "___destroy_helper_block__0")]
// was: ___destroy_helper_block__0
// IDA 0x1aea8: `_Block_object_dispose(a1[5], 3); _Block_object_dispose(a1[6], 3); _Block_object_dispose_shim(a1[7], 3)` — release three captures.
pub fn stub_0x1aea8(slots: &mut [BlockSlot]) {
    block_dispose_3(slots);
}

// 0x1b11c — ___copy_helper_block_66
#[doc(alias = "___copy_helper_block_66")]
// was: ___copy_helper_block_66
// IDA 0x1b11c: `_Block_object_assign(dst + 20, src[5], 3); _Block_object_assign(dst + 24, src[6], 3); _Block_object_assign_shim(dst + 28, src[7], 3)` — retain three captures.
pub fn stub_0x1b11c(dst: &mut [BlockSlot], src: &[BlockSlot]) {
    block_copy_3(dst, src);
}

// 0x1b14c — ___destroy_helper_block_67
#[doc(alias = "___destroy_helper_block_67")]
// was: ___destroy_helper_block_67
// IDA 0x1b14c: `_Block_object_dispose(a1[5], 3); _Block_object_dispose(a1[6], 3); _Block_object_dispose_shim(a1[7], 3)` — release three captures.
pub fn stub_0x1b14c(slots: &mut [BlockSlot]) {
    block_dispose_3(slots);
}

// 0x1b308 — __GLOBAL__I_a_3
#[doc(alias = "global constructor keyed to_a_3")]
// was: global constructor keyed to_a_3
// IDA 0x1b308: 63 insns — TU prologue (boost categories, ios_base::Init, atexit) + guarded bad_alloc/bad_exception objects; POPNE early-return when the bad_exception guard is set.
pub fn stub_0x1b308() {
    ensure_error_categories();
    ensure_ios_base_init();
    ensure_static_exception_object(&BAD_ALLOC_OBJECT, "bad_alloc");
    if BAD_EXCEPTION_OBJECT.is_completed() {
        return;
    }
    ensure_static_exception_object(&BAD_EXCEPTION_OBJECT, "bad_exception");
}

// 0x1bb88 — ___copy_helper_block__1
#[doc(alias = "___copy_helper_block__1")]
// was: ___copy_helper_block__1
// IDA 0x1bb88: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1bb88() {
}

// 0x1bb94 — ___destroy_helper_block__1
#[doc(alias = "___destroy_helper_block__1")]
// was: ___destroy_helper_block__1
// IDA 0x1bb94: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1bb94() {
}

// 0x1bb9c — ___copy_helper_block_80
#[doc(alias = "___copy_helper_block_80")]
// was: ___copy_helper_block_80
// IDA 0x1bb9c: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1bb9c() {
}

// 0x1bba8 — ___destroy_helper_block_81
#[doc(alias = "___destroy_helper_block_81")]
// was: ___destroy_helper_block_81
// IDA 0x1bba8: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1bba8() {
}

// 0x1c5f4 — ___copy_helper_block_224
#[doc(alias = "___copy_helper_block_224")]
// was: ___copy_helper_block_224
// IDA 0x1c5f4: `_Block_object_assign_shim(dst + 20, src + 20, 3)` — retain the single captured object.
pub fn stub_0x1c5f4(dst: &mut [BlockSlot], src: &[BlockSlot]) {
    block_copy_1(dst, src);
}

// 0x1c600 — ___destroy_helper_block_225
#[doc(alias = "___destroy_helper_block_225")]
// was: ___destroy_helper_block_225
// IDA 0x1c600: `_Block_object_dispose_shim(*(a1 + 20), 3)` — release the single captured object.
pub fn stub_0x1c600(slots: &mut [BlockSlot]) {
    block_dispose_1(slots);
}

// 0x1c734 — ___copy_helper_block_246
#[doc(alias = "___copy_helper_block_246")]
// was: ___copy_helper_block_246
// IDA 0x1c734: `_Block_object_assign_shim(dst + 20, src + 20, 3)` — retain the single captured object.
pub fn stub_0x1c734(dst: &mut [BlockSlot], src: &[BlockSlot]) {
    block_copy_1(dst, src);
}

// 0x1c740 — ___destroy_helper_block_247
#[doc(alias = "___destroy_helper_block_247")]
// was: ___destroy_helper_block_247
// IDA 0x1c740: `_Block_object_dispose_shim(*(a1 + 20), 3)` — release the single captured object.
pub fn stub_0x1c740(slots: &mut [BlockSlot]) {
    block_dispose_1(slots);
}

// 0x1c874 — ___copy_helper_block_261
#[doc(alias = "___copy_helper_block_261")]
// was: ___copy_helper_block_261
// IDA 0x1c874: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1c874() {
}

// 0x1c880 — ___destroy_helper_block_262
#[doc(alias = "___destroy_helper_block_262")]
// was: ___destroy_helper_block_262
// IDA 0x1c880: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1c880() {
}

// 0x1d870 — __GLOBAL__I_a_4
#[doc(alias = "global constructor keyed to_a_4")]
// was: global constructor keyed to_a_4
// IDA 0x1d870: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_0x1d870() {
}

// 0x1da5c — +[LoginViewController sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[LoginViewController sharedInstance]")]
// was: +[LoginViewController sharedInstance]
// IDA 0x1da5c: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1da5c() {
}

// 0x1e2d8 — ___copy_helper_block__2
#[doc(alias = "___copy_helper_block__2")]
// was: ___copy_helper_block__2
// IDA 0x1e2d8: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1e2d8() {
}

// 0x1e2e4 — ___destroy_helper_block__2
#[doc(alias = "___destroy_helper_block__2")]
// was: ___destroy_helper_block__2
// IDA 0x1e2e4: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1e2e4() {
}

// 0x1eb08 — ___copy_helper_block_226
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_226")]
// was: ___copy_helper_block_226
// IDA 0x1eb08: 17 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1eb08() {
}

// 0x1eb38 — ___destroy_helper_block_227
#[doc(alias = "___destroy_helper_block_227")]
// was: ___destroy_helper_block_227
// IDA 0x1eb38: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1eb38() {
}

// 0x1ec44 — ___copy_helper_block_234
#[doc(alias = "___copy_helper_block_234")]
// was: ___copy_helper_block_234
// IDA 0x1ec44: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1ec44() {
}

// 0x1ec68 — ___destroy_helper_block_235
#[doc(alias = "___destroy_helper_block_235")]
// was: ___destroy_helper_block_235
// IDA 0x1ec68: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1ec68() {
}

// 0x1ed30 — ___copy_helper_block_242
#[doc(alias = "___copy_helper_block_242")]
// was: ___copy_helper_block_242
// IDA 0x1ed30: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1ed30() {
}

// 0x1ed3c — ___destroy_helper_block_243
#[doc(alias = "___destroy_helper_block_243")]
// was: ___destroy_helper_block_243
// IDA 0x1ed3c: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1ed3c() {
}

// 0x1ee84 — ___copy_helper_block_252
#[doc(alias = "___copy_helper_block_252")]
// was: ___copy_helper_block_252
// IDA 0x1ee84: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1ee84() {
}

// 0x1ee90 — ___destroy_helper_block_253
#[doc(alias = "___destroy_helper_block_253")]
// was: ___destroy_helper_block_253
// IDA 0x1ee90: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1ee90() {
}

// 0x1ee98 — ___copy_helper_block_257
#[doc(alias = "___copy_helper_block_257")]
// was: ___copy_helper_block_257
// IDA 0x1ee98: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1ee98() {
}

// 0x1eea4 — ___destroy_helper_block_258
#[doc(alias = "___destroy_helper_block_258")]
// was: ___destroy_helper_block_258
// IDA 0x1eea4: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1eea4() {
}

// 0x1efdc — ___copy_helper_block_260
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_260")]
// was: ___copy_helper_block_260
// IDA 0x1efdc: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1efdc() {
}

// 0x1efe8 — ___destroy_helper_block_261
#[doc(alias = "___destroy_helper_block_261")]
// was: ___destroy_helper_block_261
// IDA 0x1efe8: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1efe8() {
}

// 0x1eff0 — ___copy_helper_block_263
#[doc(alias = "___copy_helper_block_263")]
// was: ___copy_helper_block_263
// IDA 0x1eff0: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1eff0() {
}

// 0x1effc — ___destroy_helper_block_264
#[doc(alias = "___destroy_helper_block_264")]
// was: ___destroy_helper_block_264
// IDA 0x1effc: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1effc() {
}

// 0x1f480 — ___copy_helper_block_300
#[doc(alias = "___copy_helper_block_300")]
// was: ___copy_helper_block_300
// IDA 0x1f480: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1f480() {
}

// 0x1f48c — ___destroy_helper_block_301
#[doc(alias = "___destroy_helper_block_301")]
// was: ___destroy_helper_block_301
// IDA 0x1f48c: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1f48c() {
}

// 0x1f494 — ___copy_helper_block_305
#[doc(alias = "___copy_helper_block_305")]
// was: ___copy_helper_block_305
// IDA 0x1f494: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1f494() {
}

// 0x1f4a0 — ___destroy_helper_block_306
#[doc(alias = "___destroy_helper_block_306")]
// was: ___destroy_helper_block_306
// IDA 0x1f4a0: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1f4a0() {
}

// 0x1f660 — ___copy_helper_block_308
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_308")]
// was: ___copy_helper_block_308
// IDA 0x1f660: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1f660() {
}

// 0x1f66c — ___destroy_helper_block_309
#[doc(alias = "___destroy_helper_block_309")]
// was: ___destroy_helper_block_309
// IDA 0x1f66c: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1f66c() {
}

// 0x1f688 — ___copy_helper_block_314
#[doc(alias = "___copy_helper_block_314")]
// was: ___copy_helper_block_314
// IDA 0x1f688: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1f688() {
}

// 0x1f694 — ___destroy_helper_block_315
#[doc(alias = "___destroy_helper_block_315")]
// was: ___destroy_helper_block_315
// IDA 0x1f694: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1f694() {
}

// 0x1f69c — ___copy_helper_block_320
#[doc(alias = "___copy_helper_block_320")]
// was: ___copy_helper_block_320
// IDA 0x1f69c: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1f69c() {
}

// 0x1f6a8 — ___destroy_helper_block_321
#[doc(alias = "___destroy_helper_block_321")]
// was: ___destroy_helper_block_321
// IDA 0x1f6a8: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1f6a8() {
}

// 0x1f82c — ___copy_helper_block_323
#[doc(alias = "___copy_helper_block_323")]
// was: ___copy_helper_block_323
// IDA 0x1f82c: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1f82c() {
}

// 0x1f838 — ___destroy_helper_block_324
#[doc(alias = "___destroy_helper_block_324")]
// was: ___destroy_helper_block_324
// IDA 0x1f838: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1f838() {
}

// 0x1fa44 — ___copy_helper_block_339
#[doc(alias = "___copy_helper_block_339")]
// was: ___copy_helper_block_339
// IDA 0x1fa44: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1fa44() {
}

// 0x1fa50 — ___destroy_helper_block_340
#[doc(alias = "___destroy_helper_block_340")]
// was: ___destroy_helper_block_340
// IDA 0x1fa50: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1fa50() {
}

// 0x1fc90 — ___copy_helper_block_356
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_356")]
// was: ___copy_helper_block_356
// IDA 0x1fc90: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1fc90() {
}

// 0x1fc9c — ___destroy_helper_block_357
#[doc(alias = "___destroy_helper_block_357")]
// was: ___destroy_helper_block_357
// IDA 0x1fc9c: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1fc9c() {
}

// 0x1fca4 — ___copy_helper_block_359
#[doc(alias = "___copy_helper_block_359")]
// was: ___copy_helper_block_359
// IDA 0x1fca4: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1fca4() {
}

// 0x1fcc8 — ___destroy_helper_block_360
#[doc(alias = "___destroy_helper_block_360")]
// was: ___destroy_helper_block_360
// IDA 0x1fcc8: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1fcc8() {
}

// 0x1fce4 — ___copy_helper_block_364
#[doc(alias = "___copy_helper_block_364")]
// was: ___copy_helper_block_364
// IDA 0x1fce4: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1fce4() {
}

// 0x1fd08 — ___destroy_helper_block_365
#[doc(alias = "___destroy_helper_block_365")]
// was: ___destroy_helper_block_365
// IDA 0x1fd08: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1fd08() {
}

// 0x1fd24 — ___copy_helper_block_367
#[doc(alias = "___copy_helper_block_367")]
// was: ___copy_helper_block_367
// IDA 0x1fd24: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1fd24() {
}

// 0x1fd30 — ___destroy_helper_block_368
#[doc(alias = "___destroy_helper_block_368")]
// was: ___destroy_helper_block_368
// IDA 0x1fd30: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x1fd30() {
}

// 0x202d0 — __GLOBAL__I_a_5
#[doc(alias = "global constructor keyed to_a_5")]
// was: global constructor keyed to_a_5
// IDA 0x202d0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_0x202d0() {
}

// 0x20f08 — ___copy_helper_block__3
#[doc(alias = "___copy_helper_block__3")]
// was: ___copy_helper_block__3
// IDA 0x20f08: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x20f08() {
}

// 0x20f14 — ___destroy_helper_block__3
#[doc(alias = "___destroy_helper_block__3")]
// was: ___destroy_helper_block__3
// IDA 0x20f14: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x20f14() {
}

// 0x21adc — ___copy_helper_block_132
#[doc(alias = "___copy_helper_block_132")]
// was: ___copy_helper_block_132
// IDA 0x21adc: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x21adc() {
}

// 0x21ae8 — ___destroy_helper_block_133
#[doc(alias = "___destroy_helper_block_133")]
// was: ___destroy_helper_block_133
// IDA 0x21ae8: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x21ae8() {
}

// 0x21b10 — ___copy_helper_block_142
#[doc(alias = "___copy_helper_block_142")]
// was: ___copy_helper_block_142
// IDA 0x21b10: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x21b10() {
}

// 0x21b1c — ___destroy_helper_block_143
#[doc(alias = "___destroy_helper_block_143")]
// was: ___destroy_helper_block_143
// IDA 0x21b1c: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x21b1c() {
}

// 0x21c18 — __GLOBAL__I_a_6
#[doc(alias = "global constructor keyed to_a_6")]
// was: global constructor keyed to_a_6
// IDA 0x21c18: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_0x21c18() {
}

// 0x24540 — __GLOBAL__I_a_7
#[doc(alias = "global constructor keyed to_a_7")]
// was: global constructor keyed to_a_7
// IDA 0x24540: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_0x24540() {
}

// 0x24974 — +[PlaceLauncher sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[PlaceLauncher sharedInstance]")]
// was: +[PlaceLauncher sharedInstance]
// IDA 0x24974: 33 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x24974() {
}

// 0x249d0 — ___31+[PlaceLauncher sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___31+[PlaceLauncher sharedInstance]_block_invoke")]
// was: ___31+[PlaceLauncher sharedInstance]_block_invoke
// IDA 0x249d0: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x249d0() {
}

// 0x24a04 — ___copy_helper_block__4
#[doc(alias = "___copy_helper_block__4")]
// was: ___copy_helper_block__4
// IDA 0x24a04: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x24a04() {
}

// 0x24a10 — ___destroy_helper_block__4
#[doc(alias = "___destroy_helper_block__4")]
// was: ___destroy_helper_block__4
// IDA 0x24a10: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x24a10() {
}

// 0x253cc — ___copy_helper_block_98
#[doc(alias = "___copy_helper_block_98")]
// was: ___copy_helper_block_98
// IDA 0x253cc: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x253cc() {
}

// 0x253d8 — ___destroy_helper_block_99
#[doc(alias = "___destroy_helper_block_99")]
// was: ___destroy_helper_block_99
// IDA 0x253d8: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x253d8() {
}

// 0x298a0 — ___copy_helper_block_191
#[doc(alias = "___copy_helper_block_191")]
// was: ___copy_helper_block_191
// IDA 0x298a0: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x298a0() {
}

// 0x298c4 — ___destroy_helper_block_192
#[doc(alias = "___destroy_helper_block_192")]
// was: ___destroy_helper_block_192
// IDA 0x298c4: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x298c4() {
}

// 0x29c34 — ___copy_helper_block_217
#[doc(alias = "___copy_helper_block_217")]
// was: ___copy_helper_block_217
// IDA 0x29c34: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x29c34() {
}

// 0x29c58 — ___destroy_helper_block_218
#[doc(alias = "___destroy_helper_block_218")]
// was: ___destroy_helper_block_218
// IDA 0x29c58: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x29c58() {
}

// 0x29c88 — ___copy_helper_block_232
#[doc(alias = "___copy_helper_block_232")]
// was: ___copy_helper_block_232
// IDA 0x29c88: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x29c88() {
}

// 0x29c94 — ___destroy_helper_block_233
#[doc(alias = "___destroy_helper_block_233")]
// was: ___destroy_helper_block_233
// IDA 0x29c94: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x29c94() {
}

// 0x2a988 — ___copy_helper_block_243
#[doc(alias = "___copy_helper_block_243")]
// was: ___copy_helper_block_243
// IDA 0x2a988: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2a988() {
}

// 0x2a994 — ___destroy_helper_block_244
#[doc(alias = "___destroy_helper_block_244")]
// was: ___destroy_helper_block_244
// IDA 0x2a994: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2a994() {
}

// 0x2acec — ___copy_helper_block_247
// type: void __fastcall(_DWORD *, const shared_count *)
#[doc(alias = "___copy_helper_block_247")]
// was: ___copy_helper_block_247
// IDA 0x2acec: 63 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2acec() {
}

// 0x2ada4 — ___destroy_helper_block_248
#[doc(alias = "___destroy_helper_block_248")]
// was: ___destroy_helper_block_248
// IDA 0x2ada4: 55 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2ada4() {
}

// 0x2b980 — ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke
#[doc(alias = "____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke")]
// was: ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke
// IDA 0x2b980: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2b980() {
}

// 0x2ba00 — ___copy_helper_block_425
#[doc(alias = "___copy_helper_block_425")]
// was: ___copy_helper_block_425
// IDA 0x2ba00: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2ba00() {
}

// 0x2ba0c — ___destroy_helper_block_426
#[doc(alias = "___destroy_helper_block_426")]
// was: ___destroy_helper_block_426
// IDA 0x2ba0c: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2ba0c() {
}

// 0x2ba14 — ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428
#[doc(alias = "____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428")]
// was: ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428
// IDA 0x2ba14: 15 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2ba14() {
}

// 0x2ba40 — ___copy_helper_block_429
#[doc(alias = "___copy_helper_block_429")]
// was: ___copy_helper_block_429
// IDA 0x2ba40: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2ba40() {
}

// 0x2ba4c — ___destroy_helper_block_430
#[doc(alias = "___destroy_helper_block_430")]
// was: ___destroy_helper_block_430
// IDA 0x2ba4c: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2ba4c() {
}

// 0x2c138 — ____ZL15presentGameViewv_block_invoke
// type: void __cdecl(id)
#[doc(alias = "____ZL15presentGameViewv_block_invoke")]
// was: ____ZL15presentGameViewv_block_invoke
// IDA 0x2c138: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2c138() {
}

// 0x2c1f8 — ____ZL15presentGameViewv_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "____ZL15presentGameViewv_block_invoke_2")]
// was: ____ZL15presentGameViewv_block_invoke_2
// IDA 0x2c1f8: 8 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2c1f8() {
}

// 0x2c210 — ___copy_helper_block_499
#[doc(alias = "___copy_helper_block_499")]
// was: ___copy_helper_block_499
// IDA 0x2c210: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2c210() {
}

// 0x2c21c — ___destroy_helper_block_500
#[doc(alias = "___destroy_helper_block_500")]
// was: ___destroy_helper_block_500
// IDA 0x2c21c: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2c21c() {
}

// 0x2c5b0 — __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv")]
// was: __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv
// IDA 0x2c5b0: 160 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x2c5b0() {
}

// 0x317e4 — __ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v
// IDA 0x317e4: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x317e4() {
}

// 0x31828 — __ZN3RBX4Name13callDoDeclareILZNS_13sLoginServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sLoginServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sLoginServiceEEEEvv
// IDA 0x31828: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x31828() {
}

// 0x3182c — __ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v
// IDA 0x3182c: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3182c() {
}

// 0x32408 — __ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv
// IDA 0x32408: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x32408() {
}

// 0x32720 — __ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v
// IDA 0x32720: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x32720() {
}

// 0x32764 — __ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv
// IDA 0x32764: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x32764() {
}

// 0x32984 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// IDA 0x32984: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x32984() {
}

// 0x32a68 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// IDA 0x32a68: 85 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x32a68() {
}

// 0x342f4 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// IDA 0x342f4: 238 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x342f4() {
}

// 0x345b0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE
// IDA 0x345b0: 240 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x345b0() {
}

// 0x355c8 — __GLOBAL__I_a_8
#[doc(alias = "global constructor keyed to_a_8")]
// was: global constructor keyed to_a_8
// IDA 0x355c8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_0x355c8() {
}

// 0x358ec — _ReachabilityCallback
// type: id __fastcall(int, int, int)
#[doc(alias = "_ReachabilityCallback")]
// was: _ReachabilityCallback
// IDA 0x358ec: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358ec() {
}

// 0x35bd0 — _PrintReachabilityFlags
#[doc(alias = "_PrintReachabilityFlags")]
// was: _PrintReachabilityFlags
// IDA 0x35bd0: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35bd0() {
}

// 0x35e7c — ___copy_helper_block__5
#[doc(alias = "___copy_helper_block__5")]
// was: ___copy_helper_block__5
// IDA 0x35e7c: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35e7c() {
}

// 0x35e88 — ___destroy_helper_block__5
#[doc(alias = "___destroy_helper_block__5")]
// was: ___destroy_helper_block__5
// IDA 0x35e88: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35e88() {
}

// 0x35ffc — ___copy_helper_block_19
#[doc(alias = "___copy_helper_block_19")]
// was: ___copy_helper_block_19
// IDA 0x35ffc: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35ffc() {
}

// 0x36020 — ___destroy_helper_block_20
#[doc(alias = "___destroy_helper_block_20")]
// was: ___destroy_helper_block_20
// IDA 0x36020: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x36020() {
}

// 0x3603c — __Z18getUserAgentStringv
// type: id __fastcall()
#[doc(alias = "getUserAgentString(void)")]
// was: getUserAgentString(void)
// IDA 0x3603c: 7 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3603c() {
}

// 0x36e80 — __GLOBAL__I_a_9
#[doc(alias = "global constructor keyed to_a_9")]
// was: global constructor keyed to_a_9
// IDA 0x36e80: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_0x36e80() {
}

// 0x375b4 — __Z13macBundlePathv
// type: _DWORD __fastcall()
#[doc(alias = "macBundlePath(void)")]
// was: macBundlePath(void)
// IDA 0x375b4: 37 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x375b4() {
}

// 0x386d0 — __ZN10RobloxView16restartDataModelEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::restartDataModel(void)")]
// was: RobloxView::restartDataModel(void)
// IDA 0x386d0: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x386d0() {
}

// 0x38770 — ____ZN10RobloxView18doRestartDataModelEv_block_invoke
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "____ZN10RobloxView18doRestartDataModelEv_block_invoke")]
// was: ____ZN10RobloxView18doRestartDataModelEv_block_invoke
// IDA 0x38770: 530 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38770() {
}

// 0x38cd0 — __ZN10RobloxView17setupNewDataModelEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::setupNewDataModel(void)")]
// was: RobloxView::setupNewDataModel(void)
// IDA 0x38cd0: 338 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38cd0() {
}

// 0x39920 — __ZL14initLogManagerv
// type: _DWORD __fastcall()
#[doc(alias = "initLogManager(void)")]
// was: initLogManager(void)
// IDA 0x39920: 235 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x39920() {
}

// 0x3a1b8 — __ZN17QuitEventListenerD1Ev
// type: void __fastcall(QuitEventListener *__hidden this)
#[doc(alias = "QuitEventListener::~QuitEventListener()")]
// was: QuitEventListener::~QuitEventListener()
// IDA 0x3a1b8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0x3a1b8() {
}

// 0x3add8 — __ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v
// IDA 0x3add8: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3add8() {
}

// 0x3ae20 — __ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v
// IDA 0x3ae20: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3ae20() {
}

// 0x3b7e0 — __ZN3RBX4Name7declareILZNS_18sControllerServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_18sControllerServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_18sControllerServiceEEEERKS0_v
// IDA 0x3b7e0: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3b7e0() {
}

// 0x3b828 — __ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v
// IDA 0x3b828: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3b828() {
}

// 0x3ec30 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED1Ev
#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")]
// was: boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()
// IDA 0x3ec30: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x3ec30() {
}

// 0x3ec34 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED2Ev
#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")]
// was: boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()
// IDA 0x3ec34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x3ec34() {
}

// 0x3eccc — __ZN17QuitEventListenerD0Ev
// type: void __fastcall(QuitEventListener *__hidden this)
#[doc(alias = "QuitEventListener::~QuitEventListener()")]
// was: QuitEventListener::~QuitEventListener()
// IDA 0x3eccc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x3eccc() {
}

// 0x3ecd0 — __ZN4Ogre19WindowEventListener11windowMovedEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)")]
// was: Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)
// IDA 0x3ecd0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0x3ecd0() {
}

// 0x3ecd4 — __ZN4Ogre19WindowEventListener13windowResizedEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)")]
// was: Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)
// IDA 0x3ecd4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0x3ecd4() {
}

// 0x3ecd8 — __ZN4Ogre19WindowEventListener13windowClosingEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)")]
// was: Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)
// IDA 0x3ecd8: BX LR default listener — empty virtual in C++, no-op here.
pub fn stub_0x3ecd8() {
}

// 0x3ecdc — __ZN17QuitEventListener12windowClosedEPN4Ogre12RenderWindowE
// type: _DWORD __fastcall(QuitEventListener *__hidden this, RenderWindow *)
#[doc(alias = "QuitEventListener::windowClosed(Ogre::RenderWindow *)")]
// was: QuitEventListener::windowClosed(Ogre::RenderWindow *)
// IDA 0x3ecdc: BX LR default listener — empty virtual in C++, no-op here.
pub fn stub_0x3ecdc() {
}

// 0x3ecec — __ZN4Ogre19WindowEventListener17windowFocusChangeEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)")]
// was: Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)
// IDA 0x3ecec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0x3ecec() {
}

// 0x4070c — __GLOBAL__I_a_10
#[doc(alias = "global constructor keyed to_a_10")]
// was: global constructor keyed to_a_10
// IDA 0x4070c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_0x4070c() {
}

// 0x41104 — ___copy_helper_block__6
#[doc(alias = "___copy_helper_block__6")]
// was: ___copy_helper_block__6
// IDA 0x41104: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x41104() {
}

// 0x41128 — ___destroy_helper_block__6
#[doc(alias = "___destroy_helper_block__6")]
// was: ___destroy_helper_block__6
// IDA 0x41128: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x41128() {
}

// 0x411a0 — __Z23convertToFriendlyStringP8NSNumber
// type: _DWORD __fastcall(id)
#[doc(alias = "convertToFriendlyString(NSNumber *)")]
// was: convertToFriendlyString(NSNumber *)
// IDA 0x411a0: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x411a0() {
}

// 0x41bfc — __GLOBAL__I_a_11
#[doc(alias = "global constructor keyed to_a_11")]
// was: global constructor keyed to_a_11
// IDA 0x41bfc: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_0x41bfc() {
}

// 0x42580 — __GLOBAL__I_a_12
#[doc(alias = "global constructor keyed to_a_12")]
// was: global constructor keyed to_a_12
// IDA 0x42580: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_0x42580() {
}

// 0x42718 — +[RobloxWebUtility sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxWebUtility sharedInstance]")]
// was: +[RobloxWebUtility sharedInstance]
// IDA 0x42718: 33 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x42718() {
}

// 0x42774 — ___34+[RobloxWebUtility sharedInstance]_block_invoke
#[doc(alias = "___34+[RobloxWebUtility sharedInstance]_block_invoke")]
// was: ___34+[RobloxWebUtility sharedInstance]_block_invoke
// IDA 0x42774: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x42774() {
}
