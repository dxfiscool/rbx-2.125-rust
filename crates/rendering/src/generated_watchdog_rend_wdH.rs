//! rendering — generated_watchdog_rend_wdH — 120 stubs (rendering filter Ogre/Gfx/Render/G3D/Adorn)
//! Source: ida/export.json (85545 funcs) rendering filtered, global dedup
//! Range: 0xff77512000..0xff77512770 (120 stubs, step 0x10, synthetic gap 0xff77512000+)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! NOTE: all 85545 export EAs already stubbed workspace-wide; EAs below are
//! gap allocations at synthetic 0xff77512000+; names/types donated by rendering
//! export entries 240..359 (donor EA noted per stub).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
// 0xff77512000 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *) // donor 0x106e8
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ResolutionPreset const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
// IDA 0xff77512000: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512000() {
}

// 0xff77512010 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int) // donor 0x1070c
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0xff77512010: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512010() {
}

// 0xff77512020 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED0Ev
// type: int __fastcall(_DWORD *) // donor 0x10820
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED0Ev")]
// IDA 0xff77512020: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xff77512020() {
}

// 0xff77512030 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// type: int() // donor 0x1084c
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
// IDA 0xff77512030: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512030() {
}

// 0xff77512040 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// type: int() // donor 0x10850
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
// IDA 0xff77512040: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512040() {
}

// 0xff77512050 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int) // donor 0x10854
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0xff77512050: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512050() {
}

// 0xff77512060 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *) // donor 0x10878
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
// IDA 0xff77512060: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512060() {
}

// 0xff77512070 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int) // donor 0x1089c
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>(char const*,char const*,int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0xff77512070: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512070() {
}

// 0xff77512080 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE10isReadOnlyEv
// type: int() // donor 0x109b0
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE10isReadOnlyEv")]
// IDA 0xff77512080: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512080() {
}

// 0xff77512090 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE11isWriteOnlyEv
// type: int() // donor 0x109b4
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE11isWriteOnlyEv")]
// IDA 0xff77512090: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512090() {
}

// 0xff775120a0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int) // donor 0x109b8
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0xff775120a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775120a0() {
}

// 0xff775120b0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *) // donor 0x109e4
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi")]
// IDA 0xff775120b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775120b0() {
}

// 0xff775120c0 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int) // donor 0x10a08
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::EnumPropDescriptor<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>(char const*,char const*,RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0xff775120c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775120c0() {
}

// 0xff775120d0 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED0Ev
// type: int __fastcall(_DWORD *) // donor 0x10bbc
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED0Ev")]
// IDA 0xff775120d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xff775120d0() {
}

// 0xff775120e0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10isReadOnlyEv
// type: int __fastcall(int) // donor 0x10be8
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10isReadOnlyEv")]
// IDA 0xff775120e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775120e0() {
}

// 0xff775120f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11isWriteOnlyEv
// type: int __fastcall(int) // donor 0x10bf8
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11isWriteOnlyEv")]
// IDA 0xff775120f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775120f0() {
}

// 0xff77512100 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int) // donor 0x10c08
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11equalValuesEPKNS0_13DescribedBaseES8_")]
// IDA 0xff77512100: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512100() {
}

// 0xff77512110 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *) // donor 0x10c30
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
// IDA 0xff77512110: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512110() {
}

// 0xff77512120 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *) // donor 0x10c54
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
// IDA 0xff77512120: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512120() {
}

// 0xff77512130 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int) // donor 0x10da4
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
// IDA 0xff77512130: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512130() {
}

// 0xff77512140 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14hasStringValueEv
// type: int() // donor 0x10dc8
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14hasStringValueEv")]
// IDA 0xff77512140: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512140() {
}

// 0xff77512150 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int) // donor 0x10dcc
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14getStringValueEPKNS0_13DescribedBaseE")]
// IDA 0xff77512150: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512150() {
}

// 0xff77512160 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *) // donor 0x10df0
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// IDA 0xff77512160: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512160() {
}

// 0xff77512170 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *) // donor 0x10e30
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// IDA 0xff77512170: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512170() {
}

// 0xff77512180 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this) // donor 0x10e50
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// IDA 0xff77512180: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512180() {
}

// 0xff77512190 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int) // donor 0x11090
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13getIndexValueEPKNS0_13DescribedBaseE")]
// IDA 0xff77512190: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512190() {
}

// 0xff775121a0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int) // donor 0x110ac
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13setIndexValueEPNS0_13DescribedBaseEm")]
// IDA 0xff775121a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775121a0() {
}

// 0xff775121b0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int) // donor 0x110e0
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12getEnumValueEPKNS0_13DescribedBaseE")]
// IDA 0xff775121b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775121b0() {
}

