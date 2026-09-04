// Auto-generated shard DH — next 100 RBX::Reflection stubs — EA-sorted ascending 0xfd0c..0x11f30 (remaining 7695) — starts 0xfd0c
// Source: ida/export.json filtered demangled contains RBX::Reflection (16171 total, 8376->8476 covered, 7695 remaining)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr (was boost::shared_ptr)
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;
use crate::descriptor::Variant;
use std::sync::LazyLock;

// ---- CRenderSettingsItem / ResolutionPreset models (IDA 0xfd0c..0x1055c) ----
// `EnumPropDescriptor<CRenderSettingsItem, ResolutionPreset>` keeps the enum
// singleton link at +40/+48 and the member GetSetImpl at +44 (IDA 0xfe84), and
// every accessor below dispatches through that member (vf+8 get / vf+12 set),
// exactly like the `ExplosionEnumPropDesc` precedent in `descriptor.rs`. The
// member-function pointers themselves have no Rust form and are elided (cf.
// `generated_refl_35.rs` batch-2 note); the access closures close over the
// described field instead.

/// `CRenderSettingsItem` described state covered here (IDA 0xfe30: the
/// `DescribedBase` adjusts by -36 to this).
#[derive(Debug, Clone, Default)]
pub struct RenderSettingsItemState {
    pub resolution_preset: i32,
    pub bool_prop: bool,
    pub int_prop: i32,
    pub antialiasing_mode: i32,
    pub shadow_mode: i32,
    pub aa_samples: i32,
    pub bound_string: String,
}

/// Get/set pair behind the member at +44 (cf. `ExplosionTypeAccess`).
pub struct ResolutionPresetAccess {
    pub get: Box<dyn Fn(&RenderSettingsItemState) -> i32 + Send + Sync>,
    pub set: Box<dyn Fn(&mut RenderSettingsItemState, i32) + Send + Sync>,
}

/// `RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem, ResolutionPreset>`
/// (IDA 0xfe84).
pub struct ResolutionPresetPropDesc {
    pub name: String,
    pub category: String,
    pub access: ResolutionPresetAccess,
    /// Singleton link stored at +40/+48 (IDA 0xfe84 `doGetSingleton`).
    pub enum_desc: &'static crate::enum_desc::EnumDesc,
    pub attributes: u32,
    pub permissions: u32,
}

/// `Singleton<EnumDesc<ResolutionPreset>>` backing store. The C2 at 0x9100
/// installs the name `"Resolution"` with empty tables; pairs are registered by
/// the `addPair` stubs, so the model starts empty too.
static RESOLUTION_PRESET_DESC: LazyLock<crate::enum_desc::EnumDesc> =
    LazyLock::new(|| crate::enum_desc::EnumDesc::new("Resolution"));

pub fn resolution_preset_enum_desc() -> &'static crate::enum_desc::EnumDesc {
    &RESOLUTION_PRESET_DESC
}

/// Canonical member access closing over `resolution_preset`.
pub fn resolution_preset_access() -> ResolutionPresetAccess {
    ResolutionPresetAccess {
        get: Box::new(|obj: &RenderSettingsItemState| obj.resolution_preset),
        set: Box::new(|obj: &mut RenderSettingsItemState, value: i32| {
            obj.resolution_preset = value;
        }),
    }
}

/// Get/set pair behind `PropDescriptor<CRenderSettingsItem, bool>` (IDA 0x1070c
/// `new(0x14)` member desc holding the getter/setter pair).
pub struct RenderSettingsItemBoolAccess {
    pub get: Box<dyn Fn(&RenderSettingsItemState) -> bool + Send + Sync>,
    pub set: Box<dyn Fn(&mut RenderSettingsItemState, bool) + Send + Sync>,
}
/// `RBX::Reflection::PropDescriptor<CRenderSettingsItem, bool>` (IDA 0x1070c).
pub struct RenderSettingsItemBoolPropDesc {
    pub name: String,
    pub category: String,
    pub access: RenderSettingsItemBoolAccess,
    pub attributes: u32,
    pub permissions: u32,
}
/// Canonical member access closing over `bool_prop`.
pub fn render_settings_item_bool_access() -> RenderSettingsItemBoolAccess {
    RenderSettingsItemBoolAccess {
        get: Box::new(|obj: &RenderSettingsItemState| obj.bool_prop),
        set: Box::new(|obj: &mut RenderSettingsItemState, value: bool| {
            obj.bool_prop = value;
        }),
    }
}
/// Get/set pair behind `PropDescriptor<CRenderSettingsItem, int>` (IDA 0x1089c
/// `new(0x14)` member desc holding the getter/setter pair).
pub struct RenderSettingsItemIntAccess {
    pub get: Box<dyn Fn(&RenderSettingsItemState) -> i32 + Send + Sync>,
    pub set: Box<dyn Fn(&mut RenderSettingsItemState, i32) + Send + Sync>,
}
/// `RBX::Reflection::PropDescriptor<CRenderSettingsItem, int>` (IDA 0x1089c).
pub struct RenderSettingsItemIntPropDesc {
    pub name: String,
    pub category: String,
    pub access: RenderSettingsItemIntAccess,
    pub attributes: u32,
    pub permissions: u32,
}
/// Canonical member access closing over `int_prop`.
pub fn render_settings_item_int_access() -> RenderSettingsItemIntAccess {
    RenderSettingsItemIntAccess {
        get: Box::new(|obj: &RenderSettingsItemState| obj.int_prop),
        set: Box::new(|obj: &mut RenderSettingsItemState, value: i32| {
            obj.int_prop = value;
        }),
    }
}
/// Get/set pair behind the AA member at +44 (IDA 0x10a08; cf. `ResolutionPresetAccess`).
pub struct AntialiasingModeAccess {
    pub get: Box<dyn Fn(&RenderSettingsItemState) -> i32 + Send + Sync>,
    pub set: Box<dyn Fn(&mut RenderSettingsItemState, i32) + Send + Sync>,
}
/// `RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem, AntialiasingMode>`
/// (IDA 0x10a08).
pub struct AntialiasingModePropDesc {
    pub name: String,
    pub category: String,
    pub access: AntialiasingModeAccess,
    /// Singleton link stored at +40/+48 (IDA 0x10a08 `doGetSingleton`).
    pub enum_desc: &'static crate::enum_desc::EnumDesc,
    pub attributes: u32,
    pub permissions: u32,
}
/// `Singleton<EnumDesc<AntialiasingMode>>` backing store. The name
/// `"Antialiasing"` matches `enum_desc_crender_settings_antialiasing_mode_ctor`
/// (IDA 0x88c4); pairs are registered by the `addPair` stubs, so the model
/// starts empty too.
static ANTIALIASING_MODE_DESC: LazyLock<crate::enum_desc::EnumDesc> =
    LazyLock::new(|| crate::enum_desc::EnumDesc::new("Antialiasing"));
