//! Auto-generated skeletons for rbx-network — filler global ascending EA-sorted
//! Filter: RakNet|RBX::Network (case-insensitive) -> 4479 funcs, 4479 already stubbed (0 remaining before batch); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x109e4..0x12ce0 | existing 16529 -> 16629 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_138::EnumDescModel;
use crate::generated_140::XmlPropValue;

/// Shared `EnumPropDescriptor::readValue` core (same template as 0x102cc):
/// xsi-nil bails, int cells go through setIntValue, string cells through
/// Name lookup + convertToValue + set (empty text falls back to set(0));
/// unconvertible text hits only the diagnostics-gated ReleaseAssert.
pub fn enum_prop_read_value(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    use crate::generated_140::XmlPropValue;
    match value {
        XmlPropValue::Nil => {}
        XmlPropValue::Int(i) => {
            stub_111f8(desc, i, &set);
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

// 0x109e4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_109e4(set: impl Fn(u32), value: u32) {
    // IDA 0x109e4: GetSetImpl<int>::setValue — same member-setter dispatch as 0xfce8 (disasm: null-check + -0x24 adjust, decode, invoke); the setter travels as a closure.
        set(value);}

// 0x10a08 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::EnumPropDescriptor<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>(char const*,char const*,RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_10a08() {
    // IDA 0x10a08: EnumPropDescriptor<CRenderSettingsItem,AntialiasingMode> C2 — same template as 0xfe84 (classDescriptor ensure, EnumDesc singleton call_once + doGet, PropertyDescriptor attach); the descriptor heap lives engine-side — faithful no-op shell.
    }

// 0x10bbc — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
pub fn stub_10bbc() {
    // IDA 0x10bbc: EnumPropDescriptor D0 — same template as 0x10038 (vtable reset, impl-holder delete when non-null, operator delete); drops with Rust ownership.
    }

// 0x10be8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isReadOnly(void)const")]
pub fn stub_10be8() -> bool {
    // IDA 0x10be8: EnumPropDescriptor::isReadOnly — same forward-to-impl template as 0x10064; the AntialiasingMode impl answers 0.
        false}

// 0x10bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isWriteOnly(void)const")]
pub fn stub_10bf8() -> bool {
    // IDA 0x10bf8: EnumPropDescriptor::isWriteOnly — same forward-to-impl template as 0x10074; the AntialiasingMode impl answers 0.
        false}

// 0x10c08 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_10c08(a: i32, b: i32) -> bool {
    // IDA 0x10c08: EnumPropDescriptor::equalValues — same get-vs-get template as 0x10084; the host compares carried values directly.
        a == b}

// 0x10c30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_10c30(get: impl Fn() -> i32) -> i32 {
    // IDA 0x10c30: EnumPropDescriptor::getVariant — same get + int-Variant-wrap template as 0x100ac; the host carries the int.
        get()}

// 0x10c54 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_10c54(value: i32, set: impl Fn(i32)) {
    // IDA 0x10c54: EnumPropDescriptor::setVariant — same any_cast/convert + setter template as 0x100d0; the host carries the int.
        set(value);}

// 0x10da4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_10da4(get: impl Fn() -> i32, set: impl Fn(i32)) {
    // IDA 0x10da4: EnumPropDescriptor::copyValue — same get-then-set template as 0x10220.
        let v = get();
    set(v);}

// 0x10dc8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::hasStringValue(void)const")]
pub fn stub_10dc8() -> bool {
    // IDA 0x10dc8: EnumPropDescriptor::hasStringValue — same 1-return template as 0x10244.
        true}

// 0x10dcc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_10dcc(desc: &EnumDescModel, get: impl Fn() -> i32, out: &mut String) {
    // IDA 0x10dcc: EnumPropDescriptor::getStringValue — same get + convertToString template as 0x10248 (disasm: +0x2C/+0x30 loads, get via +8, convertToString call); delegates to the 0xc76c port.
        crate::generated_139::stub_c76c(desc, get(), out);}