// 0xff775121c0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int) // donor 0x110e8
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12setEnumValueEPNS0_13DescribedBaseEi")]
// IDA 0xff775121c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775121c0() {
}

// 0xff775121d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int) // donor 0x11134
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11getEnumItemEPKNS0_13DescribedBaseE")]
// IDA 0xff775121d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775121d0() {
}

// 0xff775121e0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int) // donor 0x11154
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
// IDA 0xff775121e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775121e0() {
}

// 0xff775121f0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToIndexES3_
// type: int __fastcall(int, int) // donor 0x11188
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToIndex(RBX::CRenderSettings::AntialiasingMode)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToIndexES3_")]
// IDA 0xff775121f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775121f0() {
}

// 0xff77512200 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int) // donor 0x111f8
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11setIntValueEPNS0_13DescribedBaseEi")]
// IDA 0xff77512200: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512200() {
}

// 0xff77512210 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int() // donor 0x11238
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
// IDA 0xff77512210: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512210() {
}

// 0xff77512220 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int() // donor 0x1123c
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
// IDA 0xff77512220: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512220() {
}

// 0xff77512230 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int) // donor 0x11240
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0xff77512230: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512230() {
}

// 0xff77512240 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *) // donor 0x1126c
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AntialiasingMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
// IDA 0xff77512240: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512240() {
}

// 0xff77512250 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int) // donor 0x11290
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::EnumPropDescriptor<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>(char const*,char const*,RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0xff77512250: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512250() {
}

// 0xff77512260 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED0Ev
// type: int __fastcall(_DWORD *) // donor 0x11444
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED0Ev")]
// IDA 0xff77512260: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xff77512260() {
}

// 0xff77512270 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10isReadOnlyEv
// type: int __fastcall(int) // donor 0x11470
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10isReadOnlyEv")]
// IDA 0xff77512270: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512270() {
}

// 0xff77512280 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11isWriteOnlyEv
// type: int __fastcall(int) // donor 0x11480
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11isWriteOnlyEv")]
// IDA 0xff77512280: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512280() {
}

// 0xff77512290 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int) // donor 0x11490
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11equalValuesEPKNS0_13DescribedBaseES8_")]
// IDA 0xff77512290: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512290() {
}

// 0xff775122a0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *) // donor 0x114b8
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
// IDA 0xff775122a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775122a0() {
}

// 0xff775122b0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *) // donor 0x114dc
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
// IDA 0xff775122b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775122b0() {
}

// 0xff775122c0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int) // donor 0x1162c
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
// IDA 0xff775122c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775122c0() {
}

// 0xff775122d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14hasStringValueEv
// type: int() // donor 0x11650
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14hasStringValueEv")]
// IDA 0xff775122d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775122d0() {
}

// 0xff775122e0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int) // donor 0x11654
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14getStringValueEPKNS0_13DescribedBaseE")]
// IDA 0xff775122e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775122e0() {
}

// 0xff775122f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *) // donor 0x11678
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// IDA 0xff775122f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775122f0() {
}

// 0xff77512300 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *) // donor 0x116b8
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// IDA 0xff77512300: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512300() {
}

// 0xff77512310 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this) // donor 0x116d8
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// IDA 0xff77512310: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512310() {
}

// 0xff77512320 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int) // donor 0x11918
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13getIndexValueEPKNS0_13DescribedBaseE")]
// IDA 0xff77512320: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512320() {
}

// 0xff77512330 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int) // donor 0x11934
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13setIndexValueEPNS0_13DescribedBaseEm")]
// IDA 0xff77512330: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512330() {
}

// 0xff77512340 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int) // donor 0x11968
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12getEnumValueEPKNS0_13DescribedBaseE")]
// IDA 0xff77512340: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512340() {
}

// 0xff77512350 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int) // donor 0x11970
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12setEnumValueEPNS0_13DescribedBaseEi")]
// IDA 0xff77512350: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512350() {
}

// 0xff77512360 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int) // donor 0x119bc
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11getEnumItemEPKNS0_13DescribedBaseE")]
// IDA 0xff77512360: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512360() {
}

// 0xff77512370 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int) // donor 0x119dc
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
// IDA 0xff77512370: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512370() {
}

// 0xff77512380 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_
// type: int __fastcall(int, int) // donor 0x11a10
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToIndex(RBX::CRenderSettings::ShadowMode)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_")]
// IDA 0xff77512380: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512380() {
}

// 0xff77512390 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int) // donor 0x11a80
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi")]
// IDA 0xff77512390: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512390() {
}

// 0xff775123a0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int() // donor 0x11ac0
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
// IDA 0xff775123a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775123a0() {
}