pub fn antialiasing_mode_enum_desc() -> &'static crate::enum_desc::EnumDesc {
    &ANTIALIASING_MODE_DESC
}
/// Canonical member access closing over `antialiasing_mode`.
pub fn antialiasing_mode_access() -> AntialiasingModeAccess {
    AntialiasingModeAccess {
        get: Box::new(|obj: &RenderSettingsItemState| obj.antialiasing_mode),
        set: Box::new(|obj: &mut RenderSettingsItemState, value: i32| {
            obj.antialiasing_mode = value;
        }),
    }
}
/// Get/set pair behind the ShadowMode member at +44 (IDA 0x11290; cf. `ResolutionPresetAccess`).
pub struct ShadowModeAccess {
    pub get: Box<dyn Fn(&RenderSettingsItemState) -> i32 + Send + Sync>,
    pub set: Box<dyn Fn(&mut RenderSettingsItemState, i32) + Send + Sync>,
}
/// `RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem, ShadowMode>`
/// (IDA 0x11290).
pub struct ShadowModePropDesc {
    pub name: String,
    pub category: String,
    pub access: ShadowModeAccess,
    /// Singleton link stored at +40/+48 (IDA 0x11290 `doGetSingleton`).
    pub enum_desc: &'static crate::enum_desc::EnumDesc,
    pub attributes: u32,
    pub permissions: u32,
}
/// `Singleton<EnumDesc<ShadowMode>>` backing store. The name `"Shadow"`
/// matches `enum_desc_crender_settings_shadow_mode_ctor` (IDA 0x8c4c); pairs
/// are registered by the `addPair` stubs, so the model starts empty too.
static SHADOW_MODE_DESC: LazyLock<crate::enum_desc::EnumDesc> =
    LazyLock::new(|| crate::enum_desc::EnumDesc::new("Shadow"));
pub fn shadow_mode_enum_desc() -> &'static crate::enum_desc::EnumDesc {
    &SHADOW_MODE_DESC
}
/// Canonical member access closing over `shadow_mode`.
pub fn shadow_mode_access() -> ShadowModeAccess {
    ShadowModeAccess {
        get: Box::new(|obj: &RenderSettingsItemState| obj.shadow_mode),
        set: Box::new(|obj: &mut RenderSettingsItemState, value: i32| {
            obj.shadow_mode = value;
        }),
    }
}
/// Get/set pair behind the AASamples member at +44 (IDA 0x11d30; cf. `ResolutionPresetAccess`).
pub struct AaSamplesAccess {
    pub get: Box<dyn Fn(&RenderSettingsItemState) -> i32 + Send + Sync>,
    pub set: Box<dyn Fn(&mut RenderSettingsItemState, i32) + Send + Sync>,
}
/// `RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem, AASamples>`
/// (IDA 0x11d30).
pub struct AaSamplesPropDesc {
    pub name: String,
    pub category: String,
    pub access: AaSamplesAccess,
    /// Singleton link stored at +40/+48 (IDA 0x11d30 `doGetSingleton`).
    pub enum_desc: &'static crate::enum_desc::EnumDesc,
    pub attributes: u32,
    pub permissions: u32,
}
/// `Singleton<EnumDesc<AASamples>>` backing store. The name `"AASamples"`
/// matches `enum_desc_crender_settings_aa_samples_ctor` (IDA 0x850c); pairs
/// are registered by the `addPair` stubs, so the model starts empty too.
static AA_SAMPLES_DESC: LazyLock<crate::enum_desc::EnumDesc> =
    LazyLock::new(|| crate::enum_desc::EnumDesc::new("AASamples"));
pub fn aa_samples_enum_desc() -> &'static crate::enum_desc::EnumDesc {
    &AA_SAMPLES_DESC
}
/// Canonical member access closing over `aa_samples`.
pub fn aa_samples_access() -> AaSamplesAccess {
    AaSamplesAccess {
        get: Box::new(|obj: &RenderSettingsItemState| obj.aa_samples),
        set: Box::new(|obj: &mut RenderSettingsItemState, value: i32| {
            obj.aa_samples = value;
        }),
    }
}
/// `RBX::Reflection::BoundProp<std::string, Mutable>` for `CRenderSettingsItem`
/// (IDA 0x11b18): base `TypedPropertyDescriptor<string>` init plus a
/// `new(0x14)` member desc at +40 holding (owner, offset); the trailing
/// `isReadOnly`/`isWriteOnly` masks never fire since both virtuals hardcode 0
/// (IDA 0x11ca8/0x11cac). Same layout as `BoundPropDesc` in shard_ac.
#[derive(Debug, Clone)]
pub struct RenderSettingsItemBoundStringDesc {
    pub name: String,
    pub category: String,
    pub member_offset: usize,
    pub attributes: u32,
    pub permissions: u32,
}

/// `RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int (),0>` (IDA 0xfd0c):
/// member-function slot stored at +40, `Type::getSingleton<int>` return at +28.
#[derive(Debug, Clone)]
pub struct BoundFunc0Desc {
    pub name: String,
    pub category: String,
    pub member: usize,
    pub return_type: &'static str,
    pub permissions: u32,
    pub attributes: u32,
}

/// `XmlElement` value payload as consumed at IDA 0x102cc: xsi:nil leaves the
/// object untouched; otherwise the int form is tried first, then the string
/// form. Anything else hit `ReleaseAssert(false)` (`Reflection.h:359`) and has
/// no representable state here.
#[derive(Debug, Clone)]
pub enum XmlReadValue {
    Nil,
    Int(i32),
    Text(String),
}

// 0xfd0c — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::BoundFuncDesc(int (CRenderSettingsItem::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_fd0c(
    member: usize,
    name: &str,
    category: &str,
    permissions: u32,
    attributes: u32,
) -> BoundFunc0Desc {
    // IDA 0xfd0c: `classDescriptor` + `FunctionDescriptor` base init (name/category),
    // vtable install, member-function pair stored at +40 (decompiled 0xfd0c `v8` at
    // `v14 + 40`), `Type::getSingleton<int>` return stored at +28. The member
    // pointer itself has no Rust form and is kept as an opaque slot.
    BoundFunc0Desc {
        name: name.to_owned(),
        category: category.to_owned(),
        member,
        return_type: "int",
        permissions,
        attributes,
    }
}

// 0xfe04 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED0Ev")]
pub fn stub_fe04() {
    // IDA 0xfe04: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xfe30 — __ZNK3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_fe30(
    desc: &BoundFunc0Desc,
    obj: Option<&RenderSettingsItemState>,
    invoke: &dyn Fn(&RenderSettingsItemState) -> i32,
) -> Variant {
    // IDA 0xfe30: `instance = a2 ? a2 - 36 : 0`, then `Call0Helper::call(instance,
    // member@+40, member@+44, args + 4)` (decompiled 0xfe30). A null instance
    // trapped in the callee; the port panics instead. The member dispatch is
    // elided (no Rust form for member-function pointers); `invoke` is the bound
    // `int (CRenderSettingsItem::*)(void)`.
    let _ = desc.member;
    let obj = obj.expect("BoundFuncDesc::execute on null instance (IDA 0xfe30)");
    stub_fe54(obj, invoke)
}

// 0xfe54 — __ZN3RBX10Reflection11Call0HelperI19CRenderSettingsItemMS2_FivEiE4callEPS2_S4_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<CRenderSettingsItem,int (CRenderSettingsItem::*)(void),int>::call(CRenderSettingsItem*,int (CRenderSettingsItem::*)(void),RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperI19CRenderSettingsItemMS2_FivEiE4callEPS2_S4_RNS0_7VariantE")]
pub fn stub_fe54(obj: &RenderSettingsItemState, invoke: &dyn Fn(&RenderSettingsItemState) -> i32) -> Variant {
    // IDA 0xfe54: this-adjust (`a1 + (a3 >> 1)`) with virtual dispatch when the
    // member tag is odd, then `v = call(this)`; out gets `Type::getSingleton<int>`
    // and `placement_any<int> = v` (decompiled 0xfe54). Adjustment/dispatch are
    // member-pointer mechanics with no Rust form; the call and the int `Variant`
    // wrap are the observable behavior.
    Variant::Int(invoke(obj))
}

