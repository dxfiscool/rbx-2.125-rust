// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Global gap filler EA-sorted asc — next 120 uncovered funcs not in any crates/*/src (fallback: dm gaps EA-sorted asc, global gaps 0)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x1335c..0x16548 | total 85545, dm distinct before 18606, after 18726, global missing before 0, after 0 (fallback 120 dm gaps, 66939->66819 not in datamodel)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_189::{CRenderSettingsItem, RenderEnumDesc, stub_0x86d0};
use crate::generated_190::{IntVariant, RenderXmlInput, RenderXmlIntValue};
use crate::generated_191::{RenderFrameRateEnumPropDesc, frame_rate_manager_mode_enum_desc, shadow_mode_enum_desc};
use std::collections::BTreeMap;
use std::sync::LazyLock;

// 0x13a30/0x13be4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEEC2/D0Ev
/// Rust model of `EnumPropDescriptor<CRenderSettingsItem, GraphicsMode>`
/// (IDA `0x13a30` C2 / `0x13be4` D0): the shared `EnumDesc<GraphicsMode>`
/// table (via `graphics_mode_enum_desc()`, built by 0x86d0), the bound
/// getter/setter pair in the heap impl, and the read/write-only flags (both
/// queries return 0 per `0x13c10`/`0x13c20`, so the bits stay set). Same
/// shape as `RenderAAEnumPropDesc` (0x10a08).
pub struct RenderGraphicsEnumPropDesc {
    pub name: &'static str,
    pub category: &'static str,
    pub getter: fn(&CRenderSettingsItem) -> i32,
    pub setter: fn(&mut CRenderSettingsItem, i32),
    pub read_only: bool,
    pub write_only: bool,
}

/// The `EnumDesc<GraphicsMode>` singleton behind the graphics descriptor
/// suite; built once by the 0x86d0 constructor.
static GRAPHICS_MODE_ENUM_DESC: LazyLock<RenderEnumDesc> = LazyLock::new(stub_0x86d0);
/// Singleton accessor for the graphics enum table.
pub fn graphics_mode_enum_desc() -> &'static RenderEnumDesc {
    LazyLock::force(&GRAPHICS_MODE_ENUM_DESC)
}
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x1335c — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
pub fn stub_1335c(_desc: *mut RenderFrameRateEnumPropDesc) {
    // IDA 0x1335c..0x1337a (decompiled): `EnumPropDescriptor<...>::D0` —
    // vtable install (0x13370), impl `delete` on the `+44` slot
    // (0x13372..0x13378), `operator delete`. Same drop-glue shape as
    // 0x10038.
}

// 0x13388 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isReadOnly(void)const")]
pub fn stub_13388(desc: &RenderFrameRateEnumPropDesc) -> bool {
    // IDA 0x13388..0x13394 (decompiled): `isReadOnly` delegates to the
    // `+44` impl slot `+0` query, which returns `0` (cf. 0x10064).
    desc.read_only
}

// 0x13398 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isWriteOnly(void)const")]
pub fn stub_13398(desc: &RenderFrameRateEnumPropDesc) -> bool {
    // IDA 0x13398..0x133a4 (decompiled): `isWriteOnly` delegates to the
    // `+44` impl slot `+4` query, which returns `0` (cf. 0x10074).
    desc.write_only
}

// 0x133a8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_133a8(
    desc: &RenderFrameRateEnumPropDesc,
    first: &CRenderSettingsItem,
    second: &CRenderSettingsItem,
) -> bool {
    // IDA 0x133a8..0x133ce (decompiled): `equalValues` — `getValue` through
    // the `+44` slot `+8` on both sides (0x133b8/0x133ce) and compare. Same
    // shape as 0x10084.
    (desc.getter)(first) == (desc.getter)(second)
}

// 0x133d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_133d0(desc: &RenderFrameRateEnumPropDesc, item: &CRenderSettingsItem) -> IntVariant {
    // IDA 0x133d0..0x133f2 (decompiled): `getVariant` — `getEnumValue`
    // through vtable `+68` (0x133de), `Type::getSingleton<int>` tag
    // (0x133e4), `placement_any<int>::operator=` (0x133f2). Same shape as
    // 0x100ac.
    IntVariant { value: (desc.getter)(item) }
}

// 0x133f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_133f4(desc: &RenderFrameRateEnumPropDesc, item: &mut CRenderSettingsItem, variant: &IntVariant) {
    // IDA 0x133f4 (decompiled): `setVariant` — same holder-identity int fast
    // path plus generic `Variant::convert<int>` fallback as 0x100d0, then
    // the `+72` setter. Our variant only holds ints, so both paths collapse
    // into the stored setter fn.
    (desc.setter)(item, variant.value)
}

// 0x13544 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_13544(desc: &RenderFrameRateEnumPropDesc, dst: &mut CRenderSettingsItem, src: &CRenderSettingsItem) {
    // IDA 0x13544..0x13566 (decompiled): `copyValue` — `getValue` through
    // the `+44` slot `+8` into a spill (0x13556), then the `+12` setter
    // (0x13566). Same shape as 0x10220.
    let value = (desc.getter)(src);
    (desc.setter)(dst, value)
}

// 0x13568 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::hasStringValue(void)const")]
pub fn stub_13568() -> bool {
    // IDA 0x13568..0x1356a (decompiled): `hasStringValue` returns `1`. Same
    // shape as 0x10244.
    true
}

// 0x1356c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1356c(desc: &RenderFrameRateEnumPropDesc, item: &CRenderSettingsItem, out: &mut String) {
    // IDA 0x1356c..0x1358e (decompiled): `getStringValue` — `getValue`
    // through the `+44` slot `+8` (0x13576), then
    // `EnumDesc<FrameRateManagerMode>::convertToString`: empty when out of
    // range. Same shape as 0x10248.
    let value = (desc.getter)(item);
    match (value >= 0).then(|| frame_rate_manager_mode_enum_desc().lookup_name(value)).flatten() {
        Some(name) => *out = name.to_owned(),
        None => out.clear(),
    }
}

// 0x13590 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_13590(desc: &RenderFrameRateEnumPropDesc, item: &mut CRenderSettingsItem, name: &str) -> bool {
    // IDA 0x13590..0x135c8 (decompiled): `setStringValue` — `Name::lookup`
    // (0x1359e), `EnumDesc<FrameRateManagerMode>::convertToValue`, miss
    // returns 0, hit sets through the `+44` slot `+12` and returns 1. Same
    // shape as 0x1026c.
    if let Some(value) = frame_rate_manager_mode_enum_desc().lookup_value(name) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x135d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_135d0(desc: &RenderFrameRateEnumPropDesc, item: &CRenderSettingsItem) -> RenderXmlIntValue {
    // IDA 0x135d0..0x135ee (decompiled): `writeValue` — `getValue` through
    // the `+44` slot `+8` (0x135de), `clearValue` (0x135e4), type word `5`
    // (0x135ea), int word (0x135ec), return `5` (0x135ee). Same shape as
    // 0x102ac.
    RenderXmlIntValue { kind: 5, int_value: (desc.getter)(item) }
}

// 0x135f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_135f0(desc: &RenderFrameRateEnumPropDesc, item: &mut CRenderSettingsItem, input: &RenderXmlInput) {
    // IDA 0x135f0 (decompiled): `readValue` — same `isXsiNil` bail / int
    // (`setIntValue`) / string (`Name::lookup` + `convertToValue` + set with
    // `setStringValue`-mismatch fallback) / `ReleaseAssert(false)` shape as
    // 0x102cc. `setIntValue` for this desc is the direct member set (cf.
    // 0x13998 shape).
    match input {
        RenderXmlInput::Nil => {}
        RenderXmlInput::Int(value) => (desc.setter)(item, *value),
        RenderXmlInput::Text(name) => {
            if !stub_13590(desc, item, name) {
                debug_assert!(false, "0x135f0: false (Reflection.h:359)");
            }
        }
    }
}

