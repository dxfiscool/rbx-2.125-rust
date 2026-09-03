//! rendering shard 327 — 120 stubs 0x499aa8..0x49eac0 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 35620->35740 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 35620 before -> 35740 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 120 after 0x499aa8 (lowest remaining 0x499aa8..0x49eac0, next lowest 0x49eb28)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x499aa8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x499aa8: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499aa8() {
}

// 0x499bf4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x499bf4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499bf4() {
}

// 0x499c18 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14hasStringValueEv
// IDA 0x499c18: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499c18() {
}

// 0x499c1c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x499c1c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499c1c() {
}

// 0x499c40 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x499c40: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499c40() {
}

// 0x499c80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x499c80: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499c80() {
}

// 0x499ca0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x499ca0: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499ca0() {
}

// 0x499ee0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x499ee0: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499ee0() {
}

// 0x499efc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x499efc: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499efc() {
}

// 0x499f30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x499f30: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499f30() {
}

// 0x499f38 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x499f38: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499f38() {
}

// 0x499f84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x499f84: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499f84() {
}

// 0x499fa4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x499fa4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499fa4() {
}

// 0x499fd8 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToIndex(RBX::DialogRoot::DialogTone)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToIndexES3_
// IDA 0x499fd8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499fd8() {
}

// 0x49a048 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x49a048: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a048() {
}

// 0x49a088 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x49a088: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a088() {
}

// 0x49a08c — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x49a08c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a08c() {
}

// 0x49a090 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x49a090: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a090() {
}

// 0x49a0b0 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::setValue(RBX::Reflection::DescribedBase *,RBX::DialogRoot::DialogTone const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x49a0b0: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a0b0() {
}

// 0x49a0d4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::EnumPropDescriptor<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>(char const*,char const*,RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x49a0d4: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a0d4() {
}

// 0x49a288 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEED0Ev
// IDA 0x49a288: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49a288() {
}

// 0x49a2b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10isReadOnlyEv
// IDA 0x49a2b4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a2b4() {
}

// 0x49a2c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11isWriteOnlyEv
// IDA 0x49a2c4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a2c4() {
}

// 0x49a2d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x49a2d4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a2d4() {
}

// 0x49a2fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x49a2fc: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a2fc() {
}

// 0x49a320 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x49a320: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a320() {
}

// 0x49a46c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x49a46c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a46c() {
}

// 0x49a490 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14hasStringValueEv
// IDA 0x49a490: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a490() {
}

// 0x49a494 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x49a494: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a494() {
}

// 0x49a4b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x49a4b8: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a4b8() {
}

// 0x49a4f8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x49a4f8: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a4f8() {
}

// 0x49a518 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x49a518: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a518() {
}

// 0x49a758 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x49a758: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a758() {
}

// 0x49a774 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x49a774: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a774() {
}

// 0x49a7a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x49a7a8: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a7a8() {
}

// 0x49a7b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x49a7b0: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a7b0() {
}

// 0x49a7fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x49a7fc: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a7fc() {
}

// 0x49a81c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x49a81c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a81c() {
}

// 0x49a850 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToIndex(RBX::DialogRoot::DialogPurpose)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_
// IDA 0x49a850: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a850() {
}

// 0x49a8c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x49a8c0: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a8c0() {
}

// 0x49a900 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x49a900: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a900() {
}

// 0x49a904 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x49a904: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a904() {
}

// 0x49a908 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x49a908: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a908() {
}

// 0x49a928 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::setValue(RBX::Reflection::DescribedBase *,RBX::DialogRoot::DialogPurpose const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x49a928: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a928() {
}

// 0x49a94c — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::PropDescriptor<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>(char const*,char const*,std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x49a94c: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49a94c() {
}

// 0x49aa60 — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsED0Ev
// IDA 0x49aa60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49aa60() {
}

// 0x49aa8c — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
// IDA 0x49aa8c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49aa8c() {
}

