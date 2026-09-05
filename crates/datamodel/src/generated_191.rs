// Auto-generated skeletons for rbx-datamodel -- from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact RBX:: prefix), EA-sorted — filtered complete (10215/10215), global gap filler low-EA
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x109b0..0x131a8 | total filtered 10215, remaining 0 after batch; local 18400->18520 distinct, 67145->67025 not in datamodel (0 global missing)
// Shard: 191 EA-sorted asc next 120 low-EA global gap filler after 0x1089c not yet in datamodel (filtered exhausted, 67145 missing before -> 67025 after)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_189::{CRenderSettingsItem, RenderEnumDesc, stub_0x8a88, stub_0x8c4c};
use crate::generated_190::{IntVariant, RenderInt32PropDesc, RenderXmlInput, RenderXmlIntValue};
use std::sync::LazyLock;

// 0x10a08/0x10bbc — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2/D0Ev
/// Rust model of `EnumPropDescriptor<CRenderSettingsItem, AntialiasingMode>`
/// (IDA `0x10a08` C2 / `0x10bbc` D0): the shared `EnumDesc<AntialiasingMode>`
/// table (via `antialiasing_mode_enum_desc()`, built by 0x8a88), the bound
/// getter/setter pair in the heap impl, and the read/write-only flags (both
/// queries return 0 per `0x10be8`/`0x10bf8`, so the bits stay set). Same
/// shape as `RenderEnumPropDesc` (0xfe84).
pub struct RenderAAEnumPropDesc {
    pub name: &'static str,
    pub category: &'static str,
    pub getter: fn(&CRenderSettingsItem) -> i32,
    pub setter: fn(&mut CRenderSettingsItem, i32),
    pub read_only: bool,
    pub write_only: bool,
}

/// The `EnumDesc<AntialiasingMode>` singleton behind the AA descriptor suite
/// (cf. `resolution_preset_enum_desc()` for the ResolutionPreset twin);
/// built once by the 0x8a88 constructor.
static ANTIALIASING_MODE_ENUM_DESC: LazyLock<RenderEnumDesc> = LazyLock::new(stub_0x8a88);
/// Singleton accessor for the AA enum table.
pub fn antialiasing_mode_enum_desc() -> &'static RenderEnumDesc {
    LazyLock::force(&ANTIALIASING_MODE_ENUM_DESC)
}

// 0x11290/0x11444 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2/D0Ev
/// Rust model of `EnumPropDescriptor<CRenderSettingsItem, ShadowMode>`
/// (IDA `0x11290` C2 / `0x11444` D0): the shared `EnumDesc<ShadowMode>`
/// table (via `shadow_mode_enum_desc()`, built by 0x8c4c), the bound
/// getter/setter pair in the heap impl, and the read/write-only flags (both
/// queries return 0 per `0x11470`/`0x11480`, so the bits stay set). Same
/// shape as `RenderAAEnumPropDesc` (0x10a08).
pub struct RenderShadowEnumPropDesc {
    pub name: &'static str,
    pub category: &'static str,
    pub getter: fn(&CRenderSettingsItem) -> i32,
    pub setter: fn(&mut CRenderSettingsItem, i32),
    pub read_only: bool,
    pub write_only: bool,
}

/// The `EnumDesc<ShadowMode>` singleton behind the shadow descriptor suite;
/// built once by the 0x8c4c constructor.
static SHADOW_MODE_ENUM_DESC: LazyLock<RenderEnumDesc> = LazyLock::new(stub_0x8c4c);
/// Singleton accessor for the shadow enum table.
pub fn shadow_mode_enum_desc() -> &'static RenderEnumDesc {
    LazyLock::force(&SHADOW_MODE_ENUM_DESC)
}

// 0x109b0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isReadOnly(void)const")]
pub fn stub_0x109b0() -> bool {
    // IDA 0x109b0..0x109b2 (decompiled):
    // `GetSetImpl<int (CRenderSettings::*)() const, void
    // (CRenderSettingsItem::*)(int)>::isReadOnly` returns `0`. Same shape as
    // 0x106b4.
    false
}

// 0x109b4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isWriteOnly(void)const")]
pub fn stub_0x109b4() -> bool {
    // IDA 0x109b4..0x109b6 (decompiled): the int-member `GetSetImpl`
    // `isWriteOnly` twin — returns `0`. Same shape as 0x106b8.
    false
}

// 0x109b8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x109b8(desc: &RenderInt32PropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x109b8..0x109e2 (decompiled): `GetSetImpl<...>::getValue` —
    // resolves the bound `int (CRenderSettings::*)() const` member through
    // the `+4` slot (0x109bc..0x109e0) and invokes it. The member-pointer
    // dance collapses into the stored getter fn; same shape as 0x106bc.
    (desc.getter)(item)
}

// 0x109e4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0x109e4(desc: &RenderInt32PropDesc, item: &mut CRenderSettingsItem, value: i32) {
    // IDA 0x109e4..0x10a04 (decompiled): `GetSetImpl<...>::setValue` —
    // resolves the bound `void (CRenderSettingsItem::*)(int)` member through
    // the `+12` slot (0x109f0..0x10a00) and invokes it with `*a3`. Collapses
    // into the stored setter fn; same shape as 0x106e8.
    (desc.setter)(item, value)
}

