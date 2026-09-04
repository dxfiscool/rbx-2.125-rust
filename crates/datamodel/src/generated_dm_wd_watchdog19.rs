// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: Instance|DataModel|Workspace|ServiceProvider EA-sorted asc next 100 uncovered not yet in crates/datamodel/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0xeccc..0x4542c0 | shard watchdog19
#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use crate::generated_05::{Instance, instance_is_a};
use crate::generated_189::CRenderSettingsItem;
use crate::generated_190::{RenderSettingsClass, RenderSettingsCreator};
use crate::instance::CornerWedgeInstance;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::generated_189::{
    HOLDER_CREATOR_TYPE, HOLDER_GEAR_GENRE_SETTING, HOLDER_GEAR_TYPE, HOLDER_GENRE,
    PlacementAny, TypedHolder, creator_type_holder, gear_genre_setting_holder,
    gear_type_holder, genre_holder,
};
use crate::instance::{EnumDesc, PhysicsService, ScriptService, stub_0x41d3d0, stub_0x41d590, stub_0x41d864, stub_0x41da24};
use std::sync::LazyLock;

/// Shared `GearType` name/value table (IDA `0x41da24` pairs); seeded once
/// from the canonical ctor.
static GEAR_TYPE_DESC: LazyLock<EnumDesc> = LazyLock::new(stub_0x41da24);
/// Shared `GearGenreSetting` table (IDA `0x41d864` pairs).
static GEAR_GENRE_DESC: LazyLock<EnumDesc> = LazyLock::new(stub_0x41d864);
/// Shared `Genre` table (IDA `0x41d590` pairs).
static GENRE_DESC: LazyLock<EnumDesc> = LazyLock::new(stub_0x41d590);
/// Shared `CreatorType` table (IDA `0x41d3d0` pairs).
static CREATOR_TYPE_DESC: LazyLock<EnumDesc> = LazyLock::new(stub_0x41d3d0);
/// IDA 0x41da24 table accessor for the `GearType` conversion suite below.
fn gear_type_desc() -> &'static EnumDesc {
    LazyLock::force(&GEAR_TYPE_DESC)
}
/// IDA 0x41d864 table accessor for the `GearGenreSetting` suite below.
fn gear_genre_desc() -> &'static EnumDesc {
    LazyLock::force(&GEAR_GENRE_DESC)
}
/// IDA 0x41d590 table accessor for the `Genre` suite below.
fn genre_desc() -> &'static EnumDesc {
    LazyLock::force(&GENRE_DESC)
}
/// IDA 0x41d3d0 table accessor for the `CreatorType` suite below.
fn creator_type_desc() -> &'static EnumDesc {
    LazyLock::force(&CREATOR_TYPE_DESC)
}
/// Rust model of `RBX::Reflection::ClassDescriptor` (IDA `0x4419a8` family):
/// the lazily built per-class descriptor hanging off the `Instance` base
/// descriptor. `__cxa_guard` / `__cxa_atexit` collapse into `LazyLock`, the
/// same treatment as `generated_190::RenderSettingsClass`.
#[derive(Debug)]
pub struct ClassDescriptor {
    pub name: &'static str,
}
/// IDA 0x4419a8 `classDescriptor()::s` for `BaseScript`.
static BASE_SCRIPT_CLASS: LazyLock<ClassDescriptor> =
    LazyLock::new(|| ClassDescriptor { name: "BaseScript" });
/// IDA 0x442248 `classDescriptor()::s` for `GuiImageButton`.
static GUI_IMAGE_BUTTON_CLASS: LazyLock<ClassDescriptor> =
    LazyLock::new(|| ClassDescriptor { name: "GuiImageButton" });
/// IDA 0x4426c8 `classDescriptor()::s` for `GuiBase`.
static GUI_BASE_CLASS: LazyLock<ClassDescriptor> =
    LazyLock::new(|| ClassDescriptor { name: "GuiBase" });
/// IDA 0x4430a0 `classDescriptor()::s` for `MegaClusterInstance`.
static MEGA_CLUSTER_CLASS: LazyLock<ClassDescriptor> =
    LazyLock::new(|| ClassDescriptor { name: "MegaClusterInstance" });
/// IDA 0x4497f4 `classDescriptor()::s` for `ScriptService`.
static SCRIPT_SERVICE_CLASS: LazyLock<ClassDescriptor> =
    LazyLock::new(|| ClassDescriptor { name: "ScriptService" });
/// Cached class index behind `doGetClassIndex<ScriptInformationProvider>`
/// (IDA `0x44c550`).
static SCRIPT_INFO_PROVIDER_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<DebrisService>` (IDA `0x44ca78`).
static DEBRIS_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<GamePassService>` (IDA `0x44d278`).
static GAME_PASS_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<SocialService>` (IDA `0x44da78`).
static SOCIAL_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<InsertService>` (IDA `0x44e22c`).
static INSERT_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<RenderHooksService>` (IDA `0x44e51c`).
static RENDER_HOOKS_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<FriendService>` (IDA `0x44edcc`).
static FRIEND_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<GeometryService>` (IDA `0x44f14c`).
static GEOMETRY_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<BadgeService>` (IDA `0x44fe78`).
static BADGE_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<PhysicsService>` (IDA `0x450340`).
static PHYSICS_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<CollectionService>` (IDA `0x451804`).
static COLLECTION_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<StarterGuiService>` (IDA `0x453588`).
static STARTER_GUI_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<StarterPackService>` (IDA `0x453c88`).
static STARTER_PACK_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// IDA 0x450dc0 `classDescriptor()::s` for `PhysicsService`.
static PHYSICS_SERVICE_CLASS: LazyLock<ClassDescriptor> =
    LazyLock::new(|| ClassDescriptor { name: "PhysicsService" });

/// Cached class index behind `doGetClassIndex<RunService>` (IDA `0x3af08`):
/// guard-once assignment from the provider counter, shared crate-wide via
/// `alloc_class_index` so no two classes collide (cf. `part::WORKSPACE_INDEX`).
static RUN_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// Cached class index behind `doGetClassIndex<ControllerService>` (IDA `0x3b910`).
static CONTROLLER_SERVICE_INDEX: AtomicUsize = AtomicUsize::new(0);

// 0xeccc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_eccc() {
    // IDA 0xeccc: `Creator::D2` — vtable install, `wasConstructed` assert,
    // `creators.erase(name)`. Canonical body lives at
    // `crate::generated_190::stub_0xeccc` (same bytes); delegate so the two
    // shards cannot drift.
    crate::generated_190::stub_0xeccc()
}