// 0x10df0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_10df0(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x10df0: EnumPropDescriptor::setStringValue — same lookup + convertToValue + set template as 0x1026c/0x105d0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x10e30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_10e30(get: impl Fn() -> i32) -> (u32, i32) {
    // IDA 0x10e30: EnumPropDescriptor::writeValue — same get + kind-tag-5 template as 0x102ac; the (kind, value) outputs travel as a pair.
        (5, get())}

// 0x10e50 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_10e50(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    // IDA 0x10e50: EnumPropDescriptor::readValue — same xsi-nil/int/string template as 0x102cc; delegates int cells to the 0x10674 port and string cells to the 0xcc34 port (XmlPropValue carrier is shared from generated_140).
        enum_prop_read_value_shadow(desc, value, set);}

// 0x11090 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11090(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x11090: EnumPropDescriptor::getIndexValue — same get + convertToIndex template as 0x1050c; delegates to the 0x10604 port.
        stub_11188(desc, get())}

// 0x110ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_110ac(desc: &EnumDescModel, index: usize, set: impl Fn(i32)) -> bool {
    // IDA 0x110ac: EnumPropDescriptor::setIndexValue — same legacy bound-check + set template as 0x10528.
        if let Some(&v) = desc.legacy.get(index) {
        set(v);
        true
    } else {
        false
    }}

// 0x110e0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_110e0(get: impl Fn() -> i32) -> i32 {
    // IDA 0x110e0: EnumPropDescriptor::getEnumValue — same impl-slot get template as 0x1055c.
        get()}

// 0x110e8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_110e8(desc: &EnumDescModel, value: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x110e8: EnumPropDescriptor::setEnumValue — same find_if + set template as 0x10564; the host searches pairs.
        if desc.pairs.iter().any(|(v, _)| *v == value) {
        set(value);
        true
    } else {
        false
    }}

// 0x11134 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11134(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x11134: EnumPropDescriptor::getEnumItem — same get + convertToItem template as 0x105b0; delegates to the 0xc9d8 port.
        crate::generated_139::stub_c9d8(desc, get())}

// 0x11154 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_11154(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x11154: EnumPropDescriptor::setStringValue — same lookup + convertToValue + set template as 0x1026c/0x105d0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x11188 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToIndex(RBX::CRenderSettings::AntialiasingMode)const")]
pub fn stub_11188(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0x11188: EnumDesc<AntialiasingMode>::convertToIndex — same assert + index-vector template as 0x10604 (value>=0, enumconverter.h:350); host pairs-position search on dense tables.
        desc.pairs.iter().position(|(v, _)| *v == value).map(|p| p as i32).unwrap_or(-1)}

// 0x111f8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_111f8(desc: &EnumDescModel, index: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x111f8: EnumPropDescriptor::setIntValue — same table + -1-sentinel + set template as 0x10674.
        if index >= 0 {
        if let Some(&(v, _)) = desc.pairs.get(index as usize) {
            if v != -1 {
                set(v);
                return true;
            }
        }
    }
    false}

// 0x11238 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isReadOnly(void)const")]
pub fn stub_11238() -> bool {
    // IDA 0x11238: GetSetImpl<AntialiasingMode>::isReadOnly — same 0-return template as 0x106b4.
        false}

// 0x1123c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isWriteOnly(void)const")]
pub fn stub_1123c() -> bool {
    // IDA 0x1123c: GetSetImpl<AntialiasingMode>::isWriteOnly — same 0-return template as 0x106b8.
        false}

// 0x11240 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11240(get: impl Fn() -> i32) -> i32 {
    // IDA 0x11240: GetSetImpl<AntialiasingMode>::getValue — same member-getter dispatch as 0x106bc; the getter travels as a closure.
        get()}

// 0x1126c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AntialiasingMode const&)const")]
pub fn stub_1126c(set: impl Fn(i32), value: i32) {
    // IDA 0x1126c: GetSetImpl<AntialiasingMode>::setValue — same member-setter dispatch as 0x106e8; the setter travels as a closure.
        set(value);}

