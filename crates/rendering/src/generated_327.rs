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
pub fn stub_499aa8() -> ! {
    todo!("0x499aa8 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x499bf4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_499bf4() -> ! {
    todo!("0x499bf4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x499c18 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14hasStringValueEv
pub fn stub_499c18() -> ! {
    todo!("0x499c18 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::hasStringValue(void)const")
}

// 0x499c1c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14getStringValueEPKNS0_13DescribedBaseE
pub fn stub_499c1c() -> ! {
    todo!("0x499c1c RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x499c40 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_499c40() -> ! {
    todo!("0x499c40 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x499c80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_499c80() -> ! {
    todo!("0x499c80 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x499ca0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_499ca0() -> ! {
    todo!("0x499ca0 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x499ee0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE13getIndexValueEPKNS0_13DescribedBaseE
pub fn stub_499ee0() -> ! {
    todo!("0x499ee0 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x499efc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE13setIndexValueEPNS0_13DescribedBaseEm
pub fn stub_499efc() -> ! {
    todo!("0x499efc RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x499f30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE12getEnumValueEPKNS0_13DescribedBaseE
pub fn stub_499f30() -> ! {
    todo!("0x499f30 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x499f38 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE12setEnumValueEPNS0_13DescribedBaseEi
pub fn stub_499f38() -> ! {
    todo!("0x499f38 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x499f84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11getEnumItemEPKNS0_13DescribedBaseE
pub fn stub_499f84() -> ! {
    todo!("0x499f84 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x499fa4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
pub fn stub_499fa4() -> ! {
    todo!("0x499fa4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x499fd8 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToIndex(RBX::DialogRoot::DialogTone)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToIndexES3_
pub fn stub_499fd8() -> ! {
    todo!("0x499fd8 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToIndex(RBX::DialogRoot::DialogTone)const")
}

// 0x49a048 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11setIntValueEPNS0_13DescribedBaseEi
pub fn stub_49a048() -> ! {
    todo!("0x49a048 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x49a088 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_49a088() -> ! {
    todo!("0x49a088 RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::isReadOnly(void)const")
}

// 0x49a08c — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_49a08c() -> ! {
    todo!("0x49a08c RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::isWriteOnly(void)const")
}

// 0x49a090 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_49a090() -> ! {
    todo!("0x49a090 RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x49a0b0 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::setValue(RBX::Reflection::DescribedBase *,RBX::DialogRoot::DialogTone const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_49a0b0() -> ! {
    todo!("0x49a0b0 RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::setValue(RBX::Reflection::DescribedBase *,RBX::DialogRoot::DialogTone const&)const")
}

// 0x49a0d4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::EnumPropDescriptor<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>(char const*,char const*,RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_49a0d4() -> ! {
    todo!("0x49a0d4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::EnumPropDescriptor<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>(char const*,char const*,RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x49a288 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEED0Ev
pub fn stub_49a288() -> ! {
    todo!("0x49a288 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::~EnumPropDescriptor()")
}

// 0x49a2b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10isReadOnlyEv
pub fn stub_49a2b4() -> ! {
    todo!("0x49a2b4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isReadOnly(void)const")
}

// 0x49a2c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11isWriteOnlyEv
pub fn stub_49a2c4() -> ! {
    todo!("0x49a2c4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isWriteOnly(void)const")
}

// 0x49a2d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11equalValuesEPKNS0_13DescribedBaseES7_
pub fn stub_49a2d4() -> ! {
    todo!("0x49a2d4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x49a2fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
pub fn stub_49a2fc() -> ! {
    todo!("0x49a2fc RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x49a320 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_49a320() -> ! {
    todo!("0x49a320 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x49a46c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_49a46c() -> ! {
    todo!("0x49a46c RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x49a490 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14hasStringValueEv
pub fn stub_49a490() -> ! {
    todo!("0x49a490 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::hasStringValue(void)const")
}

// 0x49a494 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14getStringValueEPKNS0_13DescribedBaseE
pub fn stub_49a494() -> ! {
    todo!("0x49a494 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x49a4b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_49a4b8() -> ! {
    todo!("0x49a4b8 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x49a4f8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_49a4f8() -> ! {
    todo!("0x49a4f8 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x49a518 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_49a518() -> ! {
    todo!("0x49a518 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x49a758 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13getIndexValueEPKNS0_13DescribedBaseE
pub fn stub_49a758() -> ! {
    todo!("0x49a758 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x49a774 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13setIndexValueEPNS0_13DescribedBaseEm
pub fn stub_49a774() -> ! {
    todo!("0x49a774 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x49a7a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12getEnumValueEPKNS0_13DescribedBaseE
pub fn stub_49a7a8() -> ! {
    todo!("0x49a7a8 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x49a7b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12setEnumValueEPNS0_13DescribedBaseEi
pub fn stub_49a7b0() -> ! {
    todo!("0x49a7b0 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x49a7fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11getEnumItemEPKNS0_13DescribedBaseE
pub fn stub_49a7fc() -> ! {
    todo!("0x49a7fc RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x49a81c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
pub fn stub_49a81c() -> ! {
    todo!("0x49a81c RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x49a850 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToIndex(RBX::DialogRoot::DialogPurpose)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_
pub fn stub_49a850() -> ! {
    todo!("0x49a850 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToIndex(RBX::DialogRoot::DialogPurpose)const")
}

// 0x49a8c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11setIntValueEPNS0_13DescribedBaseEi
pub fn stub_49a8c0() -> ! {
    todo!("0x49a8c0 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x49a900 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_49a900() -> ! {
    todo!("0x49a900 RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isReadOnly(void)const")
}

// 0x49a904 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_49a904() -> ! {
    todo!("0x49a904 RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isWriteOnly(void)const")
}

// 0x49a908 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_49a908() -> ! {
    todo!("0x49a908 RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x49a928 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::setValue(RBX::Reflection::DescribedBase *,RBX::DialogRoot::DialogPurpose const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_49a928() -> ! {
    todo!("0x49a928 RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::setValue(RBX::Reflection::DescribedBase *,RBX::DialogRoot::DialogPurpose const&)const")
}

// 0x49a94c — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::PropDescriptor<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>(char const*,char const*,std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_49a94c() -> ! {
    todo!("0x49a94c RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::PropDescriptor<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>(char const*,char const*,std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x49aa60 — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsED0Ev
pub fn stub_49aa60() -> ! {
    todo!("0x49aa60 RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::~PropDescriptor()")
}

// 0x49aa8c — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
pub fn stub_49aa8c() -> ! {
    todo!("0x49aa8c RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isReadOnly(void)const")
}

// 0x49aa90 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
pub fn stub_49aa90() -> ! {
    todo!("0x49aa90 RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isWriteOnly(void)const")
}

// 0x49aa94 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_49aa94() -> ! {
    todo!("0x49aa94 RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x49aabc — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
pub fn stub_49aabc() -> ! {
    todo!("0x49aabc RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x49ac00 — __ZN3RBX10DialogRootD2Ev
// type: void __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "RBX::DialogRoot::~DialogRoot()")]
// was: __ZN3RBX10DialogRootD2Ev
pub fn stub_49ac00() -> ! {
    todo!("0x49ac00 RBX::DialogRoot::~DialogRoot()")
}

// 0x49ad94 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EED2Ev
pub fn stub_49ad94() -> ! {
    todo!("0x49ad94 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~remote_signal()")
}

// 0x49aee0 — __GLOBAL__I_a_185
#[doc(alias = "global constructor keyed to_a_185")]
// was: __GLOBAL__I_a_185
pub fn stub_49aee0() -> ! {
    todo!("0x49aee0 `global constructor keyed to'_a_185")
}

// 0x49b3e0 — __ZN3RBX6EffectC2Ev
// type: _DWORD __fastcall(RBX::Effect *__hidden this)
#[doc(alias = "RBX::Effect::Effect(void)")]
// was: __ZN3RBX6EffectC2Ev
pub fn stub_49b3e0() -> ! {
    todo!("0x49b3e0 RBX::Effect::Effect(void)")
}

// 0x49b3f0 — __ZN3RBX6EffectD0Ev
// type: void __fastcall(RBX::Effect *__hidden this)
#[doc(alias = "RBX::Effect::~Effect()")]
// was: __ZN3RBX6EffectD0Ev
pub fn stub_49b3f0() -> ! {
    todo!("0x49b3f0 RBX::Effect::~Effect()")
}

// 0x49b3f4 — __ZN3RBX6EffectD1Ev
// type: void __fastcall(RBX::Effect *__hidden this)
#[doc(alias = "RBX::Effect::~Effect()")]
// was: __ZN3RBX6EffectD1Ev
pub fn stub_49b3f4() -> ! {
    todo!("0x49b3f4 RBX::Effect::~Effect()")
}

// 0x49b3f8 — __ZN3RBX6EffectD2Ev
// type: void __fastcall(RBX::Effect *__hidden this)
#[doc(alias = "RBX::Effect::~Effect()")]
// was: __ZN3RBX6EffectD2Ev
pub fn stub_49b3f8() -> ! {
    todo!("0x49b3f8 RBX::Effect::~Effect()")
}

// 0x49b3fc — __GLOBAL__I_a_186
#[doc(alias = "global constructor keyed to_a_186")]
// was: __GLOBAL__I_a_186
pub fn stub_49b3fc() -> ! {
    todo!("0x49b3fc `global constructor keyed to'_a_186")
}

// 0x49b52c — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEC1Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEC1Ev
pub fn stub_49b52c() -> ! {
    todo!("0x49b52c RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::EnumDesc(void)")
}

// 0x49b530 — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEC2Ev
pub fn stub_49b530() -> ! {
    todo!("0x49b530 RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::EnumDesc(void)")
}

// 0x49b708 — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEC1Ev
pub fn stub_49b708() -> ! {
    todo!("0x49b708 RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::EnumDesc(void)")
}

// 0x49b70c — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEC2Ev
pub fn stub_49b70c() -> ! {
    todo!("0x49b70c RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::EnumDesc(void)")
}

// 0x49b964 — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEEC1Ev
pub fn stub_49b964() -> ! {
    todo!("0x49b964 RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::EnumDesc(void)")
}

// 0x49b968 — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEEC2Ev
pub fn stub_49b968() -> ! {
    todo!("0x49b968 RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::EnumDesc(void)")
}

// 0x49bb84 — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEEC1Ev
pub fn stub_49bb84() -> ! {
    todo!("0x49bb84 RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::EnumDesc(void)")
}

// 0x49bb88 — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEEC2Ev
pub fn stub_49bb88() -> ! {
    todo!("0x49bb88 RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::EnumDesc(void)")
}

// 0x49bdbc — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC1Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC1Ev
pub fn stub_49bdbc() -> ! {
    todo!("0x49bdbc RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)")
}

// 0x49bdc0 — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC2Ev
pub fn stub_49bdc0() -> ! {
    todo!("0x49bdc0 RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)")
}

// 0x49bf80 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC1Ev
pub fn stub_49bf80() -> ! {
    todo!("0x49bf80 RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)")
}

// 0x49bf84 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC2Ev
pub fn stub_49bf84() -> ! {
    todo!("0x49bf84 RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)")
}

// 0x49c15c — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE7addPairES3_PKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::addPair(RBX::BasicPartInstance::LegacyPartType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE7addPairES3_PKc
pub fn stub_49c15c() -> ! {
    todo!("0x49c15c RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::addPair(RBX::BasicPartInstance::LegacyPartType,char const*)")
}

// 0x49c4bc — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::addPair(RBX::ExtrudedPartInstance::VisualTrussStyle,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE7addPairES3_PKc
pub fn stub_49c4bc() -> ! {
    todo!("0x49c4bc RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::addPair(RBX::ExtrudedPartInstance::VisualTrussStyle,char const*)")
}

// 0x49c81c — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::addPair(RBX::PrismInstance::NumSidesEnum,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE7addPairES3_PKc
pub fn stub_49c81c() -> ! {
    todo!("0x49c81c RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::addPair(RBX::PrismInstance::NumSidesEnum,char const*)")
}

// 0x49cb7c — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::addPair(RBX::PyramidInstance::NumSidesEnum,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE7addPairES3_PKc
pub fn stub_49cb7c() -> ! {
    todo!("0x49cb7c RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::addPair(RBX::PyramidInstance::NumSidesEnum,char const*)")
}

// 0x49cedc — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::addPair(RBX::Handles::VisualStyle,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE7addPairES3_PKc
pub fn stub_49cedc() -> ! {
    todo!("0x49cedc RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::addPair(RBX::Handles::VisualStyle,char const*)")
}

// 0x49d23c — __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::addPair(RBX::GuiObject::SizeConstraint,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE7addPairES3_PKc
pub fn stub_49d23c() -> ! {
    todo!("0x49d23c RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::addPair(RBX::GuiObject::SizeConstraint,char const*)")
}

// 0x49d59c — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::resize(unsigned long,RBX::GuiObject::SizeConstraint)")]
// was: __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE6resizeEmS2_
pub fn stub_49d59c() -> ! {
    todo!("0x49d59c std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::resize(unsigned long,RBX::GuiObject::SizeConstraint)")
}

// 0x49d5d0 — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::push_back(RBX::GuiObject::SizeConstraint const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE9push_backERKS2_
pub fn stub_49d5d0() -> ! {
    todo!("0x49d5d0 std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::push_back(RBX::GuiObject::SizeConstraint const&)")
}

// 0x49d5f8 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject14SizeConstraintESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::SizeConstraint,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject14SizeConstraintESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_49d5f8() -> ! {
    todo!("0x49d5f8 std::map<RBX::Name const*,RBX::GuiObject::SizeConstraint,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::operator[](RBX::Name const* const&)")
}

// 0x49d650 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_49d650() -> ! {
    todo!("0x49d650 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")
}

// 0x49d704 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_49d704() -> ! {
    todo!("0x49d704 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")
}

// 0x49d75c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_49d75c() -> ! {
    todo!("0x49d75c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")
}

// 0x49d7c4 — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,RBX::GuiObject::SizeConstraint const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_49d7c4() -> ! {
    todo!("0x49d7c4 std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,RBX::GuiObject::SizeConstraint const&)")
}

// 0x49d8a8 — __ZNSt12_Vector_baseIN3RBX9GuiObject14SizeConstraintESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX9GuiObject14SizeConstraintESaIS2_EE11_M_allocateEm
pub fn stub_49d8a8() -> ! {
    todo!("0x49d8a8 std::_Vector_base<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_allocate(unsigned long)")
}

// 0x49d8c0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject14SizeConstraintES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::GuiObject::SizeConstraint * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *>(RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject14SizeConstraintES6_EET0_T_S8_S7_
pub fn stub_49d8c0() -> ! {
    todo!("0x49d8c0 RBX::GuiObject::SizeConstraint * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *>(RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *)")
}

// 0x49d8fc — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,unsigned long,RBX::GuiObject::SizeConstraint const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_49d8fc() -> ! {
    todo!("0x49d8fc std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,unsigned long,RBX::GuiObject::SizeConstraint const&)")
}

// 0x49da8c — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::resize(unsigned long,RBX::Handles::VisualStyle)")]
// was: __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE6resizeEmS2_
pub fn stub_49da8c() -> ! {
    todo!("0x49da8c std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::resize(unsigned long,RBX::Handles::VisualStyle)")
}

// 0x49dac0 — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::push_back(RBX::Handles::VisualStyle const&)")]
// was: __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE9push_backERKS2_
pub fn stub_49dac0() -> ! {
    todo!("0x49dac0 std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::push_back(RBX::Handles::VisualStyle const&)")
}

// 0x49dae8 — __ZNSt3mapIPKN3RBX4NameENS0_7Handles11VisualStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::Handles::VisualStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_7Handles11VisualStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_49dae8() -> ! {
    todo!("0x49dae8 std::map<RBX::Name const*,RBX::Handles::VisualStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::operator[](RBX::Name const* const&)")
}

// 0x49db40 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_49db40() -> ! {
    todo!("0x49db40 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")
}

// 0x49dbf4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_49dbf4() -> ! {
    todo!("0x49dbf4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")
}

// 0x49dc4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_49dc4c() -> ! {
    todo!("0x49dc4c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")
}

// 0x49dcb4 — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,RBX::Handles::VisualStyle const&)")]
// was: __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_49dcb4() -> ! {
    todo!("0x49dcb4 std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,RBX::Handles::VisualStyle const&)")
}

// 0x49dd98 — __ZNSt12_Vector_baseIN3RBX7Handles11VisualStyleESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX7Handles11VisualStyleESaIS2_EE11_M_allocateEm
pub fn stub_49dd98() -> ! {
    todo!("0x49dd98 std::_Vector_base<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_allocate(unsigned long)")
}

// 0x49ddb0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Handles11VisualStyleES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::Handles::VisualStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *>(RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Handles11VisualStyleES6_EET0_T_S8_S7_
pub fn stub_49ddb0() -> ! {
    todo!("0x49ddb0 RBX::Handles::VisualStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *>(RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *)")
}

// 0x49ddec — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,unsigned long,RBX::Handles::VisualStyle const&)")]
// was: __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_49ddec() -> ! {
    todo!("0x49ddec std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,unsigned long,RBX::Handles::VisualStyle const&)")
}

// 0x49df7c — __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::resize(unsigned long,RBX::PyramidInstance::NumSidesEnum)")]
// was: __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE6resizeEmS2_
pub fn stub_49df7c() -> ! {
    todo!("0x49df7c std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::resize(unsigned long,RBX::PyramidInstance::NumSidesEnum)")
}

// 0x49dfb0 — __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::push_back(RBX::PyramidInstance::NumSidesEnum const&)")]
// was: __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE9push_backERKS2_
pub fn stub_49dfb0() -> ! {
    todo!("0x49dfb0 std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::push_back(RBX::PyramidInstance::NumSidesEnum const&)")
}

// 0x49dfd8 — __ZNSt3mapIPKN3RBX4NameENS0_15PyramidInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::PyramidInstance::NumSidesEnum,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_15PyramidInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_49dfd8() -> ! {
    todo!("0x49dfd8 std::map<RBX::Name const*,RBX::PyramidInstance::NumSidesEnum,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::operator[](RBX::Name const* const&)")
}

// 0x49e030 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_49e030() -> ! {
    todo!("0x49e030 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum> const&)")
}

// 0x49e0e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_49e0e4() -> ! {
    todo!("0x49e0e4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum> const&)")
}

// 0x49e13c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_49e13c() -> ! {
    todo!("0x49e13c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum> const&)")
}

// 0x49e1a4 — __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PyramidInstance::NumSidesEnum*,std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>>,RBX::PyramidInstance::NumSidesEnum const&)")]
// was: __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_49e1a4() -> ! {
    todo!("0x49e1a4 std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PyramidInstance::NumSidesEnum*,std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>>,RBX::PyramidInstance::NumSidesEnum const&)")
}

// 0x49e288 — __ZNSt12_Vector_baseIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE11_M_allocateEm
pub fn stub_49e288() -> ! {
    todo!("0x49e288 std::_Vector_base<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::_M_allocate(unsigned long)")
}

// 0x49e2a0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PyramidInstance12NumSidesEnumES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::PyramidInstance::NumSidesEnum * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PyramidInstance::NumSidesEnum *,RBX::PyramidInstance::NumSidesEnum *>(RBX::PyramidInstance::NumSidesEnum *,RBX::PyramidInstance::NumSidesEnum *,RBX::PyramidInstance::NumSidesEnum *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PyramidInstance12NumSidesEnumES6_EET0_T_S8_S7_
pub fn stub_49e2a0() -> ! {
    todo!("0x49e2a0 RBX::PyramidInstance::NumSidesEnum * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PyramidInstance::NumSidesEnum *,RBX::PyramidInstance::NumSidesEnum *>(RBX::PyramidInstance::NumSidesEnum *,RBX::PyramidInstance::NumSidesEnum *,RBX::PyramidInstance::NumSidesEnum *)")
}

// 0x49e2dc — __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PyramidInstance::NumSidesEnum*,std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>>,unsigned long,RBX::PyramidInstance::NumSidesEnum const&)")]
// was: __ZNSt6vectorIN3RBX15PyramidInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_49e2dc() -> ! {
    todo!("0x49e2dc std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PyramidInstance::NumSidesEnum*,std::vector<RBX::PyramidInstance::NumSidesEnum,std::allocator<RBX::PyramidInstance::NumSidesEnum>>>,unsigned long,RBX::PyramidInstance::NumSidesEnum const&)")
}

// 0x49e46c — __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::resize(unsigned long,RBX::PrismInstance::NumSidesEnum)")]
// was: __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE6resizeEmS2_
pub fn stub_49e46c() -> ! {
    todo!("0x49e46c std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::resize(unsigned long,RBX::PrismInstance::NumSidesEnum)")
}

// 0x49e4a0 — __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::push_back(RBX::PrismInstance::NumSidesEnum const&)")]
// was: __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE9push_backERKS2_
pub fn stub_49e4a0() -> ! {
    todo!("0x49e4a0 std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::push_back(RBX::PrismInstance::NumSidesEnum const&)")
}

// 0x49e4c8 — __ZNSt3mapIPKN3RBX4NameENS0_13PrismInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::PrismInstance::NumSidesEnum,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_13PrismInstance12NumSidesEnumESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_49e4c8() -> ! {
    todo!("0x49e4c8 std::map<RBX::Name const*,RBX::PrismInstance::NumSidesEnum,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::operator[](RBX::Name const* const&)")
}

// 0x49e520 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_49e520() -> ! {
    todo!("0x49e520 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum> const&)")
}

// 0x49e5d4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_49e5d4() -> ! {
    todo!("0x49e5d4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum> const&)")
}

// 0x49e62c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_49e62c() -> ! {
    todo!("0x49e62c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum> const&)")
}

// 0x49e694 — __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PrismInstance::NumSidesEnum*,std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>>,RBX::PrismInstance::NumSidesEnum const&)")]
// was: __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_49e694() -> ! {
    todo!("0x49e694 std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PrismInstance::NumSidesEnum*,std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>>,RBX::PrismInstance::NumSidesEnum const&)")
}

// 0x49e778 — __ZNSt12_Vector_baseIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE11_M_allocateEm
pub fn stub_49e778() -> ! {
    todo!("0x49e778 std::_Vector_base<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::_M_allocate(unsigned long)")
}

// 0x49e790 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13PrismInstance12NumSidesEnumES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::PrismInstance::NumSidesEnum * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PrismInstance::NumSidesEnum *,RBX::PrismInstance::NumSidesEnum *>(RBX::PrismInstance::NumSidesEnum *,RBX::PrismInstance::NumSidesEnum *,RBX::PrismInstance::NumSidesEnum *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13PrismInstance12NumSidesEnumES6_EET0_T_S8_S7_
pub fn stub_49e790() -> ! {
    todo!("0x49e790 RBX::PrismInstance::NumSidesEnum * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PrismInstance::NumSidesEnum *,RBX::PrismInstance::NumSidesEnum *>(RBX::PrismInstance::NumSidesEnum *,RBX::PrismInstance::NumSidesEnum *,RBX::PrismInstance::NumSidesEnum *)")
}

// 0x49e7cc — __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PrismInstance::NumSidesEnum*,std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>>,unsigned long,RBX::PrismInstance::NumSidesEnum const&)")]
// was: __ZNSt6vectorIN3RBX13PrismInstance12NumSidesEnumESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_49e7cc() -> ! {
    todo!("0x49e7cc std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PrismInstance::NumSidesEnum*,std::vector<RBX::PrismInstance::NumSidesEnum,std::allocator<RBX::PrismInstance::NumSidesEnum>>>,unsigned long,RBX::PrismInstance::NumSidesEnum const&)")
}

// 0x49e95c — __ZNSt3mapIPKN3RBX4NameENS0_20ExtrudedPartInstance16VisualTrussStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::ExtrudedPartInstance::VisualTrussStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_20ExtrudedPartInstance16VisualTrussStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_49e95c() -> ! {
    todo!("0x49e95c std::map<RBX::Name const*,RBX::ExtrudedPartInstance::VisualTrussStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::operator[](RBX::Name const* const&)")
}

// 0x49e9b4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_49e9b4() -> ! {
    todo!("0x49e9b4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle> const&)")
}

// 0x49ea68 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_49ea68() -> ! {
    todo!("0x49ea68 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle> const&)")
}

// 0x49eac0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_49eac0() -> ! {
    todo!("0x49eac0 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle> const&)")
}
