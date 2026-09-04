//! Auto-generated skeletons for rbx-script — wdog_script_B2 gap filler 1788369654
//! Filter: global gap filler (exclude Script|Lua|Yield case-sensitive) — distinct not yet in crates/script/src, EA-sorted asc
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0x106bc..0x13000 | EA-sorted asc distinct not yet in script (rbx_core::SharedPtr not boost)
//! SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; boost::weak_ptr -> rbx_core::Weak

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };
use crate::generated_script_wd_watchdog19::{
    CRenderSettingsResolutionAccess, ScriptXmlElement, ScriptXmlNameValuePair,
};
use rbx_reflection::descriptor::Variant;
use rbx_reflection::enum_desc::EnumDesc;
use rbx_reflection::generated::{
    CRenderSettingsAaAccess, CRenderSettingsAaPropDesc, CRenderSettingsIntAccess,
    CRenderSettingsIntProp, CRenderSettingsItemState, CRenderSettingsShadowAccess,
    CRenderSettingsShadowPropDesc, antialiasing_mode_enum_desc, shadow_mode_enum_desc,
};

/// Get/set pair behind `PropDescriptor<CRenderSettingsItem, bool>` (IDA 0x1070c).
pub struct CRenderSettingsBoolAccess {
    pub get: Box<dyn Fn(&CRenderSettingsItemState) -> bool + Send + Sync>,
    pub set: Box<dyn Fn(&mut CRenderSettingsItemState, bool) + Send + Sync>,
}

/// `RBX::Reflection::PropDescriptor<CRenderSettingsItem, bool>` (IDA 0x1070c).
pub struct CRenderSettingsBoolProp {
    pub name: String,
    pub category: String,
    pub access: CRenderSettingsBoolAccess,
    pub attributes: u32,
    pub permissions: u32,
}
/// Get/set pair behind `BoundProp<std::string, CRenderSettingsItem>`
/// (IDA 0x11b18 ctor member-offset store; read at 0x11cb0, compared at 0x11cc8).
pub struct CRenderSettingsStringAccess {
    pub get: Box<dyn Fn(&CRenderSettingsItemState) -> String + Send + Sync>,
    pub set: Box<dyn Fn(&mut CRenderSettingsItemState, &str) + Send + Sync>,
}

/// `RBX::Reflection::BoundProp<std::string, (Mutability)1>` bound to
/// `CRenderSettingsItem` (IDA 0x11b18).
pub struct CRenderSettingsStringProp {
    pub name: String,
    pub category: String,
    pub access: CRenderSettingsStringAccess,
    pub attributes: u32,
    pub permissions: u32,
}

/// IDA 0x10564/0x110e8 `setEnumValue` core: `std::find_if` with
/// `EnumDescriptor::equalValue` over the descriptor items.
fn aa_has_value(desc: &EnumDesc, value: i32) -> bool {
    desc.items.iter().any(|item| item.value == value)
}

/// IDA 0x105b0/0x11134 `getEnumItem` core: the mapped item payload, 0 on a
/// miss (cf. `enum_desc_item` in generated_script_wd_watchdog19).
fn aa_item_of(desc: &EnumDesc, value: i32) -> i32 {
    if value >= 0 {
        if let Some(&slot) = desc.value_to_value.get(value as usize) {
            if slot != -1 {
                return slot;
            }
        }
    }
    0
}
/// IDA 0x10604/0x11188 `convertToIndex` core: ReleaseAssert(value >= 0)
/// gated on `FLog::Asserts` (enumconverter.h:350), then the [desc+156] index
/// table slot, -1 when out of range. Host models that table with
/// `value_ordinals`.
fn aa_index_of(desc: &EnumDesc, value: i32) -> i32 {
    assert!(
        value >= 0,
        "value>=0 file: ../App/include/reflection/enumconverter.h line: 350"
    );
    if value >= 0 {
        if let Some(&slot) = desc.value_ordinals.get(value as usize) {
            if slot != -1 {
                return slot;
            }
        }
    }
    -1
}

/// IDA 0x10248/0x10dcc `getStringValue` core: the mapped item name, empty on
/// a miss (cf. `enum_desc_name` in generated_script_wd_watchdog19).
fn aa_string_of(desc: &EnumDesc, value: i32, out: &mut String) {
    out.clear();
    if value >= 0 {
        if let Some(name) = desc.lookup_name(value) {
            out.push_str(name);
        }
    }
}


// 0x106bc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x106bc(access: &CRenderSettingsResolutionAccess, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0x106bc GetSetImpl `getValue`: member-pointer adjust (0x106c0..0x106d8)
    // then the getter call (0x106d8..0x106e4); the pair folds into `access`.
    (access.get)(obj)
}

// 0x106e8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ResolutionPreset const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_0x106e8(
    access: &CRenderSettingsResolutionAccess,
    obj: &mut CRenderSettingsItemState,
    value: i32,
) {
    // IDA 0x106e8 GetSetImpl `setValue`: member adjust (0x106ee..0x10704)
    // then the setter call; the pair folds into `access`.
    (access.set)(obj, value)
}