// 0xedfc — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_edfc() -> &'static str {
    // IDA 0xedfc: `Creator::getClassName` — assert + `Name::doDeclare`
    // tail-call returning "RenderSettings". See `generated_190::stub_0xedfc`.
    crate::generated_190::stub_0xedfc()
}

// 0xee84 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")]
pub fn stub_ee84() -> SharedPtr<CRenderSettingsItem> {
    // IDA 0xee84: `Creator::create` — assert + default-construct + adopt.
    // See `generated_190::stub_0xee84`.
    crate::generated_190::stub_0xee84()
}

// 0xf2bc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_f2bc() -> &'static RenderSettingsCreator {
    // IDA 0xf2bc: `Creator::C2` — registry insert + `isConstructed = 666`.
    // See `generated_190::stub_0xf2bc`.
    crate::generated_190::stub_0xf2bc()
}

// 0xf500 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_f500() -> &'static RenderSettingsCreator {
    // IDA 0xf500: `static_getCreator` — assert + return `&creatorPrivate`.
    // See `generated_190::stub_0xf500`.
    crate::generated_190::stub_0xf500()
}

// 0xfa00 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_fa00() -> &'static RenderSettingsClass {
    // IDA 0xfa00: `Described<CRenderSettingsItem,...>::classDescriptor()`.
    // See `generated_190::stub_0xfa00`.
    crate::generated_190::stub_0xfa00()
}

// 0xfb1c — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_fb1c(item: &mut CRenderSettingsItem) {
    // IDA 0xfb1c: `Described<CRenderSettingsItem,...>::D1` thunk into
    // `Instance::~Instance`. See `generated_190::stub_0xfb1c`.
    crate::generated_190::stub_0xfb1c(item)
}

// 0xfb20 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_fb20(item: &mut CRenderSettingsItem) {
    // IDA 0xfb20: `Described<CRenderSettingsItem,...>::D0` + delete.
    // See `generated_190::stub_0xfb20`.
    crate::generated_190::stub_0xfb20(item)
}

// 0xfb34 — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_fb34(item: &mut CRenderSettingsItem) {
    // IDA 0xfb34: `ZThn32` D1 — `this -= 32`, then D1.
    // See `generated_190::stub_0xfb34`.
    crate::generated_190::stub_0xfb34(item)
}

// 0xfb3c — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_fb3c(item: &mut CRenderSettingsItem) {
    // IDA 0xfb3c: `ZThn32` D0 — `this -= 32`, D0 + delete.
    // See `generated_190::stub_0xfb3c`.
    crate::generated_190::stub_0xfb3c(item)
}

// 0xfb54 — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_fb54(item: &mut CRenderSettingsItem) {
    // IDA 0xfb54: `ZThn36` D1 — `this -= 36`, then D1.
    // See `generated_190::stub_0xfb54`.
    crate::generated_190::stub_0xfb54(item)
}

// 0xfb5c — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_fb5c(item: &mut CRenderSettingsItem) {
    // IDA 0xfb5c: `ZThn36` D0 — `this -= 36`, D0 + delete.
    // See `generated_190::stub_0xfb5c`.
    crate::generated_190::stub_0xfb5c(item)
}

// 0x3af08 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10RunServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_10RunServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RunService>(void)")]
pub fn stub_3af08() -> usize {
    // IDA 0x3af08 (decompiled): `doGetClassIndex<RunService>` — guard-once
    // static `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as `instance::stub_0x3ff958`; the guard collapses into a
    // 0-sentinel atomic over the shared crate counter.
    if RUN_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        RUN_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    RUN_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x3b518 — __ZNK3RBX15ServiceProvider4findINS_17ControllerServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17ControllerServiceEEEPT_v")]
#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::find<RBX::ControllerService>(void)const")]
pub fn stub_3b518(instance: *const Instance) -> *const Instance {
    // IDA 0x3b518 (disasm): `find<ControllerService>` — provider search then
    // class scan, null on miss. Same root-walk + pre-order shape as
    // `instance::stub_0x3ff614`, matching the `ControllerService` class.
    // SAFETY: `instance` must be null or point to a valid `Instance` whose
    // whole ancestry/subtree outlives the call.
    unsafe {
        let mut root = instance;
        while !root.is_null() && !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if root.is_null() {
            return core::ptr::null();
        }
        let mut stack: Vec<*const Instance> = vec![root];
        while let Some(node) = stack.pop() {
            if instance_is_a(node, "ControllerService") {
                return node;
            }
            let mut children: Vec<*const Instance> = (*node)
                .children
                .iter()
                .map(|child| SharedPtr::as_ptr(child) as *const Instance)
                .collect();
            children.reverse();
            stack.extend(children);
        }
        core::ptr::null()
    }
}

// 0x3b910 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17ControllerServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17ControllerServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ControllerService>(void)")]
pub fn stub_3b910() -> usize {
    // IDA 0x3b910: `doGetClassIndex<ControllerService>` — guard-once static
    // `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if CONTROLLER_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        CONTROLLER_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    CONTROLLER_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x2b7568 — __ZN3RBX4Name7declareILZNS_10sWorkspaceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sWorkspaceEEEERKS0_v")]
pub fn stub_2b7568() -> &'static str {
    // IDA 0x2b7568 (disasm): `Name::declare<sWorkspace>` — loads
    // `RBX::sWorkspace` ("Workspace", 0x2b7576..0x2b7578), null-checks it
    // (0x2b757a), declares/interns on the hit path. The interned name is
    // the "Workspace" literal (same shape as `instance::stub_0x31c30`).
    "Workspace"
}

// 0x2b75b0 — __ZN3RBX4Name9doDeclareILZNS_10sWorkspaceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sWorkspaceEEEERKS0_v")]
pub fn stub_2b75b0() -> &'static str {
    // IDA 0x2b75b0: `Name::doDeclare<sWorkspace>` — the declare worker
    // behind 0x2b7568; returns the interned "Workspace" name.
    stub_2b7568()
}

// 0x418c98 — __ZThn32_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_418c98(_item: &mut CornerWedgeInstance) {
    // IDA 0x418c98: `ZThn32` D1 of `DescribedCreatable<CornerWedgeInstance>`
    // — `this -= 32` selects the subobject, then the `Instance` D2.
    // The adjustment collapses (single modelled address space); member
    // drops collapse into Rust drop. Drop glue, no-op.
}

