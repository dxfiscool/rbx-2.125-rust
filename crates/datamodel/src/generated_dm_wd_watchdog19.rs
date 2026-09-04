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
pub fn stub_439cf0() -> ! { todo!("0x439cf0 __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE13convertToItemERKS3_") }

// 0x439dbc — __ZN3rbx8any_castIRKN3RBX9DataModel8GearTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX9DataModel8GearTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::DataModel::GearType const& rbx::any_cast<RBX::DataModel::GearType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_439dbc() -> ! { todo!("0x439dbc __ZN3rbx8any_castIRKN3RBX9DataModel8GearTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

// 0x439eac — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueERKNS_4NameERS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToValue(RBX::Name const&,RBX::DataModel::GearType&)const")]
pub fn stub_439eac() -> ! { todo!("0x439eac __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueERKNS_4NameERS3_") }

// 0x439f28 — __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED2Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::~EnumDesc()")]
pub fn stub_439f28() -> ! { todo!("0x439f28 __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED2Ev") }

// 0x43a0fc — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringERKS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToString(RBX::DataModel::GearGenreSetting const&)const")]
pub fn stub_43a0fc() -> ! { todo!("0x43a0fc __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringERKS3_") }

// 0x43a29c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel16GearGenreSettingEEERS3_RKT_
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel16GearGenreSettingEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::GearGenreSetting>(RBX::DataModel::GearGenreSetting const&)")]
pub fn stub_43a29c() -> ! { todo!("0x43a29c __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel16GearGenreSettingEEERS3_RKT_") }

// 0x43a2ec — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE9singletonEv
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::singleton(void)")]
pub fn stub_43a2ec() -> ! { todo!("0x43a2ec __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE9singletonEv") }

// 0x43a358 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE14construct_funcEPKcPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE14construct_funcEPKcPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::construct_func(char const*,char *)")]
pub fn stub_43a358() -> ! { todo!("0x43a358 __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE14construct_funcEPKcPc") }

// 0x43a364 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::destruct_func(char *)")]
pub fn stub_43a364() -> ! { todo!("0x43a364 __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE13destruct_funcEPc") }

// 0x43a368 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE13convertToItemERKS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE13convertToItemERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToItem(RBX::DataModel::GearGenreSetting const&)const")]
pub fn stub_43a368() -> ! { todo!("0x43a368 __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE13convertToItemERKS3_") }

// 0x43a434 — __ZN3rbx8any_castIRKN3RBX9DataModel16GearGenreSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX9DataModel16GearGenreSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::DataModel::GearGenreSetting const& rbx::any_cast<RBX::DataModel::GearGenreSetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_43a434() -> ! { todo!("0x43a434 __ZN3rbx8any_castIRKN3RBX9DataModel16GearGenreSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

// 0x43a524 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueERKNS_4NameERS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToValue(RBX::Name const&,RBX::DataModel::GearGenreSetting&)const")]
pub fn stub_43a524() -> ! { todo!("0x43a524 __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueERKNS_4NameERS3_") }

// 0x43a5a0 — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED2Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::~EnumDesc()")]
pub fn stub_43a5a0() -> ! { todo!("0x43a5a0 __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED2Ev") }

// 0x43a774 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringERKS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToString(RBX::DataModel::Genre const&)const")]
pub fn stub_43a774() -> ! { todo!("0x43a774 __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringERKS3_") }

// 0x43a914 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel5GenreEEERS3_RKT_
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel5GenreEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::Genre>(RBX::DataModel::Genre const&)")]
pub fn stub_43a914() -> ! { todo!("0x43a914 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel5GenreEEERS3_RKT_") }

// 0x43a964 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE9singletonEv
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::Genre>::singleton(void)")]
pub fn stub_43a964() -> ! { todo!("0x43a964 __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE9singletonEv") }

// 0x43a9d0 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE14construct_funcEPKcPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE14construct_funcEPKcPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::Genre>::construct_func(char const*,char *)")]
pub fn stub_43a9d0() -> ! { todo!("0x43a9d0 __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE14construct_funcEPKcPc") }

