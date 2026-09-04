//! rendering shard 462 — 100 stubs 0x6f6098..0x6fa8c4 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Global gap filler fallback EA asc not yet in rbx_rendering (49310->49410 distinct, fallback after 0x6f6098).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc global gap filler not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x6f6098 — __ZN3RBX10Reflection4Type12getSingletonINS_4UDimEEERKS1_v
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::UDim>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_4UDimEEERKS1_v")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_4UDimEEERKS1_v
// IDA 0x6f6098: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6098() {
}

// 0x6f6180 — __ZN3RBX10Reflection4Type12getSingletonINS_11InputObjectEEERKS1_v
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::InputObject>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_11InputObjectEEERKS1_v")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_11InputObjectEEERKS1_v
// IDA 0x6f6180: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6180() {
}

// 0x6f6268 — __ZN3RBX10Reflection7Variant7convertINS_5UDim2EEERT_v
// type: 
#[doc(alias = "RBX::UDim2 & RBX::Reflection::Variant::convert<RBX::UDim2>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant7convertINS_5UDim2EEERT_v")]
// was: __ZN3RBX10Reflection7Variant7convertINS_5UDim2EEERT_v
// IDA 0x6f6268: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6f6268() {
}

// 0x6f626c — __ZN3RBX10Reflection4Type12getSingletonINS_5UDim2EEERKS1_v
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::UDim2>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_5UDim2EEERKS1_v")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_5UDim2EEERKS1_v
// IDA 0x6f626c: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f626c() {
}

// 0x6f6354 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE11getDataSizeEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE11getDataSizeEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE11getDataSizeEPKNS0_13DescribedBaseE
// IDA 0x6f6354: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6354() {
}

// 0x6f6358 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE14hasStringValueEv
// type: 
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE14hasStringValueEv")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE14hasStringValueEv
// IDA 0x6f6358: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6358() {
}

// 0x6f635c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE14getStringValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE14getStringValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x6f635c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f635c() {
}

// 0x6f637c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE14setStringValueEPNS0_13DescribedBaseERKSs
// type: 
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE14setStringValueEPNS0_13DescribedBaseERKSs")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x6f637c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f637c() {
}

// 0x6f6380 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x6f6380: 69 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6380() {
}

// 0x6f6450 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x6f6450: 69 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6450() {
}

// 0x6f6530 — __ZN3RBX10Reflection7Variant7convertINS_5FacesEEERT_v
// type: 
#[doc(alias = "RBX::Faces & RBX::Reflection::Variant::convert<RBX::Faces>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant7convertINS_5FacesEEERT_v")]
// was: __ZN3RBX10Reflection7Variant7convertINS_5FacesEEERT_v
// IDA 0x6f6530: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6f6530() {
}

// 0x6f6534 — __ZN3RBX10Reflection4Type12getSingletonINS_5FacesEEERKS1_v
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Faces>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_5FacesEEERKS1_v")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_5FacesEEERKS1_v
// IDA 0x6f6534: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6534() {
}

// 0x6f661c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE11getDataSizeEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE11getDataSizeEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE11getDataSizeEPKNS0_13DescribedBaseE
// IDA 0x6f661c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f661c() {
}

// 0x6f6620 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE14hasStringValueEv")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE14hasStringValueEv
// IDA 0x6f6620: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6620() {
}

// 0x6f6624 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE14getStringValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x6f6624: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6624() {
}

// 0x6f6644 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x6f6644: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6644() {
}

// 0x6f6648 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x6f6648: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6648() {
}

// 0x6f6694 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x6f6694: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6694() {
}

// 0x6f66d8 — __ZN3RBX10Reflection7Variant7convertINS_4AxesEEERT_v
// type: int()
#[doc(alias = "RBX::Axes & RBX::Reflection::Variant::convert<RBX::Axes>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant7convertINS_4AxesEEERT_v")]
// was: __ZN3RBX10Reflection7Variant7convertINS_4AxesEEERT_v
// IDA 0x6f66d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6f66d8() {
}

// 0x6f66dc — __ZN3RBX10Reflection4Type12getSingletonINS_4AxesEEERKS1_v
// type: int *()
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Axes>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_4AxesEEERKS1_v")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_4AxesEEERKS1_v
// IDA 0x6f66dc: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f66dc() {
}

