//! audio generated_136 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Soundscape exhausted (2398 distinct) — filler EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x128c4..0x14ccc EA-sorted asc filler after 0x128c0, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

use crate::generated::flog_asserts;
use crate::generated_134::{IntCallResult, XmlIntSlot, XmlReadValue};

/// Host carrier for `PropDescriptor<CRenderSettingsItem,bool>` GetSetImpl with
/// the getter on `CRenderSettings` (IDA 0x128c8/0x128fc) — cf.
/// `AntialiasingPair` in generated_135.rs.
pub struct BoolPair {
    pub getter: Option<fn(&BoolSettings) -> bool>,
    pub setter: Option<fn(&mut BoolItem, bool)>,
}

impl BoolPair {
    /// IDA 0x128c4 (`return 0` shape): a bound getter is never read-only.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// A bound setter is never write-only.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// CRenderSettings bool slot read by the bool getter.
#[derive(Default)]
pub struct BoolSettings {
    pub flag: bool,
}

/// CRenderSettingsItem bool slot written by the bool setter.
#[derive(Default)]
pub struct BoolItem {
    pub flag: bool,
}

/// Host carrier for `EnumPropDescriptor<CRenderSettingsItem,QualityLevel>`
/// (IDA 0x12920: same shape as generated_135 0x10a08 — classDescriptor,
/// enum Singleton init, base ctor, enum desc at +40, GetSetImpl at +44,
/// fixups at +28).
pub struct QualityProp {
    pub name: String,
    pub category: String,
    pub getter: Option<fn(&QualitySettings) -> i32>,
    pub setter: Option<fn(&mut QualityItem, i32)>,
    pub attributes: u32,
    pub permissions: u32,
    pub enum_type: &'static str,
}

impl QualityProp {
    /// IDA 0x12b00 (same delegate-to-GetSetImpl shape): slot-0 virtual.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0x12b10: slot-1 virtual.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// CRenderSettings QualityLevel slot read by the 0x12920 getter.
#[derive(Default)]
pub struct QualitySettings {
    pub level: i32,
}

/// CRenderSettingsItem QualityLevel slot written by the 0x12920 setter.
#[derive(Default)]
pub struct QualityItem {
    pub level: i32,
}

/// Host model of `EnumDesc<QualityLevel>` for IDA 0x12b48..0x12d48 (mirrors
/// `AntialiasingEnumDesc` in generated_135.rs).
#[derive(Default)]
pub struct QualityEnumDesc {
    pub items: Vec<(i32, String)>,
    pub index_to_value: Vec<i32>,
}

impl QualityEnumDesc {
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

/// Host carrier for `PropDescriptor<CRenderSettingsItem,QualityLevel>`
/// GetSetImpl (IDA 0x13158/0x13184): the same getter/setter pair the
/// QualityLevel EnumPropDescriptor keeps at +44.
pub struct QualityPair {
    pub getter: Option<fn(&QualitySettings) -> i32>,
    pub setter: Option<fn(&mut QualityItem, i32)>,
}

impl QualityPair {
    /// IDA 0x13150 (decompiled `return 0`, 0x13152): a bound getter is never
    /// read-only.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0x13154 (decompiled `return 0`, 0x13156): a bound setter is never
    /// write-only.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// Host carrier for `EnumPropDescriptor<CRenderSettingsItem,FrameRateManagerMode>`
/// (IDA 0x131a8: same shape as 0x12920 — classDescriptor, enum Singleton
/// init, base ctor, enum desc at +40, GetSetImpl at +44, fixups at +28).
pub struct FrameRateProp {
    pub name: String,
    pub category: String,
    pub getter: Option<fn(&FrameRateSettings) -> i32>,
    pub setter: Option<fn(&mut FrameRateItem, i32)>,
    pub attributes: u32,
    pub permissions: u32,
    pub enum_type: &'static str,
}

impl FrameRateProp {
    /// IDA 0x13388 (same delegate-to-GetSetImpl shape): slot-0 virtual.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0x13398: slot-1 virtual.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// CRenderSettings FrameRateManagerMode slot read by the 0x131a8 getter.
#[derive(Default)]
pub struct FrameRateSettings {
    pub mode: i32,
}

/// CRenderSettingsItem FrameRateManagerMode slot written by the 0x131a8 setter.
#[derive(Default)]
pub struct FrameRateItem {
    pub mode: i32,
}

/// Host model of `EnumDesc<FrameRateManagerMode>` for IDA 0x133d0.. (mirrors
/// `QualityEnumDesc` above).
#[derive(Default)]
pub struct FrameRateEnumDesc {
    pub items: Vec<(i32, String)>,
    pub index_to_value: Vec<i32>,
}

impl FrameRateEnumDesc {
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

// 0x128c4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_128c4(pair: &BoolPair) -> bool {
    // IDA 0x128c4 (`return 0` shape, cf. 0x128c0): the bool member pointer
    // is bound, so never write-only. (isWriteOnly follows isReadOnly at
    // 0x128c0 in generated_135.rs.)
    pair.is_write_only()
}

// 0x128c8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_128c8(pair: &BoolPair, settings: &BoolSettings) -> bool {
    // IDA 0x128c8 (decompiled 0x128c8..0x128fa shape, cf. 0x109b8; disasm
    // null-object split, `a2-36` + 96 adjust, virtual/indirect dispatch,
    // indirect call): resolves the stored `bool (CRenderSettings::*)() const`
    // and calls it. A null getter faults in the image; the host panics.
    let get = pair.getter.expect("bound bool getter");
    get(settings)
}

// 0x128fc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_128fc(pair: &BoolPair, item: &mut BoolItem, value: bool) {
    // IDA 0x128fc (decompiled 0x128fc..0x1291e shape, cf. 0x109e4; disasm
    // `a2-36` adjust, setter fetch, `>>1`/`&1` dispatch, indirect call):
    // resolves the stored `void (CRenderSettingsItem::*)(bool)` and calls
    // it. A null setter faults; host panics.
    let set = pair.setter.expect("bound bool setter");
    set(item, value);
}

// 0x12920 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::EnumPropDescriptor<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>(char const*,char const*,RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_12920(
    name: &str,
    category: &str,
    getter: fn(&QualitySettings) -> i32,
    setter: fn(&mut QualityItem, i32),
    attributes: u32,
    permissions: u32,
) -> QualityProp {
    // IDA 0x12920 (decompiled 0x12920..0x12acc shape, cf. generated_135
    // 0x10a08): classDescriptor, QualityLevel EnumDesc Singleton init, base
    // ctor, enum desc at +40, GetSetImpl at +44, attribute fixups at +28.
    let _ = crate::generated_134::stub_fa00();
    QualityProp {
        name: name.to_owned(),
        category: category.to_owned(),
        getter: Some(getter),
        setter: Some(setter),
        attributes,
        permissions,
        enum_type: "QualityLevel",
    }
}

// 0x12ad4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
pub fn stub_12ad4() {
    // IDA 0x12ad4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x12b00 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isReadOnly(void)const")]
pub fn stub_12b00(prop: &QualityProp) -> bool {
    // IDA 0x12b00 (same delegate-to-GetSetImpl shape): slot-0 virtual.
    prop.is_read_only()
}

// 0x12b10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isWriteOnly(void)const")]
pub fn stub_12b10(prop: &QualityProp) -> bool {
    // IDA 0x12b10: slot-1 virtual.
    prop.is_write_only()
}

// 0x12b20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12b20(prop: &QualityProp, a: &QualitySettings, b: &QualitySettings) -> bool {
    // IDA 0x12b20 (decompiled 0x12b20..0x12b46 shape; disasm getValue
    // slot-8 calls): compares the +44 GetSetImpl `getValue` of both
    // described objects.
    let get = prop.getter.expect("bound getter at IDA 0x12920");
    get(a) == get(b)
}

// 0x12b48 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_12b48(prop: &QualityProp, settings: &QualitySettings) -> IntCallResult {
    // IDA 0x12b48 (decompiled 0x12b48..0x12b6a; disasm getEnumValue slot-68
    // 0x12b56, `Type::getSingleton<int>()` 0x12b5c, `placement_any<int>`
    // 0x12b6a): tags the out slot with the int singleton, then stores the
    // enum value as int.
    let get = prop.getter.expect("bound getter at IDA 0x12920");
    IntCallResult {
        type_name: "int",
        value: get(settings),
    }
}

// 0x12b6c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_12b6c(prop: &QualityProp, item: &mut QualityItem, value: i32) {
    // IDA 0x12b6c (decompiled 0x12b6c..0x12d00 shape; disasm
    // typeinfo-for-int fast path, `Variant::convert<int>` slow path,
    // `setIntValue` slot-72 call): coerces the variant to int, then stores
    // it. The caller passes the coerced int.
    let set = prop.setter.expect("bound setter at IDA 0x12920");
    set(item, value);
}

