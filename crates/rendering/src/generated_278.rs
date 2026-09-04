//! rendering shard 278 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15112/15112 complete, 30320->30420 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 30320 before -> 30420 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x3bbc68 — __ZN3RBX14FormFactorPartD1Ev
// type: void __fastcall(RBX::FormFactorPart *__hidden this)
#[doc(alias = "RBX::FormFactorPart::~FormFactorPart()")]
// was: __ZN3RBX14FormFactorPartD1Ev
// IDA 0x3bbc68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bbc68() {
}


// 0x3bbc78 — __ZThn32_N3RBX14FormFactorPartD0Ev
// type: void __fastcall(RBX::FormFactorPart *__hidden this)
#[doc(alias = "__ZThn32_N3RBX14FormFactorPartD0Ev")]
// was: __ZThn32_N3RBX14FormFactorPartD0Ev
// IDA 0x3bbc78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bbc78() {
}


// 0x3bbc80 — __ZThn36_N3RBX14FormFactorPartD0Ev
// type: void __fastcall(RBX::FormFactorPart *__hidden this)
#[doc(alias = "__ZThn36_N3RBX14FormFactorPartD0Ev")]
// was: __ZThn36_N3RBX14FormFactorPartD0Ev
// IDA 0x3bbc80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bbc80() {
}


// 0x3bbc88 — __ZThn132_N3RBX14FormFactorPartD0Ev
// type: void __fastcall(RBX::FormFactorPart *__hidden this)
#[doc(alias = "__ZThn132_N3RBX14FormFactorPartD0Ev")]
// was: __ZThn132_N3RBX14FormFactorPartD0Ev
// IDA 0x3bbc88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bbc88() {
}


// 0x3bbc90 — __ZN3RBX14FormFactorPartD2Ev
// type: void __fastcall(RBX::FormFactorPart *__hidden this)
#[doc(alias = "RBX::FormFactorPart::~FormFactorPart()")]
// was: __ZN3RBX14FormFactorPartD2Ev
// IDA 0x3bbc90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bbc90() {
}


// 0x3bbc98 — __ZThn32_N3RBX14FormFactorPartD1Ev
// type: void __fastcall(RBX::FormFactorPart *__hidden this)
#[doc(alias = "__ZThn32_N3RBX14FormFactorPartD1Ev")]
// was: __ZThn32_N3RBX14FormFactorPartD1Ev
// IDA 0x3bbc98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bbc98() {
}


// 0x3bbcac — __ZThn36_N3RBX14FormFactorPartD1Ev
// type: void __fastcall(RBX::FormFactorPart *__hidden this)
#[doc(alias = "__ZThn36_N3RBX14FormFactorPartD1Ev")]
// was: __ZThn36_N3RBX14FormFactorPartD1Ev
// IDA 0x3bbcac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bbcac() {
}


// 0x3bbcc0 — __ZThn132_N3RBX14FormFactorPartD1Ev
// type: void __fastcall(RBX::FormFactorPart *__hidden this)
#[doc(alias = "__ZThn132_N3RBX14FormFactorPartD1Ev")]
// was: __ZThn132_N3RBX14FormFactorPartD1Ev
// IDA 0x3bbcc0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bbcc0() {
}


// 0x3bbcd4 — __ZN3RBX14FormFactorPart12readPropertyEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, XmlElement *, int)
#[doc(alias = "RBX::FormFactorPart::readProperty(XmlElement const*,RBX::IReferenceBinder &)")]
// was: __ZN3RBX14FormFactorPart12readPropertyEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x3bbcd4: 110 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bbcd4() {
}


// 0x3bbe50 — __ZN3RBX17BasicPartInstance20setLegacyPartTypeXmlENS0_14LegacyPartTypeE
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "RBX::BasicPartInstance::setLegacyPartTypeXml(RBX::BasicPartInstance::LegacyPartType)")]
// was: __ZN3RBX17BasicPartInstance20setLegacyPartTypeXmlENS0_14LegacyPartTypeE
// IDA 0x3bbe50: 41 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bbe50() {
}


// 0x3bbecc — __ZN3RBX17BasicPartInstance19setLegacyPartTypeUiENS0_14LegacyPartTypeE
// type: int __fastcall(Vector3 *, int)
#[doc(alias = "RBX::BasicPartInstance::setLegacyPartTypeUi(RBX::BasicPartInstance::LegacyPartType)")]
// was: __ZN3RBX17BasicPartInstance19setLegacyPartTypeUiENS0_14LegacyPartTypeE
// IDA 0x3bbecc: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bbecc() {
}


// 0x3bbf78 — __ZN3RBX17BasicPartInstanceC1Ev
// type: RBX::BasicPartInstance *__fastcall(RBX::BasicPartInstance *this)
#[doc(alias = "RBX::BasicPartInstance::BasicPartInstance(void)")]
// was: __ZN3RBX17BasicPartInstanceC1Ev
// IDA 0x3bbf78: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bbf78() {
}


