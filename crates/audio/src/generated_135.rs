//! audio generated_135 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Soundscape exhausted (2398 distinct) — filler EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x106bc..0x128c0 EA-sorted asc filler after 0x106b8, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_134::{IntCallResult, XmlIntSlot, XmlReadValue};
use crate::generated::flog_asserts;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

/// Host stand-ins for IDA 0x106bc..0x10854
/// (`PropDescriptor<CRenderSettingsItem,ResolutionPreset>` GetSetImpl +
/// `PropDescriptor<CRenderSettingsItem,bool>`). Unlike the int family in
/// `generated_134.rs`, the bool pair binds `CRenderSettingsItem` member
/// functions on both sides (IDA 0x1070c), so its getter reads the item
/// itself. Only the slots this batch touches are modelled.
#[derive(Default)]
pub struct ResolutionSettings {
    pub resolution_preference: i32,
}

#[derive(Default)]
pub struct ResolutionItem {
    pub resolution_preset: i32,
}

#[derive(Default)]
pub struct BoolItem {
    pub flag: bool,
}

/// Host carrier for `PropDescriptor<CRenderSettingsItem,ResolutionPreset>`
/// GetSetImpl (IDA 0x106bc/0x106e8): member-pointer thunk details (the
/// `a2-36`/`+96` adjusts, `>>1`/`&1` virtual-vs-direct dispatch) have no
/// host effect.
pub struct ResolutionPresetPair {
    pub getter: Option<fn(&ResolutionSettings) -> i32>,
    pub setter: Option<fn(&mut ResolutionItem, i32)>,
}

/// Host carrier for `PropDescriptor<CRenderSettingsItem,bool>` (IDA 0x1070c:
/// classDescriptor fetch 0x10734, GetSetImpl alloc + member-fn stores
/// 0x1073a..0x10774, `TypedPropertyDescriptor<bool>` ctor 0x107b2, vtable
/// install 0x107d0).
pub struct BoolProp {
    pub name: String,
    pub category: String,
    pub getter: Option<fn(&BoolItem) -> bool>,
    pub setter: Option<fn(&mut BoolItem, bool)>,
    pub attributes: u32,
    pub permissions: u32,
}

impl BoolProp {
    /// IDA 0x1084c (disasm 0x1084c..0x1084e `MOVS R0,#0; BX LR`): a bound
    /// getter is never read-only.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0x10850 (disasm 0x10850..0x10852 `MOVS R0,#0; BX LR`): a bound
    /// setter is never write-only.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// Host carrier for `PropDescriptor<CRenderSettingsItem,int>` (IDA 0x1089c:
/// classDescriptor 0x108c4, GetSetImpl alloc + member-fn stores
/// 0x108ca..0x10908, `TypedPropertyDescriptor<int>` ctor 0x10942, vtable
/// install 0x10960). Unlike the bool pair, the getter binds a
/// `CRenderSettings` member function (cf. ResolutionSettings in
/// generated_134.rs), so it reads the settings object itself.
pub struct IntProp {
    pub name: String,
    pub category: String,
    pub getter: Option<fn(&IntSettings) -> i32>,
    pub setter: Option<fn(&mut IntItem, i32)>,
    pub attributes: u32,
    pub permissions: u32,
}

impl IntProp {
    /// IDA 0x109b0 (decompiled `return 0`, 0x109b2): a bound getter is never
    /// read-only.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0x109b4 (disasm 0x109b4..0x109b6 `MOVS R0,#0; BX LR`): a bound
    /// setter is never write-only.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// CRenderSettings int slot read by the 0x1089c getter.
#[derive(Default)]
pub struct IntSettings {
    pub value: i32,
}

/// CRenderSettingsItem int slot written by the 0x1089c setter.
#[derive(Default)]
pub struct IntItem {
    pub value: i32,
}

/// Host carrier for `EnumPropDescriptor<CRenderSettingsItem,AntialiasingMode>`
/// (IDA 0x10a08): enum singleton at +40/+48 (0x10a4c..0x10b28), GetSetImpl
/// {getter, setter} at +44 (0x10ae6..0x10b0c), attribute flag fixups at +28
/// from the isReadOnly/isWriteOnly virtuals (0x10b38..0x10b5e, stored as
/// passed — cf. generated_134 stub_fe84).
pub struct AntialiasingProp {
    pub name: String,
    pub category: String,
    pub getter: Option<fn(&AntialiasingSettings) -> i32>,
    pub setter: Option<fn(&mut AntialiasingItem, i32)>,
    pub attributes: u32,
    pub permissions: u32,
    pub enum_type: &'static str,
}

impl AntialiasingProp {
    /// IDA 0x10be8 (decompiled: load impl at `[a1+44]`, tail-call its slot-0
    /// virtual at 0x10bf4): delegates to the GetSetImpl's isReadOnly.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0x10bf8 (slot-1 virtual at 0x10c04): delegates to isWriteOnly.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// CRenderSettings AntialiasingMode slot read by the 0x10a08 getter.
#[derive(Default)]
pub struct AntialiasingSettings {
    pub mode: i32,
}

/// CRenderSettingsItem AntialiasingMode slot written by the 0x10a08 setter.
#[derive(Default)]
pub struct AntialiasingItem {
    pub mode: i32,
}

/// Host model of `EnumDesc<AntialiasingMode>` for IDA 0x10c30..0x111f8.
/// `items` is the ordered (value, name) table; `index_to_value` is the legacy
/// index->value map with `-1` holes for unmapped indices (mirrors
/// `ResolutionEnumDesc` in generated_134.rs).
#[derive(Default)]
pub struct AntialiasingEnumDesc {
    pub items: Vec<(i32, String)>,
    pub index_to_value: Vec<i32>,
}

impl AntialiasingEnumDesc {
    /// Host helper mirroring the EnumDesc add path: appends the (value, name)
    /// item and records it in the legacy index->value map at `index` (gaps
    /// stay `-1`, as in the image table).
    pub fn add_pair(&mut self, value: i32, name: &str, index: usize) {
        self.items.push((value, name.to_owned()));
        if self.index_to_value.len() <= index {
            self.index_to_value.resize(index + 1, -1);
        }
        self.index_to_value[index] = value;
    }

    /// Host search behind 0x10df0/0x10e50 (`Name::lookup` + `convertToValue`):
    /// interning has no host effect; the host compares names directly.
    pub fn lookup_value(&self, name: &str) -> Option<i32> {
        self.items.iter().find(|(_, n)| n == name).map(|(v, _)| *v)
    }

    /// Host `EnumDesc::convertToString` behind 0x10dcc: assigns the item name
    /// for `value`, returns false with `out` untouched when unmapped.
    pub fn value_to_string(&self, value: i32, out: &mut String) -> bool {
        if let Some((_, name)) = self.items.iter().find(|(v, _)| *v == value) {
            *out = name.clone();
            true
        } else {
            false
        }
    }

    /// Host `EnumDesc::convertToIndex` behind 0x11090: the legacy
    /// index->value table indexed by value, `-1` when out of range.
    pub fn convert_to_index(&self, value: i32) -> i32 {
        if value >= 0 && (value as usize) < self.index_to_value.len() {
            return self.index_to_value[value as usize];
        }
        -1
    }
}

/// Host carrier for `PropDescriptor<CRenderSettingsItem,AntialiasingMode>`
/// GetSetImpl (IDA 0x11240/0x1126c): the same getter/setter pair the
/// EnumPropDescriptor keeps at +44 — cf. ResolutionPresetPair at the top of
/// this file.
pub struct AntialiasingPair {
    pub getter: Option<fn(&AntialiasingSettings) -> i32>,
    pub setter: Option<fn(&mut AntialiasingItem, i32)>,
}

impl AntialiasingPair {
    /// IDA 0x11238 (decompiled `return 0`, 0x1123a): a bound getter is never
    /// read-only.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0x1123c (decompiled `return 0`, 0x1123e): a bound setter is never
    /// write-only.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// Host carrier for `EnumPropDescriptor<CRenderSettingsItem,ShadowMode>`
/// (IDA 0x11290: same shape as 0x10a08 — classDescriptor, enum Singleton
/// init, PropertyDescriptor base, enum desc at +40, GetSetImpl at +44,
/// attribute fixups at +28).
pub struct ShadowProp {
    pub name: String,
    pub category: String,
    pub getter: Option<fn(&ShadowSettings) -> i32>,
    pub setter: Option<fn(&mut ShadowItem, i32)>,
    pub attributes: u32,
    pub permissions: u32,
    pub enum_type: &'static str,
}

impl ShadowProp {
    /// IDA 0x11470 (same delegate-to-GetSetImpl shape as 0x10be8): slot-0
    /// virtual.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0x11480 (same shape as 0x10bf8): slot-1 virtual.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// CRenderSettings ShadowMode slot read by the 0x11290 getter.
#[derive(Default)]
pub struct ShadowSettings {
    pub mode: i32,
}

/// CRenderSettingsItem ShadowMode slot written by the 0x11290 setter.
#[derive(Default)]
pub struct ShadowItem {
    pub mode: i32,
}

/// Host model of `EnumDesc<ShadowMode>` for IDA 0x114b8..0x11a80 (mirrors
/// `AntialiasingEnumDesc` above).
#[derive(Default)]
pub struct ShadowEnumDesc {
    pub items: Vec<(i32, String)>,
    pub index_to_value: Vec<i32>,
}

impl ShadowEnumDesc {
    pub fn add_pair(&mut self, value: i32, name: &str, index: usize) {
        self.items.push((value, name.to_owned()));
        if self.index_to_value.len() <= index {
            self.index_to_value.resize(index + 1, -1);
        }
        self.index_to_value[index] = value;
    }

