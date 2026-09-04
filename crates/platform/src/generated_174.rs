//! platform generated_174 — next 100 stubs EA-sorted asc global filler continuation after 0x11918 (global 21102->21202, rbx_core::SharedPtr not boost)
//! Filter: global EA-sorted asc, rbx_core::SharedPtr not boost
//! Batch: 100 stubs EA-sorted asc | skeleton batch | range 0x11934..0x139e0 (rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use super::generated_171::{RenderEnumDesc, RenderPropDescriptor, RenderSettingsItem};
use super::generated_172::{REGION_ANY_INT, RegionAny};
use super::generated_173::{stub_1026c, stub_102cc, stub_11678};

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x11934 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13setIndexValueEPNS0_13DescribedBaseEm
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_11934(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, index: usize, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x11934 (`EnumPropDescriptor<ShadowMode>::setIndexValue`): `count >
    // index` loads `table[index]`, `setValue`, returns 1; else 0. Verified
    // via IDA decompile (same shape as the ResolutionPreset twin at 0x10528).
    match desc.pairs.get(index) {
        Some(p) => {
            set(item, p.value);
            true
        }
        None => false,
    }
}

// 0x11968 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12getEnumValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11968(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x11968 (`EnumPropDescriptor<ShadowMode>::getEnumValue`): `getValue`
    // through the impl. Same as 0x1055c. Family-verified.
    get(item)
}

// 0x11970 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12setEnumValueEPNS0_13DescribedBaseEi
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_11970(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x11970 (`EnumPropDescriptor<ShadowMode>::setEnumValue`): `find_if`
    // over the item table for `value`, miss returns 0, hit calls `setValue`
    // and returns 1. Same as 0x10564. Family-verified.
    match desc.pairs.iter().find(|p| p.value == value) {
        Some(p) => {
            set(item, p.value);
            true
        }
        None => false,
    }
}

// 0x119bc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11getEnumItemEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_119bc(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> Option<i32> {
    // IDA 0x119bc (`EnumPropDescriptor<ShadowMode>::getEnumItem`): `getValue`
    // then `EnumDesc::convertToItem`. Host returns the value where the
    // original returns the item. Same as 0x105b0. Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0x119dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_119dc(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, name: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x119dc (`EnumPropDescriptor<ShadowMode>::setStringValue` with
    // `Name`): `EnumDesc::convertToValue`, hit calls `setValue` and returns
    // 1, else 0. Same as the `std::string` twin at 0x11678. Family-verified.
    stub_11678(desc, item, name, set)
}

// 0x11a10 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToIndex(RBX::CRenderSettings::ShadowMode)const")]
pub fn stub_11a10(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0x11a10 (`EnumDesc<ShadowMode>::convertToIndex`): `ReleaseAssert`
    // (`value>=0`, `enumconverter.h:350`), value→index table lookup, miss
    // returns -1. Same as 0x10604. Family-verified.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    desc.pairs.iter().position(|p| p.value == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x11a80 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_11a80(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, index: i32, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x11a80 (`EnumPropDescriptor<ShadowMode>::setIntValue`): `index <
    // 0` → 0; index→value table miss → 0; else `setValue` and 1. Same as
    // 0x10674. Family-verified.
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

// 0x11ac0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isReadOnly(void)const")]
pub fn stub_11ac0() -> bool {
    // IDA 0x11ac0 (`GetSetImpl<ShadowMode (CRenderSettings::*)()const,
    // void (CRenderSettingsItem::*)(ShadowMode)>::isReadOnly`): `return 0` —
    // getter and setter both present. Same as 0x106b4. Family-verified.
    false
}

// 0x11ac4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isWriteOnly(void)const")]
pub fn stub_11ac4() -> bool {
    // IDA 0x11ac4 (same `GetSetImpl::isWriteOnly`): `return 0`. Same as
    // 0x106b8. Family-verified.
    false
}

// 0x11ac8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11ac8(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x11ac8 (same `GetSetImpl::getValue` for the `ShadowMode` pair):
    // downcast + base adjustments, bound member getter. Same shape as
    // 0x106bc. Family-verified.
    get(item)
}

// 0x11af4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ShadowMode const&)const")]
pub fn stub_11af4(item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x11af4 (same `GetSetImpl::setValue`): downcast/adjustment, bound
    // member setter. Same shape as 0x106e8. Family-verified.
    set(item, value)
}

