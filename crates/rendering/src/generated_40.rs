//! rendering generated_40 — Ogre::|G3D:: strict 13333 total (13663 substr Ogre|G3D), 5128 prior, 120 this batch — 0x816ab0..0x882d24
//! EA-sorted ascending earliest gap after 0x816aaf (next after 0x816aaf); rbx_core::SharedPtr not boost
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x816ab0 — __ZNK3RBX7Region28containsERKN3G3D7Vector2Ef
#[doc(alias = "RBX::Region2::contains(G3D::Vector2 const&,float)const")]
// was: RBX::Region2::contains(G3D::Vector2 const&,float)const
// IDA 0x816ab0: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_816ab0() {
}

// 0x816b04 — __ZNK3RBX7Region215findCloserOtherERKN3G3D7Vector2Ef
#[doc(alias = "RBX::Region2::findCloserOther(G3D::Vector2 const&,float)const")]
// was: RBX::Region2::findCloserOther(G3D::Vector2 const&,float)const
// IDA 0x816b04: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_816b04() {
}

// 0x816b54 — __ZN3RBX7Region218closerToOtherPointERKN3G3D7Vector2ERKNS0_13WeightedPointES7_f
#[doc(alias = "RBX::Region2::closerToOtherPoint(G3D::Vector2 const&,RBX::Region2::WeightedPoint const&,RBX::Region2::WeightedPoint const&,float)")]
// was: RBX::Region2::closerToOtherPoint(G3D::Vector2 const&,RBX::Region2::WeightedPoint const&,RBX::Region2::WeightedPoint const&,float)
// IDA 0x816b54: 43 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_816b54() {
}

// 0x816d20 — __ZN3RBX7Region3C1ERKN3G3D7Vector3ES4_
#[doc(alias = "RBX::Region3::Region3(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: RBX::Region3::Region3(G3D::Vector3 const&,G3D::Vector3 const&)
// IDA 0x816d20: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_816d20() {
}

// 0x85df88 — __ZN3RBX12TextureTrail14setTextureSizeEN3G3D7Vector2E
#[doc(alias = "RBX::TextureTrail::setTextureSize(G3D::Vector2)")]
// was: RBX::TextureTrail::setTextureSize(G3D::Vector2)
// IDA 0x85df88: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_85df88() {
}

// 0x85e380 — __ZN3RBX12TextureTrail11getPositionERKN5boost8weak_ptrINS_12PartInstanceEEEPN3G3D7Vector3E
#[doc(alias = "RBX::TextureTrail::getPosition(rbx_core::WeakPtr<RBX::PartInstance> const&,G3D::Vector3 *)")]
// was: RBX::TextureTrail::getPosition(boost::weak_ptr<RBX::PartInstance> const&,G3D::Vector3 *)
// IDA 0x85e380: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_85e380() {
}

// 0x85e3bc — __ZN3RBX12TextureTrail14renderInternalEPKNS_9WorkspaceEPKNS_6CameraERKN3G3D7Vector3ESA_RKNS_9TextureIdERKNS7_7Vector2EfffRKNS7_6Color4EPNS_5AdornE
#[doc(alias = "RBX::TextureTrail::renderInternal(RBX::Workspace const*,RBX::Camera const*,G3D::Vector3 const&,G3D::Vector3 const&,RBX::TextureId const&,G3D::Vector2 const&,float,float,float,G3D::Color4 const&,RBX::Adorn *)")]
// was: RBX::TextureTrail::renderInternal(RBX::Workspace const*,RBX::Camera const*,G3D::Vector3 const&,G3D::Vector3 const&,RBX::TextureId const&,G3D::Vector2 const&,float,float,float,G3D::Color4 const&,RBX::Adorn *)
// IDA 0x85e3bc: 369 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_85e3bc() {
}

// 0x85e848 — __ZN3RBX10Reflection14PropDescriptorINS_12TextureTrailEN3G3D7Vector2EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::~PropDescriptor()
// IDA 0x85e848: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_85e848() {
}

// 0x85fde4 — __ZN3RBX10Reflection14PropDescriptorINS_12TextureTrailEN3G3D7Vector2EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x85fde4: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_85fde4() {
}

// 0x85fef8 — __ZN3RBX10Reflection14PropDescriptorINS_12TextureTrailEN3G3D7Vector2EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::~PropDescriptor()
// IDA 0x85fef8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_85fef8() {
}

// 0x85ff24 — __ZNK3RBX10Reflection14PropDescriptorINS_12TextureTrailEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(G3D::Vector2)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(G3D::Vector2)>::isReadOnly(void)const
// IDA 0x85ff24: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_85ff24() {
}

// 0x85ff28 — __ZNK3RBX10Reflection14PropDescriptorINS_12TextureTrailEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(G3D::Vector2)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(G3D::Vector2)>::isWriteOnly(void)const
// IDA 0x85ff28: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_85ff28() {
}

// 0x85ff2c — __ZNK3RBX10Reflection14PropDescriptorINS_12TextureTrailEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x85ff2c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_85ff2c() {
}

// 0x85ff54 — __ZNK3RBX10Reflection14PropDescriptorINS_12TextureTrailEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::TextureTrail,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const
// IDA 0x85ff54: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_85ff54() {
}

// 0x867a90 — __ZN3RBX9FloorWire14setTextureSizeEN3G3D7Vector2E
#[doc(alias = "RBX::FloorWire::setTextureSize(G3D::Vector2)")]
// was: RBX::FloorWire::setTextureSize(G3D::Vector2)
// IDA 0x867a90: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_867a90() {
}