// 0x43a9dc — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::Genre>::destruct_func(char *)")]
pub fn stub_43a9dc() -> ! { todo!("0x43a9dc __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE13destruct_funcEPc") }

// 0x43a9e0 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE13convertToItemERKS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE13convertToItemERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToItem(RBX::DataModel::Genre const&)const")]
pub fn stub_43a9e0() -> ! { todo!("0x43a9e0 __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE13convertToItemERKS3_") }

// 0x43aaac — __ZN3rbx8any_castIRKN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::DataModel::Genre const& rbx::any_cast<RBX::DataModel::Genre const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_43aaac() -> ! { todo!("0x43aaac __ZN3rbx8any_castIRKN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

// 0x43ab9c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueERKNS_4NameERS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToValue(RBX::Name const&,RBX::DataModel::Genre&)const")]
pub fn stub_43ab9c() -> ! { todo!("0x43ab9c __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueERKNS_4NameERS3_") }

// 0x43ac18 — __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED2Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::~EnumDesc()")]
pub fn stub_43ac18() -> ! { todo!("0x43ac18 __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED2Ev") }

// 0x43adec — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringERKS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToString(RBX::DataModel::CreatorType const&)const")]
pub fn stub_43adec() -> ! { todo!("0x43adec __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringERKS3_") }

// 0x4419a8 — __ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4419a8() -> ! { todo!("0x4419a8 __ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv") }

// 0x442248 — __ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_442248() -> ! { todo!("0x442248 __ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv") }

// 0x4426c8 — __ZN3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4426c8() -> ! { todo!("0x4426c8 __ZN3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv") }

// 0x4430a0 — __ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4430a0() -> ! { todo!("0x4430a0 __ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv") }

// 0x4497f4 — __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4497f4() -> ! { todo!("0x4497f4 __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv") }

// 0x449914 — __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_449914() -> ! { todo!("0x449914 __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev") }

// 0x449918 — __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_449918() -> ! { todo!("0x449918 __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev") }

// 0x4499b8 — __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4499b8() -> ! { todo!("0x4499b8 __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev") }

// 0x4499c0 — __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4499c0() -> ! { todo!("0x4499c0 __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev") }

// 0x449a64 — __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_449a64() -> ! { todo!("0x449a64 __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev") }

// 0x449a6c — __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_449a6c() -> ! { todo!("0x449a6c __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev") }

// 0x44c550 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_25ScriptInformationProviderEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_25ScriptInformationProviderEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptInformationProvider>(void)")]
pub fn stub_44c550() -> ! { todo!("0x44c550 __ZN3RBX15ServiceProvider15doGetClassIndexINS_25ScriptInformationProviderEEEmv") }

// 0x44c6f0 — __ZNK3RBX15ServiceProvider4findINS_13DebrisServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_13DebrisServiceEEEPT_v")]
#[doc(alias = "RBX::DebrisService * RBX::ServiceProvider::find<RBX::DebrisService>(void)const")]
pub fn stub_44c6f0() -> ! { todo!("0x44c6f0 __ZNK3RBX15ServiceProvider4findINS_13DebrisServiceEEEPT_v") }

// 0x44ca74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13DebrisServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13DebrisServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::DebrisService>(void)")]
pub fn stub_44ca74() -> ! { todo!("0x44ca74 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13DebrisServiceEEEvv") }

// 0x44ca78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13DebrisServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13DebrisServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::DebrisService>(void)")]
pub fn stub_44ca78() -> ! { todo!("0x44ca78 __ZN3RBX15ServiceProvider15doGetClassIndexINS_13DebrisServiceEEEmv") }

// 0x44cef0 — __ZNK3RBX15ServiceProvider4findINS_15GamePassServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_15GamePassServiceEEEPT_v")]
#[doc(alias = "RBX::GamePassService * RBX::ServiceProvider::find<RBX::GamePassService>(void)const")]
pub fn stub_44cef0() -> ! { todo!("0x44cef0 __ZNK3RBX15ServiceProvider4findINS_15GamePassServiceEEEPT_v") }

// 0x44d274 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GamePassServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GamePassServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GamePassService>(void)")]
pub fn stub_44d274() -> ! { todo!("0x44d274 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GamePassServiceEEEvv") }

