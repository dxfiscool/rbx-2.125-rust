//! platform generated_175 — next 100 stubs EA-sorted asc global filler continuation after 0x139e0 (global 21202->21302, rbx_core::SharedPtr not boost)
//! Filter: global EA-sorted asc, rbx_core::SharedPtr not boost
//! Batch: 100 stubs EA-sorted asc | skeleton batch | range 0x13a0c..0x16640 (rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::BTreeMap;
use std::sync::LazyLock;
use super::generated_171::{RenderEnumDesc, RenderPropDescriptor, RenderSettingsItem};
use super::generated_172::{REGION_ANY_INT, RegionAny};
use super::generated_173::{stub_1026c, stub_102cc};

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x13a0c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_13a0c(item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x13a0c (`GetSetImpl<FrameRateManagerMode (CRenderSettings::*)()const,
    // void (CRenderSettingsItem::*)(FrameRateManagerMode)>::setValue`):
    // downcast/adjustment, bound member setter. Same shape as 0x106e8 (and
    // the 0x139e0 `getValue` twin in `generated_174`). Family-verified.
    set(item, value)
}

// 0x13a30 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::EnumPropDescriptor<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>(char const*,char const*,RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_13a30(name: &str) -> RenderPropDescriptor {
    // IDA 0x13a30 (`EnumPropDescriptor<GraphicsMode>::C2` with
    // `GraphicsMode (CRenderSettings::*)()const` getter /
    // `void (CRenderSettingsItem::*)(GraphicsMode)` setter): base + enum
    // wiring. Same shape as the ResolutionPreset twin at 0xfe84. Host stores
    // the property name. Family-verified.
    RenderPropDescriptor { prop_name: name.to_string(), extra: None }
}

// 0x13be4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED0Ev
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
pub fn stub_13be4(desc: *mut RenderPropDescriptor) {
    // IDA 0x13be4 (`EnumPropDescriptor<GraphicsMode>::D0`): vtable reset
    // (host nop), `delete a1[11]`, `operator delete(a1)`. Same shape as
    // 0x10038. Family-verified; caller must have come from `Box::into_raw`.
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).extra.take();
        drop(Box::from_raw(desc));
    }
}

// 0x13c10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10isReadOnlyEv
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isReadOnly(void)const")]
pub fn stub_13c10() -> bool {
    // IDA 0x13c10 (`EnumPropDescriptor<GraphicsMode>::isReadOnly`): delegates
    // to the `GetSetImpl` impl, which returns 0. Same as 0x10064.
    // Family-verified.
    false
}

// 0x13c20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11isWriteOnlyEv
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isWriteOnly(void)const")]
pub fn stub_13c20() -> bool {
    // IDA 0x13c20 (`EnumPropDescriptor<GraphicsMode>::isWriteOnly`):
    // delegates to the impl, returns 0. Same as 0x10074. Family-verified.
    false
}

// 0x13c30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13c30(a: i32, b: i32) -> bool {
    // IDA 0x13c30 (`EnumPropDescriptor<GraphicsMode>::equalValues`):
    // `getValue` on both objects, compares the ints. Same as 0x10084.
    // Family-verified.
    a == b
}

// 0x13c58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_13c58(value: i32, out: &mut RegionAny) -> i32 {
    // IDA 0x13c58 (`EnumPropDescriptor<GraphicsMode>::getVariant`):
    // `getEnumValue` wrap + `placement_any` int store (`REGION_ANY_INT`).
    // Same as 0x100ac. Family-verified.
    out.tag = REGION_ANY_INT;
    out.value = value;
    value
}

// 0x13c7c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_13c7c(item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x13c7c (`EnumPropDescriptor<GraphicsMode>::setVariant`):
    // `any_cast<int>`, `setEnumValue`. Same as 0x100d0. Family-verified.
    set(item, value)
}

// 0x13dcc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_13dcc(src: &RenderSettingsItem, dst: &mut RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x13dcc (`EnumPropDescriptor<GraphicsMode>::copyValue`):
    // `getValue(src)`, `setValue(dst, value)`. Same as 0x10220.
    // Family-verified.
    let value = get(src);
    set(dst, value)
}

// 0x13df0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14hasStringValueEv
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::hasStringValue(void)const")]
pub fn stub_13df0() -> bool {
    // IDA 0x13df0 (`EnumPropDescriptor<GraphicsMode>::hasStringValue`):
    // `return 1`. Same as 0x10244. Family-verified.
    true
}