// 0x10a08 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::EnumPropDescriptor<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>(char const*,char const*,RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x10a08(
    name: &'static str,
    category: &'static str,
    getter: fn(&CRenderSettingsItem) -> i32,
    setter: fn(&mut CRenderSettingsItem, i32),
) -> RenderAAEnumPropDesc {
    // IDA 0x10a08 (decompiled prologue through the `classDescriptor()` touch,
    // same call shape as 0xfe84..0xfea8):
    // `EnumPropDescriptor<CRenderSettingsItem, AntialiasingMode>::C2` — same
    // construction shape as the ResolutionPreset twin (0xfe84):
    // `EnumDesc<AntialiasingMode>` singleton touch, `PropertyDescriptor` C2,
    // enum-table stores, GetSetImpl alloc with the member pair, vtable
    // install. The table touch keeps the singleton live; the member pointers
    // collapse into the getter/setter fns.
    let _ = antialiasing_mode_enum_desc();
    RenderAAEnumPropDesc { name, category, getter, setter, read_only: false, write_only: false }
}

// 0x10bbc — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
pub fn stub_0x10bbc(_desc: *mut RenderAAEnumPropDesc) {
    // IDA 0x10bbc..0x10bda (decompiled): `EnumPropDescriptor<...>::D0` —
    // vtable install (0x10bd0), impl `delete` on the `+44` slot
    // (0x10bd2..0x10bd8), `operator delete`. Same drop-glue shape as
    // 0x10038.
}

// 0x10be8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isReadOnly(void)const")]
pub fn stub_0x10be8(desc: &RenderAAEnumPropDesc) -> bool {
    // IDA 0x10be8..0x10bf4 (decompiled): `isReadOnly` delegates to the
    // `+44` impl slot `+0` query, which returns `0` (cf. 0x10064).
    desc.read_only
}

// 0x10bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isWriteOnly(void)const")]
pub fn stub_0x10bf8(desc: &RenderAAEnumPropDesc) -> bool {
    // IDA 0x10bf8..0x10c04 (decompiled): `isWriteOnly` delegates to the
    // `+44` impl slot `+4` query, which returns `0` (cf. 0x10074).
    desc.write_only
}

// 0x10c08 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x10c08(
    desc: &RenderAAEnumPropDesc,
    first: &CRenderSettingsItem,
    second: &CRenderSettingsItem,
) -> bool {
    // IDA 0x10c08..0x10c2e (decompiled): `equalValues` — `getValue` through
    // the `+44` slot `+8` on both sides (0x10c18/0x10c2e; the first call
    // reuses the stale `R1` item word) and compare. Same shape as 0x10084.
    (desc.getter)(first) == (desc.getter)(second)
}

// 0x10c30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x10c30(desc: &RenderAAEnumPropDesc, item: &CRenderSettingsItem) -> IntVariant {
    // IDA 0x10c30..0x10c52 (decompiled): `getVariant` — `getEnumValue`
    // through vtable `+68` (0x10c3e), `Type::getSingleton<int>` tag
    // (0x10c44), `placement_any<int>::operator=` (0x10c52). Tag + payload
    // collapse into the int variant. Same shape as 0x100ac.
    IntVariant { value: (desc.getter)(item) }
}

// 0x10c54 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x10c54(desc: &RenderAAEnumPropDesc, item: &mut CRenderSettingsItem, variant: &IntVariant) {
    // IDA 0x10c54 (decompiled): `setVariant` — same holder-identity int fast
    // path plus generic `Variant::convert<int>` fallback as 0x100d0, then
    // the `+72` setter. Our variant only holds ints, so both paths collapse
    // into the stored setter fn.
    (desc.setter)(item, variant.value)
}

// 0x10da4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x10da4(desc: &RenderAAEnumPropDesc, dst: &mut CRenderSettingsItem, src: &CRenderSettingsItem) {
    // IDA 0x10da4..0x10dc6 (decompiled): `copyValue` — `getValue` through
    // the `+44` slot `+8` into a spill (0x10db6), then the `+12` setter
    // (0x10dc6). Same shape as 0x10220.
    let value = (desc.getter)(src);
    (desc.setter)(dst, value)
}

// 0x10dc8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::hasStringValue(void)const")]
pub fn stub_0x10dc8() -> bool {
    // IDA 0x10dc8..0x10dca (decompiled): `hasStringValue` returns `1`. Same
    // shape as 0x10244.
    true
}

// 0x10dcc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x10dcc(desc: &RenderAAEnumPropDesc, item: &CRenderSettingsItem, out: &mut String) {
    // IDA 0x10dcc..0x10dee (decompiled): `getStringValue` — `getValue`
    // through the `+44` slot `+8` (0x10dde), then
    // `EnumDesc<AntialiasingMode>::convertToString`, which is the same
    // table-driven body as 0xd28c: empty when out of range. Same shape as
    // 0x10248.
    let value = (desc.getter)(item);
    match (value >= 0).then(|| antialiasing_mode_enum_desc().lookup_name(value)).flatten() {
        Some(name) => *out = name.to_owned(),
        None => out.clear(),
    }
}