// 0x6f67c4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE11getDataSizeEPKNS0_13DescribedBaseE
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE11getDataSizeEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE11getDataSizeEPKNS0_13DescribedBaseE
// IDA 0x6f67c4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f67c4() {
}

// 0x6f67c8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE14hasStringValueEv")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE14hasStringValueEv
// IDA 0x6f67c8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f67c8() {
}

// 0x6f67cc — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE14getStringValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x6f67cc: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f67cc() {
}

// 0x6f67ec — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x6f67ec: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f67ec() {
}

// 0x6f67f0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x6f67f0: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f67f0() {
}

// 0x6f683c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_4AxesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x6f683c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f683c() {
}

// 0x6f6ad0 — __ZN3RBX10Reflection7Variant7convertINS_6RbxRayEEERT_v
// type: int()
#[doc(alias = "RBX::RbxRay & RBX::Reflection::Variant::convert<RBX::RbxRay>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant7convertINS_6RbxRayEEERT_v")]
// was: __ZN3RBX10Reflection7Variant7convertINS_6RbxRayEEERT_v
// IDA 0x6f6ad0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6f6ad0() {
}

// 0x6f6ad4 — __ZN3RBX10Reflection4Type12getSingletonINS_6RbxRayEEERKS1_v
// type: int *()
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::RbxRay>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_6RbxRayEEERKS1_v")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_6RbxRayEEERKS1_v
// IDA 0x6f6ad4: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6ad4() {
}

// 0x6f6bbc — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE11getDataSizeEPKNS0_13DescribedBaseE
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::RbxRay>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE11getDataSizeEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE11getDataSizeEPKNS0_13DescribedBaseE
// IDA 0x6f6bbc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6bbc() {
}

// 0x6f6bc0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::RbxRay>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE14hasStringValueEv")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE14hasStringValueEv
// IDA 0x6f6bc0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6bc0() {
}

// 0x6f6bc4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::RbxRay>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE14getStringValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x6f6bc4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6bc4() {
}

// 0x6f6be4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::RbxRay>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x6f6be4: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6be4() {
}

// 0x6f6c44 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::RbxRay>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x6f6c44: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6c44() {
}

// 0x6f6cec — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::RbxRay>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6RbxRayEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x6f6cec: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6cec() {
}

// 0x6f6d50 — __ZN3RBX10Reflection7Variant7convertINS_10BrickColorEEERT_v
// type: int()
#[doc(alias = "RBX::BrickColor & RBX::Reflection::Variant::convert<RBX::BrickColor>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant7convertINS_10BrickColorEEERT_v")]
// was: __ZN3RBX10Reflection7Variant7convertINS_10BrickColorEEERT_v
// IDA 0x6f6d50: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6f6d50() {
}

// 0x6f6d54 — __ZN3RBX10Reflection4Type12getSingletonINS_10BrickColorEEERKS1_v
// type: int *()
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::BrickColor>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_10BrickColorEEERKS1_v")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_10BrickColorEEERKS1_v
// IDA 0x6f6d54: 75 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6d54() {
}

// 0x6f6e44 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE11getDataSizeEPKNS0_13DescribedBaseE
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE11getDataSizeEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE11getDataSizeEPKNS0_13DescribedBaseE
// IDA 0x6f6e44: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6e44() {
}

// 0x6f6e48 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE14hasStringValueEv")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE14hasStringValueEv
// IDA 0x6f6e48: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6e48() {
}

// 0x6f6e4c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE14getStringValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x6f6e4c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6e4c() {
}

// 0x6f6e74 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x6f6e74: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6e74() {
}

// 0x6f6e84 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x6f6e84: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6e84() {
}

// 0x6f6ec0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x6f6ec0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6ec0() {
}

// 0x6f6ee8 — __ZN3RBX10Reflection7Variant7convertINS_13SystemAddressEEERT_v
// type: int()
#[doc(alias = "RBX::SystemAddress & RBX::Reflection::Variant::convert<RBX::SystemAddress>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant7convertINS_13SystemAddressEEERT_v")]
// was: __ZN3RBX10Reflection7Variant7convertINS_13SystemAddressEEERT_v
// IDA 0x6f6ee8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6f6ee8() {
}