// 0x13df4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14getStringValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13df4(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32, out: &mut String) {
    // IDA 0x13df4 (`EnumPropDescriptor<GraphicsMode>::getStringValue`):
    // `getValue` then by-ref `convertToString` (always assigns). Same as
    // 0x10248. Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    out.clear();
    if let Some(p) = desc.pairs.iter().find(|p| p.value == value) {
        out.push_str(&p.name);
    }
}

// 0x13e18 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_13e18(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, name: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x13e18 (`EnumPropDescriptor<GraphicsMode>::setStringValue`):
    // lookup + convertToValue + setValue, 1/0. Same as 0x1026c.
    // Family-verified.
    stub_1026c(desc, item, name, set)
}

// 0x13e58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_13e58(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> (u32, i32) {
    // IDA 0x13e58 (`EnumPropDescriptor<GraphicsMode>::writeValue`): getValue
    // + `{kind = 5, value}` pair. Same as 0x102ac. Family-verified.
    (5, get(item))
}

// 0x13e78 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_13e78(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, text: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x13e78 (`EnumPropDescriptor<GraphicsMode>::readValue`): element
    // text → int-or-name set path. Same as 0x102cc. Family-verified.
    stub_102cc(desc, item, text, set)
}

// 0x140b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE13getIndexValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_140b8(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x140b8 (`EnumPropDescriptor<GraphicsMode>::getIndexValue`):
    // getValue + convertToIndex (assert :350, miss -1). Same as 0x1050c.
    // Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    desc.pairs.iter().position(|p| p.value == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x140d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE13setIndexValueEPNS0_13DescribedBaseEm
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_140d4(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, index: usize, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x140d4 (`EnumPropDescriptor<GraphicsMode>::setIndexValue`):
    // `count > index` → `table[index]`, `setValue`, 1; else 0. Same as
    // 0x10528. Family-verified.
    match desc.pairs.get(index) {
        Some(p) => {
            set(item, p.value);
            true
        }
        None => false,
    }
}

// 0x14108 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE12getEnumValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_14108(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x14108 (`EnumPropDescriptor<GraphicsMode>::getEnumValue`):
    // `getValue` through the impl. Same as 0x1055c. Family-verified.
    get(item)
}

// 0x14110 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE12setEnumValueEPNS0_13DescribedBaseEi
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_14110(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x14110 (`EnumPropDescriptor<GraphicsMode>::setEnumValue`):
    // `find_if` for `value`, miss 0, hit `setValue` + 1. Same as 0x10564.
    // Family-verified.
    match desc.pairs.iter().find(|p| p.value == value) {
        Some(p) => {
            set(item, p.value);
            true
        }
        None => false,
    }
}

// 0x1415c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11getEnumItemEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1415c(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> Option<i32> {
    // IDA 0x1415c (`EnumPropDescriptor<GraphicsMode>::getEnumItem`):
    // `getValue` then `convertToItem`. Same as 0x105b0. Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0x1417c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_1417c(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, name: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x1417c (`EnumPropDescriptor<GraphicsMode>::setStringValue` with
    // `Name`): same as the `std::string` twin at 0x13e18. Family-verified.
    stub_13e18(desc, item, name, set)
}

// 0x141b0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToIndexES3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToIndexES3_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToIndex(RBX::CRenderSettings::GraphicsMode)const")]
pub fn stub_141b0(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0x141b0 (`EnumDesc<GraphicsMode>::convertToIndex`): `ReleaseAssert`
    // (`value>=0`, `enumconverter.h:350`), value→index lookup, miss -1.
    // Same as 0x10604. Family-verified.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    desc.pairs.iter().position(|p| p.value == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x14220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11setIntValueEPNS0_13DescribedBaseEi
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_14220(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, index: i32, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x14220 (`EnumPropDescriptor<GraphicsMode>::setIntValue`):
    // `index < 0` → 0; table miss → 0; else `setValue` + 1. Same as 0x10674.
    // Family-verified.
    if index < 0 {
        return false;
    }
    match desc.pairs.get(index as usize) {
        Some(p) => {
            set(item, p.value);
            true
        }
        None => false,
    }
}

// 0x14260 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isReadOnly(void)const")]
pub fn stub_14260() -> bool {
    // IDA 0x14260 (`GetSetImpl<GraphicsMode (CRenderSettings::*)()const,
    // void (CRenderSettingsItem::*)(GraphicsMode)>::isReadOnly`): `return 0`.
    // Same as 0x106b4. Family-verified.
    false
}

// 0x14264 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isWriteOnly(void)const")]
pub fn stub_14264() -> bool {
    // IDA 0x14264 (same `GetSetImpl::isWriteOnly`): `return 0`. Same as
    // 0x106b8. Family-verified.
    false
}

// 0x14268 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_14268(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x14268 (same `GetSetImpl::getValue` for the `GraphicsMode` pair):
    // bound member getter. Same shape as 0x106bc. Family-verified.
    get(item)
}

