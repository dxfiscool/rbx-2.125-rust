//! Auto-generated skeletons for rbx-network — filler global ascending EA-sorted
//! Filter: RakNet|RBX::Network (case-insensitive) -> 4479 funcs, 4479 already stubbed (0 remaining before batch); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x12ce4..0x1523c | existing 17899 -> 17999 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_138::EnumDescModel;
use crate::generated_140::XmlPropValue;


/// QualityLevel `readValue` core — same template as 0x102cc; int cells go
/// through the QualityLevel setIntValue port (0x13110).
pub fn enum_prop_read_value_ql(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    match value {
        XmlPropValue::Nil => {}
        XmlPropValue::Int(i) => {
            stub_13110(desc, i, &set);
        }
        XmlPropValue::Text(s) => {
            let mut v = 0;
            if crate::generated_139::stub_cc34(desc, &s, &mut v) {
                set(v);
            } else if s.is_empty() {
                set(0);
            }
        }
        XmlPropValue::Other => {}
    }
}

// 0x12ce4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12ce4(desc: &EnumDescModel, get: impl Fn() -> i32, out: &mut String) {
    // IDA 0x12ce4: EnumPropDescriptor::getStringValue — same get + convertToString template as 0x10248 (disasm: +0x2C/+0x30 loads, get via +8, convertToString call); delegates to the 0xc76c port.
        crate::generated_139::stub_c76c(desc, get(), out);}

// 0x12d08 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_12d08(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x12d08: EnumPropDescriptor::setStringValue — same lookup + convertToValue + set template as 0x1026c/0x105d0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x12d48 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_12d48(get: impl Fn() -> i32) -> (u32, i32) {
    // IDA 0x12d48: EnumPropDescriptor::writeValue — same get + kind-tag-5 template as 0x102ac; the (kind, value) outputs travel as a pair.
        (5, get())}

// 0x12d68 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_12d68(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    // IDA 0x12d68: EnumPropDescriptor::readValue — same xsi-nil/int/string template as 0x102cc; delegates int cells to the 0x10674 port and string cells to the 0xcc34 port (XmlPropValue carrier is shared from generated_140).
        enum_prop_read_value_ql(desc, value, set);}

// 0x12fa8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12fa8(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x12fa8: EnumPropDescriptor::getIndexValue — same get + convertToIndex template as 0x1050c; delegates to the 0x10604 port.
        stub_130a0(desc, get())}

// 0x12fc4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_12fc4(desc: &EnumDescModel, index: usize, set: impl Fn(i32)) -> bool {
    // IDA 0x12fc4: EnumPropDescriptor::setIndexValue — same legacy bound-check + set template as 0x10528.
        if let Some(&v) = desc.legacy.get(index) {
        set(v);
        true
    } else {
        false
    }}

// 0x12ff8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12ff8(get: impl Fn() -> i32) -> i32 {
    // IDA 0x12ff8: EnumPropDescriptor::getEnumValue — same impl-slot get template as 0x1055c.
        get()}

// 0x13000 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13000(desc: &EnumDescModel, value: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x13000: EnumPropDescriptor::setEnumValue — same find_if + set template as 0x10564; the host searches pairs.
        if desc.pairs.iter().any(|(v, _)| *v == value) {
        set(value);
        true
    } else {
        false
    }}

// 0x1304c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1304c(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x1304c: EnumPropDescriptor::getEnumItem — same get + convertToItem template as 0x105b0; delegates to the 0xc9d8 port.
        crate::generated_139::stub_c9d8(desc, get())}

// 0x1306c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_1306c(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x1306c: EnumPropDescriptor::setStringValue — same lookup + convertToValue + set template as 0x1026c/0x105d0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x130a0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToIndex(RBX::CRenderSettings::QualityLevel)const
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToIndex(RBX::CRenderSettings::QualityLevel)const")]
pub fn stub_130a0(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0x130a0: EnumDesc<QualityLevel>::convertToIndex — same assert + index-vector template as 0x10604 (value>=0, enumconverter.h:350); host pairs-position search on dense tables.
        desc.pairs.iter().position(|(v, _)| *v == value).map(|p| p as i32).unwrap_or(-1)}