// 0x13830 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13830(desc: &RenderFrameRateEnumPropDesc, item: &CRenderSettingsItem) -> Option<usize> {
    // IDA 0x13830..0x1384a (decompiled): `getIndexValue` — `getValue`
    // through the `+44` slot `+8` (0x13840), then
    // `EnumDesc<FrameRateManagerMode>::convertToIndex` (0x13842): assert
    // plus position search. Same shape as 0x1050c.
    let value = (desc.getter)(item);
    debug_assert!(value >= 0, "0x13830: value>=0");
    frame_rate_manager_mode_enum_desc().pairs.iter().position(|(v, _)| *v == value)
}

// 0x1384c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_1384c(desc: &RenderFrameRateEnumPropDesc, item: &mut CRenderSettingsItem, index: usize) -> bool {
    // IDA 0x1384c..0x13874 (decompiled): `setIndexValue` — `count > index`
    // check against the enum count at `+40` (0x1385e), indexed value load
    // from the value table at `+144` (0x13868), `+44` slot `+12` set,
    // return 1; miss returns 0. The `+144` table holds the values in
    // registration order, so `pairs` stands in. Same shape as 0x110ac.
    match frame_rate_manager_mode_enum_desc().pairs.get(index) {
        Some((value, _)) => {
            (desc.setter)(item, *value);
            true
        }
        None => false,
    }
}

// 0x13880 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13880(desc: &RenderFrameRateEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x13880..0x13886 (decompiled): `getEnumValue` — `getValue`
    // through the `+44` slot `+8`. Same delegation as 0x1055c without the
    // variant wrap.
    (desc.getter)(item)
}

// 0x13888 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13888(desc: &RenderFrameRateEnumPropDesc, item: &mut CRenderSettingsItem, value: i32) -> bool {
    // IDA 0x13888..0x138d0 (decompiled): `setEnumValue` — `find_if` with
    // `equalValue` over the enum items (0x13896), miss returns 0, hit sets
    // through the `+44` slot `+12` and returns 1. Same shape as 0x10564.
    if frame_rate_manager_mode_enum_desc().pairs.iter().any(|(v, _)| *v == value) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x138d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_138d4(desc: &RenderFrameRateEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x138d4..0x138f2 (decompiled): `getEnumItem` — `getValue` through
    // the `+44` slot `+8` (0x138e6), then
    // `EnumDesc<FrameRateManagerMode>::convertToItem` (0x138f2), which is
    // the identity-table body (cf. 0xd4f8). Same shape as 0x105b0.
    let value = (desc.getter)(item);
    let table = frame_rate_manager_mode_enum_desc();
    if value >= 0 && (value as usize) < table.pairs.len() {
        value
    } else {
        0
    }
}

// 0x138f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_138f4(desc: &RenderFrameRateEnumPropDesc, item: &mut CRenderSettingsItem, name: &str) -> bool {
    // IDA 0x138f4..0x13922 (decompiled): `setStringValue` on the `Name` —
    // `EnumDesc<FrameRateManagerMode>::convertToValue` (0x1390a), miss
    // returns 0 (0x1390c), hit sets through the `+44` slot `+12` and returns
    // 1. `Name::c_str` collapses into the `&str` itself; same shape as
    // 0x105d0.
    if let Some(value) = frame_rate_manager_mode_enum_desc().lookup_value(name) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x13928 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToIndexES3_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToIndex(RBX::CRenderSettings::FrameRateManagerMode)const")]
pub fn stub_13928(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0x13928 (decompiled):
    // `EnumDesc<FrameRateManagerMode>::convertToIndex` — same
    // `ReleaseAssert(value >= 0)` + `value < table ? table[value] : -1`
    // shape as 0x10604 over the value→index remap. The remap is identity
    // over the registered pairs here, so the position search stands in.
    debug_assert!(value >= 0, "0x13928: value>=0");
    desc.pairs.iter().position(|(v, _)| *v == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x13998 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13998(desc: &RenderFrameRateEnumPropDesc, item: &mut CRenderSettingsItem, index: i32) -> bool {
    // IDA 0x13998..0x139d4 (decompiled): `setIntValue` — `index >= 0` gate
    // (0x139a2), bounds check against the value table at `+132`
    // (0x139a6..0x139b4), `-1`-hole check, `+44` slot `+12` set, return 1;
    // miss returns 0. Same shape as 0x10674.
    if index >= 0 {
        if let Some((value, _)) = frame_rate_manager_mode_enum_desc().pairs.get(index as usize) {
            if *value != -1 {
                (desc.setter)(item, *value);
                return true;
            }
        }
    }
    false
}

// 0x139d8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isReadOnly(void)const")]
pub fn stub_139d8() -> bool {
    // IDA 0x139d8..0x139da (decompiled):
    // `GetSetImpl<FrameRateManagerMode (CRenderSettings::*)() const, void
    // (CRenderSettingsItem::*)(FrameRateManagerMode)>::isReadOnly` returns
    // `0`. Same shape as 0x106b4.
    false
}

// 0x139dc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isWriteOnly(void)const")]
pub fn stub_139dc() -> bool {
    // IDA 0x139dc..0x139de (decompiled): the FrameRateManagerMode-member
    // `GetSetImpl` `isWriteOnly` twin — returns `0`. Same shape as 0x106b8.
    false
}

// 0x139e0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_139e0(desc: &RenderFrameRateEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x139e0..0x13a0a (decompiled): `GetSetImpl<...>::getValue` —
    // resolves the bound `FrameRateManagerMode (CRenderSettings::*)() const`
    // member through the `+4` slot (0x139e2..0x13a08) and invokes it. The
    // member-pointer dance collapses into the stored getter fn; same shape
    // as 0x106bc.
    (desc.getter)(item)
}

// 0x13a0c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_13a0c(desc: &RenderFrameRateEnumPropDesc, item: &mut CRenderSettingsItem, value: i32) {
    // IDA 0x13a0c..0x13a2c (decompiled): `GetSetImpl<...>::setValue` —
    // resolves the bound `void
    // (CRenderSettingsItem::*)(FrameRateManagerMode)` member through the
    // `+12` slot and invokes it with `*a3`. Collapses into the stored
    // setter fn; same shape as 0x106e8.
    (desc.setter)(item, value)
}

// 0x13a30 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::EnumPropDescriptor<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>(char const*,char const*,RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_13a30(
    name: &'static str,
    category: &'static str,
    getter: fn(&CRenderSettingsItem) -> i32,
    setter: fn(&mut CRenderSettingsItem, i32),
) -> RenderGraphicsEnumPropDesc {
    // IDA 0x13a30 (decompiled prologue through the `classDescriptor()` touch,
    // same call shape as 0xfe84..0xfea8):
    // `EnumPropDescriptor<CRenderSettingsItem, GraphicsMode>::C2` — same
    // construction shape as the ResolutionPreset twin (0xfe84):
    // `EnumDesc<GraphicsMode>` singleton touch, `PropertyDescriptor` C2,
    // enum-table stores, GetSetImpl alloc with the member pair, vtable
    // install. The member pointers collapse into the getter/setter fns.
    let _ = graphics_mode_enum_desc();
    RenderGraphicsEnumPropDesc { name, category, getter, setter, read_only: false, write_only: false }
}

// 0x13be4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
pub fn stub_13be4(_desc: *mut RenderGraphicsEnumPropDesc) {
    // IDA 0x13be4..0x13c06 (decompiled): `EnumPropDescriptor<...>::D0` —
    // vtable install (0x13bf8), impl `delete` on the `+44` slot
    // (0x13bfa..0x13c00), `operator delete`. Same drop-glue shape as
    // 0x10038.
}