// 0x11b18 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// mangled: __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,std::string  CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_11b18(name: &str) -> RenderPropDescriptor {
    // IDA 0x11b18 (`BoundProp<std::string>::C2` with
    // `std::string CRenderSettingsItem::*` member): base descriptor init
    // (names, attributes, permissions) + `BoundPropGetSet` member-offset
    // install. Host stores the property name; the member offset folds into
    // the caller's `fn` pointers (cf. 0x11cb0/0x11cc8). Verified via IDA
    // decompile (ctor prologue).
    RenderPropDescriptor { prop_name: name.to_string(), extra: None }
}

// 0x11ca8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
// mangled: __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
pub fn stub_11ca8() -> bool {
    // IDA 0x11ca8 (`BoundPropGetSet<std::string,CRenderSettingsItem>::
    // isReadOnly`): `return 0`. Verified via IDA decompile.
    false
}

// 0x11cac — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
// mangled: __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
pub fn stub_11cac() -> bool {
    // IDA 0x11cac (same `BoundPropGetSet::isWriteOnly`): `return 0`.
    // Verified via IDA decompile.
    false
}

// 0x11cb0 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(std::string *, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11cb0(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> String) -> String {
    // IDA 0x11cb0 (`BoundPropGetSet<std::string,CRenderSettingsItem>::
    // getValue`): `std::string` copy from the bound member (`this - 36`
    // base adjustment folded on the host). Verified via IDA decompile.
    get(item)
}

// 0x11cc8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKSs
// mangled: __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_11cc8(item: &mut RenderSettingsItem, value: String, prop: u32, get: fn(&RenderSettingsItem) -> String, set: fn(&mut RenderSettingsItem, String)) {
    // IDA 0x11cc8 (same `BoundPropGetSet::setValue`): `compare` equal →
    // no-op; else `assign` + `raisePropertyChanged`. Verified via IDA
    // decompile.
    if get(item) == value {
        return;
    }
    set(item, value);
    item.property_changed.fire(prop);
}

// 0x11d30 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::EnumPropDescriptor<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>(char const*,char const*,RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_11d30(name: &str) -> RenderPropDescriptor {
    // IDA 0x11d30 (`EnumPropDescriptor<AASamples>::C2` with
    // `AASamples (CRenderSettings::*)()const` getter /
    // `void (CRenderSettingsItem::*)(AASamples)` setter): base + enum
    // wiring. Same shape as the ResolutionPreset twin at 0xfe84. Host stores
    // the property name. Verified via IDA decompile (ctor prologue).
    RenderPropDescriptor { prop_name: name.to_string(), extra: None }
}

// 0x11ee4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
pub fn stub_11ee4(desc: *mut RenderPropDescriptor) {
    // IDA 0x11ee4 (`EnumPropDescriptor<AASamples>::D0`): vtable reset (host
    // nop), `delete a1[11]` (the impl box), `operator delete(a1)`. Same shape
    // as 0x10038. Family-verified; caller must have come from `Box::into_raw`.
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).extra.take();
        drop(Box::from_raw(desc));
    }
}

// 0x11f10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isReadOnly(void)const")]
pub fn stub_11f10() -> bool {
    // IDA 0x11f10 (`EnumPropDescriptor<AASamples>::isReadOnly`): delegates to
    // the `GetSetImpl` impl, which returns 0. Same as 0x10064.
    // Family-verified.
    false
}

// 0x11f20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isWriteOnly(void)const")]
pub fn stub_11f20() -> bool {
    // IDA 0x11f20 (`EnumPropDescriptor<AASamples>::isWriteOnly`): delegates
    // to the impl, returns 0. Same as 0x10074. Family-verified.
    false
}

// 0x11f30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11f30(a: i32, b: i32) -> bool {
    // IDA 0x11f30 (`EnumPropDescriptor<AASamples>::equalValues`): `getValue`
    // on both objects, compares the ints. Same as 0x10084. Family-verified.
    a == b
}

// 0x11f58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_11f58(value: i32, out: &mut RegionAny) -> i32 {
    // IDA 0x11f58 (`EnumPropDescriptor<AASamples>::getVariant`):
    // `getEnumValue`, `Type::getSingleton<int>` wrap, `placement_any` store.
    // Host stores the int payload (`REGION_ANY_INT`). Same as 0x100ac.
    // Family-verified.
    out.tag = REGION_ANY_INT;
    out.value = value;
    value
}