// 0x13110 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13110(desc: &EnumDescModel, index: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x13110: EnumPropDescriptor::setIntValue — same table + -1-sentinel + set template as 0x10674.
        if index >= 0 {
        if let Some(&(v, _)) = desc.pairs.get(index as usize) {
            if v != -1 {
                set(v);
                return true;
            }
        }
    }
    false}

// 0x13150 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isReadOnly(void)const")]
pub fn stub_13150() -> bool {
    // IDA 0x13150: GetSetImpl<QualityLevel>::isReadOnly — same 0-return template as 0x106b4.
        false}

// 0x13154 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isWriteOnly(void)const")]
pub fn stub_13154() -> bool {
    // IDA 0x13154: GetSetImpl<QualityLevel>::isWriteOnly — same 0-return template as 0x106b8.
        false}

// 0x13158 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13158(get: impl Fn() -> i32) -> i32 {
    // IDA 0x13158: GetSetImpl<QualityLevel>::getValue — same member-getter dispatch as 0x106bc; the getter travels as a closure.
        get()}

// 0x13184 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::QualityLevel const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::QualityLevel const&)const")]
pub fn stub_13184(set: impl Fn(i32), value: i32) {
    // IDA 0x13184: GetSetImpl<QualityLevel>::setValue — same member-setter dispatch as 0x106e8; the setter travels as a closure.
        set(value);}


/// FrameRateManagerMode `readValue` core — same template as 0x102cc; int cells go
/// through the FrameRateManagerMode setIntValue port (0x13998).
pub fn enum_prop_read_value_frm(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    match value {
        XmlPropValue::Nil => {}
        XmlPropValue::Int(i) => {
            stub_13998(desc, i, &set);
        }
        XmlPropValue::Text(s) => {
            let mut v = 0;
            if crate::generated_139::stub_cc34(desc, &s, &mut v) {
                set(v);
            } else if s.is_empty() {
                set(0);
            }
        }
        XmlPropValue::Other => {}
    }
}

// 0x131a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::EnumPropDescriptor<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>(char const*,char const*,RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::EnumPropDescriptor<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>(char const*,char const*,RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_131a8() {
    // IDA 0x131a8: EnumPropDescriptor<CRenderSettingsItem,FrameRateManagerMode> C2 — same template as 0xfe84 (classDescriptor ensure, EnumDesc singleton call_once + doGet, PropertyDescriptor attach); the descriptor heap lives engine-side — faithful no-op shell.
    }

// 0x1335c — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
pub fn stub_1335c() {
    // IDA 0x1335c: EnumPropDescriptor D0 — same template as 0x10038 (vtable reset, impl-holder delete when non-null, operator delete); drops with Rust ownership.
    }

// 0x13388 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isReadOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isReadOnly(void)const")]
pub fn stub_13388() -> bool {
    // IDA 0x13388: EnumPropDescriptor::isReadOnly — same forward-to-impl template as 0x10064; the FrameRateManagerMode impl answers 0.
        false}

// 0x13398 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isWriteOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isWriteOnly(void)const")]
pub fn stub_13398() -> bool {
    // IDA 0x13398: EnumPropDescriptor::isWriteOnly — same forward-to-impl template as 0x10074; the FrameRateManagerMode impl answers 0.
        false}

// 0x133a8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_133a8(a: i32, b: i32) -> bool {
    // IDA 0x133a8: EnumPropDescriptor::equalValues — same get-vs-get template as 0x10084; the host compares carried values directly.
        a == b}

// 0x133d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_133d0(get: impl Fn() -> i32) -> i32 {
    // IDA 0x133d0: EnumPropDescriptor::getVariant — same get + int-Variant-wrap template as 0x100ac; the host carries the int.
        get()}

// 0x133f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_133f4(value: i32, set: impl Fn(i32)) {
    // IDA 0x133f4: EnumPropDescriptor::setVariant — same any_cast/convert + setter template as 0x100d0; the host carries the int.
        set(value);}

// 0x13544 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_13544(get: impl Fn() -> i32, set: impl Fn(i32)) {
    // IDA 0x13544: EnumPropDescriptor::copyValue — same get-then-set template as 0x10220.
        let v = get();
    set(v);}