    pub fn lookup_value(&self, name: &str) -> Option<i32> {
        self.items.iter().find(|(_, n)| n == name).map(|(v, _)| *v)
    }

    pub fn value_to_string(&self, value: i32, out: &mut String) -> bool {
        if let Some((_, name)) = self.items.iter().find(|(v, _)| *v == value) {
            *out = name.clone();
            true
        } else {
            false
        }
    }

    pub fn convert_to_index(&self, value: i32) -> i32 {
        if value >= 0 && (value as usize) < self.index_to_value.len() {
            return self.index_to_value[value as usize];
        }
        -1
    }
}

/// Host carrier for `PropDescriptor<CRenderSettingsItem,ShadowMode>` GetSetImpl
/// (IDA 0x11ac8/0x11af4): the same getter/setter pair the ShadowMode
/// EnumPropDescriptor keeps at +44 — cf. AntialiasingPair above.
pub struct ShadowPair {
    pub getter: Option<fn(&ShadowSettings) -> i32>,
    pub setter: Option<fn(&mut ShadowItem, i32)>,
}

impl ShadowPair {
    /// IDA 0x11ac0 (decompiled `return 0`, 0x11ac2): a bound getter is never
    /// read-only.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0x11ac4 (decompiled `return 0`): a bound setter is never
    /// write-only.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// Host carrier for `BoundProp<std::string, Mutability::ReadWrite>`
/// (IDA 0x11b18: classDescriptor 0x11b3e, `TypedPropertyDescriptor<string>`
/// ctor 0x11ba0, vtable install 0x11bbe/0x11bc2, BoundPropGetSet alloc +
/// member-data offset store 0x11bcc..). Unlike GetSetImpl, BoundProp binds a
/// data member (`std::string CRenderSettingsItem::*` = field offset); the
/// host keeps the single string slot, so the offset has no host effect.
pub struct StringBoundProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
}

/// CRenderSettingsItem string slot behind the 0x11b18 bound member.
#[derive(Default)]
pub struct StringItem {
    pub text: String,
}

/// Host carrier for `EnumPropDescriptor<CRenderSettingsItem,AASamples>`
/// (IDA 0x11d30: same shape as 0x10a08 — classDescriptor, enum Singleton
/// init, base ctor, enum desc at +40, GetSetImpl at +44, fixups at +28).
pub struct AASamplesProp {
    pub name: String,
    pub category: String,
    pub getter: Option<fn(&AASamplesSettings) -> i32>,
    pub setter: Option<fn(&mut AASamplesItem, i32)>,
    pub attributes: u32,
    pub permissions: u32,
    pub enum_type: &'static str,
}

impl AASamplesProp {
    /// IDA 0x11f10 (same delegate-to-GetSetImpl shape as 0x10be8).
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0x11f20 (same shape as 0x10bf8).
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// CRenderSettings AASamples slot read by the 0x11d30 getter.
#[derive(Default)]
pub struct AASamplesSettings {
    pub samples: i32,
}

/// CRenderSettingsItem AASamples slot written by the 0x11d30 setter.
#[derive(Default)]
pub struct AASamplesItem {
    pub samples: i32,
}

/// Host model of `EnumDesc<AASamples>` for IDA 0x11f58..0x12520 (mirrors
/// `AntialiasingEnumDesc` above).
#[derive(Default)]
pub struct AASamplesEnumDesc {
    pub items: Vec<(i32, String)>,
    pub index_to_value: Vec<i32>,
}

impl AASamplesEnumDesc {
    pub fn add_pair(&mut self, value: i32, name: &str, index: usize) {
        self.items.push((value, name.to_owned()));
        if self.index_to_value.len() <= index {
            self.index_to_value.resize(index + 1, -1);
        }
        self.index_to_value[index] = value;
    }

    pub fn lookup_value(&self, name: &str) -> Option<i32> {
        self.items.iter().find(|(_, n)| n == name).map(|(v, _)| *v)
    }

    pub fn value_to_string(&self, value: i32, out: &mut String) -> bool {
        if let Some((_, name)) = self.items.iter().find(|(v, _)| *v == value) {
            *out = name.clone();
            true
        } else {
            false
        }
    }

