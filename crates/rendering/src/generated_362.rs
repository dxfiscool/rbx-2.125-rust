//! rendering shard 362 — 100 stubs 0x4ecef8..0x4efba0 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 39423->39523 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4ecef8 — __ZNK3RBX10Reflection14PropDescriptorINS_13VelocityMotorEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VelocityMotor,float>::GetSetImpl<float (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::VelocityMotor,float>::GetSetImpl<float (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x4ecef8: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ecef8() {
}

// 0x4ecf18 — __ZNK3RBX10Reflection14PropDescriptorINS_13VelocityMotorEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VelocityMotor,float>::GetSetImpl<float (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::VelocityMotor,float>::GetSetImpl<float (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
// IDA 0x4ecf18: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ecf18() {
}

// 0x4ecf3c — __ZN3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::RefPropDescriptor<RBX::Hole* (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(RBX::Hole*)>(char const*,char const*,RBX::Hole* (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(RBX::Hole*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::RefPropDescriptor<RBX::Hole* (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(RBX::Hole*)>(char const*,char const*,RBX::Hole* (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(RBX::Hole*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x4ecf3c: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ecf3c() {
}

// 0x4ecfe0 — __ZN3RBX10Reflection7RefTypeIPNS_4HoleEE9singletonEv
#[doc(alias = "RBX::Reflection::RefType<RBX::Hole *>::singleton(void)")]
// was: RBX::Reflection::RefType<RBX::Hole *>::singleton(void)
// IDA 0x4ecfe0: 79 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ecfe0() {
}

// 0x4ed0d8 — __ZN3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::~RefPropDescriptor()")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::~RefPropDescriptor()
// IDA 0x4ed0d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4ed0d8() {
}

// 0x4ed108 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::isReadOnly(void)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::isReadOnly(void)const
// IDA 0x4ed108: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed108() {
}

// 0x4ed118 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::isWriteOnly(void)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::isWriteOnly(void)const
// IDA 0x4ed118: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed118() {
}

// 0x4ed128 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x4ed128: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed128() {
}

// 0x4ed150 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x4ed150: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed150() {
}

// 0x4ed268 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x4ed268: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed268() {
}

// 0x4ed330 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x4ed330: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed330() {
}

// 0x4ed354 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// IDA 0x4ed354: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed354() {
}

// 0x4ed428 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// IDA 0x4ed428: 15 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed428() {
}

// 0x4ed44c — __ZNK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::getRefValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x4ed44c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed44c() {
}

// 0x4ed460 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE11setRefValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const
// IDA 0x4ed460: 41 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed460() {
}

// 0x4ed4d8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const
// IDA 0x4ed4d8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed4d8() {
}

// 0x4ed4f8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const
// IDA 0x4ed4f8: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed4f8() {
}

// 0x4ed5d8 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk to RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: non-virtual thunk to RBX::Reflection::RefPropDescriptor<RBX::VelocityMotor,RBX::Hole>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const
// IDA 0x4ed5d8: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed5d8() {
}

// 0x4ed5e0 — __ZNK3RBX10Reflection14PropDescriptorINS_13VelocityMotorEPNS_4HoleEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VelocityMotor,RBX::Hole *>::GetSetImpl<RBX::Hole * (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(RBX::Hole *)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::VelocityMotor,RBX::Hole *>::GetSetImpl<RBX::Hole * (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(RBX::Hole *)>::isReadOnly(void)const
// IDA 0x4ed5e0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed5e0() {
}

// 0x4ed5e4 — __ZNK3RBX10Reflection14PropDescriptorINS_13VelocityMotorEPNS_4HoleEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VelocityMotor,RBX::Hole *>::GetSetImpl<RBX::Hole * (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(RBX::Hole *)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::VelocityMotor,RBX::Hole *>::GetSetImpl<RBX::Hole * (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(RBX::Hole *)>::isWriteOnly(void)const
// IDA 0x4ed5e4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed5e4() {
}

// 0x4ed5e8 — __ZNK3RBX10Reflection14PropDescriptorINS_13VelocityMotorEPNS_4HoleEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VelocityMotor,RBX::Hole *>::GetSetImpl<RBX::Hole * (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(RBX::Hole *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::VelocityMotor,RBX::Hole *>::GetSetImpl<RBX::Hole * (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(RBX::Hole *)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x4ed5e8: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed5e8() {
}