// 0x11f7c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_11f7c(item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x11f7c (`EnumPropDescriptor<AASamples>::setVariant`):
    // `any_cast<int>`, `setEnumValue`. Host applies the setter with the
    // value. Same as 0x100d0. Family-verified.
    set(item, value)
}

// 0x120cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9copyValueEPKNS0_13DescribedBaseEPS6_
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_120cc(src: &RenderSettingsItem, dst: &mut RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x120cc (`EnumPropDescriptor<AASamples>::copyValue`):
    // `getValue(src)`, `setValue(dst, value)`. Same as 0x10220.
    // Family-verified.
    let value = get(src);
    set(dst, value)
}

// 0x120f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14hasStringValueEv
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::hasStringValue(void)const")]
pub fn stub_120f0() -> bool {
    // IDA 0x120f0 (`EnumPropDescriptor<AASamples>::hasStringValue`):
    // `return 1` — enums always have a string form. Same as 0x10244.
    // Family-verified.
    true
}

// 0x120f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14getStringValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14getStringValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_120f4(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32, out: &mut String) {
    // IDA 0x120f4 (`EnumPropDescriptor<AASamples>::getStringValue`):
    // `getValue` then `EnumDesc::convertToString` by-ref (always assigns).
    // Same as 0x10248. Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    out.clear();
    if let Some(p) = desc.pairs.iter().find(|p| p.value == value) {
        out.push_str(&p.name);
    }
}

// 0x12118 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKSs
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_12118(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, name: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x12118 (`EnumPropDescriptor<AASamples>::setStringValue`):
    // lookup + convertToValue + setValue, 1/0. Same as 0x1026c.
    // Family-verified.
    stub_1026c(desc, item, name, set)
}

// 0x12158 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_12158(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> (u32, i32) {
    // IDA 0x12158 (`EnumPropDescriptor<AASamples>::writeValue`): getValue +
    // `{kind = 5, value}` pair. Same as 0x102ac. Family-verified.
    (5, get(item))
}

// 0x12178 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_12178(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, text: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x12178 (`EnumPropDescriptor<AASamples>::readValue`): element text
    // → int-or-name set path. Same as 0x102cc. Family-verified.
    stub_102cc(desc, item, text, set)
}

// 0x123b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13getIndexValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13getIndexValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_123b8(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x123b8 (`EnumPropDescriptor<AASamples>::getIndexValue`): getValue
    // + convertToIndex (assert :350, miss -1). Same as 0x1050c.
    // Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    desc.pairs.iter().position(|p| p.value == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x123d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13setIndexValueEPNS0_13DescribedBaseEm
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13setIndexValueEPNS0_13DescribedBaseEm
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_123d4(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, index: usize, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x123d4 (`EnumPropDescriptor<AASamples>::setIndexValue`): `count >
    // index` loads `table[index]`, `setValue`, returns 1; else 0. Same as
    // 0x10528. Family-verified.
    match desc.pairs.get(index) {
        Some(p) => {
            set(item, p.value);
            true
        }
        None => false,
    }
}

// 0x12408 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12getEnumValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12getEnumValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12408(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x12408 (`EnumPropDescriptor<AASamples>::getEnumValue`): `getValue`
    // through the impl. Same as 0x1055c. Family-verified.
    get(item)
}

// 0x12410 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12setEnumValueEPNS0_13DescribedBaseEi
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12setEnumValueEPNS0_13DescribedBaseEi
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_12410(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x12410 (`EnumPropDescriptor<AASamples>::setEnumValue`): `find_if`
    // for `value`, miss returns 0, hit calls `setValue` and returns 1. Same
    // as 0x10564. Family-verified.
    match desc.pairs.iter().find(|p| p.value == value) {
        Some(p) => {
            set(item, p.value);
            true
        }
        None => false,
    }
}

// 0x1245c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11getEnumItemEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11getEnumItemEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1245c(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> Option<i32> {
    // IDA 0x1245c (`EnumPropDescriptor<AASamples>::getEnumItem`): `getValue`
    // then `EnumDesc::convertToItem`. Same as 0x105b0. Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0x1247c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_1247c(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, name: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x1247c (`EnumPropDescriptor<AASamples>::setStringValue` with
    // `Name`): same as the `std::string` twin at 0x12118. Family-verified.
    stub_12118(desc, item, name, set)
}