// 0x3bbff0 — __ZN3RBX17BasicPartInstanceC2Ev
// type: RBX::BasicPartInstance *__fastcall(RBX::BasicPartInstance *this, int *)
#[doc(alias = "RBX::BasicPartInstance::BasicPartInstance(void)")]
// was: __ZN3RBX17BasicPartInstanceC2Ev
// IDA 0x3bbff0: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bbff0() {
}


// 0x3bc054 — __ZN3RBX17BasicPartInstanceD0Ev
// type: void __fastcall(RBX::BasicPartInstance *__hidden this)
#[doc(alias = "RBX::BasicPartInstance::~BasicPartInstance()")]
// was: __ZN3RBX17BasicPartInstanceD0Ev
// IDA 0x3bc054: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc054() {
}


// 0x3bc104 — __ZN3RBX17BasicPartInstanceD1Ev
// type: void __fastcall(RBX::BasicPartInstance *__hidden this)
#[doc(alias = "RBX::BasicPartInstance::~BasicPartInstance()")]
// was: __ZN3RBX17BasicPartInstanceD1Ev
// IDA 0x3bc104: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc104() {
}


// 0x3bc114 — __ZThn32_N3RBX17BasicPartInstanceD0Ev
// type: void __fastcall(RBX::BasicPartInstance *__hidden this)
#[doc(alias = "__ZThn32_N3RBX17BasicPartInstanceD0Ev")]
// was: __ZThn32_N3RBX17BasicPartInstanceD0Ev
// IDA 0x3bc114: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc114() {
}


// 0x3bc11c — __ZThn36_N3RBX17BasicPartInstanceD0Ev
// type: void __fastcall(RBX::BasicPartInstance *__hidden this)
#[doc(alias = "__ZThn36_N3RBX17BasicPartInstanceD0Ev")]
// was: __ZThn36_N3RBX17BasicPartInstanceD0Ev
// IDA 0x3bc11c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc11c() {
}


// 0x3bc124 — __ZThn132_N3RBX17BasicPartInstanceD0Ev
// type: void __fastcall(RBX::BasicPartInstance *__hidden this)
#[doc(alias = "__ZThn132_N3RBX17BasicPartInstanceD0Ev")]
// was: __ZThn132_N3RBX17BasicPartInstanceD0Ev
// IDA 0x3bc124: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc124() {
}


// 0x3bc12c — __ZN3RBX17BasicPartInstanceD2Ev
// type: void __fastcall(RBX::BasicPartInstance *__hidden this)
#[doc(alias = "RBX::BasicPartInstance::~BasicPartInstance()")]
// was: __ZN3RBX17BasicPartInstanceD2Ev
// IDA 0x3bc12c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc12c() {
}


// 0x3bc134 — __ZThn32_N3RBX17BasicPartInstanceD1Ev
// type: void __fastcall(RBX::BasicPartInstance *__hidden this)
#[doc(alias = "__ZThn32_N3RBX17BasicPartInstanceD1Ev")]
// was: __ZThn32_N3RBX17BasicPartInstanceD1Ev
// IDA 0x3bc134: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc134() {
}


// 0x3bc148 — __ZThn36_N3RBX17BasicPartInstanceD1Ev
// type: void __fastcall(RBX::BasicPartInstance *__hidden this)
#[doc(alias = "__ZThn36_N3RBX17BasicPartInstanceD1Ev")]
// was: __ZThn36_N3RBX17BasicPartInstanceD1Ev
// IDA 0x3bc148: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc148() {
}


// 0x3bc15c — __ZThn132_N3RBX17BasicPartInstanceD1Ev
// type: void __fastcall(RBX::BasicPartInstance *__hidden this)
#[doc(alias = "__ZThn132_N3RBX17BasicPartInstanceD1Ev")]
// was: __ZThn132_N3RBX17BasicPartInstanceD1Ev
// IDA 0x3bc15c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc15c() {
}


// 0x3bc170 — __ZN3RBX17BasicPartInstance18validateFormFactorERNS_12PartInstance10FormFactorE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "RBX::BasicPartInstance::validateFormFactor(RBX::PartInstance::FormFactor &)")]
// was: __ZN3RBX17BasicPartInstance18validateFormFactorERNS_12PartInstance10FormFactorE
// IDA 0x3bc170: 6 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bc170() {
}


// 0x3bc180 — __ZNK3RBX17BasicPartInstance11getPartTypeEv
// type: int __fastcall(RBX::BasicPartInstance *this)
#[doc(alias = "RBX::BasicPartInstance::getPartType(void)const")]
// was: __ZNK3RBX17BasicPartInstance11getPartTypeEv
// IDA 0x3bc180: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bc180() {
}