// 0x418cac — __ZThn32_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_418cac(_item: &mut CornerWedgeInstance) {
    // IDA 0x418cac: `ZThn32` D0 — `this -= 32`, D2, `operator delete`.
    // The free collapses into Rust ownership (caller drops the box).
}

// 0x418d60 — __ZThn36_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_418d60(_item: &mut CornerWedgeInstance) {
    // IDA 0x418d60: `ZThn36` D1 — `this -= 36`, then the `Instance` D2.
    // Same collapse as 0x418c98.
}

// 0x418d74 — __ZThn36_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_418d74(_item: &mut CornerWedgeInstance) {
    // IDA 0x418d74: `ZThn36` D0 — `this -= 36`, D2, `operator delete`.
    // Same collapse as 0x418cac.
}

// 0x418e94 — __ZThn32_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_418e94(_item: &mut CornerWedgeInstance) {
    // IDA 0x418e94: `ZThn32` D1 of `Described<CornerWedgeInstance,...>` —
    // `this -= 32`, then the `Instance` D2. Same collapse as 0x418c98.
}

// 0x418ea8 — __ZThn32_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_418ea8(_item: &mut CornerWedgeInstance) {
    // IDA 0x418ea8: `ZThn32` D0 — `this -= 32`, D2, `operator delete`.
    // Same collapse as 0x418cac.
}

// 0x418f5c — __ZThn36_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_418f5c(_item: &mut CornerWedgeInstance) {
    // IDA 0x418f5c: `ZThn36` D1 — `this -= 36`, then the `Instance` D2.
    // Same collapse as 0x418c98.
}

// 0x418f70 — __ZThn36_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_418f70(_item: &mut CornerWedgeInstance) {
    // IDA 0x418f70: `ZThn36` D0 — `this -= 36`, D2, `operator delete`.
    // Same collapse as 0x418cac.
}

// 0x439cf0 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE13convertToItemERKS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE13convertToItemERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToItem(RBX::DataModel::GearType const&)const")]
pub fn stub_439cf0(value: i32) -> i32 {
    // IDA 0x439cf0: `EnumDesc<GearType>::convertToItem` — `ReleaseAssert`
    // (`value >= 0`, enumconverter.h:273) that falls through, then the
    // `enumToItem` table hit. The table is dense 0..8 (IDA 0x41da24), so a
    // hit returns the value itself; a miss falls back to 0, the same
    // collapse as `generated_189::stub_0xc5ac`.
    debug_assert!(value >= 0, "0x439cf0: value>=0 (enumconverter.h:273)");
    gear_type_desc().pairs.iter().find(|(v, _)| *v == value).map(|(v, _)| *v).unwrap_or(0)
}

// 0x439dbc — __ZN3rbx8any_castIRKN3RBX9DataModel8GearTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX9DataModel8GearTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::DataModel::GearType const& rbx::any_cast<RBX::DataModel::GearType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_439dbc(slot: &PlacementAny) -> i32 {
    // IDA 0x439dbc: `any_cast<GearType const&, Region3>` — holder check
    // with a typeinfo-name fallback; mismatch throws
    // `rbx::bad_placement_any_cast` (a throw becomes a panic here), hit
    // returns the payload word. Same shape as `generated_189::stub_0xcaa4`.
    if slot.holder != HOLDER_GEAR_TYPE {
        panic!("rbx::bad_placement_any_cast for N3RBX9DataModel8GearTypeE");
    }
    slot.value
}

// 0x439eac — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueERKNS_4NameERS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToValue(RBX::Name const&,RBX::DataModel::GearType&)const")]
pub fn stub_439eac(name: &str, out: &mut i32) -> bool {
    // IDA 0x439eac: `EnumDesc<GearType>::convertToValue` — map search by
    // name id over the 0x41da24 table; hit stores the value and returns 1,
    // miss returns 0. `Name::lookup` collapses into the `&str` itself.
    // Same shape as `instance::stub_0x3bd850`.
    match gear_type_desc().pairs.iter().find(|(_, n)| *n == name) {
        Some((value, _)) => {
            *out = *value;
            true
        }
        None => false,
    }
}

// 0x439f28 — __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED2Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::~EnumDesc()")]
pub fn stub_439f28(_desc: &mut EnumDesc) {
    // IDA 0x439f28: `EnumDesc<GearType>::~EnumDesc` — vtable install plus
    // the pair-vector/registrar teardown. The table lives in a `LazyLock`
    // that owns its storage to process exit, so drops collapse into Rust
    // ownership. Drop glue, no-op.
}

// 0x43a0fc — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringERKS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToString(RBX::DataModel::GearGenreSetting const&)const")]
pub fn stub_43a0fc(value: i32, out: &mut String) {
    // IDA 0x43a0fc: `EnumDesc<GearGenreSetting>::convertToString` —
    // `ReleaseAssert(value >= 0)` (:262) and `ReleaseAssert(value <
    // enumToItem.size())` (:263) that fall through, then
    // `*out = OOB ? "" : enumToItem[value]`. Same shape as
    // `generated_189::stub_0xc76c`.
    debug_assert!(value >= 0, "0x43a0fc: value>=0 (enumconverter.h:262)");
    match (value >= 0).then(|| gear_genre_desc().pairs.iter().find(|(v, _)| *v == value)).flatten() {
        Some((_, name)) => *out = (*name).to_owned(),
        None => out.clear(),
    }
}

// 0x43a29c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel16GearGenreSettingEEERS3_RKT_
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel16GearGenreSettingEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::GearGenreSetting>(RBX::DataModel::GearGenreSetting const&)")]
pub fn stub_43a29c(slot: &mut PlacementAny, value: i32) -> &mut PlacementAny {
    // IDA 0x43a29c: `placement_any<Region3>::operator=<GearGenreSetting>` —
    // singleton touch (0x43a2a8), same-holder copy (0x43a2d4), else destruct
    // (0x43a2c8) / clear (0x43a2cc) / copy (0x43a2de) / install (0x43a2e0).
    // Same shape as `generated_189::stub_0xceec`.
    let _ = gear_genre_setting_holder();
    if slot.holder == HOLDER_GEAR_GENRE_SETTING {
        slot.value = value;
    } else {
        slot.holder = 0;
        slot.value = value;
        slot.holder = HOLDER_GEAR_GENRE_SETTING;
    }
    slot
}

// 0x43a2ec — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE9singletonEv
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::singleton(void)")]
pub fn stub_43a2ec() -> &'static TypedHolder {
    // IDA 0x43a2ec: `typed_holder<GearGenreSetting>::singleton` —
    // `__cxa_guard`-checked init of `s = { typeinfo, destruct_func,
    // construct_func }`, then return `&s`. Same shape as
    // `generated_190::stub_0xcf3c`; homed on the shared `LazyLock` model in
    // `generated_189`.
    gear_genre_setting_holder()
}