// 0x14294 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_14294(item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x14294 (same `GetSetImpl::setValue`): bound member setter. Same
    // shape as 0x106e8. Family-verified.
    set(item, value)
}

// 0x142b8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// mangled: __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)")]
pub fn stub_142b8<'a>(map: &'a mut BTreeMap<String, i32>, key: &str) -> &'a mut i32 {
    // IDA 0x142b8 (`map<Name const*,ResolutionPreset>::operator[]`): tree
    // search, default-insert on miss, returns the mapped slot. Host
    // `entry().or_default()` (interning folded into the `String` key).
    // Verified via IDA decompile.
    map.entry(key.to_string()).or_default()
}

// 0x14310 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
pub fn stub_14310(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x14310 (`_Rb_tree<Name const*,ResolutionPreset>::_M_insert_unique`
    // with hint): unique insert — miss links the node and returns true, hit
    // returns false. The hint folds (host tree balances itself). Verified
    // via IDA decompile (unique-insert prologue).
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x143c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
pub fn stub_143c4(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> Option<i32> {
    // IDA 0x143c4 (`_Rb_tree<Name const*,ResolutionPreset>::_M_insert`):
    // unconditional node link at the searched position. Host
    // `BTreeMap::insert` (assign + previous). Same family as the 0x14310
    // anchor. Family-verified.
    map.insert(key.to_string(), value)
}

// 0x1441c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
pub fn stub_1441c(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x1441c (same `_M_insert_unique`, hintless): unique insert, true
    // on insert, false on duplicate. Same family as the 0x14310 anchor.
    // Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x14484 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)")]
pub fn stub_14484(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x14484 (`vector<ResolutionPreset>::resize`): shrink moves the
    // finish pointer (POD enum, no destruction); grow calls
    // `_M_fill_insert`. Host `Vec::resize`. Verified via IDA decompile.
    vec.resize(len, value);
}

// 0x144b8 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_144b8(vec: &mut Vec<i32>, value: i32) {
    // IDA 0x144b8 (`vector<ResolutionPreset>::push_back`): grow if full,
    // copy-construct at finish. Host `Vec::push`. Same STL family as the
    // 0xf704 anchor. Family-verified.
    vec.push(value);
}

// 0x144e0 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_144e0(vec: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x144e0 (`vector<ResolutionPreset>::_M_insert_aux`): realloc +
    // shift + insert at `pos`, else shift-right + store. Same as 0xf704
    // (`Vec::insert` covers both paths). Family-verified.
    vec.insert(index, value);
}

// 0x145c4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm
// mangled: __ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)")]
pub fn stub_145c4(count: usize) -> Vec<i32> {
    // IDA 0x145c4 (`_Vector_base<ResolutionPreset>::_M_allocate`): `count >=
    // 0x40000000` throws `bad_alloc`, else `operator new(4 * count)`. Same
    // as 0xf7e8. Family-verified.
    if count >= 0x40000000 {
        panic!("std::bad_alloc");
    }
    Vec::with_capacity(count)
}

// 0x145dc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_
// mangled: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)")]
pub fn stub_145dc(data: &mut Vec<i32>, src_start: usize, src_end: usize, dest_end: usize) -> usize {
    // IDA 0x145dc (`__copy_backward` for `ResolutionPreset*`): backward
    // 4-byte element copy of `[first, last)` to just below `result`. Host
    // `copy_within`. Same as 0xf800. Family-verified.
    let len = src_end - src_start;
    data.copy_within(src_start..src_end, dest_end - len);
    dest_end
}

// 0x14618 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_14618(vec: &mut Vec<i32>, index: usize, count: usize, value: i32) {
    // IDA 0x14618 (`vector<ResolutionPreset>::_M_fill_insert`): realloc +
    // shift + fill `count` copies at `pos`. Host `splice`. Same STL family
    // as the 0xf704 anchor. Family-verified.
    vec.splice(index..index, core::iter::repeat(value).take(count));
}

// 0x147a8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// mangled: __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)")]
pub fn stub_147a8<'a>(map: &'a mut BTreeMap<String, i32>, key: &str) -> &'a mut i32 {
    // IDA 0x147a8 (`map<Name const*,QualityLevel>::operator[]`): search +
    // default-insert, returns the mapped slot. Same as 0x142b8.
    // Family-verified.
    map.entry(key.to_string()).or_default()
}