    pub fn convert_to_index(&self, value: i32) -> i32 {
        if value >= 0 && (value as usize) < self.index_to_value.len() {
            return self.index_to_value[value as usize];
        }
        -1
    }
}

/// Host carrier for `PropDescriptor<CRenderSettingsItem,AASamples>` GetSetImpl
/// (IDA 0x12568/0x12594): the same getter/setter pair the AASamples
/// EnumPropDescriptor keeps at +44.
pub struct AASamplesPair {
    pub getter: Option<fn(&AASamplesSettings) -> i32>,
    pub setter: Option<fn(&mut AASamplesItem, i32)>,
}

impl AASamplesPair {
    /// IDA 0x12560 (decompiled `return 0`, 0x12562): a bound getter is never
    /// read-only.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0x12564 (decompiled `return 0`, 0x12566): a bound setter is never
    /// write-only.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// Host carrier for `BoundProp<bool, Mutability::ReadWrite>` (IDA 0x125b8:
/// same shape as the string BoundProp 0x11b18 — classDescriptor,
/// `TypedPropertyDescriptor<bool>` ctor, vtable install, BoundPropGetSet
/// alloc + member-data offset store). The bound bool member travels as the
/// single flag slot below.
pub struct BoolBoundProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
}

/// CRenderSettingsItem bool slot behind the 0x125b8 bound member.
#[derive(Default)]
pub struct BoolBoundItem {
    pub flag: bool,
}

/// Host carrier for `PropDescriptor<CRenderSettingsItem,bool>` with the
/// getter on `CRenderSettings` (IDA 0x127ac: classDescriptor 0x127d4,
/// GetSetImpl alloc + member-fn stores 0x127da..0x12814,
/// `TypedPropertyDescriptor<bool>` ctor 0x12852, vtable install 0x12870) —
/// cf. IntProp above.
pub struct BoolCProp {
    pub name: String,
    pub category: String,
    pub getter: Option<fn(&BoolCSettings) -> bool>,
    pub setter: Option<fn(&mut BoolItem, bool)>,
    pub attributes: u32,
    pub permissions: u32,
}

/// CRenderSettings bool slot read by the 0x127ac getter.
#[derive(Default)]
pub struct BoolCSettings {
    pub flag: bool,
}

// 0x106bc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_106bc(prop: &ResolutionPresetPair, settings: &ResolutionSettings) -> i32 {
    // IDA 0x106bc (decompiled 0x106bc..0x106e6; disasm null-object split
    // 0x106bc..0x106d6, `a2-36` + 96 adjust 0x106c0..0x106cc,
    // virtual/indirect dispatch 0x106d8..0x106e6): resolves the stored
    // `ResolutionPreset (CRenderSettings::*)() const` and calls it. A null
    // getter faults in the image; the host panics.
    let get = prop.getter.expect("bound getter at IDA 0xfe84");
    get(settings)
}

// 0x106e8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ResolutionPreset const&)const")]
pub fn stub_106e8(prop: &ResolutionPresetPair, item: &mut ResolutionItem, value: i32) {
    // IDA 0x106e8 (decompiled 0x106e8..0x1070a; disasm `a2-36` adjust
    // 0x106ec..0x106f0, setter fetch 0x106f4, `>>1`/`&1` dispatch
    // 0x106f6..0x10704, indirect call 0x10704..0x1070a): resolves the stored
    // `void (CRenderSettingsItem::*)(ResolutionPreset)` and calls it. A null
    // setter faults in the image; the host panics.
    let set = prop.setter.expect("bound setter at IDA 0xfe84");
    set(item, value);
}

// 0x1070c — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_1070c(
    name: &str,
    category: &str,
    getter: fn(&BoolItem) -> bool,
    setter: fn(&mut BoolItem, bool),
    attributes: u32,
    permissions: u32,
) -> BoolProp {
    // IDA 0x1070c (decompiled 0x1070c..0x107ee; disasm classDescriptor fetch
    // 0x10734, GetSetImpl alloc + member-fn stores 0x1073a..0x10774,
    // `TypedPropertyDescriptor<bool>` ctor 0x107b2, vtable install
    // 0x107d0): registers the bool get/set pair against the RenderSettings
    // class descriptor.
    let _ = crate::generated_134::stub_fa00();
    BoolProp {
        name: name.to_owned(),
        category: category.to_owned(),
        getter: Some(getter),
        setter: Some(setter),
        attributes,
        permissions,
    }
}

// 0x10820 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
pub fn stub_10820() {
    // IDA 0x10820: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x1084c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
pub fn stub_1084c(prop: &BoolProp) -> bool {
    // IDA 0x1084c (disasm 0x1084c..0x1084e `MOVS R0,#0; BX LR`): the bool
    // member pointer is bound at 0x1070c, so never read-only.
    prop.is_read_only()
}

// 0x10850 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_10850(prop: &BoolProp) -> bool {
    // IDA 0x10850 (disasm 0x10850..0x10852 `MOVS R0,#0; BX LR`): the bool
    // member pointer is bound at 0x1070c, so never write-only.
    prop.is_write_only()
}

// 0x10854 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_10854(prop: &BoolProp, item: &BoolItem) -> bool {
    // IDA 0x10854 (decompiled 0x10854..0x10876; disasm null-object split
    // 0x10856..0x1085c, getter/dispatch fetch 0x10860..0x1086a, indirect
    // call 0x1086e..0x10876): resolves the stored
    // `bool (CRenderSettingsItem::*)() const` and calls it. A null getter
    // faults in the image; the host panics.
    let get = prop.getter.expect("bound getter at IDA 0x1070c");
    get(item)
}

// 0x10878 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_10878(prop: &BoolProp, item: &mut BoolItem, value: bool) {
    // IDA 0x10878 (decompiled 0x10878..0x10898; disasm null-object split
    // 0x1087e..0x10880 `a2-36`, setter fetch 0x10884..0x1088c, `>>1`/`&1`
    // dispatch 0x1088c..0x10894, indirect call 0x10894..0x10898): resolves
    // the stored `void (CRenderSettingsItem::*)(bool)` and calls it. A null
    // setter faults in the image; the host panics.
    let set = prop.setter.expect("bound setter at IDA 0x1070c");
    set(item, value);
}

// 0x1089c — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>(char const*,char const*,int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_1089c(
    name: &str,
    category: &str,
    getter: fn(&IntSettings) -> i32,
    setter: fn(&mut IntItem, i32),
    attributes: u32,
    permissions: u32,
) -> IntProp {
    // IDA 0x1089c (decompiled 0x1089c..0x1097e; disasm classDescriptor
    // 0x108c4, GetSetImpl alloc + member-fn stores 0x108ca..0x10908,
    // `TypedPropertyDescriptor<int>` ctor 0x10942, vtable install 0x10960):
    // registers the int get/set pair against the RenderSettings class
    // descriptor (host: generated_134 stub_fa00, as in 0x1070c).
    let _ = crate::generated_134::stub_fa00();
    IntProp {
        name: name.to_owned(),
        category: category.to_owned(),
        getter: Some(getter),
        setter: Some(setter),
        attributes,
        permissions,
    }
}

// 0x109b0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isReadOnly(void)const")]
pub fn stub_109b0(prop: &IntProp) -> bool {
    // IDA 0x109b0 (decompiled `return 0`, 0x109b2): the int member pointer
    // is bound at 0x1089c, so never read-only.
    prop.is_read_only()
}

// 0x109b4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isWriteOnly(void)const")]
pub fn stub_109b4(prop: &IntProp) -> bool {
    // IDA 0x109b4 (disasm 0x109b4..0x109b6 `MOVS R0,#0; BX LR`): the int
    // member pointer is bound at 0x1089c, so never write-only.
    prop.is_write_only()
}

// 0x109b8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_109b8(prop: &IntProp, settings: &IntSettings) -> i32 {
    // IDA 0x109b8 (decompiled 0x109b8..0x109e2; disasm null-object split
    // 0x109ba..0x109d2, `a2-36` + 96 adjust 0x109c0..0x109c8,
    // virtual/indirect dispatch 0x109d4..0x109e0, indirect call): resolves
    // the stored `int (CRenderSettings::*)() const` and calls it. A null
    // getter faults in the image; the host panics.
    let get = prop.getter.expect("bound getter at IDA 0x1089c");
    get(settings)
}

// 0x109e4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_109e4(prop: &IntProp, item: &mut IntItem, value: i32) {
    // IDA 0x109e4 (decompiled 0x109e4..0x10a06; disasm `a2-36` adjust
    // 0x109ea..0x109ec, setter fetch 0x109f0..0x109f8, `>>1`/`&1` dispatch
    // 0x109f8..0x10a00, indirect call 0x10a00..0x10a06): resolves the stored
    // `void (CRenderSettingsItem::*)(int)` and calls it. A null setter
    // faults in the image; the host panics.
    let set = prop.setter.expect("bound setter at IDA 0x1089c");
    set(item, value);
}

// 0x10a08 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::EnumPropDescriptor<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>(char const*,char const*,RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_10a08(
    name: &str,
    category: &str,
    getter: fn(&AntialiasingSettings) -> i32,
    setter: fn(&mut AntialiasingItem, i32),
    attributes: u32,
    permissions: u32,
) -> AntialiasingProp {
    // IDA 0x10a08 (decompiled 0x10a08..0x10b7e; disasm classDescriptor
    // 0x10a2c, enum Singleton call_once + doGetSingleton 0x10a4c/0x10a50,
    // PropertyDescriptor base 0x10a9a, enum desc at +40 0x10abe, GetSetImpl
    // alloc + member-fn stores at +44 0x10ae6..0x10b0c, second singleton
    // touch 0x10b16..0x10b28, attribute fixups at +28 0x10b38..0x10b5e,
    // return self 0x10b7e): registers the enum get/set pair plus the
    // AntialiasingMode EnumDesc singleton (host: the desc travels
    // separately; the singleton init has no host effect).
    let _ = crate::generated_134::stub_fa00();
    AntialiasingProp {
        name: name.to_owned(),
        category: category.to_owned(),
        getter: Some(getter),
        setter: Some(setter),
        attributes,
        permissions,
        enum_type: "AntialiasingMode",
    }
}

// 0x10bbc — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
pub fn stub_10bbc() {
    // IDA 0x10bbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x10be8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isReadOnly(void)const")]
pub fn stub_10be8(prop: &AntialiasingProp) -> bool {
    // IDA 0x10be8 (decompiled; disasm: load impl at `[a1+44]`, tail-call its
    // slot-0 virtual at 0x10bf4): delegates to the +44 GetSetImpl's
    // isReadOnly.
    prop.is_read_only()
}

// 0x10bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isWriteOnly(void)const")]
pub fn stub_10bf8(prop: &AntialiasingProp) -> bool {
    // IDA 0x10bf8 (disasm: load impl at `[a1+44]`, tail-call its slot-1
    // virtual at 0x10c04): delegates to the +44 GetSetImpl's isWriteOnly.
    prop.is_write_only()
}

// 0x10c08 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_10c08(prop: &AntialiasingProp, a: &AntialiasingSettings, b: &AntialiasingSettings) -> bool {
    // IDA 0x10c08 (decompiled 0x10c08..0x10c2e; disasm getValue slot-8 calls
    // 0x10c18 and 0x10c2e): compares the +44 GetSetImpl `getValue` of both
    // described objects.
    let get = prop.getter.expect("bound getter at IDA 0x10a08");
    get(a) == get(b)
}

// 0x10c30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_10c30(prop: &AntialiasingProp, settings: &AntialiasingSettings) -> IntCallResult {
    // IDA 0x10c30 (decompiled 0x10c30..0x10c52; disasm getEnumValue slot-68
    // call 0x10c3e, `Type::getSingleton<int>()` 0x10c44, `placement_any<int>`
    // 0x10c52): tags the out slot with the int singleton, then stores the
    // enum value as int (host: generated_134 IntCallResult).
    let get = prop.getter.expect("bound getter at IDA 0x10a08");
    IntCallResult {
        type_name: "int",
        value: get(settings),
    }
}

