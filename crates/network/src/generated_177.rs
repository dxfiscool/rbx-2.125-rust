//! network generated_177 — RakNet + RBX::Network + global gap filler (auto-generated, do not edit manually)
//! Filter: RakNet|Network|Replicator|Socket|HTTP -> 6073 funcs, 0 remaining before batch (all covered); broad not yet in network 65675 gaps; global not yet in any 3838; batch EA-sorted asc next 100 global gaps not yet in any
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x49a0d4..0x4a7a94 | existing 19870 -> 19970 total (3738 global remaining, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x49a0d4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::EnumPropDescriptor<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>(char const*,char const*,RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::EnumPropDescriptor<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>(char const*,char const*,RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_49a0d4() -> ! {
    todo!("0x49a0d4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::EnumPropDescriptor<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>(char const*,char const*,RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x49a288 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::~EnumPropDescriptor()")]
pub fn stub_49a288() -> ! {
    todo!("0x49a288 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::~EnumPropDescriptor()")
}

// 0x49a2b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isReadOnly(void)const")]
pub fn stub_49a2b4() -> ! {
    todo!("0x49a2b4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isReadOnly(void)const")
}

// 0x49a2c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isWriteOnly(void)const")]
pub fn stub_49a2c4() -> ! {
    todo!("0x49a2c4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isWriteOnly(void)const")
}

// 0x49a2d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11equalValuesEPKNS0_13DescribedBaseES7_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_49a2d4() -> ! {
    todo!("0x49a2d4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x49a2fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_49a2fc() -> ! {
    todo!("0x49a2fc RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x49a320 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_49a320() -> ! {
    todo!("0x49a320 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x49a46c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_49a46c() -> ! {
    todo!("0x49a46c RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x49a490 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::hasStringValue(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::hasStringValue(void)const")]
pub fn stub_49a490() -> ! {
    todo!("0x49a490 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::hasStringValue(void)const")
}

// 0x49a494 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_49a494() -> ! {
    todo!("0x49a494 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x49a4b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_49a4b8() -> ! {
    todo!("0x49a4b8 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x49a4f8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_49a4f8() -> ! {
    todo!("0x49a4f8 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x49a518 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_49a518() -> ! {
    todo!("0x49a518 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x49a758 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_49a758() -> ! {
    todo!("0x49a758 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x49a774 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_49a774() -> ! {
    todo!("0x49a774 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x49a7a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_49a7a8() -> ! {
    todo!("0x49a7a8 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x49a7b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_49a7b0() -> ! {
    todo!("0x49a7b0 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x49a7fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_49a7fc() -> ! {
    todo!("0x49a7fc RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x49a81c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_49a81c() -> ! {
    todo!("0x49a81c RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x49a850 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToIndex(RBX::DialogRoot::DialogPurpose)const
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToIndex(RBX::DialogRoot::DialogPurpose)const")]
pub fn stub_49a850() -> ! {
    todo!("0x49a850 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToIndex(RBX::DialogRoot::DialogPurpose)const")
}

// 0x49a8c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_49a8c0() -> ! {
    todo!("0x49a8c0 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x49a900 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isReadOnly(void)const")]
pub fn stub_49a900() -> ! {
    todo!("0x49a900 RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isReadOnly(void)const")
}

// 0x49a904 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isWriteOnly(void)const")]
pub fn stub_49a904() -> ! {
    todo!("0x49a904 RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isWriteOnly(void)const")
}

// 0x49a908 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_49a908() -> ! {
    todo!("0x49a908 RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x49a928 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// demangled: RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::setValue(RBX::Reflection::DescribedBase *,RBX::DialogRoot::DialogPurpose const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::setValue(RBX::Reflection::DescribedBase *,RBX::DialogRoot::DialogPurpose const&)const")]
pub fn stub_49a928() -> ! {
    todo!("0x49a928 RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::setValue(RBX::Reflection::DescribedBase *,RBX::DialogRoot::DialogPurpose const&)const")
}

// 0x49a94c — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::PropDescriptor<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>(char const*,char const*,std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::PropDescriptor<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>(char const*,char const*,std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_49a94c() -> ! {
    todo!("0x49a94c RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::PropDescriptor<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>(char const*,char const*,std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x49aa60 — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::~PropDescriptor()")]
pub fn stub_49aa60() -> ! {
    todo!("0x49aa60 RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::~PropDescriptor()")
}

// 0x49aa8c — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isReadOnly(void)const")]
pub fn stub_49aa8c() -> ! {
    todo!("0x49aa8c RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isReadOnly(void)const")
}

// 0x49aa90 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isWriteOnly(void)const")]
pub fn stub_49aa90() -> ! {
    todo!("0x49aa90 RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isWriteOnly(void)const")
}

// 0x49aa94 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_49aa94() -> ! {
    todo!("0x49aa94 RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x49aabc — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_49aabc() -> ! {
    todo!("0x49aabc RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x49aee0 — __GLOBAL__I_a_185
// demangled: global constructor keyed to'_a_185
#[doc(alias = "global constructor keyed to_a_185")]
pub fn stub_49aee0() -> ! {
    todo!("0x49aee0 global constructor keyed to_a_185")
}

// 0x49b3fc — __GLOBAL__I_a_186
// demangled: global constructor keyed to'_a_186
#[doc(alias = "global constructor keyed to_a_186")]
pub fn stub_49b3fc() -> ! {
    todo!("0x49b3fc global constructor keyed to_a_186")
}

// 0x49bdbc — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)")]
pub fn stub_49bdbc() -> ! {
    todo!("0x49bdbc RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)")
}

// 0x49bdc0 — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)")]
pub fn stub_49bdc0() -> ! {
    todo!("0x49bdc0 RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)")
}

// 0x49bf80 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)")]
pub fn stub_49bf80() -> ! {
    todo!("0x49bf80 RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)")
}

// 0x49bf84 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)")]
pub fn stub_49bf84() -> ! {
    todo!("0x49bf84 RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)")
}

// 0x49cedc — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE7addPairES3_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::addPair(RBX::Handles::VisualStyle,char const*)
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::addPair(RBX::Handles::VisualStyle,char const*)")]
pub fn stub_49cedc() -> ! {
    todo!("0x49cedc RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::addPair(RBX::Handles::VisualStyle,char const*)")
}

// 0x49d23c — __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE7addPairES3_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::addPair(RBX::GuiObject::SizeConstraint,char const*)
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::addPair(RBX::GuiObject::SizeConstraint,char const*)")]
pub fn stub_49d23c() -> ! {
    todo!("0x49d23c RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::addPair(RBX::GuiObject::SizeConstraint,char const*)")
}

// 0x49f33c — __GLOBAL__I_a_187
// demangled: global constructor keyed to'_a_187
#[doc(alias = "global constructor keyed to_a_187")]
pub fn stub_49f33c() -> ! {
    todo!("0x49f33c global constructor keyed to_a_187")
}

// 0x49f610 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::EnumDesc(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::EnumDesc(void)")]
pub fn stub_49f610() -> ! {
    todo!("0x49f610 RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::EnumDesc(void)")
}

// 0x49f614 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::EnumDesc(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::EnumDesc(void)")]
pub fn stub_49f614() -> ! {
    todo!("0x49f614 RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::EnumDesc(void)")
}

// 0x4a0440 — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Explosion,float>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::~PropDescriptor()")]
pub fn stub_4a0440() -> ! {
    todo!("0x4a0440 RBX::Reflection::PropDescriptor<RBX::Explosion,float>::~PropDescriptor()")
}

// 0x4a0494 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::~EnumPropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::~EnumPropDescriptor()")]
pub fn stub_4a0494() -> ! {
    todo!("0x4a0494 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::~EnumPropDescriptor()")
}

// 0x4a04dc — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE7addPairES3_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::addPair(RBX::Explosion::ExplosionType,char const*)
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::addPair(RBX::Explosion::ExplosionType,char const*)")]
pub fn stub_4a04dc() -> ! {
    todo!("0x4a04dc RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::addPair(RBX::Explosion::ExplosionType,char const*)")
}

// 0x4a1380 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()")]
pub fn stub_4a1380() -> ! {
    todo!("0x4a1380 RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()")
}

// 0x4a1388 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE15convertToStringEmRSs
// demangled: RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToString(unsigned long,std::string &)const")]
pub fn stub_4a1388() -> ! {
    todo!("0x4a1388 RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToString(unsigned long,std::string &)const")
}

// 0x4a14e0 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE13convertToItemERKS3_
// demangled: RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToItem(RBX::Explosion::ExplosionType const&)const
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToItem(RBX::Explosion::ExplosionType const&)const")]
pub fn stub_4a14e0() -> ! {
    todo!("0x4a14e0 RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToItem(RBX::Explosion::ExplosionType const&)const")
}

// 0x4a15b0 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()")]
pub fn stub_4a15b0() -> ! {
    todo!("0x4a15b0 RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()")
}

// 0x4a18d0 — __ZN3RBX4Name13callDoDeclareILZNS_11sForceFieldEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sForceFieldEEEEvv")]
pub fn stub_4a18d0() -> ! {
    todo!("0x4a18d0 __ZN3RBX4Name13callDoDeclareILZNS_11sForceFieldEEEEvv")
}

// 0x4a1ba0 — __ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator12getClassNameEv")]
pub fn stub_4a1ba0() -> ! {
    todo!("0x4a1ba0 __ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator12getClassNameEv")
}

// 0x4a1c28 — __ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator6createEv
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator6createEv")]
pub fn stub_4a1c28() -> ! {
    todo!("0x4a1c28 __ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator6createEv")
}

// 0x4a1d78 — __ZN3RBX4Name13callDoDeclareILZNS_10sExplosionEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sExplosionEEEEvv")]
pub fn stub_4a1d78() -> ! {
    todo!("0x4a1d78 __ZN3RBX4Name13callDoDeclareILZNS_10sExplosionEEEEvv")
}

// 0x4a1d7c — __ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v")]
pub fn stub_4a1d7c() -> ! {
    todo!("0x4a1d7c __ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v")
}

// 0x4a1e5c — __ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorC2Ev")]
pub fn stub_4a1e5c() -> ! {
    todo!("0x4a1e5c __ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorC2Ev")
}

// 0x4a20a0 — __ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E17static_getCreatorEv")]
pub fn stub_4a20a0() -> ! {
    todo!("0x4a20a0 __ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E17static_getCreatorEv")
}

// 0x4a2900 — __ZNK3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7Creator12getClassNameEv")]
pub fn stub_4a2900() -> ! {
    todo!("0x4a2900 __ZNK3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7Creator12getClassNameEv")
}

// 0x4a2bd0 — __ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E15isNullClassNameEv")]
pub fn stub_4a2bd0() -> ! {
    todo!("0x4a2bd0 __ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E15isNullClassNameEv")
}

// 0x4a5834 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::EnumPropDescriptor<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>(char const*,char const*,RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::EnumPropDescriptor<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>(char const*,char const*,RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_4a5834() -> ! {
    todo!("0x4a5834 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::EnumPropDescriptor<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>(char const*,char const*,RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x4a59e8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::~EnumPropDescriptor()")]
pub fn stub_4a59e8() -> ! {
    todo!("0x4a59e8 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::~EnumPropDescriptor()")
}

// 0x4a5a14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isReadOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isReadOnly(void)const")]
pub fn stub_4a5a14() -> ! {
    todo!("0x4a5a14 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isReadOnly(void)const")
}

// 0x4a5a24 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isWriteOnly(void)const")]
pub fn stub_4a5a24() -> ! {
    todo!("0x4a5a24 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isWriteOnly(void)const")
}

// 0x4a5a34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_4a5a34() -> ! {
    todo!("0x4a5a34 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x4a5a5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_4a5a5c() -> ! {
    todo!("0x4a5a5c RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x4a5a80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_4a5a80() -> ! {
    todo!("0x4a5a80 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x4a5bcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_4a5bcc() -> ! {
    todo!("0x4a5bcc RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x4a5bf4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::hasStringValue(void)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::hasStringValue(void)const")]
pub fn stub_4a5bf4() -> ! {
    todo!("0x4a5bf4 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::hasStringValue(void)const")
}

// 0x4a5bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_4a5bf8() -> ! {
    todo!("0x4a5bf8 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4a5c1c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_4a5c1c() -> ! {
    todo!("0x4a5c1c RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x4a5c5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_4a5c5c() -> ! {
    todo!("0x4a5c5c RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x4a5c7c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_4a5c7c() -> ! {
    todo!("0x4a5c7c RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x4a5ebc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_4a5ebc() -> ! {
    todo!("0x4a5ebc RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4a5ed8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_4a5ed8() -> ! {
    todo!("0x4a5ed8 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x4a5f0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_4a5f0c() -> ! {
    todo!("0x4a5f0c RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4a5f14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_4a5f14() -> ! {
    todo!("0x4a5f14 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x4a5f60 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_4a5f60() -> ! {
    todo!("0x4a5f60 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x4a5f80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_4a5f80() -> ! {
    todo!("0x4a5f80 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x4a5fb8 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToIndex(RBX::Explosion::ExplosionType)const
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToIndex(RBX::Explosion::ExplosionType)const")]
pub fn stub_4a5fb8() -> ! {
    todo!("0x4a5fb8 RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToIndex(RBX::Explosion::ExplosionType)const")
}

// 0x4a6028 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_4a6028() -> ! {
    todo!("0x4a6028 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x4a606c — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isReadOnly(void)const")]
pub fn stub_4a606c() -> ! {
    todo!("0x4a606c RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isReadOnly(void)const")
}

// 0x4a6070 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isWriteOnly(void)const")]
pub fn stub_4a6070() -> ! {
    todo!("0x4a6070 RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isWriteOnly(void)const")
}

// 0x4a6074 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_4a6074() -> ! {
    todo!("0x4a6074 RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4a6094 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// demangled: RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::setValue(RBX::Reflection::DescribedBase *,RBX::Explosion::ExplosionType const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::setValue(RBX::Reflection::DescribedBase *,RBX::Explosion::ExplosionType const&)const")]
pub fn stub_4a6094() -> ! {
    todo!("0x4a6094 RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::setValue(RBX::Reflection::DescribedBase *,RBX::Explosion::ExplosionType const&)const")
}

// 0x4a60b8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9Explosion13ExplosionTypeEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType> const>::initSingleton(void)")]
pub fn stub_4a60b8() -> ! {
    todo!("0x4a60b8 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType> const>::initSingleton(void)")
}

// 0x4a64ac — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,float RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,float RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_4a64ac() -> ! {
    todo!("0x4a64ac RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,float RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x4a6640 — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EED0Ev
// demangled: RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()")]
pub fn stub_4a6640() -> ! {
    todo!("0x4a6640 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()")
}

// 0x4a666c — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv
// demangled: RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isReadOnly(void)const")]
pub fn stub_4a666c() -> ! {
    todo!("0x4a666c RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isReadOnly(void)const")
}

// 0x4a6670 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv
// demangled: RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isWriteOnly(void)const")]
pub fn stub_4a6670() -> ! {
    todo!("0x4a6670 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isWriteOnly(void)const")
}

// 0x4a6674 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_4a6674() -> ! {
    todo!("0x4a6674 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4a6680 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKf
// demangled: RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::setValue(RBX::Reflection::DescribedBase *,float const&)const
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_4a6680() -> ! {
    todo!("0x4a6680 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x4a66dc — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Explosion,float>::PropDescriptor<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>(char const*,char const*,float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::PropDescriptor<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>(char const*,char const*,float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_4a66dc() -> ! {
    todo!("0x4a66dc RBX::Reflection::PropDescriptor<RBX::Explosion,float>::PropDescriptor<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>(char const*,char const*,float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x4a67f0 — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Explosion,float>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::~PropDescriptor()")]
pub fn stub_4a67f0() -> ! {
    todo!("0x4a67f0 RBX::Reflection::PropDescriptor<RBX::Explosion,float>::~PropDescriptor()")
}

// 0x4a681c — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isReadOnly(void)const")]
pub fn stub_4a681c() -> ! {
    todo!("0x4a681c RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isReadOnly(void)const")
}

// 0x4a6820 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isWriteOnly(void)const")]
pub fn stub_4a6820() -> ! {
    todo!("0x4a6820 RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isWriteOnly(void)const")
}

// 0x4a6824 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_4a6824() -> ! {
    todo!("0x4a6824 RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4a6844 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// demangled: RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_4a6844() -> ! {
    todo!("0x4a6844 RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x4a6898 — __GLOBAL__I_a_188
// demangled: global constructor keyed to'_a_188
#[doc(alias = "global constructor keyed to_a_188")]
pub fn stub_4a6898() -> ! {
    todo!("0x4a6898 global constructor keyed to_a_188")
}

// 0x4a7758 — __ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv")]
pub fn stub_4a7758() -> ! {
    todo!("0x4a7758 __ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv")
}

// 0x4a776c — __ZThn32_NK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv")]
pub fn stub_4a776c() -> ! {
    todo!("0x4a776c __ZThn32_NK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv")
}

// 0x4a7a94 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
pub fn stub_4a7a94() -> ! {
    todo!("0x4a7a94 __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")
}