// 0x13c10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isReadOnly(void)const")]
pub fn stub_13c10(desc: &RenderGraphicsEnumPropDesc) -> bool {
    // IDA 0x13c10..0x13c1c (decompiled): `isReadOnly` delegates to the
    // `+44` impl slot `+0` query, which returns `0` (cf. 0x10064).
    desc.read_only
}

// 0x13c20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isWriteOnly(void)const")]
pub fn stub_13c20(desc: &RenderGraphicsEnumPropDesc) -> bool {
    // IDA 0x13c20..0x13c2c (decompiled): `isWriteOnly` delegates to the
    // `+44` impl slot `+4` query, which returns `0` (cf. 0x10074).
    desc.write_only
}

// 0x13c30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13c30(
    desc: &RenderGraphicsEnumPropDesc,
    first: &CRenderSettingsItem,
    second: &CRenderSettingsItem,
) -> bool {
    // IDA 0x13c30..0x13c56 (decompiled): `equalValues` — `getValue` through
    // the `+44` slot `+8` on both sides (0x13c40/0x13c56) and compare. Same
    // shape as 0x10084.
    (desc.getter)(first) == (desc.getter)(second)
}

// 0x13c58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_13c58(desc: &RenderGraphicsEnumPropDesc, item: &CRenderSettingsItem) -> IntVariant {
    // IDA 0x13c58..0x13c7a (decompiled): `getVariant` — `getEnumValue`
    // through vtable `+68` (0x13c66), `Type::getSingleton<int>` tag
    // (0x13c6c), `placement_any<int>::operator=` (0x13c7a). Same shape as
    // 0x100ac.
    IntVariant { value: (desc.getter)(item) }
}

// 0x13c7c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_13c7c(desc: &RenderGraphicsEnumPropDesc, item: &mut CRenderSettingsItem, variant: &IntVariant) {
    // IDA 0x13c7c (decompiled): `setVariant` — same holder-identity int fast
    // path plus generic `Variant::convert<int>` fallback as 0x100d0, then
    // the `+72` setter. Our variant only holds ints, so both paths collapse
    // into the stored setter fn.
    (desc.setter)(item, variant.value)
}

// 0x13dcc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_13dcc(desc: &RenderGraphicsEnumPropDesc, dst: &mut CRenderSettingsItem, src: &CRenderSettingsItem) {
    // IDA 0x13dcc..0x13dee (decompiled): `copyValue` — `getValue` through
    // the `+44` slot `+8` into a spill (0x13dde), then the `+12` setter.
    // Same shape as 0x10220.
    let value = (desc.getter)(src);
    (desc.setter)(dst, value)
}

// 0x13df0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::hasStringValue(void)const")]
pub fn stub_13df0() -> bool {
    // IDA 0x13df0..0x13df2 (decompiled): `hasStringValue` returns `1`. Same
    // shape as 0x10244.
    true
}

// 0x13df4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13df4(desc: &RenderGraphicsEnumPropDesc, item: &CRenderSettingsItem, out: &mut String) {
    // IDA 0x13df4..0x13e16 (decompiled): `getStringValue` — `getValue`
    // through the `+44` slot `+8` (0x13dfe), then
    // `EnumDesc<GraphicsMode>::convertToString`: empty when out of range.
    // Same shape as 0x10248.
    let value = (desc.getter)(item);
    match (value >= 0).then(|| graphics_mode_enum_desc().lookup_name(value)).flatten() {
        Some(name) => *out = name.to_owned(),
        None => out.clear(),
    }
}

// 0x13e18 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_13e18(desc: &RenderGraphicsEnumPropDesc, item: &mut CRenderSettingsItem, name: &str) -> bool {
    // IDA 0x13e18..0x13e50 (decompiled): `setStringValue` — `Name::lookup`
    // (0x13e22), `EnumDesc<GraphicsMode>::convertToValue`, miss returns 0,
    // hit sets through the `+44` slot `+12` and returns 1. Same shape as
    // 0x1026c.
    if let Some(value) = graphics_mode_enum_desc().lookup_value(name) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x13e58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_13e58(desc: &RenderGraphicsEnumPropDesc, item: &CRenderSettingsItem) -> RenderXmlIntValue {
    // IDA 0x13e58..0x13e76 (decompiled): `writeValue` — `getValue` through
    // the `+44` slot `+8` (0x13e66), `clearValue` (0x13e6c), type word `5`
    // (0x13e72), int word (0x13e74), return `5`. Same shape as 0x102ac.
    RenderXmlIntValue { kind: 5, int_value: (desc.getter)(item) }
}

// 0x13e78 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_13e78(desc: &RenderGraphicsEnumPropDesc, item: &mut CRenderSettingsItem, input: &RenderXmlInput) {
    // IDA 0x13e78 (decompiled): `readValue` — same `isXsiNil` bail / int
    // (`setIntValue`) / string (`Name::lookup` + `convertToValue` + set with
    // `setStringValue`-mismatch fallback) / `ReleaseAssert(false)` shape as
    // 0x102cc. `setIntValue` for this desc is the direct member set (cf.
    // 0x14220 shape).
    match input {
        RenderXmlInput::Nil => {}
        RenderXmlInput::Int(value) => (desc.setter)(item, *value),
        RenderXmlInput::Text(name) => {
            if !stub_13e18(desc, item, name) {
                debug_assert!(false, "0x13e78: false (Reflection.h:359)");
            }
        }
    }
}

// 0x140b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_140b8(desc: &RenderGraphicsEnumPropDesc, item: &CRenderSettingsItem) -> Option<usize> {
    // IDA 0x140b8..0x140d2 (decompiled): `getIndexValue` — `getValue`
    // through the `+44` slot `+8` (0x140c8), then
    // `EnumDesc<GraphicsMode>::convertToIndex` (0x140ca): assert plus
    // position search. Same shape as 0x1050c.
    let value = (desc.getter)(item);
    debug_assert!(value >= 0, "0x140b8: value>=0");
    graphics_mode_enum_desc().pairs.iter().position(|(v, _)| *v == value)
}

// 0x140d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_140d4(desc: &RenderGraphicsEnumPropDesc, item: &mut CRenderSettingsItem, index: usize) -> bool {
    // IDA 0x140d4..0x14106 (decompiled): `setIndexValue` — `count > index`
    // check against the enum count at `+40` (0x140e6), indexed value load
    // from the value table at `+144`, `+44` slot `+12` set, return 1; miss
    // returns 0. The `+144` table holds the values in registration order, so
    // `pairs` stands in. Same shape as 0x110ac.
    match graphics_mode_enum_desc().pairs.get(index) {
        Some((value, _)) => {
            (desc.setter)(item, *value);
            true
        }
        None => false,
    }
}

// 0x14108 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_14108(desc: &RenderGraphicsEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x14108..0x1410e (decompiled): `getEnumValue` — `getValue`
    // through the `+44` slot `+8`. Same delegation as 0x1055c without the
    // variant wrap.
    (desc.getter)(item)
}

// 0x14110 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_14110(desc: &RenderGraphicsEnumPropDesc, item: &mut CRenderSettingsItem, value: i32) -> bool {
    // IDA 0x14110..0x14158 (decompiled): `setEnumValue` — `find_if` with
    // `equalValue` over the enum items (0x1411e), miss returns 0, hit sets
    // through the `+44` slot `+12` and returns 1. Same shape as 0x10564.
    if graphics_mode_enum_desc().pairs.iter().any(|(v, _)| *v == value) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x1415c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1415c(desc: &RenderGraphicsEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x1415c..0x1417a (decompiled): `getEnumItem` — `getValue` through
    // the `+44` slot `+8` (0x1416e), then
    // `EnumDesc<GraphicsMode>::convertToItem` (0x14178), which is the
    // identity-table body (cf. 0xda38). Same shape as 0x105b0.
    let value = (desc.getter)(item);
    let table = graphics_mode_enum_desc();
    if value >= 0 && (value as usize) < table.pairs.len() {
        value
    } else {
        0
    }
}