// 0x10c54 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_10c54(prop: &AntialiasingProp, item: &mut AntialiasingItem, value: i32) {
    // IDA 0x10c54 (decompiled 0x10c54..0x10d88; disasm typeinfo-for-int fast
    // path 0x10cd2/0x10d50 `any_cast<int>`, `Variant::convert<int>` slow path
    // 0x10cd4..0x10d12, `setIntValue` slot-72 call 0x10d5e): coerces the
    // variant to int, then stores it. The variant-boxing dance has no host
    // effect; the caller passes the coerced int.
    let set = prop.setter.expect("bound setter at IDA 0x10a08");
    set(item, value);
}

// 0x10da4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_10da4(prop: &AntialiasingProp, src: &AntialiasingSettings, dst: &mut AntialiasingItem) {
    // IDA 0x10da4 (decompiled 0x10da4..0x10dc6; disasm getValue slot-8
    // 0x10db6, setValue slot-12 0x10dc6): reads the source through the +44
    // GetSetImpl getter, writes it through the setter.
    let get = prop.getter.expect("bound getter at IDA 0x10a08");
    let set = prop.setter.expect("bound setter at IDA 0x10a08");
    set(dst, get(src));
}

// 0x10dc8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::hasStringValue(void)const")]
pub fn stub_10dc8() -> bool {
    // IDA 0x10dc8 (disasm 0x10dca `MOVS R0,#1; BX LR`): enum properties
    // always have a string value.
    true
}

// 0x10dcc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_10dcc(
    prop: &AntialiasingProp,
    desc: &AntialiasingEnumDesc,
    settings: &AntialiasingSettings,
    out: &mut String,
) -> bool {
    // IDA 0x10dcc (decompiled 0x10dcc..0x10dee; disasm enum singleton fetch
    // at +48 0x10dd6, getValue slot-8 0x10dde, `EnumDesc::convertToString`
    // 0x10dee): renders the current enum value through the +48 enum
    // singleton (host: the desc param).
    let get = prop.getter.expect("bound getter at IDA 0x10a08");
    desc.value_to_string(get(settings), out)
}

// 0x10df0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_10df0(
    prop: &AntialiasingProp,
    desc: &AntialiasingEnumDesc,
    item: &mut AntialiasingItem,
    name: &str,
) -> bool {
    // IDA 0x10df0 (decompiled 0x10df0..0x10e2c; disasm enum singleton at +48
    // 0x10dfa, `Name::lookup` 0x10e02, `EnumDesc::convertToValue`
    // 0x10e10..0x10e16, setValue slot-12 0x10e26, return 1/0 at
    // 0x10e28/0x10e2c). `Name::lookup` interning has no host effect.
    match desc.lookup_value(name) {
        Some(value) => {
            let set = prop.setter.expect("bound setter at IDA 0x10a08");
            set(item, value);
            true
        }
        None => false,
    }
}

// 0x10e30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_10e30(prop: &AntialiasingProp, settings: &AntialiasingSettings, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x10e30 (decompiled 0x10e30..0x10e4e; disasm getValue slot-8
    // 0x10e3e, `clearValue` 0x10e44, tag `5` at +16 0x10e4a, value at +20
    // 0x10e4c, return 5 at 0x10e4e).
    let get = prop.getter.expect("bound getter at IDA 0x10a08");
    out.value_type = 0; // `clearValue` resets the pair first (0x10e44).
    out.value_type = 5; // int tag at +16 (0x10e4a).
    out.int_value = get(settings); // value at +20 (0x10e4c).
    5
}

// 0x10e50 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_10e50(
    prop: &AntialiasingProp,
    desc: &AntialiasingEnumDesc,
    item: &mut AntialiasingItem,
    xml: &XmlReadValue,
) {
    // IDA 0x10e50 (decompiled 0x10e50..0x11230 shape, cf. generated_134
    // 0x102cc): xsi:nil early-out (0x10e74); int pair -> `setIntValue`
    // (0x10ebc..0x10ecc = stub_111f8); string pair -> `Name::lookup` +
    // `convertToValue` + setValue (0x10eda..0x10f36) with the empty-string
    // `validate` fallback; anything else falls into `ReleaseAssert(false)`
    // (Reflection.h:359), which faults in the image — the host panics with
    // the same message.
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            if stub_111f8(prop, desc, item, *value) {
                return;
            }
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
        XmlReadValue::Text(text) => {
            if let Some(value) = desc.lookup_value(text) {
                let set = prop.setter.expect("bound setter at IDA 0x10a08");
                set(item, value);
                return;
            }
            if text.is_empty() {
                // Empty string -> `validate` virtual (slot-64); the host has
                // no validators, so this is a no-op.
                return;
            }
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
        XmlReadValue::Other => {
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
    }
}

// 0x11090 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11090(prop: &AntialiasingProp, desc: &AntialiasingEnumDesc, settings: &AntialiasingSettings) -> i32 {
    // IDA 0x11090 (decompiled 0x11090..0x110aa; disasm impl qword at +44
    // 0x11092, getValue slot-8 0x110a0, `convertToIndex` tail-call): the
    // +48 enum singleton word rides along in the impl qword's high half.
    let get = prop.getter.expect("bound getter at IDA 0x10a08");
    desc.convert_to_index(get(settings))
}

// 0x110ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_110ac(
    prop: &AntialiasingProp,
    desc: &AntialiasingEnumDesc,
    item: &mut AntialiasingItem,
    index: u32,
) -> bool {
    // IDA 0x110ac (decompiled 0x110ac..0x110dc; disasm enum singleton at +48
    // 0x110b2, count check 0x110be, table load `[[desc+144] + 4*index]`
    // 0x110c8, setValue slot-12 0x110d2, return 1/0 at 0x110d4/0x110dc).
    // Unlike 0x111f8, a `-1` hole is stored as-is.
    if (index as usize) < desc.index_to_value.len() {
        let value = desc.index_to_value[index as usize];
        let set = prop.setter.expect("bound setter at IDA 0x10a08");
        set(item, value);
        return true;
    }
    false
}

// 0x110e0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_110e0(prop: &AntialiasingProp, settings: &AntialiasingSettings) -> i32 {
    // IDA 0x110e0 (decompiled: the whole body is the +44 GetSetImpl slot-8
    // getValue call): the whole body is the getter — cf. generated_134
    // 0x1055c.
    let get = prop.getter.expect("bound getter at IDA 0x10a08");
    get(settings)
}

// 0x110e8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_110e8(
    prop: &AntialiasingProp,
    desc: &AntialiasingEnumDesc,
    item: &mut AntialiasingItem,
    value: i32,
) -> bool {
    // IDA 0x110e8 (decompiled 0x110e8..0x11130; disasm enum singleton at +48
    // 0x110f6, `__find_if` with `EnumDescriptor::equalValue` over
    // `[desc+28, desc+32)` 0x11112, setValue slot-12 0x11126, return 1/0 at
    // 0x11128/0x11130). was: boost::bind + __find_if -> iterator search.
    if desc.items.iter().any(|(v, _)| *v == value) {
        let set = prop.setter.expect("bound setter at IDA 0x10a08");
        set(item, value);
        return true;
    }
    false
}

// 0x11134 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11134(prop: &AntialiasingProp, desc: &AntialiasingEnumDesc, settings: &AntialiasingSettings) -> i32 {
    // IDA 0x11134 (decompiled 0x11134..0x11152 shape, cf. generated_134
    // 0x105b0; disasm impl qword at +44 0x1113a, getValue slot-8 0x11146,
    // `convertToItem` 0x11152): resolves the current value to its descriptor
    // item. Item pointers have no host meaning; the host returns the item's
    // position, or -1 when unmapped.
    let get = prop.getter.expect("bound getter at IDA 0x10a08");
    let value = get(settings);
    desc.items
        .iter()
        .position(|(v, _)| *v == value)
        .map(|i| i as i32)
        .unwrap_or(-1)
}

// 0x11154 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_11154(
    prop: &AntialiasingProp,
    desc: &AntialiasingEnumDesc,
    item: &mut AntialiasingItem,
    name: &str,
) -> bool {
    // IDA 0x11154 (decompiled 0x11154..0x11186 shape, cf. generated_134
    // 0x105d0; disasm `convertToValue` 0x1116a..0x11170, setValue slot-12
    // 0x11180, return 1/0 at 0x11182/0x11186): the `Name` overload twin of
    // 0x10df0.
    match desc.lookup_value(name) {
        Some(value) => {
            let set = prop.setter.expect("bound setter at IDA 0x10a08");
            set(item, value);
            true
        }
        None => false,
    }
}