// 0x1070c — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x1070c(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&CRenderSettingsItemState) -> bool + Send + Sync>,
    set: Box<dyn Fn(&mut CRenderSettingsItemState, bool) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> CRenderSettingsBoolProp {
    // IDA 0x1070c `PropDescriptor<bool>` ctor: `classDescriptor` init, base
    // `PropertyDescriptor` init, `new(0x14)` GetSet impl holding the
    // getter/setter member pointers, then the read-only/write-only attribute
    // masks (cf. 0xfe84, 0x379958; both queries return 0 so the masks never
    // fire).
    CRenderSettingsBoolProp {
        name: name.to_owned(),
        category: category.to_owned(),
        access: CRenderSettingsBoolAccess { get, set },
        attributes,
        permissions,
    }
}

// 0x10820 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED0Ev")]
pub fn stub_0x10820() {
    // IDA 0x10820: D0 deleting destructor — vtable reset (0x10834), member
    // delete (0x10836..0x1083c), `operator delete`; Arc Drop glue covers it.
}

// 0x1084c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_0x1084c() -> bool {
    // IDA 0x1084c GetSetImpl `isReadOnly`: hardcoded `return 0` (0x1084e);
    // get/set-bound props are never read-only.
    false
}

// 0x10850 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_0x10850() -> bool {
    // IDA 0x10850 GetSetImpl `isWriteOnly`: hardcoded `return 0` (0x10852);
    // get/set-bound props are never write-only.
    false
}

// 0x10854 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x10854(access: &CRenderSettingsBoolAccess, obj: &CRenderSettingsItemState) -> bool {
    // IDA 0x10854 GetSetImpl `getValue`: member adjust (0x1085a..0x10872)
    // then the getter call (0x10876); the pair folds into `access`.
    (access.get)(obj)
}

// 0x10878 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0x10878(access: &CRenderSettingsBoolAccess, obj: &mut CRenderSettingsItemState, value: bool) {
    // IDA 0x10878 GetSetImpl `setValue`: member adjust (0x1087e..0x10894)
    // then the setter call; the pair folds into `access`.
    (access.set)(obj, value)
}

// 0x1089c — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>(char const*,char const*,int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x1089c(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&CRenderSettingsItemState) -> i32 + Send + Sync>,
    set: Box<dyn Fn(&mut CRenderSettingsItemState, i32) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> CRenderSettingsIntProp {
    // IDA 0x1089c `PropDescriptor<int>` ctor: same shape as the bool twin at
    // 0x1070c (`classDescriptor` init, base init, `new(0x14)` GetSet impl,
    // attribute masks that never fire).
    CRenderSettingsIntProp {
        name: name.to_owned(),
        category: category.to_owned(),
        access: CRenderSettingsIntAccess { get, set },
        attributes,
        permissions,
    }
}

// 0x109b0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE10isReadOnlyEv")]
pub fn stub_0x109b0() -> bool {
    // IDA 0x109b0 GetSetImpl `isReadOnly`: hardcoded `return 0` (0x109b2).
    false
}

// 0x109b4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE11isWriteOnlyEv")]
pub fn stub_0x109b4() -> bool {
    // IDA 0x109b4 GetSetImpl `isWriteOnly`: hardcoded `return 0` (0x109b6).
    false
}

// 0x109b8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x109b8(access: &CRenderSettingsIntAccess, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0x109b8 GetSetImpl `getValue`: member-pointer adjust
    // (0x109bc..0x109d4) then the getter call (0x109d4..0x109e0).
    (access.get)(obj)
}

// 0x109e4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_0x109e4(access: &CRenderSettingsIntAccess, obj: &mut CRenderSettingsItemState, value: i32) {
    // IDA 0x109e4 GetSetImpl `setValue`: member adjust (0x109ea..0x10a00)
    // then the setter call.
    (access.set)(obj, value)
}

// 0x10a08 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::EnumPropDescriptor<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>(char const*,char const*,RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x10a08(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&CRenderSettingsItemState) -> i32 + Send + Sync>,
    set: Box<dyn Fn(&mut CRenderSettingsItemState, i32) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> CRenderSettingsAaPropDesc {
    // IDA 0x10a08 `EnumPropDescriptor<AntialiasingMode>` ctor: same shape as
    // the ResolutionPreset twin at 0xfe84 (`classDescriptor` init,
    // `EnumDesc<AntialiasingMode>` singleton `call_once` init, base init,
    // singleton links at +40/+48, `new(0x14)` GetSet impl, attribute masks).
    CRenderSettingsAaPropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: CRenderSettingsAaAccess { get, set },
        enum_desc: antialiasing_mode_enum_desc(),
        attributes,
        permissions,
    }
}