// 0x12cbc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_12cbc(prop: &QualityProp, src: &QualitySettings, dst: &mut QualityItem) {
    // IDA 0x12cbc (decompiled 0x12cbc..0x12ce0 shape; disasm getValue slot-8,
    // setValue slot-12): reads the source through the getter, writes it
    // through the setter.
    let get = prop.getter.expect("bound getter at IDA 0x12920");
    let set = prop.setter.expect("bound setter at IDA 0x12920");
    set(dst, get(src));
}

// 0x12ce0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::hasStringValue(void)const")]
pub fn stub_12ce0() -> bool {
    // IDA 0x12ce0 (`MOVS R0,#1` shape): enum properties always have a string
    // value.
    true
}

// 0x12ce4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12ce4(prop: &QualityProp, settings: &QualitySettings, out: &mut String) {
    // IDA 0x12ce4 (decompiled 0x12ce4..0x12d06 shape; disasm enum singleton
    // fetch, getValue slot-8, void `convertToString`): like generated_135
    // 0x120f4 this instantiation calls the void (string, value) overload —
    // asserts + assign-or-empty (host: generated_123 stub_cd4c).
    let get = prop.getter.expect("bound getter at IDA 0x12920");
    crate::generated_123::stub_cd4c(get(settings), out);
}