// 0x11188 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToIndex(RBX::CRenderSettings::AntialiasingMode)const")]
pub fn stub_11188(desc: &AntialiasingEnumDesc, value: i32) -> i32 {
    // IDA 0x11188 (decompiled 0x11188..0x111f6; disasm `value>=0` assert chain
    // 0x1119c..0x111d2 at enumconverter.h:350, index-table load
    // `[[a1+156] + 4*value]` 0x111e2..0x111f2, default -1 at 0x111ea..0x111f6).
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: ../App/include/reflection/enumconverter.h line: 350"
        );
    }
    desc.convert_to_index(value)
}

// 0x111f8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_111f8(
    prop: &AntialiasingProp,
    desc: &AntialiasingEnumDesc,
    item: &mut AntialiasingItem,
    value: i32,
) -> bool {
    // IDA 0x111f8 (decompiled 0x111f8..0x11230 shape, cf. generated_134
    // 0x10674): `value>=0` (0x11202), legacy table bounds (0x11206..0x11214),
    // table load (0x11216), `-1` hole check (0x11220), setValue slot-12,
    // return 1/0. The `-1` check is what 0x110ac lacks.
    if value >= 0 && (value as usize) < desc.index_to_value.len() {
        let mapped = desc.index_to_value[value as usize];
        if mapped != -1 {
            let set = prop.setter.expect("bound setter at IDA 0x10a08");
            set(item, mapped);
            return true;
        }
    }
    false
}

// 0x11238 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isReadOnly(void)const")]
pub fn stub_11238(pair: &AntialiasingPair) -> bool {
    // IDA 0x11238 (decompiled `return 0`, 0x1123a): the enum member pointer
    // is bound, so never read-only.
    pair.is_read_only()
}

// 0x1123c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isWriteOnly(void)const")]
pub fn stub_1123c(pair: &AntialiasingPair) -> bool {
    // IDA 0x1123c (decompiled `return 0`, 0x1123e): the enum member pointer
    // is bound, so never write-only.
    pair.is_write_only()
}

// 0x11240 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11240(pair: &AntialiasingPair, settings: &AntialiasingSettings) -> i32 {
    // IDA 0x11240 (decompiled 0x11240..0x1126a shape, cf. 0x109b8; disasm
    // null-object split 0x11242..0x1125a, `a2-36` + 96 adjust
    // 0x11248..0x11250, virtual/indirect dispatch 0x1125c..0x11268, indirect
    // call): resolves the stored enum getter and calls it. A null getter
    // faults in the image; the host panics.
    let get = pair.getter.expect("bound getter at IDA 0x10a08");
    get(settings)
}

// 0x1126c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AntialiasingMode const&)const")]
pub fn stub_1126c(pair: &AntialiasingPair, item: &mut AntialiasingItem, value: i32) {
    // IDA 0x1126c (decompiled 0x1126c..0x1128e shape, cf. 0x109e4; disasm
    // `a2-36` adjust 0x11272..0x11274, setter fetch 0x11278..0x11280,
    // `>>1`/`&1` dispatch 0x11280..0x11288, indirect call): resolves the
    // stored enum setter and calls it. A null setter faults; host panics.
    let set = pair.setter.expect("bound setter at IDA 0x10a08");
    set(item, value);
}

// 0x11290 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::EnumPropDescriptor<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>(char const*,char const*,RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_11290(
    name: &str,
    category: &str,
    getter: fn(&ShadowSettings) -> i32,
    setter: fn(&mut ShadowItem, i32),
    attributes: u32,
    permissions: u32,
) -> ShadowProp {
    // IDA 0x11290 (decompiled 0x11290..0x11406 shape, cf. 0x10a08):
    // classDescriptor, ShadowMode EnumDesc Singleton init, base ctor, enum
    // desc at +40, GetSetImpl at +44, attribute fixups at +28. Host: the
    // desc travels separately; the singleton init has no host effect.
    let _ = crate::generated_134::stub_fa00();
    ShadowProp {
        name: name.to_owned(),
        category: category.to_owned(),
        getter: Some(getter),
        setter: Some(setter),
        attributes,
        permissions,
        enum_type: "ShadowMode",
    }
}

// 0x11444 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
pub fn stub_11444() {
    // IDA 0x11444: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x11470 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isReadOnly(void)const")]
pub fn stub_11470(prop: &ShadowProp) -> bool {
    // IDA 0x11470 (same delegate-to-GetSetImpl shape as 0x10be8): slot-0
    // virtual.
    prop.is_read_only()
}

// 0x11480 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isWriteOnly(void)const")]
pub fn stub_11480(prop: &ShadowProp) -> bool {
    // IDA 0x11480 (same shape as 0x10bf8): slot-1 virtual.
    prop.is_write_only()
}

// 0x11490 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11490(prop: &ShadowProp, a: &ShadowSettings, b: &ShadowSettings) -> bool {
    // IDA 0x11490 (same getValue-slot-8 comparison shape as 0x10c08).
    let get = prop.getter.expect("bound getter at IDA 0x11290");
    get(a) == get(b)
}

// 0x114b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_114b8(prop: &ShadowProp, settings: &ShadowSettings) -> IntCallResult {
    // IDA 0x114b8 (same getEnumValue + int-singleton + placement_any<int>
    // shape as 0x10c30).
    let get = prop.getter.expect("bound getter at IDA 0x11290");
    IntCallResult {
        type_name: "int",
        value: get(settings),
    }
}

// 0x114dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_114dc(prop: &ShadowProp, item: &mut ShadowItem, value: i32) {
    // IDA 0x114dc (same any_cast/convert + setIntValue shape as 0x10c54).
    let set = prop.setter.expect("bound setter at IDA 0x11290");
    set(item, value);
}

// 0x1162c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_1162c(prop: &ShadowProp, src: &ShadowSettings, dst: &mut ShadowItem) {
    // IDA 0x1162c (same getValue/setValue shape as 0x10da4).
    let get = prop.getter.expect("bound getter at IDA 0x11290");
    let set = prop.setter.expect("bound setter at IDA 0x11290");
    set(dst, get(src));
}

// 0x11650 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::hasStringValue(void)const")]
pub fn stub_11650() -> bool {
    // IDA 0x11650 (same `MOVS R0,#1` shape as 0x10dc8): enum properties
    // always have a string value.
    true
}

// 0x11654 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11654(
    prop: &ShadowProp,
    desc: &ShadowEnumDesc,
    settings: &ShadowSettings,
    out: &mut String,
) -> bool {
    // IDA 0x11654 (same enum-singleton + getValue + convertToString shape as
    // 0x10dcc).
    let get = prop.getter.expect("bound getter at IDA 0x11290");
    desc.value_to_string(get(settings), out)
}

// 0x11678 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_11678(
    prop: &ShadowProp,
    desc: &ShadowEnumDesc,
    item: &mut ShadowItem,
    name: &str,
) -> bool {
    // IDA 0x11678 (same Name::lookup + convertToValue + setValue shape as
    // 0x10df0).
    match desc.lookup_value(name) {
        Some(value) => {
            let set = prop.setter.expect("bound setter at IDA 0x11290");
            set(item, value);
            true
        }
        None => false,
    }
}

// 0x116b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_116b8(prop: &ShadowProp, settings: &ShadowSettings, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x116b8 (same getValue + clearValue + tag-5 + value shape as
    // 0x10e30).
    let get = prop.getter.expect("bound getter at IDA 0x11290");
    out.value_type = 0; // `clearValue` resets the pair first.
    out.value_type = 5; // int tag at +16.
    out.int_value = get(settings); // value at +20.
    5
}

// 0x116d8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_116d8(
    prop: &ShadowProp,
    desc: &ShadowEnumDesc,
    item: &mut ShadowItem,
    xml: &XmlReadValue,
) {
    // IDA 0x116d8 (same xsi:nil / int-pair / string-pair / ReleaseAssert
    // shape as 0x10e50, cf. generated_134 0x102cc): int pair ->
    // `setIntValue` (= stub_11a80); string pair -> lookup + setValue with
    // the empty-string `validate` fallback; else `ReleaseAssert(false)`
    // (Reflection.h:359).
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            if stub_11a80(prop, desc, item, *value) {
                return;
            }
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
        XmlReadValue::Text(text) => {
            if let Some(value) = desc.lookup_value(text) {
                let set = prop.setter.expect("bound setter at IDA 0x11290");
                set(item, value);
                return;
            }
            if text.is_empty() {
                return;
            }
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
        XmlReadValue::Other => {
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
    }
}

// 0x11918 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11918(prop: &ShadowProp, desc: &ShadowEnumDesc, settings: &ShadowSettings) -> i32 {
    // IDA 0x11918 (same getValue + convertToIndex shape as 0x11090).
    let get = prop.getter.expect("bound getter at IDA 0x11290");
    desc.convert_to_index(get(settings))
}