// 0x3bc1e0 — __ZN3RBX17BasicPartInstance23hasThreeDimensionalSizeEv
// type: int __fastcall(RBX::BasicPartInstance *this)
#[doc(alias = "RBX::BasicPartInstance::hasThreeDimensionalSize(void)")]
// was: __ZN3RBX17BasicPartInstance23hasThreeDimensionalSizeEv
// IDA 0x3bc1e0: 5 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bc1e0() {
}


// 0x3bc1fc — __ZN3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEED1Ev
// IDA 0x3bc1fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc1fc() {
}


// 0x3bc220 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x3bc220: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bc220() {
}


// 0x3bc460 — __ZN3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEED1Ev
// IDA 0x3bc460: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc460() {
}


// 0x3bc484 — __ZNK3RBX17BasicPartInstance17getLegacyPartTypeEv
// type: int __fastcall(RBX::BasicPartInstance *this)
#[doc(alias = "RBX::BasicPartInstance::getLegacyPartType(void)const")]
// was: __ZNK3RBX17BasicPartInstance17getLegacyPartTypeEv
// IDA 0x3bc484: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bc484() {
}


// 0x3bc48c — __ZN3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev
// type: __guard *__fastcall(RBX::FormFactorPart *, _DWORD *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")]
// was: __ZN3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev
// IDA 0x3bc48c: 177 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bc48c() {
}


// 0x3bc6a8 — __ZNK3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEE12getClassNameEv
// IDA 0x3bc6a8: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bc6a8() {
}


// 0x3bc6d0 — __ZNK3RBX14FormFactorPart13getFormFactorEv
// type: int __fastcall(RBX::FormFactorPart *this)
#[doc(alias = "RBX::FormFactorPart::getFormFactor(void)const")]
// was: __ZNK3RBX14FormFactorPart13getFormFactorEv
// IDA 0x3bc6d0: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bc6d0() {
}


// 0x3bc6d8 — __ZN3RBX14FormFactorPart18validateFormFactorERNS_12PartInstance10FormFactorE
// type: void()
#[doc(alias = "RBX::FormFactorPart::validateFormFactor(RBX::PartInstance::FormFactor &)")]
// was: __ZN3RBX14FormFactorPart18validateFormFactorERNS_12PartInstance10FormFactorE
// IDA 0x3bc6d8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3bc6d8() {
}


// 0x3bc6dc — __ZThn32_NK3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEE12getClassNameEv
// IDA 0x3bc6dc: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bc6dc() {
}


// 0x3bc704 — __ZN3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3bc704: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc704() {
}


// 0x3bc718 — __ZN3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3bc718: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc718() {
}


// 0x3bc7c8 — __ZThn132_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn132_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3bc7c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc7c8() {
}


// 0x3bc7dc — __ZThn132_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn132_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3bc7dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc7dc() {
}


// 0x3bc890 — __ZN3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3bc890: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc890() {
}


// 0x3bc8a4 — __ZN3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3bc8a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc8a4() {
}


// 0x3bc954 — __ZThn132_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn132_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3bc954: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc954() {
}


// 0x3bc968 — __ZThn132_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn132_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3bc968: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bc968() {
}


// 0x3bca1c — __ZN3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev")]
// was: __ZN3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev
// IDA 0x3bca1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bca1c() {
}


// 0x3bca30 — __ZN3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev")]
// was: __ZN3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev
// IDA 0x3bca30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bca30() {
}


// 0x3bcae0 — __ZThn132_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev")]
// was: __ZThn132_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev
// IDA 0x3bcae0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bcae0() {
}


// 0x3bcaf4 — __ZThn132_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev")]
// was: __ZThn132_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev
// IDA 0x3bcaf4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bcaf4() {
}


// 0x3bcba4 — __ZNK3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE12getClassNameEv
// IDA 0x3bcba4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bcba4() {
}


// 0x3bcbb4 — __ZThn32_NK3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE12getClassNameEv
// IDA 0x3bcbb4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bcbb4() {
}


// 0x3bcbc4 — __ZN3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3bcbc4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bcbc4() {
}


// 0x3bcbd8 — __ZN3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3bcbd8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bcbd8() {
}


// 0x3bcc88 — __ZThn132_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn132_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3bcc88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bcc88() {
}


// 0x3bcc9c — __ZThn132_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn132_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3bcc9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bcc9c() {
}


// 0x3bcd50 — __ZN3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3bcd50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bcd50() {
}


// 0x3bcd64 — __ZN3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3bcd64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bcd64() {
}


// 0x3bce14 — __ZThn132_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn132_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3bce14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bce14() {
}


// 0x3bce28 — __ZThn132_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn132_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3bce28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bce28() {
}


// 0x3bcedc — __ZN3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED1Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED1Ev")]
// was: __ZN3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED1Ev
// IDA 0x3bcedc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bcedc() {
}