// 0x12d08 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_12d08(
    prop: &QualityProp,
    desc: &QualityEnumDesc,
    item: &mut QualityItem,
    name: &str,
) -> bool {
    // IDA 0x12d08 (decompiled 0x12d08..0x12d46 shape; disasm enum singleton,
    // `Name::lookup`, `EnumDesc::convertToValue`, setValue slot-12, return
    // 1/0). `Name::lookup` interning has no host effect.
    match desc.lookup_value(name) {
        Some(value) => {
            let set = prop.setter.expect("bound setter at IDA 0x12920");
            set(item, value);
            true
        }
        None => false,
    }
}

// 0x12d48 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_12d48(prop: &QualityProp, settings: &QualitySettings, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x12d48 (decompiled 0x12d48..0x12d66 shape; disasm getValue slot-8,
    // `clearValue`, tag `5` at +16, value at +20, return 5).
    let get = prop.getter.expect("bound getter at IDA 0x12920");
    out.value_type = 0; // `clearValue` resets the pair first.
    out.value_type = 5; // int tag at +16.
    out.int_value = get(settings); // value at +20.
    5
}

// 0x12d68 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_12d68(
    prop: &QualityProp,
    desc: &QualityEnumDesc,
    item: &mut QualityItem,
    xml: &XmlReadValue,
) {
    // IDA 0x12d68 (decompiled 0x12d68..0x12f80 shape, cf. generated_135
    // 0x10e50): xsi:nil early-out; int pair -> `setIntValue` (= stub_13110);
    // string pair -> lookup + setValue with the empty-string `validate`
    // fallback; else `ReleaseAssert(false)` (Reflection.h:359).
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            if stub_13110(prop, desc, item, *value) {
                return;
            }
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
        XmlReadValue::Text(text) => {
            if let Some(value) = desc.lookup_value(text) {
                let set = prop.setter.expect("bound setter at IDA 0x12920");
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

// 0x12fa8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12fa8(prop: &QualityProp, desc: &QualityEnumDesc, settings: &QualitySettings) -> i32 {
    // IDA 0x12fa8 (decompiled 0x12fa8..0x12fc2 shape; disasm impl qword at
    // +44, getValue slot-8, `convertToIndex` tail-call).
    let get = prop.getter.expect("bound getter at IDA 0x12920");
    desc.convert_to_index(get(settings))
}

// 0x12fc4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_12fc4(
    prop: &QualityProp,
    desc: &QualityEnumDesc,
    item: &mut QualityItem,
    index: u32,
) -> bool {
    // IDA 0x12fc4 (decompiled 0x12fc4..0x12ff6 shape; disasm enum singleton
    // at +48, count check, table load, setValue slot-12, return 1/0; a `-1`
    // hole is stored as-is).
    if (index as usize) < desc.index_to_value.len() {
        let value = desc.index_to_value[index as usize];
        let set = prop.setter.expect("bound setter at IDA 0x12920");
        set(item, value);
        return true;
    }
    false
}

// 0x12ff8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12ff8(prop: &QualityProp, settings: &QualitySettings) -> i32 {
    // IDA 0x12ff8 (decompiled: the whole body is the +44 GetSetImpl slot-8
    // getValue call): the whole body is the getter.
    let get = prop.getter.expect("bound getter at IDA 0x12920");
    get(settings)
}