// 0x11934 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_11934(
    prop: &ShadowProp,
    desc: &ShadowEnumDesc,
    item: &mut ShadowItem,
    index: u32,
) -> bool {
    // IDA 0x11934 (same count-check + table-load + setValue shape as 0x110ac;
    // a `-1` hole is stored as-is).
    if (index as usize) < desc.index_to_value.len() {
        let value = desc.index_to_value[index as usize];
        let set = prop.setter.expect("bound setter at IDA 0x11290");
        set(item, value);
        return true;
    }
    false
}

// 0x11968 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11968(prop: &ShadowProp, settings: &ShadowSettings) -> i32 {
    // IDA 0x11968 (decompiled: the whole body is the +44 GetSetImpl slot-8
    // getValue call): the whole body is the getter — cf. 0x110e0.
    let get = prop.getter.expect("bound getter at IDA 0x11290");
    get(settings)
}

// 0x11970 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_11970(
    prop: &ShadowProp,
    desc: &ShadowEnumDesc,
    item: &mut ShadowItem,
    value: i32,
) -> bool {
    // IDA 0x11970 (decompiled 0x11970..0x119b8; disasm enum singleton at +48
    // 0x1197e, `__find_if` with `EnumDescriptor::equalValue` 0x1199a, setValue
    // slot-12 0x119ae, return 1/0 at 0x119b0/0x119b8). was: boost::bind +
    // __find_if -> iterator search.
    if desc.items.iter().any(|(v, _)| *v == value) {
        let set = prop.setter.expect("bound setter at IDA 0x11290");
        set(item, value);
        return true;
    }
    false
}

// 0x119bc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_119bc(prop: &ShadowProp, desc: &ShadowEnumDesc, settings: &ShadowSettings) -> i32 {
    // IDA 0x119bc (decompiled 0x119bc..0x119da; disasm impl qword at +44
    // 0x119c2, getValue slot-8 0x119ce, `convertToItem` 0x119da): position
    // or -1 — cf. 0x11134.
    let get = prop.getter.expect("bound getter at IDA 0x11290");
    let value = get(settings);
    desc.items
        .iter()
        .position(|(v, _)| *v == value)
        .map(|i| i as i32)
        .unwrap_or(-1)
}

// 0x119dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_119dc(
    prop: &ShadowProp,
    desc: &ShadowEnumDesc,
    item: &mut ShadowItem,
    name: &str,
) -> bool {
    // IDA 0x119dc (decompiled 0x119dc..0x11a0e; disasm `convertToValue`
    // 0x119f2..0x119f8, setValue slot-12 0x11a08, return 1/0 at
    // 0x11a0a/0x11a0e): the `Name` overload twin of 0x11678.
    match desc.lookup_value(name) {
        Some(value) => {
            let set = prop.setter.expect("bound setter at IDA 0x11290");
            set(item, value);
            true
        }
        None => false,
    }
}

// 0x11a10 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToIndex(RBX::CRenderSettings::ShadowMode)const")]
pub fn stub_11a10(desc: &ShadowEnumDesc, value: i32) -> i32 {
    // IDA 0x11a10 (decompiled 0x11a10..0x11a7e; disasm `value>=0` assert chain
    // 0x11a24..0x11a5a at enumconverter.h:350, index-table load
    // 0x11a6a..0x11a7a, default -1 at 0x11a72..0x11a7e).
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: ../App/include/reflection/enumconverter.h line: 350"
        );
    }
    desc.convert_to_index(value)
}

// 0x11a80 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_11a80(
    prop: &ShadowProp,
    desc: &ShadowEnumDesc,
    item: &mut ShadowItem,
    value: i32,
) -> bool {
    // IDA 0x11a80 (decompiled 0x11a80..0x11abc; disasm `value>=0` 0x11a8a,
    // legacy table bounds 0x11a8e..0x11a9c, table load 0x11a9e, `-1` hole
    // check 0x11aa8, setValue slot-12 0x11ab4, return 1/0 at 0x11ab6/0x11abc).
    // The `-1` check is what 0x11934 lacks.
    if value >= 0 && (value as usize) < desc.index_to_value.len() {
        let mapped = desc.index_to_value[value as usize];
        if mapped != -1 {
            let set = prop.setter.expect("bound setter at IDA 0x11290");
            set(item, mapped);
            return true;
        }
    }
    false
}

// 0x11ac0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isReadOnly(void)const")]
pub fn stub_11ac0(pair: &ShadowPair) -> bool {
    // IDA 0x11ac0 (decompiled `return 0`, 0x11ac2): the enum member pointer
    // is bound, so never read-only.
    pair.is_read_only()
}

// 0x11ac4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isWriteOnly(void)const")]
pub fn stub_11ac4(pair: &ShadowPair) -> bool {
    // IDA 0x11ac4 (decompiled `return 0`): the enum member pointer is bound,
    // so never write-only.
    pair.is_write_only()
}

// 0x11ac8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11ac8(pair: &ShadowPair, settings: &ShadowSettings) -> i32 {
    // IDA 0x11ac8 (decompiled 0x11ac8..0x11af2; disasm null-object split
    // 0x11aca..0x11ae2, `a2-36` + 96 adjust 0x11ad0..0x11ad8,
    // virtual/indirect dispatch 0x11ae4..0x11af0, indirect call): resolves
    // the stored enum getter and calls it. A null getter faults in the
    // image; the host panics.
    let get = pair.getter.expect("bound getter at IDA 0x11290");
    get(settings)
}

// 0x11af4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ShadowMode const&)const")]
pub fn stub_11af4(pair: &ShadowPair, item: &mut ShadowItem, value: i32) {
    // IDA 0x11af4 (decompiled 0x11af4..0x11b16; disasm `a2-36` adjust
    // 0x11afa..0x11afc, setter fetch 0x11b00..0x11b08, `>>1`/`&1` dispatch
    // 0x11b08..0x11b10, indirect call): resolves the stored enum setter and
    // calls it. A null setter faults; host panics.
    let set = pair.setter.expect("bound setter at IDA 0x11290");
    set(item, value);
}

// 0x11b18 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,std::string  CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_11b18(
    name: &str,
    category: &str,
    attributes: u32,
    permissions: u32,
) -> StringBoundProp {
    // IDA 0x11b18 (decompiled 0x11b18..0x11c58; disasm classDescriptor
    // 0x11b3e, `TypedPropertyDescriptor<string>` ctor 0x11ba0, vtable install
    // 0x11bbe/0x11bc2, BoundPropGetSet alloc + member-data offset store
    // 0x11bcc..): registers the bound string member against the
    // RenderSettings class descriptor (host: generated_134 stub_fa00).
    let _ = crate::generated_134::stub_fa00();
    StringBoundProp {
        name: name.to_owned(),
        category: category.to_owned(),
        attributes,
        permissions,
    }
}

// 0x11ca8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
pub fn stub_11ca8(prop: &StringBoundProp) -> bool {
    // IDA 0x11ca8 (decompiled `return 0`, 0x11caa): Mutability::1 is
    // read-write, so never read-only. Attributes travel on the prop; the
    // host answers from the mutability the ctor fixed.
    let _ = prop;
    false
}

// 0x11cac — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
pub fn stub_11cac(prop: &StringBoundProp) -> bool {
    // IDA 0x11cac (decompiled `return 0`, 0x11cae): same — never write-only.
    let _ = prop;
    false
}

// 0x11cb0 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11cb0(item: &StringItem) -> String {
    // IDA 0x11cb0 (decompiled 0x11cb0..0x11cc6; disasm null-object split
    // 0x11cb6..0x11cb8 `a3-36`, string copy-construct 0x11cc6): copies the
    // bound member into the out string.
    item.text.clone()
}

// 0x11cc8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_11cc8(item: &mut StringItem, value: &str) {
    // IDA 0x11cc8 (decompiled 0x11cc8..0x11d2a; disasm null-object split
    // 0x11cd6..0x11cd8, member address 0x11ce0, `compare` 0x11ce6, `assign`
    // on change 0x11cf0, change-callback check + indirect call
    // 0x11cf4..0x11d12, `raisePropertyChanged` 0x11d2a; equal strings return
    // 0 at 0x11d1a with no notification): assigns only on change, then
    // notifies. The host has no change-callback/signal wired to this slot,
    // so the notify edge is a comment.
    if item.text != value {
        item.text.clear();
        item.text.push_str(value);
        // raisePropertyChanged would fire here (0x11d2a); no host slot.
    }
}