// 0x44d278 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GamePassService>(void)")]
pub fn stub_44d278() -> ! { todo!("0x44d278 __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv") }

// 0x44d6f0 — __ZNK3RBX15ServiceProvider4findINS_13SocialServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_13SocialServiceEEEPT_v")]
#[doc(alias = "RBX::SocialService * RBX::ServiceProvider::find<RBX::SocialService>(void)const")]
pub fn stub_44d6f0() -> ! { todo!("0x44d6f0 __ZNK3RBX15ServiceProvider4findINS_13SocialServiceEEEPT_v") }

// 0x44da74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13SocialServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13SocialServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::SocialService>(void)")]
pub fn stub_44da74() -> ! { todo!("0x44da74 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13SocialServiceEEEvv") }

// 0x44da78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::SocialService>(void)")]
pub fn stub_44da78() -> ! { todo!("0x44da78 __ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv") }

// 0x44e228 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13InsertServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13InsertServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::InsertService>(void)")]
pub fn stub_44e228() -> ! { todo!("0x44e228 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13InsertServiceEEEvv") }

// 0x44e22c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::InsertService>(void)")]
pub fn stub_44e22c() -> ! { todo!("0x44e22c __ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv") }

// 0x44e518 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_18RenderHooksServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_18RenderHooksServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::RenderHooksService>(void)")]
pub fn stub_44e518() -> ! { todo!("0x44e518 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_18RenderHooksServiceEEEvv") }

// 0x44e51c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18RenderHooksServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_18RenderHooksServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RenderHooksService>(void)")]
pub fn stub_44e51c() -> ! { todo!("0x44e51c __ZN3RBX15ServiceProvider15doGetClassIndexINS_18RenderHooksServiceEEEmv") }

// 0x44edc8 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13FriendServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13FriendServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FriendService>(void)")]
pub fn stub_44edc8() -> ! { todo!("0x44edc8 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13FriendServiceEEEvv") }

// 0x44edcc — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FriendService>(void)")]
pub fn stub_44edcc() -> ! { todo!("0x44edcc __ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv") }

// 0x44eea4 — __ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v")]
#[doc(alias = "RBX::GeometryService * RBX::ServiceProvider::find<RBX::GeometryService>(void)const")]
pub fn stub_44eea4() -> ! { todo!("0x44eea4 __ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v") }

// 0x44f148 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GeometryServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GeometryServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GeometryService>(void)")]
pub fn stub_44f148() -> ! { todo!("0x44f148 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GeometryServiceEEEvv") }

// 0x44f14c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GeometryService>(void)")]
pub fn stub_44f14c() -> ! { todo!("0x44f14c __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv") }

// 0x44fc58 — __ZNK3RBX15ServiceProvider4findINS_12BadgeServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_12BadgeServiceEEEPT_v")]
#[doc(alias = "RBX::BadgeService * RBX::ServiceProvider::find<RBX::BadgeService>(void)const")]
pub fn stub_44fc58() -> ! { todo!("0x44fc58 __ZNK3RBX15ServiceProvider4findINS_12BadgeServiceEEEPT_v") }

// 0x44fe74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12BadgeServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12BadgeServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::BadgeService>(void)")]
pub fn stub_44fe74() -> ! { todo!("0x44fe74 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12BadgeServiceEEEvv") }

// 0x44fe78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::BadgeService>(void)")]
pub fn stub_44fe78() -> ! { todo!("0x44fe78 __ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv") }

// 0x44ffb8 — __ZNK3RBX15ServiceProvider4findINS_14PhysicsServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_14PhysicsServiceEEEPT_v")]
#[doc(alias = "RBX::PhysicsService * RBX::ServiceProvider::find<RBX::PhysicsService>(void)const")]
pub fn stub_44ffb8() -> ! { todo!("0x44ffb8 __ZNK3RBX15ServiceProvider4findINS_14PhysicsServiceEEEPT_v") }

// 0x450340 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14PhysicsServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_14PhysicsServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::PhysicsService>(void)")]
pub fn stub_450340() -> ! { todo!("0x450340 __ZN3RBX15ServiceProvider15doGetClassIndexINS_14PhysicsServiceEEEmv") }