// 0x43a358 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE14construct_funcEPKcPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE14construct_funcEPKcPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::construct_func(char const*,char *)")]
pub fn stub_43a358(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0x43a358: `typed_holder<GearGenreSetting>::construct_func` —
    // `if (dst) { value = *src; *dst = value; } return value`. Same shape
    // as `generated_190::stub_0xcfa8`.
    // SAFETY: `src` must be readable; `dst` must be writable when non-null.
    unsafe {
        let value = *src;
        if !dst.is_null() {
            *dst = value;
        }
        value
    }
}

// 0x43a364 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::destruct_func(char *)")]
pub fn stub_43a364() {
    // IDA 0x43a364: `typed_holder<GearGenreSetting>::destruct_func` — empty;
    // trivial enum payload, nothing to destroy.
}

// 0x43a368 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE13convertToItemERKS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE13convertToItemERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToItem(RBX::DataModel::GearGenreSetting const&)const")]
pub fn stub_43a368(value: i32) -> i32 {
    // IDA 0x43a368: `EnumDesc<GearGenreSetting>::convertToItem` — same
    // assert + table-hit shape as 0x439cf0 over the 0x41d864 table.
    debug_assert!(value >= 0, "0x43a368: value>=0 (enumconverter.h:273)");
    gear_genre_desc().pairs.iter().find(|(v, _)| *v == value).map(|(v, _)| *v).unwrap_or(0)
}

// 0x43a434 — __ZN3rbx8any_castIRKN3RBX9DataModel16GearGenreSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX9DataModel16GearGenreSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::DataModel::GearGenreSetting const& rbx::any_cast<RBX::DataModel::GearGenreSetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_43a434(slot: &PlacementAny) -> i32 {
    // IDA 0x43a434: `any_cast<GearGenreSetting const&, Region3>` — same
    // holder-check + panic shape as 0x439dbc.
    if slot.holder != HOLDER_GEAR_GENRE_SETTING {
        panic!("rbx::bad_placement_any_cast for N3RBX9DataModel16GearGenreSettingE");
    }
    slot.value
}

// 0x43a524 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueERKNS_4NameERS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToValue(RBX::Name const&,RBX::DataModel::GearGenreSetting&)const")]
pub fn stub_43a524(name: &str, out: &mut i32) -> bool {
    // IDA 0x43a524: `EnumDesc<GearGenreSetting>::convertToValue` — same
    // map-search shape as 0x439eac over the 0x41d864 table.
    match gear_genre_desc().pairs.iter().find(|(_, n)| *n == name) {
        Some((value, _)) => {
            *out = *value;
            true
        }
        None => false,
    }
}

// 0x43a5a0 — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED2Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::~EnumDesc()")]
pub fn stub_43a5a0(_desc: &mut EnumDesc) {
    // IDA 0x43a5a0: `EnumDesc<GearGenreSetting>::~EnumDesc` — same drop-glue
    // shape as 0x439f28.
}

// 0x43a774 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringERKS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToString(RBX::DataModel::Genre const&)const")]
pub fn stub_43a774(value: i32, out: &mut String) {
    // IDA 0x43a774: `EnumDesc<Genre>::convertToString` — same assert +
    // empty-on-miss shape as 0x43a0fc over the 0x41d590 table.
    debug_assert!(value >= 0, "0x43a774: value>=0 (enumconverter.h:262)");
    match (value >= 0).then(|| genre_desc().pairs.iter().find(|(v, _)| *v == value)).flatten() {
        Some((_, name)) => *out = (*name).to_owned(),
        None => out.clear(),
    }
}

// 0x43a914 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel5GenreEEERS3_RKT_
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel5GenreEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::Genre>(RBX::DataModel::Genre const&)")]
pub fn stub_43a914(slot: &mut PlacementAny, value: i32) -> &mut PlacementAny {
    // IDA 0x43a914: `placement_any<Region3>::operator=<Genre>` — same
    // singleton-touch / store shape as 0x43a29c for the `Genre` holder.
    let _ = genre_holder();
    if slot.holder == HOLDER_GENRE {
        slot.value = value;
    } else {
        slot.holder = 0;
        slot.value = value;
        slot.holder = HOLDER_GENRE;
    }
    slot
}

// 0x43a964 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE9singletonEv
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::Genre>::singleton(void)")]
pub fn stub_43a964() -> &'static TypedHolder {
    // IDA 0x43a964: `typed_holder<Genre>::singleton` — same
    // `__cxa_guard`-init shape as 0x43a2ec; homed on the shared `LazyLock`
    // model in `generated_189`.
    genre_holder()
}

// 0x43a9d0 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE14construct_funcEPKcPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE14construct_funcEPKcPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::Genre>::construct_func(char const*,char *)")]
pub fn stub_43a9d0(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0x43a9d0: `typed_holder<Genre>::construct_func` — same copy
    // shape as 0x43a358.
    // SAFETY: `src` must be readable; `dst` must be writable when non-null.
    unsafe {
        let value = *src;
        if !dst.is_null() {
            *dst = value;
        }
        value
    }
}

// 0x43a9dc — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::Genre>::destruct_func(char *)")]
pub fn stub_43a9dc() {
    // IDA 0x43a9dc: `typed_holder<Genre>::destruct_func` — empty; same
    // shape as 0x43a364.
}

// 0x43a9e0 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE13convertToItemERKS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE13convertToItemERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToItem(RBX::DataModel::Genre const&)const")]
pub fn stub_43a9e0(value: i32) -> i32 {
    // IDA 0x43a9e0: `EnumDesc<Genre>::convertToItem` — same assert +
    // table-hit shape as 0x439cf0 over the 0x41d590 table.
    debug_assert!(value >= 0, "0x43a9e0: value>=0 (enumconverter.h:273)");
    genre_desc().pairs.iter().find(|(v, _)| *v == value).map(|(v, _)| *v).unwrap_or(0)
}

// 0x43aaac — __ZN3rbx8any_castIRKN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::DataModel::Genre const& rbx::any_cast<RBX::DataModel::Genre const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_43aaac(slot: &PlacementAny) -> i32 {
    // IDA 0x43aaac: `any_cast<Genre const&, Region3>` — same holder-check
    // + panic shape as 0x439dbc.
    if slot.holder != HOLDER_GENRE {
        panic!("rbx::bad_placement_any_cast for N3RBX9DataModel5GenreE");
    }
    slot.value
}