/// ShadowMode `readValue` core — same template as 0x102cc/0x10e50; int cells
/// go through the ShadowMode setIntValue port (0x11a80).
pub fn enum_prop_read_value_shadow(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    use crate::generated_140::XmlPropValue;
    match value {
        XmlPropValue::Nil => {}
        XmlPropValue::Int(i) => {
            stub_11a80(desc, i, &set);
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

// 0x11290 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::EnumPropDescriptor<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>(char const*,char const*,RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_11290() {
    // IDA 0x11290: EnumPropDescriptor<CRenderSettingsItem,ShadowMode> C2 — same template as 0xfe84 (classDescriptor ensure, EnumDesc singleton call_once + doGet, PropertyDescriptor attach); the descriptor heap lives engine-side — faithful no-op shell.
    }

// 0x11444 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
pub fn stub_11444() {
    // IDA 0x11444: EnumPropDescriptor D0 — same template as 0x10038 (vtable reset, impl-holder delete when non-null, operator delete); drops with Rust ownership.
    }

// 0x11470 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isReadOnly(void)const")]
pub fn stub_11470() -> bool {
    // IDA 0x11470: EnumPropDescriptor::isReadOnly — same forward-to-impl template as 0x10064; the ShadowMode impl answers 0.
        false}

// 0x11480 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isWriteOnly(void)const")]
pub fn stub_11480() -> bool {
    // IDA 0x11480: EnumPropDescriptor::isWriteOnly — same forward-to-impl template as 0x10074; the ShadowMode impl answers 0.
        false}

// 0x11490 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11490(a: i32, b: i32) -> bool {
    // IDA 0x11490: EnumPropDescriptor::equalValues — same get-vs-get template as 0x10084; the host compares carried values directly.
        a == b}

// 0x114b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_114b8(get: impl Fn() -> i32) -> i32 {
    // IDA 0x114b8: EnumPropDescriptor::getVariant — same get + int-Variant-wrap template as 0x100ac; the host carries the int.
        get()}

// 0x114dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_114dc(value: i32, set: impl Fn(i32)) {
    // IDA 0x114dc: EnumPropDescriptor::setVariant — same any_cast/convert + setter template as 0x100d0; the host carries the int.
        set(value);}

// 0x1162c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_1162c(get: impl Fn() -> i32, set: impl Fn(i32)) {
    // IDA 0x1162c: EnumPropDescriptor::copyValue — same get-then-set template as 0x10220.
        let v = get();
    set(v);}

// 0x11650 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::hasStringValue(void)const")]
pub fn stub_11650() -> bool {
    // IDA 0x11650: EnumPropDescriptor::hasStringValue — same 1-return template as 0x10244.
        true}

// 0x11654 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11654(desc: &EnumDescModel, get: impl Fn() -> i32, out: &mut String) {
    // IDA 0x11654: EnumPropDescriptor::getStringValue — same get + convertToString template as 0x10248 (disasm: +0x2C/+0x30 loads, get via +8, convertToString call); delegates to the 0xc76c port.
        crate::generated_139::stub_c76c(desc, get(), out);}

// 0x11678 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_11678(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x11678: EnumPropDescriptor::setStringValue — same lookup + convertToValue + set template as 0x1026c/0x105d0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x116b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_116b8(get: impl Fn() -> i32) -> (u32, i32) {
    // IDA 0x116b8: EnumPropDescriptor::writeValue — same get + kind-tag-5 template as 0x102ac; the (kind, value) outputs travel as a pair.
        (5, get())}

// 0x116d8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_116d8(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    // IDA 0x116d8: EnumPropDescriptor::readValue — same xsi-nil/int/string template as 0x102cc; delegates int cells to the 0x10674 port and string cells to the 0xcc34 port (XmlPropValue carrier is shared from generated_140).
        enum_prop_read_value_shadow(desc, value, set);}

// 0x11918 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11918(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x11918: EnumPropDescriptor::getIndexValue — same get + convertToIndex template as 0x1050c; delegates to the 0x10604 port.
        stub_11a10(desc, get())}

// 0x11934 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_11934(desc: &EnumDescModel, index: usize, set: impl Fn(i32)) -> bool {
    // IDA 0x11934: EnumPropDescriptor::setIndexValue — same legacy bound-check + set template as 0x10528.
        if let Some(&v) = desc.legacy.get(index) {
        set(v);
        true
    } else {
        false
    }}