// 0x14800 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
pub fn stub_14800(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x14800 (`_Rb_tree<Name const*,QualityLevel>::_M_insert_unique`
    // with hint): unique insert, true on insert, false on duplicate. Same
    // as 0x14310. Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x148b4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
pub fn stub_148b4(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> Option<i32> {
    // IDA 0x148b4 (`_Rb_tree<Name const*,QualityLevel>::_M_insert`):
    // unconditional node link. Host `BTreeMap::insert`. Same as 0x143c4.
    // Family-verified.
    map.insert(key.to_string(), value)
}

// 0x1490c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
pub fn stub_1490c(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x1490c (same `_M_insert_unique`, hintless): unique insert. Same
    // as 0x1441c. Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x14974 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)")]
pub fn stub_14974(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x14974 (`vector<QualityLevel>::resize`): shrink moves finish,
    // grow calls `_M_fill_insert`. Same as 0x14484. Family-verified.
    vec.resize(len, value);
}

// 0x149a8 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_149a8(vec: &mut Vec<i32>, value: i32) {
    // IDA 0x149a8 (`vector<QualityLevel>::push_back`). Same as 0x144b8.
    // Family-verified.
    vec.push(value);
}

// 0x149d0 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_149d0(vec: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x149d0 (`vector<QualityLevel>::_M_insert_aux`). Same as 0x144e0
    // (and 0xf704). Family-verified.
    vec.insert(index, value);
}

// 0x14ab4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm
// mangled: __ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)")]
pub fn stub_14ab4(count: usize) -> Vec<i32> {
    // IDA 0x14ab4 (`_Vector_base<QualityLevel>::_M_allocate`). Same as
    // 0x145c4 (and 0xf7e8). Family-verified.
    if count >= 0x40000000 {
        panic!("std::bad_alloc");
    }
    Vec::with_capacity(count)
}

// 0x14acc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_
// mangled: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)")]
pub fn stub_14acc(data: &mut Vec<i32>, src_start: usize, src_end: usize, dest_end: usize) -> usize {
    // IDA 0x14acc (`__copy_backward` for `QualityLevel*`). Same as 0x145dc
    // (and 0xf800). Family-verified.
    let len = src_end - src_start;
    data.copy_within(src_start..src_end, dest_end - len);
    dest_end
}

// 0x14b08 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_14b08(vec: &mut Vec<i32>, index: usize, count: usize, value: i32) {
    // IDA 0x14b08 (`vector<QualityLevel>::_M_fill_insert`). Same as 0x14618.
    // Family-verified.
    vec.splice(index..index, core::iter::repeat(value).take(count));
}

// 0x14c98 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)")]
pub fn stub_14c98(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x14c98 (`vector<ShadowMode>::resize`): shrink moves finish, grow
    // calls `_M_fill_insert`. Same as 0x14484. Family-verified.
    vec.resize(len, value);
}

// 0x14ccc — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_14ccc(vec: &mut Vec<i32>, value: i32) {
    // IDA 0x14ccc (`vector<ShadowMode>::push_back`). Same as 0x144b8.
    // Family-verified.
    vec.push(value);
}

// 0x14cf4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// mangled: __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_14cf4<'a>(map: &'a mut BTreeMap<String, i32>, key: &str) -> &'a mut i32 {
    // IDA 0x14cf4 (`map<Name const*,ShadowMode>::operator[]`): search +
    // default-insert, returns the mapped slot. Same as 0x142b8.
    // Family-verified.
    map.entry(key.to_string()).or_default()
}