// 0x124b0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToIndex(RBX::CRenderSettings::AASamples)const")]
pub fn stub_124b0(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0x124b0 (`EnumDesc<AASamples>::convertToIndex`): `ReleaseAssert`
    // (`value>=0`, `enumconverter.h:350`), value→index lookup, miss -1.
    // Same as 0x10604. Family-verified.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    desc.pairs.iter().position(|p| p.value == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x12520 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_12520(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, index: i32, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x12520 (`EnumPropDescriptor<AASamples>::setIntValue`): `index <
    // 0` → 0; table miss → 0; else `setValue` and 1. Same as 0x10674.
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

// 0x12560 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isReadOnly(void)const")]
pub fn stub_12560() -> bool {
    // IDA 0x12560 (`GetSetImpl<AASamples (CRenderSettings::*)()const,
    // void (CRenderSettingsItem::*)(AASamples)>::isReadOnly`): `return 0`.
    // Same as 0x106b4. Family-verified.
    false
}

// 0x12564 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isWriteOnly(void)const")]
pub fn stub_12564() -> bool {
    // IDA 0x12564 (same `GetSetImpl::isWriteOnly`): `return 0`. Same as
    // 0x106b8. Family-verified.
    false
}

// 0x12568 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12568(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x12568 (same `GetSetImpl::getValue` for the `AASamples` pair):
    // bound member getter. Same shape as 0x106bc. Family-verified.
    get(item)
}

// 0x12594 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_12594(item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x12594 (same `GetSetImpl::setValue`): bound member setter. Same
    // shape as 0x106e8. Family-verified.
    set(item, value)
}

// 0x125b8 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// mangled: __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,bool CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_125b8(name: &str) -> RenderPropDescriptor {
    // IDA 0x125b8 (`BoundProp<bool>::C2` with
    // `bool CRenderSettingsItem::*` member): base descriptor init +
    // `BoundPropGetSet` member-offset install. Same shape as the string twin
    // at 0x11b18. Host stores the property name. Family-verified.
    RenderPropDescriptor { prop_name: name.to_string(), extra: None }
}

// 0x12748 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
// mangled: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
pub fn stub_12748() -> bool {
    // IDA 0x12748 (`BoundPropGetSet<bool,CRenderSettingsItem>::isReadOnly`):
    // `return 0`. Verified via IDA decompile.
    false
}

// 0x1274c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
// mangled: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
pub fn stub_1274c() -> bool {
    // IDA 0x1274c (same `BoundPropGetSet::isWriteOnly`): `return 0`. Same as
    // the string twin at 0x11cac. Family-verified.
    false
}

// 0x12750 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12750(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> bool) -> bool {
    // IDA 0x12750 (`BoundPropGetSet<bool,CRenderSettingsItem>::getValue`):
    // byte load from the bound member (`this - 36` adjustment folded).
    // Verified via IDA decompile.
    get(item)
}

// 0x1275c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKb
// mangled: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKb
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_1275c(item: &mut RenderSettingsItem, value: bool, prop: u32, get: fn(&RenderSettingsItem) -> bool, set: fn(&mut RenderSettingsItem, bool)) {
    // IDA 0x1275c (same `BoundPropGetSet::setValue`): equal → no-op; else
    // byte store + `raisePropertyChanged`. Same shape as the string twin at
    // 0x11cc8. Verified via IDA decompile.
    if get(item) == value {
        return;
    }
    set(item, value);
    item.property_changed.fire(prop);
}

// 0x127ac — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// mangled: __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_127ac(name: &str) -> RenderPropDescriptor {
    // IDA 0x127ac (`PropDescriptor<bool>::C2` with
    // `bool (CRenderSettings::*)()const` getter /
    // `void (CRenderSettingsItem::*)(bool)` setter): base descriptor init +
    // `GetSetImpl` pair install. Same shape as the bool twin at 0x1070c.
    // Host stores the property name. Family-verified.
    RenderPropDescriptor { prop_name: name.to_string(), extra: None }
}

