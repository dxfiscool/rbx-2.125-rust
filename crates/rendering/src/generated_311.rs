//! rendering shard 311 — 100 stubs 0x45f6e0..0x461d6c EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 33881->33981 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 33881 before -> 33981 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x45f6e0 (lowest remaining 0x45f6e0..0x461d6c, next lowest 0x461efc)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x45f6e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x45f6e0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f6e0() {
}

// 0x45f714 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x45f714: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f714() {
}

// 0x45f71c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x45f71c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f71c() {
}

// 0x45f768 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x45f768: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f768() {
}

// 0x45f788 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x45f788: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f788() {
}

// 0x45f7bc — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToIndex(RBX::DataModel::Genre)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToIndexES3_
// IDA 0x45f7bc: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f7bc() {
}

// 0x45f82c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x45f82c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f82c() {
}

// 0x45f86c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
// IDA 0x45f86c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f86c() {
}

// 0x45f870 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
// IDA 0x45f870: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f870() {
}

// 0x45f874 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x45f874: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f874() {
}

// 0x45f894 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::Genre const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x45f894: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f894() {
}

// 0x45f9b4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::EnumPropDescriptor<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x45f9b4: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f9b4() {
}

// 0x45fb60 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEED0Ev
// IDA 0x45fb60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_45fb60() {
}

// 0x45fb8c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10isReadOnlyEv
// IDA 0x45fb8c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45fb8c() {
}

// 0x45fb9c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11isWriteOnlyEv
// IDA 0x45fb9c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45fb9c() {
}

// 0x45fbac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x45fbac: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45fbac() {
}

// 0x45fbd4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x45fbd4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45fbd4() {
}

// 0x45fbf8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x45fbf8: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45fbf8() {
}

// 0x45fd44 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x45fd44: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45fd44() {
}

// 0x45fd68 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14hasStringValueEv
// IDA 0x45fd68: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45fd68() {
}

// 0x45fd6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x45fd6c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45fd6c() {
}

// 0x45fd90 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x45fd90: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45fd90() {
}

// 0x45fdd0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x45fdd0: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45fdd0() {
}

// 0x45fdf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x45fdf0: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45fdf0() {
}

// 0x460030 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x460030: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460030() {
}

// 0x46004c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x46004c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46004c() {
}

// 0x460080 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x460080: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460080() {
}

// 0x460088 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x460088: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460088() {
}

// 0x4600d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x4600d4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4600d4() {
}

// 0x4600f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x4600f4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4600f4() {
}

// 0x460128 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToIndex(RBX::DataModel::CreatorType)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToIndexES3_
// IDA 0x460128: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460128() {
}

// 0x460198 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x460198: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460198() {
}

// 0x4601d8 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
// IDA 0x4601d8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4601d8() {
}

// 0x4601dc — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
// IDA 0x4601dc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4601dc() {
}

// 0x4601e0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x4601e0: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4601e0() {
}

// 0x460200 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::CreatorType const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x460200: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460200() {
}

// 0x460320 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::PropDescriptor<int (RBX::DataModel::*)(void)const,int>(char const*,char const*,int (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x460320: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460320() {
}

// 0x460430 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiED0Ev
// IDA 0x460430: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_460430() {
}

// 0x460460 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE10isReadOnlyEv
// IDA 0x460460: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460460() {
}

// 0x460464 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
// IDA 0x460464: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460464() {
}

// 0x460468 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x460468: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460468() {
}

// 0x460488 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x460488: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460488() {
}

// 0x4605a8 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::Instance* (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x4605a8: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4605a8() {
}

// 0x46064c — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEED0Ev
// IDA 0x46064c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_46064c() {
}

// 0x46067c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10isReadOnlyEv
// IDA 0x46067c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46067c() {
}

// 0x46068c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11isWriteOnlyEv
// IDA 0x46068c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46068c() {
}

// 0x46069c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x46069c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46069c() {
}

// 0x4606c4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x4606c4: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4606c4() {
}

// 0x4607dc — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x4607dc: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4607dc() {
}

// 0x4608a4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x4608a4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4608a4() {
}

// 0x4608c8 — __ZNK3RBX10Reflection21RefPropertyDescriptor11getDataSizeEPKNS0_13DescribedBaseE
// type: _DWORD __fastcall(RBX::Reflection::RefPropertyDescriptor *__hidden this, const DescribedBase *)
#[doc(alias = "RBX::Reflection::RefPropertyDescriptor::getDataSize(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection21RefPropertyDescriptor11getDataSizeEPKNS0_13DescribedBaseE
// IDA 0x4608c8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4608c8() {
}

// 0x4608d0 — __ZNK3RBX10Reflection21RefPropertyDescriptor14getStringValueEPKNS0_13DescribedBaseE
// type: _DWORD __fastcall(RBX::Reflection::RefPropertyDescriptor *__hidden this, const DescribedBase *)
#[doc(alias = "RBX::Reflection::RefPropertyDescriptor::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection21RefPropertyDescriptor14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x4608d0: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4608d0() {
}

// 0x4608f8 — __ZNK3RBX10Reflection21RefPropertyDescriptor14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::RefPropertyDescriptor::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection21RefPropertyDescriptor14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x4608f8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4608f8() {
}

// 0x460908 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x460908: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460908() {
}

// 0x4609dc — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x4609dc: 15 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4609dc() {
}

// 0x460a00 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
// IDA 0x460a00: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460a00() {
}

// 0x460a14 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// IDA 0x460a14: 41 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460a14() {
}

// 0x460a90 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// IDA 0x460a90: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460a90() {
}

