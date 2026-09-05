//! audio generated_audio_wd_watchdog16 — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x065cb80 | rbx_core::SharedPtr not boost
//! Range 0x065cbac..0x065edd4 | existing 36603 -> 36703 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
use crate::generated::flog_asserts;
use crate::generated_134::{XmlIntSlot, XmlReadValue};
use crate::generated_audio_wd_watchdog13::SurfaceState;
use crate::generated_audio_wd_watchdog14::{
    FaceFloatProp, FaceFloatSlot, FaceInputProp, FaceTypeProp, INPUT_TYPE_ITEMS, PartSurfaceData,
    SURFACE_TYPE_ITEMS, SurfaceVariant, input_type_index, input_type_name, stub_0658e24,
    stub_0658f8c, stub_0659cc8, stub_065a544, surface_type_index, surface_type_name,
};
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };


// 0x065cbac — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE10isReadOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE10isReadOnlyEv")]
pub fn stub_065cbac() -> bool {
    // IDA 0x65cbac (`SurfaceEnumPropDescriptor<3, InputType>::
    // isReadOnly`): delegates to the inner face-3 `GetSet` at +44 —
    // always readable (same shape as the face-2 twin at 0x6595e0).
    false
}

// 0x065cbbc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE11isWriteOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")]
pub fn stub_065cbbc() -> bool {
    // IDA 0x65cbbc (`SurfaceEnumPropDescriptor<3, InputType>::
    // isWriteOnly`): delegates to the inner face-3 `GetSet` at +44 —
    // always writable (same shape as the face-2 twin at 0x6595f0).
    false
}

// 0x065cbcc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")]
pub fn stub_065cbcc(first: &PartSurfaceData, second: &PartSurfaceData) -> bool {
    // IDA 0x65cbcc (`SurfaceEnumPropDescriptor<3, InputType>::
    // equalValues`): reads the inner value for both instances via
    // the +44 `GetSet` and compares. Host: compare the face-3
    // input slots.
    first.faces[3].surface_input == second.faces[3].surface_input
}

// 0x065cbf4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")]
pub fn stub_065cbf4(part: &PartSurfaceData) -> SurfaceVariant {
    // IDA 0x65cbf4 (`SurfaceEnumPropDescriptor<3, InputType>::
    // getVariant`): reads the inner value, tags it with the
    // `InputType` singleton and placement-moves it in. Same as the
    // face-2 twin at 0x659628.
    SurfaceVariant::SurfaceInput(part.faces[3].surface_input)
}

// 0x065cc1c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")]
pub fn stub_065cc1c(part: &mut PartSurfaceData, variant: &SurfaceVariant) {
    // IDA 0x65cc1c (`SurfaceEnumPropDescriptor<3, InputType>::
    // setVariant`): any-cast-or-convert then inner set, same as the
    // face-2 twin at 0x659650 (decompile same length 2201). Host:
    // the convert-or-throw into the face-3 slot.
    part.faces[3].surface_input = stub_0658f8c(variant);
}

// 0x065cd74 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")]
pub fn stub_065cd74(first: &PartSurfaceData, second: &mut PartSurfaceData) {
    // IDA 0x65cd74 (`SurfaceEnumPropDescriptor<3, InputType>::
    // copyValue`): inner `getValue` + inner `setValue`, same as the
    // face-2 twin at 0x6597a8. Host: copy the face-3 slot.
    second.faces[3].surface_input = first.faces[3].surface_input;
}

// 0x065cd98 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE14hasStringValueEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::hasStringValue(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE14hasStringValueEv")]
pub fn stub_065cd98() -> bool {
    // IDA 0x65cd98 (`SurfaceEnumPropDescriptor<3, InputType>::
    // hasStringValue`): returns 1 — always stringable.
    true
}

// 0x065cd9c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065cd9c(part: &PartSurfaceData) -> String {
    // IDA 0x65cd9c (`SurfaceEnumPropDescriptor<3, InputType>::
    // getStringValue`): singleton once + inner `getValue` +
    // `convertToString`, same as the face-2 twin at 0x6597d0.
    // Host: the grounded item name.
    input_type_name(part.faces[3].surface_input).to_owned()
}