// 0x14d4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
pub fn stub_14d4c(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x14d4c (`_Rb_tree<Name const*,ShadowMode>::_M_insert_unique` with
    // hint): unique insert, true on insert, false on duplicate. Same as
    // 0x14310. Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x14e00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
pub fn stub_14e00(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> Option<i32> {
    // IDA 0x14e00 (`_Rb_tree<Name const*,ShadowMode>::_M_insert`):
    // unconditional node link. Host `BTreeMap::insert`. Same as 0x143c4.
    // Family-verified.
    map.insert(key.to_string(), value)
}

// 0x14e58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
pub fn stub_14e58(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x14e58 (same `_M_insert_unique`, hintless): unique insert. Same
    // as 0x1441c. Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x14ec0 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_14ec0(vec: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x14ec0 (`vector<ShadowMode>::_M_insert_aux`). Same as 0x144e0
    // (and 0xf704). Family-verified.
    vec.insert(index, value);
}

// 0x14fa4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings10ShadowModeESaIS2_EE11_M_allocateEm
// mangled: __ZNSt12_Vector_baseIN3RBX15CRenderSettings10ShadowModeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_allocate(unsigned long)")]
pub fn stub_14fa4(count: usize) -> Vec<i32> {
    // IDA 0x14fa4 (`_Vector_base<ShadowMode>::_M_allocate`). Same as 0x145c4
    // (and 0xf7e8). Family-verified.
    if count >= 0x40000000 {
        panic!("std::bad_alloc");
    }
    Vec::with_capacity(count)
}

// 0x14fbc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings10ShadowModeES6_EET0_T_S8_S7_
// mangled: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings10ShadowModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::ShadowMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *>(RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *)")]
pub fn stub_14fbc(data: &mut Vec<i32>, src_start: usize, src_end: usize, dest_end: usize) -> usize {
    // IDA 0x14fbc (`__copy_backward` for `ShadowMode*`). Same as 0x145dc
    // (and 0xf800). Family-verified.
    let len = src_end - src_start;
    data.copy_within(src_start..src_end, dest_end - len);
    dest_end
}

// 0x14ff8 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,unsigned long,RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_14ff8(vec: &mut Vec<i32>, index: usize, count: usize, value: i32) {
    // IDA 0x14ff8 (`vector<ShadowMode>::_M_fill_insert`). Same as 0x14618.
    // Family-verified.
    vec.splice(index..index, core::iter::repeat(value).take(count));
}

// 0x15188 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE6resizeEmS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::resize(unsigned long,RBX::CRenderSettings::AntialiasingMode)")]
pub fn stub_15188(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x15188 (`vector<AntialiasingMode>::resize`): shrink moves finish,
    // grow calls `_M_fill_insert`. Same as 0x14484. Family-verified.
    vec.resize(len, value);
}

// 0x151bc — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE9push_backERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::push_back(RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_151bc(vec: &mut Vec<i32>, value: i32) {
    // IDA 0x151bc (`vector<AntialiasingMode>::push_back`). Same as 0x144b8.
    // Family-verified.
    vec.push(value);
}

// 0x151e4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16AntialiasingModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// mangled: __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16AntialiasingModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AntialiasingMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_151e4<'a>(map: &'a mut BTreeMap<String, i32>, key: &str) -> &'a mut i32 {
    // IDA 0x151e4 (`map<Name const*,AntialiasingMode>::operator[]`): search
    // + default-insert, returns the mapped slot. Same as 0x142b8.
    // Family-verified.
    map.entry(key.to_string()).or_default()
}

