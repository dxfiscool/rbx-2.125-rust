//! audio generated_audio_wd_watchdog15 — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x065a8d3 | rbx_core::SharedPtr not boost
//! Range 0x065a8d4..0x065cb80 | existing 36502 -> 36602 distinct
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


// 0x065a8d4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")]
pub fn stub_065a8d4(part: &mut PartSurfaceData, variant: &SurfaceVariant) {
    // IDA 0x65a8d4 (`SurfaceEnumPropDescriptor<5, InputType>::
    // setVariant`): same any-cast-or-convert shape as the face-2
    // twin at 0x659650 (decompile same length 2201). Host: convert
    // into the face-5 slot.
    part.faces[5].surface_input = stub_0658f8c(variant);
}

// 0x065aa2c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")]
pub fn stub_065aa2c(first: &PartSurfaceData, second: &mut PartSurfaceData) {
    // IDA 0x65aa2c (`SurfaceEnumPropDescriptor<5, InputType>::
    // copyValue`): inner `getValue` + inner `setValue`, same as the
    // face-2 twin at 0x6597a8. Host: copy the face-5 slot.
    second.faces[5].surface_input = first.faces[5].surface_input;
}

// 0x065aa50 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14hasStringValueEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::hasStringValue(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14hasStringValueEv")]
pub fn stub_065aa50() -> bool {
    // IDA 0x65aa50 (`SurfaceEnumPropDescriptor<5, InputType>::
    // hasStringValue`): returns 1 — always stringable, same as the
    // face-2 twin at 0x6597cc.
    true
}

// 0x065aa54 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065aa54(part: &PartSurfaceData) -> String {
    // IDA 0x65aa54 (`SurfaceEnumPropDescriptor<5, InputType>::
    // getStringValue`): singleton once + inner `getValue` +
    // `convertToString`, same as the face-2 twin at 0x6597d0.
    // Host: the grounded item name.
    input_type_name(part.faces[5].surface_input).to_owned()
}

// 0x065aaa4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
pub fn stub_065aaa4(part: &mut PartSurfaceData, name: &str) -> bool {
    // IDA 0x65aaa4 (`SurfaceEnumPropDescriptor<5, InputType>::
    // setStringValue(string)`): lookup + convert + conditional
    // inner set, same as the face-2 twin at 0x659820. Host: table
    // position decides.
    match INPUT_TYPE_ITEMS.iter().position(|(n, _)| *n == name) {
        Some(index) => {
            part.faces[5].surface_input = INPUT_TYPE_ITEMS[index].1;
            true
        }
        None => false,
    }
}

// 0x065ab08 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
pub fn stub_065ab08(part: &PartSurfaceData, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x65ab08 (`SurfaceEnumPropDescriptor<5, InputType>::
    // writeValue`): inner `getValue`, `clearValue`, tag `5` at +16,
    // value at +20, returns 5 — same as the face-2 twin at 0x659884.
    out.value_type = 0;
    out.value_type = 5;
    out.int_value = part.faces[5].surface_input as i32;
    5
}

// 0x065ab28 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_065ab28(part: &mut PartSurfaceData, xml: &XmlReadValue) {
    // IDA 0x65ab28 (`SurfaceEnumPropDescriptor<5, InputType>::
    // readValue`): xsi:nil early-out, string pair with fallthrough,
    // raw int set, else `ReleaseAssert(false)` (Surface.cpp line
    // 313) — same as the face-2 twin at 0x6598a4 (decompile same
    // length 5208).
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            part.faces[5].surface_input = *value as u32;
        }
        XmlReadValue::Text(text) => {
            if stub_065aaa4(part, text) {
                return;
            }
            if flog_asserts() {
                panic!("false file: Client/App/v8datamodel/Surface.cpp line: 313 (IDA 0x65ab28)");
            }
        }
        XmlReadValue::Other => {
            if flog_asserts() {
                panic!("false file: Client/App/v8datamodel/Surface.cpp line: 313 (IDA 0x65ab28)");
            }
        }
    }
}