// 0x065cdec — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
pub fn stub_065cdec(part: &mut PartSurfaceData, name: &str) -> bool {
    // IDA 0x65cdec (`SurfaceEnumPropDescriptor<3, InputType>::
    // setStringValue(string)`): lookup + convert + conditional
    // inner set, same as the face-2 twin at 0x659820. Host: table
    // position decides.
    match INPUT_TYPE_ITEMS.iter().position(|(n, _)| *n == name) {
        Some(index) => {
            part.faces[3].surface_input = INPUT_TYPE_ITEMS[index].1;
            true
        }
        None => false,
    }
}

// 0x065ce50 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
pub fn stub_065ce50(part: &PartSurfaceData, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x65ce50 (`SurfaceEnumPropDescriptor<3, InputType>::
    // writeValue`): inner `getValue`, `clearValue`, tag `5` at +16,
    // value at +20, returns 5 — same as the face-2 twin at 0x659884.
    out.value_type = 0;
    out.value_type = 5;
    out.int_value = part.faces[3].surface_input as i32;
    5
}

// 0x065ce70 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_065ce70(part: &mut PartSurfaceData, xml: &XmlReadValue) {
    // IDA 0x65ce70 (`SurfaceEnumPropDescriptor<3, InputType>::
    // readValue`): xsi:nil early-out, string pair with fallthrough,
    // raw int set, else `ReleaseAssert(false)` (Surface.cpp line
    // 313) — same as the face-2 twin at 0x6598a4 (decompile same
    // length 5208).
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            part.faces[3].surface_input = *value as u32;
        }
        XmlReadValue::Text(text) => {
            if stub_065cdec(part, text) {
                return;
            }
            if flog_asserts() {
                panic!("false file: Client/App/v8datamodel/Surface.cpp line: 313 (IDA 0x65ce70)");
            }
        }
        XmlReadValue::Other => {
            if flog_asserts() {
                panic!("false file: Client/App/v8datamodel/Surface.cpp line: 313 (IDA 0x65ce70)");
            }
        }
    }
}

// 0x065d0c8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065d0c8(part: &PartSurfaceData) -> i32 {
    // IDA 0x65d0c8 (`SurfaceEnumPropDescriptor<3, InputType>::
    // getIndexValue`, disasm singleton once + inner `getValue` +
    // tail-call `EnumDesc::convertToIndex`): the item index of the
    // live value (host: stub_0659cc8). Same as the face-2 twin at
    // 0x659afc.
    stub_0659cc8(part.faces[3].surface_input as i32)
}

// 0x065d110 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
pub fn stub_065d110(part: &mut PartSurfaceData, index: u32) -> bool {
    // IDA 0x65d110 (`SurfaceEnumPropDescriptor<3, InputType>::
    // setIndexValue`): `count > index` gates storing the indexed
    // item's value, same as the face-2 twin at 0x659b44. Host:
    // table read decides.
    match INPUT_TYPE_ITEMS.get(index as usize) {
        Some((_, value)) => {
            part.faces[3].surface_input = *value;
            true
        }
        None => false,
    }
}

// 0x065d16c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065d16c(part: &PartSurfaceData) -> u32 {
    // IDA 0x65d16c (`SurfaceEnumPropDescriptor<3, InputType>::
    // getEnumValue`): inner `getValue` through the +44 `GetSet`
    // (host: stub_065d29c). Host: the face-3 slot.
    part.faces[3].surface_input
}

// 0x065d174 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
pub fn stub_065d174(part: &mut PartSurfaceData, value: u32) -> bool {
    // IDA 0x65d174 (`SurfaceEnumPropDescriptor<3, InputType>::
    // setEnumValue`): `find_if` membership + conditional inner set,
    // same as the face-2 twin at 0x659ba8. Host: table membership
    // decides.
    if INPUT_TYPE_ITEMS.iter().any(|(_, v)| *v == value) {
        part.faces[3].surface_input = value;
        true
    } else {
        false
    }
}

// 0x065d1e8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065d1e8(part: &PartSurfaceData) -> Option<u32> {
    // IDA 0x65d1e8 (`SurfaceEnumPropDescriptor<3, InputType>::
    // getEnumItem`, disasm singleton once + inner `getValue` +
    // `convertToItem`): the table entry for the live value. Same
    // as the face-2 twin at 0x659c1c.
    INPUT_TYPE_ITEMS
        .iter()
        .position(|(_, v)| *v == part.faces[3].surface_input)
        .map(|i| i as u32)
}