// 0x13568 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::hasStringValue(void)const
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::hasStringValue(void)const")]
pub fn stub_13568() -> bool {
    // IDA 0x13568: EnumPropDescriptor::hasStringValue — same 1-return template as 0x10244.
        true}

// 0x1356c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getStringValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1356c(desc: &EnumDescModel, get: impl Fn() -> i32, out: &mut String) {
    // IDA 0x1356c: EnumPropDescriptor::getStringValue — same get + convertToString template as 0x10248 (disasm: +0x2C/+0x30 loads, get via +8, convertToString call); delegates to the 0xc76c port.
        crate::generated_139::stub_c76c(desc, get(), out);}

// 0x13590 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_13590(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x13590: EnumPropDescriptor::setStringValue — same lookup + convertToValue + set template as 0x1026c/0x105d0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x135d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_135d0(get: impl Fn() -> i32) -> (u32, i32) {
    // IDA 0x135d0: EnumPropDescriptor::writeValue — same get + kind-tag-5 template as 0x102ac; the (kind, value) outputs travel as a pair.
        (5, get())}

// 0x135f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_135f0(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    // IDA 0x135f0: EnumPropDescriptor::readValue — same xsi-nil/int/string template as 0x102cc; delegates int cells to the 0x10674 port and string cells to the 0xcc34 port (XmlPropValue carrier is shared from generated_140).
        enum_prop_read_value_frm(desc, value, set);}

// 0x13830 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13830(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x13830: EnumPropDescriptor::getIndexValue — same get + convertToIndex template as 0x1050c; delegates to the 0x10604 port.
        stub_13928(desc, get())}

// 0x1384c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_1384c(desc: &EnumDescModel, index: usize, set: impl Fn(i32)) -> bool {
    // IDA 0x1384c: EnumPropDescriptor::setIndexValue — same legacy bound-check + set template as 0x10528.
        if let Some(&v) = desc.legacy.get(index) {
        set(v);
        true
    } else {
        false
    }}

// 0x13880 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13880(get: impl Fn() -> i32) -> i32 {
    // IDA 0x13880: EnumPropDescriptor::getEnumValue — same impl-slot get template as 0x1055c.
        get()}

// 0x13888 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13888(desc: &EnumDescModel, value: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x13888: EnumPropDescriptor::setEnumValue — same find_if + set template as 0x10564; the host searches pairs.
        if desc.pairs.iter().any(|(v, _)| *v == value) {
        set(value);
        true
    } else {
        false
    }}

// 0x138d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_138d4(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x138d4: EnumPropDescriptor::getEnumItem — same get + convertToItem template as 0x105b0; delegates to the 0xc9d8 port.
        crate::generated_139::stub_c9d8(desc, get())}

// 0x138f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_138f4(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x138f4: EnumPropDescriptor::setStringValue — same lookup + convertToValue + set template as 0x1026c/0x105d0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x13928 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToIndex(RBX::CRenderSettings::FrameRateManagerMode)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToIndex(RBX::CRenderSettings::FrameRateManagerMode)const")]
pub fn stub_13928(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0x13928: EnumDesc<FrameRateManagerMode>::convertToIndex — same assert + index-vector template as 0x10604 (value>=0, enumconverter.h:350); host pairs-position search on dense tables.
        desc.pairs.iter().position(|(v, _)| *v == value).map(|p| p as i32).unwrap_or(-1)}

// 0x13998 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_13998(desc: &EnumDescModel, index: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x13998: EnumPropDescriptor::setIntValue — same table + -1-sentinel + set template as 0x10674.
        if index >= 0 {
        if let Some(&(v, _)) = desc.pairs.get(index as usize) {
            if v != -1 {
                set(v);
                return true;
            }
        }
    }
    false}

// 0x139d8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isReadOnly(void)const")]
pub fn stub_139d8() -> bool {
    // IDA 0x139d8: GetSetImpl<FrameRateManagerMode>::isReadOnly — same 0-return template as 0x106b4.
        false}

// 0x139dc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isWriteOnly(void)const")]
pub fn stub_139dc() -> bool {
    // IDA 0x139dc: GetSetImpl<FrameRateManagerMode>::isWriteOnly — same 0-return template as 0x106b8.
        false}