// 0x4ed608 — __ZNK3RBX10Reflection14PropDescriptorINS_13VelocityMotorEPNS_4HoleEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VelocityMotor,RBX::Hole *>::GetSetImpl<RBX::Hole * (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(RBX::Hole *)>::setValue(RBX::Reflection::DescribedBase *,RBX::Hole * const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::VelocityMotor,RBX::Hole *>::GetSetImpl<RBX::Hole * (RBX::VelocityMotor::*)(void)const,void (RBX::VelocityMotor::*)(RBX::Hole *)>::setValue(RBX::Reflection::DescribedBase *,RBX::Hole * const&)const
// IDA 0x4ed608: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed608() {
}

// 0x4ed62c — __ZN3RBX10Reflection7RefTypeIPNS_4HoleEED1Ev
#[doc(alias = "RBX::Reflection::RefType<RBX::Hole *>::~RefType()")]
// was: RBX::Reflection::RefType<RBX::Hole *>::~RefType()
// IDA 0x4ed62c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4ed62c() {
}

// 0x4ed630 — __ZN3RBX10Reflection4TypeC2IPNS_4HoleEEEPKcS6_PT_
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Hole *>(char const*,char const*,RBX::Hole * *)")]
// was: RBX::Reflection::Type::Type<RBX::Hole *>(char const*,char const*,RBX::Hole * *)
// IDA 0x4ed630: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ed630() {
}

// 0x4ed6dc — __ZN3RBX10Reflection7RefTypeIPNS_4HoleEED0Ev
#[doc(alias = "RBX::Reflection::RefType<RBX::Hole *>::~RefType()")]
// was: RBX::Reflection::RefType<RBX::Hole *>::~RefType()
// IDA 0x4ed6dc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ed6dc() {
}

// 0x4ed6e0 — __GLOBAL__I_a_192
#[doc(alias = "global constructor keyed to_a_192")]
// was: global constructor keyed to _a_192
// IDA 0x4ed6e0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_4ed6e0() {
}

// 0x4edc88 — __ZN3RBX8FileMeshC1Ev
#[doc(alias = "RBX::FileMesh::FileMesh(void)")]
// was: RBX::FileMesh::FileMesh(void)
// IDA 0x4edc88: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4edc88() {
}

// 0x4edc8c — __ZN3RBX8FileMeshC2Ev
#[doc(alias = "RBX::FileMesh::FileMesh(void)")]
// was: RBX::FileMesh::FileMesh(void)
// IDA 0x4edc8c: 144 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4edc8c() {
}

// 0x4ede2c — __ZN3RBX8FileMesh9setMeshIdERKNS_6MeshIdE
#[doc(alias = "RBX::FileMesh::setMeshId(RBX::MeshId const&)")]
// was: RBX::FileMesh::setMeshId(RBX::MeshId const&)
// IDA 0x4ede2c: 21 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ede2c() {
}

// 0x4ede6c — __ZN3RBX8FileMesh12setTextureIdERKNS_9TextureIdE
#[doc(alias = "RBX::FileMesh::setTextureId(RBX::TextureId const&)")]
// was: RBX::FileMesh::setTextureId(RBX::TextureId const&)
// IDA 0x4ede6c: 21 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ede6c() {
}

// 0x4edeac — __ZNK3RBX8FileMesh9getMeshIdEv
#[doc(alias = "RBX::FileMesh::getMeshId(void)const")]
// was: RBX::FileMesh::getMeshId(void)const
// IDA 0x4edeac: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4edeac() {
}

// 0x4edeb0 — __ZN3RBX10Reflection14PropDescriptorINS_8FileMeshENS_6MeshIdEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::~PropDescriptor()
// IDA 0x4edeb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4edeb0() {
}

// 0x4eded4 — __ZNK3RBX8FileMesh12getTextureIdEv
#[doc(alias = "RBX::FileMesh::getTextureId(void)const")]
// was: RBX::FileMesh::getTextureId(void)const
// IDA 0x4eded4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4eded4() {
}

// 0x4eded8 — __ZN3RBX10Reflection14PropDescriptorINS_8FileMeshENS_9TextureIdEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::~PropDescriptor()
// IDA 0x4eded8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4eded8() {
}

// 0x4edefc — __ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4edefc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4edefc() {
}

// 0x4edf00 — __ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4edf00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4edf00() {
}

// 0x4edfa0 — __ZThn32_N3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4edfa0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4edfa0() {
}

// 0x4edfa8 — __ZThn32_N3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4edfa8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4edfa8() {
}