// 0xfe84 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::EnumPropDescriptor<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>(char const*,char const*,RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_fe84(name: &str, category: &str, attributes: u32, permissions: u32) -> ResolutionPresetPropDesc {
    // IDA 0xfe84: `classDescriptor` + `Singleton<EnumDesc<ResolutionPreset>>`
    // fetch, `PropertyDescriptor` base init, enum link stored at +40/+48, vtable
    // install, member GetSetImpl `new(0x14)` holding the getter/setter pair at +44,
    // then the read/write-only flag fixups (decompiled 0xfe84). Same shape as the
    // `ExplosionEnumPropDesc` ctor at 0x4a5834. The getter/setter member pointers
    // (a4..a7) have no Rust form; the canonical field access is installed.
    ResolutionPresetPropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: resolution_preset_access(),
        enum_desc: resolution_preset_enum_desc(),
        attributes,
        permissions,
    }
}

// 0x10038 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED0Ev")]
pub fn stub_10038() {
    // IDA 0x10038: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x10064 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10isReadOnlyEv")]
pub fn stub_10064() {
    // IDA 0x10064: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x10074 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11isWriteOnlyEv")]
pub fn stub_10074() {
    // IDA 0x10074: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x10084 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_10084(desc: &ResolutionPresetPropDesc, a: &RenderSettingsItemState, b: &RenderSettingsItemState) -> bool {
    // IDA 0x10084: `v = member(+44)->get(a)` then `return v == member->get(b)`
    // (both through vf+8, decompiled 0x10084). Same as 0x4a5a34.
    (desc.access.get)(a) == (desc.access.get)(b)
}

// 0x100ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_100ac(desc: &ResolutionPresetPropDesc, obj: &RenderSettingsItemState) -> Variant {
    // IDA 0x100ac: `v = getEnumValue(obj)` (vf+68), out = `Variant(int, v)` with
    // `Type<int>` + `placement_any<int>=` (decompiled 0x100ac). Same as 0x4a5a5c.
    Variant::Int((desc.access.get)(obj))
}

// 0x100d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_100d0(desc: &ResolutionPresetPropDesc, obj: &mut RenderSettingsItemState, value: &Variant) {
    // IDA 0x100d0: int-typed payloads use `any_cast<int>` directly; anything else
    // goes through `Variant::convert<int>`; then `setEnumValue(obj, v)` (vf+72,
    // decompiled 0x100d0). Same as 0x4a5a80.
    let v = match value {
        Variant::Int(v) => *v,
        other => other.convert_to_int(),
    };
    (desc.access.set)(obj, v);
}

// 0x10220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_10220(desc: &ResolutionPresetPropDesc, src: &RenderSettingsItemState, dst: &mut RenderSettingsItemState) {
    // IDA 0x10220: `v = member(+44)->get(src)` (vf+8), then `member->set(dst, v)`
    // (vf+12, decompiled 0x10220). Same as 0x4a5bcc.
    let v = (desc.access.get)(src);
    (desc.access.set)(dst, v);
}

// 0x10244 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14hasStringValueEv")]
pub fn stub_10244() -> bool {
    // IDA 0x10244: EnumPropDescriptor::hasStringValue -- hardcoded `return 1` (decompiled 0x10244/0x10dc8/0x11650).
    true
}

// 0x10248 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_10248(desc: &ResolutionPresetPropDesc, obj: &RenderSettingsItemState) -> String {
    // IDA 0x10248: `v = member(+44)->get(obj)`, then
    // `EnumDesc<ResolutionPreset>::convertToString(enumdesc@+48, v)` (decompiled
    // 0x10248). Same as 0x4a5bf8.
    let v = (desc.access.get)(obj);
    desc.enum_desc.lookup_name(v).unwrap_or_default().to_owned()
}

// 0x1026c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_1026c(desc: &ResolutionPresetPropDesc, obj: &mut RenderSettingsItemState, name: &str) -> bool {
    // IDA 0x1026c: `Name::lookup(&name, str)`, `convertToValue(enumdesc@+48, name,
    // &out)`; on 1, `member(+44)->set(obj, out)` and return 1, else 0 (decompiled
    // 0x1026c). `&str` folds the lookup step; `lookup_value` covers
    // `convertToValue` including legacy names. Same as 0x4a5c1c.
    match desc.enum_desc.lookup_value(name) {
        Some(v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x102ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_102ac(desc: &ResolutionPresetPropDesc, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x102ac: `v = member(+44)->get(obj)` (vf+8), `clearValue(pair)` then store
    // int tag 5 + value, return 5 (decompiled 0x102ac). The tag is the Xml int type
    // code; the payload is the enum int, which is what the model returns. Same as
    // 0x4a5c5c.
    (desc.access.get)(obj)
}

// 0x102cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_102cc(desc: &ResolutionPresetPropDesc, obj: &mut RenderSettingsItemState, value: &XmlReadValue) -> bool {
    // IDA 0x102cc: xsi:nil returns untouched; an int-valued element runs
    // `setIntValue` and returns on success; a string-valued element runs
    // `Name::lookup` + `convertToValue` then `member(+44)->set(obj, v)`; any other
    // shape falls into `ReleaseAssert(false)` (`Reflection.h:359`, decompiled
    // 0x102cc). A failed int mapping falls through to the string check and then
    // the assert, so it panics too.
    match value {
        XmlReadValue::Nil => false,
        XmlReadValue::Int(v) => {
            match usize::try_from(*v)
                .ok()
                .and_then(|slot| desc.enum_desc.value_to_value.get(slot).copied())
            {
                Some(mapped) if mapped != -1 => {
                    (desc.access.set)(obj, mapped);
                    true
                }
                _ => panic!("false file: ../App/include/Reflection/Reflection.h line: 359 (IDA 0x102cc)"),
            }
        }
        XmlReadValue::Text(text) => match desc.enum_desc.lookup_value(text) {
            Some(v) => {
                (desc.access.set)(obj, v);
                true
            }
            None => panic!("false file: ../App/include/Reflection/Reflection.h line: 359 (IDA 0x102cc)"),
        },
    }
}

// 0x1050c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_1050c(desc: &ResolutionPresetPropDesc, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x1050c: `v = member(+44)->get(obj)` (vf+8), return
    // `convertToIndex(enumdesc@+48, v)` (decompiled 0x1050c). Same as 0x4a5ebc.
    stub_10604(desc.enum_desc, (desc.access.get)(obj))
}