// 0x13000 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13000(
    prop: &QualityProp,
    desc: &QualityEnumDesc,
    item: &mut QualityItem,
    value: i32,
) -> bool {
    // IDA 0x13000 (decompiled 0x13000..0x1304a shape; disasm enum singleton
    // at +48, `__find_if` with `EnumDescriptor::equalValue`, setValue
    // slot-12, return 1/0). was: boost::bind + __find_if -> iterator search.
    if desc.items.iter().any(|(v, _)| *v == value) {
        let set = prop.setter.expect("bound setter at IDA 0x12920");
        set(item, value);
        return true;
    }
    false
}

// 0x1304c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1304c(prop: &QualityProp, desc: &QualityEnumDesc, settings: &QualitySettings) -> i32 {
    // IDA 0x1304c (decompiled 0x1304c..0x1306a shape; disasm impl qword at
    // +44, getValue slot-8, `convertToItem`): position or -1.
    let get = prop.getter.expect("bound getter at IDA 0x12920");
    let value = get(settings);
    desc.items
        .iter()
        .position(|(v, _)| *v == value)
        .map(|i| i as i32)
        .unwrap_or(-1)
}

// 0x1306c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_1306c(
    prop: &QualityProp,
    desc: &QualityEnumDesc,
    item: &mut QualityItem,
    name: &str,
) -> bool {
    // IDA 0x1306c (decompiled 0x1306c..0x1309e shape; disasm `convertToValue`,
    // setValue slot-12, return 1/0): the `Name` overload twin.
    match desc.lookup_value(name) {
        Some(value) => {
            let set = prop.setter.expect("bound setter at IDA 0x12920");
            set(item, value);
            true
        }
        None => false,
    }
}

// 0x130a0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToIndex(RBX::CRenderSettings::QualityLevel)const")]
pub fn stub_130a0(desc: &QualityEnumDesc, value: i32) -> i32 {
    // IDA 0x130a0 (decompiled 0x130a0..0x1310e; disasm `value>=0` assert chain
    // 0x130b4..0x130ea at enumconverter.h:350, index-table load
    // 0x130fa..0x1310a, default -1 at 0x13102..0x1310e).
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: ../App/include/reflection/enumconverter.h line: 350"
        );
    }
    desc.convert_to_index(value)
}