// 0x6f6eec — __ZN3RBX10Reflection4Type12getSingletonINS_13SystemAddressEEERKS1_v
// type: int *()
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::SystemAddress>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_13SystemAddressEEERKS1_v")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_13SystemAddressEEERKS1_v
// IDA 0x6f6eec: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6eec() {
}

// 0x6f6fd4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE11getDataSizeEPKNS0_13DescribedBaseE
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE11getDataSizeEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE11getDataSizeEPKNS0_13DescribedBaseE
// IDA 0x6f6fd4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6fd4() {
}

// 0x6f6fd8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE14hasStringValueEv")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE14hasStringValueEv
// IDA 0x6f6fd8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6fd8() {
}

// 0x6f6fdc — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE14getStringValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x6f6fdc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6fdc() {
}

// 0x6f7004 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x6f7004: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f7004() {
}

// 0x6f7014 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x6f7014: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f7014() {
}

// 0x6f7070 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x6f7070: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f7070() {
}

// 0x6f70cc — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x6f70cc: 129 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f70cc() {
}

// 0x6f7234 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x6f7234: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f7234() {
}

// 0x6f7354 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE11getDataSizeEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE11getDataSizeEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE11getDataSizeEPKNS0_13DescribedBaseE
// IDA 0x6f7354: 34 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f7354() {
}

// 0x6f73b0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE14hasStringValueEv")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE14hasStringValueEv
// IDA 0x6f73b0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f73b0() {
}

// 0x6f73b4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE14getStringValueEPKNS0_13DescribedBaseE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE14getStringValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x6f73b4: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f73b4() {
}

// 0x6f74d0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(RBX::Name *, int, std::string *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9ContentIdEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x6f74d0: 126 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f74d0() {
}

// 0x6f7630 — __ZN3RBX10Reflection8EnumDescINS_8NormalIdEE7addPairES2_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NormalId>::addPair(RBX::NormalId,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_8NormalIdEE7addPairES2_PKc")]
// was: __ZN3RBX10Reflection8EnumDescINS_8NormalIdEE7addPairES2_PKc
// IDA 0x6f7630: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f7630() {
}

// 0x6f7990 — __ZN3RBX10Reflection7Variant14genericConvertINS_8NormalIdEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::NormalId & RBX::Reflection::Variant::genericConvert<RBX::NormalId>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_8NormalIdEEERT_v")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_8NormalIdEEERT_v
// IDA 0x6f7990: 143 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f7990() {
}

// 0x6f7b7c — __ZN16XmlNameValuePair8setValueESs
// type: void __fastcall(int, const std::string *)
#[doc(alias = "XmlNameValuePair::setValue(std::string)")]
#[doc(alias = "__ZN16XmlNameValuePair8setValueESs")]
// was: __ZN16XmlNameValuePair8setValueESs
// IDA 0x6f7b7c: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f7b7c() {
}

// 0x6f7c30 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS0_13DescribedBaseEEEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<boost::shared_ptr<RBX::Reflection::DescribedBase>>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS0_13DescribedBaseEEEED1Ev")]
// was: __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS0_13DescribedBaseEEEED1Ev
// IDA 0x6f7c30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6f7c30() {
}

// 0x6f7c34 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS_8InstanceEEEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<boost::shared_ptr<RBX::Instance>>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS_8InstanceEEEED1Ev")]
// was: __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS_8InstanceEEEED1Ev
// IDA 0x6f7c34: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6f7c34() {
}

// 0x6f7c38 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEEED1Ev")]
// was: __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEEED1Ev
// IDA 0x6f7c38: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6f7c38() {
}

// 0x6f7c3c — __ZN3RBX10Reflection5TTypeIiED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<int>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIiED1Ev")]
// was: __ZN3RBX10Reflection5TTypeIiED1Ev
// IDA 0x6f7c3c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6f7c3c() {
}

// 0x6f7c40 — __ZN3RBX10Reflection5TTypeIlED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<long>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIlED1Ev")]
// was: __ZN3RBX10Reflection5TTypeIlED1Ev
// IDA 0x6f7c40: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6f7c40() {
}

// 0x6f7c44 — __ZN3rbx8any_castIdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: __int64 __fastcall(int)
#[doc(alias = "double rbx::any_cast<double,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x6f7c44: 79 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f7c44() {
}