// 0x10bbc — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED0Ev")]
pub fn stub_0x10bbc() {
    // IDA 0x10bbc: D0 deleting destructor — vtable reset (0x10bd0), member
    // delete (0x10bd2..0x10bd8), `operator delete`; Arc Drop glue covers it.
}

// 0x10be8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10isReadOnlyEv")]
pub fn stub_0x10be8(prop: &CRenderSettingsAaPropDesc) -> bool {
    // IDA 0x10be8 `isReadOnly`: routes through the +44 GetSet impl
    // (0x10bf4 -> 0x11238). Get/set-bound enum props are never read-only.
    let _ = prop;
    false
}

// 0x10bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11isWriteOnlyEv")]
pub fn stub_0x10bf8(prop: &CRenderSettingsAaPropDesc) -> bool {
    // IDA 0x10bf8 `isWriteOnly`: routes through the +44 GetSet impl
    // (0x10c04 -> 0x1123c). Get/set-bound enum props are never write-only.
    let _ = prop;
    false
}

// 0x10c08 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_0x10c08(prop: &CRenderSettingsAaPropDesc, obj: &CRenderSettingsItemState, value: i32) -> bool {
    // IDA 0x10c08 `equalValues`: v5 = getValue(impl) (0x10c18);
    // return v5 == getValue(impl, a3) (0x10c2e).
    (prop.access.get)(obj) == value
}

// 0x10c30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x10c30(prop: &CRenderSettingsAaPropDesc, obj: &CRenderSettingsItemState) -> Variant {
    // IDA 0x10c30 `getVariant`: v5 = getIntValue (0x10c3e),
    // out = `Variant(int, v5)` via `Type::getSingleton<int>` +
    // `operator=<int>` (0x10c44..0x10c52). `Variant::Int` is the out.
    Variant::Int((prop.access.get)(obj))
}

// 0x10c54 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x10c54(prop: &CRenderSettingsAaPropDesc, obj: &mut CRenderSettingsItemState, variant: &Variant) {
    // IDA 0x10c54 `setVariant`: int-typed variants read the `any_cast<int>`
    // payload (0x10d50); anything else goes through the holder clone +
    // `Variant::convert<int>`; then `setIntValue`. `convert_to_int` covers
    // both int-source paths (cf. 0x100d0).
    let value = variant.convert_to_int();
    (prop.access.set)(obj, value);
}

// 0x10da4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_0x10da4(
    prop: &CRenderSettingsAaPropDesc,
    src: &CRenderSettingsItemState,
    dst: &mut CRenderSettingsItemState,
) {
    // IDA 0x10da4 `copyValue`: v6 = getValue(src-impl) (0x10db6), then
    // setValue(dst-impl, &v6) (0x10dc6).
    let value = (prop.access.get)(src);
    (prop.access.set)(dst, value);
}

// 0x10dc8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14hasStringValueEv")]
pub fn stub_0x10dc8() -> bool {
    // IDA 0x10dc8 `hasStringValue`: hardcoded `return 1` (0x10dca) — enum
    // props always have a string form.
    true
}

// 0x10dcc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x10dcc(prop: &CRenderSettingsAaPropDesc, obj: &CRenderSettingsItemState, out: &mut String) {
    // IDA 0x10dcc `getStringValue`: v = getValue(impl) (0x10dde), then
    // `EnumDesc<AntialiasingMode>::convertToString(desc, v, out)` (0x10dee).
    let value = (prop.access.get)(obj);
    aa_string_of(prop.enum_desc, value, out);
}

// 0x10df0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x10df0(prop: &CRenderSettingsAaPropDesc, obj: &mut CRenderSettingsItemState, name: &str) -> bool {
    // IDA 0x10df0 `setStringValue`: `Name::lookup(text)` (0x10e02),
    // `EnumDesc::convertToValue` (0x10e10); on a hit `setValue` runs
    // (0x10e26) and 1 returns, otherwise 0 (0x10e12..0x10e2c).
    if let Some(value) = prop.enum_desc.lookup_value(name) {
        (prop.access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x10e30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x10e30(
    prop: &CRenderSettingsAaPropDesc,
    obj: &CRenderSettingsItemState,
    pair: &mut ScriptXmlNameValuePair,
) -> u32 {
    // IDA 0x10e30 `writeValue`: v = getValue (0x10e3e), `clearValue`
    // (0x10e44), type tag 5 (0x10e4a), payload v (0x10e4c); returns 5.
    let value = (prop.access.get)(obj);
    *pair = ScriptXmlNameValuePair { int_value: Some(value), text: None };
    5
}

// 0x10e50 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x10e50(prop: &CRenderSettingsAaPropDesc, obj: &mut CRenderSettingsItemState, element: &ScriptXmlElement) {
    // IDA 0x10e50 `readValue`: same shape as the ResolutionPreset twin at
    // 0x102cc — xsi:nil returns; an int payload runs `setIntValue`; a string
    // payload goes through `Name::lookup` + `convertToValue` into `setValue`,
    // with the empty string falling back to 0; anything else hits
    // `ReleaseAssert(false)` (Reflection.h:359).
    if element.is_xsi_nil {
        return;
    }
    if let Some(value) = element.pair.int_value {
        (prop.access.set)(obj, value);
        return;
    }
    if let Some(text) = &element.pair.text {
        if let Some(value) = prop.enum_desc.lookup_value(text) {
            (prop.access.set)(obj, value);
            return;
        }
        if text.is_empty() {
            (prop.access.set)(obj, 0);
            return;
        }
    }
    panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
}

// 0x11090 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x11090(prop: &CRenderSettingsAaPropDesc, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0x11090 `getIndexValue`: v = getValue(impl) (0x110a0), then
    // `EnumDesc::convertToIndex(desc, v)` (0x110a4 -> 0x11188).
    let value = (prop.access.get)(obj);
    aa_index_of(prop.enum_desc, value)
}