// 0x1417c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_1417c(desc: &RenderGraphicsEnumPropDesc, item: &mut CRenderSettingsItem, name: &str) -> bool {
    // IDA 0x1417c..0x141aa (decompiled): `setStringValue` on the `Name` —
    // `EnumDesc<GraphicsMode>::convertToValue` (0x14192), miss returns 0,
    // hit sets through the `+44` slot `+12` and returns 1. `Name::c_str`
    // collapses into the `&str` itself; same shape as 0x105d0.
    if let Some(value) = graphics_mode_enum_desc().lookup_value(name) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x141b0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToIndexES3_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToIndex(RBX::CRenderSettings::GraphicsMode)const")]
pub fn stub_141b0(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0x141b0 (decompiled): `EnumDesc<GraphicsMode>::convertToIndex` —
    // same `ReleaseAssert(value >= 0)` + `value < table ? table[value] : -1`
    // shape as 0x10604 over the value→index remap. The remap is identity
    // over the registered pairs here, so the position search stands in.
    debug_assert!(value >= 0, "0x141b0: value>=0");
    desc.pairs.iter().position(|(v, _)| *v == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x14220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_14220(desc: &RenderGraphicsEnumPropDesc, item: &mut CRenderSettingsItem, index: i32) -> bool {
    // IDA 0x14220 (decompiled): `setIntValue` — same `index >= 0` gate /
    // bounds check / `-1`-hole check / `+44` slot `+12` set / return-1 shape
    // as 0x10674 over the GraphicsMode value table.
    if index >= 0 {
        if let Some((value, _)) = graphics_mode_enum_desc().pairs.get(index as usize) {
            if *value != -1 {
                (desc.setter)(item, *value);
                return true;
            }
        }
    }
    false
}

// 0x14260 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isReadOnly(void)const")]
pub fn stub_14260() -> bool {
    // IDA 0x14260..0x14262 (decompiled):
    // `GetSetImpl<GraphicsMode (CRenderSettings::*)() const, void
    // (CRenderSettingsItem::*)(GraphicsMode)>::isReadOnly` returns `0`.
    // Same shape as 0x106b4.
    false
}

// 0x14264 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isWriteOnly(void)const")]
pub fn stub_14264() -> bool {
    // IDA 0x14264..0x14266 (decompiled): the GraphicsMode-member `GetSetImpl`
    // `isWriteOnly` twin — returns `0`. Same shape as 0x106b8.
    false
}

// 0x14268 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_14268(desc: &RenderGraphicsEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x14268..0x14292 (decompiled): `GetSetImpl<...>::getValue` —
    // resolves the bound `GraphicsMode (CRenderSettings::*)() const` member
    // through the `+4` slot (0x1426c..0x14290) and invokes it. The
    // member-pointer dance collapses into the stored getter fn; same shape
    // as 0x106bc.
    (desc.getter)(item)
}

// 0x14294 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_14294(desc: &RenderGraphicsEnumPropDesc, item: &mut CRenderSettingsItem, value: i32) {
    // IDA 0x14294..0x142b4 (decompiled): `GetSetImpl<...>::setValue` —
    // resolves the bound `void (CRenderSettingsItem::*)(GraphicsMode)`
    // member through the `+12` slot (0x142a0..0x142b0) and invokes it with
    // `*a3`. Collapses into the stored setter fn; same shape as 0x106e8.
    (desc.setter)(item, value)
}

// 0x142b8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)")]
pub fn stub_142b8(map: &mut BTreeMap<*const (), i32>, key: *const ()) -> &mut i32 {
    // IDA 0x142b8 (`map<Name const*, ResolutionPreset>::operator[]`): tree
    // search down the right spine with a default-inserted zero on miss
    // (decomp 0x142c2..0x142d0); `entry().or_insert(0)` is the same
    // lookup-or-create. The `Name const*` key keeps address order, so the
    // opaque pointer stands in. Same shape as `instance::stub_0x3dd564`.
    map.entry(key).or_insert(0)
}

// 0x14310 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
pub fn stub_14310(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x14310 (`_Rb_tree::_M_insert_unique` with the position hint):
    // the hint only seeds the search, so the hinted insert collapses into a
    // plain unique insert; `true` on fresh placement. Same shape as
    // `instance::stub_0x3dd5bc`.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x143c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
pub fn stub_143c4(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) {
    // IDA 0x143c4 (`_Rb_tree::_M_insert`): links the already-uniqueness-
    // checked node into the tree (decomp 0x143f4 `operator new(0x18)`);
    // after the uniqueness check the link is a plain insert. Same shape as
    // `instance::stub_0x3dd670`.
    map.insert(key, value);
}

// 0x1441c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
pub fn stub_1441c(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x1441c (`_Rb_tree::_M_insert_unique` by value): search, then link
    // on miss; same unique insert as 0x14310 without the hint. Same shape
    // as `instance::stub_0x3dd6c8`.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x14484 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)")]
pub fn stub_14484(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x14484 (`vector<ResolutionPreset>::resize(n, value)`): grow
    // through `_M_fill_insert` when below size (decomp 0x1449c..0x144ac),
    // else truncate the finish pointer (decomp 0x144a2). `resize` is both;
    // the enum payload is an `i32` word.
    vec.resize(len, value);
}

// 0x144b8 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_144b8(vec: &mut Vec<i32>, value: &i32) {
    // IDA 0x144b8 (`vector<ResolutionPreset>::push_back`): fast-path store
    // at finish + bump (decomp 0x144c8..0x144d0), slow path through
    // `_M_insert_aux` (decomp 0x144da). `push` grows the same way.
    vec.push(*value);
}

// 0x144e0 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_144e0(vec: &mut Vec<i32>, index: usize, value: &i32) {
    // IDA 0x144e0 (`vector<ResolutionPreset>::_M_insert_aux`): the
    // reallocation/shift tail behind `push_back`/`insert` (decomp through
    // 0x144e0+2681); the element lands at the iterator position with the
    // tail shifted up. `insert` is the same place-and-shift.
    vec.insert(index, *value);
}

// 0x145c4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)")]
pub fn stub_145c4(vec: &mut Vec<i32>, n: usize) {
    // IDA 0x145c4 (`_Vector_base<ResolutionPreset>::_M_allocate(n)`): raw
    // storage for `n` elements; the returned pointer is unmanaged in Rust,
    // so only the capacity effect is modelled. `reserve` is the same
    // allocation without the pointer.
    vec.reserve(n);
}

// 0x145dc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)")]
pub fn stub_145dc(vec: &mut Vec<i32>, src_start: usize, src_end: usize, dst_end: usize) {
    // IDA 0x145dc (`__copy_backward` over `ResolutionPreset*`): copies
    // `[first, last)` to the range ending at `result`, back to front so
    // overlap is safe. `copy_within` with the matching start is the same
    // overlap-safe backward move.
    let len = src_end - src_start;
    vec.copy_within(src_start..src_end, dst_end - len);
}

// 0x14618 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_14618(vec: &mut Vec<i32>, index: usize, n: usize, value: &i32) {
    // IDA 0x14618 (`vector<ResolutionPreset>::_M_fill_insert`): inserts `n`
    // copies of the value at the iterator position, shifting the tail.
    // `splice` with a repeat tail is the same insert-and-shift.
    vec.splice(index..index, std::iter::repeat(*value).take(n));
}