// 0x10df0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x10df0(desc: &RenderAAEnumPropDesc, item: &mut CRenderSettingsItem, name: &str) -> bool {
    // IDA 0x10df0..0x10e20 (decompiled): `setStringValue` — `Name::lookup`
    // (0x10e02), `EnumDesc<AntialiasingMode>::convertToValue` (0x10e10),
    // miss returns 0, hit sets through the `+44` slot `+12` and returns 1.
    // `Name::lookup` collapses into the `&str` itself; same shape as
    // 0x1026c.
    if let Some(value) = antialiasing_mode_enum_desc().lookup_value(name) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x10e30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x10e30(desc: &RenderAAEnumPropDesc, item: &CRenderSettingsItem) -> RenderXmlIntValue {
    // IDA 0x10e30..0x10e4e (decompiled): `writeValue` — `getValue` through
    // the `+44` slot `+8` (0x10e3e), `clearValue` (0x10e44), type word `5`
    // (0x10e4a), int word (0x10e4c), return `5` (0x10e4e). Collapses into
    // the kind/value pair. Same shape as 0x102ac.
    RenderXmlIntValue { kind: 5, int_value: (desc.getter)(item) }
}

// 0x10e50 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x10e50(desc: &RenderAAEnumPropDesc, item: &mut CRenderSettingsItem, input: &RenderXmlInput) {
    // IDA 0x10e50 (decompiled): `readValue` — same `isXsiNil` bail / int
    // (`setIntValue`) / string (`Name::lookup` + `convertToValue` + set with
    // `setStringValue`-mismatch fallback) / `ReleaseAssert(false)` shape as
    // 0x102cc. `setIntValue` for this desc is the direct member set (cf.
    // 0x111f8 shape).
    match input {
        RenderXmlInput::Nil => {}
        RenderXmlInput::Int(value) => (desc.setter)(item, *value),
        RenderXmlInput::Text(name) => {
            if !stub_0x10df0(desc, item, name) {
                debug_assert!(false, "0x10e50: false (Reflection.h:359)");
            }
        }
    }
}

// 0x11090 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x11090(desc: &RenderAAEnumPropDesc, item: &CRenderSettingsItem) -> Option<usize> {
    // IDA 0x11090..0x110aa (decompiled): `getIndexValue` — `getValue`
    // through the `+44` slot `+8` (0x110a0), then
    // `EnumDesc<AntialiasingMode>::convertToIndex` (0x110aa): assert plus
    // position search. Same shape as 0x1050c.
    let value = (desc.getter)(item);
    debug_assert!(value >= 0, "0x11090: value>=0");
    antialiasing_mode_enum_desc().pairs.iter().position(|(v, _)| *v == value)
}

// 0x110ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x110ac(desc: &RenderAAEnumPropDesc, item: &mut CRenderSettingsItem, index: usize) -> bool {
    // IDA 0x110ac..0x110dc (decompiled): `setIndexValue` — `count > index`
    // check against the enum count at `+40` (0x110be), indexed value load
    // from the value table at `+144` (0x110c8), `+44` slot `+12` set
    // (0x110d2), return 1 (0x110d4); miss returns 0. The `+144` table holds
    // the values in registration order, so `pairs` stands in. Same shape as
    // 0x10528.
    match antialiasing_mode_enum_desc().pairs.get(index) {
        Some((value, _)) => {
            (desc.setter)(item, *value);
            true
        }
        None => false,
    }
}

// 0x110e0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x110e0(desc: &RenderAAEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x110e0..0x110e6 (decompiled): `getEnumValue` — `getValue`
    // through the `+44` slot `+8`. Same delegation as 0x1055c without the
    // variant wrap.
    (desc.getter)(item)
}

// 0x110e8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x110e8(desc: &RenderAAEnumPropDesc, item: &mut CRenderSettingsItem, value: i32) -> bool {
    // IDA 0x110e8..0x11130 (decompiled): `setEnumValue` — `find_if` with
    // `equalValue` over the enum items (0x11112), miss returns 0 (0x11114),
    // hit sets through the `+44` slot `+12` (0x11126) and returns 1
    // (0x11128). Same shape as 0x10564.
    if antialiasing_mode_enum_desc().pairs.iter().any(|(v, _)| *v == value) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x11134 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x11134(desc: &RenderAAEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x11134..0x11152 (decompiled): `getEnumItem` — `getValue` through
    // the `+44` slot `+8` (0x11146), then
    // `EnumDesc<AntialiasingMode>::convertToItem` (0x11152), which is the
    // identity-table body of 0xda38. Same shape as 0x105b0.
    let value = (desc.getter)(item);
    let table = antialiasing_mode_enum_desc();
    if value >= 0 && (value as usize) < table.pairs.len() {
        value
    } else {
        0
    }
}

// 0x11154 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x11154(desc: &RenderAAEnumPropDesc, item: &mut CRenderSettingsItem, name: &str) -> bool {
    // IDA 0x11154 (decompiled): `setStringValue` on the `Name` —
    // `EnumDesc<AntialiasingMode>::convertToValue`, miss returns 0, hit sets
    // through the `+44` slot `+12` and returns 1. `Name::c_str` collapses
    // into the `&str` itself; same shape as 0x105d0.
    if let Some(value) = antialiasing_mode_enum_desc().lookup_value(name) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x11188 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToIndex(RBX::CRenderSettings::AntialiasingMode)const")]