// 0x11968 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11968(get: impl Fn() -> i32) -> i32 {
    // IDA 0x11968: EnumPropDescriptor::getEnumValue — same impl-slot get template as 0x1055c.
        get()}

// 0x11970 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_11970(desc: &EnumDescModel, value: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x11970: EnumPropDescriptor::setEnumValue — same find_if + set template as 0x10564; the host searches pairs.
        if desc.pairs.iter().any(|(v, _)| *v == value) {
        set(value);
        true
    } else {
        false
    }}

// 0x119bc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_119bc(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x119bc: EnumPropDescriptor::getEnumItem — same get + convertToItem template as 0x105b0; delegates to the 0xc9d8 port.
        crate::generated_139::stub_c9d8(desc, get())}

// 0x119dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_119dc(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x119dc: EnumPropDescriptor::setStringValue — same lookup + convertToValue + set template as 0x1026c/0x105d0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x11a10 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToIndex(RBX::CRenderSettings::ShadowMode)const")]
pub fn stub_11a10(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0x11a10: EnumDesc<ShadowMode>::convertToIndex — same assert + index-vector template as 0x10604 (value>=0, enumconverter.h:350); host pairs-position search on dense tables.
        desc.pairs.iter().position(|(v, _)| *v == value).map(|p| p as i32).unwrap_or(-1)}

// 0x11a80 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_11a80(desc: &EnumDescModel, index: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x11a80: EnumPropDescriptor::setIntValue — same table + -1-sentinel + set template as 0x10674.
        if index >= 0 {
        if let Some(&(v, _)) = desc.pairs.get(index as usize) {
            if v != -1 {
                set(v);
                return true;
            }
        }
    }
    false}

// 0x11ac0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isReadOnly(void)const")]
pub fn stub_11ac0() -> bool {
    // IDA 0x11ac0: GetSetImpl<ShadowMode>::isReadOnly — same 0-return template as 0x106b4.
        false}

// 0x11ac4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isWriteOnly(void)const")]
pub fn stub_11ac4() -> bool {
    // IDA 0x11ac4: GetSetImpl<ShadowMode>::isWriteOnly — same 0-return template as 0x106b8.
        false}

// 0x11ac8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11ac8(get: impl Fn() -> i32) -> i32 {
    // IDA 0x11ac8: GetSetImpl<ShadowMode>::getValue — same member-getter dispatch as 0x106bc; the getter travels as a closure.
        get()}

// 0x11af4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ShadowMode const&)const")]
pub fn stub_11af4(set: impl Fn(i32), value: i32) {
    // IDA 0x11af4: GetSetImpl<ShadowMode>::setValue — same member-setter dispatch as 0x106e8; the setter travels as a closure.
        set(value);}


/// AASamples `readValue` core — same template as 0x102cc; int cells go
/// through the AASamples setIntValue port (0x12520).
pub fn enum_prop_read_value_aas(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    match value {
        XmlPropValue::Nil => {}
        XmlPropValue::Int(i) => {
            stub_12520(desc, i, &set);
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

// 0x11b18 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,std::string  CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_11b18() {
    // IDA 0x11b18: BoundProp<string> C2 — classDescriptor ensure (0x11b3e), TypedPropertyDescriptor<string> attach (0x11ba0..0x11baa), vtable install (0x11bbe), 0x14 BoundPropGetSet holder with member offset (0x11bcc..0x11c02), attribute bits from impl queries (0x11c12..0x11c38); the descriptor heap lives engine-side — faithful no-op shell.
}

// 0x11ca8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
pub fn stub_11ca8() -> bool {
    // IDA 0x11ca8: BoundPropGetSet<string>::isReadOnly — returns 0 (0x11caa).
    false}

// 0x11cac — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
pub fn stub_11cac() -> bool {
    // IDA 0x11cac: BoundPropGetSet<string>::isWriteOnly — returns 0 (0x11cae).
    false}

// 0x11cb0 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(std::string *, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11cb0(get: impl Fn() -> String) -> String {
    // IDA 0x11cb0: BoundPropGetSet<string>::getValue — Described-36 adjust when obj != 0 (0x11cb6..0x11cb8), member string copy-out (0x11cc6); the member address travels as a getter closure, the std::string out-param as the return.
    get()}

// 0x11cc8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_11cc8(current: &mut String, value: String) -> bool {
    // IDA 0x11cc8: BoundPropGetSet<string>::setValue — member adjust (0x11cd6..0x11ce0), compare (0x11ce6); same returns 0 (0x11d1a), different assigns (0x11cf0) + optional setter-notify (0x11cf4..0x11d12) + raisePropertyChanged (0x11d2a). The host assigns and reports changed; notification folds into emit_prop_changed at the call site.
    if *current != value {
        *current = value;
        true
    } else {
        false
    }}

