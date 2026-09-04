//! rendering shard 429 — 100 stubs 0x66c93c..0x66f29c EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 46210->46310 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x66c93c..0x66f29c (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x66c93c — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEED0Ev
// IDA 0x66c93c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_66c93c() {
}

// 0x66c968 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10isReadOnlyEv
// IDA 0x66c968: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66c968() {
}

// 0x66c978 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11isWriteOnlyEv
// IDA 0x66c978: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66c978() {
}

// 0x66c988 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
// IDA 0x66c988: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66c988() {
}

// 0x66c9b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x66c9b0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66c9b0() {
}

// 0x66c9d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x66c9d4: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66c9d4() {
}

// 0x66cb20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
// IDA 0x66cb20: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66cb20() {
}

// 0x66cb44 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14hasStringValueEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14hasStringValueEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14hasStringValueEv
// IDA 0x66cb44: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66cb44() {
}

// 0x66cb48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14getStringValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x66cb48: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66cb48() {
}

// 0x66cb6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x66cb6c: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66cb6c() {
}

// 0x66cbac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x66cbac: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66cbac() {
}

// 0x66cbcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x66cbcc: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66cbcc() {
}

// 0x66ce0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x66ce0c: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ce0c() {
}

// 0x66ce28 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x66ce28: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ce28() {
}

// 0x66ce5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x66ce5c: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ce5c() {
}

// 0x66ce64 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x66ce64: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ce64() {
}

// 0x66ceb0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x66ceb0: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ceb0() {
}

// 0x66ced0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x66ced0: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ced0() {
}

// 0x66cf04 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueERKNS_4NameERS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToValue(RBX::Name const&,RBX::TextService::YAlignment&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueERKNS_4NameERS3_
// IDA 0x66cf04: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66cf04() {
}

// 0x66cf80 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE13convertToItemERKS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE13convertToItemERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToItem(RBX::TextService::YAlignment const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE13convertToItemERKS3_
// IDA 0x66cf80: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66cf80() {
}

// 0x66d04c — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToIndexES3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToIndex(RBX::TextService::YAlignment)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToIndexES3_
// IDA 0x66d04c: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d04c() {
}

// 0x66d0bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x66d0bc: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d0bc() {
}

// 0x66d0fc — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToString(RBX::TextService::YAlignment const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringERKS3_
// IDA 0x66d0fc: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d0fc() {
}

// 0x66d29c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// IDA 0x66d29c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d29c() {
}

// 0x66d2a0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// IDA 0x66d2a0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d2a0() {
}

// 0x66d2a4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x66d2a4: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d2a4() {
}

// 0x66d2d0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::YAlignment const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x66d2d0: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d2d0() {
}

// 0x66d2f4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE13initSingletonEv
// type: 
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE13initSingletonEv")]
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::YAlignment> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE13initSingletonEv
// IDA 0x66d2f4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_66d2f4() {
}

// 0x66d2f8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE14doGetSingletonEv
// type: 
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE14doGetSingletonEv")]
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::YAlignment> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE14doGetSingletonEv
// IDA 0x66d2f8: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d2f8() {
}

// 0x66d3e8 — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED1Ev
// IDA 0x66d3e8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_66d3e8() {
}

// 0x66d3ec — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED2Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED2Ev
// IDA 0x66d3ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_66d3ec() {
}

// 0x66d5c0 — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED0Ev
// IDA 0x66d5c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_66d5c0() {
}

// 0x66d660 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE6lookupEPKc
// type: 
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE6lookupEPKc")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE6lookupEPKc
// IDA 0x66d660: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d660() {
}

// 0x66d690 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE6lookupERKNS0_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE6lookupERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE6lookupERKNS0_7VariantE
// IDA 0x66d690: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d690() {
}

// 0x66d6b0 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueEmRNS0_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueEmRNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueEmRNS0_7VariantE
// IDA 0x66d6b0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d6b0() {
}

// 0x66d6e4 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringEmRSs
// type: 
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringEmRSs")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringEmRSs
// IDA 0x66d6e4: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d6e4() {
}

// 0x66d828 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10YAlignmentEEERS3_RKT_
// type: int(void)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10YAlignmentEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::YAlignment>(RBX::TextService::YAlignment const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10YAlignmentEEERS3_RKT_
// IDA 0x66d828: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d828() {
}

// 0x66d878 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE9singletonEv
// type: int(void)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::YAlignment>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE9singletonEv
// IDA 0x66d878: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d878() {
}

// 0x66d8e4 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE14construct_funcEPKcPc
// type: 
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE14construct_funcEPKcPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::YAlignment>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE14construct_funcEPKcPc
// IDA 0x66d8e4: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d8e4() {
}

// 0x66d8f0 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE13destruct_funcEPc
// type: 
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::YAlignment>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE13destruct_funcEPc
// IDA 0x66d8f0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_66d8f0() {
}

// 0x66d8f4 — __ZN3rbx8any_castIRKN3RBX11TextService10YAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService10YAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::TextService::YAlignment const& rbx::any_cast<RBX::TextService::YAlignment const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX11TextService10YAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x66d8f4: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d8f4() {
}

// 0x66d9e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int(void)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::YAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x66d9e4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66d9e4() {
}

// 0x66da0c — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x66da0c: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66da0c() {
}

// 0x66dbc0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEED0Ev
// IDA 0x66dbc0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_66dbc0() {
}

// 0x66dbec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10isReadOnlyEv
// IDA 0x66dbec: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66dbec() {
}

// 0x66dbfc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11isWriteOnlyEv
// IDA 0x66dbfc: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66dbfc() {
}

