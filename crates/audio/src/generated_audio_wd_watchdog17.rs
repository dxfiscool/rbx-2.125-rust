//! audio generated_audio_wd_watchdog17 — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x65edd8 | rbx_core::SharedPtr not boost
//! Range 0x65edd8..0x662800 | existing 36802 -> 36902 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
use std::sync::LazyLock;
use std::sync::atomic::AtomicBool;
use core::sync::atomic::{AtomicU32, Ordering};
use crate::generated::flog_asserts;
use crate::generated_134::{XmlIntSlot, XmlReadValue};
use crate::generated_audio_wd_watchdog13::SurfaceState;
use crate::generated_audio_wd_watchdog14::{
    FaceFloatProp, FaceFloatSlot, FaceInputProp, FaceTypeProp, INPUT_TYPE_ITEMS, PartSurfaceData,
    SURFACE_TYPE_ITEMS, SurfaceVariant, input_type_index, input_type_name, stub_0658e24,
    stub_0658f8c, stub_0659cc8, stub_065a544, surface_type_index, surface_type_name,
};
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };


/// `EnumDesc<NormalId>` items in `addPair` order (IDA 0x6f2970: the
/// `MOVS R1, #N` ahead of each call grounds dense values 0..=5).
pub const NORMAL_ID_ITEMS: [(&str, u32); 6] = [
    ("Right", 0),
    ("Top", 1),
    ("Back", 2),
    ("Left", 3),
    ("Bottom", 4),
    ("Front", 5),
];
/// Name of a `NormalId` value for `convertToString` (IDA 0x662800).
/// Values with no item yield "" — the writers only ever store table
/// members.
pub fn normal_id_name(value: u32) -> &'static str {
    NORMAL_ID_ITEMS
        .iter()
        .find(|(_, v)| *v == value)
        .map(|(n, _)| *n)
        .unwrap_or("")
}
/// `RBX::SurfaceSelection` cutover (IDA 0x6608b0): the `Surface`
/// face at +140 (word 35, init 0), the visibility flag at +128, the
/// adorned-part link at +132 and the `BrickColor` at +120..+128.
/// The `PartAdornment`/`Instance`/`Described` bases fold away; only
/// the +140 init is grounded by the ctor, the rest rides the base
/// init (host: cleared).
#[derive(Debug, Clone)]
pub struct SurfaceSelectionState {
    pub surface: u32,
    pub visible: bool,
    pub part_present: bool,
    pub color: [f32; 3],
}
/// `DrawAdorn::partSurface` call `render3dAdorn` emits (IDA 0x660ac0):
/// the part face, its color, full alpha (1.0 = 1065353216) and the
/// 0.2 overlay factor (1045220557 = 0.2f).
#[derive(Debug, Clone, Copy)]
pub struct SurfaceAdornDraw {
    pub surface: u32,
    pub color: [f32; 3],
    pub alpha: f32,
    pub overlay: f32,
}
/// Factory creator for `SurfaceSelection` (IDA
/// `FactoryProduct<SurfaceSelection, PartAdornment>::Creator`) —
/// stateless on the host.
pub struct SurfaceSelectionCreator;
/// Declared `RBX::Name` for `sSurfaceSelection` (IDA 0x661978:
/// `Name::declare(&sSurfaceSelection)` under a guard-once static;
/// host: `&str`).
static SURFACE_SELECTION_NAME: LazyLock<String> =
    LazyLock::new(|| "SurfaceSelection".to_owned());
/// `FactoryProduct<SurfaceSelection, PartAdornment>::creatorPrivate`
/// (IDA 0x661c9c). The image keeps one static `Creator`; `LazyLock`
/// never drops (atexit equivalent).
static SURFACE_SELECTION_CREATOR: LazyLock<SurfaceSelectionCreator> =
    LazyLock::new(|| SurfaceSelectionCreator);
/// `Creator::isConstructedE` sentinel (IDA 0x661a58/0x661c9c: 666
/// once C2 ran).
static SURFACE_SELECTION_CONSTRUCTED: AtomicBool = AtomicBool::new(false);
/// `RBX::Reflection::EnumPropDescriptor<SurfaceSelection, NormalId>`
/// cutover (IDA 0x662440): name/category/attributes/permissions.
/// The getter/setter member-pointer pair folds into direct field
/// access (same shape as `StudioToolBoolProp` at 0x6579d0).
#[derive(Debug, Clone)]
pub struct NormalIdProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
}
impl NormalIdProp {
    pub fn new(name: &str, category: &str, attributes: u32, permissions: u32) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
        }
    }
}
/// `RBX::Reflection::Type<RBX::Surface>` cutover (IDA 0x65fee4): the
/// registered tag. The `Descriptor` base, vtable, typeinfo and the
/// all-types registry fold in.
#[derive(Debug, Clone)]
pub struct SurfaceTypeTag {
    pub name: String,
}
/// Process-wide static-init run count behind the `__GLOBAL__I_a_*`
/// ctors in this file (IDA 0x65ff94). The category/ios/descriptor/
/// pool/guard stores fold into host statics (initialized on use),
/// so only the run is recorded.
static WATCHDOG17_STATIC_INITS: AtomicU32 = AtomicU32::new(0);
/// Records one `__GLOBAL__I_a_*` run in this file.
pub fn watchdog17_static_init() {
    WATCHDOG17_STATIC_INITS.fetch_add(1, Ordering::SeqCst);
}
/// Returns the recorded static-init run count (test hook).
pub fn watchdog17_static_inits() -> u32 {
    WATCHDOG17_STATIC_INITS.load(Ordering::SeqCst)
}
// 0x65edd8 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)1,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065edd8(part: &PartSurfaceData, slot: FaceFloatSlot) -> f32 {
    // IDA 0x65edd8 (`SurfaceGetSet<1, float>::getValue`): the
    // member-pointer resolve tail-calling the getter with the face
    // baked in (`, 1)`, verified in the decompile). Same shape as
    // the face-2 twin at 0x6594c4.
    part.faces[1].float_slot(slot)
}

// 0x65edf8 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)1,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf")]
pub fn stub_065edf8(part: &mut PartSurfaceData, slot: FaceFloatSlot, value: f32) {
    // IDA 0x65edf8 (`SurfaceGetSet<1, float>::setValue`): the
    // member-pointer resolve tail-calling the setter with
    // `(instance, 1, value)`. Same shape as the face-2 twin at
    // 0x6594e4.
    part.faces[1].set_float_slot(slot, value);
}