// 0x065ad80 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065ad80(part: &PartSurfaceData) -> i32 {
    // IDA 0x65ad80 (`SurfaceEnumPropDescriptor<5, InputType>::
    // getIndexValue`, disasm singleton once + inner `getValue` +
    // tail-call `convertToIndex`): the item index of the live value
    // (host: stub_0659cc8). Same as the face-2 twin at 0x659afc.
    stub_0659cc8(part.faces[5].surface_input as i32)
}

// 0x065adc8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
pub fn stub_065adc8(part: &mut PartSurfaceData, index: u32) -> bool {
    // IDA 0x65adc8 (`SurfaceEnumPropDescriptor<5, InputType>::
    // setIndexValue`): `count > index` gates storing the indexed
    // item's value, same as the face-2 twin at 0x659b44. Host:
    // table read decides.
    match INPUT_TYPE_ITEMS.get(index as usize) {
        Some((_, value)) => {
            part.faces[5].surface_input = *value;
            true
        }
        None => false,
    }
}

// 0x065ae24 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065ae24(part: &PartSurfaceData) -> u32 {
    // IDA 0x65ae24 (`SurfaceEnumPropDescriptor<5, InputType>::
    // getEnumValue`): inner `getValue` through the +44 `GetSet`
    // (host: stub_065af54). Host: the face-5 slot.
    part.faces[5].surface_input
}

// 0x065ae2c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
pub fn stub_065ae2c(part: &mut PartSurfaceData, value: u32) -> bool {
    // IDA 0x65ae2c (`SurfaceEnumPropDescriptor<5, InputType>::
    // setEnumValue`): `find_if` membership + conditional inner set,
    // same as the face-2 twin at 0x659ba8. Host: table membership
    // decides.
    if INPUT_TYPE_ITEMS.iter().any(|(_, v)| *v == value) {
        part.faces[5].surface_input = value;
        true
    } else {
        false
    }
}

// 0x065aea0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065aea0(part: &PartSurfaceData) -> Option<u32> {
    // IDA 0x65aea0 (`SurfaceEnumPropDescriptor<5, InputType>::
    // getEnumItem`, disasm singleton once + inner `getValue` +
    // `convertToItem`): the table entry for the live value. Same
    // as the face-2 twin at 0x659c1c.
    INPUT_TYPE_ITEMS
        .iter()
        .position(|(_, v)| *v == part.faces[5].surface_input)
        .map(|i| i as u32)
}

// 0x065aef0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
pub fn stub_065aef0(part: &mut PartSurfaceData, name: &str) -> bool {
    // IDA 0x65aef0 (`SurfaceEnumPropDescriptor<5, InputType>::
    // setStringValue(Name)`): convert + conditional inner set, same
    // as the face-2 twin at 0x659c6c. Host: the string twin in
    // this file.
    stub_065aaa4(part, name)
}

// 0x065af4c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv")]
pub fn stub_065af4c() -> bool {
    // IDA 0x65af4c (`SurfaceGetSet<5, InputType>::isReadOnly`):
    // `MOVS R0, #0; BX LR` — always readable.
    false
}

// 0x065af50 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv")]
pub fn stub_065af50() -> bool {
    // IDA 0x65af50 (`SurfaceGetSet<5, InputType>::isWriteOnly`):
    // `MOVS R0, #0; BX LR` — always writable.
    false
}

// 0x065af54 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065af54(part: &PartSurfaceData) -> u32 {
    // IDA 0x65af54 (`SurfaceGetSet<5, InputType>::getValue`): the
    // member-pointer resolve tail-calling the getter with the face
    // baked in (`, 5)`, verified in the decompile). Same shape as
    // the face-2 twin at 0x659d40.
    part.faces[5].surface_input
}