// 0x065d238 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
pub fn stub_065d238(part: &mut PartSurfaceData, name: &str) -> bool {
    // IDA 0x65d238 (`SurfaceEnumPropDescriptor<3, InputType>::
    // setStringValue(Name)`): convert + conditional inner set, same
    // as the face-2 twin at 0x659c6c. Host: the string twin in
    // this file.
    stub_065cdec(part, name)
}

// 0x065d294 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv")]
pub fn stub_065d294() -> bool {
    // IDA 0x65d294 (`SurfaceGetSet<3, InputType>::isReadOnly`):
    // returns 0 — always readable.
    false
}

// 0x065d298 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv")]
pub fn stub_065d298() -> bool {
    // IDA 0x65d298 (`SurfaceGetSet<3, InputType>::isWriteOnly`):
    // returns 0 — always writable.
    false
}

// 0x065d29c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065d29c(part: &PartSurfaceData) -> u32 {
    // IDA 0x65d29c (`SurfaceGetSet<3, InputType>::getValue`): the
    // member-pointer resolve tail-calling the getter with the face
    // baked in (`, 3)`, verified in the decompile). Same shape as
    // the face-2 twin at 0x659d40.
    part.faces[3].surface_input
}

// 0x065d2bc — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_")]
pub fn stub_065d2bc(part: &mut PartSurfaceData, value: u32) {
    // IDA 0x65d2bc (`SurfaceGetSet<3, InputType>::setValue`): the
    // member-pointer resolve tail-calling the setter with
    // `(instance, 3, value)`. Same shape as the face-2 twin at
    // 0x659d60.
    part.faces[3].surface_input = value;
}

// 0x065d2e0 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
pub fn stub_065d2e0(name: &str, category: &str, functionality: u32) -> FaceTypeProp {
    // IDA 0x65d2e0 (`SurfaceEnumPropDescriptor<3, SurfaceType>`
    // ctor): same `classDescriptor` + `EnumDesc` singleton + impl
    // shape as the face-2 twin at 0x659d84 (decompile same length
    // 2547), with the face baked in as 3. Host: the cutover with
    // face 3.
    FaceTypeProp::new(3, name, category, functionality)
}