// 0x110ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x110ac(prop: &CRenderSettingsAaPropDesc, obj: &mut CRenderSettingsItemState, index: u32) -> bool {
    // IDA 0x110ac `setIndexValue`: count = [desc+40] (0x110be); idx < count
    // loads the ordinal from [desc+144][idx] (0x110c8), runs `setValue`
    // (0x110d2) and returns 1 (0x110d4); otherwise 0. Host models the count
    // with `values` and the table with `value_ordinals`.
    if (index as usize) < prop.enum_desc.values.len() {
        let ordinal = prop
            .enum_desc
            .value_ordinals
            .get(index as usize)
            .copied()
            .unwrap_or(index as i32);
        (prop.access.set)(obj, ordinal);
        true
    } else {
        false
    }
}

// 0x110e0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x110e0(prop: &CRenderSettingsAaPropDesc, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0x110e0 `getEnumValue`: `getValue(impl)` through the +44 GetSet
    // impl (the +8 vtable slot); the enum payload is the stored int.
    (prop.access.get)(obj)
}

// 0x110e8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x110e8(prop: &CRenderSettingsAaPropDesc, obj: &mut CRenderSettingsItemState, value: i32) -> bool {
    // IDA 0x110e8 `setEnumValue`: `std::find_if` with `equalValue` over the
    // descriptor items (0x11112); a hit runs the +12 `setValue` slot
    // (0x11126) and returns 1, a miss returns 0.
    if aa_has_value(prop.enum_desc, value) {
        (prop.access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x11134 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x11134(prop: &CRenderSettingsAaPropDesc, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0x11134 `getEnumItem`: v = getValue(impl) (0x11146), then
    // `EnumDesc<AntialiasingMode>::convertToItem(desc, v)` (0x11152).
    let value = (prop.access.get)(obj);
    aa_item_of(prop.enum_desc, value)
}

// 0x11154 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x11154(prop: &CRenderSettingsAaPropDesc, obj: &mut CRenderSettingsItemState, name: &str) -> bool {
    // IDA 0x11154 `setStringValue` (Name overload): `convertToValue` on the
    // interned name (0x1116a); on a hit the +12 `setValue` slot runs
    // (0x11180) and 1 returns, otherwise 0. Host keys names by string, so
    // this shares the string-overload (0x10df0) lookup path.
    if let Some(value) = prop.enum_desc.lookup_value(name) {
        (prop.access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x11188 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToIndex(RBX::CRenderSettings::AntialiasingMode)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToIndexES3_")]
pub fn stub_0x11188(desc: &EnumDesc, value: i32) -> i32 {
    // IDA 0x11188 `convertToIndex`: AntialiasingMode instantiation of the
    // 0x10604 shape; LABEL_7 core in `aa_index_of`.
    aa_index_of(desc, value)
}

// 0x111f8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x111f8(prop: &CRenderSettingsAaPropDesc, obj: &mut CRenderSettingsItemState, index: i32) -> bool {
    // IDA 0x111f8 `setIntValue`: negative indices fail (0x11202); the
    // [desc+132] ordinal table (0x11206..0x11214) yields the payload, -1
    // entries fail (0x11220); otherwise the +12 `setValue` slot runs
    // (0x1122c) and 1 returns. Host models that table with `values`.
    if index >= 0 {
        if let Some(&payload) = prop.enum_desc.values.get(index as usize) {
            if payload != -1 {
                (prop.access.set)(obj, payload);
                return true;
            }
        }
    }
    false
}

// 0x11238 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_0x11238() -> bool {
    // IDA 0x11238 GetSetImpl `isReadOnly`: hardcoded `return 0` (0x1123a).
    false
}

// 0x1123c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_0x1123c() -> bool {
    // IDA 0x1123c GetSetImpl `isWriteOnly`: hardcoded `return 0` (0x1123e).
    false
}

// 0x11240 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x11240(access: &CRenderSettingsAaAccess, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0x11240 GetSetImpl `getValue`: member-pointer adjust
    // (0x11244..0x1125c) then the getter call (0x1125c..0x11268).
    (access.get)(obj)
}