// 0x065af74 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_")]
pub fn stub_065af74(part: &mut PartSurfaceData, value: u32) {
    // IDA 0x65af74 (`SurfaceGetSet<5, InputType>::setValue`): the
    // member-pointer resolve tail-calling the setter with
    // `(instance, 5, value)`. Same shape as the face-2 twin at
    // 0x659d60.
    part.faces[5].surface_input = value;
}

// 0x065af98 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
pub fn stub_065af98(name: &str, category: &str, functionality: u32) -> FaceTypeProp {
    // IDA 0x65af98 (`SurfaceEnumPropDescriptor<5, SurfaceType>`
    // ctor): same `classDescriptor` + `EnumDesc` singleton +
    // impl shape as the face-2 twin at 0x659d84 (decompile same
    // length 2547), with the face baked in as 5. Host: the cutover
    // with face 5.
    FaceTypeProp::new(5, name, category, functionality)
}

// 0x065b044 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEED0Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEED0Ev")]
pub fn stub_065b044() {
    // IDA 0x065b044: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065b070 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10isReadOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10isReadOnlyEv")]
pub fn stub_065b070() -> bool {
    // IDA 0x65b070 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // isReadOnly`): delegates to the inner `GetSet` at +44 —
    // always readable (same shape as the face-2 twin at 0x659e5c).
    false
}

// 0x065b080 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE11isWriteOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE11isWriteOnlyEv")]
pub fn stub_065b080() -> bool {
    // IDA 0x65b080 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // isWriteOnly`): delegates to the inner `GetSet` at +44 —
    // always writable (same shape as the face-2 twin at 0x659e6c).
    false
}

// 0x065b090 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_")]
pub fn stub_065b090(first: &PartSurfaceData, second: &PartSurfaceData) -> bool {
    // IDA 0x65b090 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // equalValues`): reads the inner value for both instances and
    // compares, same as the face-2 twin at 0x659e7c. Host: compare
    // the face-5 type slots.
    first.faces[5].surface_type == second.faces[5].surface_type
}

// 0x065b0b8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE")]
pub fn stub_065b0b8(part: &PartSurfaceData) -> SurfaceVariant {
    // IDA 0x65b0b8 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // getVariant`): reads the inner value, tags it with the
    // `SurfaceType` singleton and placement-moves it in. Same as
    // the face-2 twin at 0x659ea4.
    SurfaceVariant::SurfaceType(part.faces[5].surface_type)
}

// 0x065b0e0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE")]
pub fn stub_065b0e0(part: &mut PartSurfaceData, variant: &SurfaceVariant) {
    // IDA 0x65b0e0 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // setVariant`): any-cast-or-convert then inner set, same as the
    // face-2 twin at 0x659ecc (decompile same length 2118). Host:
    // the convert-or-throw into the face-5 slot.
    part.faces[5].surface_type = stub_0658e24(variant);
}

// 0x065b238 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_")]
pub fn stub_065b238(first: &PartSurfaceData, second: &mut PartSurfaceData) {
    // IDA 0x65b238 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // copyValue`): inner `getValue` + inner `setValue`, same as the
    // face-2 twin at 0x65a024. Host: copy the face-5 type slot.
    second.faces[5].surface_type = first.faces[5].surface_type;
}

// 0x065b25c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14hasStringValueEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::hasStringValue(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14hasStringValueEv")]
pub fn stub_065b25c() -> bool {
    // IDA 0x65b25c (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // hasStringValue`): returns 1 — always stringable, same as the
    // face-2 twin at 0x65a048.
    true
}

// 0x065b260 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065b260(part: &PartSurfaceData) -> String {
    // IDA 0x65b260 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // getStringValue`): singleton once + inner `getValue` +
    // `convertToString`, same as the face-2 twin at 0x65a04c
    // (decompile same length 1431). Host: the grounded item name.
    surface_type_name(part.faces[5].surface_type).to_owned()
}