// 0x4ee04c — __ZThn36_N3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4ee04c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4ee04c() {
}

// 0x4ee054 — __ZThn36_N3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4ee054: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4ee054() {
}

// 0x4ee0f8 — __ZN3RBX10Reflection14PropDescriptorINS_8FileMeshENS_9TextureIdEEC2IMS2_KFRKS3_vEMS2_FvS7_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::PropDescriptor<RBX::TextureId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::TextureId const&)>(char const*,char const*,RBX::TextureId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::TextureId const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::PropDescriptor<RBX::TextureId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::TextureId const&)>(char const*,char const*,RBX::TextureId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::TextureId const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x4ee0f8: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee0f8() {
}

// 0x4ee20c — __ZN3RBX10Reflection14PropDescriptorINS_8FileMeshENS_9TextureIdEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::~PropDescriptor()
// IDA 0x4ee20c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4ee20c() {
}

// 0x4ee238 — __ZNK3RBX10Reflection14PropDescriptorINS_8FileMeshENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS7_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::TextureId const&)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::TextureId const&)>::isReadOnly(void)const
// IDA 0x4ee238: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee238() {
}

// 0x4ee23c — __ZNK3RBX10Reflection14PropDescriptorINS_8FileMeshENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS7_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::TextureId const&)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::TextureId const&)>::isWriteOnly(void)const
// IDA 0x4ee23c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee23c() {
}

// 0x4ee240 — __ZNK3RBX10Reflection14PropDescriptorINS_8FileMeshENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS7_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::TextureId const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::TextureId const&)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x4ee240: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee240() {
}

// 0x4ee274 — __ZNK3RBX10Reflection14PropDescriptorINS_8FileMeshENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS7_EE8setValueEPNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::TextureId const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::TextureId const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const
// IDA 0x4ee274: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee274() {
}

// 0x4ee298 — __ZN3RBX10Reflection14PropDescriptorINS_8FileMeshENS_6MeshIdEEC2IMS2_KFRKS3_vEMS2_FvS7_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::PropDescriptor<RBX::MeshId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::MeshId const&)>(char const*,char const*,RBX::MeshId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::MeshId const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::PropDescriptor<RBX::MeshId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::MeshId const&)>(char const*,char const*,RBX::MeshId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::MeshId const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x4ee298: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee298() {
}

// 0x4ee3ac — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_6MeshIdEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x4ee3ac: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee3ac() {
}

// 0x4ee4d0 — __ZN3RBX10Reflection14PropDescriptorINS_8FileMeshENS_6MeshIdEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::~PropDescriptor()
// IDA 0x4ee4d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4ee4d0() {
}

// 0x4ee4fc — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6MeshIdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::isReadOnly(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::isReadOnly(void)const
// IDA 0x4ee4fc: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee4fc() {
}

// 0x4ee50c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6MeshIdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::isWriteOnly(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::isWriteOnly(void)const
// IDA 0x4ee50c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee50c() {
}

// 0x4ee51c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6MeshIdEE11equalValuesEPKNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x4ee51c: 150 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee51c() {
}

// 0x4ee6c8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6MeshIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x4ee6c8: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee6c8() {
}

// 0x4ee7f4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6MeshIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x4ee7f4: 187 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee7f4() {
}

// 0x4ee9f0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_6MeshIdEE9copyValueEPKNS0_13DescribedBaseEPS4_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x4ee9f0: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ee9f0() {
}

// 0x4eeb18 — __ZN3rbx8any_castIRKN3RBX6MeshIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::MeshId const& rbx::any_cast<RBX::MeshId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::MeshId const& rbx::any_cast<RBX::MeshId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4eeb18: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4eeb18() {
}

// 0x4eec08 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_6MeshIdEED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::~TypedPropertyDescriptor()
// IDA 0x4eec08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4eec08() {
}

// 0x4eec2c — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_6MeshIdEED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::MeshId>::~TypedPropertyDescriptor()
// IDA 0x4eec2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4eec2c() {
}

// 0x4eec58 — __ZNK3RBX10Reflection14PropDescriptorINS_8FileMeshENS_6MeshIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS7_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::GetSetImpl<RBX::MeshId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::MeshId const&)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::GetSetImpl<RBX::MeshId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::MeshId const&)>::isReadOnly(void)const
// IDA 0x4eec58: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4eec58() {
}