// 0x139e0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_139e0(get: impl Fn() -> i32) -> i32 {
    // IDA 0x139e0: GetSetImpl<FrameRateManagerMode>::getValue — same member-getter dispatch as 0x106bc; the getter travels as a closure.
        get()}

// 0x13a0c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::FrameRateManagerMode const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_13a0c(set: impl Fn(i32), value: i32) {
    // IDA 0x13a0c: GetSetImpl<FrameRateManagerMode>::setValue — same member-setter dispatch as 0x106e8; the setter travels as a closure.
        set(value);}


/// GraphicsMode `readValue` core — same template as 0x102cc; int cells go
/// through the GraphicsMode setIntValue port (0x14220).
pub fn enum_prop_read_value_graphics(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    match value {
        XmlPropValue::Nil => {}
        XmlPropValue::Int(i) => {
            stub_14220(desc, i, &set);
        }
        XmlPropValue::Text(s) => {
            let mut v = 0;
            if crate::generated_139::stub_cc34(desc, &s, &mut v) {
                set(v);
            } else if s.is_empty() {
                set(0);
            }
        }
        XmlPropValue::Other => {}
    }
}

// 0x13a30 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::EnumPropDescriptor<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>(char const*,char const*,RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::EnumPropDescriptor<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>(char const*,char const*,RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_13a30() {
    // IDA 0x13a30: EnumPropDescriptor<CRenderSettingsItem,GraphicsMode> C2 — same template as 0xfe84 (classDescriptor ensure, EnumDesc singleton call_once + doGet, PropertyDescriptor attach); the descriptor heap lives engine-side — faithful no-op shell.
    }

// 0x13be4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
pub fn stub_13be4() {
    // IDA 0x13be4: EnumPropDescriptor D0 — same template as 0x10038 (vtable reset, impl-holder delete when non-null, operator delete); drops with Rust ownership.
    }

// 0x13c10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isReadOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isReadOnly(void)const")]
pub fn stub_13c10() -> bool {
    // IDA 0x13c10: EnumPropDescriptor::isReadOnly — same forward-to-impl template as 0x10064; the GraphicsMode impl answers 0.
        false}

// 0x13c20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isWriteOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isWriteOnly(void)const")]
pub fn stub_13c20() -> bool {
    // IDA 0x13c20: EnumPropDescriptor::isWriteOnly — same forward-to-impl template as 0x10074; the GraphicsMode impl answers 0.
        false}

// 0x13c30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13c30(a: i32, b: i32) -> bool {
    // IDA 0x13c30: EnumPropDescriptor::equalValues — same get-vs-get template as 0x10084; the host compares carried values directly.
        a == b}

// 0x13c58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_13c58(get: impl Fn() -> i32) -> i32 {
    // IDA 0x13c58: EnumPropDescriptor::getVariant — same get + int-Variant-wrap template as 0x100ac; the host carries the int.
        get()}

// 0x13c7c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_13c7c(value: i32, set: impl Fn(i32)) {
    // IDA 0x13c7c: EnumPropDescriptor::setVariant — same any_cast/convert + setter template as 0x100d0; the host carries the int.
        set(value);}

// 0x13dcc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_13dcc(get: impl Fn() -> i32, set: impl Fn(i32)) {
    // IDA 0x13dcc: EnumPropDescriptor::copyValue — same get-then-set template as 0x10220.
        let v = get();
    set(v);}

// 0x13df0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::hasStringValue(void)const
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::hasStringValue(void)const")]
pub fn stub_13df0() -> bool {
    // IDA 0x13df0: EnumPropDescriptor::hasStringValue — same 1-return template as 0x10244.
        true}

// 0x13df4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getStringValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_13df4(desc: &EnumDescModel, get: impl Fn() -> i32, out: &mut String) {
    // IDA 0x13df4: EnumPropDescriptor::getStringValue — same get + convertToString template as 0x10248 (disasm: +0x2C/+0x30 loads, get via +8, convertToString call); delegates to the 0xc76c port.
        crate::generated_139::stub_c76c(desc, get(), out);}