// 0x65ee1c — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
pub fn stub_065ee1c(name: &str, category: &str, functionality: u32) -> FaceInputProp {
    // IDA 0x65ee1c (`SurfaceEnumPropDescriptor<1, InputType>`
    // ctor): same `classDescriptor` + `EnumDesc` singleton +
    // base-init + impl shape as the face-2 twin at 0x659508
    // (decompile same length 2675), with the face baked in as 1.
    // Host: the cutover with face 1.
    FaceInputProp::new(1, name, category, functionality)
}

// 0x65eec8 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED0Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED0Ev")]
pub fn stub_065eec8() {
    // IDA 0x065eec8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x65eef4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10isReadOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10isReadOnlyEv")]
pub fn stub_065eef4() -> bool {
    // IDA 0x65eef4 (`SurfaceEnumPropDescriptor<1, InputType>::
    // isReadOnly`): delegates to the inner face-1 `GetSet` at +44 —
    // always readable (same shape as the face-2 twin at 0x6595e0).
    false
}

// 0x65ef04 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE11isWriteOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")]
pub fn stub_065ef04() -> bool {
    // IDA 0x65ef04 (`SurfaceEnumPropDescriptor<1, InputType>::
    // isWriteOnly`): delegates to the inner face-1 `GetSet` at +44 —
    // always writable (same shape as the face-2 twin at 0x6595f0).
    false
}

// 0x65ef14 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")]
pub fn stub_065ef14(first: &PartSurfaceData, second: &PartSurfaceData) -> bool {
    // IDA 0x65ef14 (`SurfaceEnumPropDescriptor<1, InputType>::
    // equalValues`): reads the inner value for both instances via
    // the +44 `GetSet` and compares. Host: compare the face-1
    // input slots.
    first.faces[1].surface_input == second.faces[1].surface_input
}

// 0x65ef3c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")]
pub fn stub_065ef3c(part: &PartSurfaceData) -> SurfaceVariant {
    // IDA 0x65ef3c (`SurfaceEnumPropDescriptor<1, InputType>::
    // getVariant`): reads the inner value, tags it with the
    // `InputType` singleton and placement-moves it in. Same as the
    // face-2 twin at 0x659628.
    SurfaceVariant::SurfaceInput(part.faces[1].surface_input)
}

// 0x65ef64 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")]
pub fn stub_065ef64(part: &mut PartSurfaceData, variant: &SurfaceVariant) {
    // IDA 0x65ef64 (`SurfaceEnumPropDescriptor<1, InputType>::
    // setVariant`): any-cast-or-convert then inner set, same as the
    // face-2 twin at 0x659650 (decompile same length 2201). Host:
    // the convert-or-throw into the face-1 slot.
    part.faces[1].surface_input = stub_0658f8c(variant);
}

// 0x65f0bc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")]
pub fn stub_065f0bc(first: &PartSurfaceData, second: &mut PartSurfaceData) {
    // IDA 0x65f0bc (`SurfaceEnumPropDescriptor<1, InputType>::
    // copyValue`): inner `getValue` + inner `setValue`, same as the
    // face-2 twin at 0x6597a8. Host: copy the face-1 slot.
    second.faces[1].surface_input = first.faces[1].surface_input;
}

// 0x65f0e0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14hasStringValueEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::hasStringValue(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14hasStringValueEv")]
pub fn stub_065f0e0() -> bool {
    // IDA 0x65f0e0 (`SurfaceEnumPropDescriptor<1, InputType>::
    // hasStringValue`): returns 1 — always stringable.
    true
}

// 0x65f0e4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065f0e4(part: &PartSurfaceData) -> String {
    // IDA 0x65f0e4 (`SurfaceEnumPropDescriptor<1, InputType>::
    // getStringValue`): singleton once + inner `getValue` +
    // `convertToString`, same as the face-2 twin at 0x6597d0.
    // Host: the grounded item name.
    input_type_name(part.faces[1].surface_input).to_owned()
}

// 0x65f134 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
pub fn stub_065f134(part: &mut PartSurfaceData, name: &str) -> bool {
    // IDA 0x65f134 (`SurfaceEnumPropDescriptor<1, InputType>::
    // setStringValue(string)`): lookup + convert + conditional
    // inner set, same as the face-2 twin at 0x659820. Host: table
    // position decides.
    match INPUT_TYPE_ITEMS.iter().position(|(n, _)| *n == name) {
        Some(index) => {
            part.faces[1].surface_input = INPUT_TYPE_ITEMS[index].1;
            true
        }
        None => false,
    }
}

// 0x65f198 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
pub fn stub_065f198(part: &PartSurfaceData, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x65f198 (`SurfaceEnumPropDescriptor<1, InputType>::
    // writeValue`): inner `getValue`, `clearValue`, tag `5` at +16,
    // value at +20, returns 5 — same as the face-2 twin at 0x659884.
    out.value_type = 0;
    out.value_type = 5;
    out.int_value = part.faces[1].surface_input as i32;
    5
}

// 0x65f1b8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_065f1b8(part: &mut PartSurfaceData, xml: &XmlReadValue) {
    // IDA 0x65f1b8 (`SurfaceEnumPropDescriptor<1, InputType>::
    // readValue`): xsi:nil early-out, string pair with fallthrough,
    // raw int set, else `ReleaseAssert(false)` (Surface.cpp line
    // 313) — same as the face-2 twin at 0x6598a4 (decompile same
    // length 5208).
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            part.faces[1].surface_input = *value as u32;
        }
        XmlReadValue::Text(text) => {
            if stub_065f134(part, text) {
                return;
            }
            if flog_asserts() {
                panic!("false file: Client/App/v8datamodel/Surface.cpp line: 313 (IDA 0x65f1b8)");
            }
        }
        XmlReadValue::Other => {
            if flog_asserts() {
                panic!("false file: Client/App/v8datamodel/Surface.cpp line: 313 (IDA 0x65f1b8)");
            }
        }
    }
}