// 0x128c0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE10isReadOnlyEv
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE10isReadOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
pub fn stub_128c0() -> bool {
    // IDA 0x128c0 (`GetSetImpl<bool (CRenderSettings::*)()const,
    // void (CRenderSettingsItem::*)(bool)>::isReadOnly`): `return 0`. Same
    // as 0x1084c. Family-verified.
    false
}

// 0x128c4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE11isWriteOnlyEv
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE11isWriteOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_128c4() -> bool {
    // IDA 0x128c4 (same `GetSetImpl::isWriteOnly`): `return 0`. Same as
    // 0x10850. Family-verified.
    false
}

// 0x128c8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_128c8(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> bool) -> bool {
    // IDA 0x128c8 (same `GetSetImpl::getValue` for the bool pair): bound
    // member getter. Same shape as 0x10854. Family-verified.
    get(item)
}

// 0x128fc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_128fc(item: &mut RenderSettingsItem, value: bool, set: fn(&mut RenderSettingsItem, bool)) {
    // IDA 0x128fc (same `GetSetImpl::setValue`): bound member setter. Same
    // shape as 0x10878. Family-verified.
    set(item, value)
}

// 0x12920 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::EnumPropDescriptor<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>(char const*,char const*,RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_12920(name: &str) -> RenderPropDescriptor {
    // IDA 0x12920 (`EnumPropDescriptor<QualityLevel>::C2` with
    // `QualityLevel (CRenderSettings::*)()const` getter /
    // `void (CRenderSettingsItem::*)(QualityLevel)` setter): base + enum
    // wiring. Same shape as the ResolutionPreset twin at 0xfe84. Host stores
    // the property name. Family-verified.
    RenderPropDescriptor { prop_name: name.to_string(), extra: None }
}

// 0x12ad4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED0Ev
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED0Ev
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
pub fn stub_12ad4(desc: *mut RenderPropDescriptor) {
    // IDA 0x12ad4 (`EnumPropDescriptor<QualityLevel>::D0`): vtable reset
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

// 0x12b00 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10isReadOnlyEv
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10isReadOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isReadOnly(void)const")]
pub fn stub_12b00() -> bool {
    // IDA 0x12b00 (`EnumPropDescriptor<QualityLevel>::isReadOnly`): delegates
    // to the `GetSetImpl` impl, which returns 0. Same as 0x10064.
    // Family-verified.
    false
}

// 0x12b10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11isWriteOnlyEv
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11isWriteOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isWriteOnly(void)const")]
pub fn stub_12b10() -> bool {
    // IDA 0x12b10 (`EnumPropDescriptor<QualityLevel>::isWriteOnly`):
    // delegates to the impl, returns 0. Same as 0x10074. Family-verified.
    false
}

// 0x12b20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11equalValuesEPKNS0_13DescribedBaseES8_
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12b20(a: i32, b: i32) -> bool {
    // IDA 0x12b20 (`EnumPropDescriptor<QualityLevel>::equalValues`):
    // `getValue` on both objects, compares the ints. Same as 0x10084.
    // Family-verified.
    a == b
}

// 0x12b48 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_12b48(value: i32, out: &mut RegionAny) -> i32 {
    // IDA 0x12b48 (`EnumPropDescriptor<QualityLevel>::getVariant`):
    // `getEnumValue` wrap + `placement_any` int store (`REGION_ANY_INT`).
    // Same as 0x100ac. Family-verified.
    out.tag = REGION_ANY_INT;
    out.value = value;
    value
}

// 0x12b6c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_12b6c(item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x12b6c (`EnumPropDescriptor<QualityLevel>::setVariant`):
    // `any_cast<int>`, `setEnumValue`. Same as 0x100d0. Family-verified.
    set(item, value)
}

// 0x12cbc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9copyValueEPKNS0_13DescribedBaseEPS6_
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_12cbc(src: &RenderSettingsItem, dst: &mut RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x12cbc (`EnumPropDescriptor<QualityLevel>::copyValue`):
    // `getValue(src)`, `setValue(dst, value)`. Same as 0x10220.
    // Family-verified.
    let value = get(src);
    set(dst, value)
}

// 0x12ce0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14hasStringValueEv
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14hasStringValueEv
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::hasStringValue(void)const")]
pub fn stub_12ce0() -> bool {
    // IDA 0x12ce0 (`EnumPropDescriptor<QualityLevel>::hasStringValue`):
    // `return 1`. Same as 0x10244. Family-verified.
    true
}