// 0x867f68 — __ZN3RBX9FloorWire18buildTrailSegmentsEPKNS_9WorkspaceERKN5boost10shared_ptrINS_12PartInstanceEEES9_PSt6vectorIN3G3D7Vector3ESaISC_EE
#[doc(alias = "RBX::FloorWire::buildTrailSegments(RBX::Workspace const*,rbx_core::SharedPtr<RBX::PartInstance> const&,rbx_core::SharedPtr<RBX::PartInstance> const&,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> *)")]
// was: RBX::FloorWire::buildTrailSegments(RBX::Workspace const*,boost::shared_ptr<RBX::PartInstance> const&,boost::shared_ptr<RBX::PartInstance> const&,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> *)
// IDA 0x867f68: 266 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_867f68() {
}

// 0x868290 — __ZN3RBX9FloorWire12drawSegmentsEPKNS_9WorkspaceEPKNS_6CameraERKSt6vectorIN3G3D7Vector3ESaIS9_EEPNS_5AdornE
#[doc(alias = "RBX::FloorWire::drawSegments(RBX::Workspace const*,RBX::Camera const*,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&,RBX::Adorn *)")]
// was: RBX::FloorWire::drawSegments(RBX::Workspace const*,RBX::Camera const*,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&,RBX::Adorn *)
// IDA 0x868290: 279 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_868290() {
}

// 0x8685e0 — __ZN3RBX9FloorWire22computeSurfacePositionERKN5boost10shared_ptrINS_12PartInstanceEEERKNS_6RbxRayEPN3G3D7Vector3E
#[doc(alias = "RBX::FloorWire::computeSurfacePosition(rbx_core::SharedPtr<RBX::PartInstance> const&,RBX::RbxRay const&,G3D::Vector3 *)")]
// was: RBX::FloorWire::computeSurfacePosition(boost::shared_ptr<RBX::PartInstance> const&,RBX::RbxRay const&,G3D::Vector3 *)
// IDA 0x8685e0: 107 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8685e0() {
}

// 0x868764 — __ZN3RBX9FloorWire24incrementalBuildSegmentsEPKNS_9WorkspaceEPKNS_14ContactManagerERKN3G3D7Vector3EbPSt6vectorIS8_SaIS8_EE
#[doc(alias = "RBX::FloorWire::incrementalBuildSegments(RBX::Workspace const*,RBX::ContactManager const*,G3D::Vector3 const&,bool,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> *)")]
// was: RBX::FloorWire::incrementalBuildSegments(RBX::Workspace const*,RBX::ContactManager const*,G3D::Vector3 const&,bool,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> *)
// IDA 0x868764: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_868764() {
}

// 0x869fc8 — __ZN3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x869fc8: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_869fc8() {
}

// 0x86a108 — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>::isReadOnly(void)const
// IDA 0x86a108: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86a108() {
}

// 0x86a10c — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>::isWriteOnly(void)const
// IDA 0x86a10c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86a10c() {
}

// 0x86a110 — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x86a110: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86a110() {
}

// 0x86a138 — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const
// IDA 0x86a138: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86a138() {
}

// 0x86ca80 — __ZN3RBX19MegaClusterInstance28worldToCellPreferSolidScriptEN3G3D7Vector3E
#[doc(alias = "RBX::MegaClusterInstance::worldToCellPreferSolidScript(G3D::Vector3)")]
// was: RBX::MegaClusterInstance::worldToCellPreferSolidScript(G3D::Vector3)
// IDA 0x86ca80: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86ca80() {
}

// 0x86caec — __ZN3RBX19MegaClusterInstance28worldToCellPreferEmptyScriptEN3G3D7Vector3E
#[doc(alias = "RBX::MegaClusterInstance::worldToCellPreferEmptyScript(G3D::Vector3)")]
// was: RBX::MegaClusterInstance::worldToCellPreferEmptyScript(G3D::Vector3)
// IDA 0x86caec: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86caec() {
}

// 0x86cb58 — __ZN3RBX19MegaClusterInstance17worldToCellScriptEN3G3D7Vector3E
#[doc(alias = "RBX::MegaClusterInstance::worldToCellScript(G3D::Vector3)")]
// was: RBX::MegaClusterInstance::worldToCellScript(G3D::Vector3)
// IDA 0x86cb58: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86cb58() {
}

// 0x86cc6c — __ZNK3RBX19MegaClusterInstance9CellChunk15readCellOrEmptyERKN3G3D12Vector3int16E
#[doc(alias = "RBX::MegaClusterInstance::CellChunk::readCellOrEmpty(G3D::Vector3int16 const&)const")]
// was: RBX::MegaClusterInstance::CellChunk::readCellOrEmpty(G3D::Vector3int16 const&)const
// IDA 0x86cc6c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86cc6c() {
}

// 0x86ccc8 — __ZNK3RBX19MegaClusterInstance9CellChunk17fillLocalAreaInfoERKN3G3D12Vector3int16ERKNS_5Voxel5Water17RelevantNeighborsEPNS7_13LocalAreaInfoE
#[doc(alias = "RBX::MegaClusterInstance::CellChunk::fillLocalAreaInfo(G3D::Vector3int16 const&,RBX::Voxel::Water::RelevantNeighbors const&,RBX::Voxel::Water::LocalAreaInfo *)const")]
// was: RBX::MegaClusterInstance::CellChunk::fillLocalAreaInfo(G3D::Vector3int16 const&,RBX::Voxel::Water::RelevantNeighbors const&,RBX::Voxel::Water::LocalAreaInfo *)const
// IDA 0x86ccc8: 96 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86ccc8() {
}