// 0x65f410 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065f410(part: &PartSurfaceData) -> i32 {
    // IDA 0x65f410 (`SurfaceEnumPropDescriptor<1, InputType>::
    // getIndexValue`, disasm singleton once + inner `getValue` +
    // tail-call `EnumDesc::convertToIndex`): the item index of the
    // live value (host: stub_0659cc8). Same as the face-2 twin at
    // 0x659afc.
    stub_0659cc8(part.faces[1].surface_input as i32)
}

// 0x65f458 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
pub fn stub_065f458(part: &mut PartSurfaceData, index: u32) -> bool {
    // IDA 0x65f458 (`SurfaceEnumPropDescriptor<1, InputType>::
    // setIndexValue`): `count > index` gates storing the indexed
    // item's value, same as the face-2 twin at 0x659b44. Host:
    // table read decides.
    match INPUT_TYPE_ITEMS.get(index as usize) {
        Some((_, value)) => {
            part.faces[1].surface_input = *value;
            true
        }
        None => false,
    }
}

// 0x65f4b4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065f4b4(part: &PartSurfaceData) -> u32 {
    // IDA 0x65f4b4 (`SurfaceEnumPropDescriptor<1, InputType>::
    // getEnumValue`): inner `getValue` through the +44 `GetSet`
    // (host: stub_065f5e4). Host: the face-1 slot.
    part.faces[1].surface_input
}

// 0x65f4bc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
pub fn stub_065f4bc(part: &mut PartSurfaceData, value: u32) -> bool {
    // IDA 0x65f4bc (`SurfaceEnumPropDescriptor<1, InputType>::
    // setEnumValue`): `find_if` membership + conditional inner set,
    // same as the face-2 twin at 0x659ba8. Host: table membership
    // decides.
    if INPUT_TYPE_ITEMS.iter().any(|(_, v)| *v == value) {
        part.faces[1].surface_input = value;
        true
    } else {
        false
    }
}

// 0x65f530 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065f530(part: &PartSurfaceData) -> Option<u32> {
    // IDA 0x65f530 (`SurfaceEnumPropDescriptor<1, InputType>::
    // getEnumItem`, disasm singleton once + inner `getValue` +
    // `convertToItem`): the table entry for the live value. Same
    // as the face-2 twin at 0x659c1c.
    INPUT_TYPE_ITEMS
        .iter()
        .position(|(_, v)| *v == part.faces[1].surface_input)
        .map(|i| i as u32)
}

// 0x65f580 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
pub fn stub_065f580(part: &mut PartSurfaceData, name: &str) -> bool {
    // IDA 0x65f580 (`SurfaceEnumPropDescriptor<1, InputType>::
    // setStringValue(Name)`): convert + conditional inner set, same
    // as the face-2 twin at 0x659c6c. Host: the string twin in
    // this file.
    stub_065f134(part, name)
}

// 0x65f5dc — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv")]
pub fn stub_065f5dc() -> bool {
    // IDA 0x65f5dc (`SurfaceGetSet<1, InputType>::isReadOnly`):
    // returns 0 — always readable.
    false
}

// 0x65f5e0 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv")]
pub fn stub_065f5e0() -> bool {
    // IDA 0x65f5e0 (`SurfaceGetSet<1, InputType>::isWriteOnly`):
    // returns 0 — always writable.
    false
}

// 0x65f5e4 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065f5e4(part: &PartSurfaceData) -> u32 {
    // IDA 0x65f5e4 (`SurfaceGetSet<1, InputType>::getValue`): the
    // member-pointer resolve tail-calling the getter with the face
    // baked in (`, 1)`, verified in the decompile). Same shape as
    // the face-2 twin at 0x659d40.
    part.faces[1].surface_input
}

// 0x65f604 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_")]
pub fn stub_065f604(part: &mut PartSurfaceData, value: u32) {
    // IDA 0x65f604 (`SurfaceGetSet<1, InputType>::setValue`): the
    // member-pointer resolve tail-calling the setter with
    // `(instance, 1, value)`. Same shape as the face-2 twin at
    // 0x659d60.
    part.faces[1].surface_input = value;
}

// 0x65f628 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
pub fn stub_065f628(name: &str, category: &str, functionality: u32) -> FaceTypeProp {
    // IDA 0x65f628 (`SurfaceEnumPropDescriptor<1, SurfaceType>`
    // ctor): same `classDescriptor` + `EnumDesc` singleton + impl
    // shape as the face-2 twin at 0x659d84 (decompile same length
    // 2547), with the face baked in as 1. Host: the cutover with
    // face 1.
    FaceTypeProp::new(1, name, category, functionality)
}

// 0x65f6d4 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEED0Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEED0Ev")]
pub fn stub_065f6d4() {
    // IDA 0x065f6d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x65f700 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10isReadOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10isReadOnlyEv")]
pub fn stub_065f700() -> bool {
    // IDA 0x65f700 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // isReadOnly`): delegates to the inner face-1 `GetSet` at +44 —
    // always readable (same shape as the face-2 twin at 0x659e5c).
    false
}

// 0x65f710 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE11isWriteOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE11isWriteOnlyEv")]
pub fn stub_065f710() -> bool {
    // IDA 0x65f710 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // isWriteOnly`): delegates to the inner face-1 `GetSet` at +44 —
    // always writable (same shape as the face-2 twin at 0x659e6c).
    false
}

// 0x65f720 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_")]
pub fn stub_065f720(first: &PartSurfaceData, second: &PartSurfaceData) -> bool {
    // IDA 0x65f720 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // equalValues`): reads the inner value for both instances via
    // the +44 `GetSet` and compares. Host: compare the face-1 type
    // slots.
    first.faces[1].surface_type == second.faces[1].surface_type
}

// 0x65f748 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE")]
pub fn stub_065f748(part: &PartSurfaceData) -> SurfaceVariant {
    // IDA 0x65f748 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // getVariant`): reads the inner value, tags it with the
    // `SurfaceType` singleton and placement-moves it in. Same as
    // the face-2 twin at 0x659ea4.
    SurfaceVariant::SurfaceType(part.faces[1].surface_type)
}

// 0x65f770 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE")]
pub fn stub_065f770(part: &mut PartSurfaceData, variant: &SurfaceVariant) {
    // IDA 0x65f770 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // setVariant`): any-cast-or-convert then inner set, same as the
    // face-2 twin at 0x659ecc (decompile same length 2118). Host:
    // the convert-or-throw into the face-1 slot.
    part.faces[1].surface_type = stub_0658e24(variant);
}

// 0x65f8c8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_")]
pub fn stub_065f8c8(first: &PartSurfaceData, second: &mut PartSurfaceData) {
    // IDA 0x65f8c8 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // copyValue`): inner `getValue` + inner `setValue`, same as the
    // face-2 twin at 0x65a024. Host: copy the face-1 type slot.
    second.faces[1].surface_type = first.faces[1].surface_type;
}

// 0x65f8ec — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14hasStringValueEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::hasStringValue(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14hasStringValueEv")]
pub fn stub_065f8ec() -> bool {
    // IDA 0x65f8ec (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // hasStringValue`): returns 1 — always stringable.
    true
}

// 0x65f8f0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065f8f0(part: &PartSurfaceData) -> String {
    // IDA 0x65f8f0 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // getStringValue`): singleton once + inner `getValue` +
    // `convertToString`, same as the face-2 twin at 0x65a04c
    // (decompile same length 1431). Host: the grounded item name.
    surface_type_name(part.faces[1].surface_type).to_owned()
}