pub fn stub_0x11188(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0x11188 (decompiled): `EnumDesc<AntialiasingMode>::convertToIndex`
    // — same `ReleaseAssert(value >= 0)` + `value < table ? table[value] :
    // -1` shape as 0x10604 over the value→index remap. The remap is identity
    // over the registered pairs here, so the position search stands in.
    debug_assert!(value >= 0, "0x11188: value>=0");
    desc.pairs.iter().position(|(v, _)| *v == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x111f8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x111f8(desc: &RenderAAEnumPropDesc, item: &mut CRenderSettingsItem, index: i32) -> bool {
    // IDA 0x111f8 (decompiled): `setIntValue` — same `index >= 0` gate /
    // bounds check / `-1`-hole check / `+44` slot `+12` set / return-1 shape
    // as 0x10674 over the AntialiasingMode value table.
    if index >= 0 {
        if let Some((value, _)) = antialiasing_mode_enum_desc().pairs.get(index as usize) {
            if *value != -1 {
                (desc.setter)(item, *value);
                return true;
            }
        }
    }
    false
}

// 0x11238 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isReadOnly(void)const")]
pub fn stub_0x11238() -> bool {
    // IDA 0x11238..0x1123a (decompiled):
    // `GetSetImpl<AntialiasingMode (CRenderSettings::*)() const, void
    // (CRenderSettingsItem::*)(AntialiasingMode)>::isReadOnly` returns `0`.
    // Same shape as 0x106b4.
    false
}

// 0x1123c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isWriteOnly(void)const")]
pub fn stub_0x1123c() -> bool {
    // IDA 0x1123c..0x1123e (decompiled): the AA-member `GetSetImpl`
    // `isWriteOnly` twin — returns `0`. Same shape as 0x106b8.
    false
}

// 0x11240 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x11240(desc: &RenderAAEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x11240..0x1126a (decompiled): `GetSetImpl<...>::getValue` —
    // resolves the bound `AntialiasingMode (CRenderSettings::*)() const`
    // member through the `+4` slot (0x11244..0x11268) and invokes it. The
    // member-pointer dance collapses into the stored getter fn; same shape
    // as 0x106bc.
    (desc.getter)(item)
}

// 0x1126c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AntialiasingMode const&)const")]
pub fn stub_0x1126c(desc: &RenderAAEnumPropDesc, item: &mut CRenderSettingsItem, value: i32) {
    // IDA 0x1126c..0x1128c (decompiled): `GetSetImpl<...>::setValue` —
    // resolves the bound `void (CRenderSettingsItem::*)(AntialiasingMode)`
    // member through the `+12` slot (0x11278..0x11288) and invokes it with
    // `*a3`. Collapses into the stored setter fn; same shape as 0x106e8.
    (desc.setter)(item, value)
}

// 0x11290 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::EnumPropDescriptor<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>(char const*,char const*,RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x11290(
    name: &'static str,
    category: &'static str,
    getter: fn(&CRenderSettingsItem) -> i32,
    setter: fn(&mut CRenderSettingsItem, i32),
) -> RenderShadowEnumPropDesc {
    // IDA 0x11290 (decompiled prologue through the `classDescriptor()` touch,
    // same call shape as 0xfe84..0xfea8):
    // `EnumPropDescriptor<CRenderSettingsItem, ShadowMode>::C2` — same
    // construction shape as the ResolutionPreset twin (0xfe84):
    // `EnumDesc<ShadowMode>` singleton touch, `PropertyDescriptor` C2,
    // enum-table stores, GetSetImpl alloc with the member pair, vtable
    // install. The member pointers collapse into the getter/setter fns.
    let _ = shadow_mode_enum_desc();
    RenderShadowEnumPropDesc { name, category, getter, setter, read_only: false, write_only: false }
}

// 0x11444 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
pub fn stub_0x11444(_desc: *mut RenderShadowEnumPropDesc) {
    // IDA 0x11444..0x11462 (decompiled): `EnumPropDescriptor<...>::D0` —
    // vtable install (0x11458), impl `delete` on the `+44` slot
    // (0x1145a..0x11460), `operator delete`. Same drop-glue shape as
    // 0x10038.
}

// 0x11470 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isReadOnly(void)const")]
pub fn stub_0x11470(desc: &RenderShadowEnumPropDesc) -> bool {
    // IDA 0x11470..0x1147c (decompiled): `isReadOnly` delegates to the
    // `+44` impl slot `+0` query, which returns `0` (cf. 0x10064).
    desc.read_only
}

// 0x11480 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isWriteOnly(void)const")]
pub fn stub_0x11480(desc: &RenderShadowEnumPropDesc) -> bool {
    // IDA 0x11480..0x1148c (decompiled): `isWriteOnly` delegates to the
    // `+44` impl slot `+4` query, which returns `0` (cf. 0x10074).
    desc.write_only
}

// 0x11490 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x11490(
    desc: &RenderShadowEnumPropDesc,
    first: &CRenderSettingsItem,
    second: &CRenderSettingsItem,
) -> bool {
    // IDA 0x11490..0x114b6 (decompiled): `equalValues` — `getValue` through
    // the `+44` slot `+8` on both sides (0x114a0/0x114b6) and compare. Same
    // shape as 0x10084.
    (desc.getter)(first) == (desc.getter)(second)
}

// 0x114b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x114b8(desc: &RenderShadowEnumPropDesc, item: &CRenderSettingsItem) -> IntVariant {
    // IDA 0x114b8..0x114da (decompiled): `getVariant` — `getEnumValue`
    // through vtable `+68` (0x114c6), `Type::getSingleton<int>` tag
    // (0x114cc), `placement_any<int>::operator=` (0x114da). Same shape as
    // 0x100ac.
    IntVariant { value: (desc.getter)(item) }
}