// 0x86de70 — __ZN3RBX19MegaClusterInstance14setPartSizeXmlERKN3G3D7Vector3E
#[doc(alias = "RBX::MegaClusterInstance::setPartSizeXml(G3D::Vector3 const&)")]
// was: RBX::MegaClusterInstance::setPartSizeXml(G3D::Vector3 const&)
// IDA 0x86de70: 14 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86de70() {
}

// 0x86dfb8 — __ZN3RBX19MegaClusterInstance13setPartSizeUiERKN3G3D7Vector3E
#[doc(alias = "RBX::MegaClusterInstance::setPartSizeUi(G3D::Vector3 const&)")]
// was: RBX::MegaClusterInstance::setPartSizeUi(G3D::Vector3 const&)
// IDA 0x86dfb8: 14 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86dfb8() {
}

// 0x86dfe0 — __ZN3RBX19MegaClusterInstance16setTranslationUiERKN3G3D7Vector3E
#[doc(alias = "RBX::MegaClusterInstance::setTranslationUi(G3D::Vector3 const&)")]
// was: RBX::MegaClusterInstance::setTranslationUi(G3D::Vector3 const&)
// IDA 0x86dfe0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_86dfe0() {
}

// 0x86dfe4 — __ZN3RBX19MegaClusterInstance18setCoordinateFrameERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::MegaClusterInstance::setCoordinateFrame(G3D::CoordinateFrame const&)")]
// was: RBX::MegaClusterInstance::setCoordinateFrame(G3D::CoordinateFrame const&)
// IDA 0x86dfe4: 2 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86dfe4() {
}

// 0x86e2b0 — __ZNK3RBX19MegaClusterInstance15getCellInternalERKN3G3D12Vector3int16E
#[doc(alias = "RBX::MegaClusterInstance::getCellInternal(G3D::Vector3int16 const&)const")]
// was: RBX::MegaClusterInstance::getCellInternal(G3D::Vector3int16 const&)const
// IDA 0x86e2b0: 9 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86e2b0() {
}

// 0x86e2d0 — __ZNK3RBX19MegaClusterInstance20getWaterCellInternalERKN3G3D12Vector3int16E
#[doc(alias = "RBX::MegaClusterInstance::getWaterCellInternal(G3D::Vector3int16 const&)const")]
// was: RBX::MegaClusterInstance::getWaterCellInternal(G3D::Vector3int16 const&)const
// IDA 0x86e2d0: 115 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86e2d0() {
}

// 0x86e53c — __ZNK3RBX19MegaClusterInstance49getCellInternal_alwaysReadFromMegaClusterInstanceERKN3G3D12Vector3int16E
#[doc(alias = "RBX::MegaClusterInstance::getCellInternal_alwaysReadFromMegaClusterInstance(G3D::Vector3int16 const&)const")]
// was: RBX::MegaClusterInstance::getCellInternal_alwaysReadFromMegaClusterInstance(G3D::Vector3int16 const&)const
// IDA 0x86e53c: 80 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86e53c() {
}

// 0x86e630 — __ZN3RBX19MegaClusterInstance15setCellInternalERKN3G3D12Vector3int16ERKNS_5Voxel4CellERKNS5_12CellMaterialEb
#[doc(alias = "RBX::MegaClusterInstance::setCellInternal(G3D::Vector3int16 const&,RBX::Voxel::Cell const&,RBX::Voxel::CellMaterial const&,bool)")]
// was: RBX::MegaClusterInstance::setCellInternal(G3D::Vector3int16 const&,RBX::Voxel::Cell const&,RBX::Voxel::CellMaterial const&,bool)
// IDA 0x86e630: 254 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86e630() {
}

// 0x86e9c4 — __ZNK3RBX19MegaClusterInstance12getChunkDataERKN3G3D12Vector3int16E
#[doc(alias = "RBX::MegaClusterInstance::getChunkData(G3D::Vector3int16 const&)const")]
// was: RBX::MegaClusterInstance::getChunkData(G3D::Vector3int16 const&)const
// IDA 0x86e9c4: 188 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86e9c4() {
}

// 0x86ec1c — __ZNK3RBX19MegaClusterInstance9getRegionERKN3G3D12Vector3int16ES4_
#[doc(alias = "RBX::MegaClusterInstance::getRegion(G3D::Vector3int16 const&,G3D::Vector3int16 const&)const")]
// was: RBX::MegaClusterInstance::getRegion(G3D::Vector3int16 const&,G3D::Vector3int16 const&)const
// IDA 0x86ec1c: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86ec1c() {
}

// 0x86f164 — __ZN3RBX19MegaClusterInstance28setCellInternalV1_DeprecatedERKN3G3D12Vector3int16ERKh
#[doc(alias = "RBX::MegaClusterInstance::setCellInternalV1_Deprecated(G3D::Vector3int16 const&,unsigned char const&)")]
// was: RBX::MegaClusterInstance::setCellInternalV1_Deprecated(G3D::Vector3int16 const&,unsigned char const&)
// IDA 0x86f164: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86f164() {
}

// 0x86f1ac — __ZN3RBX12MovePositionERN3G3D12Vector3int16ENS_5Voxel13FaceDirectionE
#[doc(alias = "RBX::MovePosition(G3D::Vector3int16 &,RBX::Voxel::FaceDirection)")]
// was: RBX::MovePosition(G3D::Vector3int16 &,RBX::Voxel::FaceDirection)
// IDA 0x86f1ac: 36 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86f1ac() {
}