// 0x6f7d30 — __ZN3rbx8any_castIfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(int)
#[doc(alias = "float rbx::any_cast<float,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x6f7d30: 79 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f7d30() {
}

// 0x6f7e18 — __ZN3rbx8any_castIbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: bool __fastcall(int)
#[doc(alias = "bool rbx::any_cast<bool,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x6f7e18: 82 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f7e18() {
}

// 0x6f7f08 — __ZN3RBX10Reflection7Variant14genericConvertIiEERT_v
// type: int __fastcall(_DWORD *)
#[doc(alias = "int & RBX::Reflection::Variant::genericConvert<int>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertIiEERT_v")]
// was: __ZN3RBX10Reflection7Variant14genericConvertIiEERT_v
// IDA 0x6f7f08: 164 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f7f08() {
}

// 0x6f8140 — __ZN3RBX10Reflection5TTypeIbED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<bool>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIbED1Ev")]
// was: __ZN3RBX10Reflection5TTypeIbED1Ev
// IDA 0x6f8140: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6f8140() {
}

// 0x6f8144 — __ZN3rbx8any_castIiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(int)
#[doc(alias = "int rbx::any_cast<int,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x6f8144: 79 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f8144() {
}

// 0x6f822c — __ZN3RBX10Reflection7Variant14genericConvertIbEERT_v
// type: int __fastcall(_DWORD *)
#[doc(alias = "bool & RBX::Reflection::Variant::genericConvert<bool>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertIbEERT_v")]
// was: __ZN3RBX10Reflection7Variant14genericConvertIbEERT_v
// IDA 0x6f822c: 168 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f822c() {
}

// 0x6f846c — __ZN3RBX10Reflection5TTypeIfED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<float>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIfED1Ev")]
// was: __ZN3RBX10Reflection5TTypeIfED1Ev
// IDA 0x6f846c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6f846c() {
}

// 0x6f8470 — __ZN3RBX10Reflection7Variant14genericConvertIfEERT_v
// type: int __fastcall(_DWORD *)
#[doc(alias = "float & RBX::Reflection::Variant::genericConvert<float>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertIfEERT_v")]
// was: __ZN3RBX10Reflection7Variant14genericConvertIfEERT_v
// IDA 0x6f8470: 164 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f8470() {
}

// 0x6f86ac — __ZN3RBX10Reflection5TTypeIdED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<double>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIdED1Ev")]
// was: __ZN3RBX10Reflection5TTypeIdED1Ev
// IDA 0x6f86ac: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6f86ac() {
}

// 0x6f86b0 — __ZN3RBX10Reflection7Variant14genericConvertIdEERT_v
// type: int __fastcall(_DWORD *)
#[doc(alias = "double & RBX::Reflection::Variant::genericConvert<double>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertIdEERT_v")]
// was: __ZN3RBX10Reflection7Variant14genericConvertIdEERT_v
// IDA 0x6f86b0: 164 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f86b0() {
}

// 0x6f88ec — __ZN3RBX10Reflection7Variant14genericConvertINS_7Region3EEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::Region3 & RBX::Reflection::Variant::genericConvert<RBX::Region3>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_7Region3EEERT_v")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_7Region3EEERT_v
// IDA 0x6f88ec: 147 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f88ec() {
}

// 0x6f8aec — __ZN3RBX10Reflection7Variant14genericConvertINS_12Region3int16EEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::Region3int16 & RBX::Reflection::Variant::genericConvert<RBX::Region3int16>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_12Region3int16EEERT_v")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_12Region3int16EEERT_v
// IDA 0x6f8aec: 147 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f8aec() {
}

// 0x6f9304 — __ZN3RBX10Reflection5TTypeINS_9ContentIdEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<RBX::ContentId>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_9ContentIdEED1Ev")]
// was: __ZN3RBX10Reflection5TTypeINS_9ContentIdEED1Ev
// IDA 0x6f9304: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6f9304() {
}

// 0x6f9308 — __ZN3RBX10Reflection7Variant14genericConvertINS_9ContentIdEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::ContentId & RBX::Reflection::Variant::genericConvert<RBX::ContentId>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_9ContentIdEEERT_v")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_9ContentIdEEERT_v
// IDA 0x6f9308: 179 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f9308() {
}