// 0x114dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x114dc(desc: &RenderShadowEnumPropDesc, item: &mut CRenderSettingsItem, variant: &IntVariant) {
    // IDA 0x114dc (decompiled): `setVariant` — same holder-identity int fast
    // path plus generic `Variant::convert<int>` fallback as 0x100d0, then
    // the `+72` setter. Our variant only holds ints, so both paths collapse
    // into the stored setter fn.
    (desc.setter)(item, variant.value)
}

// 0x1162c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x1162c(desc: &RenderShadowEnumPropDesc, dst: &mut CRenderSettingsItem, src: &CRenderSettingsItem) {
    // IDA 0x1162c..0x1164e (decompiled): `copyValue` — `getValue` through
    // the `+44` slot `+8` into a spill (0x1163e), then the `+12` setter
    // (0x1164e). Same shape as 0x10220.
    let value = (desc.getter)(src);
    (desc.setter)(dst, value)
}

// 0x11650 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::hasStringValue(void)const")]
pub fn stub_0x11650() -> bool {
    // IDA 0x11650..0x11652 (decompiled): `hasStringValue` returns `1`. Same
    // shape as 0x10244.
    true
}

// 0x11654 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x11654(desc: &RenderShadowEnumPropDesc, item: &CRenderSettingsItem, out: &mut String) {
    // IDA 0x11654..0x11676 (decompiled): `getStringValue` — `getValue`
    // through the `+44` slot `+8` (0x11666), then
    // `EnumDesc<ShadowMode>::convertToString`, which is the same
    // table-driven body as 0xd28c: empty when out of range. Same shape as
    // 0x10248.
    let value = (desc.getter)(item);
    match (value >= 0).then(|| shadow_mode_enum_desc().lookup_name(value)).flatten() {
        Some(name) => *out = name.to_owned(),
        None => out.clear(),
    }
}

// 0x11678 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x11678(desc: &RenderShadowEnumPropDesc, item: &mut CRenderSettingsItem, name: &str) -> bool {
    // IDA 0x11678..0x116a8 (decompiled): `setStringValue` — `Name::lookup`
    // (0x1168a), `EnumDesc<ShadowMode>::convertToValue` (0x11698), miss
    // returns 0, hit sets through the `+44` slot `+12` and returns 1. Same
    // shape as 0x1026c.
    if let Some(value) = shadow_mode_enum_desc().lookup_value(name) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x116b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x116b8(desc: &RenderShadowEnumPropDesc, item: &CRenderSettingsItem) -> RenderXmlIntValue {
    // IDA 0x116b8..0x116d6 (decompiled): `writeValue` — `getValue` through
    // the `+44` slot `+8` (0x116c6), `clearValue` (0x116cc), type word `5`
    // (0x116d2), int word (0x116d4), return `5` (0x116d6). Same shape as
    // 0x102ac.
    RenderXmlIntValue { kind: 5, int_value: (desc.getter)(item) }
}

// 0x116d8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x116d8(desc: &RenderShadowEnumPropDesc, item: &mut CRenderSettingsItem, input: &RenderXmlInput) {
    // IDA 0x116d8 (decompiled): `readValue` — same `isXsiNil` bail / int
    // (`setIntValue`) / string (`Name::lookup` + `convertToValue` + set with
    // `setStringValue`-mismatch fallback) / `ReleaseAssert(false)` shape as
    // 0x102cc. `setIntValue` for this desc is the direct member set (cf.
    // 0x11a80 shape).
    match input {
        RenderXmlInput::Nil => {}
        RenderXmlInput::Int(value) => (desc.setter)(item, *value),
        RenderXmlInput::Text(name) => {
            if !stub_0x11678(desc, item, name) {
                debug_assert!(false, "0x116d8: false (Reflection.h:359)");
            }
        }
    }
}

// 0x11918 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x11918(desc: &RenderShadowEnumPropDesc, item: &CRenderSettingsItem) -> Option<usize> {
    // IDA 0x11918 (decompiled): `getIndexValue` — `getValue` through the
    // `+44` slot `+8`, then `EnumDesc<ShadowMode>::convertToIndex`: assert
    // plus position search. Same shape as 0x1050c.
    let value = (desc.getter)(item);
    debug_assert!(value >= 0, "0x11918: value>=0");
    shadow_mode_enum_desc().pairs.iter().position(|(v, _)| *v == value)
}

// 0x11934 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x11934(desc: &RenderShadowEnumPropDesc, item: &mut CRenderSettingsItem, index: usize) -> bool {
    // IDA 0x11934..0x1195c (decompiled): `setIndexValue` — `count > index`
    // check against the enum count at `+40` (0x11946), indexed value load
    // from the value table at `+144` (0x11950), `+44` slot `+12` set
    // (0x1195a), return 1 (0x1195c); miss returns 0. The `+144` table holds
    // the values in registration order, so `pairs` stands in. Same shape as
    // 0x110ac.
    match shadow_mode_enum_desc().pairs.get(index) {
        Some((value, _)) => {
            (desc.setter)(item, *value);
            true
        }
        None => false,
    }
}

// 0x11968 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x11968(desc: &RenderShadowEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x11968..0x1196e (decompiled): `getEnumValue` — `getValue`
    // through the `+44` slot `+8`. Same delegation as 0x1055c without the
    // variant wrap.
    (desc.getter)(item)
}