// 0x13110 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13110(
    prop: &QualityProp,
    desc: &QualityEnumDesc,
    item: &mut QualityItem,
    value: i32,
) -> bool {
    // IDA 0x13110 (decompiled 0x13110..0x13148 shape; disasm `value>=0`
    // 0x1311a, legacy table bounds 0x1311e..0x1312c, table load 0x1312e,
    // `-1` hole check 0x13132.., setValue slot-12, return 1/0). The `-1`
    // check is what 0x12fc4 lacks.
    if value >= 0 && (value as usize) < desc.index_to_value.len() {
        let mapped = desc.index_to_value[value as usize];
        if mapped != -1 {
            let set = prop.setter.expect("bound setter at IDA 0x12920");
            set(item, mapped);
            return true;
        }
    }
    false
}

// 0x13150 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isReadOnly(void)const")]
pub fn stub_13150(pair: &QualityPair) -> bool {
    // IDA 0x13150 (decompiled `return 0`, 0x13152): the enum member pointer
    // is bound, so never read-only.
    pair.is_read_only()
}

// 0x13154 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isWriteOnly(void)const")]
pub fn stub_13154(pair: &QualityPair) -> bool {
    // IDA 0x13154 (decompiled `return 0`, 0x13156): never write-only.
    pair.is_write_only()
}

// 0x13158 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13158(pair: &QualityPair, settings: &QualitySettings) -> i32 {
    // IDA 0x13158 (decompiled 0x13158..0x13182; disasm null-object split
    // 0x1315a..0x13172, `a2-36` + 96 adjust 0x13160..0x13168,
    // virtual/indirect dispatch 0x13174..0x13180, indirect call): resolves
    // the stored enum getter and calls it. A null getter faults; host
    // panics.
    let get = pair.getter.expect("bound getter at IDA 0x12920");
    get(settings)
}

// 0x13184 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::QualityLevel const&)const")]
pub fn stub_13184(pair: &QualityPair, item: &mut QualityItem, value: i32) {
    // IDA 0x13184 (decompiled 0x13184..0x131a6; disasm `a2-36` adjust
    // 0x1318a..0x1318c, setter fetch 0x13190..0x13198, `>>1`/`&1` dispatch
    // 0x13198..0x1319c, indirect call): resolves the stored enum setter and
    // calls it. A null setter faults; host panics.
    let set = pair.setter.expect("bound setter at IDA 0x12920");
    set(item, value);
}

// 0x131a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::EnumPropDescriptor<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>(char const*,char const*,RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_131a8(
    name: &str,
    category: &str,
    getter: fn(&FrameRateSettings) -> i32,
    setter: fn(&mut FrameRateItem, i32),
    attributes: u32,
    permissions: u32,
) -> FrameRateProp {
    // IDA 0x131a8 (decompiled 0x131a8..0x13380 shape, cf. 0x12920):
    // classDescriptor, FrameRateManagerMode EnumDesc Singleton init, base
    // ctor, enum desc at +40, GetSetImpl at +44, attribute fixups at +28.
    let _ = crate::generated_134::stub_fa00();
    FrameRateProp {
        name: name.to_owned(),
        category: category.to_owned(),
        getter: Some(getter),
        setter: Some(setter),
        attributes,
        permissions,
        enum_type: "FrameRateManagerMode",
    }
}

// 0x1335c — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
pub fn stub_1335c() {
    // IDA 0x1335c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x13388 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isReadOnly(void)const")]
pub fn stub_13388(prop: &FrameRateProp) -> bool {
    // IDA 0x13388 (same delegate-to-GetSetImpl shape): slot-0 virtual.
    prop.is_read_only()
}

// 0x13398 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isWriteOnly(void)const")]
pub fn stub_13398(prop: &FrameRateProp) -> bool {
    // IDA 0x13398: slot-1 virtual.
    prop.is_write_only()
}

// 0x133a8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_133a8(prop: &FrameRateProp, a: &FrameRateSettings, b: &FrameRateSettings) -> bool {
    // IDA 0x133a8 (decompiled 0x133a8..0x133cc shape; disasm getValue
    // slot-8 calls): compares the +44 GetSetImpl `getValue` of both
    // described objects.
    let get = prop.getter.expect("bound getter at IDA 0x131a8");
    get(a) == get(b)
}