// 0x65f940 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
pub fn stub_065f940(part: &mut PartSurfaceData, name: &str) -> bool {
    // IDA 0x65f940 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // setStringValue(string)`): lookup + convert + conditional
    // inner set, same as the face-2 twin at 0x65a09c. Host: table
    // position decides.
    match SURFACE_TYPE_ITEMS.iter().position(|(n, _)| *n == name) {
        Some(index) => {
            part.faces[1].surface_type = SURFACE_TYPE_ITEMS[index].1;
            true
        }
        None => false,
    }
}

// 0x65f9a4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
pub fn stub_065f9a4(part: &PartSurfaceData, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x65f9a4 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // writeValue`): inner `getValue`, `clearValue`, tag `5` at +16,
    // value at +20, returns 5 — same as the face-2 twin at 0x65a100.
    out.value_type = 0;
    out.value_type = 5;
    out.int_value = part.faces[1].surface_type as i32;
    5
}

// 0x65f9c4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_065f9c4(part: &mut PartSurfaceData, xml: &XmlReadValue) {
    // IDA 0x65f9c4 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // readValue`): xsi:nil early-out, string pair with fallthrough,
    // raw int set, else `ReleaseAssert(false)` (Surface.cpp line
    // 313) — same as the face-2 twin at 0x65a120 (decompile length
    // 5112 vs 5111, same shape).
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            part.faces[1].surface_type = *value as u32;
        }
        XmlReadValue::Text(text) => {
            if stub_065f940(part, text) {
                return;
            }
            if flog_asserts() {
                panic!("false file: Client/App/v8datamodel/Surface.cpp line: 313 (IDA 0x65f9c4)");
            }
        }
        XmlReadValue::Other => {
            if flog_asserts() {
                panic!("false file: Client/App/v8datamodel/Surface.cpp line: 313 (IDA 0x65f9c4)");
            }
        }
    }
}

// 0x65fc1c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065fc1c(part: &PartSurfaceData) -> i32 {
    // IDA 0x65fc1c (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // getIndexValue`, disasm singleton once + inner `getValue` +
    // tail-call `EnumDesc::convertToIndex`): the item index of the
    // live value (host: stub_065a544). Same as the face-2 twin at
    // 0x65a378.
    stub_065a544(part.faces[1].surface_type as i32)
}

// 0x65fc64 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
pub fn stub_065fc64(part: &mut PartSurfaceData, index: u32) -> bool {
    // IDA 0x65fc64 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // setIndexValue`): `count > index` gates storing the indexed
    // item's value, same as the face-2 twin at 0x65a3c0. Host:
    // table read decides.
    match SURFACE_TYPE_ITEMS.get(index as usize) {
        Some((_, value)) => {
            part.faces[1].surface_type = *value;
            true
        }
        None => false,
    }
}

// 0x65fcc0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065fcc0(part: &PartSurfaceData) -> u32 {
    // IDA 0x65fcc0 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // getEnumValue`): inner `getValue` through the +44 `GetSet`.
    // Host: the face-1 slot.
    part.faces[1].surface_type
}

// 0x65fcc8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
pub fn stub_065fcc8(part: &mut PartSurfaceData, value: u32) -> bool {
    // IDA 0x65fcc8 (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // setEnumValue`): `find_if` membership + conditional inner set,
    // same as the face-2 twin at 0x65a424. Host: table membership
    // decides.
    if SURFACE_TYPE_ITEMS.iter().any(|(_, v)| *v == value) {
        part.faces[1].surface_type = value;
        true
    } else {
        false
    }
}

// 0x65fd3c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065fd3c(part: &PartSurfaceData) -> Option<u32> {
    // IDA 0x65fd3c (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // getEnumItem`, disasm singleton once + inner `getValue` +
    // `convertToItem`): the table entry for the live value. Same
    // as the face-2 twin at 0x65a498.
    SURFACE_TYPE_ITEMS
        .iter()
        .position(|(_, v)| *v == part.faces[1].surface_type)
        .map(|i| i as u32)
}

// 0x65fd8c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
pub fn stub_065fd8c(part: &mut PartSurfaceData, name: &str) -> bool {
    // IDA 0x65fd8c (`SurfaceEnumPropDescriptor<1, SurfaceType>::
    // setStringValue(Name)`): convert + conditional inner set, same
    // as the face-2 twin at 0x65a4e8. Host: the string twin in
    // this file.
    stub_065f940(part, name)
}

// 0x65fde8 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv")]
pub fn stub_065fde8() -> bool {
    // IDA 0x65fde8 (`SurfaceGetSet<1, SurfaceType>::isReadOnly`):
    // returns 0 — always readable.
    false
}

// 0x65fdec — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv")]
pub fn stub_065fdec() -> bool {
    // IDA 0x65fdec (`SurfaceGetSet<1, SurfaceType>::isWriteOnly`):
    // returns 0 — always writable.
    false
}

// 0x65fdf0 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065fdf0(part: &PartSurfaceData) -> u32 {
    // IDA 0x65fdf0 (`SurfaceGetSet<1, SurfaceType>::getValue`): the
    // member-pointer resolve tail-calling the getter with the face
    // baked in (`, 1)`). Same shape as the face-2 twin at 0x65a5bc.
    part.faces[1].surface_type
}