// 0x6f95e4 — __ZN3RBX10Reflection5TTypeISsED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<std::string>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeISsED1Ev")]
// was: __ZN3RBX10Reflection5TTypeISsED1Ev
// IDA 0x6f95e4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6f95e4() {
}

// 0x6f95f4 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKNS0_5TupleEEEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<boost::shared_ptr<RBX::Reflection::Tuple const>>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKNS0_5TupleEEEED1Ev")]
// was: __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKNS0_5TupleEEEED1Ev
// IDA 0x6f95f4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6f95f4() {
}

// 0x6f95f8 — __ZN3rbx8any_castIlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(int)
#[doc(alias = "long rbx::any_cast<long,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x6f95f8: 79 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f95f8() {
}

// 0x6f98e4 — __ZN3rbx8any_castIN3RBX3Lua15WeakFunctionRefENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: void __fastcall(RBX::Lua::WeakFunctionRef *, _DWORD **)
#[doc(alias = "RBX::Lua::WeakFunctionRef rbx::any_cast<RBX::Lua::WeakFunctionRef,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX3Lua15WeakFunctionRefENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIN3RBX3Lua15WeakFunctionRefENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x6f98e4: 82 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f98e4() {
}

// 0x6f99d8 — __ZN3rbx8any_castIN5boost10shared_ptrIN3RBX8InstanceEEENS3_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "boost::shared_ptr<RBX::Instance> * rbx::any_cast<boost::shared_ptr<RBX::Instance>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN5boost10shared_ptrIN3RBX8InstanceEEENS3_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIN5boost10shared_ptrIN3RBX8InstanceEEENS3_7Region3EEEPT_PNS_13placement_anyIT0_EE
// IDA 0x6f99d8: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f99d8() {
}

// 0x6f9a30 — __ZN3rbx8any_castISsN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "std::string * rbx::any_cast<std::string,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castISsN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castISsN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
// IDA 0x6f9a30: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f9a30() {
}

// 0x6f9a88 — __ZN3rbx8any_castIN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEENS3_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "boost::shared_ptr<RBX::Reflection::DescribedBase> * rbx::any_cast<boost::shared_ptr<RBX::Reflection::DescribedBase>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEENS3_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEENS3_7Region3EEEPT_PNS_13placement_anyIT0_EE
// IDA 0x6f9a88: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f9a88() {
}

// 0x6f9ae0 — __ZN3rbx8any_castIN5boost10shared_ptrIKSt6vectorINS2_IN3RBX8InstanceEEESaIS6_EEEENS4_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> * rbx::any_cast<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN5boost10shared_ptrIKSt6vectorINS2_IN3RBX8InstanceEEESaIS6_EEEENS4_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIN5boost10shared_ptrIKSt6vectorINS2_IN3RBX8InstanceEEESaIS6_EEEENS4_7Region3EEEPT_PNS_13placement_anyIT0_EE
// IDA 0x6f9ae0: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f9ae0() {
}

// 0x6f9b38 — __ZN3rbx8any_castIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS6_EEEENS4_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> * rbx::any_cast<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS6_EEEENS4_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS6_EEEENS4_7Region3EEEPT_PNS_13placement_anyIT0_EE
// IDA 0x6f9b38: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f9b38() {
}

// 0x6f9b90 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN3RBX10Reflection7VariantESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvPFvS4_NSB_10shared_ptrIS7_INSE_INS2_8InstanceEEESaISG_EEEEENSC_5list2INSB_3argILi1EEENSC_5valueISJ_EEEEEEET0_T_SU_ST_
// type: sp_counted_base *__fastcall(sp_counted_base **, int, int, const shared_count *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>)")]
#[doc(alias = "__ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN3RBX10Reflection7VariantESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvPFvS4_NSB_10shared_ptrIS7_INSE_INS2_8InstanceEEESaISG_EEEEENSC_5list2INSB_3argILi1EEENSC_5valueISJ_EEEEEEET0_T_SU_ST_")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN3RBX10Reflection7VariantESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvPFvS4_NSB_10shared_ptrIS7_INSE_INS2_8InstanceEEESaISG_EEEEENSC_5list2INSB_3argILi1EEENSC_5valueISJ_EEEEEEET0_T_SU_ST_
// IDA 0x6f9b90: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f9b90() {
}