// 0x13e18 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_13e18(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x13e18: EnumPropDescriptor::setStringValue — same lookup + convertToValue + set template as 0x1026c/0x105d0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x13e58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_13e58(get: impl Fn() -> i32) -> (u32, i32) {
    // IDA 0x13e58: EnumPropDescriptor::writeValue — same get + kind-tag-5 template as 0x102ac; the (kind, value) outputs travel as a pair.
        (5, get())}

// 0x13e78 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_13e78(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    // IDA 0x13e78: EnumPropDescriptor::readValue — same xsi-nil/int/string template as 0x102cc; delegates int cells to the 0x10674 port and string cells to the 0xcc34 port (XmlPropValue carrier is shared from generated_140).
        enum_prop_read_value_graphics(desc, value, set);}

// 0x140b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_140b8(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x140b8: EnumPropDescriptor::getIndexValue — same get + convertToIndex template as 0x1050c; delegates to the 0x10604 port.
        stub_141b0(desc, get())}

// 0x140d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_140d4(desc: &EnumDescModel, index: usize, set: impl Fn(i32)) -> bool {
    // IDA 0x140d4: EnumPropDescriptor::setIndexValue — same legacy bound-check + set template as 0x10528.
        if let Some(&v) = desc.legacy.get(index) {
        set(v);
        true
    } else {
        false
    }}

// 0x14108 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_14108(get: impl Fn() -> i32) -> i32 {
    // IDA 0x14108: EnumPropDescriptor::getEnumValue — same impl-slot get template as 0x1055c.
        get()}

// 0x14110 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_14110(desc: &EnumDescModel, value: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x14110: EnumPropDescriptor::setEnumValue — same find_if + set template as 0x10564; the host searches pairs.
        if desc.pairs.iter().any(|(v, _)| *v == value) {
        set(value);
        true
    } else {
        false
    }}

// 0x1415c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1415c(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x1415c: EnumPropDescriptor::getEnumItem — same get + convertToItem template as 0x105b0; delegates to the 0xc9d8 port.
        crate::generated_139::stub_c9d8(desc, get())}

// 0x1417c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_1417c(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x1417c: EnumPropDescriptor::setStringValue — same lookup + convertToValue + set template as 0x1026c/0x105d0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x141b0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToIndex(RBX::CRenderSettings::GraphicsMode)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToIndex(RBX::CRenderSettings::GraphicsMode)const")]
pub fn stub_141b0(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0x141b0: EnumDesc<GraphicsMode>::convertToIndex — same assert + index-vector template as 0x10604 (value>=0, enumconverter.h:350); host pairs-position search on dense tables.
        desc.pairs.iter().position(|(v, _)| *v == value).map(|p| p as i32).unwrap_or(-1)}

// 0x14220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_14220(desc: &EnumDescModel, index: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x14220: EnumPropDescriptor::setIntValue — same table + -1-sentinel + set template as 0x10674.
        if index >= 0 {
        if let Some(&(v, _)) = desc.pairs.get(index as usize) {
            if v != -1 {
                set(v);
                return true;
            }
        }
    }
    false}

// 0x14260 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isReadOnly(void)const")]
pub fn stub_14260() -> bool {
    // IDA 0x14260: GetSetImpl<GraphicsMode>::isReadOnly — same 0-return template as 0x106b4.
        false}

// 0x14264 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isWriteOnly(void)const")]
pub fn stub_14264() -> bool {
    // IDA 0x14264: GetSetImpl<GraphicsMode>::isWriteOnly — same 0-return template as 0x106b8.
        false}

// 0x14268 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_14268(get: impl Fn() -> i32) -> i32 {
    // IDA 0x14268: GetSetImpl<GraphicsMode>::getValue — same member-getter dispatch as 0x106bc; the getter travels as a closure.
        get()}

// 0x14294 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::GraphicsMode const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_14294(set: impl Fn(i32), value: i32) {
    // IDA 0x14294: GetSetImpl<GraphicsMode>::setValue — same member-setter dispatch as 0x106e8; the setter travels as a closure.
        set(value);}