// 0x1523c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
pub fn stub_1523c(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x1523c (`_Rb_tree<Name const*,AntialiasingMode>::_M_insert_unique`
    // with hint): unique insert. Same as 0x14310. Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x152f0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
pub fn stub_152f0(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> Option<i32> {
    // IDA 0x152f0 (`_Rb_tree<Name const*,AntialiasingMode>::_M_insert`):
    // unconditional node link. Same as 0x143c4. Family-verified.
    map.insert(key.to_string(), value)
}

// 0x15348 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
pub fn stub_15348(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x15348 (same `_M_insert_unique`, hintless): unique insert. Same
    // as 0x1441c. Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x153b0 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_153b0(vec: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x153b0 (`vector<AntialiasingMode>::_M_insert_aux`). Same as
    // 0x144e0 (and 0xf704). Family-verified.
    vec.insert(index, value);
}

// 0x15494 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE11_M_allocateEm
// mangled: __ZNSt12_Vector_baseIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_allocate(unsigned long)")]
pub fn stub_15494(count: usize) -> Vec<i32> {
    // IDA 0x15494 (`_Vector_base<AntialiasingMode>::_M_allocate`). Same as
    // 0x145c4 (and 0xf7e8). Family-verified.
    if count >= 0x40000000 {
        panic!("std::bad_alloc");
    }
    Vec::with_capacity(count)
}

// 0x154ac — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16AntialiasingModeES6_EET0_T_S8_S7_
// mangled: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16AntialiasingModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::AntialiasingMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *>(RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *)")]
pub fn stub_154ac(data: &mut Vec<i32>, src_start: usize, src_end: usize, dest_end: usize) -> usize {
    // IDA 0x154ac (`__copy_backward` for `AntialiasingMode*`). Same as
    // 0x145dc (and 0xf800). Family-verified.
    let len = src_end - src_start;
    data.copy_within(src_start..src_end, dest_end - len);
    dest_end
}

// 0x154e8 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,unsigned long,RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_154e8(vec: &mut Vec<i32>, index: usize, count: usize, value: i32) {
    // IDA 0x154e8 (`vector<AntialiasingMode>::_M_fill_insert`). Same as
    // 0x14618. Family-verified.
    vec.splice(index..index, core::iter::repeat(value).take(count));
}

// 0x15678 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE6resizeEmS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::resize(unsigned long,RBX::CRenderSettings::FrameRateManagerMode)")]
pub fn stub_15678(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x15678 (`vector<FrameRateManagerMode>::resize`). Same as 0x14484.
    // Family-verified.
    vec.resize(len, value);
}

// 0x156ac — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE9push_backERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::push_back(RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_156ac(vec: &mut Vec<i32>, value: i32) {
    // IDA 0x156ac (`vector<FrameRateManagerMode>::push_back`). Same as
    // 0x144b8. Family-verified.
    vec.push(value);
}

// 0x156d4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings20FrameRateManagerModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// mangled: __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings20FrameRateManagerModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::FrameRateManagerMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_156d4<'a>(map: &'a mut BTreeMap<String, i32>, key: &str) -> &'a mut i32 {
    // IDA 0x156d4 (`map<Name const*,FrameRateManagerMode>::operator[]`).
    // Same as 0x142b8. Family-verified.
    map.entry(key.to_string()).or_default()
}

// 0x1572c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
pub fn stub_1572c(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x1572c (`_Rb_tree<Name const*,FrameRateManagerMode>::
    // _M_insert_unique` with hint). Same as 0x14310. Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x157e0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
pub fn stub_157e0(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> Option<i32> {
    // IDA 0x157e0 (`_Rb_tree<Name const*,FrameRateManagerMode>::_M_insert`).
    // Same as 0x143c4. Family-verified.
    map.insert(key.to_string(), value)
}

// 0x15838 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
pub fn stub_15838(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x15838 (same `_M_insert_unique`, hintless). Same as 0x1441c.
    // Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x158a0 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_158a0(vec: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x158a0 (`vector<FrameRateManagerMode>::_M_insert_aux`). Same as
    // 0x144e0 (and 0xf704). Family-verified.
    vec.insert(index, value);
}

// 0x15984 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE11_M_allocateEm
// mangled: __ZNSt12_Vector_baseIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_allocate(unsigned long)")]
pub fn stub_15984(count: usize) -> Vec<i32> {
    // IDA 0x15984 (`_Vector_base<FrameRateManagerMode>::_M_allocate`). Same
    // as 0x145c4 (and 0xf7e8). Family-verified.
    if count >= 0x40000000 {
        panic!("std::bad_alloc");
    }
    Vec::with_capacity(count)
}

// 0x1599c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings20FrameRateManagerModeES6_EET0_T_S8_S7_
// mangled: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings20FrameRateManagerModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *>(RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *)")]
pub fn stub_1599c(data: &mut Vec<i32>, src_start: usize, src_end: usize, dest_end: usize) -> usize {
    // IDA 0x1599c (`__copy_backward` for `FrameRateManagerMode*`). Same as
    // 0x145dc (and 0xf800). Family-verified.
    let len = src_end - src_start;
    data.copy_within(src_start..src_end, dest_end - len);
    dest_end
}

// 0x159d8 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,unsigned long,RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_159d8(vec: &mut Vec<i32>, index: usize, count: usize, value: i32) {
    // IDA 0x159d8 (`vector<FrameRateManagerMode>::_M_fill_insert`). Same as
    // 0x14618. Family-verified.
    vec.splice(index..index, core::iter::repeat(value).take(count));
}

// 0x15b68 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE6resizeEmS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::resize(unsigned long,RBX::CRenderSettings::GraphicsMode)")]
pub fn stub_15b68(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x15b68 (`vector<GraphicsMode>::resize`). Same as 0x14484.
    // Family-verified.
    vec.resize(len, value);
}

// 0x15b9c — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12GraphicsModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// mangled: __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12GraphicsModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::GraphicsMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_15b9c<'a>(map: &'a mut BTreeMap<String, i32>, key: &str) -> &'a mut i32 {
    // IDA 0x15b9c (`map<Name const*,GraphicsMode>::operator[]`). Same as
    // 0x142b8. Family-verified.
    map.entry(key.to_string()).or_default()
}