// 0x86f204 — __ZN3RBX19MegaClusterInstance25worldToCellWithPreferenceERKN3G3D7Vector3Eb
#[doc(alias = "RBX::MegaClusterInstance::worldToCellWithPreference(G3D::Vector3 const&,bool)")]
// was: RBX::MegaClusterInstance::worldToCellWithPreference(G3D::Vector3 const&,bool)
// IDA 0x86f204: 324 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86f204() {
}

// 0x86f578 — __ZNK3RBX19MegaClusterInstance19worldToCellInternalEN3G3D7Vector3E
#[doc(alias = "RBX::MegaClusterInstance::worldToCellInternal(G3D::Vector3)const")]
// was: RBX::MegaClusterInstance::worldToCellInternal(G3D::Vector3)const
// IDA 0x86f578: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86f578() {
}

// 0x86f668 — __ZN3RBX19MegaClusterInstance18cellToWorldExtentsERKN3G3D12Vector3int16E
#[doc(alias = "RBX::MegaClusterInstance::cellToWorldExtents(G3D::Vector3int16 const&)")]
// was: RBX::MegaClusterInstance::cellToWorldExtents(G3D::Vector3int16 const&)
// IDA 0x86f668: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86f668() {
}

// 0x86f79c — __ZNK3RBX19MegaClusterInstance17cellCornerToWorldERKN3G3D12Vector3int16E
#[doc(alias = "RBX::MegaClusterInstance::cellCornerToWorld(G3D::Vector3int16 const&)const")]
// was: RBX::MegaClusterInstance::cellCornerToWorld(G3D::Vector3int16 const&)const
// IDA 0x86f79c: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86f79c() {
}

// 0x871508 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::~BoundFuncDesc()
// IDA 0x871508: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_871508() {
}

// 0x87155c — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::~BoundFuncDesc()
// IDA 0x87155c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_87155c() {
}

// 0x87164c — __ZNK3G3D12Vector3int1618isBetweenInclusiveERKS0_S2_
#[doc(alias = "G3D::Vector3int16::isBetweenInclusive(G3D::Vector3int16 const&,G3D::Vector3int16 const&)const")]
// was: G3D::Vector3int16::isBetweenInclusive(G3D::Vector3int16 const&,G3D::Vector3int16 const&)const
// IDA 0x87164c: 36 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87164c() {
}

// 0x871c4c — __ZN3RBX19MegaClusterInstance38decodeChunkDataFromStreamV1_DeprecatedINS_16StringReadBufferEEEvRKN3G3D12Vector3int16ERT_
#[doc(alias = "void RBX::MegaClusterInstance::decodeChunkDataFromStreamV1_Deprecated<RBX::StringReadBuffer>(G3D::Vector3int16 const&,RBX::StringReadBuffer &)")]
// was: void RBX::MegaClusterInstance::decodeChunkDataFromStreamV1_Deprecated<RBX::StringReadBuffer>(G3D::Vector3int16 const&,RBX::StringReadBuffer &)
// IDA 0x871c4c: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_871c4c() {
}

// 0x871d50 — __ZN3RBX19MegaClusterInstance25decodeChunkDataFromStreamINS_16StringReadBufferEEEvRKN3G3D12Vector3int16ERT_
#[doc(alias = "void RBX::MegaClusterInstance::decodeChunkDataFromStream<RBX::StringReadBuffer>(G3D::Vector3int16 const&,RBX::StringReadBuffer &)")]
// was: void RBX::MegaClusterInstance::decodeChunkDataFromStream<RBX::StringReadBuffer>(G3D::Vector3int16 const&,RBX::StringReadBuffer &)
// IDA 0x871d50: 362 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_871d50() {
}

// 0x87211c — __ZN3RBX5Voxel5Water12cellHasWaterINS_19MegaClusterInstance9CellChunkEEEbPKT_RKNS0_4CellERKN3G3D12Vector3int16E
#[doc(alias = "bool RBX::Voxel::Water::cellHasWater<RBX::MegaClusterInstance::CellChunk>(RBX::MegaClusterInstance::CellChunk const*,RBX::Voxel::Cell const&,G3D::Vector3int16 const&)")]
// was: bool RBX::Voxel::Water::cellHasWater<RBX::MegaClusterInstance::CellChunk>(RBX::MegaClusterInstance::CellChunk const*,RBX::Voxel::Cell const&,G3D::Vector3int16 const&)
// IDA 0x87211c: 165 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87211c() {
}

// 0x872330 — __ZN3RBX19MegaClusterInstance17setLinearVelocityERKN3G3D7Vector3E
#[doc(alias = "RBX::MegaClusterInstance::setLinearVelocity(G3D::Vector3 const&)")]
// was: RBX::MegaClusterInstance::setLinearVelocity(G3D::Vector3 const&)
// IDA 0x872330: 3 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872330() {
}

// 0x872340 — __ZN3RBX19MegaClusterInstance21setRotationalVelocityERKN3G3D7Vector3E
#[doc(alias = "RBX::MegaClusterInstance::setRotationalVelocity(G3D::Vector3 const&)")]
// was: RBX::MegaClusterInstance::setRotationalVelocity(G3D::Vector3 const&)
// IDA 0x872340: 3 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872340() {
}

// 0x873a84 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EEC2EMS2_FS4_S4_EPKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::BoundFuncDesc(G3D::Vector3 (RBX::MegaClusterInstance::*)(G3D::Vector3),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::BoundFuncDesc(G3D::Vector3 (RBX::MegaClusterInstance::*)(G3D::Vector3),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0x873a84: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_873a84() {
}

// 0x873bfc — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::declareSignature(char const*,RBX::Reflection::Variant)
// IDA 0x873bfc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_873bfc() {
}