// 0x11d30 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::EnumPropDescriptor<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>(char const*,char const*,RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_11d30() {
    // IDA 0x11d30: EnumPropDescriptor<CRenderSettingsItem,AASamples> C2 — same template as 0xfe84 (classDescriptor ensure, EnumDesc singleton call_once + doGet, PropertyDescriptor attach); the descriptor heap lives engine-side — faithful no-op shell.
    }

// 0x11ee4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
pub fn stub_11ee4() {
    // IDA 0x11ee4: EnumPropDescriptor D0 — same template as 0x10038 (vtable reset, impl-holder delete when non-null, operator delete); drops with Rust ownership.
    }

// 0x11f10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isReadOnly(void)const")]
pub fn stub_11f10() -> bool {
    // IDA 0x11f10: EnumPropDescriptor::isReadOnly — same forward-to-impl template as 0x10064; the AASamples impl answers 0.
        false}

// 0x11f20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isWriteOnly(void)const")]
pub fn stub_11f20() -> bool {
    // IDA 0x11f20: EnumPropDescriptor::isWriteOnly — same forward-to-impl template as 0x10074; the AASamples impl answers 0.
        false}

// 0x11f30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_11f30(a: i32, b: i32) -> bool {
    // IDA 0x11f30: EnumPropDescriptor::equalValues — same get-vs-get template as 0x10084; the host compares carried values directly.
        a == b}

// 0x11f58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_11f58(get: impl Fn() -> i32) -> i32 {
    // IDA 0x11f58: EnumPropDescriptor::getVariant — same get + int-Variant-wrap template as 0x100ac; the host carries the int.
        get()}

// 0x11f7c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_11f7c(value: i32, set: impl Fn(i32)) {
    // IDA 0x11f7c: EnumPropDescriptor::setVariant — same any_cast/convert + setter template as 0x100d0; the host carries the int.
        set(value);}

// 0x120cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_120cc(get: impl Fn() -> i32, set: impl Fn(i32)) {
    // IDA 0x120cc: EnumPropDescriptor::copyValue — same get-then-set template as 0x10220.
        let v = get();
    set(v);}

// 0x120f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::hasStringValue(void)const")]
pub fn stub_120f0() -> bool {
    // IDA 0x120f0: EnumPropDescriptor::hasStringValue — same 1-return template as 0x10244.
        true}

// 0x120f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_120f4(desc: &EnumDescModel, get: impl Fn() -> i32, out: &mut String) {
    // IDA 0x120f4: EnumPropDescriptor::getStringValue — same get + convertToString template as 0x10248 (disasm: +0x2C/+0x30 loads, get via +8, convertToString call); delegates to the 0xc76c port.
        crate::generated_139::stub_c76c(desc, get(), out);}

// 0x12118 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_12118(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x12118: EnumPropDescriptor::setStringValue — same lookup + convertToValue + set template as 0x1026c/0x105d0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x12158 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_12158(get: impl Fn() -> i32) -> (u32, i32) {
    // IDA 0x12158: EnumPropDescriptor::writeValue — same get + kind-tag-5 template as 0x102ac; the (kind, value) outputs travel as a pair.
        (5, get())}

// 0x12178 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_12178(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    // IDA 0x12178: EnumPropDescriptor::readValue — same xsi-nil/int/string template as 0x102cc; delegates int cells to the 0x10674 port and string cells to the 0xcc34 port (XmlPropValue carrier is shared from generated_140).
        enum_prop_read_value_aas(desc, value, set);}