// 0x3bcef0 — __ZN3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED0Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED0Ev")]
// was: __ZN3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED0Ev
// IDA 0x3bcef0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bcef0() {
}


// 0x3bcfa0 — __ZThn132_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED1Ev")]
// was: __ZThn132_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED1Ev
// IDA 0x3bcfa0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bcfa0() {
}


// 0x3bcfb4 — __ZThn132_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED0Ev")]
// was: __ZThn132_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED0Ev
// IDA 0x3bcfb4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bcfb4() {
}


// 0x3bcfbc — __ZN3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE17static_getCreatorEv
// IDA 0x3bcfbc: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bcfbc() {
}


// 0x3bd030 — __ZN3RBX4Name13callDoDeclareILZNS_15sFormFactorPartEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sFormFactorPartEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_15sFormFactorPartEEEEvv
// IDA 0x3bd030: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3bd030() {
}


// 0x3bd034 — __ZN3RBX4Name9doDeclareILZNS_15sFormFactorPartEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sFormFactorPartEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_15sFormFactorPartEEEERKS0_v
// IDA 0x3bd034: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd034() {
}


// 0x3bd114 — __ZThn32_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED1Ev")]
// was: __ZThn32_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED1Ev
// IDA 0x3bd114: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd114() {
}


// 0x3bd128 — __ZThn36_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED1Ev")]
// was: __ZThn36_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED1Ev
// IDA 0x3bd128: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd128() {
}


// 0x3bd13c — __ZThn32_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED0Ev")]
// was: __ZThn32_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED0Ev
// IDA 0x3bd13c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd13c() {
}


// 0x3bd144 — __ZThn36_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED0Ev")]
// was: __ZThn36_N3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEED0Ev
// IDA 0x3bd144: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd144() {
}


// 0x3bd14c — __ZThn32_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3bd14c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd14c() {
}


// 0x3bd160 — __ZThn32_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3bd160: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd160() {
}


// 0x3bd214 — __ZThn36_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3bd214: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd214() {
}


// 0x3bd228 — __ZThn36_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3bd228: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd228() {
}


// 0x3bd2dc — __ZThn32_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3bd2dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd2dc() {
}


// 0x3bd2f0 — __ZThn32_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3bd2f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd2f0() {
}


// 0x3bd3a4 — __ZThn36_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3bd3a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd3a4() {
}


// 0x3bd3b8 — __ZThn36_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3bd3b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd3b8() {
}


// 0x3bd46c — __ZN3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::EnumPropDescriptor<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>(char const*,char const*,RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x3bd46c: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd46c() {
}


// 0x3bd620 — __ZN3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEED0Ev
// IDA 0x3bd620: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bd620() {
}


// 0x3bd64c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10isReadOnlyEv
// IDA 0x3bd64c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd64c() {
}


// 0x3bd65c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE11isWriteOnlyEv
// IDA 0x3bd65c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd65c() {
}


// 0x3bd66c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x3bd66c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd66c() {
}


// 0x3bd694 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x3bd694: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd694() {
}


// 0x3bd6b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x3bd6b8: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd6b8() {
}


// 0x3bd804 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x3bd804: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd804() {
}


// 0x3bd828 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE14hasStringValueEv
// IDA 0x3bd828: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd828() {
}


// 0x3bd82c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x3bd82c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd82c() {
}


// 0x3bd850 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x3bd850: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd850() {
}


// 0x3bd890 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x3bd890: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd890() {
}


// 0x3bd8b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x3bd8b0: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bd8b0() {
}


// 0x3bdaf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x3bdaf0: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bdaf0() {
}


// 0x3bdb0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x3bdb0c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bdb0c() {
}


// 0x3bdb40 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x3bdb40: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bdb40() {
}


// 0x3bdb48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x3bdb48: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bdb48() {
}


// 0x3bdb94 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x3bdb94: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bdb94() {
}


// 0x3bdbb4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x3bdbb4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bdbb4() {
}


// 0x3bdbe8 — __ZNK3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToIndex(RBX::BasicPartInstance::LegacyPartType)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE14convertToIndexES3_
// IDA 0x3bdbe8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bdbe8() {
}


// 0x3bdc58 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x3bdc58: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bdc58() {
}


// 0x3bdc98 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::GetSetImpl<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x3bdc98: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bdc98() {
}


// 0x3bdc9c — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::GetSetImpl<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x3bdc9c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bdc9c() {
}


// 0x3bdca0 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::GetSetImpl<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x3bdca0: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bdca0() {
}


// 0x3bdcc0 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::GetSetImpl<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::setValue(RBX::Reflection::DescribedBase *,RBX::BasicPartInstance::LegacyPartType const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x3bdcc0: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bdcc0() {
}