// 0x142b8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// demangled: std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)")]
pub fn stub_142b8(map: &mut std::collections::HashMap<String, i32>, name: &str) -> i32 {
    // IDA 0x142b8: map<Name const*, ResolutionPreset>::operator[] — lower_bound walk (cf. 0x142b8: 0x142d0..0x142f0); miss inserts via _M_insert_unique (0x14304), returns &mapped (0x1430c). T() default is 0 for the int enum; entry API is the same observable.
        *map.entry(name.to_owned()).or_insert(0)}

// 0x14310 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
pub fn stub_14310(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x14310: _Rb_tree<ResolutionPreset>::_M_insert_unique(hint, value) — hint-aware unique insert into the name→value map (cf. 0x14310, falling into _M_insert at 0x1435e); HashMap has no order hints. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x143c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
pub fn stub_143c4(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) {
    // IDA 0x143c4: _Rb_tree<ResolutionPreset>::_M_insert — fresh 0x18 node, key+value copy (cf. 0x143c4: 0x143f4..0x143fa), rebalance (0x14406), count++ (0x14410); the node heap folds into the HashMap entry.
        map.insert(name.to_owned(), value);}

// 0x1441c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
pub fn stub_1441c(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x1441c: _Rb_tree<ResolutionPreset>::_M_insert_unique(value) — lower_bound + dup check (cf. 0x1441c: 0x14428..0x1447a); existing keys keep their value. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x14484 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_
// demangled: std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)")]
pub fn stub_14484(xs: &mut Vec<i32>, n: usize, value: i32) {
    // IDA 0x14484: vector<ResolutionPreset>::resize — shrink finish (cf. 0x14484: 0x144a2) or _M_fill_insert grow (0x144ac); Vec::resize covers both arms.
        xs.resize(n, value);}

// 0x144b8 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_144b8(xs: &mut Vec<i32>, value: i32) {
    // IDA 0x144b8: vector<ResolutionPreset>::push_back — fast store + finish bump (cf. 0x144b8: 0x144c8..0x144d0), _M_insert_aux when full (0x144da); Vec::push covers both arms.
        xs.push(value);}

// 0x144e0 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_144e0(xs: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x144e0: vector<ResolutionPreset>::_M_insert_aux — same shift-store template as 0xf704 (finish bump, copy_backward, store); Vec::insert covers both arms.
        xs.insert(index, value);}

// 0x145c4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)")]
pub fn stub_145c4(n: usize) -> Vec<i32> {
    // IDA 0x145c4: _Vector_base<ResolutionPreset>::_M_allocate — same template as 0xf7e8 (bad_alloc past the max, else operator new(4n)); with_capacity is the uninit-storage carrier.
        Vec::with_capacity(n)}

// 0x145dc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_
// demangled: RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)")]
pub fn stub_145dc(xs: &mut [i32], first: usize, last: usize, result: usize) -> usize {
    // IDA 0x145dc: __copy_backward<ResolutionPreset> — same word-loop template as 0xf800; copy_within is the overlapping-backward carrier.
        let n = last - first;
    xs.copy_within(first..last, result - n);
    result - n}

// 0x14618 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// demangled: std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_14618(xs: &mut Vec<i32>, index: usize, n: usize, value: i32) {
    // IDA 0x14618: vector<ResolutionPreset>::_M_fill_insert — n-copy fill at pos with spare/realloc paths (cf. 0x14618); splice with repeat covers all arms.
        xs.splice(index..index, std::iter::repeat(value).take(n));}

// 0x147a8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// demangled: std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)")]
pub fn stub_147a8(map: &mut std::collections::HashMap<String, i32>, name: &str) -> i32 {
    // IDA 0x147a8: map<Name const*, QualityLevel>::operator[] — lower_bound walk (cf. 0x142b8: 0x142d0..0x142f0); miss inserts via _M_insert_unique (0x14304), returns &mapped (0x1430c). T() default is 0 for the int enum; entry API is the same observable.
        *map.entry(name.to_owned()).or_insert(0)}