// 0x147a8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)")]
pub fn stub_147a8(map: &mut BTreeMap<*const (), i32>, key: *const ()) -> &mut i32 {
    // IDA 0x147a8 (`map<Name const*, QualityLevel>::operator[]`): tree
    // search with a default-inserted zero on miss; `entry().or_insert(0)`
    // is the same lookup-or-create. Same shape as 0x142b8.
    map.entry(key).or_insert(0)
}

// 0x14800 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
pub fn stub_14800(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x14800 (`_Rb_tree::_M_insert_unique` with the position hint):
    // the hint only seeds the search, so the hinted insert collapses into a
    // plain unique insert; `true` on fresh placement. Same shape as 0x14310.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x148b4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
pub fn stub_148b4(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) {
    // IDA 0x148b4 (`_Rb_tree::_M_insert`): links the
    // already-uniqueness-checked node into the tree; after the uniqueness
    // check the link is a plain insert. Same shape as 0x143c4.
    map.insert(key, value);
}

// 0x1490c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
pub fn stub_1490c(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x1490c (`_Rb_tree::_M_insert_unique` by value): search, then
    // link on miss; same unique insert as the hinted twin without the hint.
    // Same shape as 0x1441c.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x14974 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)")]
pub fn stub_14974(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x14974 (`vector<QualityLevel>::resize(n, value)`): grow
    // through `_M_fill_insert` when below size, else truncate the finish
    // pointer. `resize` is both; the enum payload is an `i32` word. Same
    // shape as 0x14484.
    vec.resize(len, value);
}

// 0x149a8 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_149a8(vec: &mut Vec<i32>, value: &i32) {
    // IDA 0x149a8 (`vector<QualityLevel>::push_back`): fast-path store
    // at finish + bump, slow path through `_M_insert_aux`. `push` grows the
    // same way. Same shape as 0x144b8.
    vec.push(*value);
}

// 0x149d0 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_149d0(vec: &mut Vec<i32>, index: usize, value: &i32) {
    // IDA 0x149d0 (`vector<QualityLevel>::_M_insert_aux`): the
    // reallocation/shift tail behind `push_back`/`insert`; the element lands
    // at the iterator position with the tail shifted up. Same shape as
    // 0x144e0.
    vec.insert(index, *value);
}

// 0x14ab4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)")]
pub fn stub_14ab4(vec: &mut Vec<i32>, n: usize) {
    // IDA 0x14ab4 (`_Vector_base<QualityLevel>::_M_allocate(n)`):
    // `std::__throw_bad_alloc()` for `n >= 0x40000000`, else raw storage
    // for `n` elements. The pointer is unmanaged in Rust, so only the
    // capacity effect is modelled; Rust aborts on OOM the same way. Same
    // shape as 0x145c4.
    vec.reserve(n);
}

// 0x14acc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)")]
pub fn stub_14acc(vec: &mut Vec<i32>, src_start: usize, src_end: usize, dst_end: usize) {
    // IDA 0x14acc (`__copy_backward` over `QualityLevel*`): copies
    // `[first, last)` to the range ending at `result`, back to front so
    // overlap is safe. Same shape as 0x145dc.
    let len = src_end - src_start;
    vec.copy_within(src_start..src_end, dst_end - len);
}

// 0x14b08 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_14b08(vec: &mut Vec<i32>, index: usize, n: usize, value: &i32) {
    // IDA 0x14b08 (`vector<QualityLevel>::_M_fill_insert`): inserts
    // `n` copies of the value at the iterator position, shifting the tail.
    // Same shape as 0x14618.
    vec.splice(index..index, std::iter::repeat(*value).take(n));
}

// 0x14c98 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)")]
pub fn stub_14c98(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x14c98 (`vector<ShadowMode>::resize(n, value)`): grow
    // through `_M_fill_insert` when below size, else truncate the finish
    // pointer. `resize` is both; the enum payload is an `i32` word. Same
    // shape as 0x14484.
    vec.resize(len, value);
}

// 0x14ccc — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_14ccc(vec: &mut Vec<i32>, value: &i32) {
    // IDA 0x14ccc (`vector<ShadowMode>::push_back`): fast-path store
    // at finish + bump, slow path through `_M_insert_aux`. `push` grows the
    // same way. Same shape as 0x144b8.
    vec.push(*value);
}

// 0x14cf4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_14cf4(map: &mut BTreeMap<*const (), i32>, key: *const ()) -> &mut i32 {
    // IDA 0x14cf4 (`map<Name const*, ShadowMode>::operator[]`): tree
    // search with a default-inserted zero on miss; `entry().or_insert(0)`
    // is the same lookup-or-create. Same shape as 0x142b8.
    map.entry(key).or_insert(0)
}

// 0x14d4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
pub fn stub_14d4c(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x14d4c (`_Rb_tree::_M_insert_unique` with the position hint):
    // the hint only seeds the search, so the hinted insert collapses into a
    // plain unique insert; `true` on fresh placement. Same shape as 0x14310.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x14e00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
pub fn stub_14e00(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) {
    // IDA 0x14e00 (`_Rb_tree::_M_insert`): links the
    // already-uniqueness-checked node into the tree; after the uniqueness
    // check the link is a plain insert. Same shape as 0x143c4.
    map.insert(key, value);
}

// 0x14e58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
pub fn stub_14e58(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x14e58 (`_Rb_tree::_M_insert_unique` by value): search, then
    // link on miss; same unique insert as the hinted twin without the hint.
    // Same shape as 0x1441c.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x14ec0 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_14ec0(vec: &mut Vec<i32>, index: usize, value: &i32) {
    // IDA 0x14ec0 (`vector<ShadowMode>::_M_insert_aux`): the
    // reallocation/shift tail behind `push_back`/`insert`; the element lands
    // at the iterator position with the tail shifted up. Same shape as
    // 0x144e0.
    vec.insert(index, *value);
}

// 0x14fa4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings10ShadowModeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_allocate(unsigned long)")]
pub fn stub_14fa4(vec: &mut Vec<i32>, n: usize) {
    // IDA 0x14fa4 (`_Vector_base<ShadowMode>::_M_allocate(n)`):
    // `std::__throw_bad_alloc()` for `n >= 0x40000000`, else raw storage
    // for `n` elements. The pointer is unmanaged in Rust, so only the
    // capacity effect is modelled; Rust aborts on OOM the same way. Same
    // shape as 0x145c4.
    vec.reserve(n);
}

// 0x14fbc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings10ShadowModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::ShadowMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *>(RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *)")]
pub fn stub_14fbc(vec: &mut Vec<i32>, src_start: usize, src_end: usize, dst_end: usize) {
    // IDA 0x14fbc (`__copy_backward` over `ShadowMode*`): copies
    // `[first, last)` to the range ending at `result`, back to front so
    // overlap is safe. Same shape as 0x145dc.
    let len = src_end - src_start;
    vec.copy_within(src_start..src_end, dst_end - len);
}

// 0x14ff8 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,unsigned long,RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_14ff8(vec: &mut Vec<i32>, index: usize, n: usize, value: &i32) {
    // IDA 0x14ff8 (`vector<ShadowMode>::_M_fill_insert`): inserts
    // `n` copies of the value at the iterator position, shifting the tail.
    // Same shape as 0x14618.
    vec.splice(index..index, std::iter::repeat(*value).take(n));
}

// 0x15188 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::resize(unsigned long,RBX::CRenderSettings::AntialiasingMode)")]
pub fn stub_15188(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x15188 (`vector<AntialiasingMode>::resize(n, value)`): grow
    // through `_M_fill_insert` when below size, else truncate the finish
    // pointer. `resize` is both; the enum payload is an `i32` word. Same
    // shape as 0x14484.
    vec.resize(len, value);
}