// 0x10528 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_10528(desc: &ResolutionPresetPropDesc, obj: &mut RenderSettingsItemState, index: usize) -> bool {
    // IDA 0x10528: `if (*(enumdesc+40) > index)` load `values[index]`,
    // `member(+44)->set(obj, v)`, return 1; else return 0 (decompiled 0x10528).
    // Same as 0x4a5ed8.
    match desc.enum_desc.values.get(index) {
        Some(&v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x1055c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_1055c(desc: &ResolutionPresetPropDesc, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x1055c: tail-jump to `member(+44)->get(obj)` (vf+8, disasm 0x1055c);
    // the whole body is the forward. Same as 0x4a5f0c.
    (desc.access.get)(obj)
}

// 0x10564 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_10564(desc: &ResolutionPresetPropDesc, obj: &mut RenderSettingsItemState, value: i32) -> bool {
    // IDA 0x10564: `find_if(items, bind(equalValue, _1, value))` (disasm 0x10588); miss returns 0
    // (0x10594), hit runs `member(+44)->set(obj, value)` (0x105a2) and returns 1. Same as 0x4a5f14.
    if desc.enum_desc.items.iter().any(|it| it.value == value) {
        (desc.access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x105b0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_105b0(desc: &ResolutionPresetPropDesc, obj: &RenderSettingsItemState) -> Option<crate::enum_desc::EnumItem> {
    // IDA 0x105b0: `v = member(+44)->get(obj)` (disasm 0x105c2), return
    // `convertToItem(enumdesc@+48, &v)` (0x105ce): the `Item*` for the value, or null. Same as 0x4a5f60.
    let v = (desc.access.get)(obj);
    usize::try_from(v)
        .ok()
        .and_then(|slot| desc.enum_desc.items_by_value.get(slot).copied().flatten())
        .and_then(|idx| desc.enum_desc.items.get(idx).cloned())
}

// 0x105d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_105d0(desc: &ResolutionPresetPropDesc, obj: &mut RenderSettingsItemState, name: &str) -> bool {
    // IDA 0x105d0 (`Name` overload): `convertToValue(enumdesc@+48, name, &out)` (0x105e6);
    // success runs `member(+44)->set(obj, out)` (0x105fc) and returns 1, else 0. Same as 0x4a5f80.
    match desc.enum_desc.lookup_value(name) {
        Some(v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x10604 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToIndex(RBX::CRenderSettings::ResolutionPreset)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToIndexES3_")]
pub fn stub_10604(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x10604: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x10674 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_10674(desc: &ResolutionPresetPropDesc, obj: &mut RenderSettingsItemState, value: i32) -> bool {
    // IDA 0x10674: `if (value >= 0)` (disasm 0x1067e) and `value < value_to_value.size` (0x10690)
    // load `mapped = value_to_value[value]` (0x10692); `mapped == -1` returns 0 (0x1069c),
    // else `member(+44)->set(obj, mapped)` (0x106a8) and return 1. Same as 0x4a6028.
    match usize::try_from(value)
        .ok()
        .and_then(|slot| desc.enum_desc.value_to_value.get(slot).copied())
    {
        Some(mapped) if mapped != -1 => {
            (desc.access.set)(obj, mapped);
            true
        }
        _ => false,
    }
}

// 0x106b4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_106b4() -> bool {
    // IDA 0x106b4: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x106b8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_106b8() -> bool {
    // IDA 0x106b8: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x106bc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_106bc(access: &ResolutionPresetAccess, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x106bc: null→`obj-36` member adjust (disasm 0x106be-0x106cc), split the member pointer
    // (offset at +8, encoding at +4), virtual-adjust if the low bit is set (0x106e0-0x106e4),
    // call the getter. The adjust/encoding is member-pointer mechanics with no Rust equivalent;
    // the observable effect is the get. Same as 0x4a6074.
    (access.get)(obj)
}

// 0x106e8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ResolutionPreset const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_106e8(access: &ResolutionPresetAccess, obj: &mut RenderSettingsItemState, value: i32) {
    // IDA 0x106e8: same member-pointer dispatch as stub_106bc through the setter at +12/+16
    // (0x106f4-0x10704); the observable effect is the set. Same as 0x4a6094.
    (access.set)(obj, value);
}

// 0x1070c — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_1070c(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&RenderSettingsItemState) -> bool + Send + Sync>,
    set: Box<dyn Fn(&mut RenderSettingsItemState, bool) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> RenderSettingsItemBoolPropDesc {
    // IDA 0x1070c: `Described<CRenderSettingsItem>::classDescriptor()` init (0x10734),
    // `new(0x14)` member desc holding the (getter, setter) member-pointer pair
    // (0x10762-0x10770), base `TypedPropertyDescriptor<bool>` init (0x107b2), temp
    // release (0x107ba-0x107bc), vtable install (0x107d0). Same shape as 0xfb74.
    RenderSettingsItemBoolPropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: RenderSettingsItemBoolAccess { get, set },
        attributes,
        permissions,
    }
}

// 0x10820 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED0Ev")]
pub fn stub_10820() {
    // IDA 0x10820: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x1084c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_1084c() -> bool {
    // IDA 0x1084c: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x10850 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_10850() -> bool {
    // IDA 0x10850: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x10854 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_10854(access: &RenderSettingsItemBoolAccess, obj: &RenderSettingsItemState) -> bool {
    // IDA 0x10854: `a2 ? a2-36 : 0` base adjust (0x1085a-0x1085c), split the member-pointer
    // pair at +4/+8, virtual-adjust when the low bit is set (0x1086e-0x10872), call the
    // getter (0x10876). Same dispatch shape as stub_106bc; the observable effect is the get.
    (access.get)(obj)
}

// 0x10878 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_10878(access: &RenderSettingsItemBoolAccess, obj: &mut RenderSettingsItemState, value: bool) {
    // IDA 0x10878: same member-pointer dispatch as stub_10854 through the setter at +12/+16
    // (0x10884-0x10894); the observable effect is the set.
    (access.set)(obj, value);
}

// 0x1089c — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>(char const*,char const*,int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_1089c(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&RenderSettingsItemState) -> i32 + Send + Sync>,
    set: Box<dyn Fn(&mut RenderSettingsItemState, i32) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> RenderSettingsItemIntPropDesc {
    // IDA 0x1089c: `Described<CRenderSettingsItem>::classDescriptor()` init (0x108c4),
    // `new(0x14)` member desc holding the (getter, setter) member-pointer pair
    // (0x108f2-0x10900), base `TypedPropertyDescriptor<int>` init (0x10942), temp
    // release (0x1094a-0x1094c), vtable install (0x10960). Same shape as 0xfb74.
    RenderSettingsItemIntPropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: RenderSettingsItemIntAccess { get, set },
        attributes,
        permissions,
    }
}

// 0x109b0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE10isReadOnlyEv")]
pub fn stub_109b0() -> bool {
    // IDA 0x109b0: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x109b4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE11isWriteOnlyEv")]
pub fn stub_109b4() -> bool {
    // IDA 0x109b4: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x109b8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_109b8(access: &RenderSettingsItemIntAccess, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x109b8: null→`obj-36` member adjust (0x109ba-0x109c8), split the member pointer
    // (offset at +8, encoding at +4), virtual-adjust if the low bit is set (0x109dc-0x109e0),
    // call the getter. Same shape as stub_106bc; the observable effect is the get.
    (access.get)(obj)
}

// 0x109e4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_109e4(access: &RenderSettingsItemIntAccess, obj: &mut RenderSettingsItemState, value: i32) {
    // IDA 0x109e4: same member-pointer dispatch as stub_109b8 through the setter at +12/+16
    // (0x109f0-0x10a00); the observable effect is the set.
    (access.set)(obj, value);
}

// 0x10a08 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::EnumPropDescriptor<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>(char const*,char const*,RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_10a08(name: &str, category: &str, attributes: u32, permissions: u32) -> AntialiasingModePropDesc {
    // IDA 0x10a08: `classDescriptor` + `Singleton<EnumDesc<AntialiasingMode>>`
    // fetch (0x10a4c-0x10a50), `PropertyDescriptor` base init, enum link stored at +40/+48
    // (0x10abe-0x10b28), vtable install, member GetSetImpl `new(0x14)` holding the
    // getter/setter pair at +44 (0x10ae6-0x10b0c), then the read/write-only flag fixups
    // (0x10b38-0x10b5e). Same shape as stub_fe84. The getter/setter member pointers
    // (a4..a7) have no Rust form; the canonical field access is installed.
    AntialiasingModePropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: antialiasing_mode_access(),
        enum_desc: antialiasing_mode_enum_desc(),
        attributes,
        permissions,
    }
}