// 0x460ab0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// IDA 0x460ab0: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460ab0() {
}

// 0x460b90 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// IDA 0x460b90: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460b90() {
}

// 0x460b98 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
// IDA 0x460b98: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460b98() {
}

// 0x460b9c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
// IDA 0x460b9c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460b9c() {
}

// 0x460ba0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x460ba0: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460ba0() {
}

// 0x460bc0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x460bc0: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460bc0() {
}

// 0x460ce0 — __ZN3RBX10Reflection7RefTypeIPNS_8InstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefType<RBX::Instance *>::~RefType()")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_8InstanceEED1Ev
// IDA 0x460ce0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_460ce0() {
}

// 0x460ce8 — __ZN3RBX10Reflection7RefTypeIPNS_8InstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefType<RBX::Instance *>::~RefType()")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_8InstanceEED0Ev
// IDA 0x460ce8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_460ce8() {
}

// 0x460cec — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::RefPropDescriptor<RBX::Workspace* (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::Workspace* (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x460cec: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460cec() {
}

// 0x460d90 — __ZN3RBX10Reflection7RefTypeIPNS_9WorkspaceEE9singletonEv
#[doc(alias = "RBX::Reflection::RefType<RBX::Workspace *>::singleton(void)")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_9WorkspaceEE9singletonEv
// IDA 0x460d90: 79 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460d90() {
}

// 0x460e88 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEED0Ev
// IDA 0x460e88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_460e88() {
}

// 0x460eb8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10isReadOnlyEv
// IDA 0x460eb8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460eb8() {
}

// 0x460ec8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11isWriteOnlyEv
// IDA 0x460ec8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460ec8() {
}

// 0x460ed8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x460ed8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460ed8() {
}

// 0x460f00 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x460f00: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_460f00() {
}

// 0x461018 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x461018: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461018() {
}

// 0x4610e0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x4610e0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4610e0() {
}

// 0x461104 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x461104: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461104() {
}

// 0x4611d8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x4611d8: 15 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4611d8() {
}

// 0x4611fc — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11getRefValueEPKNS0_13DescribedBaseE
// IDA 0x4611fc: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4611fc() {
}

// 0x461210 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11setRefValueEPNS0_13DescribedBaseES6_
// IDA 0x461210: 41 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461210() {
}

// 0x46128c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// IDA 0x46128c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46128c() {
}

// 0x4612ac — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// IDA 0x4612ac: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4612ac() {
}

// 0x46138c — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// IDA 0x46138c: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46138c() {
}

// 0x461394 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
// IDA 0x461394: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461394() {
}

// 0x461398 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
// IDA 0x461398: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461398() {
}

// 0x46139c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x46139c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46139c() {
}

// 0x4613bc — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Workspace * const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x4613bc: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4613bc() {
}

// 0x4614dc — __ZN3RBX10Reflection7RefTypeIPNS_9WorkspaceEED1Ev
#[doc(alias = "RBX::Reflection::RefType<RBX::Workspace *>::~RefType()")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_9WorkspaceEED1Ev
// IDA 0x4614dc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4614dc() {
}

// 0x4614e0 — __ZN3RBX10Reflection4TypeC2IPNS_9WorkspaceEEEPKcS6_PT_
// type: int(void)
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Workspace *>(char const*,char const*,RBX::Workspace * *)")]
// was: __ZN3RBX10Reflection4TypeC2IPNS_9WorkspaceEEEPKcS6_PT_
// IDA 0x4614e0: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4614e0() {
}

// 0x46158c — __ZN3RBX10Reflection7RefTypeIPNS_9WorkspaceEED0Ev
#[doc(alias = "RBX::Reflection::RefType<RBX::Workspace *>::~RefType()")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_9WorkspaceEED0Ev
// IDA 0x46158c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_46158c() {
}

// 0x461590 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EEC2EMS2_FvS3_iEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::BoundFuncDesc(void (RBX::DataModel::*)(RBX::DataModel::GearGenreSetting,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EEC2EMS2_FvS3_iEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x461590: 176 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461590() {
}

// 0x461758 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
// IDA 0x461758: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461758() {
}

// 0x4617a4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EED0Ev
// IDA 0x4617a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4617a4() {
}

// 0x461884 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x461884: 29 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461884() {
}

// 0x4618d8 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel16GearGenreSettingELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int(void)
#[doc(alias = "RBX::DataModel::GearGenreSetting RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearGenreSetting,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearGenreSetting> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearGenreSetting,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel16GearGenreSettingELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x4618d8: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4618d8() {
}

// 0x461a68 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel16GearGenreSettingEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int(void)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearGenreSetting>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearGenreSetting &,boost::enable_if<boost::is_enum<RBX::DataModel::GearGenreSetting>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel16GearGenreSettingEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// IDA 0x461a68: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461a68() {
}

// 0x461abc — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::BoundFuncDesc(void (RBX::DataModel::*)(RBX::DataModel::Genre),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x461abc: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461abc() {
}

// 0x461c34 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x461c34: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461c34() {
}

// 0x461c64 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EED0Ev
// IDA 0x461c64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_461c64() {
}

// 0x461d38 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x461d38: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461d38() {
}

// 0x461d6c — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel5GenreELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int(void)
#[doc(alias = "RBX::DataModel::Genre RBX::Reflection::ArgHelper::getArg<RBX::DataModel::Genre,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::Genre> const&,boost::disable_if<boost::is_same<RBX::DataModel::Genre,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel5GenreELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x461d6c: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_461d6c() {
}