// 0x43ab9c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueERKNS_4NameERS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToValue(RBX::Name const&,RBX::DataModel::Genre&)const")]
pub fn stub_43ab9c(name: &str, out: &mut i32) -> bool {
    // IDA 0x43ab9c: `EnumDesc<Genre>::convertToValue` — same map-search
    // shape as 0x439eac over the 0x41d590 table.
    match genre_desc().pairs.iter().find(|(_, n)| *n == name) {
        Some((value, _)) => {
            *out = *value;
            true
        }
        None => false,
    }
}

// 0x43ac18 — __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED2Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::~EnumDesc()")]
pub fn stub_43ac18(_desc: &mut EnumDesc) {
    // IDA 0x43ac18: `EnumDesc<Genre>::~EnumDesc` — same drop-glue shape as
    // 0x439f28.
}

// 0x43adec — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringERKS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToString(RBX::DataModel::CreatorType const&)const")]
pub fn stub_43adec(value: i32, out: &mut String) {
    // IDA 0x43adec: `EnumDesc<CreatorType>::convertToString` — same assert
    // + empty-on-miss shape as 0x43a0fc over the 0x41d3d0 table.
    debug_assert!(value >= 0, "0x43adec: value>=0 (enumconverter.h:262)");
    match (value >= 0).then(|| creator_type_desc().pairs.iter().find(|(v, _)| *v == value)).flatten() {
        Some((_, name)) => *out = (*name).to_owned(),
        None => out.clear(),
    }
}

// 0x4419a8 — __ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4419a8() -> &'static ClassDescriptor {
    // IDA 0x4419a8: `Described<BaseScript,...>::classDescriptor()` —
    // `__cxa_guard_acquire` once-check, base
    // `Described<Instance>::classDescriptor()` touch, `ClassDescriptor` C2
    // with ("BaseScript", base), `__cxa_atexit`, guard release, return the
    // static. Guard/atexit collapse into `LazyLock`, the same shape as
    // `generated_190::stub_0xfa00`.
    LazyLock::force(&BASE_SCRIPT_CLASS)
}

// 0x442248 — __ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_442248() -> &'static ClassDescriptor {
    // IDA 0x442248: `Described<GuiImageButton,...>::classDescriptor()` —
    // same once-shape as 0x4419a8 with ("GuiImageButton", base).
    LazyLock::force(&GUI_IMAGE_BUTTON_CLASS)
}

// 0x4426c8 — __ZN3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4426c8() -> &'static ClassDescriptor {
    // IDA 0x4426c8: `Described<GuiBase,...>::classDescriptor()` — same
    // once-shape as 0x4419a8 with ("GuiBase", base).
    LazyLock::force(&GUI_BASE_CLASS)
}

// 0x4430a0 — __ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4430a0() -> &'static ClassDescriptor {
    // IDA 0x4430a0: `Described<MegaClusterInstance,...>::classDescriptor()`
    // — same once-shape as 0x4419a8 with ("MegaClusterInstance", base).
    LazyLock::force(&MEGA_CLUSTER_CLASS)
}

// 0x4497f4 — __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4497f4() -> &'static ClassDescriptor {
    // IDA 0x4497f4: `Described<ScriptService,...>::classDescriptor()` —
    // same once-shape as 0x4419a8 with ("ScriptService", base).
    LazyLock::force(&SCRIPT_SERVICE_CLASS)
}

// 0x449914 — __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_449914(_service: &mut ScriptService) {
    // IDA 0x449914: `Described<ScriptService,...>::D1` tail-calls
    // `Instance::~Instance`. Member drops collapse into Rust drop; drop
    // glue, no-op. Same shape as `generated_190::stub_0xfb1c`.
}

// 0x449918 — __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_449918(_service: &mut ScriptService) {
    // IDA 0x449918: `Described<ScriptService,...>::D0` — `Instance` D2
    // then `operator delete`. The free collapses into Rust ownership.
    // Same shape as `generated_190::stub_0xfb20`.
}

// 0x4499b8 — __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4499b8(_service: &mut ScriptService) {
    // IDA 0x4499b8: `ZThn32` D1 — `this -= 32`, then the D1 above. Same
    // collapse as `generated_190::stub_0xfb34`.
}

// 0x4499c0 — __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4499c0(_service: &mut ScriptService) {
    // IDA 0x4499c0: `ZThn32` D0 — `this -= 32`, D0 + delete. Same collapse
    // as `generated_190::stub_0xfb3c`.
}

// 0x449a64 — __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_449a64(_service: &mut ScriptService) {
    // IDA 0x449a64: `ZThn36` D1 — `this -= 36`, then the D1 above. Same
    // collapse as `generated_190::stub_0xfb54`.
}

// 0x449a6c — __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_449a6c(_service: &mut ScriptService) {
    // IDA 0x449a6c: `ZThn36` D0 — `this -= 36`, D0 + delete. Same collapse
    // as `generated_190::stub_0xfb5c`.
}

// 0x44c550 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_25ScriptInformationProviderEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_25ScriptInformationProviderEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptInformationProvider>(void)")]
pub fn stub_44c550() -> usize {
    // IDA 0x44c550: `doGetClassIndex<ScriptInformationProvider>` —
    // guard-once static `index = ServiceProvider::newIndex(1)`, then the
    // cached index. Same shape as 0x3af08 above.
    if SCRIPT_INFO_PROVIDER_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        SCRIPT_INFO_PROVIDER_INDEX.store(fresh, Ordering::Relaxed);
    }
    SCRIPT_INFO_PROVIDER_INDEX.load(Ordering::Relaxed)
}

// 0x44c6f0 — __ZNK3RBX15ServiceProvider4findINS_13DebrisServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_13DebrisServiceEEEPT_v")]
#[doc(alias = "RBX::DebrisService * RBX::ServiceProvider::find<RBX::DebrisService>(void)const")]
pub fn stub_44c6f0(instance: *const Instance) -> *const Instance {
    // IDA 0x44c6f0: `find<DebrisService>` — provider search then class
    // scan, null on miss. Same root-walk + pre-order shape as
    // `instance::stub_0x3ff614`, matching the `DebrisService` class.
    // SAFETY: `instance` must be null or point to a valid `Instance` whose
    // whole ancestry/subtree outlives the call.
    unsafe {
        let mut root = instance;
        while !root.is_null() && !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if root.is_null() {
            return core::ptr::null();
        }
        let mut stack: Vec<*const Instance> = vec![root];
        while let Some(node) = stack.pop() {
            if instance_is_a(node, "DebrisService") {
                return node;
            }
            let mut children: Vec<*const Instance> = (*node)
                .children
                .iter()
                .map(|child| SharedPtr::as_ptr(child) as *const Instance)
                .collect();
            children.reverse();
            stack.extend(children);
        }
        core::ptr::null()
    }
}