// 0x65fe10 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_")]
pub fn stub_065fe10(part: &mut PartSurfaceData, value: u32) {
    // IDA 0x65fe10 (`SurfaceGetSet<1, SurfaceType>::setValue`): the
    // member-pointer resolve tail-calling the setter with
    // `(instance, 1, value)`. Same shape as the face-2 twin at
    // 0x65a5dc.
    part.faces[1].surface_type = value;
}

// 0x65fe34 — __ZN3rbx8any_castIN3RBX16LegacyController9InputTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// demangled: RBX::LegacyController::InputType * rbx::any_cast<RBX::LegacyController::InputType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// type: int(void)
#[doc(alias = "RBX::LegacyController::InputType * rbx::any_cast<RBX::LegacyController::InputType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX16LegacyController9InputTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_065fe34(variant: &SurfaceVariant) -> Option<u32> {
    // IDA 0x65fe34 (`any_cast<InputType>`): null input returns null
    // (0x65fe34-0x65fe36); a typeinfo-pointer or mangled-name match
    // (`N3RBX16LegacyController9InputTypeE`) returns the payload
    // (0x65fe5e-0x65fe84); else null (0x65fe88). Host: tagged
    // match.
    match *variant {
        SurfaceVariant::SurfaceInput(value) => Some(value),
        _ => None,
    }
}

// 0x65fe8c — __ZN3rbx8any_castIN3RBX11SurfaceTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// demangled: RBX::SurfaceType * rbx::any_cast<RBX::SurfaceType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// type: int(void)
#[doc(alias = "RBX::SurfaceType * rbx::any_cast<RBX::SurfaceType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX11SurfaceTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_065fe8c(variant: &SurfaceVariant) -> Option<u32> {
    // IDA 0x65fe8c (`any_cast<SurfaceType>`): same null/match/null
    // shape over `N3RBX11SurfaceTypeE` (0x65fe8c-0x65fee0). Host:
    // tagged match.
    match *variant {
        SurfaceVariant::SurfaceType(value) => Some(value),
        _ => None,
    }
}

// 0x65fee4 — __ZN3RBX10Reflection4TypeC2INS_7SurfaceEEEPKcS5_PT_
// demangled: RBX::Reflection::Type::Type<RBX::Surface>(char const*,char const*,RBX::Surface *)
// type: int(void)
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Surface>(char const*,char const*,RBX::Surface *)")]
#[doc(alias = "__ZN3RBX10Reflection4TypeC2INS_7SurfaceEEEPKcS5_PT_")]
pub fn stub_065fee4(tag: &str) -> SurfaceTypeTag {
    // IDA 0x65fee4 (`Type<Surface>` ctor): `Descriptor` base init
    // (0x65fefa), vtable + typeinfo install (0x65ff1a-0x65ff1c),
    // `Name::declare(tag)` (0x65ff24-0x65ff2e), the non-empty-tag
    // `ReleaseAssert` gated on `FLog::Asserts` (Type.h line 77,
    // 0x65ff32-0x65ff7e — a host seam) and `addToAllTypes`
    // (0x65ff80). The vtable/registry fold into the host tag.
    if flog_asserts() {
        assert!(
            !tag.is_empty(),
            "!this->tag.empty() file: include/reflection/Type.h line: 77 (IDA 0x65fee4)"
        );
    }
    SurfaceTypeTag {
        name: tag.to_owned(),
    }
}