// 0x49aa90 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
// IDA 0x49aa90: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49aa90() {
}

// 0x49aa94 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x49aa94: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49aa94() {
}

// 0x49aabc — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
// IDA 0x49aabc: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49aabc() {
}

// 0x49ac00 — __ZN3RBX10DialogRootD2Ev
// type: void __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "RBX::DialogRoot::~DialogRoot()")]
// was: __ZN3RBX10DialogRootD2Ev
// IDA 0x49ac00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49ac00() {
}

// 0x49ad94 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EED2Ev
// IDA 0x49ad94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49ad94() {
}

// 0x49aee0 — __GLOBAL__I_a_185
#[doc(alias = "global constructor keyed to_a_185")]
// was: __GLOBAL__I_a_185
// IDA 0x49aee0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_49aee0() {
}

// 0x49b3e0 — __ZN3RBX6EffectC2Ev
// type: _DWORD __fastcall(RBX::Effect *__hidden this)
#[doc(alias = "RBX::Effect::Effect(void)")]
// was: __ZN3RBX6EffectC2Ev
// IDA 0x49b3e0: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49b3e0() {
}

// 0x49b3f0 — __ZN3RBX6EffectD0Ev
// type: void __fastcall(RBX::Effect *__hidden this)
#[doc(alias = "RBX::Effect::~Effect()")]
// was: __ZN3RBX6EffectD0Ev
// IDA 0x49b3f0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_49b3f0() {
}

// 0x49b3f4 — __ZN3RBX6EffectD1Ev
// type: void __fastcall(RBX::Effect *__hidden this)
#[doc(alias = "RBX::Effect::~Effect()")]
// was: __ZN3RBX6EffectD1Ev
// IDA 0x49b3f4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_49b3f4() {
}

// 0x49b3f8 — __ZN3RBX6EffectD2Ev
// type: void __fastcall(RBX::Effect *__hidden this)
#[doc(alias = "RBX::Effect::~Effect()")]
// was: __ZN3RBX6EffectD2Ev
// IDA 0x49b3f8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_49b3f8() {
}

// 0x49b3fc — __GLOBAL__I_a_186
#[doc(alias = "global constructor keyed to_a_186")]
// was: __GLOBAL__I_a_186
// IDA 0x49b3fc: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_49b3fc() {
}

// 0x49b52c — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEC1Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEC1Ev
// IDA 0x49b52c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_49b52c() {
}

// 0x49b530 — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEC2Ev
// IDA 0x49b530: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49b530() {
}

// 0x49b708 — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEC1Ev
// IDA 0x49b708: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_49b708() {
}

// 0x49b70c — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEC2Ev
// IDA 0x49b70c: 215 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49b70c() {
}

// 0x49b964 — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEEC1Ev
// IDA 0x49b964: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_49b964() {
}

// 0x49b968 — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEEC2Ev
// IDA 0x49b968: 190 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49b968() {
}

// 0x49bb84 — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEEC1Ev
// IDA 0x49bb84: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_49bb84() {
}

// 0x49bb88 — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEEC2Ev
// IDA 0x49bb88: 198 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49bb88() {
}

// 0x49bdbc — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC1Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC1Ev
// IDA 0x49bdbc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_49bdbc() {
}

// 0x49bdc0 — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC2Ev
// IDA 0x49bdc0: 158 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49bdc0() {
}

// 0x49bf80 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC1Ev
// IDA 0x49bf80: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_49bf80() {
}

// 0x49bf84 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC2Ev
// IDA 0x49bf84: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49bf84() {
}

// 0x49c15c — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE7addPairES3_PKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::addPair(RBX::BasicPartInstance::LegacyPartType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE7addPairES3_PKc
// IDA 0x49c15c: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49c15c() {
}

// 0x49c4bc — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::addPair(RBX::ExtrudedPartInstance::VisualTrussStyle,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE7addPairES3_PKc
// IDA 0x49c4bc: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49c4bc() {
}