// 0x450dc0 — __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_450dc0() -> ! { todo!("0x450dc0 __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv") }

// 0x450ee0 — __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_450ee0() -> ! { todo!("0x450ee0 __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev") }

// 0x450ee4 — __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_450ee4() -> ! { todo!("0x450ee4 __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev") }

// 0x450f84 — __ZThn32_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_450f84() -> ! { todo!("0x450f84 __ZThn32_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev") }

// 0x450f8c — __ZThn32_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_450f8c() -> ! { todo!("0x450f8c __ZThn32_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev") }

// 0x451030 — __ZThn36_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_451030() -> ! { todo!("0x451030 __ZThn36_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev") }

// 0x451038 — __ZThn36_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_451038() -> ! { todo!("0x451038 __ZThn36_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev") }

// 0x45147c — __ZNK3RBX15ServiceProvider4findINS_17CollectionServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17CollectionServiceEEEPT_v")]
#[doc(alias = "RBX::CollectionService * RBX::ServiceProvider::find<RBX::CollectionService>(void)const")]
pub fn stub_45147c() -> ! { todo!("0x45147c __ZNK3RBX15ServiceProvider4findINS_17CollectionServiceEEEPT_v") }

// 0x451800 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17CollectionServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17CollectionServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::CollectionService>(void)")]
pub fn stub_451800() -> ! { todo!("0x451800 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17CollectionServiceEEEvv") }

// 0x451804 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17CollectionServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17CollectionServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::CollectionService>(void)")]
pub fn stub_451804() -> ! { todo!("0x451804 __ZN3RBX15ServiceProvider15doGetClassIndexINS_17CollectionServiceEEEmv") }

// 0x453038 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10RunServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10RunServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::RunService>(void)")]
pub fn stub_453038() -> ! { todo!("0x453038 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10RunServiceEEEvv") }

// 0x453200 — __ZNK3RBX15ServiceProvider4findINS_17StarterGuiServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17StarterGuiServiceEEEPT_v")]
#[doc(alias = "RBX::StarterGuiService * RBX::ServiceProvider::find<RBX::StarterGuiService>(void)const")]
pub fn stub_453200() -> ! { todo!("0x453200 __ZNK3RBX15ServiceProvider4findINS_17StarterGuiServiceEEEPT_v") }

// 0x453584 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17StarterGuiServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17StarterGuiServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::StarterGuiService>(void)")]
pub fn stub_453584() -> ! { todo!("0x453584 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17StarterGuiServiceEEEvv") }

// 0x453588 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17StarterGuiServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17StarterGuiServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::StarterGuiService>(void)")]
pub fn stub_453588() -> ! { todo!("0x453588 __ZN3RBX15ServiceProvider15doGetClassIndexINS_17StarterGuiServiceEEEmv") }

// 0x4539e0 — __ZNK3RBX15ServiceProvider4findINS_18StarterPackServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_18StarterPackServiceEEEPT_v")]
#[doc(alias = "RBX::StarterPackService * RBX::ServiceProvider::find<RBX::StarterPackService>(void)const")]
pub fn stub_4539e0() -> ! { todo!("0x4539e0 __ZNK3RBX15ServiceProvider4findINS_18StarterPackServiceEEEPT_v") }

// 0x453c88 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18StarterPackServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_18StarterPackServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::StarterPackService>(void)")]
pub fn stub_453c88() -> ! { todo!("0x453c88 __ZN3RBX15ServiceProvider15doGetClassIndexINS_18StarterPackServiceEEEmv") }

// 0x4542c0 — __ZNK3RBX15ServiceProvider4findINS_13LocalBackpackEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_13LocalBackpackEEEPT_v")]
#[doc(alias = "RBX::LocalBackpack * RBX::ServiceProvider::find<RBX::LocalBackpack>(void)const")]
pub fn stub_4542c0() -> ! { todo!("0x4542c0 __ZNK3RBX15ServiceProvider4findINS_13LocalBackpackEEEPT_v") }