// 0x65ff90 — __ZN3RBX10Reflection5TTypeINS_7SurfaceEED0Ev
// demangled: RBX::Reflection::TType<RBX::Surface>::~TType()
#[doc(alias = "RBX::Reflection::TType<RBX::Surface>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_7SurfaceEED0Ev")]
pub fn stub_065ff90() {
    // IDA 0x065ff90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x65ff94 — __GLOBAL__I_a_266
// demangled: global constructor keyed to_a_266
#[doc(alias = "global constructor keyed to_a_266")]
#[doc(alias = "__GLOBAL__I_a_266")]
pub fn stub_065ff94() {
    // IDA 0x65ff94 (`__GLOBAL__I_a_266`): `ios_base::Init` +
    // `__cxa_atexit` (0x65ffa8-0x65ffca), the error-category stores,
    // the per-face `Surface` descriptor static constructions
    // (faces 1/4/3/0/5/2 x Type/Input/float, 0x660030-0x66064a),
    // the `Type<Surface>` singleton (0x66065c), the
    // `boost::exception` statics, the `singleton_pool` guards and
    // the `Camera` creator (0x660682-0x66087e). Host statics
    // initialize on use; only the run is recorded.
    watchdog17_static_init();
}

// 0x660890 — __ZN3RBX16SurfaceSelection10setSurfaceENS_8NormalIdE
// demangled: RBX::SurfaceSelection::setSurface(RBX::NormalId)
#[doc(alias = "RBX::SurfaceSelection::setSurface(RBX::NormalId)")]
#[doc(alias = "__ZN3RBX16SurfaceSelection10setSurfaceENS_8NormalIdE")]
pub fn stub_0660890(state: &mut SurfaceSelectionState, surface: u32) -> bool {
    // IDA 0x660890 (`RBX::SurfaceSelection::setSurface`): compares
    // word 35 (+140, 0x660896); on change stores it (0x6608a2) and
    // raises `raisePropertyChanged` (0x6608ac), else returns
    // unchanged (0x660898). The raise folds into the changed flag
    // (same shape as `StudioTool::setEnabled` at 0x65793c).
    if state.surface == surface {
        return false;
    }
    state.surface = surface;
    true
}

// 0x6608b0 — __ZN3RBX16SurfaceSelectionC2Ev
// demangled: RBX::SurfaceSelection::SurfaceSelection(void)
// type: _DWORD __fastcall(RBX::SurfaceSelection *__hidden this)
#[doc(alias = "RBX::SurfaceSelection::SurfaceSelection(void)")]
#[doc(alias = "__ZN3RBX16SurfaceSelectionC2Ev")]
pub fn stub_06608b0() -> SurfaceSelectionState {
    // IDA 0x6608b0 (`RBX::SurfaceSelection::SurfaceSelection`):
    // `PartAdornment` base with `setName("SurfaceSelection")`
    // (0x6608da), vtable installs + class registration
    // (0x66090c-0x660982) and word 35 (+140, the face) = 0
    // (0x6609b4-0x6609d6). Host: the cleared cutover.
    SurfaceSelectionState {
        surface: 0,
        visible: false,
        part_present: false,
        color: [0.0; 3],
    }
}

// 0x660ac0 — __ZN3RBX16SurfaceSelection13render3dAdornEPNS_5AdornE
// demangled: RBX::SurfaceSelection::render3dAdorn(RBX::Adorn *)
// type: _DWORD __fastcall(RBX::SurfaceSelection *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::SurfaceSelection::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX16SurfaceSelection13render3dAdornEPNS_5AdornE")]
pub fn stub_0660ac0(state: &SurfaceSelectionState) -> Option<SurfaceAdornDraw> {
    // IDA 0x660ac0 (`RBX::SurfaceSelection::render3dAdorn`): the
    // +128 visibility gate (0x660aec); a null +132 part link skips
    // the draw (0x660b1e-0x660b24); `getPart` (0x660b2e), the word-35
    // face (0x660b36), `BrickColor::color3` from +120
    // (0x660b40-0x660b4e), alpha 1.0 (0x660b54 = 1065353216) and
    // `DrawAdorn::partSurface(part, face, adorn, color, 0.2)`
    // (0x660b6c; 1045220557 = 0.2f). The adorn/world handles fold
    // into the request.
    if !state.visible || !state.part_present {
        return None;
    }
    Some(SurfaceAdornDraw {
        surface: state.surface,
        color: state.color,
        alpha: 1.0,
        overlay: 0.2,
    })
}

// 0x660bd8 — __ZThn96_N3RBX16SurfaceSelection13render3dAdornEPNS_5AdornE
// demangled: non-virtual thunk toRBX::SurfaceSelection::render3dAdorn(RBX::Adorn *)
// type: _DWORD __fastcall(RBX::SurfaceSelection *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::SurfaceSelection::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX16SurfaceSelection13render3dAdornEPNS_5AdornE")]
pub fn stub_0660bd8() {
    // IDA 0x0660bd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x660be0 — __ZNK3RBX16SurfaceSelection10getSurfaceEv
// demangled: RBX::SurfaceSelection::getSurface(void)const
// type: _DWORD __fastcall(RBX::SurfaceSelection *__hidden this)
#[doc(alias = "RBX::SurfaceSelection::getSurface(void)const")]
#[doc(alias = "__ZNK3RBX16SurfaceSelection10getSurfaceEv")]
pub fn stub_0660be0(state: &SurfaceSelectionState) -> u32 {
    // IDA 0x660be0 (`RBX::SurfaceSelection::getSurface`): loads word
    // 35 (+140, 0x660be4). Host: direct field read.
    state.surface
}

// 0x660be8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEED1Ev")]
pub fn stub_0660be8() {
    // IDA 0x0660be8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x660c0c — __ZN3RBX16SurfaceSelectionD1Ev
// demangled: RBX::SurfaceSelection::~SurfaceSelection()
// type: void __fastcall(RBX::SurfaceSelection *__hidden this)
#[doc(alias = "RBX::SurfaceSelection::~SurfaceSelection()")]
#[doc(alias = "__ZN3RBX16SurfaceSelectionD1Ev")]
pub fn stub_0660c0c() {
    // IDA 0x0660c0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x660d54 — __ZN3RBX16SurfaceSelectionD0Ev
// demangled: RBX::SurfaceSelection::~SurfaceSelection()
// type: void __fastcall(RBX::SurfaceSelection *__hidden this)
#[doc(alias = "RBX::SurfaceSelection::~SurfaceSelection()")]
#[doc(alias = "__ZN3RBX16SurfaceSelectionD0Ev")]
pub fn stub_0660d54() {
    // IDA 0x0660d54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x660df4 — __ZNK3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0660df4() -> &'static str {
    // IDA 0x660df4 (`FactoryProduct<SurfaceSelection>::getClassName`):
    // `static_getCreator` (0x660df8, host: stub_0661c9c) then the
    // `Creator::getClassName` shim (host: stub_06613fc). Same shape
    // as the `SoundChannel` twin at 0x37750c.
    stub_0661c9c();
    stub_06613fc()
}

// 0x660e04 — __ZThn32_N3RBX16SurfaceSelectionD1Ev
// demangled: non-virtual thunk toRBX::SurfaceSelection::~SurfaceSelection()
// type: void __fastcall(RBX::SurfaceSelection *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SurfaceSelection::~SurfaceSelection()")]
#[doc(alias = "__ZThn32_N3RBX16SurfaceSelectionD1Ev")]
pub fn stub_0660e04() {
    // IDA 0x0660e04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x660f4c — __ZThn32_N3RBX16SurfaceSelectionD0Ev
// demangled: non-virtual thunk toRBX::SurfaceSelection::~SurfaceSelection()
// type: void __fastcall(RBX::SurfaceSelection *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SurfaceSelection::~SurfaceSelection()")]
#[doc(alias = "__ZThn32_N3RBX16SurfaceSelectionD0Ev")]
pub fn stub_0660f4c() {
    // IDA 0x0660f4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6610a8 — __ZThn32_NK3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE12getClassNameEv")]
pub fn stub_06610a8() {
    // IDA 0x06610a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6610b8 — __ZThn36_N3RBX16SurfaceSelectionD1Ev
// demangled: non-virtual thunk toRBX::SurfaceSelection::~SurfaceSelection()
// type: void __fastcall(RBX::SurfaceSelection *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SurfaceSelection::~SurfaceSelection()")]
#[doc(alias = "__ZThn36_N3RBX16SurfaceSelectionD1Ev")]
pub fn stub_06610b8() {
    // IDA 0x06610b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x661200 — __ZThn36_N3RBX16SurfaceSelectionD0Ev
// demangled: non-virtual thunk toRBX::SurfaceSelection::~SurfaceSelection()
// type: void __fastcall(RBX::SurfaceSelection *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SurfaceSelection::~SurfaceSelection()")]
#[doc(alias = "__ZThn36_N3RBX16SurfaceSelectionD0Ev")]
pub fn stub_0661200() {
    // IDA 0x0661200: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66135c — __ZN3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_066135c() {
    // IDA 0x066135c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x661360 — __ZN3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0661360() {
    // IDA 0x0661360: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6613fc — __ZNK3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_06613fc() -> &'static str {
    // IDA 0x6613fc (`Creator::getClassName`): FLog::Asserts-gated
    // `wasConstructed()` (isConstructed == 666) ReleaseAssert
    // (include/Util/Object.h line 236, 0x66140e-0x66145c — a host
    // seam), `Name::declare` call_once (0x661460-0x661478), then
    // tail-calls `doDeclare` (0x661480, host: stub_0661978)
    // returning the `sSurfaceSelection` name. Same shape as the
    // `SoundChannel` twin at 0x377efc.
    if flog_asserts() {
        assert!(
            SURFACE_SELECTION_CONSTRUCTED.load(Ordering::Relaxed),
            "wasConstructed() file: include/Util/Object.h line: 236 (IDA 0x6613fc)"
        );
    }
    stub_0661978()
}

// 0x661484 — __ZNK3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0661484() -> SharedPtr<SurfaceSelectionState> {
    // IDA 0x661484 (`Creator::create`): FLog::Asserts-gated
    // `wasConstructed()` ReleaseAssert (Object.h line 231,
    // 0x6614cc-0x66152a — a host seam), then
    // `Creatable::create<SurfaceSelection>` into a local
    // `shared_ptr` (0x66153c, host: stub_06615c8), null check with
    // the +0x20 `Instance`-base adjust (0x66154e-0x661552, host: no
    // base-subobject offset) and the `shared_count` copy
    // (0x66155e-0x66156c). Same shape as the `SoundChannel` twin
    // at 0x377f84.
    if flog_asserts() {
        assert!(
            SURFACE_SELECTION_CONSTRUCTED.load(Ordering::Relaxed),
            "wasConstructed() file: include/Util/Object.h line: 231 (IDA 0x661484)"
        );
    }
    stub_06615c8()
}

// 0x6615c8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16SurfaceSelectionEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::SurfaceSelection> RBX::Creatable<RBX::Instance>::create<RBX::SurfaceSelection>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::SurfaceSelection> RBX::Creatable<RBX::Instance>::create<RBX::SurfaceSelection>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_16SurfaceSelectionEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_06615c8() -> SharedPtr<SurfaceSelectionState> {
    // IDA 0x6615c8 (`Creatable<Instance>::create<SurfaceSelection>`):
    // `operator new(0x90)` (0x6615fc) + the `SurfaceSelection` ctor
    // (0x661620, host: stub_06608b0) + the adopting `shared_ptr`
    // with `Creatable::Deleter` (0x66162e, host: stub_0661678).
    // Arc construction adopts owners. Same shape as the
    // `SpawnLocation` twin at 0x63edbc.
    SharedPtr::new(stub_06608b0())
}

// 0x661678 — __ZN5boost10shared_ptrIN3RBX16SurfaceSelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::SurfaceSelection>::shared_ptr<RBX::SurfaceSelection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::SurfaceSelection>::shared_ptr<RBX::SurfaceSelection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX16SurfaceSelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0661678(state: SurfaceSelectionState) -> SharedPtr<SurfaceSelectionState> {
    // IDA 0x661678 (`shared_ptr<SurfaceSelection>` from raw +
    // Deleter): stores the pointer (0x661698), builds the
    // `shared_count` with the deleter (0x6616a0) and wires the weak
    // owner for non-null (0x6616ce-0x6616de). Arc move covers it;
    // the control block folds into the `Arc`. Same shape as the
    // `SpawnLocation` twin at 0x63ee70.
    SharedPtr::new(state)
}

// 0x661740 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16SurfaceSelectionES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SurfaceSelection,RBX::SurfaceSelection>(boost::shared_ptr<RBX::SurfaceSelection> const*,RBX::SurfaceSelection *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SurfaceSelection,RBX::SurfaceSelection>(rbx_core::SharedPtr<RBX::SurfaceSelection> const*,RBX::SurfaceSelection *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16SurfaceSelectionES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0661740() {
    // IDA 0x0661740: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x661828 — __ZN5boost6detail12shared_countC2IPN3RBX16SurfaceSelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX16SurfaceSelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0661828() {
    // IDA 0x0661828: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x661930 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16SurfaceSelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16SurfaceSelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0661930() {
    // IDA 0x0661930: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x661934 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16SurfaceSelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16SurfaceSelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0661934() {
    // IDA 0x0661934: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x661938 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16SurfaceSelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16SurfaceSelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0661938() {
    // IDA 0x0661938: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x661958 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16SurfaceSelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16SurfaceSelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0661958() {
    // IDA 0x0661958: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x661970 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16SurfaceSelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SurfaceSelection *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16SurfaceSelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0661970() {
    // IDA 0x0661970: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x661974 — __ZN3RBX4Name13callDoDeclareILZNS_17sSurfaceSelectionEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sSurfaceSelectionEEEEvv")]
pub fn stub_0661974() -> &'static str {
    // IDA 0x661974 (`Name::callDoDeclare<sSurfaceSelection>`):
    // single branch into `doDeclare` (the call_once target, host:
    // stub_0661978). Same shape as the `SoundChannel` twin at
    // 0x378478.
    stub_0661978()
}

// 0x661978 — __ZN3RBX4Name9doDeclareILZNS_17sSurfaceSelectionEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sSurfaceSelectionEEEERKS0_v")]
pub fn stub_0661978() -> &'static str {
    // IDA 0x661978 (`Name::doDeclare<sSurfaceSelection>`):
    // guard-once static `n` (0x6619d4-0x6619fe),
    // `Name::declare(&sSurfaceSelection)` (0x6619fa) stored into
    // `n` and returned (0x661a2c). Host: `LazyLock` init. Same
    // shape as the `SoundChannel` twin at 0x37847c.
    LazyLock::force(&SURFACE_SELECTION_NAME);
    SURFACE_SELECTION_NAME.as_str()
}

// 0x661a58 — __ZN3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0661a58() -> &'static SurfaceSelectionCreator {
    // IDA 0x661a58 (`Creator` C2): vtable install (0x661a8e),
    // `Name::declare` call_once + `doDeclare` (0x661a90-0x661aa6,
    // host: stub_0661978), then the lower_bound walk and unique
    // insert into `getCreators()` keyed by name (0x661aac-0x661bbe)
    // under the duplicate-name (Object.h line 244) and
    // `!wasConstructed` (line 245) ReleaseAsserts, the re-walk
    // verifying the insert (line 250, 0x661bd0-0x661c44) and the
    // final `wasConstructed` assert (line 251, 0x661c4c-0x661c8e)
    // before marking constructed (0x661bbe). Host: the creator is
    // stateless; force the name and mark constructed (same shape
    // as the `RenderSettings` twin at 0xf2bc — no cross-module
    // registry in this crate).
    stub_0661978();
    if flog_asserts() {
        assert!(
            !SURFACE_SELECTION_CONSTRUCTED.load(Ordering::Relaxed),
            "!wasConstructed() file: ../App/include/Util/Object.h line: 245"
        );
    }
    SURFACE_SELECTION_CONSTRUCTED.store(true, Ordering::Relaxed);
    &*SURFACE_SELECTION_CREATOR
}

// 0x661c9c — __ZN3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0661c9c() -> &'static SurfaceSelectionCreator {
    // IDA 0x661c9c (`static_getCreator`): FLog::Asserts-gated
    // `Creator::wasConstructed()` ReleaseAssert (Object.h line 282,
    // 0x661cac-0x661cfe — a host seam), return `creatorPrivate`
    // (0x661d0e, host: the `LazyLock` singleton). Same shape as the
    // `SoundChannel` twin at 0x3787a0.
    if flog_asserts() {
        assert!(
            SURFACE_SELECTION_CONSTRUCTED.load(Ordering::Relaxed),
            "Creator::wasConstructed() file: ../App/include/Util/Object.h line: 282"
        );
    }
    &*SURFACE_SELECTION_CREATOR
}

// 0x661d10 — __ZN3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0661d10() {
    // IDA 0x0661d10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x661e58 — __ZN3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0661e58() {
    // IDA 0x0661e58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x661ef8 — __ZThn32_N3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0661ef8() {
    // IDA 0x0661ef8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x662040 — __ZThn32_N3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0662040() {
    // IDA 0x0662040: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66219c — __ZThn36_N3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_066219c() {
    // IDA 0x066219c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6622e4 — __ZThn36_N3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_06622e4() {
    // IDA 0x06622e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x662440 — __ZN3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0662440(
    name: &str,
    category: &str,
    attributes: u32,
    permissions: u32,
) -> NormalIdProp {
    // IDA 0x662440 (`EnumPropDescriptor<SurfaceSelection, NormalId>`
    // ctor): the `SurfaceSelection` `classDescriptor` call
    // (0x662464), the `EnumDesc<NormalId>` singleton once-init
    // (0x662484-0x662488) and the `PropertyDescriptor` base init
    // with name/category/attributes/permissions plus the impl
    // holding the getter/setter member-pointer pair. The pair folds
    // into direct field access (same shape as `StudioToolBoolProp`
    // at 0x6579d0).
    NormalIdProp::new(name, category, attributes, permissions)
}