// 0x12ce4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14getStringValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14getStringValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12ce4(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32, out: &mut String) {
    // IDA 0x12ce4 (`EnumPropDescriptor<QualityLevel>::getStringValue`):
    // `getValue` then by-ref `convertToString` (always assigns). Same as
    // 0x10248. Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    out.clear();
    if let Some(p) = desc.pairs.iter().find(|p| p.value == value) {
        out.push_str(&p.name);
    }
}

// 0x12d08 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKSs
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_12d08(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, name: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x12d08 (`EnumPropDescriptor<QualityLevel>::setStringValue`):
    // lookup + convertToValue + setValue, 1/0. Same as 0x1026c.
    // Family-verified.
    stub_1026c(desc, item, name, set)
}

// 0x12d48 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_12d48(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> (u32, i32) {
    // IDA 0x12d48 (`EnumPropDescriptor<QualityLevel>::writeValue`): getValue
    // + `{kind = 5, value}` pair. Same as 0x102ac. Family-verified.
    (5, get(item))
}

// 0x12d68 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_12d68(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, text: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x12d68 (`EnumPropDescriptor<QualityLevel>::readValue`): element
    // text → int-or-name set path. Same as 0x102cc. Family-verified.
    stub_102cc(desc, item, text, set)
}

// 0x12fa8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13getIndexValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13getIndexValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12fa8(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x12fa8 (`EnumPropDescriptor<QualityLevel>::getIndexValue`):
    // getValue + convertToIndex (assert :350, miss -1). Same as 0x1050c.
    // Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    desc.pairs.iter().position(|p| p.value == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x12fc4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13setIndexValueEPNS0_13DescribedBaseEm
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13setIndexValueEPNS0_13DescribedBaseEm
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_12fc4(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, index: usize, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x12fc4 (`EnumPropDescriptor<QualityLevel>::setIndexValue`):
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

// 0x12ff8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12getEnumValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12getEnumValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12ff8(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x12ff8 (`EnumPropDescriptor<QualityLevel>::getEnumValue`):
    // `getValue` through the impl. Same as 0x1055c. Family-verified.
    get(item)
}

// 0x13000 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12setEnumValueEPNS0_13DescribedBaseEi
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12setEnumValueEPNS0_13DescribedBaseEi
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13000(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x13000 (`EnumPropDescriptor<QualityLevel>::setEnumValue`):
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

// 0x1304c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11getEnumItemEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11getEnumItemEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1304c(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> Option<i32> {
    // IDA 0x1304c (`EnumPropDescriptor<QualityLevel>::getEnumItem`):
    // `getValue` then `convertToItem`. Same as 0x105b0. Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0x1306c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: 
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_1306c(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, name: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x1306c (`EnumPropDescriptor<QualityLevel>::setStringValue` with
    // `Name`): same as the `std::string` twin at 0x12d08. Family-verified.
    stub_12d08(desc, item, name, set)
}

// 0x130a0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToIndexES3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToIndex(RBX::CRenderSettings::QualityLevel)const")]
pub fn stub_130a0(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0x130a0 (`EnumDesc<QualityLevel>::convertToIndex`): `ReleaseAssert`
    // (`value>=0`, `enumconverter.h:350`), value→index lookup, miss -1.
    // Same as 0x10604. Family-verified.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    desc.pairs.iter().position(|p| p.value == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x13110 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11setIntValueEPNS0_13DescribedBaseEi
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13110(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, index: i32, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x13110 (`EnumPropDescriptor<QualityLevel>::setIntValue`):
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

// 0x13150 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isReadOnly(void)const")]
pub fn stub_13150() -> bool {
    // IDA 0x13150 (`GetSetImpl<QualityLevel (CRenderSettings::*)()const,
    // void (CRenderSettingsItem::*)(QualityLevel)>::isReadOnly`): `return 0`.
    // Same as 0x106b4. Family-verified.
    false
}

// 0x13154 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isWriteOnly(void)const")]
pub fn stub_13154() -> bool {
    // IDA 0x13154 (same `GetSetImpl::isWriteOnly`): `return 0`. Same as
    // 0x106b8. Family-verified.
    false
}

// 0x13158 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13158(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x13158 (same `GetSetImpl::getValue` for the `QualityLevel` pair):
    // bound member getter. Same shape as 0x106bc. Family-verified.
    get(item)
}

// 0x13184 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::QualityLevel const&)const")]
pub fn stub_13184(item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x13184 (same `GetSetImpl::setValue`): bound member setter. Same
    // shape as 0x106e8. Family-verified.
    set(item, value)
}

// 0x131a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::EnumPropDescriptor<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>(char const*,char const*,RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_131a8(name: &str) -> RenderPropDescriptor {
    // IDA 0x131a8 (`EnumPropDescriptor<FrameRateManagerMode>::C2` with
    // `FrameRateManagerMode (CRenderSettings::*)()const` getter /
    // `void (CRenderSettingsItem::*)(FrameRateManagerMode)` setter): base +
    // enum wiring. Same shape as the ResolutionPreset twin at 0xfe84. Host
    // stores the property name. Family-verified.
    RenderPropDescriptor { prop_name: name.to_string(), extra: None }
}

// 0x1335c — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED0Ev
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
pub fn stub_1335c(desc: *mut RenderPropDescriptor) {
    // IDA 0x1335c (`EnumPropDescriptor<FrameRateManagerMode>::D0`): vtable
    // reset (host nop), `delete a1[11]`, `operator delete(a1)`. Same shape
    // as 0x10038. Family-verified; caller must have come from `Box::into_raw`.
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).extra.take();
        drop(Box::from_raw(desc));
    }
}