// 0x11d30 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::EnumPropDescriptor<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>(char const*,char const*,RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_11d30(
    name: &str,
    category: &str,
    getter: fn(&AASamplesSettings) -> i32,
    setter: fn(&mut AASamplesItem, i32),
    attributes: u32,
    permissions: u32,
) -> AASamplesProp {
    // IDA 0x11d30 (decompiled 0x11d30..0x11ee0 shape, cf. 0x10a08):
    // classDescriptor, AASamples EnumDesc Singleton init, base ctor, enum
    // desc at +40, GetSetImpl at +44, attribute fixups at +28.
    let _ = crate::generated_134::stub_fa00();
    AASamplesProp {
        name: name.to_owned(),
        category: category.to_owned(),
        getter: Some(getter),
        setter: Some(setter),
        attributes,
        permissions,
        enum_type: "AASamples",
    }
}

// 0x11ee4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
pub fn stub_11ee4() {
    // IDA 0x11ee4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x11f10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isReadOnly(void)const")]
pub fn stub_11f10(prop: &AASamplesProp) -> bool {
    // IDA 0x11f10 (same delegate-to-GetSetImpl shape as 0x10be8): slot-0
    // virtual.
    prop.is_read_only()
}

// 0x11f20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isWriteOnly(void)const")]
pub fn stub_11f20(prop: &AASamplesProp) -> bool {
    // IDA 0x11f20 (same shape as 0x10bf8): slot-1 virtual.
    prop.is_write_only()
}

// 0x11f30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11f30(prop: &AASamplesProp, a: &AASamplesSettings, b: &AASamplesSettings) -> bool {
    // IDA 0x11f30 (same getValue-slot-8 comparison shape as 0x10c08).
    let get = prop.getter.expect("bound getter at IDA 0x11d30");
    get(a) == get(b)
}

// 0x11f58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_11f58(prop: &AASamplesProp, settings: &AASamplesSettings) -> IntCallResult {
    // IDA 0x11f58 (same getEnumValue + int-singleton + placement_any<int>
    // shape as 0x10c30).
    let get = prop.getter.expect("bound getter at IDA 0x11d30");
    IntCallResult {
        type_name: "int",
        value: get(settings),
    }
}

// 0x11f7c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_11f7c(prop: &AASamplesProp, item: &mut AASamplesItem, value: i32) {
    // IDA 0x11f7c (same any_cast/convert + setIntValue shape as 0x10c54).
    let set = prop.setter.expect("bound setter at IDA 0x11d30");
    set(item, value);
}

// 0x120cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_120cc(prop: &AASamplesProp, src: &AASamplesSettings, dst: &mut AASamplesItem) {
    // IDA 0x120cc (same getValue/setValue shape as 0x10da4).
    let get = prop.getter.expect("bound getter at IDA 0x11d30");
    let set = prop.setter.expect("bound setter at IDA 0x11d30");
    set(dst, get(src));
}

// 0x120f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::hasStringValue(void)const")]
pub fn stub_120f0() -> bool {
    // IDA 0x120f0 (same `MOVS R0,#1` shape): enum properties always have a
    // string value.
    true
}

// 0x120f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_120f4(prop: &AASamplesProp, settings: &AASamplesSettings, out: &mut String) {
    // IDA 0x120f4 (decompiled 0x120f4..0x12110; disasm enum singleton fetch
    // at +48 0x120fe, getValue slot-8 0x12106, void `convertToString`
    // 0x12110): unlike 0x10dcc this instantiation calls the void
    // (string, value) overload — asserts + assign-or-empty (host:
    // generated_123 stub_e78c).
    let get = prop.getter.expect("bound getter at IDA 0x11d30");
    crate::generated_123::stub_e78c(get(settings), out);
}

// 0x12118 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_12118(
    prop: &AASamplesProp,
    desc: &AASamplesEnumDesc,
    item: &mut AASamplesItem,
    name: &str,
) -> bool {
    // IDA 0x12118 (decompiled 0x12118..0x12154; disasm enum singleton at +48
    // 0x12122, `Name::lookup` 0x1212a, `EnumDesc::convertToValue`
    // 0x12138..0x1213e, setValue slot-12 0x1214e, return 1/0 at
    // 0x12150/0x12154).
    match desc.lookup_value(name) {
        Some(value) => {
            let set = prop.setter.expect("bound setter at IDA 0x11d30");
            set(item, value);
            true
        }
        None => false,
    }
}

// 0x12158 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_12158(prop: &AASamplesProp, settings: &AASamplesSettings, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x12158 (decompiled 0x12158..0x12176; disasm getValue slot-8
    // 0x12166, `clearValue` 0x1216c, tag `5` at +16 0x12172, value at +20
    // 0x12174, return 5 at 0x12176).
    let get = prop.getter.expect("bound getter at IDA 0x11d30");
    out.value_type = 0; // `clearValue` resets the pair first (0x1216c).
    out.value_type = 5; // int tag at +16 (0x12172).
    out.int_value = get(settings); // value at +20 (0x12174).
    5
}

// 0x12178 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_12178(
    prop: &AASamplesProp,
    desc: &AASamplesEnumDesc,
    item: &mut AASamplesItem,
    xml: &XmlReadValue,
) {
    // IDA 0x12178 (decompiled 0x12178..0x12370 shape, cf. 0x10e50):
    // xsi:nil early-out (0x1219c); int pair -> `setIntValue` (0x121d2.. =
    // stub_12520); string pair -> lookup + setValue with the empty-string
    // `validate` fallback; else `ReleaseAssert(false)` (Reflection.h:359).
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            if stub_12520(prop, desc, item, *value) {
                return;
            }
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
        XmlReadValue::Text(text) => {
            if let Some(value) = desc.lookup_value(text) {
                let set = prop.setter.expect("bound setter at IDA 0x11d30");
                set(item, value);
                return;
            }
            if text.is_empty() {
                return;
            }
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
        XmlReadValue::Other => {
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
    }
}

// 0x123b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_123b8(prop: &AASamplesProp, desc: &AASamplesEnumDesc, settings: &AASamplesSettings) -> i32 {
    // IDA 0x123b8 (decompiled 0x123b8..0x123d2 shape, cf. 0x11090; disasm impl
    // qword at +44 0x123ba, getValue slot-8 0x123c8, `convertToIndex`
    // tail-call).
    let get = prop.getter.expect("bound getter at IDA 0x11d30");
    desc.convert_to_index(get(settings))
}

// 0x123d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_123d4(
    prop: &AASamplesProp,
    desc: &AASamplesEnumDesc,
    item: &mut AASamplesItem,
    index: u32,
) -> bool {
    // IDA 0x123d4 (decompiled 0x123d4..0x12404; disasm enum singleton at +48
    // 0x123da, count check 0x123e6, table load 0x123f0, setValue slot-12
    // 0x123fa, return 1/0 at 0x123fc/0x12404; a `-1` hole is stored as-is).
    if (index as usize) < desc.index_to_value.len() {
        let value = desc.index_to_value[index as usize];
        let set = prop.setter.expect("bound setter at IDA 0x11d30");
        set(item, value);
        return true;
    }
    false
}

// 0x12408 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12408(prop: &AASamplesProp, settings: &AASamplesSettings) -> i32 {
    // IDA 0x12408 (decompiled: the whole body is the +44 GetSetImpl slot-8
    // getValue call): the whole body is the getter — cf. 0x110e0.
    let get = prop.getter.expect("bound getter at IDA 0x11d30");
    get(settings)
}

// 0x12410 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_12410(
    prop: &AASamplesProp,
    desc: &AASamplesEnumDesc,
    item: &mut AASamplesItem,
    value: i32,
) -> bool {
    // IDA 0x12410 (decompiled 0x12410..0x12458; disasm enum singleton at +48
    // 0x1241e, `__find_if` with `EnumDescriptor::equalValue` 0x1243a, setValue
    // slot-12 0x1244e, return 1/0 at 0x12450/0x12458). was: boost::bind +
    // __find_if -> iterator search.
    if desc.items.iter().any(|(v, _)| *v == value) {
        let set = prop.setter.expect("bound setter at IDA 0x11d30");
        set(item, value);
        return true;
    }
    false
}

// 0x1245c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1245c(prop: &AASamplesProp, desc: &AASamplesEnumDesc, settings: &AASamplesSettings) -> i32 {
    // IDA 0x1245c (decompiled 0x1245c..0x1247a; disasm impl qword at +44
    // 0x12462, getValue slot-8 0x1246e, `convertToItem` 0x1247a): position
    // or -1 — cf. 0x11134.
    let get = prop.getter.expect("bound getter at IDA 0x11d30");
    let value = get(settings);
    desc.items
        .iter()
        .position(|(v, _)| *v == value)
        .map(|i| i as i32)
        .unwrap_or(-1)
}