// 0x15bf4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
pub fn stub_15bf4(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x15bf4 (`_Rb_tree<Name const*,GraphicsMode>::_M_insert_unique`
    // with hint). Same as 0x14310. Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x15ca8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
pub fn stub_15ca8(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> Option<i32> {
    // IDA 0x15ca8 (`_Rb_tree<Name const*,GraphicsMode>::_M_insert`). Same
    // as 0x143c4. Family-verified.
    map.insert(key.to_string(), value)
}

// 0x15d00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
pub fn stub_15d00(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x15d00 (same `_M_insert_unique`, hintless). Same as 0x1441c.
    // Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x15d68 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,unsigned long,RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_15d68(vec: &mut Vec<i32>, index: usize, count: usize, value: i32) {
    // IDA 0x15d68 (`vector<GraphicsMode>::_M_fill_insert`). Same as 0x14618.
    // Family-verified.
    vec.splice(index..index, core::iter::repeat(value).take(count));
}

// 0x15ef8 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm
// mangled: __ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_allocate(unsigned long)")]
pub fn stub_15ef8(count: usize) -> Vec<i32> {
    // IDA 0x15ef8 (`_Vector_base<GraphicsMode>::_M_allocate`). Same as
    // 0x145c4 (and 0xf7e8). Family-verified.
    if count >= 0x40000000 {
        panic!("std::bad_alloc");
    }
    Vec::with_capacity(count)
}

// 0x15f10 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_
// mangled: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::GraphicsMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *>(RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *)")]
pub fn stub_15f10(data: &mut Vec<i32>, src_start: usize, src_end: usize, dest_end: usize) -> usize {
    // IDA 0x15f10 (`__copy_backward` for `GraphicsMode*`). Same as 0x145dc
    // (and 0xf800). Family-verified.
    let len = src_end - src_start;
    data.copy_within(src_start..src_end, dest_end - len);
    dest_end
}

// 0x15f4c — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::push_back(RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_15f4c(vec: &mut Vec<i32>, value: i32) {
    // IDA 0x15f4c (`vector<GraphicsMode>::push_back`). Same as 0x144b8.
    // Family-verified.
    vec.push(value);
}

// 0x15f74 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_15f74(vec: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x15f74 (`vector<GraphicsMode>::_M_insert_aux`). Same as 0x144e0
    // (and 0xf704). Family-verified.
    vec.insert(index, value);
}

// 0x16058 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::resize(unsigned long,RBX::CRenderSettings::AASamples)")]
pub fn stub_16058(vec: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x16058 (`vector<AASamples>::resize`). Same as 0x14484.
    // Family-verified.
    vec.resize(len, value);
}

// 0x1608c — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::push_back(RBX::CRenderSettings::AASamples const&)")]
pub fn stub_1608c(vec: &mut Vec<i32>, value: i32) {
    // IDA 0x1608c (`vector<AASamples>::push_back`). Same as 0x144b8.
    // Family-verified.
    vec.push(value);
}

// 0x160b4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// mangled: __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AASamples,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::operator[](RBX::Name const* const&)")]
pub fn stub_160b4<'a>(map: &'a mut BTreeMap<String, i32>, key: &str) -> &'a mut i32 {
    // IDA 0x160b4 (`map<Name const*,AASamples>::operator[]`). Same as
    // 0x142b8. Family-verified.
    map.entry(key.to_string()).or_default()
}