// 0x065d38c — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEED0Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEED0Ev")]
pub fn stub_065d38c() {
    // IDA 0x065d38c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065d3b8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10isReadOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10isReadOnlyEv")]
pub fn stub_065d3b8() -> bool {
    // IDA 0x65d3b8 (`SurfaceEnumPropDescriptor<3, SurfaceType>::
    // isReadOnly`): delegates to the inner face-3 `GetSet` at +44 —
    // always readable (same shape as the face-2 twin at 0x659e5c).
    false
}

// 0x065d3c8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE11isWriteOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE11isWriteOnlyEv")]
pub fn stub_065d3c8() -> bool {
    // IDA 0x65d3c8 (`SurfaceEnumPropDescriptor<3, SurfaceType>::
    // isWriteOnly`): delegates to the inner face-3 `GetSet` at +44 —
    // always writable (same shape as the face-2 twin at 0x659e6c).
    false
}

// 0x065d3d8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_")]
pub fn stub_065d3d8(first: &PartSurfaceData, second: &PartSurfaceData) -> bool {
    // IDA 0x65d3d8 (`SurfaceEnumPropDescriptor<3, SurfaceType>::
    // equalValues`): reads the inner value for both instances via
    // the +44 `GetSet` and compares. Host: compare the face-3 type
    // slots.
    first.faces[3].surface_type == second.faces[3].surface_type
}

// 0x065d400 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE")]
pub fn stub_065d400() -> ! {
    todo!("0x065d400 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE")
}

// 0x065d428 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE")]
pub fn stub_065d428() -> ! {
    todo!("0x065d428 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE")
}

// 0x065d580 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_")]
pub fn stub_065d580() -> ! {
    todo!("0x065d580 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_")
}

// 0x065d5a4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14hasStringValueEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::hasStringValue(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14hasStringValueEv")]
pub fn stub_065d5a4() -> ! {
    todo!("0x065d5a4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14hasStringValueEv")
}

// 0x065d5a8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065d5a8() -> ! {
    todo!("0x065d5a8 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065d5f8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
pub fn stub_065d5f8() -> ! {
    todo!("0x065d5f8 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")
}

// 0x065d65c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
pub fn stub_065d65c() -> ! {
    todo!("0x065d65c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")
}

// 0x065d67c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_065d67c() -> ! {
    todo!("0x065d67c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x065d8d4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065d8d4() -> ! {
    todo!("0x065d8d4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065d91c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
pub fn stub_065d91c() -> ! {
    todo!("0x065d91c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")
}

// 0x065d978 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065d978() -> ! {
    todo!("0x065d978 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065d980 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
pub fn stub_065d980() -> ! {
    todo!("0x065d980 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")
}

// 0x065d9f4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065d9f4() -> ! {
    todo!("0x065d9f4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")
}

// 0x065da44 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
pub fn stub_065da44() -> ! {
    todo!("0x065da44 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")
}

// 0x065daa0 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv")]
pub fn stub_065daa0() -> ! {
    todo!("0x065daa0 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv")
}

// 0x065daa4 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv")]
pub fn stub_065daa4() -> ! {
    todo!("0x065daa4 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv")
}

// 0x065daa8 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065daa8() -> ! {
    todo!("0x065daa8 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065dac8 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)3,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_")]
pub fn stub_065dac8() -> ! {
    todo!("0x065dac8 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_")
}

// 0x065daec — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)4,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)4,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE")]
pub fn stub_065daec() -> ! {
    todo!("0x065daec __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE")
}

// 0x065dc00 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfED0Ev
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)4,float>::~SurfacePropDescriptor()
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)4,float>::~SurfacePropDescriptor()")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfED0Ev")]
pub fn stub_065dc00() {
    // IDA 0x065dc00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065dc2c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)4,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv")]
pub fn stub_065dc2c() -> ! {
    todo!("0x065dc2c __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv")
}

// 0x065dc30 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)4,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv")]
pub fn stub_065dc30() -> ! {
    todo!("0x065dc30 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv")
}

// 0x065dc34 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)4,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065dc34() -> ! {
    todo!("0x065dc34 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065dc54 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)4,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf")]
pub fn stub_065dc54() -> ! {
    todo!("0x065dc54 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf")
}

// 0x065dc78 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
pub fn stub_065dc78() -> ! {
    todo!("0x065dc78 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")
}

