//! rendering shard 430 — 100 stubs 0x66f2a0..0x671ef4 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 46310->46410 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x66f2a0..0x671ef4 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x66f2a0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x66f2a0: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f2a0() {
}

// 0x66f2d8 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x66f2d8: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f2d8() {
}

// 0x66f2fc — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x66f2fc: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f2fc() {
}

// 0x66f4b0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEED0Ev
// IDA 0x66f4b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_66f4b0() {
}

// 0x66f4dc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10isReadOnlyEv
// IDA 0x66f4dc: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f4dc() {
}

// 0x66f4ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11isWriteOnlyEv
// IDA 0x66f4ec: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f4ec() {
}

// 0x66f4fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11equalValuesEPKNS0_13DescribedBaseES8_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11equalValuesEPKNS0_13DescribedBaseES8_
// IDA 0x66f4fc: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f4fc() {
}

// 0x66f524 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x66f524: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f524() {
}

// 0x66f548 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x66f548: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f548() {
}

// 0x66f694 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9copyValueEPKNS0_13DescribedBaseEPS6_
// IDA 0x66f694: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f694() {
}

// 0x66f6b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14hasStringValueEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14hasStringValueEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14hasStringValueEv
// IDA 0x66f6b8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f6b8() {
}

// 0x66f6bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14getStringValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x66f6bc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f6bc() {
}

// 0x66f6e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x66f6e0: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f6e0() {
}

// 0x66f720 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x66f720: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f720() {
}

// 0x66f740 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x66f740: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f740() {
}

// 0x66f980 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13getIndexValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x66f980: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f980() {
}

// 0x66f99c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13setIndexValueEPNS0_13DescribedBaseEm")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x66f99c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f99c() {
}

// 0x66f9d0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12getEnumValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x66f9d0: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f9d0() {
}

// 0x66f9d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12setEnumValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x66f9d8: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f9d8() {
}

// 0x66fa24 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11getEnumItemEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x66fa24: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66fa24() {
}

// 0x66fa44 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x66fa44: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66fa44() {
}

// 0x66fa78 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueERKNS_4NameERS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(RBX::Name const&,RBX::TextService::Font&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueERKNS_4NameERS3_
// IDA 0x66fa78: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66fa78() {
}

// 0x66faf4 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE13convertToItemERKS3_
// type: int __fastcall(int, int *, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE13convertToItemERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToItem(RBX::TextService::Font const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE13convertToItemERKS3_
// IDA 0x66faf4: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66faf4() {
}

// 0x66fbc0 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToIndexES3_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToIndexES3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToIndex(RBX::TextService::Font)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToIndexES3_
// IDA 0x66fbc0: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66fbc0() {
}

// 0x66fc30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x66fc30: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66fc30() {
}

// 0x66fc70 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(RBX::TextService::Font const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringERKS3_
// IDA 0x66fc70: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66fc70() {
}

// 0x66fe10 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// IDA 0x66fe10: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66fe10() {
}

// 0x66fe14 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// IDA 0x66fe14: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66fe14() {
}

// 0x66fe18 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x66fe18: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66fe18() {
}

// 0x66fe44 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::Font const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x66fe44: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66fe44() {
}

// 0x66fe68 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE13initSingletonEv
// type: 
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE13initSingletonEv")]
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE13initSingletonEv
// IDA 0x66fe68: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_66fe68() {
}

// 0x66fe6c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE14doGetSingletonEv")]
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE14doGetSingletonEv
// IDA 0x66fe6c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66fe6c() {
}

// 0x66ff5c — __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED1Ev
// IDA 0x66ff5c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_66ff5c() {
}

// 0x66ff60 — __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED2Ev
// IDA 0x66ff60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_66ff60() {
}

// 0x670134 — __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED0Ev
// IDA 0x670134: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_670134() {
}

// 0x6701d4 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE6lookupEPKc")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE6lookupEPKc
// IDA 0x6701d4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6701d4() {
}