// 0x151bc — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::push_back(RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_151bc(vec: &mut Vec<i32>, value: &i32) {
    // IDA 0x151bc (`vector<AntialiasingMode>::push_back`): fast-path store
    // at finish + bump, slow path through `_M_insert_aux`. `push` grows the
    // same way. Same shape as 0x144b8.
    vec.push(*value);
}

// 0x151e4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16AntialiasingModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AntialiasingMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_151e4(map: &mut BTreeMap<*const (), i32>, key: *const ()) -> &mut i32 {
    // IDA 0x151e4 (`map<Name const*, AntialiasingMode>::operator[]`): tree
    // search with a default-inserted zero on miss; `entry().or_insert(0)`
    // is the same lookup-or-create. Same shape as 0x142b8.
    map.entry(key).or_insert(0)
}

// 0x1523c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
pub fn stub_1523c(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x1523c (`_Rb_tree::_M_insert_unique` with the position hint):
    // the hint only seeds the search, so the hinted insert collapses into a
    // plain unique insert; `true` on fresh placement. Same shape as 0x14310.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x152f0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
pub fn stub_152f0(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) {
    // IDA 0x152f0 (`_Rb_tree::_M_insert`): links the
    // already-uniqueness-checked node into the tree; after the uniqueness
    // check the link is a plain insert. Same shape as 0x143c4.
    map.insert(key, value);
}

// 0x15348 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
pub fn stub_15348(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x15348 (`_Rb_tree::_M_insert_unique` by value): search, then
    // link on miss; same unique insert as the hinted twin without the hint.
    // Same shape as 0x1441c.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x153b0 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_153b0(vec: &mut Vec<i32>, index: usize, value: &i32) {
    // IDA 0x153b0 (`vector<AntialiasingMode>::_M_insert_aux`): the
    // reallocation/shift tail behind `push_back`/`insert`; the element lands
    // at the iterator position with the tail shifted up. Same shape as
    // 0x144e0.
    vec.insert(index, *value);
}

// 0x15494 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_allocate(unsigned long)")]
pub fn stub_15494(vec: &mut Vec<i32>, n: usize) {
    // IDA 0x15494 (`_Vector_base<AntialiasingMode>::_M_allocate(n)`):
    // `std::__throw_bad_alloc()` for `n >= 0x40000000`, else raw storage
    // for `n` elements. The pointer is unmanaged in Rust, so only the
    // capacity effect is modelled; Rust aborts on OOM the same way. Same
    // shape as 0x145c4.
    vec.reserve(n);
}

// 0x154ac — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16AntialiasingModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::AntialiasingMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *>(RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *)")]
pub fn stub_154ac(vec: &mut Vec<i32>, src_start: usize, src_end: usize, dst_end: usize) {
    // IDA 0x154ac (`__copy_backward` over `AntialiasingMode*`): copies
    // `[first, last)` to the range ending at `result`, back to front so
    // overlap is safe. Same shape as 0x145dc.
    let len = src_end - src_start;
    vec.copy_within(src_start..src_end, dst_end - len);
}

// 0x154e8 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,unsigned long,RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_154e8(vec: &mut Vec<i32>, index: usize, n: usize, value: &i32) {
    // IDA 0x154e8 (`vector<AntialiasingMode>::_M_fill_insert`): inserts
    // `n` copies of the value at the iterator position, shifting the tail.
    // Same shape as 0x14618.
    vec.splice(index..index, std::iter::repeat(*value).take(n));
}

// 0x15678 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::resize(unsigned long,RBX::CRenderSettings::FrameRateManagerMode)")]
pub fn stub_15678(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x15678 (`vector<FrameRateManagerMode>::resize(n, value)`): grow
    // through `_M_fill_insert` when below size (decomp 0x15678+12),
    // else truncate the finish pointer. `resize` is both. Same shape as
    // 0x14484.
    vec.resize(len, value);
}

// 0x156ac — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::push_back(RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_156ac(vec: &mut Vec<i32>, value: &i32) {
    // IDA 0x156ac (`vector<FrameRateManagerMode>::push_back`): fast-path store
    // at finish + bump, slow path through `_M_insert_aux`. `push` grows the
    // same way. Same shape as 0x144b8.
    vec.push(*value);
}

// 0x156d4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings20FrameRateManagerModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::FrameRateManagerMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_156d4(map: &mut BTreeMap<*const (), i32>, key: *const ()) -> &mut i32 {
    // IDA 0x156d4 (`map<Name const*, FrameRateManagerMode>::operator[]`): tree
    // search with a default-inserted zero on miss; `entry().or_insert(0)`
    // is the same lookup-or-create. Same shape as 0x142b8.
    map.entry(key).or_insert(0)
}

// 0x1572c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
pub fn stub_1572c(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x1572c (`_Rb_tree::_M_insert_unique` with the position hint):
    // the hint only seeds the search; `true` on fresh placement. Same shape
    // as 0x14310.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x157e0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
pub fn stub_157e0(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) {
    // IDA 0x157e0 (`_Rb_tree::_M_insert`): links the
    // already-uniqueness-checked node into the tree; after the uniqueness
    // check the link is a plain insert. Same shape as 0x143c4.
    map.insert(key, value);
}

// 0x15838 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
pub fn stub_15838(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x15838 (`_Rb_tree::_M_insert_unique` by value): search, then
    // link on miss. Same shape as 0x1441c.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x158a0 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_158a0(vec: &mut Vec<i32>, index: usize, value: &i32) {
    // IDA 0x158a0 (`vector<FrameRateManagerMode>::_M_insert_aux`): the
    // reallocation/shift tail; the element lands at the iterator position
    // with the tail shifted up. Same shape as 0x144e0.
    vec.insert(index, *value);
}

// 0x15984 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_allocate(unsigned long)")]
pub fn stub_15984(vec: &mut Vec<i32>, n: usize) {
    // IDA 0x15984 (`_Vector_base<FrameRateManagerMode>::_M_allocate(n)`):
    // `std::__throw_bad_alloc()` for huge `n`, else raw storage. The pointer
    // is unmanaged in Rust, so only the capacity effect is modelled. Same
    // shape as 0x145c4.
    vec.reserve(n);
}

// 0x1599c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings20FrameRateManagerModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *>(RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *)")]
pub fn stub_1599c(vec: &mut Vec<i32>, src_start: usize, src_end: usize, dst_end: usize) {
    // IDA 0x1599c (`__copy_backward` over `FrameRateManagerMode*`): copies
    // `[first, last)` to the range ending at `result`, back to front so
    // overlap is safe. Same shape as 0x145dc.
    let len = src_end - src_start;
    vec.copy_within(src_start..src_end, dst_end - len);
}

// 0x159d8 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,unsigned long,RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_159d8(vec: &mut Vec<i32>, index: usize, n: usize, value: &i32) {
    // IDA 0x159d8 (`vector<FrameRateManagerMode>::_M_fill_insert`): inserts
    // `n` copies of the value at the iterator position, shifting the tail.
    // Same shape as 0x14618.
    vec.splice(index..index, std::iter::repeat(*value).take(n));
}

// 0x15b68 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::resize(unsigned long,RBX::CRenderSettings::GraphicsMode)")]
pub fn stub_15b68(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x15b68 (`vector<GraphicsMode>::resize(n, value)`): grow
    // through `_M_fill_insert` when below size (decomp 0x15b68+12),
    // else truncate the finish pointer. `resize` is both. Same shape as
    // 0x14484.
    vec.resize(len, value);
}

// 0x15b9c — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12GraphicsModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::GraphicsMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_15b9c(map: &mut BTreeMap<*const (), i32>, key: *const ()) -> &mut i32 {
    // IDA 0x15b9c (`map<Name const*, GraphicsMode>::operator[]`): tree
    // search with a default-inserted zero on miss; `entry().or_insert(0)`
    // is the same lookup-or-create. Same shape as 0x142b8.
    map.entry(key).or_insert(0)
}