// 0x873c2c — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::~BoundFuncDesc()
// IDA 0x873c2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_873c2c() {
}

// 0x873d00 — __ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// IDA 0x873d00: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_873d00() {
}

// 0x873d40 — __ZN3RBX10Reflection11Call1HelperINS_19MegaClusterInstanceEMS2_FN3G3D7Vector3ES4_ES4_S4_E4callEPS2_S6_RNS0_7VariantERKS4_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::MegaClusterInstance,G3D::Vector3 (RBX::MegaClusterInstance::*)(G3D::Vector3),G3D::Vector3,G3D::Vector3>::call(RBX::MegaClusterInstance*,G3D::Vector3 (RBX::MegaClusterInstance::*)(G3D::Vector3),RBX::Reflection::Variant &,G3D::Vector3 const&)")]
// was: RBX::Reflection::Call1Helper<RBX::MegaClusterInstance,G3D::Vector3 (RBX::MegaClusterInstance::*)(G3D::Vector3),G3D::Vector3,G3D::Vector3>::call(RBX::MegaClusterInstance*,G3D::Vector3 (RBX::MegaClusterInstance::*)(G3D::Vector3),RBX::Reflection::Variant &,G3D::Vector3 const&)
// IDA 0x873d40: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_873d40() {
}

// 0x873d7c — __ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector3ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "G3D::Vector3 RBX::Reflection::ArgHelper::getArg<G3D::Vector3,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3> const&,boost::disable_if<boost::is_same<G3D::Vector3,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: G3D::Vector3 RBX::Reflection::ArgHelper::getArg<G3D::Vector3,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3> const&,boost::disable_if<boost::is_same<G3D::Vector3,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
// IDA 0x873d7c: 176 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_873d7c() {
}

// 0x873f50 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EEC2EMS2_FS4_iiiEPKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::BoundFuncDesc(G3D::Vector3 (RBX::MegaClusterInstance::*)(int,int,int),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::BoundFuncDesc(G3D::Vector3 (RBX::MegaClusterInstance::*)(int,int,int),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0x873f50: 213 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_873f50() {
}

// 0x874168 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EE16declareSignatureEPKcNS0_7VariantES8_S9_S8_S9_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
// IDA 0x874168: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_874168() {
}

// 0x8741d0 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::~BoundFuncDesc()
// IDA 0x8741d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8741d0() {
}

// 0x8742bc — __ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// IDA 0x8742bc: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8742bc() {
}

// 0x87431c — __ZN3RBX10Reflection11Call3HelperINS_19MegaClusterInstanceEMS2_FN3G3D7Vector3EiiiEiiiS4_E4callEPS2_S6_RNS0_7VariantERKiSC_SC_
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::MegaClusterInstance,G3D::Vector3 (RBX::MegaClusterInstance::*)(int,int,int),int,int,int,G3D::Vector3>::call(RBX::MegaClusterInstance*,G3D::Vector3 (RBX::MegaClusterInstance::*)(int,int,int),RBX::Reflection::Variant &,int const&,int const&,int const&)")]
// was: RBX::Reflection::Call3Helper<RBX::MegaClusterInstance,G3D::Vector3 (RBX::MegaClusterInstance::*)(int,int,int),int,int,int,G3D::Vector3>::call(RBX::MegaClusterInstance*,G3D::Vector3 (RBX::MegaClusterInstance::*)(int,int,int),RBX::Reflection::Variant &,int const&,int const&,int const&)
// IDA 0x87431c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87431c() {
}

// 0x87b3bc — __ZN3RBX15MegaClusterPoly7hitTestERKNS_6RbxRayERN3G3D7Vector3ERbfRNS_6CellIDEbb
#[doc(alias = "RBX::MegaClusterPoly::hitTest(RBX::RbxRay const&,G3D::Vector3 &,bool &,float,RBX::CellID &,bool,bool)")]
// was: RBX::MegaClusterPoly::hitTest(RBX::RbxRay const&,G3D::Vector3 &,bool &,float,RBX::CellID &,bool,bool)
// IDA 0x87b3bc: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b3bc() {
}

// 0x87b414 — __ZN3RBX15MegaClusterPoly9hitTestMCERKNS_6RbxRayERN3G3D7Vector3ERbRiRNS4_15CoordinateFrameEfRNS_6CellIDEbb
#[doc(alias = "RBX::MegaClusterPoly::hitTestMC(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)")]
// was: RBX::MegaClusterPoly::hitTestMC(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)
// IDA 0x87b414: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b414() {
}

// 0x87b488 — __ZNK3RBX15MegaClusterPoly26findTouchingSurfacesConvexERKN3G3D15CoordinateFrameERmRKNS_8GeometryES4_S5_
#[doc(alias = "RBX::MegaClusterPoly::findTouchingSurfacesConvex(G3D::CoordinateFrame const&,unsigned long &,RBX::Geometry const&,G3D::CoordinateFrame const&,unsigned long &)const")]
// was: RBX::MegaClusterPoly::findTouchingSurfacesConvex(G3D::CoordinateFrame const&,unsigned long &,RBX::Geometry const&,G3D::CoordinateFrame const&,unsigned long &)const
// IDA 0x87b488: 113 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b488() {
}

// 0x87b5bc — __ZNK3RBX15MegaClusterPoly35findCellsTouchingGeometryWithBufferERKfRKN3G3D15CoordinateFrameERKNS_8GeometryES6_PSt3mapIiPNS3_12Vector3int16ESt4lessIiESaISt4pairIKiSC_EEE
#[doc(alias = "RBX::MegaClusterPoly::findCellsTouchingGeometryWithBuffer(float const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const")]
// was: RBX::MegaClusterPoly::findCellsTouchingGeometryWithBuffer(float const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const
// IDA 0x87b5bc: 135 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b5bc() {
}