// 0x11970 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x11970(desc: &RenderShadowEnumPropDesc, item: &mut CRenderSettingsItem, value: i32) -> bool {
    // IDA 0x11970..0x119b8 (decompiled): `setEnumValue` — `find_if` with
    // `equalValue` over the enum items, miss returns 0, hit sets through the
    // `+44` slot `+12` and returns 1. Same shape as 0x10564.
    if shadow_mode_enum_desc().pairs.iter().any(|(v, _)| *v == value) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x119bc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x119bc(desc: &RenderShadowEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x119bc..0x119da (decompiled): `getEnumItem` — `getValue` through
    // the `+44` slot `+8` (0x119ce), then
    // `EnumDesc<ShadowMode>::convertToItem` (0x119da), which is the
    // identity-table body of 0xd4f8. Same shape as 0x105b0.
    let value = (desc.getter)(item);
    let table = shadow_mode_enum_desc();
    if value >= 0 && (value as usize) < table.pairs.len() {
        value
    } else {
        0
    }
}

// 0x119dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x119dc(desc: &RenderShadowEnumPropDesc, item: &mut CRenderSettingsItem, name: &str) -> bool {
    // IDA 0x119dc..0x11a0e (decompiled): `setStringValue` on the `Name` —
    // `EnumDesc<ShadowMode>::convertToValue` (0x119f2), miss returns 0
    // (0x119f4), hit sets through the `+44` slot `+12` and returns 1.
    // `Name::c_str` collapses into the `&str` itself; same shape as 0x105d0.
    if let Some(value) = shadow_mode_enum_desc().lookup_value(name) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x11a10 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToIndex(RBX::CRenderSettings::ShadowMode)const")]