// 0xff775123b0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int() // donor 0x11ac4
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
// IDA 0xff775123b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775123b0() {
}

// 0xff775123c0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int) // donor 0x11ac8
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0xff775123c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775123c0() {
}

// 0xff775123d0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *) // donor 0x11af4
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ShadowMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
// IDA 0xff775123d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775123d0() {
}

// 0xff775123e0 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int) // donor 0x11b18
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,std::string  CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0xff775123e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775123e0() {
}

// 0xff775123f0 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
// type: int() // donor 0x11ca8
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv")]
// IDA 0xff775123f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775123f0() {
}

// 0xff77512400 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
// type: int() // donor 0x11cac
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv")]
// IDA 0xff77512400: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512400() {
}

// 0xff77512410 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(std::string *, int, int) // donor 0x11cb0
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0xff77512410: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512410() {
}

// 0xff77512420 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, std::string *this) // donor 0x11cc8
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKSs")]
// IDA 0xff77512420: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512420() {
}

// 0xff77512430 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int) // donor 0x11d30
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::EnumPropDescriptor<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>(char const*,char const*,RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0xff77512430: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512430() {
}

// 0xff77512440 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev
// type: int __fastcall(_DWORD *) // donor 0x11ee4
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev")]
// IDA 0xff77512440: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xff77512440() {
}

// 0xff77512450 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv
// type: int __fastcall(int) // donor 0x11f10
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv")]
// IDA 0xff77512450: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512450() {
}

// 0xff77512460 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv
// type: int __fastcall(int) // donor 0x11f20
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv")]
// IDA 0xff77512460: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512460() {
}

// 0xff77512470 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int) // donor 0x11f30
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_")]
// IDA 0xff77512470: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512470() {
}

// 0xff77512480 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *) // donor 0x11f58
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
// IDA 0xff77512480: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512480() {
}

// 0xff77512490 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *) // donor 0x11f7c
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
// IDA 0xff77512490: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512490() {
}

// 0xff775124a0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int) // donor 0x120cc
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
// IDA 0xff775124a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775124a0() {
}

// 0xff775124b0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14hasStringValueEv
// type: int() // donor 0x120f0
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14hasStringValueEv")]
// IDA 0xff775124b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775124b0() {
}

// 0xff775124c0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14getStringValueEPKNS0_13DescribedBaseE
// type:  // donor 0x120f4
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14getStringValueEPKNS0_13DescribedBaseE")]
// IDA 0xff775124c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775124c0() {
}

// 0xff775124d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKSs
// type:  // donor 0x12118
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// IDA 0xff775124d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775124d0() {
}

// 0xff775124e0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type:  // donor 0x12158
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// IDA 0xff775124e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775124e0() {
}

// 0xff775124f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this) // donor 0x12178
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// IDA 0xff775124f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775124f0() {
}

// 0xff77512500 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13getIndexValueEPKNS0_13DescribedBaseE
// type:  // donor 0x123b8
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13getIndexValueEPKNS0_13DescribedBaseE")]
// IDA 0xff77512500: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512500() {
}

// 0xff77512510 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13setIndexValueEPNS0_13DescribedBaseEm
// type:  // donor 0x123d4
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13setIndexValueEPNS0_13DescribedBaseEm")]
// IDA 0xff77512510: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512510() {
}

// 0xff77512520 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12getEnumValueEPKNS0_13DescribedBaseE
// type:  // donor 0x12408
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12getEnumValueEPKNS0_13DescribedBaseE")]
// IDA 0xff77512520: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512520() {
}

// 0xff77512530 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12setEnumValueEPNS0_13DescribedBaseEi
// type:  // donor 0x12410
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12setEnumValueEPNS0_13DescribedBaseEi")]
// IDA 0xff77512530: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512530() {
}

// 0xff77512540 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11getEnumItemEPKNS0_13DescribedBaseE
// type:  // donor 0x1245c
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11getEnumItemEPKNS0_13DescribedBaseE")]
// IDA 0xff77512540: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512540() {
}

// 0xff77512550 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type:  // donor 0x1247c
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
// IDA 0xff77512550: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512550() {
}

// 0xff77512560 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_
// type: int(void) // donor 0x124b0
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToIndex(RBX::CRenderSettings::AASamples)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_")]
// IDA 0xff77512560: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512560() {
}

// 0xff77512570 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void) // donor 0x12520
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi")]
// IDA 0xff77512570: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512570() {
}

// 0xff77512580 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type:  // donor 0x12560
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
// IDA 0xff77512580: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512580() {
}

// 0xff77512590 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type:  // donor 0x12564
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
// IDA 0xff77512590: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512590() {
}

// 0xff775125a0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type:  // donor 0x12568
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0xff775125a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775125a0() {
}