// 0x123b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_123b8(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x123b8: EnumPropDescriptor::getIndexValue — same get + convertToIndex template as 0x1050c; delegates to the 0x10604 port.
        stub_124b0(desc, get())}

// 0x123d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_123d4(desc: &EnumDescModel, index: usize, set: impl Fn(i32)) -> bool {
    // IDA 0x123d4: EnumPropDescriptor::setIndexValue — same legacy bound-check + set template as 0x10528.
        if let Some(&v) = desc.legacy.get(index) {
        set(v);
        true
    } else {
        false
    }}

// 0x12408 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12408(get: impl Fn() -> i32) -> i32 {
    // IDA 0x12408: EnumPropDescriptor::getEnumValue — same impl-slot get template as 0x1055c.
        get()}

// 0x12410 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_12410(desc: &EnumDescModel, value: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x12410: EnumPropDescriptor::setEnumValue — same find_if + set template as 0x10564; the host searches pairs.
        if desc.pairs.iter().any(|(v, _)| *v == value) {
        set(value);
        true
    } else {
        false
    }}

// 0x1245c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1245c(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x1245c: EnumPropDescriptor::getEnumItem — same get + convertToItem template as 0x105b0; delegates to the 0xc9d8 port.
        crate::generated_139::stub_c9d8(desc, get())}

// 0x1247c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_1247c(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x1247c: EnumPropDescriptor::setStringValue — same lookup + convertToValue + set template as 0x1026c/0x105d0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x124b0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToIndex(RBX::CRenderSettings::AASamples)const")]
pub fn stub_124b0(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0x124b0: EnumDesc<AASamples>::convertToIndex — same assert + index-vector template as 0x10604 (value>=0, enumconverter.h:350); host pairs-position search on dense tables.
        desc.pairs.iter().position(|(v, _)| *v == value).map(|p| p as i32).unwrap_or(-1)}

// 0x12520 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_12520(desc: &EnumDescModel, index: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x12520: EnumPropDescriptor::setIntValue — same table + -1-sentinel + set template as 0x10674.
        if index >= 0 {
        if let Some(&(v, _)) = desc.pairs.get(index as usize) {
            if v != -1 {
                set(v);
                return true;
            }
        }
    }
    false}

// 0x12560 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isReadOnly(void)const")]
pub fn stub_12560() -> bool {
    // IDA 0x12560: GetSetImpl<AASamples>::isReadOnly — same 0-return template as 0x106b4.
        false}

// 0x12564 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isWriteOnly(void)const")]
pub fn stub_12564() -> bool {
    // IDA 0x12564: GetSetImpl<AASamples>::isWriteOnly — same 0-return template as 0x106b8.
        false}

// 0x12568 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12568(get: impl Fn() -> i32) -> i32 {
    // IDA 0x12568: GetSetImpl<AASamples>::getValue — same member-getter dispatch as 0x106bc; the getter travels as a closure.
        get()}

// 0x12594 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_12594(set: impl Fn(i32), value: i32) {
    // IDA 0x12594: GetSetImpl<AASamples>::setValue — same member-setter dispatch as 0x106e8; the setter travels as a closure.
        set(value);}

// 0x125b8 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,bool CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_125b8() {
    // IDA 0x125b8: BoundProp<bool> C2 — same template as the string C2 at 0x11b18 (classDescriptor ensure 0x125d8+, TypedPropertyDescriptor attach, vtable, GetSet holder, attribute bits); the descriptor heap lives engine-side — faithful no-op shell.
    }

// 0x12748 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
pub fn stub_12748() -> bool {
    // IDA 0x12748: BoundPropGetSet<bool>::isReadOnly — returns 0 (0x1274a).
        false}

// 0x1274c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
pub fn stub_1274c() -> bool {
    // IDA 0x1274c: BoundPropGetSet<bool>::isWriteOnly — returns 0 (0x1274e).
        false}