// 0x1126c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AntialiasingMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_0x1126c(access: &CRenderSettingsAaAccess, obj: &mut CRenderSettingsItemState, value: i32) {
    // IDA 0x1126c GetSetImpl `setValue`: member adjust (0x11272..0x11288)
    // then the setter call.
    (access.set)(obj, value)
}

// 0x11290 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::EnumPropDescriptor<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>(char const*,char const*,RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x11290(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&CRenderSettingsItemState) -> i32 + Send + Sync>,
    set: Box<dyn Fn(&mut CRenderSettingsItemState, i32) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> CRenderSettingsShadowPropDesc {
    // IDA 0x11290 `EnumPropDescriptor<ShadowMode>` ctor: same shape as the
    // AntialiasingMode twin at 0x10a08 (`classDescriptor` init,
    // `EnumDesc<ShadowMode>` singleton `call_once` init, base init,
    // singleton links at +40/+48, `new(0x14)` GetSet impl, attribute masks).
    CRenderSettingsShadowPropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: CRenderSettingsShadowAccess { get, set },
        enum_desc: shadow_mode_enum_desc(),
        attributes,
        permissions,
    }
}

// 0x11444 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED0Ev")]
pub fn stub_0x11444() {
    // IDA 0x11444: D0 deleting destructor — vtable reset (0x11458), member
    // delete (0x1145a..0x11460), `operator delete`; Arc Drop glue covers it.
}

// 0x11470 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10isReadOnlyEv")]
pub fn stub_0x11470(prop: &CRenderSettingsShadowPropDesc) -> bool {
    // IDA 0x11470 `isReadOnly`: routes through the +44 GetSet impl (0x1147c).
    // Get/set-bound enum props are never read-only.
    let _ = prop;
    false
}

// 0x11480 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11isWriteOnlyEv")]
pub fn stub_0x11480(prop: &CRenderSettingsShadowPropDesc) -> bool {
    // IDA 0x11480 `isWriteOnly`: routes through the +44 GetSet impl
    // (0x1148c). Get/set-bound enum props are never write-only.
    let _ = prop;
    false
}

// 0x11490 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_0x11490(prop: &CRenderSettingsShadowPropDesc, obj: &CRenderSettingsItemState, value: i32) -> bool {
    // IDA 0x11490 `equalValues`: v5 = getValue(impl) (0x114a0);
    // return v5 == getValue(impl, a3) (0x114b6).
    (prop.access.get)(obj) == value
}

// 0x114b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x114b8(prop: &CRenderSettingsShadowPropDesc, obj: &CRenderSettingsItemState) -> Variant {
    // IDA 0x114b8 `getVariant`: v5 = getIntValue (0x114c6),
    // out = `Variant(int, v5)` via `Type::getSingleton<int>` +
    // `operator=<int>` (0x114cc..0x114da). `Variant::Int` is the out.
    Variant::Int((prop.access.get)(obj))
}

// 0x114dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x114dc(prop: &CRenderSettingsShadowPropDesc, obj: &mut CRenderSettingsItemState, variant: &Variant) {
    // IDA 0x114dc `setVariant`: int-typed variants read the `any_cast<int>`
    // payload (0x11526..0x11550); anything else goes through the holder
    // clone + `Variant::convert<int>`; then `setIntValue`. `convert_to_int`
    // covers both int-source paths (cf. 0x100d0).
    let value = variant.convert_to_int();
    (prop.access.set)(obj, value);
}

// 0x1162c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_0x1162c(
    prop: &CRenderSettingsShadowPropDesc,
    src: &CRenderSettingsItemState,
    dst: &mut CRenderSettingsItemState,
) {
    // IDA 0x1162c `copyValue`: v6 = getValue(src-impl) (0x1163e), then
    // setValue(dst-impl, &v6) (0x1164e).
    let value = (prop.access.get)(src);
    (prop.access.set)(dst, value);
}

// 0x11650 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14hasStringValueEv")]
pub fn stub_0x11650() -> bool {
    // IDA 0x11650 `hasStringValue`: hardcoded `return 1` — enum props always
    // have a string form (cf. 0x10244/0x10dc8).
    true
}

// 0x11654 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x11654(prop: &CRenderSettingsShadowPropDesc, obj: &CRenderSettingsItemState, out: &mut String) {
    // IDA 0x11654 `getStringValue`: v = getValue(impl), then
    // `EnumDesc<ShadowMode>::convertToString(desc, v, out)` (cf. 0x10dcc).
    let value = (prop.access.get)(obj);
    aa_string_of(prop.enum_desc, value, out);
}