// 0x6f9be8 — __ZN5boost4bindIvN3RBX10Reflection7VariantENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEENS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_ENSD_9list_av_2IT2_T3_E4typeEEESJ_SL_SM_
// type: void __fastcall(_DWORD *, int, const shared_count *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list_av_2<boost::arg<1>,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::type> boost::bind<void,RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::arg<1>,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(void (*)(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::arg<1>,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection7VariantENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEENS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_ENSD_9list_av_2IT2_T3_E4typeEEESJ_SL_SM_")]
// was: __ZN5boost4bindIvN3RBX10Reflection7VariantENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEENS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_ENSD_9list_av_2IT2_T3_E4typeEEESJ_SL_SM_
// IDA 0x6f9be8: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f9be8() {
}

// 0x6f9d00 — __ZN3rbx11make_sharedIN3RBX10Reflection5TupleEiEEN5boost10shared_ptrIT_EERKT0_
// type: int __fastcall(int *, unsigned int *)
#[doc(alias = "boost::shared_ptr<RBX::Reflection::Tuple> rbx::make_shared<RBX::Reflection::Tuple,int>(int const&)")]
#[doc(alias = "__ZN3rbx11make_sharedIN3RBX10Reflection5TupleEiEEN5boost10shared_ptrIT_EERKT0_")]
// was: __ZN3rbx11make_sharedIN3RBX10Reflection5TupleEiEEN5boost10shared_ptrIT_EERKT0_
// IDA 0x6f9d00: 129 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f9d00() {
}

// 0x6fa074 — __ZN3RBX10Reflection5TTypeINS_4UDimEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<RBX::UDim>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_4UDimEED1Ev")]
// was: __ZN3RBX10Reflection5TTypeINS_4UDimEED1Ev
// IDA 0x6fa074: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6fa074() {
}

// 0x6fa078 — __ZN3RBX10Reflection5TTypeINS_11InputObjectEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<RBX::InputObject>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_11InputObjectEED1Ev")]
// was: __ZN3RBX10Reflection5TTypeINS_11InputObjectEED1Ev
// IDA 0x6fa078: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6fa078() {
}

// 0x6fa07c — __ZN3RBX10Reflection7Variant14genericConvertINS_5UDim2EEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::UDim2 & RBX::Reflection::Variant::genericConvert<RBX::UDim2>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5UDim2EEERT_v")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_5UDim2EEERT_v
// IDA 0x6fa07c: 152 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6fa07c() {
}

// 0x6fa290 — __ZN3RBX10Reflection5TTypeINS_5UDim2EED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<RBX::UDim2>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_5UDim2EED1Ev")]
// was: __ZN3RBX10Reflection5TTypeINS_5UDim2EED1Ev
// IDA 0x6fa290: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6fa290() {
}

// 0x6fa294 — __ZN3RBX10Reflection7Variant14genericConvertINS_5FacesEEERT_v
// type: int __fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::Faces & RBX::Reflection::Variant::genericConvert<RBX::Faces>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5FacesEEERT_v")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_5FacesEEERT_v
// IDA 0x6fa294: 152 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6fa294() {
}

// 0x6fa4a4 — __ZN3RBX10Reflection5TTypeINS_5FacesEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<RBX::Faces>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_5FacesEED1Ev")]
// was: __ZN3RBX10Reflection5TTypeINS_5FacesEED1Ev
// IDA 0x6fa4a4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6fa4a4() {
}

// 0x6fa4a8 — __ZN3RBX10Reflection7Variant14genericConvertINS_4AxesEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::Axes & RBX::Reflection::Variant::genericConvert<RBX::Axes>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_4AxesEEERT_v")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_4AxesEEERT_v
// IDA 0x6fa4a8: 152 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6fa4a8() {
}

// 0x6fa6b8 — __ZN3RBX10Reflection5TTypeINS_4AxesEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<RBX::Axes>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_4AxesEED1Ev")]
// was: __ZN3RBX10Reflection5TTypeINS_4AxesEED1Ev
// IDA 0x6fa6b8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6fa6b8() {
}

// 0x6fa8c4 — __ZN3RBX10Reflection7Variant14genericConvertINS_6RbxRayEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::RbxRay & RBX::Reflection::Variant::genericConvert<RBX::RbxRay>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_6RbxRayEEERT_v")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_6RbxRayEEERT_v
// IDA 0x6fa8c4: 166 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6fa8c4() {
}