// 0x15bf4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
pub fn stub_15bf4(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x15bf4 (`_Rb_tree::_M_insert_unique` with the position hint):
    // the hint only seeds the search; `true` on fresh placement. Same shape
    // as 0x14310.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x15ca8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
pub fn stub_15ca8(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) {
    // IDA 0x15ca8 (`_Rb_tree::_M_insert`): links the
    // already-uniqueness-checked node into the tree; after the uniqueness
    // check the link is a plain insert. Same shape as 0x143c4.
    map.insert(key, value);
}

// 0x15d00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
pub fn stub_15d00(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x15d00 (`_Rb_tree::_M_insert_unique` by value): search, then
    // link on miss. Same shape as 0x1441c.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x15d68 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,unsigned long,RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_15d68(vec: &mut Vec<i32>, index: usize, n: usize, value: &i32) {
    // IDA 0x15d68 (`vector<GraphicsMode>::_M_fill_insert`): inserts
    // `n` copies of the value at the iterator position, shifting the tail.
    // Same shape as 0x14618.
    vec.splice(index..index, std::iter::repeat(*value).take(n));
}

// 0x15ef8 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_allocate(unsigned long)")]
pub fn stub_15ef8(vec: &mut Vec<i32>, n: usize) {
    // IDA 0x15ef8 (`_Vector_base<GraphicsMode>::_M_allocate(n)`):
    // `std::__throw_bad_alloc()` for huge `n`, else raw storage. The pointer
    // is unmanaged in Rust, so only the capacity effect is modelled. Same
    // shape as 0x145c4.
    vec.reserve(n);
}

// 0x15f10 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::GraphicsMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *>(RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *)")]
pub fn stub_15f10(vec: &mut Vec<i32>, src_start: usize, src_end: usize, dst_end: usize) {
    // IDA 0x15f10 (`__copy_backward` over `GraphicsMode*`): copies
    // `[first, last)` to the range ending at `result`, back to front so
    // overlap is safe. Same shape as 0x145dc.
    let len = src_end - src_start;
    vec.copy_within(src_start..src_end, dst_end - len);
}

// 0x15f4c — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::push_back(RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_15f4c(vec: &mut Vec<i32>, value: &i32) {
    // IDA 0x15f4c (`vector<GraphicsMode>::push_back`): fast-path store
    // at finish + bump, slow path through `_M_insert_aux`. `push` grows the
    // same way. Same shape as 0x144b8.
    vec.push(*value);
}

// 0x15f74 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_15f74(vec: &mut Vec<i32>, index: usize, value: &i32) {
    // IDA 0x15f74 (`vector<GraphicsMode>::_M_insert_aux`): the
    // reallocation/shift tail; the element lands at the iterator position
    // with the tail shifted up. Same shape as 0x144e0.
    vec.insert(index, *value);
}

// 0x16058 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::resize(unsigned long,RBX::CRenderSettings::AASamples)")]
pub fn stub_16058(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x16058 (`vector<AASamples>::resize(n, value)`): grow
    // through `_M_fill_insert` when below size (decomp 0x16058+12),
    // else truncate the finish pointer. `resize` is both. Same shape as
    // 0x14484.
    vec.resize(len, value);
}

// 0x1608c — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::push_back(RBX::CRenderSettings::AASamples const&)")]
pub fn stub_1608c(vec: &mut Vec<i32>, value: &i32) {
    // IDA 0x1608c (`vector<AASamples>::push_back`): fast-path store
    // at finish + bump, slow path through `_M_insert_aux`. `push` grows the
    // same way. Same shape as 0x144b8.
    vec.push(*value);
}

// 0x160b4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AASamples,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::operator[](RBX::Name const* const&)")]
pub fn stub_160b4(map: &mut BTreeMap<*const (), i32>, key: *const ()) -> &mut i32 {
    // IDA 0x160b4 (`map<Name const*, AASamples>::operator[]`): tree
    // search with a default-inserted zero on miss; `entry().or_insert(0)`
    // is the same lookup-or-create. Same shape as 0x142b8.
    map.entry(key).or_insert(0)
}