// 0x49c81c — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::addPair(RBX::PrismInstance::NumSidesEnum,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE7addPairES3_PKc
// IDA 0x49c81c: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49c81c() {
}

// 0x49cb7c — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::addPair(RBX::PyramidInstance::NumSidesEnum,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE7addPairES3_PKc
// IDA 0x49cb7c: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49cb7c() {
}

// 0x49cedc — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::addPair(RBX::Handles::VisualStyle,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE7addPairES3_PKc
// IDA 0x49cedc: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49cedc() {
}

// 0x49d23c — __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::addPair(RBX::GuiObject::SizeConstraint,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE7addPairES3_PKc
// IDA 0x49d23c: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49d23c() {
}

// 0x49d59c — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::resize(unsigned long,RBX::GuiObject::SizeConstraint)")]
// was: __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE6resizeEmS2_
// IDA 0x49d59c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49d59c() {
}

// 0x49d5d0 — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::push_back(RBX::GuiObject::SizeConstraint const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE9push_backERKS2_
// IDA 0x49d5d0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_49d5d0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x49d5f8 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject14SizeConstraintESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::SizeConstraint,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject14SizeConstraintESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x49d5f8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49d5f8() {
}

// 0x49d650 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x49d650: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49d650() {
}

// 0x49d704 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x49d704: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49d704() {
}

// 0x49d75c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x49d75c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49d75c() {
}

// 0x49d7c4 — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,RBX::GuiObject::SizeConstraint const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x49d7c4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_49d7c4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x49d8a8 — __ZNSt12_Vector_baseIN3RBX9GuiObject14SizeConstraintESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX9GuiObject14SizeConstraintESaIS2_EE11_M_allocateEm
// IDA 0x49d8a8: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_49d8a8() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x49d8c0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject14SizeConstraintES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::GuiObject::SizeConstraint * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *>(RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject14SizeConstraintES6_EET0_T_S8_S7_
// IDA 0x49d8c0: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_49d8c0() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x49d8fc — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,unsigned long,RBX::GuiObject::SizeConstraint const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x49d8fc: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49d8fc() {
}

// 0x49da8c — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::resize(unsigned long,RBX::Handles::VisualStyle)")]
// was: __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE6resizeEmS2_
// IDA 0x49da8c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49da8c() {
}

// 0x49dac0 — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::push_back(RBX::Handles::VisualStyle const&)")]
// was: __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE9push_backERKS2_
// IDA 0x49dac0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_49dac0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x49dae8 — __ZNSt3mapIPKN3RBX4NameENS0_7Handles11VisualStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::Handles::VisualStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_7Handles11VisualStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x49dae8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49dae8() {
}

// 0x49db40 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x49db40: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49db40() {
}

// 0x49dbf4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x49dbf4: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49dbf4() {
}

// 0x49dc4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x49dc4c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49dc4c() {
}

// 0x49dcb4 — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,RBX::Handles::VisualStyle const&)")]
// was: __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x49dcb4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_49dcb4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x49dd98 — __ZNSt12_Vector_baseIN3RBX7Handles11VisualStyleESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX7Handles11VisualStyleESaIS2_EE11_M_allocateEm
// IDA 0x49dd98: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_49dd98() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x49ddb0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Handles11VisualStyleES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::Handles::VisualStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *>(RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Handles11VisualStyleES6_EET0_T_S8_S7_
// IDA 0x49ddb0: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_49ddb0() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x49ddec — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,unsigned long,RBX::Handles::VisualStyle const&)")]
// was: __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x49ddec: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49ddec() {
}

// 0x49df7c — __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::resize(unsigned long,RBX::PyramidInstance::NumSidesEnum)")]
// was: __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE6resizeEmS2_
// IDA 0x49df7c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49df7c() {
}