// 0x065dd24 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED0Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED0Ev")]
pub fn stub_065dd24() {
    // IDA 0x065dd24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065dd50 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10isReadOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10isReadOnlyEv")]
pub fn stub_065dd50() -> ! {
    todo!("0x065dd50 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10isReadOnlyEv")
}

// 0x065dd60 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE11isWriteOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")]
pub fn stub_065dd60() -> ! {
    todo!("0x065dd60 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")
}

// 0x065dd70 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")]
pub fn stub_065dd70() -> ! {
    todo!("0x065dd70 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")
}

// 0x065dd98 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")]
pub fn stub_065dd98() -> ! {
    todo!("0x065dd98 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")
}

// 0x065ddc0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")]
pub fn stub_065ddc0() -> ! {
    todo!("0x065ddc0 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")
}

// 0x065df18 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")]
pub fn stub_065df18() -> ! {
    todo!("0x065df18 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")
}

// 0x065df3c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14hasStringValueEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::hasStringValue(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14hasStringValueEv")]
pub fn stub_065df3c() -> ! {
    todo!("0x065df3c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14hasStringValueEv")
}

// 0x065df40 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065df40() -> ! {
    todo!("0x065df40 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065df90 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
pub fn stub_065df90() -> ! {
    todo!("0x065df90 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")
}

// 0x065dff4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
pub fn stub_065dff4() -> ! {
    todo!("0x065dff4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")
}

// 0x065e014 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_065e014() -> ! {
    todo!("0x065e014 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x065e26c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065e26c() -> ! {
    todo!("0x065e26c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065e2b4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
pub fn stub_065e2b4() -> ! {
    todo!("0x065e2b4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")
}

// 0x065e310 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065e310() -> ! {
    todo!("0x065e310 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065e318 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
pub fn stub_065e318() -> ! {
    todo!("0x065e318 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")
}

// 0x065e38c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065e38c() -> ! {
    todo!("0x065e38c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")
}

// 0x065e3dc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
pub fn stub_065e3dc() -> ! {
    todo!("0x065e3dc __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")
}

// 0x065e438 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv")]
pub fn stub_065e438() -> ! {
    todo!("0x065e438 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv")
}

// 0x065e43c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv")]
pub fn stub_065e43c() -> ! {
    todo!("0x065e43c __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv")
}

// 0x065e440 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065e440() -> ! {
    todo!("0x065e440 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065e460 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_")]
pub fn stub_065e460() -> ! {
    todo!("0x065e460 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_")
}

// 0x065e484 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
pub fn stub_065e484() -> ! {
    todo!("0x065e484 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")
}

// 0x065e530 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEED0Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEED0Ev")]
pub fn stub_065e530() {
    // IDA 0x065e530: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065e55c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10isReadOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10isReadOnlyEv")]
pub fn stub_065e55c() -> ! {
    todo!("0x065e55c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10isReadOnlyEv")
}

// 0x065e56c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11isWriteOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11isWriteOnlyEv")]
pub fn stub_065e56c() -> ! {
    todo!("0x065e56c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11isWriteOnlyEv")
}

// 0x065e57c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_")]
pub fn stub_065e57c() -> ! {
    todo!("0x065e57c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_")
}

// 0x065e5a4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE")]
pub fn stub_065e5a4() -> ! {
    todo!("0x065e5a4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE")
}

// 0x065e5cc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE")]
pub fn stub_065e5cc() -> ! {
    todo!("0x065e5cc __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE")
}

// 0x065e724 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_")]
pub fn stub_065e724() -> ! {
    todo!("0x065e724 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_")
}

// 0x065e748 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14hasStringValueEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::hasStringValue(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14hasStringValueEv")]
pub fn stub_065e748() -> ! {
    todo!("0x065e748 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14hasStringValueEv")
}

// 0x065e74c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065e74c() -> ! {
    todo!("0x065e74c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065e79c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
pub fn stub_065e79c() -> ! {
    todo!("0x065e79c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")
}

// 0x065e800 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
pub fn stub_065e800() -> ! {
    todo!("0x065e800 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")
}

// 0x065e820 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_065e820() -> ! {
    todo!("0x065e820 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x065ea78 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065ea78() -> ! {
    todo!("0x065ea78 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065eac0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
pub fn stub_065eac0() -> ! {
    todo!("0x065eac0 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")
}

// 0x065eb1c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065eb1c() -> ! {
    todo!("0x065eb1c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065eb24 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
pub fn stub_065eb24() -> ! {
    todo!("0x065eb24 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")
}

// 0x065eb98 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065eb98() -> ! {
    todo!("0x065eb98 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")
}

// 0x065ebe8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
pub fn stub_065ebe8() -> ! {
    todo!("0x065ebe8 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")
}

// 0x065ec44 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv")]
pub fn stub_065ec44() -> ! {
    todo!("0x065ec44 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv")
}

// 0x065ec48 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv")]
pub fn stub_065ec48() -> ! {
    todo!("0x065ec48 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv")
}

// 0x065ec4c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065ec4c() -> ! {
    todo!("0x065ec4c __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065ec6c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_")]
pub fn stub_065ec6c() -> ! {
    todo!("0x065ec6c __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_")
}

// 0x065ec90 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE")]
pub fn stub_065ec90() -> ! {
    todo!("0x065ec90 __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE")
}

// 0x065eda4 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfED0Ev
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::~SurfacePropDescriptor()
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::~SurfacePropDescriptor()")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfED0Ev")]
pub fn stub_065eda4() {
    // IDA 0x065eda4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065edd0 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)1,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv")]
pub fn stub_065edd0() -> ! {
    todo!("0x065edd0 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv")
}

// 0x065edd4 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)1,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv")]
pub fn stub_065edd4() -> ! {
    todo!("0x065edd4 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv")
}