// 0x065b2b0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
pub fn stub_065b2b0(part: &mut PartSurfaceData, name: &str) -> bool {
    // IDA 0x65b2b0 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // setStringValue(string)`): lookup + convert + conditional
    // inner set, same as the face-2 twin at 0x65a09c. Host: table
    // position decides.
    match SURFACE_TYPE_ITEMS.iter().position(|(n, _)| *n == name) {
        Some(index) => {
            part.faces[5].surface_type = SURFACE_TYPE_ITEMS[index].1;
            true
        }
        None => false,
    }
}

// 0x065b314 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
pub fn stub_065b314(part: &PartSurfaceData, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x65b314 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // writeValue`): inner `getValue`, `clearValue`, tag `5` at +16,
    // value at +20, returns 5 — same as the face-2 twin at 0x65a100.
    out.value_type = 0;
    out.value_type = 5;
    out.int_value = part.faces[5].surface_type as i32;
    5
}

// 0x065b334 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_065b334(part: &mut PartSurfaceData, xml: &XmlReadValue) {
    // IDA 0x65b334 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // readValue`): xsi:nil early-out, string pair with fallthrough,
    // raw int set, else `ReleaseAssert(false)` (Surface.cpp line
    // 313) — same as the face-2 twin at 0x65a120 (decompile same
    // length 5112).
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            part.faces[5].surface_type = *value as u32;
        }
        XmlReadValue::Text(text) => {
            if stub_065b2b0(part, text) {
                return;
            }
            if flog_asserts() {
                panic!("false file: Client/App/v8datamodel/Surface.cpp line: 313 (IDA 0x65b334)");
            }
        }
        XmlReadValue::Other => {
            if flog_asserts() {
                panic!("false file: Client/App/v8datamodel/Surface.cpp line: 313 (IDA 0x65b334)");
            }
        }
    }
}

// 0x065b58c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065b58c(part: &PartSurfaceData) -> i32 {
    // IDA 0x65b58c (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // getIndexValue`, disasm singleton once + inner `getValue` +
    // tail-call `EnumDesc::convertToIndex`): the item index of the
    // live value (host: stub_065a544). Same as the face-2 twin at
    // 0x65a378.
    stub_065a544(part.faces[5].surface_type as i32)
}

// 0x065b5d4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
pub fn stub_065b5d4(part: &mut PartSurfaceData, index: u32) -> bool {
    // IDA 0x65b5d4 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // setIndexValue`): `count > index` gates storing the indexed
    // item's value, same as the face-2 twin at 0x65a3c0. Host:
    // table read decides.
    match SURFACE_TYPE_ITEMS.get(index as usize) {
        Some((_, value)) => {
            part.faces[5].surface_type = *value;
            true
        }
        None => false,
    }
}

// 0x065b630 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065b630(part: &PartSurfaceData) -> u32 {
    // IDA 0x65b630 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // getEnumValue`): inner `getValue` through the +44 `GetSet`
    // (host: stub_065b760). Host: the face-5 slot.
    part.faces[5].surface_type
}

// 0x065b638 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
pub fn stub_065b638(part: &mut PartSurfaceData, value: u32) -> bool {
    // IDA 0x65b638 (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // setEnumValue`): `find_if` membership + conditional inner set,
    // same as the face-2 twin at 0x65a424. Host: table membership
    // decides.
    if SURFACE_TYPE_ITEMS.iter().any(|(_, v)| *v == value) {
        part.faces[5].surface_type = value;
        true
    } else {
        false
    }
}

// 0x065b6ac — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065b6ac(part: &PartSurfaceData) -> Option<u32> {
    // IDA 0x65b6ac (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // getEnumItem`, disasm singleton once + inner `getValue` +
    // `convertToItem`): the table entry for the live value. Same
    // as the face-2 twin at 0x65a498.
    SURFACE_TYPE_ITEMS
        .iter()
        .position(|(_, v)| *v == part.faces[5].surface_type)
        .map(|i| i as u32)
}

// 0x065b6fc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
pub fn stub_065b6fc(part: &mut PartSurfaceData, name: &str) -> bool {
    // IDA 0x65b6fc (`SurfaceEnumPropDescriptor<5, SurfaceType>::
    // setStringValue(Name)`): convert + conditional inner set, same
    // as the face-2 twin at 0x65a4e8. Host: the string twin in
    // this file.
    stub_065b2b0(part, name)
}