// 0x11678 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x11678(prop: &CRenderSettingsShadowPropDesc, obj: &mut CRenderSettingsItemState, name: &str) -> bool {
    // IDA 0x11678 `setStringValue`: `Name::lookup(text)` +
    // `EnumDesc::convertToValue`; on a hit `setValue` runs and 1 returns,
    // otherwise 0 (cf. 0x10df0).
    if let Some(value) = prop.enum_desc.lookup_value(name) {
        (prop.access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x116b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x116b8(
    prop: &CRenderSettingsShadowPropDesc,
    obj: &CRenderSettingsItemState,
    pair: &mut ScriptXmlNameValuePair,
) -> u32 {
    // IDA 0x116b8 `writeValue`: v = getValue, `clearValue`, type tag 5,
    // payload v; returns 5 (cf. 0x10e30).
    let value = (prop.access.get)(obj);
    *pair = ScriptXmlNameValuePair { int_value: Some(value), text: None };
    5
}

// 0x116d8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x116d8(prop: &CRenderSettingsShadowPropDesc, obj: &mut CRenderSettingsItemState, element: &ScriptXmlElement) {
    // IDA 0x116d8 `readValue`: same shape as the twins at 0x102cc/0x10e50 —
    // xsi:nil returns; int payload runs `setIntValue`; string payload goes
    // through `convertToValue` into `setValue`, empty string falls back to
    // 0; anything else hits `ReleaseAssert(false)` (Reflection.h:359).
    if element.is_xsi_nil {
        return;
    }
    if let Some(value) = element.pair.int_value {
        (prop.access.set)(obj, value);
        return;
    }
    if let Some(text) = &element.pair.text {
        if let Some(value) = prop.enum_desc.lookup_value(text) {
            (prop.access.set)(obj, value);
            return;
        }
        if text.is_empty() {
            (prop.access.set)(obj, 0);
            return;
        }
    }
    panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
}

// 0x11918 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x11918(prop: &CRenderSettingsShadowPropDesc, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0x11918 `getIndexValue`: v = getValue(impl), then
    // `EnumDesc::convertToIndex(desc, v)` (cf. 0x11090).
    let value = (prop.access.get)(obj);
    aa_index_of(prop.enum_desc, value)
}

// 0x11934 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x11934(prop: &CRenderSettingsShadowPropDesc, obj: &mut CRenderSettingsItemState, index: u32) -> bool {
    // IDA 0x11934 `setIndexValue`: count = [desc+40]; idx < count loads the
    // ordinal from [desc+144][idx], runs `setValue` and returns 1; otherwise
    // 0 (cf. 0x110ac).
    if (index as usize) < prop.enum_desc.values.len() {
        let ordinal = prop
            .enum_desc
            .value_ordinals
            .get(index as usize)
            .copied()
            .unwrap_or(index as i32);
        (prop.access.set)(obj, ordinal);
        true
    } else {
        false
    }
}

// 0x11968 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x11968(prop: &CRenderSettingsShadowPropDesc, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0x11968 `getEnumValue`: `getValue(impl)` through the +44 GetSet
    // impl (cf. 0x110e0).
    (prop.access.get)(obj)
}

// 0x11970 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x11970(prop: &CRenderSettingsShadowPropDesc, obj: &mut CRenderSettingsItemState, value: i32) -> bool {
    // IDA 0x11970 `setEnumValue`: `std::find_if` with `equalValue`; a hit
    // runs `setValue` and returns 1, a miss returns 0 (cf. 0x110e8).
    if aa_has_value(prop.enum_desc, value) {
        (prop.access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x119bc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x119bc(prop: &CRenderSettingsShadowPropDesc, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0x119bc `getEnumItem`: v = getValue(impl), then
    // `EnumDesc<ShadowMode>::convertToItem(desc, v)` (cf. 0x11134).
    let value = (prop.access.get)(obj);
    aa_item_of(prop.enum_desc, value)
}

// 0x119dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x119dc(prop: &CRenderSettingsShadowPropDesc, obj: &mut CRenderSettingsItemState, name: &str) -> bool {
    // IDA 0x119dc `setStringValue` (Name overload): `convertToValue` on the
    // interned name; on a hit `setValue` runs and 1 returns, otherwise 0
    // (cf. 0x11154).
    if let Some(value) = prop.enum_desc.lookup_value(name) {
        (prop.access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x11a10 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToIndex(RBX::CRenderSettings::ShadowMode)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_")]
pub fn stub_0x11a10(desc: &EnumDesc, value: i32) -> i32 {
    // IDA 0x11a10 `convertToIndex`: ShadowMode instantiation of the
    // 0x10604/0x11188 shape; LABEL_7 core in `aa_index_of`.
    aa_index_of(desc, value)
}

// 0x11a80 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x11a80(prop: &CRenderSettingsShadowPropDesc, obj: &mut CRenderSettingsItemState, index: i32) -> bool {
    // IDA 0x11a80 `setIntValue`: negative indices fail; the [desc+132]
    // ordinal table yields the payload, -1 entries fail; otherwise `setValue`
    // runs and 1 returns (cf. 0x111f8).
    if index >= 0 {
        if let Some(&payload) = prop.enum_desc.values.get(index as usize) {
            if payload != -1 {
                (prop.access.set)(obj, payload);
                return true;
            }
        }
    }
    false
}

// 0x11ac0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_0x11ac0() -> bool {
    // IDA 0x11ac0 GetSetImpl `isReadOnly`: hardcoded `return 0`.
    false
}

// 0x11ac4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_0x11ac4() -> bool {
    // IDA 0x11ac4 GetSetImpl `isWriteOnly`: hardcoded `return 0`.
    false
}

// 0x11ac8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x11ac8(access: &CRenderSettingsShadowAccess, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0x11ac8 GetSetImpl `getValue`: member-pointer adjust then the
    // getter call (cf. 0x11240).
    (access.get)(obj)
}