// 0x10bbc — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED0Ev")]
pub fn stub_10bbc() {
    // IDA 0x10bbc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x10be8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10isReadOnlyEv")]
pub fn stub_10be8() {
    // IDA 0x10be8: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x10bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11isWriteOnlyEv")]
pub fn stub_10bf8() {
    // IDA 0x10bf8: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x10c08 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_10c08(desc: &AntialiasingModePropDesc, a: &RenderSettingsItemState, b: &RenderSettingsItemState) -> bool {
    // IDA 0x10c08: `v = member(+44)->get(a)` then `return v == member->get(b)`
    // (both through vf+8, decompiled 0x10c08). Same as stub_10084.
    (desc.access.get)(a) == (desc.access.get)(b)
}

// 0x10c30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_10c30(desc: &AntialiasingModePropDesc, obj: &RenderSettingsItemState) -> Variant {
    // IDA 0x10c30: `v = getEnumValue(obj)` (vf+68), out = `Variant(int, v)` with
    // `Type<int>` + `placement_any<int>=` (decompiled 0x10c30). Same as stub_100ac.
    Variant::Int((desc.access.get)(obj))
}

// 0x10c54 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_10c54(desc: &AntialiasingModePropDesc, obj: &mut RenderSettingsItemState, value: &Variant) {
    // IDA 0x10c54: int-typed payloads use `any_cast<int>` directly; anything else
    // goes through `Variant::convert<int>`; then `setEnumValue(obj, v)` (vf+72,
    // decompiled 0x10c54). Same as stub_100d0.
    let v = match value {
        Variant::Int(v) => *v,
        other => other.convert_to_int(),
    };
    (desc.access.set)(obj, v);
}

// 0x10da4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_10da4(desc: &AntialiasingModePropDesc, src: &RenderSettingsItemState, dst: &mut RenderSettingsItemState) {
    // IDA 0x10da4: `v = member(+44)->get(src)` (vf+8), then `member->set(dst, v)`
    // (vf+12, decompiled 0x10da4). Same as stub_10220.
    let v = (desc.access.get)(src);
    (desc.access.set)(dst, v);
}

// 0x10dc8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14hasStringValueEv")]
pub fn stub_10dc8() -> bool {
    // IDA 0x10dc8: EnumPropDescriptor::hasStringValue -- hardcoded `return 1` (decompiled 0x10244/0x10dc8/0x11650).
    true
}

// 0x10dcc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_10dcc(desc: &AntialiasingModePropDesc, obj: &RenderSettingsItemState) -> String {
    // IDA 0x10dcc: `v = member(+44)->get(obj)`, then
    // `EnumDesc<AntialiasingMode>::convertToString(enumdesc@+48, v)` (decompiled
    // 0x10dcc). Same as stub_10248.
    let v = (desc.access.get)(obj);
    desc.enum_desc.lookup_name(v).unwrap_or_default().to_owned()
}

// 0x10df0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_10df0(desc: &AntialiasingModePropDesc, obj: &mut RenderSettingsItemState, name: &str) -> bool {
    // IDA 0x10df0: `Name::lookup(&name, str)`, `convertToValue(enumdesc@+48, name,
    // &out)`; on 1, `member(+44)->set(obj, out)` and return 1, else 0 (decompiled
    // 0x10df0). `&str` folds the lookup step; `lookup_value` covers
    // `convertToValue` including legacy names. Same as stub_1026c.
    match desc.enum_desc.lookup_value(name) {
        Some(v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x10e30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_10e30(desc: &AntialiasingModePropDesc, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x10e30: `v = member(+44)->get(obj)` (vf+8), `clearValue(pair)` then store
    // int tag 5 + value, return 5 (decompiled 0x10e30). The tag is the Xml int type
    // code; the payload is the enum int, which is what the model returns. Same as
    // stub_102ac.
    (desc.access.get)(obj)
}

// 0x10e50 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_10e50(desc: &AntialiasingModePropDesc, obj: &mut RenderSettingsItemState, value: &XmlReadValue) -> bool {
    // IDA 0x10e50: xsi:nil returns untouched; an int-valued element runs
    // `setIntValue` and returns on success; a string-valued element runs
    // `Name::lookup` + `convertToValue` then `member(+44)->set(obj, v)`; any other
    // shape falls into `ReleaseAssert(false)` (`Reflection.h:359`, decompiled
    // 0x10e50). A failed int mapping falls through to the string check and then
    // the assert, so it panics too. Same as stub_102cc.
    match value {
        XmlReadValue::Nil => false,
        XmlReadValue::Int(v) => {
            match usize::try_from(*v)
                .ok()
                .and_then(|slot| desc.enum_desc.value_to_value.get(slot).copied())
            {
                Some(mapped) if mapped != -1 => {
                    (desc.access.set)(obj, mapped);
                    true
                }
                _ => panic!("false file: ../App/include/Reflection/Reflection.h line: 359 (IDA 0x10e50)"),
            }
        }
        XmlReadValue::Text(text) => match desc.enum_desc.lookup_value(text) {
            Some(v) => {
                (desc.access.set)(obj, v);
                true
            }
            None => panic!("false file: ../App/include/Reflection/Reflection.h line: 359 (IDA 0x10e50)"),
        },
    }
}

// 0x11090 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_11090(desc: &AntialiasingModePropDesc, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x11090: `v = member(+44)->get(obj)` (vf+8), return
    // `convertToIndex(enumdesc@+48, v)` (decompiled 0x11090). Same as stub_1050c.
    stub_11188(desc.enum_desc, (desc.access.get)(obj))
}