// 0x49dfb0 — __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::push_back(RBX::PyramidInstance::NumSidesEnum const&)")]
// was: __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE9push_backERKS2_
// IDA 0x49dfb0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_49dfb0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x49dfd8 — __ZNSt3mapIPKN3RBX4NameENS0_15PyramidInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::PyramidInstance::NumSidesEnum,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_15PyramidInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x49dfd8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49dfd8() {
}

// 0x49e030 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x49e030: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49e030() {
}

// 0x49e0e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x49e0e4: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49e0e4() {
}

// 0x49e13c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x49e13c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49e13c() {
}

// 0x49e1a4 — __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PyramidInstance::NumSidesEnum*,std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>>,RBX::PyramidInstance::NumSidesEnum const&)")]
// was: __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x49e1a4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_49e1a4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x49e288 — __ZNSt12_Vector_baseIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE11_M_allocateEm
// IDA 0x49e288: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_49e288() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x49e2a0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PyramidInstance12NumSidesEnumES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::PyramidInstance::NumSidesEnum * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PyramidInstance::NumSidesEnum *,RBX::PyramidInstance::NumSidesEnum *>(RBX::PyramidInstance::NumSidesEnum *,RBX::PyramidInstance::NumSidesEnum *,RBX::PyramidInstance::NumSidesEnum *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PyramidInstance12NumSidesEnumES6_EET0_T_S8_S7_
// IDA 0x49e2a0: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_49e2a0() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x49e2dc — __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PyramidInstance::NumSidesEnum*,std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>>,unsigned long,RBX::PyramidInstance::NumSidesEnum const&)")]
// was: __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x49e2dc: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49e2dc() {
}

// 0x49e46c — __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::resize(unsigned long,RBX::PrismInstance::NumSidesEnum)")]
// was: __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE6resizeEmS2_
// IDA 0x49e46c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49e46c() {
}

// 0x49e4a0 — __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::push_back(RBX::PrismInstance::NumSidesEnum const&)")]
// was: __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE9push_backERKS2_
// IDA 0x49e4a0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_49e4a0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x49e4c8 — __ZNSt3mapIPKN3RBX4NameENS0_13PrismInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::PrismInstance::NumSidesEnum,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_13PrismInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x49e4c8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49e4c8() {
}

// 0x49e520 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x49e520: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49e520() {
}

// 0x49e5d4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x49e5d4: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49e5d4() {
}

// 0x49e62c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x49e62c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49e62c() {
}

// 0x49e694 — __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PrismInstance::NumSidesEnum*,std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>>,RBX::PrismInstance::NumSidesEnum const&)")]
// was: __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x49e694: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_49e694() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x49e778 — __ZNSt12_Vector_baseIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE11_M_allocateEm
// IDA 0x49e778: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_49e778() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x49e790 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13PrismInstance12NumSidesEnumES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::PrismInstance::NumSidesEnum * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PrismInstance::NumSidesEnum *,RBX::PrismInstance::NumSidesEnum *>(RBX::PrismInstance::NumSidesEnum *,RBX::PrismInstance::NumSidesEnum *,RBX::PrismInstance::NumSidesEnum *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13PrismInstance12NumSidesEnumES6_EET0_T_S8_S7_
// IDA 0x49e790: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_49e790() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x49e7cc — __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PrismInstance::NumSidesEnum*,std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>>,unsigned long,RBX::PrismInstance::NumSidesEnum const&)")]
// was: __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x49e7cc: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49e7cc() {
}

// 0x49e95c — __ZNSt3mapIPKN3RBX4NameENS0_20ExtrudedPartInstance16VisualTrussStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::ExtrudedPartInstance::VisualTrussStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_20ExtrudedPartInstance16VisualTrussStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x49e95c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49e95c() {
}

// 0x49e9b4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x49e9b4: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49e9b4() {
}

// 0x49ea68 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x49ea68: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49ea68() {
}

// 0x49eac0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x49eac0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49eac0() {
}