// 0x4eec5c — __ZNK3RBX10Reflection14PropDescriptorINS_8FileMeshENS_6MeshIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS7_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::GetSetImpl<RBX::MeshId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::MeshId const&)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::GetSetImpl<RBX::MeshId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::MeshId const&)>::isWriteOnly(void)const
// IDA 0x4eec5c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4eec5c() {
}

// 0x4eec60 — __ZNK3RBX10Reflection14PropDescriptorINS_8FileMeshENS_6MeshIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS7_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::GetSetImpl<RBX::MeshId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::MeshId const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::GetSetImpl<RBX::MeshId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::MeshId const&)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x4eec60: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4eec60() {
}

// 0x4eec94 — __ZNK3RBX10Reflection14PropDescriptorINS_8FileMeshENS_6MeshIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS7_EE8setValueEPNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::GetSetImpl<RBX::MeshId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::MeshId const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::MeshId const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FileMesh,RBX::MeshId>::GetSetImpl<RBX::MeshId const& (RBX::FileMesh::*)(void)const,void (RBX::FileMesh::*)(RBX::MeshId const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::MeshId const&)const
// IDA 0x4eec94: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4eec94() {
}

// 0x4eecb8 — __GLOBAL__I_a_193
#[doc(alias = "global constructor keyed to_a_193")]
// was: global constructor keyed to _a_193
// IDA 0x4eecb8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_4eecb8() {
}

// 0x4eef38 — __ZN3RBX27FilterInvisibleNonCollidingC1Ev
#[doc(alias = "RBX::FilterInvisibleNonColliding::FilterInvisibleNonColliding(void)")]
// was: RBX::FilterInvisibleNonColliding::FilterInvisibleNonColliding(void)
// IDA 0x4eef38: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4eef38() {
}

// 0x4eef48 — __ZNK3RBX27FilterInvisibleNonColliding12filterResultEPKNS_9PrimitiveE
#[doc(alias = "RBX::FilterInvisibleNonColliding::filterResult(RBX::Primitive const*)const")]
// was: RBX::FilterInvisibleNonColliding::filterResult(RBX::Primitive const*)const
// IDA 0x4eef48: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4eef48() {
}

// 0x4eef84 — __ZN3RBX20PartByLocalCharacterC1EPNS_8InstanceE
#[doc(alias = "RBX::PartByLocalCharacter::PartByLocalCharacter(RBX::Instance *)")]
// was: RBX::PartByLocalCharacter::PartByLocalCharacter(RBX::Instance *)
// IDA 0x4eef84: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4eef84() {
}

// 0x4eef88 — __ZN3RBX20PartByLocalCharacterC2EPNS_8InstanceE
#[doc(alias = "RBX::PartByLocalCharacter::PartByLocalCharacter(RBX::Instance *)")]
// was: RBX::PartByLocalCharacter::PartByLocalCharacter(RBX::Instance *)
// IDA 0x4eef88: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4eef88() {
}

// 0x4ef0f4 — __ZNK3RBX20PartByLocalCharacter12filterResultEPKNS_9PrimitiveE
#[doc(alias = "RBX::PartByLocalCharacter::filterResult(RBX::Primitive const*)const")]
// was: RBX::PartByLocalCharacter::filterResult(RBX::Primitive const*)const
// IDA 0x4ef0f4: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef0f4() {
}

// 0x4ef164 — __ZNK3RBX28UnlockedPartByLocalCharacter12filterResultEPKNS_9PrimitiveE
#[doc(alias = "RBX::UnlockedPartByLocalCharacter::filterResult(RBX::Primitive const*)const")]
// was: RBX::UnlockedPartByLocalCharacter::filterResult(RBX::Primitive const*)const
// IDA 0x4ef164: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef164() {
}

// 0x4ef18c — __ZN3RBX17FilterDescendentsC1EN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::FilterDescendents::FilterDescendents(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::FilterDescendents::FilterDescendents(boost::shared_ptr<RBX::Instance>)
// IDA 0x4ef18c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ef18c() {
}

// 0x4ef190 — __ZN3RBX17FilterDescendentsC2EN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::FilterDescendents::FilterDescendents(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::FilterDescendents::FilterDescendents(boost::shared_ptr<RBX::Instance>)
// IDA 0x4ef190: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef190() {
}

// 0x4ef260 — __ZNK3RBX17FilterDescendents12filterResultEPKNS_9PrimitiveE
#[doc(alias = "RBX::FilterDescendents::filterResult(RBX::Primitive const*)const")]
// was: RBX::FilterDescendents::filterResult(RBX::Primitive const*)const
// IDA 0x4ef260: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef260() {
}