// 0x44ca74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13DebrisServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13DebrisServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::DebrisService>(void)")]
pub fn stub_44ca74() -> usize {
    // IDA 0x44ca74: `callDoGetClassIndex<DebrisService>` — forwards to the
    // cached `doGetClassIndex`. Same shape as `instance::stub_0x3ff954`.
    stub_44ca78()
}

// 0x44ca78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13DebrisServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13DebrisServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::DebrisService>(void)")]
pub fn stub_44ca78() -> usize {
    // IDA 0x44ca78: `doGetClassIndex<DebrisService>` — guard-once static
    // `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if DEBRIS_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        DEBRIS_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    DEBRIS_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x44cef0 — __ZNK3RBX15ServiceProvider4findINS_15GamePassServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_15GamePassServiceEEEPT_v")]
#[doc(alias = "RBX::GamePassService * RBX::ServiceProvider::find<RBX::GamePassService>(void)const")]
pub fn stub_44cef0(instance: *const Instance) -> *const Instance {
    // IDA 0x44cef0: `find<GamePassService>` — same root-walk + pre-order
    // shape as 0x44c6f0, matching the `GamePassService` class.
    // SAFETY: same contract as 0x44c6f0.
    unsafe {
        let mut root = instance;
        while !root.is_null() && !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if root.is_null() {
            return core::ptr::null();
        }
        let mut stack: Vec<*const Instance> = vec![root];
        while let Some(node) = stack.pop() {
            if instance_is_a(node, "GamePassService") {
                return node;
            }
            let mut children: Vec<*const Instance> = (*node)
                .children
                .iter()
                .map(|child| SharedPtr::as_ptr(child) as *const Instance)
                .collect();
            children.reverse();
            stack.extend(children);
        }
        core::ptr::null()
    }
}

// 0x44d274 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GamePassServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GamePassServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GamePassService>(void)")]
pub fn stub_44d274() -> usize {
    // IDA 0x44d274: `callDoGetClassIndex<GamePassService>` — forwards to
    // the cached `doGetClassIndex`. Same shape as 0x44ca74.
    stub_44d278()
}

// 0x44d278 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GamePassService>(void)")]
pub fn stub_44d278() -> usize {
    // IDA 0x44d278: `doGetClassIndex<GamePassService>` — guard-once static
    // `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if GAME_PASS_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        GAME_PASS_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    GAME_PASS_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x44d6f0 — __ZNK3RBX15ServiceProvider4findINS_13SocialServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_13SocialServiceEEEPT_v")]
#[doc(alias = "RBX::SocialService * RBX::ServiceProvider::find<RBX::SocialService>(void)const")]
pub fn stub_44d6f0(instance: *const Instance) -> *const Instance {
    // IDA 0x44d6f0: `find<SocialService>` — same root-walk + pre-order
    // shape as 0x44c6f0, matching the `SocialService` class.
    // SAFETY: same contract as 0x44c6f0.
    unsafe {
        let mut root = instance;
        while !root.is_null() && !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if root.is_null() {
            return core::ptr::null();
        }
        let mut stack: Vec<*const Instance> = vec![root];
        while let Some(node) = stack.pop() {
            if instance_is_a(node, "SocialService") {
                return node;
            }
            let mut children: Vec<*const Instance> = (*node)
                .children
                .iter()
                .map(|child| SharedPtr::as_ptr(child) as *const Instance)
                .collect();
            children.reverse();
            stack.extend(children);
        }
        core::ptr::null()
    }
}

// 0x44da74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13SocialServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13SocialServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::SocialService>(void)")]
pub fn stub_44da74() -> usize {
    // IDA 0x44da74: `callDoGetClassIndex<SocialService>` — forwards to the
    // cached `doGetClassIndex`. Same shape as 0x44ca74.
    stub_44da78()
}

// 0x44da78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::SocialService>(void)")]
pub fn stub_44da78() -> usize {
    // IDA 0x44da78: `doGetClassIndex<SocialService>` — guard-once static
    // `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if SOCIAL_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        SOCIAL_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    SOCIAL_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x44e228 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13InsertServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13InsertServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::InsertService>(void)")]
pub fn stub_44e228() -> usize {
    // IDA 0x44e228: `callDoGetClassIndex<InsertService>` — forwards to the
    // cached `doGetClassIndex`. Same shape as 0x44ca74.
    stub_44e22c()
}

// 0x44e22c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::InsertService>(void)")]
pub fn stub_44e22c() -> usize {
    // IDA 0x44e22c: `doGetClassIndex<InsertService>` — guard-once static
    // `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if INSERT_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        INSERT_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    INSERT_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x44e518 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_18RenderHooksServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_18RenderHooksServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::RenderHooksService>(void)")]
pub fn stub_44e518() -> usize {
    // IDA 0x44e518: `callDoGetClassIndex<RenderHooksService>` — forwards to
    // the cached `doGetClassIndex`. Same shape as 0x44ca74.
    stub_44e51c()
}

// 0x44e51c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18RenderHooksServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_18RenderHooksServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RenderHooksService>(void)")]
pub fn stub_44e51c() -> usize {
    // IDA 0x44e51c: `doGetClassIndex<RenderHooksService>` — guard-once
    // static `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if RENDER_HOOKS_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        RENDER_HOOKS_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    RENDER_HOOKS_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x44edc8 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13FriendServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13FriendServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FriendService>(void)")]
pub fn stub_44edc8() -> usize {
    // IDA 0x44edc8: `callDoGetClassIndex<FriendService>` — forwards to the
    // cached `doGetClassIndex`. Same shape as 0x44ca74.
    stub_44edcc()
}

// 0x44edcc — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FriendService>(void)")]
pub fn stub_44edcc() -> usize {
    // IDA 0x44edcc: `doGetClassIndex<FriendService>` — guard-once static
    // `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if FRIEND_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        FRIEND_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    FRIEND_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x44eea4 — __ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v")]
#[doc(alias = "RBX::GeometryService * RBX::ServiceProvider::find<RBX::GeometryService>(void)const")]
pub fn stub_44eea4(instance: *const Instance) -> *const Instance {
    // IDA 0x44eea4: `find<GeometryService>` — same root-walk + pre-order
    // shape as 0x44c6f0, matching the `GeometryService` class.
    // SAFETY: same contract as 0x44c6f0.
    unsafe {
        let mut root = instance;
        while !root.is_null() && !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if root.is_null() {
            return core::ptr::null();
        }
        let mut stack: Vec<*const Instance> = vec![root];
        while let Some(node) = stack.pop() {
            if instance_is_a(node, "GeometryService") {
                return node;
            }
            let mut children: Vec<*const Instance> = (*node)
                .children
                .iter()
                .map(|child| SharedPtr::as_ptr(child) as *const Instance)
                .collect();
            children.reverse();
            stack.extend(children);
        }
        core::ptr::null()
    }
}

// 0x44f148 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GeometryServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GeometryServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GeometryService>(void)")]
pub fn stub_44f148() -> usize {
    // IDA 0x44f148: `callDoGetClassIndex<GeometryService>` — forwards to
    // the cached `doGetClassIndex`. Same shape as 0x44ca74.
    stub_44f14c()
}

// 0x44f14c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GeometryService>(void)")]
pub fn stub_44f14c() -> usize {
    // IDA 0x44f14c: `doGetClassIndex<GeometryService>` — guard-once static
    // `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if GEOMETRY_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        GEOMETRY_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    GEOMETRY_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x44fc58 — __ZNK3RBX15ServiceProvider4findINS_12BadgeServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_12BadgeServiceEEEPT_v")]
#[doc(alias = "RBX::BadgeService * RBX::ServiceProvider::find<RBX::BadgeService>(void)const")]
pub fn stub_44fc58(instance: *const Instance) -> *const Instance {
    // IDA 0x44fc58: `find<BadgeService>` — same root-walk + pre-order
    // shape as 0x44c6f0, matching the `BadgeService` class.
    // SAFETY: same contract as 0x44c6f0.
    unsafe {
        let mut root = instance;
        while !root.is_null() && !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if root.is_null() {
            return core::ptr::null();
        }
        let mut stack: Vec<*const Instance> = vec![root];
        while let Some(node) = stack.pop() {
            if instance_is_a(node, "BadgeService") {
                return node;
            }
            let mut children: Vec<*const Instance> = (*node)
                .children
                .iter()
                .map(|child| SharedPtr::as_ptr(child) as *const Instance)
                .collect();
            children.reverse();
            stack.extend(children);
        }
        core::ptr::null()
    }
}

// 0x44fe74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12BadgeServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12BadgeServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::BadgeService>(void)")]
pub fn stub_44fe74() -> usize {
    // IDA 0x44fe74: `callDoGetClassIndex<BadgeService>` — forwards to the
    // cached `doGetClassIndex`. Same shape as 0x44ca74.
    stub_44fe78()
}

// 0x44fe78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::BadgeService>(void)")]
pub fn stub_44fe78() -> usize {
    // IDA 0x44fe78: `doGetClassIndex<BadgeService>` — guard-once static
    // `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if BADGE_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        BADGE_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    BADGE_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x44ffb8 — __ZNK3RBX15ServiceProvider4findINS_14PhysicsServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_14PhysicsServiceEEEPT_v")]
#[doc(alias = "RBX::PhysicsService * RBX::ServiceProvider::find<RBX::PhysicsService>(void)const")]
pub fn stub_44ffb8(instance: *const Instance) -> *const Instance {
    // IDA 0x44ffb8: `find<PhysicsService>` — same root-walk + pre-order
    // shape as 0x44c6f0, matching the `PhysicsService` class.
    // SAFETY: same contract as 0x44c6f0.
    unsafe {
        let mut root = instance;
        while !root.is_null() && !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if root.is_null() {
            return core::ptr::null();
        }
        let mut stack: Vec<*const Instance> = vec![root];
        while let Some(node) = stack.pop() {
            if instance_is_a(node, "PhysicsService") {
                return node;
            }
            let mut children: Vec<*const Instance> = (*node)
                .children
                .iter()
                .map(|child| SharedPtr::as_ptr(child) as *const Instance)
                .collect();
            children.reverse();
            stack.extend(children);
        }
        core::ptr::null()
    }
}

// 0x450340 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14PhysicsServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_14PhysicsServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::PhysicsService>(void)")]
pub fn stub_450340() -> usize {
    // IDA 0x450340: `doGetClassIndex<PhysicsService>` — guard-once static
    // `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if PHYSICS_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        PHYSICS_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    PHYSICS_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x450dc0 — __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_450dc0() -> &'static ClassDescriptor {
    // IDA 0x450dc0: `Described<PhysicsService,...>::classDescriptor()` —
    // same once-shape as 0x4419a8 with ("PhysicsService", base).
    LazyLock::force(&PHYSICS_SERVICE_CLASS)
}

// 0x450ee0 — __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_450ee0(_service: &mut PhysicsService) {
    // IDA 0x450ee0: `Described<PhysicsService,...>::D1` tail-calls
    // `Instance::~Instance`. Same drop-glue shape as 0x449914.
}

// 0x450ee4 — __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_450ee4(_service: &mut PhysicsService) {
    // IDA 0x450ee4: `Described<PhysicsService,...>::D0` — D2 + delete.
    // Same shape as 0x449918.
}

// 0x450f84 — __ZThn32_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_450f84(_service: &mut PhysicsService) {
    // IDA 0x450f84: `ZThn32` D1 — `this -= 32`, then the D1 above. Same
    // collapse as 0x4499b8.
}

// 0x450f8c — __ZThn32_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_450f8c(_service: &mut PhysicsService) {
    // IDA 0x450f8c: `ZThn32` D0 — `this -= 32`, D0 + delete. Same collapse
    // as 0x4499c0.
}

// 0x451030 — __ZThn36_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_451030(_service: &mut PhysicsService) {
    // IDA 0x451030: `ZThn36` D1 — `this -= 36`, then the D1 above. Same
    // collapse as 0x449a64.
}

// 0x451038 — __ZThn36_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_451038(_service: &mut PhysicsService) {
    // IDA 0x451038: `ZThn36` D0 — `this -= 36`, D0 + delete. Same collapse
    // as 0x449a6c.
}

// 0x45147c — __ZNK3RBX15ServiceProvider4findINS_17CollectionServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17CollectionServiceEEEPT_v")]
#[doc(alias = "RBX::CollectionService * RBX::ServiceProvider::find<RBX::CollectionService>(void)const")]
pub fn stub_45147c(instance: *const Instance) -> *const Instance {
    // IDA 0x45147c: `find<CollectionService>` — same root-walk + pre-order
    // shape as 0x44c6f0, matching the `CollectionService` class.
    // SAFETY: same contract as 0x44c6f0.
    unsafe {
        let mut root = instance;
        while !root.is_null() && !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if root.is_null() {
            return core::ptr::null();
        }
        let mut stack: Vec<*const Instance> = vec![root];
        while let Some(node) = stack.pop() {
            if instance_is_a(node, "CollectionService") {
                return node;
            }
            let mut children: Vec<*const Instance> = (*node)
                .children
                .iter()
                .map(|child| SharedPtr::as_ptr(child) as *const Instance)
                .collect();
            children.reverse();
            stack.extend(children);
        }
        core::ptr::null()
    }
}

// 0x451800 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17CollectionServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17CollectionServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::CollectionService>(void)")]
pub fn stub_451800() -> usize {
    // IDA 0x451800: `callDoGetClassIndex<CollectionService>` — forwards to
    // the cached `doGetClassIndex`. Same shape as 0x44ca74.
    stub_451804()
}

// 0x451804 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17CollectionServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17CollectionServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::CollectionService>(void)")]
pub fn stub_451804() -> usize {
    // IDA 0x451804: `doGetClassIndex<CollectionService>` — guard-once
    // static `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if COLLECTION_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        COLLECTION_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    COLLECTION_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x453038 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10RunServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10RunServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::RunService>(void)")]
pub fn stub_453038() -> usize {
    // IDA 0x453038: `callDoGetClassIndex<RunService>` — forwards to the
    // cached `doGetClassIndex` (0x3af08). Same shape as 0x44ca74.
    stub_3af08()
}

// 0x453200 — __ZNK3RBX15ServiceProvider4findINS_17StarterGuiServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17StarterGuiServiceEEEPT_v")]
#[doc(alias = "RBX::StarterGuiService * RBX::ServiceProvider::find<RBX::StarterGuiService>(void)const")]
pub fn stub_453200(instance: *const Instance) -> *const Instance {
    // IDA 0x453200: `find<StarterGuiService>` — same root-walk + pre-order
    // shape as 0x44c6f0, matching the `StarterGuiService` class.
    // SAFETY: same contract as 0x44c6f0.
    unsafe {
        let mut root = instance;
        while !root.is_null() && !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if root.is_null() {
            return core::ptr::null();
        }
        let mut stack: Vec<*const Instance> = vec![root];
        while let Some(node) = stack.pop() {
            if instance_is_a(node, "StarterGuiService") {
                return node;
            }
            let mut children: Vec<*const Instance> = (*node)
                .children
                .iter()
                .map(|child| SharedPtr::as_ptr(child) as *const Instance)
                .collect();
            children.reverse();
            stack.extend(children);
        }
        core::ptr::null()
    }
}

// 0x453584 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17StarterGuiServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17StarterGuiServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::StarterGuiService>(void)")]
pub fn stub_453584() -> usize {
    // IDA 0x453584: `callDoGetClassIndex<StarterGuiService>` — forwards to
    // the cached `doGetClassIndex`. Same shape as 0x44ca74.
    stub_453588()
}

// 0x453588 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17StarterGuiServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17StarterGuiServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::StarterGuiService>(void)")]
pub fn stub_453588() -> usize {
    // IDA 0x453588: `doGetClassIndex<StarterGuiService>` — guard-once
    // static `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if STARTER_GUI_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        STARTER_GUI_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    STARTER_GUI_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x4539e0 — __ZNK3RBX15ServiceProvider4findINS_18StarterPackServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_18StarterPackServiceEEEPT_v")]
#[doc(alias = "RBX::StarterPackService * RBX::ServiceProvider::find<RBX::StarterPackService>(void)const")]
pub fn stub_4539e0(instance: *const Instance) -> *const Instance {
    // IDA 0x4539e0: `find<StarterPackService>` — same root-walk + pre-order
    // shape as 0x44c6f0, matching the `StarterPackService` class.
    // SAFETY: same contract as 0x44c6f0.
    unsafe {
        let mut root = instance;
        while !root.is_null() && !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if root.is_null() {
            return core::ptr::null();
        }
        let mut stack: Vec<*const Instance> = vec![root];
        while let Some(node) = stack.pop() {
            if instance_is_a(node, "StarterPackService") {
                return node;
            }
            let mut children: Vec<*const Instance> = (*node)
                .children
                .iter()
                .map(|child| SharedPtr::as_ptr(child) as *const Instance)
                .collect();
            children.reverse();
            stack.extend(children);
        }
        core::ptr::null()
    }
}

// 0x453c88 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18StarterPackServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_18StarterPackServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::StarterPackService>(void)")]
pub fn stub_453c88() -> usize {
    // IDA 0x453c88: `doGetClassIndex<StarterPackService>` — guard-once
    // static `index = ServiceProvider::newIndex(1)`, then the cached index.
    // Same shape as 0x3af08 above.
    if STARTER_PACK_SERVICE_INDEX.load(Ordering::Relaxed) == 0 {
        let fresh = crate::instance::alloc_class_index();
        STARTER_PACK_SERVICE_INDEX.store(fresh, Ordering::Relaxed);
    }
    STARTER_PACK_SERVICE_INDEX.load(Ordering::Relaxed)
}

// 0x4542c0 — __ZNK3RBX15ServiceProvider4findINS_13LocalBackpackEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_13LocalBackpackEEEPT_v")]
#[doc(alias = "RBX::LocalBackpack * RBX::ServiceProvider::find<RBX::LocalBackpack>(void)const")]
pub fn stub_4542c0(instance: *const Instance) -> *const Instance {
    // IDA 0x4542c0: `find<LocalBackpack>` — same root-walk + pre-order
    // shape as 0x44c6f0, matching the `LocalBackpack` class.
    // SAFETY: same contract as 0x44c6f0.
    unsafe {
        let mut root = instance;
        while !root.is_null() && !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if root.is_null() {
            return core::ptr::null();
        }
        let mut stack: Vec<*const Instance> = vec![root];
        while let Some(node) = stack.pop() {
            if instance_is_a(node, "LocalBackpack") {
                return node;
            }
            let mut children: Vec<*const Instance> = (*node)
                .children
                .iter()
                .map(|child| SharedPtr::as_ptr(child) as *const Instance)
                .collect();
            children.reverse();
            stack.extend(children);
        }
        core::ptr::null()
    }
}