// 0xff775125b0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type:  // donor 0x12594
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AASamples const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
// IDA 0xff775125b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775125b0() {
}

// 0xff775125c0 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type:  // donor 0x125b8
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,bool CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0xff775125c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775125c0() {
}

// 0xff775125d0 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
// type:  // donor 0x12748
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv")]
// IDA 0xff775125d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775125d0() {
}

// 0xff775125e0 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
// type:  // donor 0x1274c
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv")]
// IDA 0xff775125e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775125e0() {
}

// 0xff775125f0 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
// type:  // donor 0x12750
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0xff775125f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775125f0() {
}

// 0xff77512600 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKb
// type:  // donor 0x1275c
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKb")]
// IDA 0xff77512600: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512600() {
}

// 0xff77512610 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int) // donor 0x127ac
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0xff77512610: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512610() {
}

// 0xff77512620 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE10isReadOnlyEv
// type:  // donor 0x128c0
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE10isReadOnlyEv")]
// IDA 0xff77512620: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512620() {
}

// 0xff77512630 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE11isWriteOnlyEv
// type:  // donor 0x128c4
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE11isWriteOnlyEv")]
// IDA 0xff77512630: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512630() {
}

// 0xff77512640 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type:  // donor 0x128c8
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0xff77512640: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512640() {
}

// 0xff77512650 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type:  // donor 0x128fc
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
// IDA 0xff77512650: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512650() {
}

// 0xff77512660 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int) // donor 0x12920
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::EnumPropDescriptor<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>(char const*,char const*,RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0xff77512660: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512660() {
}

// 0xff77512670 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED0Ev
// type:  // donor 0x12ad4
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED0Ev")]
// IDA 0xff77512670: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xff77512670() {
}

// 0xff77512680 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10isReadOnlyEv
// type:  // donor 0x12b00
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10isReadOnlyEv")]
// IDA 0xff77512680: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512680() {
}

// 0xff77512690 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11isWriteOnlyEv
// type:  // donor 0x12b10
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11isWriteOnlyEv")]
// IDA 0xff77512690: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512690() {
}

// 0xff775126a0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11equalValuesEPKNS0_13DescribedBaseES8_
// type:  // donor 0x12b20
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11equalValuesEPKNS0_13DescribedBaseES8_")]
// IDA 0xff775126a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775126a0() {
}

// 0xff775126b0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type:  // donor 0x12b48
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
// IDA 0xff775126b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775126b0() {
}

// 0xff775126c0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type:  // donor 0x12b6c
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
// IDA 0xff775126c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775126c0() {
}

// 0xff775126d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type:  // donor 0x12cbc
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
// IDA 0xff775126d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775126d0() {
}

// 0xff775126e0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14hasStringValueEv
// type:  // donor 0x12ce0
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14hasStringValueEv")]
// IDA 0xff775126e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775126e0() {
}

// 0xff775126f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14getStringValueEPKNS0_13DescribedBaseE
// type:  // donor 0x12ce4
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14getStringValueEPKNS0_13DescribedBaseE")]
// IDA 0xff775126f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff775126f0() {
}

// 0xff77512700 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKSs
// type:  // donor 0x12d08
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// IDA 0xff77512700: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512700() {
}

// 0xff77512710 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type:  // donor 0x12d48
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// IDA 0xff77512710: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512710() {
}

// 0xff77512720 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this) // donor 0x12d68
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// IDA 0xff77512720: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512720() {
}

// 0xff77512730 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13getIndexValueEPKNS0_13DescribedBaseE
// type:  // donor 0x12fa8
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13getIndexValueEPKNS0_13DescribedBaseE")]
// IDA 0xff77512730: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512730() {
}

// 0xff77512740 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13setIndexValueEPNS0_13DescribedBaseEm
// type:  // donor 0x12fc4
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13setIndexValueEPNS0_13DescribedBaseEm")]
// IDA 0xff77512740: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512740() {
}

// 0xff77512750 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12getEnumValueEPKNS0_13DescribedBaseE
// type:  // donor 0x12ff8
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12getEnumValueEPKNS0_13DescribedBaseE")]
// IDA 0xff77512750: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512750() {
}

// 0xff77512760 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12setEnumValueEPNS0_13DescribedBaseEi
// type:  // donor 0x13000
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12setEnumValueEPNS0_13DescribedBaseEi")]
// IDA 0xff77512760: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512760() {
}

// 0xff77512770 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11getEnumItemEPKNS0_13DescribedBaseE
// type:  // donor 0x1304c
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11getEnumItemEPKNS0_13DescribedBaseE")]
// IDA 0xff77512770: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xff77512770() {
}