// 0x4ef28c — __ZN3RBX21FilterDescendentsListC1EPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EE
#[doc(alias = "RBX::FilterDescendentsList::FilterDescendentsList(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const*)")]
// was: RBX::FilterDescendentsList::FilterDescendentsList(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const*)
// IDA 0x4ef28c: 6 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef28c() {
}

// 0x4ef2a0 — __ZNK3RBX21FilterDescendentsList12filterResultEPKNS_9PrimitiveE
#[doc(alias = "RBX::FilterDescendentsList::filterResult(RBX::Primitive const*)const")]
// was: RBX::FilterDescendentsList::filterResult(RBX::Primitive const*)const
// IDA 0x4ef2a0: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef2a0() {
}

// 0x4ef2e0 — __ZN3RBX24FilterCharacterOcclusionC1Ef
#[doc(alias = "RBX::FilterCharacterOcclusion::FilterCharacterOcclusion(float)")]
// was: RBX::FilterCharacterOcclusion::FilterCharacterOcclusion(float)
// IDA 0x4ef2e0: 6 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef2e0() {
}

// 0x4ef2f4 — __ZNK3RBX24FilterCharacterOcclusion12filterResultEPKNS_9PrimitiveE
#[doc(alias = "RBX::FilterCharacterOcclusion::filterResult(RBX::Primitive const*)const")]
// was: RBX::FilterCharacterOcclusion::filterResult(RBX::Primitive const*)const
// IDA 0x4ef2f4: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef2f4() {
}

// 0x4ef388 — __ZNK3RBX19FilterHumanoidParts12filterResultEPKNS_9PrimitiveE
#[doc(alias = "RBX::FilterHumanoidParts::filterResult(RBX::Primitive const*)const")]
// was: RBX::FilterHumanoidParts::filterResult(RBX::Primitive const*)const
// IDA 0x4ef388: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef388() {
}

// 0x4ef3a4 — __ZN3RBX12MergedFilterC1EPKNS_13HitTestFilterES3_
#[doc(alias = "RBX::MergedFilter::MergedFilter(RBX::HitTestFilter const*,RBX::HitTestFilter const*)")]
// was: RBX::MergedFilter::MergedFilter(RBX::HitTestFilter const*,RBX::HitTestFilter const*)
// IDA 0x4ef3a4: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef3a4() {
}

// 0x4ef3b8 — __ZNK3RBX12MergedFilter12filterResultEPKNS_9PrimitiveE
#[doc(alias = "RBX::MergedFilter::filterResult(RBX::Primitive const*)const")]
// was: RBX::MergedFilter::filterResult(RBX::Primitive const*)const
// IDA 0x4ef3b8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef3b8() {
}

// 0x4ef3ec — __ZN5boost10shared_ptrIN3RBX13ModelInstanceEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::ModelInstance>::operator=(rbx_core::SharedPtr<RBX::ModelInstance> const&)")]
// was: boost::shared_ptr<RBX::ModelInstance>::operator=(boost::shared_ptr<RBX::ModelInstance> const&)
// IDA 0x4ef3ec: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef3ec() {
}

// 0x4ef424 — __ZN3RBX27FilterInvisibleNonCollidingD1Ev
#[doc(alias = "RBX::FilterInvisibleNonColliding::~FilterInvisibleNonColliding()")]
// was: RBX::FilterInvisibleNonColliding::~FilterInvisibleNonColliding()
// IDA 0x4ef424: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4ef424() {
}

// 0x4ef428 — __ZN3RBX27FilterInvisibleNonCollidingD0Ev
#[doc(alias = "RBX::FilterInvisibleNonColliding::~FilterInvisibleNonColliding()")]
// was: RBX::FilterInvisibleNonColliding::~FilterInvisibleNonColliding()
// IDA 0x4ef428: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ef428() {
}

// 0x4ef42c — __ZN3RBX21FilterDescendentsListD1Ev
#[doc(alias = "RBX::FilterDescendentsList::~FilterDescendentsList()")]
// was: RBX::FilterDescendentsList::~FilterDescendentsList()
// IDA 0x4ef42c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4ef42c() {
}

// 0x4ef430 — __ZN3RBX21FilterDescendentsListD0Ev
#[doc(alias = "RBX::FilterDescendentsList::~FilterDescendentsList()")]
// was: RBX::FilterDescendentsList::~FilterDescendentsList()
// IDA 0x4ef430: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ef430() {
}