// 0x87b784 — __ZNK3RBX15MegaClusterPoly25findPlanarTouchesWithGeomERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE
#[doc(alias = "RBX::MegaClusterPoly::findPlanarTouchesWithGeom(G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const")]
// was: RBX::MegaClusterPoly::findPlanarTouchesWithGeom(G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const
// IDA 0x87b784: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b784() {
}

// 0x87b828 — __ZNK3RBX15MegaClusterPoly22hasPlanarTouchWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_
#[doc(alias = "RBX::MegaClusterPoly::hasPlanarTouchWithGeom(G3D::Vector3int16 const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&)const")]
// was: RBX::MegaClusterPoly::hasPlanarTouchWithGeom(G3D::Vector3int16 const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&)const
// IDA 0x87b828: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b828() {
}

// 0x87b874 — __ZNK3RBX15MegaClusterPoly28findCellIntersectionWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_Rm
#[doc(alias = "RBX::MegaClusterPoly::findCellIntersectionWithGeom(G3D::Vector3int16 const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,unsigned long &)const")]
// was: RBX::MegaClusterPoly::findCellIntersectionWithGeom(G3D::Vector3int16 const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,unsigned long &)const
// IDA 0x87b874: 500 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b874() {
}

// 0x87be18 — __ZNK3RBX15MegaClusterPoly28hitLocationOnCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnCornerWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
// was: RBX::MegaClusterPoly::hitLocationOnCornerWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const
// IDA 0x87be18: 477 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87be18() {
}

// 0x87c450 — __ZNK3RBX15MegaClusterPoly32hitLocationOnHorizontalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnHorizontalWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
// was: RBX::MegaClusterPoly::hitLocationOnHorizontalWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const
// IDA 0x87c450: 593 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87c450() {
}

// 0x87cc0c — __ZNK3RBX15MegaClusterPoly30hitLocationOnVerticalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnVerticalWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
// was: RBX::MegaClusterPoly::hitLocationOnVerticalWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const
// IDA 0x87cc0c: 599 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87cc0c() {
}

// 0x87d3e0 — __ZNK3RBX15MegaClusterPoly35hitLocationOnInverseCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnInverseCornerWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
// was: RBX::MegaClusterPoly::hitLocationOnInverseCornerWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const
// IDA 0x87d3e0: 783 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87d3e0() {
}

// 0x87de28 — __ZNK3RBX15MegaClusterPoly22hitLocationOnBlockCellERKNS_6RbxRayERKN3G3D12Vector3int16ERNS4_7Vector3ERiRNS4_15CoordinateFrameE
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnBlockCell(RBX::RbxRay const&,G3D::Vector3int16 const&,G3D::Vector3 &,int &,G3D::CoordinateFrame &)const")]
// was: RBX::MegaClusterPoly::hitLocationOnBlockCell(RBX::RbxRay const&,G3D::Vector3int16 const&,G3D::Vector3 &,int &,G3D::CoordinateFrame &)const
// IDA 0x87de28: 704 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87de28() {
}

// 0x87e738 — __ZNK3RBX15MegaClusterPoly25findCellsTouchingGeometryERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE
#[doc(alias = "RBX::MegaClusterPoly::findCellsTouchingGeometry(G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const")]
// was: RBX::MegaClusterPoly::findCellsTouchingGeometry(G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const
// IDA 0x87e738: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87e738() {
}

// 0x87e758 — __ZN3RBX15MegaClusterPoly18cellsInBoundingBoxERKN3G3D7Vector3ES4_
#[doc(alias = "RBX::MegaClusterPoly::cellsInBoundingBox(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: RBX::MegaClusterPoly::cellsInBoundingBox(G3D::Vector3 const&,G3D::Vector3 const&)
// IDA 0x87e758: 620 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87e758() {
}

// 0x87edc4 — __ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY15MegaClusterMeshENS1_15Vector3ComparerEE5TokenEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token> const&)")]
// was: boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::operator=(boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token> const&)
// IDA 0x87edc4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87edc4() {
}

// 0x87edfc — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE8getTokenERKS2_
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)
// IDA 0x87edfc: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87edfc() {
}

// 0x87ef60 — __ZNK3RBX15MegaClusterPoly19hitTestMC_templatedINS_5Voxel4GridEEEbRKNS_6RbxRayERN3G3D7Vector3ERbRiRNS7_15CoordinateFrameEfRNS_6CellIDEbb
#[doc(alias = "bool RBX::MegaClusterPoly::hitTestMC_templated<RBX::Voxel::Grid>(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)const")]
// was: bool RBX::MegaClusterPoly::hitTestMC_templated<RBX::Voxel::Grid>(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)const
// IDA 0x87ef60: 500 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87ef60() {
}

// 0x87f548 — __ZNK3RBX15MegaClusterPoly19hitTestMC_templatedINS_19MegaClusterInstanceEEEbRKNS_6RbxRayERN3G3D7Vector3ERbRiRNS6_15CoordinateFrameEfRNS_6CellIDEbb
#[doc(alias = "bool RBX::MegaClusterPoly::hitTestMC_templated<RBX::MegaClusterInstance>(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)const")]
// was: bool RBX::MegaClusterPoly::hitTestMC_templated<RBX::MegaClusterInstance>(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)const
// IDA 0x87f548: 529 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87f548() {
}