// 0x13388 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10isReadOnlyEv
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isReadOnly(void)const")]
pub fn stub_13388() -> bool {
    // IDA 0x13388 (`EnumPropDescriptor<FrameRateManagerMode>::isReadOnly`):
    // delegates to the `GetSetImpl` impl, which returns 0. Same as 0x10064.
    // Family-verified.
    false
}

// 0x13398 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11isWriteOnlyEv
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isWriteOnly(void)const")]
pub fn stub_13398() -> bool {
    // IDA 0x13398 (`EnumPropDescriptor<FrameRateManagerMode>::isWriteOnly`):
    // delegates to the impl, returns 0. Same as 0x10074. Family-verified.
    false
}

// 0x133a8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_133a8(a: i32, b: i32) -> bool {
    // IDA 0x133a8 (`EnumPropDescriptor<FrameRateManagerMode>::equalValues`):
    // `getValue` on both objects, compares the ints. Same as 0x10084.
    // Family-verified.
    a == b
}

// 0x133d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_133d0(value: i32, out: &mut RegionAny) -> i32 {
    // IDA 0x133d0 (`EnumPropDescriptor<FrameRateManagerMode>::getVariant`):
    // `getEnumValue` wrap + `placement_any` int store (`REGION_ANY_INT`).
    // Same as 0x100ac. Family-verified.
    out.tag = REGION_ANY_INT;
    out.value = value;
    value
}

// 0x133f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_133f4(item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x133f4 (`EnumPropDescriptor<FrameRateManagerMode>::setVariant`):
    // `any_cast<int>`, `setEnumValue`. Same as 0x100d0. Family-verified.
    set(item, value)
}

// 0x13544 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_13544(src: &RenderSettingsItem, dst: &mut RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32, set: fn(&mut RenderSettingsItem, i32)) {
    // IDA 0x13544 (`EnumPropDescriptor<FrameRateManagerMode>::copyValue`):
    // `getValue(src)`, `setValue(dst, value)`. Same as 0x10220.
    // Family-verified.
    let value = get(src);
    set(dst, value)
}

// 0x13568 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14hasStringValueEv
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::hasStringValue(void)const")]
pub fn stub_13568() -> bool {
    // IDA 0x13568 (`EnumPropDescriptor<FrameRateManagerMode>::hasStringValue`):
    // `return 1`. Same as 0x10244. Family-verified.
    true
}

// 0x1356c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14getStringValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1356c(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32, out: &mut String) {
    // IDA 0x1356c (`EnumPropDescriptor<FrameRateManagerMode>::getStringValue`):
    // `getValue` then by-ref `convertToString` (always assigns). Same as
    // 0x10248. Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    out.clear();
    if let Some(p) = desc.pairs.iter().find(|p| p.value == value) {
        out.push_str(&p.name);
    }
}