// 0x4ef434 — __ZN3RBX24FilterCharacterOcclusionD1Ev
#[doc(alias = "RBX::FilterCharacterOcclusion::~FilterCharacterOcclusion()")]
// was: RBX::FilterCharacterOcclusion::~FilterCharacterOcclusion()
// IDA 0x4ef434: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4ef434() {
}

// 0x4ef438 — __ZN3RBX24FilterCharacterOcclusionD0Ev
#[doc(alias = "RBX::FilterCharacterOcclusion::~FilterCharacterOcclusion()")]
// was: RBX::FilterCharacterOcclusion::~FilterCharacterOcclusion()
// IDA 0x4ef438: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ef438() {
}

// 0x4ef43c — __ZN3RBX12MergedFilterD1Ev
#[doc(alias = "RBX::MergedFilter::~MergedFilter()")]
// was: RBX::MergedFilter::~MergedFilter()
// IDA 0x4ef43c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4ef43c() {
}

// 0x4ef440 — __ZN3RBX12MergedFilterD0Ev
#[doc(alias = "RBX::MergedFilter::~MergedFilter()")]
// was: RBX::MergedFilter::~MergedFilter()
// IDA 0x4ef440: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ef440() {
}

// 0x4ef444 — __ZN3RBX19FilterHumanoidPartsD1Ev
#[doc(alias = "RBX::FilterHumanoidParts::~FilterHumanoidParts()")]
// was: RBX::FilterHumanoidParts::~FilterHumanoidParts()
// IDA 0x4ef444: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4ef444() {
}

// 0x4ef448 — __ZN3RBX19FilterHumanoidPartsD0Ev
#[doc(alias = "RBX::FilterHumanoidParts::~FilterHumanoidParts()")]
// was: RBX::FilterHumanoidParts::~FilterHumanoidParts()
// IDA 0x4ef448: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ef448() {
}

// 0x4ef44c — __GLOBAL__I_a_194
#[doc(alias = "global constructor keyed to_a_194")]
// was: global constructor keyed to _a_194
// IDA 0x4ef44c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_4ef44c() {
}

// 0x4ef7c0 — __ZN3RBX4Fire9setSizeUiEf
#[doc(alias = "RBX::Fire::setSizeUi(float)")]
// was: RBX::Fire::setSizeUi(float)
// IDA 0x4ef7c0: 21 insns (VMOV.F32..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef7c0() {
}

// 0x4ef80c — __ZN3RBX4Fire9setHeatUiEf
#[doc(alias = "RBX::Fire::setHeatUi(float)")]
// was: RBX::Fire::setHeatUi(float)
// IDA 0x4ef80c: 21 insns (VMOV.F32..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef80c() {
}

// 0x4ef858 — __ZN3RBX4Fire7setSizeEf
#[doc(alias = "RBX::Fire::setSize(float)")]
// was: RBX::Fire::setSize(float)
// IDA 0x4ef858: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef858() {
}

// 0x4ef898 — __ZN3RBX4Fire7setHeatEf
#[doc(alias = "RBX::Fire::setHeat(float)")]
// was: RBX::Fire::setHeat(float)
// IDA 0x4ef898: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef898() {
}

// 0x4ef8d8 — __ZN3RBX4FireC2Ev
#[doc(alias = "RBX::Fire::Fire(void)")]
// was: RBX::Fire::Fire(void)
// IDA 0x4ef8d8: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef8d8() {
}

// 0x4efaf4 — __ZN3RBX4FireD0Ev
#[doc(alias = "RBX::Fire::~Fire()")]
// was: RBX::Fire::~Fire()
// IDA 0x4efaf4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4efaf4() {
}

// 0x4efb94 — __ZN3RBX4FireD1Ev
#[doc(alias = "RBX::Fire::~Fire()")]
// was: RBX::Fire::~Fire()
// IDA 0x4efb94: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4efb94() {
}

// 0x4efb98 — __ZThn32_N3RBX4FireD0Ev
#[doc(alias = "non-virtual thunk to RBX::Fire::~Fire()")]
// was: non-virtual thunk to RBX::Fire::~Fire()
// IDA 0x4efb98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4efb98() {
}

// 0x4efba0 — __ZThn36_N3RBX4FireD0Ev
#[doc(alias = "non-virtual thunk to RBX::Fire::~Fire()")]
// was: non-virtual thunk to RBX::Fire::~Fire()
// IDA 0x4efba0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4efba0() {
}