// 0x87fb74 — __ZNSt6vectorIN3G3D7Vector3ESaIS1_EEaSERKS3_
#[doc(alias = "std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>::operator=(std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&)")]
// was: std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>::operator=(std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&)
// IDA 0x87fb74: 88 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87fb74() {
}

// 0x87fef4 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3G3D7Vector3ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Vector3 * std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 *,G3D::Vector3 *>(G3D::Vector3 *,G3D::Vector3 *,G3D::Vector3 *)")]
// was: G3D::Vector3 * std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 *,G3D::Vector3 *>(G3D::Vector3 *,G3D::Vector3 *,G3D::Vector3 *)
// IDA 0x87fef4: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_87fef4() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x87ff50 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3G3D7Vector3EPS4_EET0_T_S9_S8_
#[doc(alias = "G3D::Vector3* std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 const*,G3D::Vector3*>(G3D::Vector3 const*,G3D::Vector3 const*,G3D::Vector3*)")]
// was: G3D::Vector3* std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 const*,G3D::Vector3*>(G3D::Vector3 const*,G3D::Vector3 const*,G3D::Vector3*)
// IDA 0x87ff50: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_87ff50() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x87ffac — __ZN3RBX24getRegionForCellLocationINS_19MegaClusterInstanceEEEKNT_6RegionEPKS2_RKN3G3D12Vector3int16EPS3_
#[doc(alias = "RBX::MegaClusterInstance::Region const RBX::getRegionForCellLocation<RBX::MegaClusterInstance>(RBX::MegaClusterInstance::Region const*,G3D::Vector3int16 const&,RBX::MegaClusterInstance::Region const*)")]
// was: RBX::MegaClusterInstance::Region const RBX::getRegionForCellLocation<RBX::MegaClusterInstance>(RBX::MegaClusterInstance::Region const*,G3D::Vector3int16 const&,RBX::MegaClusterInstance::Region const*)
// IDA 0x87ffac: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87ffac() {
}

// 0x88004c — __ZN3RBX24getRegionForCellLocationINS_5Voxel4GridEEEKNT_6RegionEPKS3_RKN3G3D12Vector3int16EPS4_
#[doc(alias = "RBX::Voxel::Grid::Region const RBX::getRegionForCellLocation<RBX::Voxel::Grid>(RBX::Voxel::Grid::Region const*,G3D::Vector3int16 const&,RBX::Voxel::Grid::Region const*)")]
// was: RBX::Voxel::Grid::Region const RBX::getRegionForCellLocation<RBX::Voxel::Grid>(RBX::Voxel::Grid::Region const*,G3D::Vector3int16 const&,RBX::Voxel::Grid::Region const*)
// IDA 0x88004c: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88004c() {
}

// 0x8800ec — __ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_
#[doc(alias = "std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)")]
// was: std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)
// IDA 0x8800ec: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8800ec() {
}

// 0x880160 — __ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY15MegaClusterMeshENS1_15Vector3ComparerEE5TokenEEC2IS9_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token *)")]
// was: boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token *)
// IDA 0x880160: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880160() {
}

// 0x880234 — __ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolIN3G3D7Vector3ENS3_4POLY15MegaClusterMeshENS3_15Vector3ComparerEE5TokenEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token *)")]
// was: boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token *)
// IDA 0x880234: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880234() {
}

// 0x880344 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *)
// IDA 0x880344: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880344() {
}

// 0x880520 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10ValueCountD2Ev
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount::~ValueCount()")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount::~ValueCount()
// IDA 0x880520: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_880520() {
}

// 0x8806b8 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)
// IDA 0x8806b8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8806b8() {
}

// 0x8806e0 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>)
// IDA 0x8806e0: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8806e0() {
}

// 0x880740 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>> *)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>> *)
// IDA 0x880740: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880740() {
}

// 0x880768 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11lower_boundERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)
// IDA 0x880768: 34 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880768() {
}

// 0x8807c4 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11upper_boundERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)
// IDA 0x8807c4: 34 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8807c4() {
}

// 0x880820 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()
// IDA 0x880820: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_880820() {
}

// 0x880824 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()
// IDA 0x880824: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_880824() {
}

// 0x880828 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::dispose(void)")]
// was: boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::dispose(void)
// IDA 0x880828: 57 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880828() {
}

// 0x8808d0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::get_deleter(std::type_info const&)
// IDA 0x8808d0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8808d0() {
}

// 0x8808d4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE5TokenEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::Token>::get_untyped_deleter(void)
// IDA 0x8808d4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8808d4() {
}

// 0x8808d8 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)
// IDA 0x8808d8: 147 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8808d8() {
}

// 0x880a80 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)
// IDA 0x880a80: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880a80() {
}

// 0x880b24 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueERKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)
// IDA 0x880b24: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880b24() {
}

// 0x880bf8 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10ValueCountC2ERKS2_
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount::ValueCount(G3D::Vector3 const&)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount::ValueCount(G3D::Vector3 const&)
// IDA 0x880bf8: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880bf8() {
}

// 0x880d28 — __ZN3RBX4POLY15MegaClusterMeshC2ERKN3G3D7Vector3E
#[doc(alias = "RBX::POLY::MegaClusterMesh::MegaClusterMesh(G3D::Vector3 const&)")]
// was: RBX::POLY::MegaClusterMesh::MegaClusterMesh(G3D::Vector3 const&)
// IDA 0x880d28: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880d28() {
}

// 0x880f24 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE4findERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)
// IDA 0x880f24: 57 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880f24() {
}

// 0x880fc8 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE27safe_static_init_staticDataEv
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::safe_static_init_staticData(void)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::safe_static_init_staticData(void)
// IDA 0x880fc8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_880fc8() {
}