// 0x670204 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE6lookupERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE6lookupERKNS0_7VariantE
// IDA 0x670204: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670204() {
}

// 0x670224 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueEmRNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueEmRNS0_7VariantE
// IDA 0x670224: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670224() {
}

// 0x670258 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringEmRSs")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringEmRSs
// IDA 0x670258: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670258() {
}

// 0x67039c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::Font>(RBX::TextService::Font const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_
// IDA 0x67039c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_67039c() {
}

// 0x6703ec — __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE9singletonEv
// type: _DWORD *()
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::Font>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE9singletonEv
// IDA 0x6703ec: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6703ec() {
}

// 0x670458 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE14construct_funcEPKcPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::Font>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE14construct_funcEPKcPc
// IDA 0x670458: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670458() {
}

// 0x670464 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE13destruct_funcEPc
// type: void()
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::Font>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE13destruct_funcEPc
// IDA 0x670464: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_670464() {
}

// 0x670468 — __ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::TextService::Font const& rbx::any_cast<RBX::TextService::Font const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x670468: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670468() {
}

// 0x670558 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::Font>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::Font>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x670558: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670558() {
}

// 0x670580 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::EnumPropDescriptor<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>(char const*,char const*,RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x670580: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670580() {
}

// 0x670734 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEED0Ev
// IDA 0x670734: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_670734() {
}

// 0x670760 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10isReadOnlyEv
// IDA 0x670760: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670760() {
}

// 0x670770 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11isWriteOnlyEv
// IDA 0x670770: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670770() {
}

// 0x670780 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11equalValuesEPKNS0_13DescribedBaseES8_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11equalValuesEPKNS0_13DescribedBaseES8_
// IDA 0x670780: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670780() {
}

// 0x6707a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x6707a8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6707a8() {
}

// 0x6707cc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x6707cc: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6707cc() {
}

// 0x670918 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// IDA 0x670918: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670918() {
}

// 0x67093c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14hasStringValueEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14hasStringValueEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14hasStringValueEv
// IDA 0x67093c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_67093c() {
}

// 0x670940 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14getStringValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x670940: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670940() {
}

// 0x670964 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x670964: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670964() {
}

// 0x6709a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x6709a4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6709a4() {
}

// 0x6709c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x6709c4: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6709c4() {
}

// 0x670c04 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE13getIndexValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x670c04: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670c04() {
}

// 0x670c20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE13setIndexValueEPNS0_13DescribedBaseEm")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x670c20: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670c20() {
}

// 0x670c54 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE12getEnumValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x670c54: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670c54() {
}

// 0x670c5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE12setEnumValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x670c5c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670c5c() {
}

// 0x670ca8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11getEnumItemEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x670ca8: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670ca8() {
}

// 0x670cc8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x670cc8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670cc8() {
}

// 0x670cfc — __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToValueERKNS_4NameERS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToValue(RBX::Name const&,RBX::TextService::FontSize&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToValueERKNS_4NameERS3_
// IDA 0x670cfc: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670cfc() {
}

// 0x670d78 — __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE13convertToItemERKS3_
// type: int __fastcall(int, int *, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE13convertToItemERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToItem(RBX::TextService::FontSize const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE13convertToItemERKS3_
// IDA 0x670d78: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670d78() {
}

// 0x670e44 — __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToIndexES3_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToIndexES3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToIndex(RBX::TextService::FontSize)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToIndexES3_
// IDA 0x670e44: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670e44() {
}

// 0x670eb4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11setIntValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x670eb4: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670eb4() {
}

// 0x670ef4 — __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE15convertToStringERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToString(RBX::TextService::FontSize const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE15convertToStringERKS3_
// IDA 0x670ef4: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_670ef4() {
}

// 0x671094 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::GetSetImpl<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// IDA 0x671094: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671094() {
}

// 0x671098 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::GetSetImpl<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// IDA 0x671098: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671098() {
}

// 0x67109c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::GetSetImpl<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x67109c: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_67109c() {
}