// 0x14800 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
pub fn stub_14800(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x14800: _Rb_tree<QualityLevel>::_M_insert_unique(hint, value) — hint-aware unique insert into the name→value map (cf. 0x14310, falling into _M_insert at 0x1435e); HashMap has no order hints. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x148b4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
pub fn stub_148b4(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) {
    // IDA 0x148b4: _Rb_tree<QualityLevel>::_M_insert — fresh 0x18 node, key+value copy (cf. 0x143c4: 0x143f4..0x143fa), rebalance (0x14406), count++ (0x14410); the node heap folds into the HashMap entry.
        map.insert(name.to_owned(), value);}

// 0x1490c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
pub fn stub_1490c(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x1490c: _Rb_tree<QualityLevel>::_M_insert_unique(value) — lower_bound + dup check (cf. 0x1441c: 0x14428..0x1447a); existing keys keep their value. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x14974 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_
// demangled: std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)")]
pub fn stub_14974(xs: &mut Vec<i32>, n: usize, value: i32) {
    // IDA 0x14974: vector<QualityLevel>::resize — shrink finish (cf. 0x14484: 0x144a2) or _M_fill_insert grow (0x144ac); Vec::resize covers both arms.
        xs.resize(n, value);}

// 0x149a8 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_149a8(xs: &mut Vec<i32>, value: i32) {
    // IDA 0x149a8: vector<QualityLevel>::push_back — fast store + finish bump (cf. 0x144b8: 0x144c8..0x144d0), _M_insert_aux when full (0x144da); Vec::push covers both arms.
        xs.push(value);}

// 0x149d0 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_149d0(xs: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x149d0: vector<QualityLevel>::_M_insert_aux — same shift-store template as 0xf704 (finish bump, copy_backward, store); Vec::insert covers both arms.
        xs.insert(index, value);}

// 0x14ab4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)")]
pub fn stub_14ab4(n: usize) -> Vec<i32> {
    // IDA 0x14ab4: _Vector_base<QualityLevel>::_M_allocate — same template as 0xf7e8 (bad_alloc past the max, else operator new(4n)); with_capacity is the uninit-storage carrier.
        Vec::with_capacity(n)}

// 0x14acc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_
// demangled: RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)")]
pub fn stub_14acc(xs: &mut [i32], first: usize, last: usize, result: usize) -> usize {
    // IDA 0x14acc: __copy_backward<QualityLevel> — same word-loop template as 0xf800; copy_within is the overlapping-backward carrier.
        let n = last - first;
    xs.copy_within(first..last, result - n);
    result - n}

// 0x14b08 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// demangled: std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_14b08(xs: &mut Vec<i32>, index: usize, n: usize, value: i32) {
    // IDA 0x14b08: vector<QualityLevel>::_M_fill_insert — n-copy fill at pos with spare/realloc paths (cf. 0x14618); splice with repeat covers all arms.
        xs.splice(index..index, std::iter::repeat(value).take(n));}

// 0x14c98 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_
// demangled: std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)")]
pub fn stub_14c98(xs: &mut Vec<i32>, n: usize, value: i32) {
    // IDA 0x14c98: vector<ShadowMode>::resize — shrink finish (cf. 0x14484: 0x144a2) or _M_fill_insert grow (0x144ac); Vec::resize covers both arms.
        xs.resize(n, value);}

// 0x14ccc — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_14ccc(xs: &mut Vec<i32>, value: i32) {
    // IDA 0x14ccc: vector<ShadowMode>::push_back — fast store + finish bump (cf. 0x144b8: 0x144c8..0x144d0), _M_insert_aux when full (0x144da); Vec::push covers both arms.
        xs.push(value);}

// 0x14cf4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// demangled: std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_14cf4(map: &mut std::collections::HashMap<String, i32>, name: &str) -> i32 {
    // IDA 0x14cf4: map<Name const*, ShadowMode>::operator[] — lower_bound walk (cf. 0x142b8: 0x142d0..0x142f0); miss inserts via _M_insert_unique (0x14304), returns &mapped (0x1430c). T() default is 0 for the int enum; entry API is the same observable.
        *map.entry(name.to_owned()).or_insert(0)}