// 0x065b758 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv")]
pub fn stub_065b758() -> bool {
    // IDA 0x65b758 (`SurfaceGetSet<5, SurfaceType>::isReadOnly`):
    // returns 0 — always readable.
    false
}

// 0x065b75c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv")]
pub fn stub_065b75c() -> bool {
    // IDA 0x65b75c (`SurfaceGetSet<5, SurfaceType>::isWriteOnly`):
    // returns 0 — always writable.
    false
}

// 0x065b760 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065b760(part: &PartSurfaceData) -> u32 {
    // IDA 0x65b760 (`SurfaceGetSet<5, SurfaceType>::getValue`): the
    // member-pointer resolve tail-calling the getter with the face
    // baked in (`, 5)`, verified in the decompile). Same shape as
    // the face-2 twin at 0x65a5bc.
    part.faces[5].surface_type
}

// 0x065b780 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)5,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_")]
pub fn stub_065b780(part: &mut PartSurfaceData, value: u32) {
    // IDA 0x65b780 (`SurfaceGetSet<5, SurfaceType>::setValue`): the
    // member-pointer resolve tail-calling the setter with
    // `(instance, 5, value)`. Same shape as the face-2 twin at
    // 0x65a5dc.
    part.faces[5].surface_type = value;
}

// 0x065b7a4 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE0EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)0,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)0,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE0EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE")]
pub fn stub_065b7a4(
    name: &str,
    category: &str,
    slot: FaceFloatSlot,
    functionality: u32,
    permissions: u32,
) -> FaceFloatProp {
    // IDA 0x65b7a4 (`SurfacePropDescriptor<0, float>` ctor): same
    // `classDescriptor` + impl + base-init shape as the face-2 twin
    // at 0x65937c (decompile same length 2306), with the face baked
    // in as 0. Host: the cutover with face 0.
    FaceFloatProp::new(0, slot, name, category, functionality, permissions)
}

// 0x065b8b8 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE0EfED0Ev
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)0,float>::~SurfacePropDescriptor()
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)0,float>::~SurfacePropDescriptor()")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE0EfED0Ev")]
pub fn stub_065b8b8() {
    // IDA 0x065b8b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065b8e4 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)0,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)0,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv")]
pub fn stub_065b8e4() -> bool {
    // IDA 0x65b8e4 (`SurfaceGetSet<0, float>::isReadOnly`):
    // returns 0 — always readable.
    false
}

// 0x065b8e8 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)0,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)0,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv")]
pub fn stub_065b8e8() -> bool {
    // IDA 0x65b8e8 (`SurfaceGetSet<0, float>::isWriteOnly`):
    // returns 0 — always writable.
    false
}

// 0x065b8ec — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)0,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)0,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065b8ec(part: &PartSurfaceData, slot: FaceFloatSlot) -> f32 {
    // IDA 0x65b8ec (`SurfaceGetSet<0, float>::getValue`): the
    // member-pointer resolve tail-calling the getter with the face
    // baked in (`, 0)`, verified in the decompile). Same shape as
    // the face-2 twin at 0x6594c4.
    part.faces[0].float_slot(slot)
}

// 0x065b90c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)0,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)0,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf")]
pub fn stub_065b90c(part: &mut PartSurfaceData, slot: FaceFloatSlot, value: f32) {
    // IDA 0x65b90c (`SurfaceGetSet<0, float>::setValue`): the
    // member-pointer resolve tail-calling the setter with
    // `(instance, 0, value)`. Same shape as the face-2 twin at
    // 0x6594e4.
    part.faces[0].set_float_slot(slot, value);
}