// 0x6625f4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEED0Ev")]
pub fn stub_06625f4() {
    // IDA 0x06625f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x662620 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10isReadOnlyEv")]
pub fn stub_0662620() -> bool {
    // IDA 0x662620 (`EnumPropDescriptor<.., NormalId>::isReadOnly`):
    // delegates to the inner `GetSet` at +44 (0x66262c) — always
    // readable.
    false
}

// 0x662630 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE11isWriteOnlyEv")]
pub fn stub_0662630() -> bool {
    // IDA 0x662630 (`EnumPropDescriptor<.., NormalId>::isWriteOnly`):
    // delegates to the inner `GetSet` at +44 (0x66263c) — always
    // writable.
    false
}

// 0x662640 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE11equalValuesEPKNS0_13DescribedBaseES7_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_0662640(first: &SurfaceSelectionState, second: &SurfaceSelectionState) -> bool {
    // IDA 0x662640 (`EnumPropDescriptor<.., NormalId>::equalValues`):
    // reads the inner value for both instances via the +44 `GetSet`
    // (0x662650-0x662666) and compares. Host: compare the faces.
    first.surface == second.surface
}

// 0x662668 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0662668(state: &SurfaceSelectionState) -> SurfaceVariant {
    // IDA 0x662668 (`EnumPropDescriptor<.., NormalId>::getVariant`):
    // calls the getter member (0x662676), tags the value with the
    // plain-`int` singleton (0x66267c) and placement-moves it into
    // the variant (0x66268a). Host: the int-backed `NormalId` tag.
    SurfaceVariant::NormalId(state.surface)
}