// 0x1610c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
pub fn stub_1610c(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x1610c (`_Rb_tree<Name const*,AASamples>::_M_insert_unique` with
    // hint). Same as 0x14310. Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x161c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
pub fn stub_161c0(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> Option<i32> {
    // IDA 0x161c0 (`_Rb_tree<Name const*,AASamples>::_M_insert`). Same as
    // 0x143c4. Family-verified.
    map.insert(key.to_string(), value)
}

// 0x16218 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
pub fn stub_16218(map: &mut BTreeMap<String, i32>, key: &str, value: i32) -> bool {
    // IDA 0x16218 (same `_M_insert_unique`, hintless). Same as 0x1441c.
    // Family-verified.
    match map.entry(key.to_string()) {
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(value);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}

// 0x16280 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,RBX::CRenderSettings::AASamples const&)")]
pub fn stub_16280(vec: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x16280 (`vector<AASamples>::_M_insert_aux`). Same as 0x144e0
    // (and 0xf704). Family-verified.
    vec.insert(index, value);
}

// 0x16364 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm
// mangled: __ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_allocate(unsigned long)")]
pub fn stub_16364(count: usize) -> Vec<i32> {
    // IDA 0x16364 (`_Vector_base<AASamples>::_M_allocate`). Same as 0x145c4
    // (and 0xf7e8). Family-verified.
    if count >= 0x40000000 {
        panic!("std::bad_alloc");
    }
    Vec::with_capacity(count)
}

// 0x1637c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_
// mangled: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::AASamples * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *>(RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *)")]
pub fn stub_1637c(data: &mut Vec<i32>, src_start: usize, src_end: usize, dest_end: usize) -> usize {
    // IDA 0x1637c (`__copy_backward` for `AASamples*`). Same as 0x145dc
    // (and 0xf800). Family-verified.
    let len = src_end - src_start;
    data.copy_within(src_start..src_end, dest_end - len);
    dest_end
}

// 0x163b8 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// mangled: __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,unsigned long,RBX::CRenderSettings::AASamples const&)")]
pub fn stub_163b8(vec: &mut Vec<i32>, index: usize, count: usize, value: i32) {
    // IDA 0x163b8 (`vector<AASamples>::_M_fill_insert`). Same as 0x14618.
    // Family-verified.
    vec.splice(index..index, core::iter::repeat(value).take(count));
}

// 0x16548 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE13initSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE13initSingletonEv
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::initSingleton(void)")]
pub fn stub_1654c() -> &'static RenderEnumDesc {
    // IDA 0x1654c (`Singleton<EnumDesc<ShadowMode> const>::doGetSingleton`):
    // guard-once construction of the static enum descriptor
    // (`EnumDesc<ShadowMode>::EnumDesc` at 0x8c4c). Host folds the guard
    // into a `LazyLock`; the pair table comes from the 0x8c4c ctor.
    // Verified via IDA decompile.
    static DESC: LazyLock<RenderEnumDesc> =
        LazyLock::new(super::generated_171::stub_8c4c);
    &DESC
}

// 0x1654c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE14doGetSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::doGetSingleton(void)")]
pub fn stub_16548() -> &'static RenderEnumDesc {
    // IDA 0x16548 (`Singleton<EnumDesc<ShadowMode> const>::initSingleton`):
    // thunk tail-calling `doGetSingleton` at 0x1654c. Verified via IDA
    // decompile (`// attributes: thunk`).
    stub_1654c()
}

// 0x1663c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE13initSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE13initSingletonEv
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::initSingleton(void)")]
pub fn stub_16640() -> &'static RenderEnumDesc {
    // IDA 0x16640 (`Singleton<EnumDesc<ResolutionPreset>
    // const>::doGetSingleton`): guard-once construction of the static enum
    // descriptor (`EnumDesc<ResolutionPreset>::EnumDesc` at 0x9100). Same
    // shape as the ShadowMode twin at 0x1654c. Family-verified.
    static DESC: LazyLock<RenderEnumDesc> =
        LazyLock::new(super::generated_171::stub_9100);
    &DESC
}

// 0x16640 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE14doGetSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::doGetSingleton(void)")]
pub fn stub_1663c() -> &'static RenderEnumDesc {
    // IDA 0x1663c (`Singleton<EnumDesc<ResolutionPreset>
    // const>::initSingleton`): thunk tail-calling `doGetSingleton` at
    // 0x16640. Same as 0x16548. Family-verified.
    stub_16640()
}