// 0x11af4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ShadowMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_0x11af4(access: &CRenderSettingsShadowAccess, obj: &mut CRenderSettingsItemState, value: i32) {
    // IDA 0x11af4 GetSetImpl `setValue`: member adjust then the setter call
    // (cf. 0x1126c).
    (access.set)(obj, value)
}

// 0x11b18 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,std::string  CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x11b18(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&CRenderSettingsItemState) -> String + Send + Sync>,
    set: Box<dyn Fn(&mut CRenderSettingsItemState, &str) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> CRenderSettingsStringProp {
    // IDA 0x11b18 `BoundProp<std::string>` ctor: `classDescriptor` init
    // (0x11b3e), `TypedPropertyDescriptor<std::string>` base init, fresh
    // BoundPropGetSet block holding the owner + member offset, then the
    // read-only/write-only attribute masks (both queries return 0 below, so
    // the masks never fire; cf. 0x379958).
    CRenderSettingsStringProp {
        name: name.to_owned(),
        category: category.to_owned(),
        access: CRenderSettingsStringAccess { get, set },
        attributes,
        permissions,
    }
}

// 0x11ca8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv")]
pub fn stub_0x11ca8() -> bool {
    // IDA 0x11ca8 BoundPropGetSet<std::string> `isReadOnly`: hardcoded
    // `return 0` (0x11caa).
    false
}

// 0x11cac — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv")]
pub fn stub_0x11cac() -> bool {
    // IDA 0x11cac BoundPropGetSet<std::string> `isWriteOnly`: hardcoded
    // `return 0` (0x11cae).
    false
}

// 0x11cb0 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x11cb0(access: &CRenderSettingsStringAccess, obj: &CRenderSettingsItemState) -> String {
    // IDA 0x11cb0 BoundPropGetSet<std::string> `getValue`: member-offset
    // load `*(member_offset(a2+8) + obj - 36)` into the out string (0x11cb2..
    // 0x11cc6); the pair folds into `access`.
    (access.get)(obj)
}

// 0x11cc8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x11cc8(access: &CRenderSettingsStringAccess, obj: &mut CRenderSettingsItemState, value: &str) -> bool {
    // IDA 0x11cc8 BoundPropGetSet<std::string> `setValue`: member adjust
    // (0x11cd6..0x11ce0), `compare` with early-out when equal
    // (0x11ce6..0x11cea), else `assign` (0x11cf0) and
    // `raisePropertyChanged` when the notify bits are set (0x11cf4..0x11d2a).
    // The signal lives on Instance (datamodel side); the model keeps the
    // compare-and-store (cf. 0x379afc).
    if (access.get)(obj) != value {
        (access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x11d30 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::EnumPropDescriptor<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>(char const*,char const*,RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x11d30() -> ! {
    todo!("0x11d30 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x11ee4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev")]
pub fn stub_0x11ee4() -> ! {
    todo!("0x11ee4 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev")
}

// 0x11f10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv")]
pub fn stub_0x11f10() -> ! {
    todo!("0x11f10 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv")
}

// 0x11f20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv")]
pub fn stub_0x11f20() -> ! {
    todo!("0x11f20 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv")
}

// 0x11f30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_0x11f30() -> ! {
    todo!("0x11f30 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_")
}

// 0x11f58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x11f58() -> ! {
    todo!("0x11f58 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")
}

// 0x11f7c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x11f7c() -> ! {
    todo!("0x11f7c __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")
}

// 0x120cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_0x120cc() -> ! {
    todo!("0x120cc __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9copyValueEPKNS0_13DescribedBaseEPS6_")
}

// 0x120f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14hasStringValueEv")]
pub fn stub_0x120f0() -> ! {
    todo!("0x120f0 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14hasStringValueEv")
}

// 0x120f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x120f4() -> ! {
    todo!("0x120f4 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14getStringValueEPKNS0_13DescribedBaseE")
}

// 0x12118 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x12118() -> ! {
    todo!("0x12118 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKSs")
}