// 0x065b930 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
pub fn stub_065b930(name: &str, category: &str, functionality: u32) -> FaceInputProp {
    // IDA 0x65b930 (`SurfaceEnumPropDescriptor<0, InputType>`
    // ctor): same `classDescriptor` + `EnumDesc` singleton +
    // base-init + impl shape as the face-2 twin at 0x659508
    // (decompile same length 2675), with the face baked in as 0.
    // Host: the cutover with face 0.
    FaceInputProp::new(0, name, category, functionality)
}

// 0x065b9dc — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEED0Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEED0Ev")]
pub fn stub_065b9dc() {
    // IDA 0x065b9dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065ba08 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE10isReadOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE10isReadOnlyEv")]
pub fn stub_065ba08() -> bool {
    // IDA 0x65ba08 (`SurfaceEnumPropDescriptor<0, InputType>::
    // isReadOnly`): delegates to the inner face-0 `GetSet` at +44 —
    // always readable (same shape as the face-2 twin at 0x6595e0).
    false
}

// 0x065ba18 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE11isWriteOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")]
pub fn stub_065ba18() -> bool {
    // IDA 0x65ba18 (`SurfaceEnumPropDescriptor<0, InputType>::
    // isWriteOnly`): delegates to the inner face-0 `GetSet` at +44 —
    // always writable (same shape as the face-2 twin at 0x6595f0).
    false
}

// 0x065ba28 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")]
pub fn stub_065ba28(first: &PartSurfaceData, second: &PartSurfaceData) -> bool {
    // IDA 0x65ba28 (`SurfaceEnumPropDescriptor<0, InputType>::
    // equalValues`): reads the inner value for both instances via
    // the +44 `GetSet` and compares. Host: compare the face-0
    // input slots.
    first.faces[0].surface_input == second.faces[0].surface_input
}

// 0x065ba50 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")]
pub fn stub_065ba50(part: &PartSurfaceData) -> SurfaceVariant {
    // IDA 0x65ba50 (`SurfaceEnumPropDescriptor<0, InputType>::
    // getVariant`): reads the inner value, tags it with the
    // `InputType` singleton and placement-moves it in. Same as the
    // face-2 twin at 0x659628.
    SurfaceVariant::SurfaceInput(part.faces[0].surface_input)
}

// 0x065ba78 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")]
pub fn stub_065ba78() -> ! {
    todo!("0x065ba78 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")
}

// 0x065bbd0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")]
pub fn stub_065bbd0() -> ! {
    todo!("0x065bbd0 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")
}

// 0x065bbf4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14hasStringValueEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::hasStringValue(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14hasStringValueEv")]
pub fn stub_065bbf4() -> ! {
    todo!("0x065bbf4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14hasStringValueEv")
}

// 0x065bbf8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065bbf8() -> ! {
    todo!("0x065bbf8 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065bc48 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
pub fn stub_065bc48() -> ! {
    todo!("0x065bc48 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")
}

// 0x065bcac — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
pub fn stub_065bcac() -> ! {
    todo!("0x065bcac __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")
}

// 0x065bccc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_065bccc() -> ! {
    todo!("0x065bccc __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x065bf24 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065bf24() -> ! {
    todo!("0x065bf24 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065bf6c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
pub fn stub_065bf6c() -> ! {
    todo!("0x065bf6c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")
}

// 0x065bfc8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065bfc8() -> ! {
    todo!("0x065bfc8 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065bfd0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
pub fn stub_065bfd0() -> ! {
    todo!("0x065bfd0 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")
}

// 0x065c044 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065c044() -> ! {
    todo!("0x065c044 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")
}

// 0x065c094 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
pub fn stub_065c094() -> ! {
    todo!("0x065c094 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")
}

// 0x065c0f0 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv")]
pub fn stub_065c0f0() -> ! {
    todo!("0x065c0f0 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv")
}

// 0x065c0f4 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv")]
pub fn stub_065c0f4() -> ! {
    todo!("0x065c0f4 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv")
}

// 0x065c0f8 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065c0f8() -> ! {
    todo!("0x065c0f8 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065c118 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_")]
pub fn stub_065c118() -> ! {
    todo!("0x065c118 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_")
}

// 0x065c13c — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
pub fn stub_065c13c() -> ! {
    todo!("0x065c13c __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")
}

// 0x065c1e8 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEED0Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEED0Ev")]
pub fn stub_065c1e8() {
    // IDA 0x065c1e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065c214 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10isReadOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10isReadOnlyEv")]
pub fn stub_065c214() -> ! {
    todo!("0x065c214 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10isReadOnlyEv")
}

// 0x065c224 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE11isWriteOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE11isWriteOnlyEv")]
pub fn stub_065c224() -> ! {
    todo!("0x065c224 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE11isWriteOnlyEv")
}

// 0x065c234 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_")]
pub fn stub_065c234() -> ! {
    todo!("0x065c234 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_")
}

// 0x065c25c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE")]
pub fn stub_065c25c() -> ! {
    todo!("0x065c25c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE")
}

// 0x065c284 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE")]
pub fn stub_065c284() -> ! {
    todo!("0x065c284 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE")
}

// 0x065c3dc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_")]
pub fn stub_065c3dc() -> ! {
    todo!("0x065c3dc __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_")
}

// 0x065c400 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14hasStringValueEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::hasStringValue(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14hasStringValueEv")]
pub fn stub_065c400() -> ! {
    todo!("0x065c400 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14hasStringValueEv")
}

// 0x065c404 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065c404() -> ! {
    todo!("0x065c404 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065c454 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
pub fn stub_065c454() -> ! {
    todo!("0x065c454 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")
}

// 0x065c4b8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
pub fn stub_065c4b8() -> ! {
    todo!("0x065c4b8 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")
}

// 0x065c4d8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_065c4d8() -> ! {
    todo!("0x065c4d8 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x065c730 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065c730() -> ! {
    todo!("0x065c730 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065c778 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
pub fn stub_065c778() -> ! {
    todo!("0x065c778 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")
}

// 0x065c7d4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065c7d4() -> ! {
    todo!("0x065c7d4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065c7dc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
pub fn stub_065c7dc() -> ! {
    todo!("0x065c7dc __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")
}

// 0x065c850 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065c850() -> ! {
    todo!("0x065c850 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")
}

// 0x065c8a0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
pub fn stub_065c8a0() -> ! {
    todo!("0x065c8a0 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")
}

// 0x065c8fc — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv")]
pub fn stub_065c8fc() -> ! {
    todo!("0x065c8fc __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv")
}

// 0x065c900 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv")]
pub fn stub_065c900() -> ! {
    todo!("0x065c900 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv")
}

// 0x065c904 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065c904() -> ! {
    todo!("0x065c904 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065c924 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)0,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_")]
pub fn stub_065c924() -> ! {
    todo!("0x065c924 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE0ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_")
}

// 0x065c948 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE3EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)3,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)3,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE3EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE")]
pub fn stub_065c948() -> ! {
    todo!("0x065c948 __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE3EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE")
}

// 0x065ca5c — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE3EfED0Ev
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)3,float>::~SurfacePropDescriptor()
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)3,float>::~SurfacePropDescriptor()")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE3EfED0Ev")]
pub fn stub_065ca5c() {
    // IDA 0x065ca5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065ca88 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)3,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)3,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv")]
pub fn stub_065ca88() -> ! {
    todo!("0x065ca88 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv")
}

// 0x065ca8c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)3,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)3,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv")]
pub fn stub_065ca8c() -> ! {
    todo!("0x065ca8c __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv")
}

// 0x065ca90 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)3,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)3,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065ca90() -> ! {
    todo!("0x065ca90 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x065cab0 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)3,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)3,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf")]
pub fn stub_065cab0() -> ! {
    todo!("0x065cab0 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE3EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf")
}

// 0x065cad4 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
pub fn stub_065cad4() -> ! {
    todo!("0x065cad4 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")
}

// 0x065cb80 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEED0Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEED0Ev")]
pub fn stub_065cb80() {
    // IDA 0x065cb80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