// 0x12750 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12750(get: impl Fn() -> bool) -> bool {
    // IDA 0x12750: BoundPropGetSet<bool>::getValue — byte member load *(u8*)(*(a1+8) + a2 - 36) (0x12758); the member address travels as a getter closure.
        get()}

// 0x1275c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_1275c(current: &mut bool, value: bool) -> bool {
    // IDA 0x1275c: BoundPropGetSet<bool>::setValue — offset compare (0x1276a..0x12774), byte store (0x12778), optional setter-notify (0x1277a..0x12798), raisePropertyChanged; the host assigns and reports changed, notification folds into emit_prop_changed at the call site.
        if *current != value {
        *current = value;
        true
    } else {
        false
    }}

// 0x127ac — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_127ac() {
    // IDA 0x127ac: PropDescriptor<CRenderSettingsItem,bool> C2 — same template as 0x1070c (classDescriptor ensure 0x127be+, GetSetImpl holder, TypedProperty attach, vtable); descriptor heap engine-side — faithful no-op shell.
    }

// 0x128c0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
pub fn stub_128c0() -> bool {
    // IDA 0x128c0: GetSetImpl<bool>::isReadOnly — same 0-return shape as 0x1084c.
        false}

// 0x128c4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_128c4() -> bool {
    // IDA 0x128c4: GetSetImpl<bool>::isWriteOnly — same 0-return shape as 0x10850.
        false}

// 0x128c8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_128c8(get: impl Fn() -> bool) -> bool {
    // IDA 0x128c8: GetSetImpl<bool>::getValue — same member-getter dispatch as 0x10854; the getter travels as a closure.
        get()}

// 0x128fc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_128fc(set: impl Fn(bool), value: bool) {
    // IDA 0x128fc: GetSetImpl<bool>::setValue — same member-setter dispatch as 0x10878; the setter travels as a closure.
        set(value);}

// 0x12920 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::EnumPropDescriptor<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>(char const*,char const*,RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_12920() {
    // IDA 0x12920: EnumPropDescriptor<CRenderSettingsItem,QualityLevel> C2 — same template as 0xfe84 (classDescriptor ensure, EnumDesc singleton call_once + doGet, PropertyDescriptor attach); the descriptor heap lives engine-side — faithful no-op shell.
    }

// 0x12ad4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
pub fn stub_12ad4() {
    // IDA 0x12ad4: EnumPropDescriptor D0 — same template as 0x10038 (vtable reset, impl-holder delete when non-null, operator delete); drops with Rust ownership.
    }

// 0x12b00 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isReadOnly(void)const")]
pub fn stub_12b00() -> bool {
    // IDA 0x12b00: EnumPropDescriptor::isReadOnly — same forward-to-impl template as 0x10064; the QualityLevel impl answers 0.
        false}

// 0x12b10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isWriteOnly(void)const")]
pub fn stub_12b10() -> bool {
    // IDA 0x12b10: EnumPropDescriptor::isWriteOnly — same forward-to-impl template as 0x10074; the QualityLevel impl answers 0.
        false}

// 0x12b20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_12b20(a: i32, b: i32) -> bool {
    // IDA 0x12b20: EnumPropDescriptor::equalValues — same get-vs-get template as 0x10084; the host compares carried values directly.
        a == b}

// 0x12b48 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_12b48(get: impl Fn() -> i32) -> i32 {
    // IDA 0x12b48: EnumPropDescriptor::getVariant — same get + int-Variant-wrap template as 0x100ac; the host carries the int.
        get()}

// 0x12b6c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_12b6c(value: i32, set: impl Fn(i32)) {
    // IDA 0x12b6c: EnumPropDescriptor::setVariant — same any_cast/convert + setter template as 0x100d0; the host carries the int.
        set(value);}

// 0x12cbc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_12cbc(get: impl Fn() -> i32, set: impl Fn(i32)) {
    // IDA 0x12cbc: EnumPropDescriptor::copyValue — same get-then-set template as 0x10220.
        let v = get();
    set(v);}

// 0x12ce0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::hasStringValue(void)const")]
pub fn stub_12ce0() -> bool {
    // IDA 0x12ce0: EnumPropDescriptor::hasStringValue — same 1-return template as 0x10244.
        true}