// 0x880fcc — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE29safe_static_do_get_staticDataEv
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)
// IDA 0x880fcc: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880fcc() {
}

// 0x8810dc — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10StaticDataD1Ev
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::StaticData::~StaticData()")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::StaticData::~StaticData()
// IDA 0x8810dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8810dc() {
}

// 0x8815fc — __ZN3RBX15PolyCellContactC1EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E
#[doc(alias = "RBX::PolyCellContact::PolyCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)")]
// was: RBX::PolyCellContact::PolyCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)
// IDA 0x8815fc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8815fc() {
}

// 0x881600 — __ZN3RBX15PolyCellContactC2EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E
#[doc(alias = "RBX::PolyCellContact::PolyCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)")]
// was: RBX::PolyCellContact::PolyCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)
// IDA 0x881600: 233 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_881600() {
}

// 0x881f48 — __ZN3RBX16CellFaceFacePair15computeVerticesERNS_10FixedArrayIN3G3D7Vector3ELm40EEERKNS2_15CoordinateFrameE
#[doc(alias = "RBX::CellFaceFacePair::computeVertices(RBX::FixedArray<G3D::Vector3,40ul> &,G3D::CoordinateFrame const&)")]
// was: RBX::CellFaceFacePair::computeVertices(RBX::FixedArray<G3D::Vector3,40ul> &,G3D::CoordinateFrame const&)
// IDA 0x881f48: 101 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_881f48() {
}

// 0x882090 — __ZN3RBX16CellFaceFacePair13closestVertexEPKNS_4POLY4FaceERKNS_10FixedArrayIN3G3D7Vector3ELm40EEERPKNS1_6VertexE
#[doc(alias = "RBX::CellFaceFacePair::closestVertex(RBX::POLY::Face const*,RBX::FixedArray<G3D::Vector3,40ul> const&,RBX::POLY::Vertex const*&)")]
// was: RBX::CellFaceFacePair::closestVertex(RBX::POLY::Face const*,RBX::FixedArray<G3D::Vector3,40ul> const&,RBX::POLY::Vertex const*&)
// IDA 0x882090: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_882090() {
}

// 0x8827b0 — __ZN3RBX16CellFaceFacePair12loadVerticesEPNS_10FixedArrayINS0_12VertexStatusELm40EEEPN3G3D15CoordinateFrameERNS1_IPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::CellFaceFacePair::loadVertices(RBX::FixedArray<RBX::CellFaceFacePair::VertexStatus,40ul> *,G3D::CoordinateFrame *,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// was: RBX::CellFaceFacePair::loadVertices(RBX::FixedArray<RBX::CellFaceFacePair::VertexStatus,40ul> *,G3D::CoordinateFrame *,RBX::FixedArray<RBX::PolyConnector *,40ul> &)
// IDA 0x8827b0: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8827b0() {
}

// 0x8828fc — __ZN3RBX16CellFaceFacePair24checkOneSideIntersectionEPKNS_4POLY6VertexES4_RKN3G3D15CoordinateFrameERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::CellFaceFacePair::checkOneSideIntersection(RBX::POLY::Vertex const*,RBX::POLY::Vertex const*,G3D::CoordinateFrame const&,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// was: RBX::CellFaceFacePair::checkOneSideIntersection(RBX::POLY::Vertex const*,RBX::POLY::Vertex const*,G3D::CoordinateFrame const&,RBX::FixedArray<RBX::PolyConnector *,40ul> &)
// IDA 0x8828fc: 96 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8828fc() {
}

// 0x882a50 — __ZN3RBX16CellFaceFacePair25checkTwoSideIntersectionsEPKNS_4POLY6VertexES4_RKN3G3D15CoordinateFrameERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::CellFaceFacePair::checkTwoSideIntersections(RBX::POLY::Vertex const*,RBX::POLY::Vertex const*,G3D::CoordinateFrame const&,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// was: RBX::CellFaceFacePair::checkTwoSideIntersections(RBX::POLY::Vertex const*,RBX::POLY::Vertex const*,G3D::CoordinateFrame const&,RBX::FixedArray<RBX::PolyConnector *,40ul> &)
// IDA 0x882a50: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_882a50() {
}

// 0x882bd0 — __ZN3RBX16CellFaceFacePair27validateOneSideIntersectionEPKNS_4POLY6VertexES4_RKN3G3D15CoordinateFrameERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::CellFaceFacePair::validateOneSideIntersection(RBX::POLY::Vertex const*,RBX::POLY::Vertex const*,G3D::CoordinateFrame const&,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// was: RBX::CellFaceFacePair::validateOneSideIntersection(RBX::POLY::Vertex const*,RBX::POLY::Vertex const*,G3D::CoordinateFrame const&,RBX::FixedArray<RBX::PolyConnector *,40ul> &)
// IDA 0x882bd0: 96 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_882bd0() {
}

// 0x882d24 — __ZN3RBX16CellFaceFacePair18testVerticesInsideEmRNS_10FixedArrayINS0_12VertexStatusELm40EEERKN3G3D15CoordinateFrameERNS1_IPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::CellFaceFacePair::testVerticesInside(unsigned long,RBX::FixedArray<RBX::CellFaceFacePair::VertexStatus,40ul> &,G3D::CoordinateFrame const&,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// was: RBX::CellFaceFacePair::testVerticesInside(unsigned long,RBX::FixedArray<RBX::CellFaceFacePair::VertexStatus,40ul> &,G3D::CoordinateFrame const&,RBX::FixedArray<RBX::PolyConnector *,40ul> &)
// IDA 0x882d24: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_882d24() {
}