// 0x66268c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_066268c(state: &mut SurfaceSelectionState, variant: &SurfaceVariant) {
    // IDA 0x66268c (`EnumPropDescriptor<.., NormalId>::setVariant`):
    // an int-typed variant runs `any_cast<int>` (0x662758); anything
    // else runs `Variant::convert<int>` (0x662738, throws on
    // failure); then the +72 setter = `setSurface` (0x662768, host:
    // stub_0660890). Host: convert-or-throw, then set.
    let value = match *variant {
        SurfaceVariant::NormalId(value) => value as i32,
        _ => panic!("Unable to convert variant to int (IDA 0x66268c)"),
    };
    stub_0660890(state, value as u32);
}

// 0x6627d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE9copyValueEPKNS0_13DescribedBaseEPS5_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_06627d8(first: &SurfaceSelectionState, second: &mut SurfaceSelectionState) {
    // IDA 0x6627d8 (`EnumPropDescriptor<.., NormalId>::copyValue`):
    // inner `getValue` on the source (0x6627ea) then inner
    // `setValue` on the target (0x6627fa). Host: copy the face.
    second.surface = first.surface;
}

// 0x6627fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::hasStringValue(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE14hasStringValueEv")]
pub fn stub_06627fc() -> bool {
    // IDA 0x6627fc (`EnumPropDescriptor<.., NormalId>::hasStringValue`):
    // returns 1 — always stringable.
    true
}

// 0x662800 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0662800(state: &SurfaceSelectionState) -> String {
    // IDA 0x662800 (`EnumPropDescriptor<.., NormalId>::getStringValue`):
    // reads the enum-desc singleton slot at +48 (0x66280a), the
    // inner value via the +44 `GetSet` (0x662812) and
    // `EnumDesc::convertToString` (0x662822). Host: the grounded
    // item name.
    normal_id_name(state.surface).to_owned()
}