// 0x133d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_133d0(prop: &FrameRateProp, settings: &FrameRateSettings) -> IntCallResult {
    // IDA 0x133d0 (decompiled 0x133d0..0x133f2 shape; disasm getEnumValue
    // slot-68, `Type::getSingleton<int>()`, `placement_any<int>`): tags the
    // out slot with the int singleton, then stores the enum value as int.
    let get = prop.getter.expect("bound getter at IDA 0x131a8");
    IntCallResult {
        type_name: "int",
        value: get(settings),
    }
}

// 0x133f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_133f4(prop: &FrameRateProp, item: &mut FrameRateItem, value: i32) {
    // IDA 0x133f4 (decompiled 0x133f4..0x13440 shape; disasm
    // typeinfo-for-int fast path, `Variant::convert<int>` slow path,
    // `setIntValue` slot-72 call): coerces the variant to int, then stores
    // it. The caller passes the coerced int.
    let set = prop.setter.expect("bound setter at IDA 0x131a8");
    set(item, value);
}

// 0x13544 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_13544() -> ! {
    todo!("0x13544 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x13568 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::hasStringValue(void)const")]
pub fn stub_13568() -> ! {
    todo!("0x13568 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::hasStringValue(void)const")
}

// 0x1356c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1356c() -> ! {
    todo!("0x1356c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x13590 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_13590() -> ! {
    todo!("0x13590 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x135d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_135d0() -> ! {
    todo!("0x135d0 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x135f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_135f0() -> ! {
    todo!("0x135f0 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x13830 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13830() -> ! {
    todo!("0x13830 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x1384c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_1384c() -> ! {
    todo!("0x1384c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x13880 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13880() -> ! {
    todo!("0x13880 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x13888 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13888() -> ! {
    todo!("0x13888 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x138d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_138d4() -> ! {
    todo!("0x138d4 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x138f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_138f4() -> ! {
    todo!("0x138f4 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x13928 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToIndex(RBX::CRenderSettings::FrameRateManagerMode)const")]
pub fn stub_13928() -> ! {
    todo!("0x13928 RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToIndex(RBX::CRenderSettings::FrameRateManagerMode)const")
}

// 0x13998 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13998() -> ! {
    todo!("0x13998 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x139d8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isReadOnly(void)const")]
pub fn stub_139d8() -> ! {
    todo!("0x139d8 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isReadOnly(void)const")
}

// 0x139dc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isWriteOnly(void)const")]
pub fn stub_139dc() -> ! {
    todo!("0x139dc RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isWriteOnly(void)const")
}

// 0x139e0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_139e0() -> ! {
    todo!("0x139e0 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x13a0c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_13a0c() -> ! {
    todo!("0x13a0c RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::FrameRateManagerMode const&)const")
}

// 0x13a30 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::EnumPropDescriptor<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>(char const*,char const*,RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_13a30() -> ! {
    todo!("0x13a30 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::EnumPropDescriptor<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>(char const*,char const*,RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x13be4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
pub fn stub_13be4() {
    // IDA 0x13be4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x13c10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isReadOnly(void)const")]
pub fn stub_13c10() -> ! {
    todo!("0x13c10 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isReadOnly(void)const")
}

// 0x13c20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isWriteOnly(void)const")]
pub fn stub_13c20() -> ! {
    todo!("0x13c20 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isWriteOnly(void)const")
}

// 0x13c30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13c30() -> ! {
    todo!("0x13c30 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x13c58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_13c58() -> ! {
    todo!("0x13c58 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x13c7c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_13c7c() -> ! {
    todo!("0x13c7c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x13dcc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_13dcc() -> ! {
    todo!("0x13dcc RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x13df0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::hasStringValue(void)const")]
pub fn stub_13df0() -> ! {
    todo!("0x13df0 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::hasStringValue(void)const")
}

// 0x13df4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13df4() -> ! {
    todo!("0x13df4 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x13e18 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_13e18() -> ! {
    todo!("0x13e18 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x13e58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_13e58() -> ! {
    todo!("0x13e58 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x13e78 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_13e78() -> ! {
    todo!("0x13e78 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x140b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_140b8() -> ! {
    todo!("0x140b8 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x140d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_140d4() -> ! {
    todo!("0x140d4 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x14108 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_14108() -> ! {
    todo!("0x14108 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x14110 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_14110() -> ! {
    todo!("0x14110 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x1415c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1415c() -> ! {
    todo!("0x1415c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x1417c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_1417c() -> ! {
    todo!("0x1417c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x141b0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToIndex(RBX::CRenderSettings::GraphicsMode)const")]
pub fn stub_141b0() -> ! {
    todo!("0x141b0 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToIndex(RBX::CRenderSettings::GraphicsMode)const")
}

// 0x14220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_14220() -> ! {
    todo!("0x14220 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x14260 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isReadOnly(void)const")]
pub fn stub_14260() -> ! {
    todo!("0x14260 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isReadOnly(void)const")
}

// 0x14264 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isWriteOnly(void)const")]
pub fn stub_14264() -> ! {
    todo!("0x14264 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isWriteOnly(void)const")
}

// 0x14268 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_14268() -> ! {
    todo!("0x14268 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x14294 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_14294() -> ! {
    todo!("0x14294 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::GraphicsMode const&)const")
}

// 0x142b8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)")]
pub fn stub_142b8() -> ! {
    todo!("0x142b8 std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)")
}

// 0x14310 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
pub fn stub_14310() -> ! {
    todo!("0x14310 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")
}

// 0x143c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
pub fn stub_143c4() -> ! {
    todo!("0x143c4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")
}

// 0x1441c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
pub fn stub_1441c() -> ! {
    todo!("0x1441c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")
}

// 0x14484 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)")]
pub fn stub_14484() -> ! {
    todo!("0x14484 std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)")
}

// 0x144b8 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_144b8() -> ! {
    todo!("0x144b8 std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)")
}

// 0x144e0 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_144e0() -> ! {
    todo!("0x144e0 std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)")
}

// 0x145c4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)")]
pub fn stub_145c4() -> ! {
    todo!("0x145c4 std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)")
}

// 0x145dc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)")]
pub fn stub_145dc() -> ! {
    todo!("0x145dc RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)")
}

// 0x14618 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_14618() -> ! {
    todo!("0x14618 std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)")
}

// 0x147a8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)")]
pub fn stub_147a8() -> ! {
    todo!("0x147a8 std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)")
}

// 0x14800 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
pub fn stub_14800() -> ! {
    todo!("0x14800 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")
}

// 0x148b4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
pub fn stub_148b4() -> ! {
    todo!("0x148b4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")
}

// 0x1490c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
pub fn stub_1490c() -> ! {
    todo!("0x1490c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")
}

// 0x14974 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)")]
pub fn stub_14974() -> ! {
    todo!("0x14974 std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)")
}

// 0x149a8 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_149a8() -> ! {
    todo!("0x149a8 std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)")
}

// 0x149d0 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_149d0() -> ! {
    todo!("0x149d0 std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)")
}

// 0x14ab4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)")]
pub fn stub_14ab4() -> ! {
    todo!("0x14ab4 std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)")
}

// 0x14acc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)")]
pub fn stub_14acc() -> ! {
    todo!("0x14acc RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)")
}

// 0x14b08 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_14b08() -> ! {
    todo!("0x14b08 std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)")
}

// 0x14c98 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)")]
pub fn stub_14c98() -> ! {
    todo!("0x14c98 std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)")
}

// 0x14ccc — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_14ccc() -> ! {
    todo!("0x14ccc std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)")
}