// 0x110ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_110ac(desc: &AntialiasingModePropDesc, obj: &mut RenderSettingsItemState, index: usize) -> bool {
    // IDA 0x110ac: `if (*(enumdesc+40) > index)` load `values[index]`,
    // `member(+44)->set(obj, v)`, return 1; else return 0 (decompiled 0x110ac).
    // Same as stub_10528.
    match desc.enum_desc.values.get(index) {
        Some(&v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x110e0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_110e0(desc: &AntialiasingModePropDesc, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x110e0: tail-jump to `member(+44)->get(obj)` (vf+8, disasm 0x110e0);
    // the whole body is the forward. Same as stub_1055c.
    (desc.access.get)(obj)
}

// 0x110e8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_110e8(desc: &AntialiasingModePropDesc, obj: &mut RenderSettingsItemState, value: i32) -> bool {
    // IDA 0x110e8: `find_if(items, bind(equalValue, _1, value))` (0x11112); miss returns 0
    // (0x11118), hit runs `member(+44)->set(obj, value)` (0x11126) and returns 1. Same as stub_10564.
    if desc.enum_desc.items.iter().any(|it| it.value == value) {
        (desc.access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x11134 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_11134(desc: &AntialiasingModePropDesc, obj: &RenderSettingsItemState) -> Option<crate::enum_desc::EnumItem> {
    // IDA 0x11134: `v = member(+44)->get(obj)` (0x11146), return
    // `convertToItem(enumdesc@+48, &v)` (0x11152): the `Item*` for the value, or null.
    // Same as stub_105b0.
    let v = (desc.access.get)(obj);
    usize::try_from(v)
        .ok()
        .and_then(|slot| desc.enum_desc.items_by_value.get(slot).copied().flatten())
        .and_then(|idx| desc.enum_desc.items.get(idx).cloned())
}

// 0x11154 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_11154(desc: &AntialiasingModePropDesc, obj: &mut RenderSettingsItemState, name: &str) -> bool {
    // IDA 0x11154 (`Name` overload): `convertToValue(enumdesc@+48, name, &out)` (0x1116a);
    // success runs `member(+44)->set(obj, out)` (0x11180) and returns 1, else 0. Same as stub_105d0.
    match desc.enum_desc.lookup_value(name) {
        Some(v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x11188 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToIndex(RBX::CRenderSettings::AntialiasingMode)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToIndexES3_")]
pub fn stub_11188(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x11188: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x111f8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_111f8(desc: &AntialiasingModePropDesc, obj: &mut RenderSettingsItemState, value: i32) -> bool {
    // IDA 0x111f8: `if (value >= 0)` (0x11202) and `value < value_to_value.size` (0x11214)
    // load `mapped = value_to_value[value]` (0x11216); `mapped == -1` returns 0 (0x11220),
    // else `member(+44)->set(obj, mapped)` (0x1122c) and return 1. Same as stub_10674.
    match usize::try_from(value)
        .ok()
        .and_then(|slot| desc.enum_desc.value_to_value.get(slot).copied())
    {
        Some(mapped) if mapped != -1 => {
            (desc.access.set)(obj, mapped);
            true
        }
        _ => false,
    }
}

// 0x11238 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_11238() -> bool {
    // IDA 0x11238: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x1123c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_1123c() -> bool {
    // IDA 0x1123c: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x11240 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_11240(access: &AntialiasingModeAccess, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x11240: null→`obj-36` member adjust (0x11242-0x11250), split the member pointer
    // (offset at +8, encoding at +4), virtual-adjust if the low bit is set (0x11264-0x11268),
    // call the getter. The adjust/encoding is member-pointer mechanics with no Rust equivalent;
    // the observable effect is the get. Same as stub_106bc.
    (access.get)(obj)
}

// 0x1126c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AntialiasingMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_1126c(access: &AntialiasingModeAccess, obj: &mut RenderSettingsItemState, value: i32) {
    // IDA 0x1126c: same member-pointer dispatch as stub_11240 through the setter at +12/+16
    // (0x11278-0x11288); the observable effect is the set. Same as stub_106e8.
    (access.set)(obj, value);
}

// 0x11290 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::EnumPropDescriptor<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>(char const*,char const*,RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_11290(name: &str, category: &str, attributes: u32, permissions: u32) -> ShadowModePropDesc {
    // IDA 0x11290: `classDescriptor` + `Singleton<EnumDesc<ShadowMode>>`
    // fetch (0x112d4-0x112d8), `PropertyDescriptor` base init, enum link stored at +40/+48
    // (0x11346-0x113b0), vtable install, member GetSetImpl `new(0x14)` holding the
    // getter/setter pair at +44 (0x1136e-0x11394), then the read/write-only flag fixups
    // (0x113c0-0x113e6). Same shape as stub_10a08. The getter/setter member pointers
    // (a4..a7) have no Rust form; the canonical field access is installed.
    ShadowModePropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: shadow_mode_access(),
        enum_desc: shadow_mode_enum_desc(),
        attributes,
        permissions,
    }
}

// 0x11444 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED0Ev")]
pub fn stub_11444() {
    // IDA 0x11444: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x11470 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10isReadOnlyEv")]
pub fn stub_11470() {
    // IDA 0x11470: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x11480 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11isWriteOnlyEv")]
pub fn stub_11480() {
    // IDA 0x11480: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x11490 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_11490(desc: &ShadowModePropDesc, a: &RenderSettingsItemState, b: &RenderSettingsItemState) -> bool {
    // IDA 0x11490: `v = member(+44)->get(a)` then `return v == member->get(b)`
    // (both through vf+8, 0x114a0-0x114b6). Same as stub_10c08.
    (desc.access.get)(a) == (desc.access.get)(b)
}

// 0x114b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_114b8(desc: &ShadowModePropDesc, obj: &RenderSettingsItemState) -> Variant {
    // IDA 0x114b8: `v = getEnumValue(obj)` (vf+68), out = `Variant(int, v)` with
    // `Type<int>` + `placement_any<int>=` (0x114c6-0x114da). Same as stub_10c30.
    Variant::Int((desc.access.get)(obj))
}

// 0x114dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_114dc(desc: &ShadowModePropDesc, obj: &mut RenderSettingsItemState, value: &Variant) {
    // IDA 0x114dc: int-typed payloads use `any_cast<int>` directly; anything else
    // goes through `Variant::convert<int>`; then `setEnumValue(obj, v)` (vf+72,
    // 0x11528-0x115e6). Same as stub_10c54.
    let v = match value {
        Variant::Int(v) => *v,
        other => other.convert_to_int(),
    };
    (desc.access.set)(obj, v);
}

// 0x1162c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_1162c(desc: &ShadowModePropDesc, src: &RenderSettingsItemState, dst: &mut RenderSettingsItemState) {
    // IDA 0x1162c: `v = member(+44)->get(src)` (vf+8), then `member->set(dst, v)`
    // (vf+12, 0x1163e-0x1164e). Same as stub_10da4.
    let v = (desc.access.get)(src);
    (desc.access.set)(dst, v);
}

// 0x11650 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14hasStringValueEv")]
pub fn stub_11650() -> bool {
    // IDA 0x11650: EnumPropDescriptor::hasStringValue -- hardcoded `return 1` (decompiled 0x10244/0x10dc8/0x11650).
    true
}

pub fn stub_11654(desc: &ShadowModePropDesc, obj: &RenderSettingsItemState) -> String {
    // IDA 0x11654: `v = member(+44)->get(obj)` (vf+8, 0x11666), then
    // `EnumDesc<ShadowMode>::convertToString(enumdesc@+48, v)` (0x11676).
    // Same as stub_10dcc.
    let v = (desc.access.get)(obj);
    desc.enum_desc.lookup_name(v).unwrap_or_default().to_owned()
}

pub fn stub_11678(desc: &ShadowModePropDesc, obj: &mut RenderSettingsItemState, name: &str) -> bool {
    // IDA 0x11678: `Name::lookup(&name, str)` (0x1168a), `convertToValue(enumdesc@+48,
    // name, &out)` (0x11698); on 1, `member(+44)->set(obj, out)` (0x116ae) and return 1,
    // else 0 (0x1169a-0x116b4). `&str` folds the lookup step; `lookup_value` covers
    // `convertToValue` including legacy names. Same as stub_10df0.
    match desc.enum_desc.lookup_value(name) {
        Some(v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

pub fn stub_116b8(desc: &ShadowModePropDesc, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x116b8: `v = member(+44)->get(obj)` (vf+8), `clearValue(pair)` then store
    // int tag 5 + value, return 5 (0x116c6-0x116d6). The tag is the Xml int type
    // code; the payload is the enum int, which is what the model returns. Same as
    // stub_10e30.
    (desc.access.get)(obj)
}

pub fn stub_116d8(desc: &ShadowModePropDesc, obj: &mut RenderSettingsItemState, value: &XmlReadValue) -> bool {
    // IDA 0x116d8: xsi:nil returns untouched (0x116fc); an int-valued element runs
    // `setIntValue` and returns on success (0x11744-0x11754); a string-valued element
    // runs `Name::lookup` + `convertToValue` then `member(+44)->set(obj, v)`
    // (0x11762-0x117be); any other shape falls into `ReleaseAssert(false)`
    // (`Reflection.h:359`, 0x11808-0x118b6). A failed int mapping falls through to
    // the string check and then the assert, so it panics too. Same as stub_10e50.
    match value {
        XmlReadValue::Nil => false,
        XmlReadValue::Int(v) => {
            match usize::try_from(*v)
                .ok()
                .and_then(|slot| desc.enum_desc.value_to_value.get(slot).copied())
            {
                Some(mapped) if mapped != -1 => {
                    (desc.access.set)(obj, mapped);
                    true
                }
                _ => panic!("false file: ../App/include/Reflection/Reflection.h line: 359 (IDA 0x116d8)"),
            }
        }
        XmlReadValue::Text(text) => match desc.enum_desc.lookup_value(text) {
            Some(v) => {
                (desc.access.set)(obj, v);
                true
            }
            None => panic!("false file: ../App/include/Reflection/Reflection.h line: 359 (IDA 0x116d8)"),
        },
    }
}

pub fn stub_11918(desc: &ShadowModePropDesc, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x11918: `v = member(+44)->get(obj)` (vf+8, 0x11928), return
    // `convertToIndex(enumdesc@+48, v)` (0x11928). Same as stub_11090.
    stub_11a10(desc.enum_desc, (desc.access.get)(obj))
}

pub fn stub_11934(desc: &ShadowModePropDesc, obj: &mut RenderSettingsItemState, index: usize) -> bool {
    // IDA 0x11934: `if (*(enumdesc+40) > index)` (0x11946) load `values[index]`
    // (0x11950), `member(+44)->set(obj, v)` (0x1195a), return 1; else return 0
    // (0x11964). Same as stub_110ac.
    match desc.enum_desc.values.get(index) {
        Some(&v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

pub fn stub_11968(desc: &ShadowModePropDesc, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x11968: tail-jump to `member(+44)->get(obj)` (vf+8, 0x11968);
    // the whole body is the forward. Same as stub_110e0.
    (desc.access.get)(obj)
}

// 0x11970 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_11970(desc: &ShadowModePropDesc, obj: &mut RenderSettingsItemState, value: i32) -> bool {
    // IDA 0x11970: `find_if(items, bind(equalValue, _1, value))` (0x1199a); miss returns 0
    // (0x1199c), hit runs `member(+44)->set(obj, value)` (0x119ae) and returns 1. Same as stub_110e8.
    if desc.enum_desc.items.iter().any(|it| it.value == value) {
        (desc.access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x119bc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_119bc(desc: &ShadowModePropDesc, obj: &RenderSettingsItemState) -> Option<crate::enum_desc::EnumItem> {
    // IDA 0x119bc: `v = member(+44)->get(obj)` (0x119ce), return
    // `convertToItem(enumdesc@+48, &v)` (0x119da): the `Item*` for the value, or null.
    // Same as stub_11134.
    let v = (desc.access.get)(obj);
    usize::try_from(v)
        .ok()
        .and_then(|slot| desc.enum_desc.items_by_value.get(slot).copied().flatten())
        .and_then(|idx| desc.enum_desc.items.get(idx).cloned())
}

// 0x119dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_119dc(desc: &ShadowModePropDesc, obj: &mut RenderSettingsItemState, name: &str) -> bool {
    // IDA 0x119dc (`Name` overload): `convertToValue(enumdesc@+48, name, &out)` (0x119f2);
    // success runs `member(+44)->set(obj, out)` (0x11a08) and returns 1, else 0. Same as stub_11154.
    match desc.enum_desc.lookup_value(name) {
        Some(v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x11a10 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToIndex(RBX::CRenderSettings::ShadowMode)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_")]
pub fn stub_11a10(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x11a10: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x11a80 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_11a80(desc: &ShadowModePropDesc, obj: &mut RenderSettingsItemState, value: i32) -> bool {
    // IDA 0x11a80: `if (value >= 0)` (0x11a8a) and `value < value_to_value.size` (0x11a9c)
    // load `mapped = value_to_value[value]` (0x11a9e); `mapped == -1` returns 0 (0x11aa8),
    // else `member(+44)->set(obj, mapped)` (0x11ab4) and return 1. Same as stub_111f8.
    match usize::try_from(value)
        .ok()
        .and_then(|slot| desc.enum_desc.value_to_value.get(slot).copied())
    {
        Some(mapped) if mapped != -1 => {
            (desc.access.set)(obj, mapped);
            true
        }
        _ => false,
    }
}

// 0x11ac0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_11ac0() -> bool {
    // IDA 0x11ac0: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x11ac4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_11ac4() -> bool {
    // IDA 0x11ac4: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x11ac8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_11ac8(access: &ShadowModeAccess, obj: &RenderSettingsItemState) -> i32 {
    // IDA 0x11ac8: null→`obj-36` member adjust (0x11aca-0x11ad8), split the member pointer
    // (offset at +8, encoding at +4), virtual-adjust if the low bit is set (0x11aec-0x11af0),
    // call the getter. The adjust/encoding is member-pointer mechanics with no Rust equivalent;
    // the observable effect is the get. Same as stub_106bc.
    (access.get)(obj)
}

// 0x11af4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ShadowMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_11af4(access: &ShadowModeAccess, obj: &mut RenderSettingsItemState, value: i32) {
    // IDA 0x11af4: same member-pointer dispatch as stub_11ac8 through the setter at +12/+16
    // (0x11b00-0x11b10); the observable effect is the set. Same as stub_106e8.
    (access.set)(obj, value);
}

// 0x11b18 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,std::string  CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_11b18(
    name: &str,
    category: &str,
    member_offset: usize,
    attributes: u32,
    permissions: u32,
) -> RenderSettingsItemBoundStringDesc {
    // IDA 0x11b18: `classDescriptor` init (0x11b3e), base `TypedPropertyDescriptor<string>`
    // init (0x11ba0), temp release (0x11ba8-0x11baa), vtable install (0x11bbe), `new(0x14)`
    // member desc holding (owner, offset) installed at +40 over the deleted stale member
    // (0x11bcc-0x11c02), then `if (isReadOnly() == 1) attrs &= ~0x14` (0x11c12-0x11c1c) and
    // `if (isWriteOnly() == 1) attrs &= ~0x0C` (0x11c2e-0x11c38). Both virtuals hardcode 0
    // (stub_11ca8/stub_11cac), so the masks never fire. Same as stub_0x81142c.
    let mut attributes = attributes;
    if stub_11ca8() {
        attributes &= !0x14;
    }
    if stub_11cac() {
        attributes &= !0x0C;
    }
    RenderSettingsItemBoundStringDesc { name: name.to_owned(), category: category.to_owned(), member_offset, attributes, permissions }
}

// 0x11ca8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv")]
pub fn stub_11ca8() -> bool {
    // IDA 0x11ca8: BoundPropGetSet::isReadOnly -- hardcoded `return 0` (decompiled 0x659d38/0x659d3c SurfaceGetSet, 0x6ba528/0x6ba52c BoundPropGetSet).
    false
}

// 0x11cac — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv")]
pub fn stub_11cac() -> bool {
    // IDA 0x11cac: BoundPropGetSet::isWriteOnly -- hardcoded `return 0` (decompiled 0x659d38/0x659d3c SurfaceGetSet, 0x6ba528/0x6ba52c BoundPropGetSet).
    false
}

// 0x11cb0 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_11cb0(obj: &RenderSettingsItemState) -> String {
    // IDA 0x11cb0: member-offset adjust (`a3 ? a3 - 36 : 0`, 0x11cb6-0x11cb8), member
    // pointer load (0x11cc6), `string::string` copy-construct of the bound member.
    // The adjust is member-pointer mechanics over an unmodeled instance; the copy is
    // the observable effect. Same as stub_0x8115c4.
    obj.bound_string.clone()
}

// 0x11cc8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_11cc8(obj: &mut RenderSettingsItemState, value: &str) -> bool {
    // IDA 0x11cc8: `string::compare` (0x11ce6); equal returns 0 (0x11d1a). Else
    // `string::assign` (0x11cf0), validator dispatch through the +12/+16 pair when
    // present (0x11cf4-0x11d12), `raisePropertyChanged` (0x11d2a). Validator and
    // change-notification live outside this crate; the stored update is the effect.
    // Same as stub_0x8115dc.
    if obj.bound_string == value {
        return false;
    }
    obj.bound_string = value.to_owned();
    true
}

// 0x11d30 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::EnumPropDescriptor<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>(char const*,char const*,RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_11d30(name: &str, category: &str, attributes: u32, permissions: u32) -> AaSamplesPropDesc {
    // IDA 0x11d30: `classDescriptor` + `Singleton<EnumDesc<AASamples>>`
    // fetch (0x11d74-0x11d78), `PropertyDescriptor` base init, enum link stored at +40/+48
    // (0x11de6-0x11e50), vtable install, member GetSetImpl `new(0x14)` holding the
    // getter/setter pair at +44 (0x11e0e-0x11e34), then the read/write-only flag fixups
    // (0x11e60-0x11e86). Same shape as stub_10a08. The getter/setter member pointers
    // (a4..a7) have no Rust form; the canonical field access is installed.
    AaSamplesPropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: aa_samples_access(),
        enum_desc: aa_samples_enum_desc(),
        attributes,
        permissions,
    }
}

// 0x11ee4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev")]
pub fn stub_11ee4() {
    // IDA 0x11ee4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x11f10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv")]
pub fn stub_11f10() {
    // IDA 0x11f10: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x11f20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv")]
pub fn stub_11f20() {
    // IDA 0x11f20: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x11f30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_11f30(desc: &AaSamplesPropDesc, a: &RenderSettingsItemState, b: &RenderSettingsItemState) -> bool {
    // IDA 0x11f30: `v = member(+44)->get(a)` then `return v == member->get(b)`
    // (both through vf+8, 0x11f40-0x11f56). Same as stub_10c08.
    (desc.access.get)(a) == (desc.access.get)(b)
}

#[cfg(test)]
mod resolution_preset_tests {
    use super::*;
    use crate::enum_desc::EnumDesc;

    static TEST_DESC: LazyLock<EnumDesc> = LazyLock::new(|| {
        let mut d = EnumDesc::new("Resolution");
        d.add_pair(0, "Low");
        d.add_pair(1, "High");
        d
    });

    fn test_desc() -> ResolutionPresetPropDesc {
        ResolutionPresetPropDesc {
            name: "Resolution".to_owned(),
            category: "Rendering".to_owned(),
            access: resolution_preset_access(),
            enum_desc: &TEST_DESC,
            attributes: 0,
            permissions: 0,
        }
    }

    #[test]
    fn ctor_links_enum_singleton() {
        let desc = stub_fe84("Resolution", "Rendering", 0, 0);
        assert_eq!(desc.name, "Resolution");
        assert!(std::ptr::eq(desc.enum_desc, resolution_preset_enum_desc()));
        assert_eq!(desc.enum_desc.enum_name, "Resolution");
        let func = stub_fd0c(0x40, "Replay", "Rendering", 0, 0);
        assert_eq!(func.return_type, "int");
    }

    #[test]
    fn execute_calls_member_and_wraps_int() {
        let func = stub_fd0c(0x40, "Replay", "Rendering", 0, 0);
        let obj = RenderSettingsItemState { resolution_preset: 1, ..Default::default() };
        let out = stub_fe30(&func, Some(&obj), &|o| o.resolution_preset + 10);
        match out {
            Variant::Int(v) => assert_eq!(v, 11),
            _ => panic!("expected int variant"),
        }
        assert!(matches!(stub_fe54(&obj, &|o| o.resolution_preset), Variant::Int(1)));
    }

    #[test]
    #[should_panic]
    fn execute_null_instance_panics() {
        let func = stub_fd0c(0x40, "Replay", "Rendering", 0, 0);
        let _ = stub_fe30(&func, None, &|o| o.resolution_preset);
    }

    #[test]
    fn value_round_trip() {
        let desc = test_desc();
        let a = RenderSettingsItemState { resolution_preset: 1, ..Default::default() };
        let b = RenderSettingsItemState { resolution_preset: 0, ..Default::default() };
        assert!(stub_10084(&desc, &a, &a));
        assert!(!stub_10084(&desc, &a, &b));
        assert!(matches!(stub_100ac(&desc, &a), Variant::Int(1)));
        let mut dst = RenderSettingsItemState::default();
        stub_100d0(&desc, &mut dst, &Variant::Int(1));
        assert_eq!(dst.resolution_preset, 1);
        stub_100d0(&desc, &mut dst, &Variant::Float(0.0));
        assert_eq!(dst.resolution_preset, 0);
        let mut cp = RenderSettingsItemState::default();
        stub_10220(&desc, &a, &mut cp);
        assert_eq!(cp.resolution_preset, 1);
        assert_eq!(stub_1055c(&desc, &a), 1);
        assert_eq!(stub_102ac(&desc, &a), 1);
    }

    #[test]
    fn string_index_xml_paths() {
        let desc = test_desc();
        let a = RenderSettingsItemState { resolution_preset: 1, ..Default::default() };
        assert_eq!(stub_10248(&desc, &a), "High");
        let mut obj = RenderSettingsItemState::default();
        assert!(stub_1026c(&desc, &mut obj, "High"));
        assert_eq!(obj.resolution_preset, 1);
        assert!(!stub_1026c(&desc, &mut obj, "Ultra"));
        assert_eq!(stub_1050c(&desc, &a), 1);
        assert!(stub_10528(&desc, &mut obj, 0));
        assert_eq!(obj.resolution_preset, 0);
        assert!(!stub_10528(&desc, &mut obj, 9));
        assert!(stub_102cc(&desc, &mut obj, &XmlReadValue::Text("High".to_owned())));
        assert_eq!(obj.resolution_preset, 1);
        assert!(stub_102cc(&desc, &mut obj, &XmlReadValue::Int(0)));
        assert_eq!(obj.resolution_preset, 0);
        assert!(!stub_102cc(&desc, &mut obj, &XmlReadValue::Nil));
    }

    #[test]
    #[should_panic]
    fn read_unknown_name_panics() {
        let desc = test_desc();
        let mut obj = RenderSettingsItemState::default();
        let _ = stub_102cc(&desc, &mut obj, &XmlReadValue::Text("Ultra".to_owned()));
    }

    #[test]
    #[should_panic]
    fn read_unmapped_int_panics() {
        let desc = test_desc();
        let mut obj = RenderSettingsItemState::default();
        let _ = stub_102cc(&desc, &mut obj, &XmlReadValue::Int(7));
    }
}