// 0x13590 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_13590(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, name: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x13590 (`EnumPropDescriptor<FrameRateManagerMode>::setStringValue`):
    // lookup + convertToValue + setValue, 1/0. Same as 0x1026c.
    // Family-verified.
    stub_1026c(desc, item, name, set)
}

// 0x135d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_135d0(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> (u32, i32) {
    // IDA 0x135d0 (`EnumPropDescriptor<FrameRateManagerMode>::writeValue`):
    // getValue + `{kind = 5, value}` pair. Same as 0x102ac. Family-verified.
    (5, get(item))
}

// 0x135f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_135f0(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, text: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x135f0 (`EnumPropDescriptor<FrameRateManagerMode>::readValue`):
    // element text → int-or-name set path. Same as 0x102cc. Family-verified.
    stub_102cc(desc, item, text, set)
}

// 0x13830 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE13getIndexValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13830(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x13830 (`EnumPropDescriptor<FrameRateManagerMode>::getIndexValue`):
    // getValue + convertToIndex (assert :350, miss -1). Same as 0x1050c.
    // Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    desc.pairs.iter().position(|p| p.value == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x1384c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE13setIndexValueEPNS0_13DescribedBaseEm
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_1384c(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, index: usize, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x1384c (`EnumPropDescriptor<FrameRateManagerMode>::setIndexValue`):
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

// 0x13880 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE12getEnumValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13880(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x13880 (`EnumPropDescriptor<FrameRateManagerMode>::getEnumValue`):
    // `getValue` through the impl. Same as 0x1055c. Family-verified.
    get(item)
}

// 0x13888 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE12setEnumValueEPNS0_13DescribedBaseEi
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13888(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, value: i32, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x13888 (`EnumPropDescriptor<FrameRateManagerMode>::setEnumValue`):
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

// 0x138d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11getEnumItemEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_138d4(desc: &RenderEnumDesc, item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> Option<i32> {
    // IDA 0x138d4 (`EnumPropDescriptor<FrameRateManagerMode>::getEnumItem`):
    // `getValue` then `convertToItem`. Same as 0x105b0. Family-verified.
    let value = get(item);
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0x138f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_138f4(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, name: &str, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x138f4 (`EnumPropDescriptor<FrameRateManagerMode>::setStringValue`
    // with `Name`): same as the `std::string` twin at 0x13590.
    // Family-verified.
    stub_13590(desc, item, name, set)
}

// 0x13928 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToIndexES3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToIndexES3_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToIndex(RBX::CRenderSettings::FrameRateManagerMode)const")]
pub fn stub_13928(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0x13928 (`EnumDesc<FrameRateManagerMode>::convertToIndex`):
    // `ReleaseAssert` (`value>=0`, `enumconverter.h:350`), value→index
    // lookup, miss -1. Same as 0x10604. Family-verified.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    desc.pairs.iter().position(|p| p.value == value).map(|i| i as i32).unwrap_or(-1)
}

// 0x13998 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11setIntValueEPNS0_13DescribedBaseEi
// mangled: __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13998(desc: &RenderEnumDesc, item: &mut RenderSettingsItem, index: i32, set: fn(&mut RenderSettingsItem, i32)) -> bool {
    // IDA 0x13998 (`EnumPropDescriptor<FrameRateManagerMode>::setIntValue`):
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

// 0x139d8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isReadOnly(void)const")]
pub fn stub_139d8() -> bool {
    // IDA 0x139d8 (`GetSetImpl<FrameRateManagerMode (CRenderSettings::*)()const,
    // void (CRenderSettingsItem::*)(FrameRateManagerMode)>::isReadOnly`):
    // `return 0`. Same as 0x106b4. Family-verified.
    false
}

// 0x139dc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isWriteOnly(void)const")]
pub fn stub_139dc() -> bool {
    // IDA 0x139dc (same `GetSetImpl::isWriteOnly`): `return 0`. Same as
    // 0x106b8. Family-verified.
    false
}

// 0x139e0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// mangled: __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_139e0(item: &RenderSettingsItem, get: fn(&RenderSettingsItem) -> i32) -> i32 {
    // IDA 0x139e0 (same `GetSetImpl::getValue` for the `FrameRateManagerMode`
    // pair): bound member getter. Same shape as 0x106bc. Family-verified.
    get(item)
}