// 0x12158 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x12158() -> ! {
    todo!("0x12158 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")
}

// 0x12178 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x12178() -> ! {
    todo!("0x12178 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x123b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x123b8() -> ! {
    todo!("0x123b8 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13getIndexValueEPKNS0_13DescribedBaseE")
}

// 0x123d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x123d4() -> ! {
    todo!("0x123d4 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13setIndexValueEPNS0_13DescribedBaseEm")
}

// 0x12408 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x12408() -> ! {
    todo!("0x12408 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12getEnumValueEPKNS0_13DescribedBaseE")
}

// 0x12410 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x12410() -> ! {
    todo!("0x12410 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12setEnumValueEPNS0_13DescribedBaseEi")
}

// 0x1245c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x1245c() -> ! {
    todo!("0x1245c __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11getEnumItemEPKNS0_13DescribedBaseE")
}

// 0x1247c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x1247c() -> ! {
    todo!("0x1247c __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")
}

// 0x124b0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToIndex(RBX::CRenderSettings::AASamples)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_")]
pub fn stub_0x124b0() -> ! {
    todo!("0x124b0 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_")
}

// 0x12520 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x12520() -> ! {
    todo!("0x12520 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0x12560 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_0x12560() -> ! {
    todo!("0x12560 __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")
}

// 0x12564 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_0x12564() -> ! {
    todo!("0x12564 __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")
}

// 0x12568 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x12568() -> ! {
    todo!("0x12568 __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")
}

// 0x12594 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AASamples const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_0x12594() -> ! {
    todo!("0x12594 __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")
}

// 0x125b8 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,bool CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x125b8() -> ! {
    todo!("0x125b8 __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x12748 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv")]
pub fn stub_0x12748() -> ! {
    todo!("0x12748 __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv")
}

// 0x1274c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv")]
pub fn stub_0x1274c() -> ! {
    todo!("0x1274c __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv")
}

// 0x12750 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x12750() -> ! {
    todo!("0x12750 __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE")
}

// 0x1275c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0x1275c() -> ! {
    todo!("0x1275c __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKb")
}

// 0x127ac — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x127ac() -> ! {
    todo!("0x127ac __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x128c0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_0x128c0() -> ! {
    todo!("0x128c0 __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE10isReadOnlyEv")
}

// 0x128c4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_0x128c4() -> ! {
    todo!("0x128c4 __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE11isWriteOnlyEv")
}

// 0x128c8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x128c8() -> ! {
    todo!("0x128c8 __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")
}

// 0x128fc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0x128fc() -> ! {
    todo!("0x128fc __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")
}

// 0x12920 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::EnumPropDescriptor<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>(char const*,char const*,RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x12920() -> ! {
    todo!("0x12920 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x12ad4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED0Ev")]
pub fn stub_0x12ad4() -> ! {
    todo!("0x12ad4 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED0Ev")
}

// 0x12b00 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10isReadOnlyEv")]
pub fn stub_0x12b00() -> ! {
    todo!("0x12b00 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10isReadOnlyEv")
}

// 0x12b10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11isWriteOnlyEv")]
pub fn stub_0x12b10() -> ! {
    todo!("0x12b10 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11isWriteOnlyEv")
}

// 0x12b20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_0x12b20() -> ! {
    todo!("0x12b20 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11equalValuesEPKNS0_13DescribedBaseES8_")
}

// 0x12b48 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x12b48() -> ! {
    todo!("0x12b48 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")
}

// 0x12b6c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x12b6c() -> ! {
    todo!("0x12b6c __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")
}

// 0x12cbc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_0x12cbc() -> ! {
    todo!("0x12cbc __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9copyValueEPKNS0_13DescribedBaseEPS6_")
}

// 0x12ce0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14hasStringValueEv")]
pub fn stub_0x12ce0() -> ! {
    todo!("0x12ce0 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14hasStringValueEv")
}

// 0x12ce4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x12ce4() -> ! {
    todo!("0x12ce4 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14getStringValueEPKNS0_13DescribedBaseE")
}

// 0x12d08 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x12d08() -> ! {
    todo!("0x12d08 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKSs")
}

// 0x12d48 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x12d48() -> ! {
    todo!("0x12d48 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")
}

// 0x12d68 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x12d68() -> ! {
    todo!("0x12d68 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x12fa8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x12fa8() -> ! {
    todo!("0x12fa8 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13getIndexValueEPKNS0_13DescribedBaseE")
}

// 0x12fc4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x12fc4() -> ! {
    todo!("0x12fc4 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13setIndexValueEPNS0_13DescribedBaseEm")
}

// 0x12ff8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x12ff8() -> ! {
    todo!("0x12ff8 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12getEnumValueEPKNS0_13DescribedBaseE")
}

// 0x13000 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x13000() -> ! {
    todo!("0x13000 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12setEnumValueEPNS0_13DescribedBaseEi")
}