// 0x1610c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
pub fn stub_1610c(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x1610c (`_Rb_tree::_M_insert_unique` with the position hint):
    // the hint only seeds the search; `true` on fresh placement. Same shape
    // as 0x14310.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x161c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
pub fn stub_161c0(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) {
    // IDA 0x161c0 (`_Rb_tree::_M_insert`): links the
    // already-uniqueness-checked node into the tree; after the uniqueness
    // check the link is a plain insert. Same shape as 0x143c4.
    map.insert(key, value);
}

// 0x16218 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
pub fn stub_16218(map: &mut BTreeMap<*const (), i32>, key: *const (), value: i32) -> bool {
    // IDA 0x16218 (`_Rb_tree::_M_insert_unique` by value): search, then
    // link on miss. Same shape as 0x1441c.
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x16280 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,RBX::CRenderSettings::AASamples const&)")]
pub fn stub_16280(vec: &mut Vec<i32>, index: usize, value: &i32) {
    // IDA 0x16280 (`vector<AASamples>::_M_insert_aux`): the
    // reallocation/shift tail; the element lands at the iterator position
    // with the tail shifted up. Same shape as 0x144e0.
    vec.insert(index, *value);
}

// 0x16364 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_allocate(unsigned long)")]
pub fn stub_16364(vec: &mut Vec<i32>, n: usize) {
    // IDA 0x16364 (`_Vector_base<AASamples>::_M_allocate(n)`):
    // `std::__throw_bad_alloc()` for huge `n`, else raw storage. The pointer
    // is unmanaged in Rust, so only the capacity effect is modelled. Same
    // shape as 0x145c4.
    vec.reserve(n);
}

// 0x1637c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::AASamples * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *>(RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *)")]
pub fn stub_1637c(vec: &mut Vec<i32>, src_start: usize, src_end: usize, dst_end: usize) {
    // IDA 0x1637c (`__copy_backward` over `AASamples*`): copies
    // `[first, last)` to the range ending at `result`, back to front so
    // overlap is safe. Same shape as 0x145dc.
    let len = src_end - src_start;
    vec.copy_within(src_start..src_end, dst_end - len);
}

// 0x163b8 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,unsigned long,RBX::CRenderSettings::AASamples const&)")]
pub fn stub_163b8(vec: &mut Vec<i32>, index: usize, n: usize, value: &i32) {
    // IDA 0x163b8 (`vector<AASamples>::_M_fill_insert`): inserts
    // `n` copies of the value at the iterator position, shifting the tail.
    // Same shape as 0x14618.
    vec.splice(index..index, std::iter::repeat(*value).take(n));
}

// 0x16548 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::initSingleton(void)")]
pub fn stub_16548() {
    // IDA 0x16548 (decompiled, thunk):
    // `Singleton<EnumDesc<ShadowMode>>::initSingleton` — tail-calls
    // `doGetSingleton`. The once-init collapses into the table singleton
    // touch; same treatment as the 0xfecc/0xff9a touches behind
    // `resolution_preset_enum_desc()`.
    let _ = shadow_mode_enum_desc();
}

#[cfg(test)]
mod fr_enum_desc_tests {
    use super::*;

    fn get_fr(item: &CRenderSettingsItem) -> i32 {
        item.frame_rate_manager_mode
    }

    fn set_fr(item: &mut CRenderSettingsItem, value: i32) {
        item.frame_rate_manager_mode = value;
    }

    #[test]
    fn dense_framerate_round_trip() {
        let table = frame_rate_manager_mode_enum_desc();
        assert_eq!(stub_13928(table, 1), 1);
        assert_eq!(stub_13928(table, 0), 0);
        assert_eq!(stub_13928(table, 2), 2);
        let mut d = crate::generated_191::stub_0x131a8("FrameRateManager", "Rendering", get_fr, set_fr);
        let mut item = CRenderSettingsItem::default();
        assert!(stub_13888(&d, &mut item, 1));
        assert_eq!(stub_13880(&d, &item), 1);
        assert_eq!(stub_13830(&d, &item), Some(1));
        assert!(stub_1384c(&d, &mut item, 0));
        assert_eq!(item.frame_rate_manager_mode, 0);
        assert!(!stub_13888(&d, &mut item, 5));
        assert_eq!(stub_138d4(&d, &item), 0);
        assert!(stub_13998(&d, &mut item, 1));
        assert_eq!(item.frame_rate_manager_mode, 1);
        assert!(!stub_139d8() && !stub_139dc());
        assert_eq!(stub_139e0(&d, &item), 1);
        stub_13a0c(&d, &mut item, 0);
        assert_eq!(item.frame_rate_manager_mode, 0);
        assert!(stub_13388(&d) == false && stub_13398(&d) == false);
        stub_1335c(&mut d as *mut crate::generated_191::RenderFrameRateEnumPropDesc);
    }

    #[test]
    fn sparse_graphics_remap() {
        fn get_gm(item: &CRenderSettingsItem) -> i32 {
            item.graphics_mode
        }
        fn set_gm(item: &mut CRenderSettingsItem, value: i32) {
            item.graphics_mode = value;
        }
        let table = graphics_mode_enum_desc();
        assert_eq!(table.pairs.len(), 4);
        let d = stub_13a30("GraphicsMode", "Rendering", get_gm, set_gm);
        let mut item = CRenderSettingsItem::default();
        stub_13c58(&d, &item);
        assert!(stub_13c30(&d, &item, &item));
        assert!(stub_13c10(&d) == false && stub_13c20(&d) == false);
        let mut desc = d;
        stub_13be4(&mut desc as *mut RenderGraphicsEnumPropDesc);
    }
}

#[cfg(test)]
mod gm_stl_tests {
    use super::*;
    use std::collections::BTreeMap;

    fn get_gm(item: &CRenderSettingsItem) -> i32 {
        item.graphics_mode
    }

    fn set_gm(item: &mut CRenderSettingsItem, value: i32) {
        item.graphics_mode = value;
    }

    #[test]
    fn sparse_graphics_full_suite() {
        let table = graphics_mode_enum_desc();
        assert_eq!(stub_141b0(table, 5), 3);
        assert_eq!(stub_141b0(table, 4), 2);
        assert_eq!(stub_141b0(table, 3), 1);
        assert_eq!(stub_141b0(table, 1), 0);
        assert_eq!(stub_141b0(table, 0), -1);
        assert_eq!(stub_141b0(table, 2), -1);
        let mut d = stub_13a30("GraphicsMode", "Rendering", get_gm, set_gm);
        let mut item = CRenderSettingsItem::default();
        assert!(stub_14110(&d, &mut item, 4));
        assert_eq!(stub_14108(&d, &item), 4);
        assert_eq!(stub_140b8(&d, &item), Some(2));
        assert!(stub_140d4(&d, &mut item, 3));
        assert_eq!(item.graphics_mode, 5);
        assert!(!stub_14110(&d, &mut item, 2));
        assert!(stub_14220(&d, &mut item, 0));
        assert_eq!(item.graphics_mode, 1);
        assert!(!stub_14220(&d, &mut item, 9));
        assert_eq!(stub_14268(&d, &item), 1);
        stub_14294(&d, &mut item, 3);
        assert_eq!(item.graphics_mode, 3);
        assert!(!stub_14260() && !stub_14264());
        let mut out = String::new();
        stub_13df4(&d, &mut item, &mut out);
        assert_eq!(out, "Direct3D");
        assert!(stub_13e18(&d, &mut item, "OpenGL"));
        assert_eq!(item.graphics_mode, 4);
        assert!(!stub_13e18(&d, &mut item, "Vulkan"));
        stub_13c7c(&d, &mut item, &IntVariant { value: 5 });
        assert_eq!(item.graphics_mode, 5);
        assert_eq!(stub_13c58(&d, &item).value, 5);
        assert_eq!(stub_13e58(&d, &item).int_value, 5);
        stub_13e78(&d, &mut item, &RenderXmlInput::Int(1));
        assert_eq!(item.graphics_mode, 1);
        assert!(stub_13dcc(&d, &mut CRenderSettingsItem::default(), &item) == ());
        let _ = stub_13df0();
        stub_13be4(&mut d as *mut RenderGraphicsEnumPropDesc);
    }

    #[test]
    fn name_map_and_preset_vec() {
        let mut map: BTreeMap<*const (), i32> = BTreeMap::new();
        let k1 = 0x1000 as *const ();
        let k2 = 0x2000 as *const ();
        assert!(stub_14310(&mut map, k1, 7));
        assert!(!stub_14310(&mut map, k1, 9));
        assert_eq!(*stub_142b8(&mut map, k1), 7);
        assert_eq!(*stub_142b8(&mut map, k2), 0);
        stub_143c4(&mut map, k2, 3);
        assert_eq!(map[&k2], 3);
        assert!(stub_1441c(&mut map, 0x3000 as *const (), 1));
        assert_eq!(map.len(), 3);
        let mut vec: Vec<i32> = Vec::new();
        stub_144b8(&mut vec, &4);
        stub_144b8(&mut vec, &8);
        assert_eq!(vec, [4, 8]);
        stub_14484(&mut vec, 4, 1);
        assert_eq!(vec, [4, 8, 1, 1]);
        stub_14484(&mut vec, 1, 9);
        assert_eq!(vec, [4]);
        stub_144e0(&mut vec, 1, &8);
        assert_eq!(vec, [4, 8]);
        stub_145dc(&mut vec, 0, 1, 2);
        assert_eq!(vec, [4, 4]);
        stub_14618(&mut vec, 1, 2, &8);
        assert_eq!(vec, [4, 8, 8, 4]);
        stub_145c4(&mut vec, 64);
        assert!(vec.capacity() >= 4);
    }
}

#[cfg(test)]
mod ql_sh_stl_tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn quality_map_and_shadow_vec() {
        let mut map: BTreeMap<*const (), i32> = BTreeMap::new();
        let k = 0x5000 as *const ();
        assert!(stub_14800(&mut map, k, 21));
        assert!(!stub_14800(&mut map, k, 0));
        assert_eq!(*stub_147a8(&mut map, k), 21);
        stub_148b4(&mut map, k, 5);
        assert_eq!(map[&k], 5);
        assert!(stub_1490c(&mut map, 0x6000 as *const (), 2));
        let mut vec: Vec<i32> = Vec::new();
        stub_149a8(&mut vec, &1);
        stub_149a8(&mut vec, &3);
        stub_149d0(&mut vec, 1, &2);
        assert_eq!(vec, [1, 2, 3]);
        stub_14974(&mut vec, 5, 0);
        assert_eq!(vec, [1, 2, 3, 0, 0]);
        stub_14acc(&mut vec, 0, 2, 5);
        assert_eq!(vec, [1, 2, 3, 1, 2]);
        stub_14b08(&mut vec, 0, 2, &9);
        assert_eq!(vec, [9, 9, 1, 2, 3, 1, 2]);
        stub_14ab4(&mut vec, 32);
        let mut sh: Vec<i32> = Vec::new();
        stub_14ccc(&mut sh, &2);
        assert_eq!(sh, [2]);
        stub_14ff8(&mut sh, 1, 3, &1);
        assert_eq!(sh, [2, 1, 1, 1]);
    }
}