// 0x6710c8 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::GetSetImpl<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::FontSize const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x6710c8: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6710c8() {
}

// 0x6710ec — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE13initSingletonEv
// type: 
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE13initSingletonEv")]
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::FontSize> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE13initSingletonEv
// IDA 0x6710ec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6710ec() {
}

// 0x6710f0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE14doGetSingletonEv")]
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::FontSize> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE14doGetSingletonEv
// IDA 0x6710f0: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6710f0() {
}

// 0x6711e0 — __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED1Ev
// IDA 0x6711e0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6711e0() {
}

// 0x6711e4 — __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED2Ev
// IDA 0x6711e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6711e4() {
}

// 0x6713b8 — __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED0Ev
// IDA 0x6713b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6713b8() {
}

// 0x671458 — __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE6lookupEPKc")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE6lookupEPKc
// IDA 0x671458: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671458() {
}

// 0x671488 — __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE6lookupERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE6lookupERKNS0_7VariantE
// IDA 0x671488: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671488() {
}

// 0x6714a8 — __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToValueEmRNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToValueEmRNS0_7VariantE
// IDA 0x6714a8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6714a8() {
}

// 0x6714dc — __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE15convertToStringEmRSs")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE15convertToStringEmRSs
// IDA 0x6714dc: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6714dc() {
}

// 0x671620 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService8FontSizeEEERS3_RKT_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService8FontSizeEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::FontSize>(RBX::TextService::FontSize const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService8FontSizeEEERS3_RKT_
// IDA 0x671620: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671620() {
}

// 0x671670 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::FontSize>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE9singletonEv
// IDA 0x671670: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671670() {
}

// 0x6716dc — __ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE14construct_funcEPKcPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::FontSize>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE14construct_funcEPKcPc
// IDA 0x6716dc: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6716dc() {
}

// 0x6716e8 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE13destruct_funcEPc
// type: void()
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::FontSize>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE13destruct_funcEPc
// IDA 0x6716e8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6716e8() {
}

// 0x6716ec — __ZN3rbx8any_castIRKN3RBX11TextService8FontSizeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService8FontSizeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::TextService::FontSize const& rbx::any_cast<RBX::TextService::FontSize const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX11TextService8FontSizeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x6716ec: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6716ec() {
}

// 0x6717dc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::FontSize>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::FontSize>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x6717dc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6717dc() {
}

// 0x671804 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsEC2IMNS_12GuiTextMixinEKFSsvEMS2_FvSsEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsEC2IMNS_12GuiTextMixinEKFSsvEMS2_FvSsEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::PropDescriptor<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>(char const*,char const*,std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsEC2IMNS_12GuiTextMixinEKFSsvEMS2_FvSsEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x671804: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671804() {
}

// 0x671918 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsED0Ev
// IDA 0x671918: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_671918() {
}

// 0x671944 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::GetSetImpl<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE10isReadOnlyEv
// IDA 0x671944: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671944() {
}

// 0x671948 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::GetSetImpl<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE11isWriteOnlyEv
// IDA 0x671948: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671948() {
}

// 0x67194c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::GetSetImpl<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x67194c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_67194c() {
}

// 0x671984 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
// type: void __fastcall(int, int, const std::string *)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::GetSetImpl<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
// IDA 0x671984: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671984() {
}

// 0x671ac8 — __ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::EventDesc(rbx::signal<void ()(bool)> RBX::TextBox::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x671ac8: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671ac8() {
}

// 0x671c4c — __ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// IDA 0x671c4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_671c4c() {
}

// 0x671d00 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x671d00: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671d00() {
}

// 0x671e54 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x671e54: 45 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671e54() {
}

// 0x671ee0 — __ZNK3RBX10Reflection13EventDescBaseINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x671ee0: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671ee0() {
}

// 0x671ef4 — __ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TextBox,void ()(void),0>::BoundFuncDesc(void (RBX::TextBox::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x671ef4: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_671ef4() {
}