// 0x66dc0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
// IDA 0x66dc0c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66dc0c() {
}

// 0x66dc34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x66dc34: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66dc34() {
}

// 0x66dc58 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x66dc58: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66dc58() {
}

// 0x66dda4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
// IDA 0x66dda4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66dda4() {
}

// 0x66ddc8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14hasStringValueEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14hasStringValueEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14hasStringValueEv
// IDA 0x66ddc8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ddc8() {
}

// 0x66ddcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14getStringValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x66ddcc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ddcc() {
}

// 0x66ddf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x66ddf0: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ddf0() {
}

// 0x66de30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x66de30: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66de30() {
}

// 0x66de50 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x66de50: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66de50() {
}

// 0x66e090 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x66e090: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e090() {
}

// 0x66e0ac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x66e0ac: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e0ac() {
}

// 0x66e0e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x66e0e0: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e0e0() {
}

// 0x66e0e8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x66e0e8: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e0e8() {
}

// 0x66e134 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x66e134: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e134() {
}

// 0x66e154 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x66e154: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e154() {
}

// 0x66e188 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueERKNS_4NameERS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToValue(RBX::Name const&,RBX::TextService::XAlignment&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueERKNS_4NameERS3_
// IDA 0x66e188: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e188() {
}

// 0x66e204 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE13convertToItemERKS3_
// type: int __fastcall(int, int *, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE13convertToItemERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToItem(RBX::TextService::XAlignment const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE13convertToItemERKS3_
// IDA 0x66e204: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e204() {
}

// 0x66e2d0 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToIndexES3_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToIndexES3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToIndex(RBX::TextService::XAlignment)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToIndexES3_
// IDA 0x66e2d0: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e2d0() {
}

// 0x66e340 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x66e340: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e340() {
}

// 0x66e380 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToString(RBX::TextService::XAlignment const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringERKS3_
// IDA 0x66e380: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e380() {
}

// 0x66e520 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// IDA 0x66e520: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e520() {
}

// 0x66e524 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// IDA 0x66e524: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e524() {
}

// 0x66e528 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x66e528: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e528() {
}

// 0x66e554 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::XAlignment const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x66e554: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e554() {
}

// 0x66e578 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE13initSingletonEv
// type: int()
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE13initSingletonEv")]
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::XAlignment> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE13initSingletonEv
// IDA 0x66e578: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_66e578() {
}

// 0x66e57c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE14doGetSingletonEv")]
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::XAlignment> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE14doGetSingletonEv
// IDA 0x66e57c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e57c() {
}

// 0x66e66c — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED1Ev
// IDA 0x66e66c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_66e66c() {
}

// 0x66e670 — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED2Ev
// IDA 0x66e670: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_66e670() {
}

// 0x66e844 — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED0Ev
// IDA 0x66e844: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_66e844() {
}

// 0x66e8e4 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE6lookupEPKc
// type: int __fastcall(_DWORD *, const char *const *)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE6lookupEPKc")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE6lookupEPKc
// IDA 0x66e8e4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e8e4() {
}

// 0x66e914 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE6lookupERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE6lookupERKNS0_7VariantE
// IDA 0x66e914: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e914() {
}

// 0x66e934 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueEmRNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueEmRNS0_7VariantE
// IDA 0x66e934: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e934() {
}

// 0x66e968 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringEmRSs")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringEmRSs
// IDA 0x66e968: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66e968() {
}

// 0x66eaac — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10XAlignmentEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10XAlignmentEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::XAlignment>(RBX::TextService::XAlignment const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10XAlignmentEEERS3_RKT_
// IDA 0x66eaac: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66eaac() {
}

// 0x66eafc — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE9singletonEv
// type: _DWORD *()
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::XAlignment>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE9singletonEv
// IDA 0x66eafc: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66eafc() {
}

// 0x66eb68 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE14construct_funcEPKcPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::XAlignment>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE14construct_funcEPKcPc
// IDA 0x66eb68: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66eb68() {
}

// 0x66eb74 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE13destruct_funcEPc
// type: void()
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::XAlignment>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE13destruct_funcEPc
// IDA 0x66eb74: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_66eb74() {
}

// 0x66eb78 — __ZN3rbx8any_castIRKN3RBX11TextService10XAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService10XAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::TextService::XAlignment const& rbx::any_cast<RBX::TextService::XAlignment const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX11TextService10XAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x66eb78: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66eb78() {
}

// 0x66ec68 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::XAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x66ec68: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ec68() {
}

// 0x66ec90 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x66ec90: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ec90() {
}

// 0x66eda4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE10isReadOnlyEv
// IDA 0x66eda4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66eda4() {
}

// 0x66eda8 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE11isWriteOnlyEv
// IDA 0x66eda8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66eda8() {
}

// 0x66edac — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x66edac: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66edac() {
}

// 0x66ede0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x66ede0: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ede0() {
}

// 0x66ee04 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x66ee04: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ee04() {
}

// 0x66ef18 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfED0Ev
// IDA 0x66ef18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_66ef18() {
}

// 0x66ef44 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE10isReadOnlyEv
// IDA 0x66ef44: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ef44() {
}

// 0x66ef48 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE11isWriteOnlyEv
// IDA 0x66ef48: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ef48() {
}

// 0x66ef4c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x66ef4c: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ef4c() {
}

// 0x66ef78 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// IDA 0x66ef78: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66ef78() {
}

// 0x66f158 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x66f158: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f158() {
}

// 0x66f26c — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEED0Ev
// IDA 0x66f26c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_66f26c() {
}

// 0x66f298 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x66f298: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f298() {
}

// 0x66f29c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x66f29c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_66f29c() {
}