pub fn stub_0x11a10(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0x11a10..0x11a7e (decompiled): `EnumDesc<ShadowMode>::convertToIndex`
    // — `ReleaseAssert(value >= 0)` (enumconverter.h:350, 0x11a24..0x11a64)
    // falls through, then `value < table ? table[value] : -1` over the
    // value→index remap. The remap is identity over the registered pairs
    // here, so the position search stands in. Same shape as 0x10604.
    debug_assert!(value >= 0, "0x11a10: value>=0");
    desc.pairs.iter().position(|(v, _)| *v == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x11a80 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x11a80(desc: &RenderShadowEnumPropDesc, item: &mut CRenderSettingsItem, index: i32) -> bool {
    // IDA 0x11a80..0x11abc (decompiled): `setIntValue` — `index >= 0` gate
    // (0x11a8a), bounds check against the value table at `+132`
    // (0x11a8e..0x11a9c), `-1`-hole check (0x11aa8), `+44` slot `+12` set,
    // return 1; miss returns 0. Same shape as 0x10674.
    if index >= 0 {
        if let Some((value, _)) = shadow_mode_enum_desc().pairs.get(index as usize) {
            if *value != -1 {
                (desc.setter)(item, *value);
                return true;
            }
        }
    }
    false
}

// 0x11ac0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isReadOnly(void)const")]
pub fn stub_0x11ac0() -> bool {
    // IDA 0x11ac0..0x11ac2 (decompiled):
    // `GetSetImpl<ShadowMode (CRenderSettings::*)() const, void
    // (CRenderSettingsItem::*)(ShadowMode)>::isReadOnly` returns `0`. Same
    // shape as 0x106b4.
    false
}

// 0x11ac4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isWriteOnly(void)const")]
pub fn stub_0x11ac4() -> bool {
    // IDA 0x11ac4..0x11ac6 (decompiled): the ShadowMode-member `GetSetImpl`
    // `isWriteOnly` twin — returns `0`. Same shape as 0x106b8.
    false
}

// 0x11ac8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x11ac8(desc: &RenderShadowEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x11ac8..0x11af2 (decompiled): `GetSetImpl<...>::getValue` —
    // resolves the bound `ShadowMode (CRenderSettings::*)() const` member
    // through the `+4` slot (0x11acc..0x11af0) and invokes it. The
    // member-pointer dance collapses into the stored getter fn; same shape
    // as 0x106bc.
    (desc.getter)(item)
}

// 0x11af4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ShadowMode const&)const")]
pub fn stub_0x11af4(desc: &RenderShadowEnumPropDesc, item: &mut CRenderSettingsItem, value: i32) {
    // IDA 0x11af4 (decompiled): `GetSetImpl<...>::setValue` — resolves the
    // bound `void (CRenderSettingsItem::*)(ShadowMode)` member through the
    // `+12` slot and invokes it with `*a3`. Collapses into the stored
    // setter fn; same shape as 0x106e8.
    (desc.setter)(item, value)
}

// 0x11b18 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,std::string  CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x11b18() -> ! {
    todo!("0x11b18 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,std::string  CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x11ca8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
pub fn stub_0x11ca8() -> ! {
    todo!("0x11ca8 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")
}

// 0x11cac — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
pub fn stub_0x11cac() -> ! {
    todo!("0x11cac RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")
}

// 0x11cb0 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(std::string *, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x11cb0() -> ! {
    todo!("0x11cb0 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x11cc8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x11cc8() -> ! {
    todo!("0x11cc8 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x11d30 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::EnumPropDescriptor<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>(char const*,char const*,RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x11d30() -> ! {
    todo!("0x11d30 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::EnumPropDescriptor<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>(char const*,char const*,RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x11ee4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
pub fn stub_0x11ee4() -> ! {
    todo!("0x11ee4 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")
}

// 0x11f10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isReadOnly(void)const")]
pub fn stub_0x11f10() -> ! {
    todo!("0x11f10 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isReadOnly(void)const")
}

// 0x11f20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isWriteOnly(void)const")]
pub fn stub_0x11f20() -> ! {
    todo!("0x11f20 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isWriteOnly(void)const")
}

// 0x11f30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x11f30() -> ! {
    todo!("0x11f30 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x11f58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x11f58() -> ! {
    todo!("0x11f58 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x11f7c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x11f7c() -> ! {
    todo!("0x11f7c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x120cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x120cc() -> ! {
    todo!("0x120cc RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x120f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::hasStringValue(void)const")]
pub fn stub_0x120f0() -> ! {
    todo!("0x120f0 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::hasStringValue(void)const")
}

// 0x120f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x120f4() -> ! {
    todo!("0x120f4 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x12118 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x12118() -> ! {
    todo!("0x12118 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x12158 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x12158() -> ! {
    todo!("0x12158 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x12178 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x12178() -> ! {
    todo!("0x12178 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x123b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x123b8() -> ! {
    todo!("0x123b8 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x123d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x123d4() -> ! {
    todo!("0x123d4 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x12408 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x12408() -> ! {
    todo!("0x12408 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x12410 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x12410() -> ! {
    todo!("0x12410 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x1245c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x1245c() -> ! {
    todo!("0x1245c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x1247c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x1247c() -> ! {
    todo!("0x1247c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x124b0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToIndex(RBX::CRenderSettings::AASamples)const")]
pub fn stub_0x124b0() -> ! {
    todo!("0x124b0 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToIndex(RBX::CRenderSettings::AASamples)const")
}

// 0x12520 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x12520() -> ! {
    todo!("0x12520 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x12560 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isReadOnly(void)const")]
pub fn stub_0x12560() -> ! {
    todo!("0x12560 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isReadOnly(void)const")
}

// 0x12564 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isWriteOnly(void)const")]
pub fn stub_0x12564() -> ! {
    todo!("0x12564 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isWriteOnly(void)const")
}

// 0x12568 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x12568() -> ! {
    todo!("0x12568 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x12594 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_0x12594() -> ! {
    todo!("0x12594 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AASamples const&)const")
}

// 0x125b8 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,bool CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x125b8() -> ! {
    todo!("0x125b8 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,bool CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x12748 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
pub fn stub_0x12748() -> ! {
    todo!("0x12748 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")
}

// 0x1274c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
pub fn stub_0x1274c() -> ! {
    todo!("0x1274c RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")
}

// 0x12750 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x12750() -> ! {
    todo!("0x12750 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x1275c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x1275c() -> ! {
    todo!("0x1275c RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x127ac — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x127ac() -> ! {
    todo!("0x127ac RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x128c0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
pub fn stub_0x128c0() -> ! {
    todo!("0x128c0 RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")
}

// 0x128c4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_0x128c4() -> ! {
    todo!("0x128c4 RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")
}

// 0x128c8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x128c8() -> ! {
    todo!("0x128c8 RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x128fc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x128fc() -> ! {
    todo!("0x128fc RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x12920 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::EnumPropDescriptor<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>(char const*,char const*,RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x12920() -> ! {
    todo!("0x12920 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::EnumPropDescriptor<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>(char const*,char const*,RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x12ad4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
pub fn stub_0x12ad4() -> ! {
    todo!("0x12ad4 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")
}

// 0x12b00 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isReadOnly(void)const")]
pub fn stub_0x12b00() -> ! {
    todo!("0x12b00 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isReadOnly(void)const")
}

// 0x12b10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isWriteOnly(void)const")]
pub fn stub_0x12b10() -> ! {
    todo!("0x12b10 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isWriteOnly(void)const")
}

// 0x12b20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x12b20() -> ! {
    todo!("0x12b20 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x12b48 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x12b48() -> ! {
    todo!("0x12b48 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x12b6c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x12b6c() -> ! {
    todo!("0x12b6c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x12cbc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x12cbc() -> ! {
    todo!("0x12cbc RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x12ce0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::hasStringValue(void)const")]
pub fn stub_0x12ce0() -> ! {
    todo!("0x12ce0 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::hasStringValue(void)const")
}

// 0x12ce4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x12ce4() -> ! {
    todo!("0x12ce4 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x12d08 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x12d08() -> ! {
    todo!("0x12d08 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x12d48 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x12d48() -> ! {
    todo!("0x12d48 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x12d68 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x12d68() -> ! {
    todo!("0x12d68 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x12fa8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x12fa8() -> ! {
    todo!("0x12fa8 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x12fc4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x12fc4() -> ! {
    todo!("0x12fc4 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x12ff8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x12ff8() -> ! {
    todo!("0x12ff8 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x13000 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x13000() -> ! {
    todo!("0x13000 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x1304c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x1304c() -> ! {
    todo!("0x1304c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x1306c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x1306c() -> ! {
    todo!("0x1306c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x130a0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToIndex(RBX::CRenderSettings::QualityLevel)const")]
pub fn stub_0x130a0() -> ! {
    todo!("0x130a0 RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToIndex(RBX::CRenderSettings::QualityLevel)const")
}

// 0x13110 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x13110() -> ! {
    todo!("0x13110 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x13150 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isReadOnly(void)const")]
pub fn stub_0x13150() -> ! {
    todo!("0x13150 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isReadOnly(void)const")
}

// 0x13154 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isWriteOnly(void)const")]
pub fn stub_0x13154() -> ! {
    todo!("0x13154 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isWriteOnly(void)const")
}

// 0x13158 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x13158() -> ! {
    todo!("0x13158 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x13184 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::QualityLevel const&)const")]
pub fn stub_0x13184() -> ! {
    todo!("0x13184 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::QualityLevel const&)const")
}

// 0x131a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::EnumPropDescriptor<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>(char const*,char const*,RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x131a8() -> ! {
    todo!("0x131a8 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::EnumPropDescriptor<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>(char const*,char const*,RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

#[cfg(test)]
mod aa_enum_desc_tests {
    use super::*;

    fn get_aa(item: &CRenderSettingsItem) -> i32 {
        item.antialiasing_mode
    }

    fn set_aa(item: &mut CRenderSettingsItem, value: i32) {
        item.antialiasing_mode = value;
    }

    fn aa_desc() -> RenderAAEnumPropDesc {
        stub_0x10a08("AntialiasingMode", "Rendering", get_aa, set_aa)
    }

    #[test]
    fn non_identity_index_remap() {
        let desc = antialiasing_mode_enum_desc();
        assert_eq!(stub_0x11188(desc, 0), 0);
        assert_eq!(stub_0x11188(desc, 2), 1);
        assert_eq!(stub_0x11188(desc, 1), 2);
        assert_eq!(stub_0x11188(desc, 9), -1);
    }

    #[test]
    fn index_value_round_trip() {
        let desc = aa_desc();
        let mut item = CRenderSettingsItem::default();
        assert!(stub_0x110ac(&desc, &mut item, 1));
        assert_eq!(item.antialiasing_mode, 2);
        assert_eq!(stub_0x11090(&desc, &item), Some(1));
        assert!(!stub_0x110ac(&desc, &mut item, 9));
    }

    #[test]
    fn enum_value_item_guards() {
        let desc = aa_desc();
        let mut item = CRenderSettingsItem::default();
        assert!(stub_0x110e8(&desc, &mut item, 1));
        assert_eq!(stub_0x110e0(&desc, &item), 1);
        assert_eq!(stub_0x11134(&desc, &item), 1);
        assert!(!stub_0x110e8(&desc, &mut item, 7));
        item.antialiasing_mode = 9;
        assert_eq!(stub_0x11134(&desc, &item), 0);
        assert!(stub_0x111f8(&desc, &mut item, 0));
        assert_eq!(item.antialiasing_mode, 0);
        assert!(!stub_0x111f8(&desc, &mut item, -1));
    }

    #[test]
    fn string_and_variant_paths() {
        let mut desc = aa_desc();
        let mut item = CRenderSettingsItem::default();
        assert!(stub_0x10df0(&desc, &mut item, "Off"));
        assert_eq!(item.antialiasing_mode, 2);
        assert!(!stub_0x10df0(&desc, &mut item, "Missing"));
        assert!(stub_0x11154(&desc, &mut item, "On"));
        assert_eq!(item.antialiasing_mode, 1);
        let mut out = String::new();
        stub_0x10dcc(&desc, &item, &mut out);
        assert_eq!(out, "On");
        assert_eq!(stub_0x10c30(&desc, &item).value, 1);
        assert!(stub_0x10be8(&desc) == false && stub_0x10bf8(&desc) == false);
        assert!(stub_0x10dc8());
        stub_0x10bbc(&mut desc as *mut RenderAAEnumPropDesc);
    }

    #[test]
    fn shadow_index_remap_and_guards() {
        let desc = shadow_mode_enum_desc();
        assert_eq!(stub_0x11a10(desc, 3), 2);
        assert_eq!(stub_0x11a10(desc, 2), 3);
        assert_eq!(stub_0x11a10(desc, 9), -1);
        let mut d = stub_0x11290(
            "ShadowMode",
            "Rendering",
            |item: &CRenderSettingsItem| item.shadow_mode,
            |item: &mut CRenderSettingsItem, value: i32| {
                item.shadow_mode = value;
            },
        );
        let mut item = CRenderSettingsItem::default();
        assert!(stub_0x11970(&d, &mut item, 3));
        assert_eq!(stub_0x11968(&d, &item), 3);
        assert_eq!(stub_0x11918(&d, &item), Some(2));
        assert!(stub_0x11934(&d, &mut item, 3));
        assert_eq!(item.shadow_mode, 2);
        assert!(!stub_0x11970(&d, &mut item, 7));
        assert!(!stub_0x11a80(&d, &mut item, 9));
        assert!(stub_0x11a80(&d, &mut item, 0));
        assert_eq!(item.shadow_mode, 0);
        assert_eq!(stub_0x11ac8(&d, &item), 0);
        stub_0x11af4(&d, &mut item, 1);
        assert_eq!(item.shadow_mode, 1);
        assert!(!stub_0x11ac0() && !stub_0x11ac4());
        let mut out = String::new();
        stub_0x11654(&d, &item, &mut out);
        assert_eq!(out, "All");
        assert!(stub_0x11470(&d) == false && stub_0x11480(&d) == false);
        assert!(stub_0x11650());
        stub_0x11444(&mut d as *mut RenderShadowEnumPropDesc);
    }

    #[test]
    fn int_getset_impl_direct() {
        let desc = RenderInt32PropDesc {
            name: "N",
            category: "C",
            getter: get_aa,
            setter: set_aa,
        };
        let mut item = CRenderSettingsItem::default();
        stub_0x109e4(&desc, &mut item, 2);
        assert_eq!(stub_0x109b8(&desc, &item), 2);
        assert!(!stub_0x109b0() && !stub_0x109b4());
    }
}