// 0x1247c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_1247c(
    prop: &AASamplesProp,
    desc: &AASamplesEnumDesc,
    item: &mut AASamplesItem,
    name: &str,
) -> bool {
    // IDA 0x1247c (decompiled 0x1247c..0x124ae; disasm `convertToValue`
    // 0x12492..0x12498, setValue slot-12 0x124a8, return 1/0 at
    // 0x124aa/0x124ae): the `Name` overload twin of 0x12118.
    match desc.lookup_value(name) {
        Some(value) => {
            let set = prop.setter.expect("bound setter at IDA 0x11d30");
            set(item, value);
            true
        }
        None => false,
    }
}

// 0x124b0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToIndex(RBX::CRenderSettings::AASamples)const")]
pub fn stub_124b0(desc: &AASamplesEnumDesc, value: i32) -> i32 {
    // IDA 0x124b0 (decompiled 0x124b0..0x1251e; disasm `value>=0` assert chain
    // 0x124c4..0x124fa at enumconverter.h:350, index-table load
    // 0x1250a..0x1251a, default -1 at 0x12512..0x1251e).
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: ../App/include/reflection/enumconverter.h line: 350"
        );
    }
    desc.convert_to_index(value)
}

// 0x12520 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_12520(
    prop: &AASamplesProp,
    desc: &AASamplesEnumDesc,
    item: &mut AASamplesItem,
    value: i32,
) -> bool {
    // IDA 0x12520 (decompiled 0x12520..0x1255c shape, cf. 0x111f8; disasm
    // `value>=0` 0x1252a, legacy table bounds 0x1252e..0x1253c, table load
    // 0x1253e, `-1` hole check 0x12548, setValue slot-12 0x12554, return 1/0
    // at 0x12556/0x1255c). The `-1` check is what 0x123d4 lacks.
    if value >= 0 && (value as usize) < desc.index_to_value.len() {
        let mapped = desc.index_to_value[value as usize];
        if mapped != -1 {
            let set = prop.setter.expect("bound setter at IDA 0x11d30");
            set(item, mapped);
            return true;
        }
    }
    false
}

// 0x12560 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isReadOnly(void)const")]
pub fn stub_12560(pair: &AASamplesPair) -> bool {
    // IDA 0x12560 (decompiled `return 0`, 0x12562): the enum member pointer
    // is bound, so never read-only.
    pair.is_read_only()
}

// 0x12564 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isWriteOnly(void)const")]
pub fn stub_12564(pair: &AASamplesPair) -> bool {
    // IDA 0x12564 (decompiled `return 0`, 0x12566): the enum member pointer
    // is bound, so never write-only.
    pair.is_write_only()
}

// 0x12568 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12568(pair: &AASamplesPair, settings: &AASamplesSettings) -> i32 {
    // IDA 0x12568 (decompiled 0x12568..0x12592; disasm null-object split
    // 0x1256a..0x12582, `a2-36` + 96 adjust 0x12570..0x12578,
    // virtual/indirect dispatch 0x12584..0x12590, indirect call): resolves
    // the stored enum getter and calls it. A null getter faults in the
    // image; the host panics.
    let get = pair.getter.expect("bound getter at IDA 0x11d30");
    get(settings)
}

// 0x12594 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_12594(pair: &AASamplesPair, item: &mut AASamplesItem, value: i32) {
    // IDA 0x12594 (decompiled 0x12594..0x125b6; disasm `a2-36` adjust
    // 0x1259a..0x1259c, setter fetch 0x125a0..0x125a8, `>>1`/`&1` dispatch
    // 0x125a8..0x125ac, indirect call): resolves the stored enum setter and
    // calls it. A null setter faults; host panics.
    let set = pair.setter.expect("bound setter at IDA 0x11d30");
    set(item, value);
}

// 0x125b8 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,bool CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_125b8(
    name: &str,
    category: &str,
    attributes: u32,
    permissions: u32,
) -> BoolBoundProp {
    // IDA 0x125b8 (same BoundProp shape as 0x11b18 — classDescriptor,
    // `TypedPropertyDescriptor<bool>` ctor, vtable install, BoundPropGetSet
    // alloc + member-data offset store): registers the bound bool member.
    let _ = crate::generated_134::stub_fa00();
    BoolBoundProp {
        name: name.to_owned(),
        category: category.to_owned(),
        attributes,
        permissions,
    }
}

// 0x12748 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
pub fn stub_12748(prop: &BoolBoundProp) -> bool {
    // IDA 0x12748 (decompiled `return 0`, 0x1274a): Mutability::1 is
    // read-write, so never read-only.
    let _ = prop;
    false
}

// 0x1274c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
pub fn stub_1274c(prop: &BoolBoundProp) -> bool {
    // IDA 0x1274c (decompiled `return 0`, 0x1274e): same — never write-only.
    let _ = prop;
    false
}

// 0x12750 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12750(item: &BoolBoundItem) -> bool {
    // IDA 0x12750 (decompiled 0x12750..0x12758; disasm member load
    // `*(offset + obj - 36)` at 0x12758): reads the bound bool member.
    item.flag
}

// 0x1275c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_1275c(item: &mut BoolBoundItem, value: bool) {
    // IDA 0x1275c (decompiled 0x1275c..0x127a6; disasm null-object split
    // 0x12764..0x12766, offset load 0x1276a, compare 0x12774, assign on
    // change 0x12778, change-callback check + indirect call 0x1277a..0x12798,
    // `raisePropertyChanged` 0x127a6; equal values return the offset at
    // 0x12776 with no notification): assigns only on change, then notifies.
    // The host has no change-callback/signal wired to this slot.
    if item.flag != value {
        item.flag = value;
        // raisePropertyChanged would fire here (0x127a6); no host slot.
    }
}

// 0x127ac — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_127ac(
    name: &str,
    category: &str,
    getter: fn(&BoolCSettings) -> bool,
    setter: fn(&mut BoolItem, bool),
    attributes: u32,
    permissions: u32,
) -> BoolCProp {
    // IDA 0x127ac (decompiled 0x127ac..0x1288e; disasm classDescriptor
    // 0x127d4, GetSetImpl alloc + member-fn stores 0x127da..0x12814,
    // `TypedPropertyDescriptor<bool>` ctor 0x12852, vtable install 0x12870):
    // registers the bool get/set pair (getter on CRenderSettings) — cf.
    // IntProp at 0x1089c.
    let _ = crate::generated_134::stub_fa00();
    BoolCProp {
        name: name.to_owned(),
        category: category.to_owned(),
        getter: Some(getter),
        setter: Some(setter),
        attributes,
        permissions,
    }
}

// 0x128c0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
pub fn stub_128c0(prop: &BoolCProp) -> bool {
    // IDA 0x128c0 (decompiled `return 0`, 0x128c2): the bool member pointer
    // is bound at 0x127ac, so never read-only.
    prop.getter.is_none()
}

#[cfg(test)]
mod batch4_tests {
    use super::*;

    fn res_get(settings: &ResolutionSettings) -> i32 {
        settings.resolution_preference
    }

    fn res_set(item: &mut ResolutionItem, value: i32) {
        item.resolution_preset = value;
    }

    fn flag_get(item: &BoolItem) -> bool {
        item.flag
    }

    fn flag_set(item: &mut BoolItem, value: bool) {
        item.flag = value;
    }

    #[test]
    fn resolution_pair_getset_roundtrip() {
        // IDA 0x106bc getValue + 0x106e8 setValue: member-pointer thunk
        // details have no host effect; the calls are the whole body.
        let pair = ResolutionPresetPair {
            getter: Some(res_get),
            setter: Some(res_set),
        };
        let settings = ResolutionSettings {
            resolution_preference: 4,
        };
        assert_eq!(stub_106bc(&pair, &settings), 4);
        let mut item = ResolutionItem::default();
        stub_106e8(&pair, &mut item, 4);
        assert_eq!(item.resolution_preset, 4);
    }

    #[test]
    fn bool_prop_getset_roundtrip() {
        // IDA 0x1070c ctor + 0x1084c/0x10850/0x10854 virtuals.
        let prop = stub_1070c("ShowGrid", "Rendering", flag_get, flag_set, 0, 0);
        assert_eq!(prop.name, "ShowGrid");
        assert_eq!(prop.category, "Rendering");
        assert!(!stub_1084c(&prop));
        assert!(!stub_10850(&prop));
        let item = BoolItem { flag: true };
        assert!(stub_10854(&prop, &item));
        let mut target = BoolItem::default();
        (prop.setter.expect("bound"))(&mut target, true);
        assert!(stub_10854(&prop, &target));
    }
}