// 0x14d4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
pub fn stub_14d4c(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x14d4c: _Rb_tree<ShadowMode>::_M_insert_unique(hint, value) — hint-aware unique insert into the name→value map (cf. 0x14310, falling into _M_insert at 0x1435e); HashMap has no order hints. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x14e00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
pub fn stub_14e00(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) {
    // IDA 0x14e00: _Rb_tree<ShadowMode>::_M_insert — fresh 0x18 node, key+value copy (cf. 0x143c4: 0x143f4..0x143fa), rebalance (0x14406), count++ (0x14410); the node heap folds into the HashMap entry.
        map.insert(name.to_owned(), value);}

// 0x14e58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
pub fn stub_14e58(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x14e58: _Rb_tree<ShadowMode>::_M_insert_unique(value) — lower_bound + dup check (cf. 0x1441c: 0x14428..0x1447a); existing keys keep their value. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x14ec0 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,RBX::CRenderSettings::ShadowMode const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_14ec0(xs: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x14ec0: vector<ShadowMode>::_M_insert_aux — same shift-store template as 0xf704 (finish bump, copy_backward, store); Vec::insert covers both arms.
        xs.insert(index, value);}

// 0x14fa4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings10ShadowModeESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_allocate(unsigned long)
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_allocate(unsigned long)")]
pub fn stub_14fa4(n: usize) -> Vec<i32> {
    // IDA 0x14fa4: _Vector_base<ShadowMode>::_M_allocate — same template as 0xf7e8 (bad_alloc past the max, else operator new(4n)); with_capacity is the uninit-storage carrier.
        Vec::with_capacity(n)}

// 0x14fbc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings10ShadowModeES6_EET0_T_S8_S7_
// demangled: RBX::CRenderSettings::ShadowMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *>(RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::ShadowMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *>(RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *)")]
pub fn stub_14fbc(xs: &mut [i32], first: usize, last: usize, result: usize) -> usize {
    // IDA 0x14fbc: __copy_backward<ShadowMode> — same word-loop template as 0xf800; copy_within is the overlapping-backward carrier.
        let n = last - first;
    xs.copy_within(first..last, result - n);
    result - n}

// 0x14ff8 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// demangled: std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,unsigned long,RBX::CRenderSettings::ShadowMode const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,unsigned long,RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_14ff8(xs: &mut Vec<i32>, index: usize, n: usize, value: i32) {
    // IDA 0x14ff8: vector<ShadowMode>::_M_fill_insert — n-copy fill at pos with spare/realloc paths (cf. 0x14618); splice with repeat covers all arms.
        xs.splice(index..index, std::iter::repeat(value).take(n));}

// 0x15188 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE6resizeEmS2_
// demangled: std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::resize(unsigned long,RBX::CRenderSettings::AntialiasingMode)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::resize(unsigned long,RBX::CRenderSettings::AntialiasingMode)")]
pub fn stub_15188(xs: &mut Vec<i32>, n: usize, value: i32) {
    // IDA 0x15188: vector<AntialiasingMode>::resize — shrink finish (cf. 0x14484: 0x144a2) or _M_fill_insert grow (0x144ac); Vec::resize covers both arms.
        xs.resize(n, value);}

// 0x151bc — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::push_back(RBX::CRenderSettings::AntialiasingMode const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::push_back(RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_151bc(xs: &mut Vec<i32>, value: i32) {
    // IDA 0x151bc: vector<AntialiasingMode>::push_back — fast store + finish bump (cf. 0x144b8: 0x144c8..0x144d0), _M_insert_aux when full (0x144da); Vec::push covers both arms.
        xs.push(value);}

// 0x151e4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16AntialiasingModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// demangled: std::map<RBX::Name const*,RBX::CRenderSettings::AntialiasingMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::operator[](RBX::Name const* const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AntialiasingMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_151e4(map: &mut std::collections::HashMap<String, i32>, name: &str) -> i32 {
    // IDA 0x151e4: map<Name const*, AntialiasingMode>::operator[] — lower_bound walk (cf. 0x142b8: 0x142d0..0x142f0); miss inserts via _M_insert_unique (0x14304), returns &mapped (0x1430c). T() default is 0 for the int enum; entry API is the same observable.
        *map.entry(name.to_owned()).or_insert(0)}

// 0x1523c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
pub fn stub_1523c(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x1523c: _Rb_tree<AntialiasingMode>::_M_insert_unique(hint, value) — hint-aware unique insert into the name→value map (cf. 0x14310, falling into _M_insert at 0x1435e); HashMap has no order hints. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}
