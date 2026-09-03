//! Generated watchdog rend wd2a — 120 stubs Ogre core (Ogre namespace) global dedup EA-sorted asc
//! Source: ida/export.json (85545 funcs) filtered Ogre (9822 total, 8013 already stubbed, 1809 candidates) -> 120 lowest EAs
//! Range: 0xe3adb4..0xe43594 (120 stubs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };


// 0xe3adb4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: int(void)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// IDA 0xe3adb4: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3adb4() {
}


// 0xe3aed0 — __ZN4Ogre9SubEntityC1EPNS_6EntityEPNS_7SubMeshE
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this, Ogre::Entity *, Ogre::SubMesh *)
#[doc(alias = "Ogre::SubEntity::SubEntity(Ogre::Entity *,Ogre::SubMesh *)")]
#[doc(alias = "__ZN4Ogre9SubEntityC1EPNS_6EntityEPNS_7SubMeshE")]
// IDA 0xe3aed0: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3aed0() {
}


// 0xe3aedc — __ZN4Ogre9SubEntityC2EPNS_6EntityEPNS_7SubMeshE
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this, Ogre::Entity *, Ogre::SubMesh *)
#[doc(alias = "Ogre::SubEntity::SubEntity(Ogre::Entity *,Ogre::SubMesh *)")]
#[doc(alias = "__ZN4Ogre9SubEntityC2EPNS_6EntityEPNS_7SubMeshE")]
// IDA 0xe3aedc: 640 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3aedc() {
}


// 0xe3b554 — __ZN4Ogre9SubEntityD0Ev
// type: void __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::~SubEntity()")]
#[doc(alias = "__ZN4Ogre9SubEntityD0Ev")]
// IDA 0xe3b554: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe3b554() {
}


// 0xe3b5e4 — __ZN4Ogre9SubEntityD1Ev
// type: void __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::~SubEntity()")]
#[doc(alias = "__ZN4Ogre9SubEntityD1Ev")]
// IDA 0xe3b5e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe3b5e4() {
}


// 0xe3b5f0 — __ZN4Ogre9SubEntityD2Ev
// type: void __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::~SubEntity()")]
#[doc(alias = "__ZN4Ogre9SubEntityD2Ev")]
// IDA 0xe3b5f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe3b5f0() {
}


// 0xe3b7cc — __ZN4Ogre9SubEntity10getSubMeshEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::getSubMesh(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity10getSubMeshEv")]
// IDA 0xe3b7cc: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3b7cc() {
}


// 0xe3b7d0 — __ZNK4Ogre9SubEntity15getMaterialNameEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::getMaterialName(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity15getMaterialNameEv")]
// IDA 0xe3b7d0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3b7d0() {
}


// 0xe3b7d4 — __ZN4Ogre9SubEntity15setMaterialNameERKSsS2_
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this, const std::string *, const std::string *)
#[doc(alias = "Ogre::SubEntity::setMaterialName(std::string const&,std::string const&)")]
#[doc(alias = "__ZN4Ogre9SubEntity15setMaterialNameERKSsS2_")]
// IDA 0xe3b7d4: 975 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3b7d4() {
}


// 0xe3c25c — __ZN4Ogre9SubEntity11setMaterialERKNS_11MaterialPtrE
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this, const Ogre::MaterialPtr *)
#[doc(alias = "Ogre::SubEntity::setMaterial(Ogre::MaterialPtr const&)")]
#[doc(alias = "__ZN4Ogre9SubEntity11setMaterialERKNS_11MaterialPtrE")]
// IDA 0xe3c25c: 373 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3c25c() {
}


// 0xe3c968 — __ZNK4Ogre9SubEntity11getMaterialEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::getMaterial(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity11getMaterialEv")]
// IDA 0xe3c968: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3c968() {
}


// 0xe3c96c — __ZNK4Ogre9SubEntity12getTechniqueEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::getTechnique(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity12getTechniqueEv")]
// IDA 0xe3c96c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3c96c() {
}


// 0xe3c980 — __ZN4Ogre9SubEntity18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::SubEntity::getRenderOperation(Ogre::RenderOperation &)")]
#[doc(alias = "__ZN4Ogre9SubEntity18getRenderOperationERNS_15RenderOperationE")]
// IDA 0xe3c980: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3c980() {
}


// 0xe3cd18 — __ZNK4Ogre9SubEntity9getLightsEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::getLights(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity9getLightsEv")]
// IDA 0xe3cd18: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3cd18() {
}


// 0xe3cd28 — __ZN4Ogre9SubEntity10setVisibleEb
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this, bool)
#[doc(alias = "Ogre::SubEntity::setVisible(bool)")]
#[doc(alias = "__ZN4Ogre9SubEntity10setVisibleEb")]
// IDA 0xe3cd28: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3cd28() {
}


// 0xe3cd30 — __ZNK4Ogre9SubEntity9isVisibleEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::isVisible(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity9isVisibleEv")]
// IDA 0xe3cd30: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3cd30() {
}


// 0xe3cd38 — __ZN4Ogre9SubEntity23prepareTempBlendBuffersEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::prepareTempBlendBuffers(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity23prepareTempBlendBuffersEv")]
// IDA 0xe3cd38: 129 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3cd38() {
}


// 0xe3ce9c — __ZNK4Ogre9SubEntity15getCastsShadowsEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::getCastsShadows(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity15getCastsShadowsEv")]
// IDA 0xe3ce9c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3ce9c() {
}


// 0xe3ceac — __ZN4Ogre9SubEntity22_getSkelAnimVertexDataEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::_getSkelAnimVertexData(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity22_getSkelAnimVertexDataEv")]
// IDA 0xe3ceac: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3ceac() {
}


// 0xe3ceb0 — __ZN4Ogre9SubEntity32_getSoftwareVertexAnimVertexDataEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::_getSoftwareVertexAnimVertexData(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity32_getSoftwareVertexAnimVertexDataEv")]
// IDA 0xe3ceb0: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3ceb0() {
}


// 0xe3ceb8 — __ZN4Ogre9SubEntity32_getHardwareVertexAnimVertexDataEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::_getHardwareVertexAnimVertexData(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity32_getHardwareVertexAnimVertexDataEv")]
// IDA 0xe3ceb8: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3ceb8() {
}


// 0xe3cec0 — __ZN4Ogre9SubEntity28_getVertexAnimTempBufferInfoEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::_getVertexAnimTempBufferInfo(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity28_getVertexAnimTempBufferInfoEv")]
// IDA 0xe3cec0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3cec0() {
}


// 0xe3cec4 — __ZNK4Ogre9SubEntity25_updateCustomGpuParameterERKNS_20GpuProgramParameters17AutoConstantEntryEPS1_
// type: int __fastcall(int, int, int)
#[doc(alias = "Ogre::SubEntity::_updateCustomGpuParameter(Ogre::GpuProgramParameters::AutoConstantEntry const&,Ogre::GpuProgramParameters*)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity25_updateCustomGpuParameterERKNS_20GpuProgramParameters17AutoConstantEntryEPS1_")]
// IDA 0xe3cec4: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3cec4() {
}


// 0xe3cf64 — __ZN4Ogre9SubEntity30_markBuffersUnusedForAnimationEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::_markBuffersUnusedForAnimation(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity30_markBuffersUnusedForAnimationEv")]
// IDA 0xe3cf64: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3cf64() {
}


// 0xe3cf6c — __ZN4Ogre9SubEntity28_markBuffersUsedForAnimationEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::_markBuffersUsedForAnimation(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity28_markBuffersUsedForAnimationEv")]
// IDA 0xe3cf6c: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3cf6c() {
}


// 0xe3cf74 — __ZN4Ogre9SubEntity33_restoreBuffersForUnusedAnimationEb
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this, bool)
#[doc(alias = "Ogre::SubEntity::_restoreBuffersForUnusedAnimation(bool)")]
#[doc(alias = "__ZN4Ogre9SubEntity33_restoreBuffersForUnusedAnimationEb")]
// IDA 0xe3cf74: 236 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3cf74() {
}


// 0xe3d1c0 — __ZN4Ogre9SubEntity19setRenderQueueGroupEh
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this, unsigned __int8)
#[doc(alias = "Ogre::SubEntity::setRenderQueueGroup(unsigned char)")]
#[doc(alias = "__ZN4Ogre9SubEntity19setRenderQueueGroupEh")]
// IDA 0xe3d1c0: 4 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d1c0() {
}


// 0xe3d1cc — __ZN4Ogre9SubEntity30setRenderQueueGroupAndPriorityEht
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this, unsigned __int8, unsigned __int16)
#[doc(alias = "Ogre::SubEntity::setRenderQueueGroupAndPriority(unsigned char,unsigned short)")]
#[doc(alias = "__ZN4Ogre9SubEntity30setRenderQueueGroupAndPriorityEht")]
// IDA 0xe3d1cc: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d1cc() {
}


// 0xe3d1e8 — __ZNK4Ogre9SubEntity19getRenderQueueGroupEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::getRenderQueueGroup(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity19getRenderQueueGroupEv")]
// IDA 0xe3d1e8: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d1e8() {
}


// 0xe3d1f0 — __ZNK4Ogre9SubEntity22getRenderQueuePriorityEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::getRenderQueuePriority(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity22getRenderQueuePriorityEv")]
// IDA 0xe3d1f0: 2 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d1f0() {
}


// 0xe3d1f8 — __ZNK4Ogre9SubEntity21isRenderQueueGroupSetEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::isRenderQueueGroupSet(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity21isRenderQueueGroupSetEv")]
// IDA 0xe3d1f8: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d1f8() {
}


// 0xe3d200 — __ZNK4Ogre9SubEntity24isRenderQueuePrioritySetEv
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this)
#[doc(alias = "Ogre::SubEntity::isRenderQueuePrioritySet(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity24isRenderQueuePrioritySetEv")]
// IDA 0xe3d200: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d200() {
}


// 0xe3d23c — __ZN4Ogre7SubMeshC1Ev
// type: _DWORD __fastcall(Ogre::SubMesh *__hidden this)
#[doc(alias = "Ogre::SubMesh::SubMesh(void)")]
#[doc(alias = "__ZN4Ogre7SubMeshC1Ev")]
// IDA 0xe3d23c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d23c() {
}


// 0xe3d248 — __ZN4Ogre7SubMeshC2Ev
// type: _DWORD __fastcall(Ogre::SubMesh *__hidden this)
#[doc(alias = "Ogre::SubMesh::SubMesh(void)")]
#[doc(alias = "__ZN4Ogre7SubMeshC2Ev")]
// IDA 0xe3d248: 224 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d248() {
}


// 0xe3d498 — __ZN4Ogre7SubMeshD1Ev
// type: void __fastcall(Ogre::SubMesh *__hidden this)
#[doc(alias = "Ogre::SubMesh::~SubMesh()")]
#[doc(alias = "__ZN4Ogre7SubMeshD1Ev")]
// IDA 0xe3d498: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe3d498() {
}


// 0xe3d4a4 — __ZN4Ogre7SubMeshD2Ev
// type: void __fastcall(Ogre::SubMesh *__hidden this)
#[doc(alias = "Ogre::SubMesh::~SubMesh()")]
#[doc(alias = "__ZN4Ogre7SubMeshD2Ev")]
// IDA 0xe3d4a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe3d4a4() {
}


// 0xe3d620 — __ZN4Ogre7SubMesh15removeLodLevelsEv
// type: _DWORD __fastcall(Ogre::SubMesh *__hidden this)
#[doc(alias = "Ogre::SubMesh::removeLodLevels(void)")]
#[doc(alias = "__ZN4Ogre7SubMesh15removeLodLevelsEv")]
// IDA 0xe3d620: 66 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d620() {
}


// 0xe3d6d4 — __ZN4Ogre7SubMesh15setMaterialNameERKSsS2_
// type: _DWORD __fastcall(Ogre::SubMesh *__hidden this, const std::string *, const std::string *)
#[doc(alias = "Ogre::SubMesh::setMaterialName(std::string const&,std::string const&)")]
#[doc(alias = "__ZN4Ogre7SubMesh15setMaterialNameERKSsS2_")]
// IDA 0xe3d6d4: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d6d4() {
}


// 0xe3d6ec — __ZNK4Ogre7SubMesh15getMaterialNameEv
// type: _DWORD __fastcall(Ogre::SubMesh *__hidden this)
#[doc(alias = "Ogre::SubMesh::getMaterialName(void)const")]
#[doc(alias = "__ZNK4Ogre7SubMesh15getMaterialNameEv")]
// IDA 0xe3d6ec: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d6ec() {
}


// 0xe3d6f0 — __ZNK4Ogre7SubMesh16isMatInitialisedEv
// type: _DWORD __fastcall(Ogre::SubMesh *__hidden this)
#[doc(alias = "Ogre::SubMesh::isMatInitialised(void)const")]
#[doc(alias = "__ZNK4Ogre7SubMesh16isMatInitialisedEv")]
// IDA 0xe3d6f0: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d6f0() {
}


// 0xe3d6f8 — __ZN4Ogre7SubMesh19_getRenderOperationERNS_15RenderOperationEt
#[doc(alias = "Ogre::SubMesh::_getRenderOperation(Ogre::RenderOperation &,unsigned short)")]
#[doc(alias = "__ZN4Ogre7SubMesh19_getRenderOperationERNS_15RenderOperationEt")]
// IDA 0xe3d6f8: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d6f8() {
}


// 0xe3d73c — __ZN4Ogre7SubMesh17addBoneAssignmentERKNS_22VertexBoneAssignment_sE
#[doc(alias = "Ogre::SubMesh::addBoneAssignment(Ogre::VertexBoneAssignment_s const&)")]
#[doc(alias = "__ZN4Ogre7SubMesh17addBoneAssignmentERKNS_22VertexBoneAssignment_sE")]
// IDA 0xe3d73c: 205 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d73c() {
}


// 0xe3d994 — __ZN4Ogre7SubMesh23_compileBoneAssignmentsEv
// type: _DWORD __fastcall(Ogre::SubMesh *__hidden this)
#[doc(alias = "Ogre::SubMesh::_compileBoneAssignments(void)")]
#[doc(alias = "__ZN4Ogre7SubMesh23_compileBoneAssignmentsEv")]
// IDA 0xe3d994: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d994() {
}


// 0xe3d9cc — __ZN4Ogre7SubMesh15addTextureAliasERKSsS2_
// type: _DWORD __fastcall(Ogre::SubMesh *__hidden this, const std::string *, const std::string *)
#[doc(alias = "Ogre::SubMesh::addTextureAlias(std::string const&,std::string const&)")]
#[doc(alias = "__ZN4Ogre7SubMesh15addTextureAliasERKSsS2_")]
// IDA 0xe3d9cc: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d9cc() {
}


// 0xe3d9e0 — __ZN4Ogre7SubMesh33updateMaterialUsingTextureAliasesEv
// type: _DWORD __fastcall(Ogre::SubMesh *__hidden this)
#[doc(alias = "Ogre::SubMesh::updateMaterialUsingTextureAliases(void)")]
#[doc(alias = "__ZN4Ogre7SubMesh33updateMaterialUsingTextureAliasesEv")]
// IDA 0xe3d9e0: 679 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3d9e0() {
}


// 0xe3e0e0 — __ZNK4Ogre7SubMesh22getVertexAnimationTypeEv
// type: _DWORD __fastcall(Ogre::SubMesh *__hidden this)
#[doc(alias = "Ogre::SubMesh::getVertexAnimationType(void)const")]
#[doc(alias = "__ZNK4Ogre7SubMesh22getVertexAnimationTypeEv")]
// IDA 0xe3e0e0: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3e0e0() {
}


// 0xe3e0fc — __ZNSt12_Vector_baseIPN4Ogre9IndexDataENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::IndexData *,Ogre::STLAllocator<Ogre::IndexData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre9IndexDataENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// IDA 0xe3e0fc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xe3e0fc() {
}


// 0xe3e100 — __ZNSt12_Vector_baseIPN4Ogre9IndexDataENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Vector_base<Ogre::IndexData *,Ogre::STLAllocator<Ogre::IndexData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre9IndexDataENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// IDA 0xe3e100: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe3e100() {
}


// 0xe3e140 — __ZNK4Ogre8TagPoint15getParentEntityEv
// type: _DWORD __fastcall(Ogre::TagPoint *__hidden this)
#[doc(alias = "Ogre::TagPoint::getParentEntity(void)const")]
#[doc(alias = "__ZNK4Ogre8TagPoint15getParentEntityEv")]
// IDA 0xe3e140: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3e140() {
}


// 0xe3e148 — __ZNK4Ogre8TagPoint22_getFullLocalTransformEv
// type: _DWORD __fastcall(Ogre::TagPoint *__hidden this)
#[doc(alias = "Ogre::TagPoint::_getFullLocalTransform(void)const")]
#[doc(alias = "__ZNK4Ogre8TagPoint22_getFullLocalTransformEv")]
// IDA 0xe3e148: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3e148() {
}


// 0xe3e1b8 — __ZN4Ogre9TechniqueC1EPNS_8MaterialE
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, Ogre::Material *)
#[doc(alias = "Ogre::Technique::Technique(Ogre::Material *)")]
#[doc(alias = "__ZN4Ogre9TechniqueC1EPNS_8MaterialE")]
// IDA 0xe3e1b8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3e1b8() {
}


// 0xe3e1c4 — __ZN4Ogre9TechniqueC2EPNS_8MaterialE
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, Ogre::Material *)
#[doc(alias = "Ogre::Technique::Technique(Ogre::Material *)")]
#[doc(alias = "__ZN4Ogre9TechniqueC2EPNS_8MaterialE")]
// IDA 0xe3e1c4: 313 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3e1c4() {
}


// 0xe3e4e8 — __ZN4Ogre9TechniqueaSERKS0_
// type: int __fastcall(int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::Technique::operator=(Ogre::Technique const&)")]
#[doc(alias = "__ZN4Ogre9TechniqueaSERKS0_")]
// IDA 0xe3e4e8: 173 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3e4e8() {
}


// 0xe3e6c8 — __ZN4Ogre9TechniqueD1Ev
// type: void __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::~Technique()")]
#[doc(alias = "__ZN4Ogre9TechniqueD1Ev")]
// IDA 0xe3e6c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe3e6c8() {
}


// 0xe3e6d4 — __ZN4Ogre9TechniqueD2Ev
// type: void __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::~Technique()")]
#[doc(alias = "__ZN4Ogre9TechniqueD2Ev")]
// IDA 0xe3e6d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe3e6d4() {
}


// 0xe3ec68 — __ZNK4Ogre9Technique11isSupportedEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::isSupported(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique11isSupportedEv")]
// IDA 0xe3ec68: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3ec68() {
}


// 0xe3ec70 — __ZN4Ogre9Technique8_compileEb
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, bool)
#[doc(alias = "Ogre::Technique::_compile(bool)")]
#[doc(alias = "__ZN4Ogre9Technique8_compileEb")]
// IDA 0xe3ec70: 222 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3ec70() {
}


// 0xe3eee0 — __ZN4Ogre9Technique13checkGPURulesERSt18basic_stringstreamIcSt11char_traitsIcESaIcEE
#[doc(alias = "Ogre::Technique::checkGPURules(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> &)")]
#[doc(alias = "__ZN4Ogre9Technique13checkGPURulesERSt18basic_stringstreamIcSt11char_traitsIcESaIcEE")]
// IDA 0xe3eee0: 742 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3eee0() {
}


// 0xe3f704 — __ZN4Ogre9Technique20checkHardwareSupportEbRSt18basic_stringstreamIcSt11char_traitsIcESaIcEE
#[doc(alias = "Ogre::Technique::checkHardwareSupport(bool,std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> &)")]
#[doc(alias = "__ZN4Ogre9Technique20checkHardwareSupportEbRSt18basic_stringstreamIcSt11char_traitsIcESaIcEE")]
// IDA 0xe3f704: 483 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3f704() {
}


// 0xe3fc54 — __ZN4Ogre9Technique10createPassEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::createPass(void)")]
#[doc(alias = "__ZN4Ogre9Technique10createPassEv")]
// IDA 0xe3fc54: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fc54() {
}


// 0xe3fd54 — __ZN4Ogre9Technique7getPassEt
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::Technique::getPass(unsigned short)")]
#[doc(alias = "__ZN4Ogre9Technique7getPassEt")]
// IDA 0xe3fd54: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fd54() {
}


// 0xe3fd5c — __ZN4Ogre9Technique7getPassERKSs
#[doc(alias = "Ogre::Technique::getPass(std::string const&)")]
#[doc(alias = "__ZN4Ogre9Technique7getPassERKSs")]
// IDA 0xe3fd5c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fd5c() {
}


// 0xe3fdbc — __ZNK4Ogre9Technique12getNumPassesEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::getNumPasses(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique12getNumPassesEv")]
// IDA 0xe3fdbc: 4 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fdbc() {
}


// 0xe3fdc8 — __ZN4Ogre9Technique15getPassIteratorEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::getPassIterator(void)")]
#[doc(alias = "__ZN4Ogre9Technique15getPassIteratorEv")]
// IDA 0xe3fdc8: 6 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fdc8() {
}


// 0xe3fdd4 — __ZNK4Ogre9Technique13isTransparentEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::isTransparent(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique13isTransparentEv")]
// IDA 0xe3fdd4: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fdd4() {
}


// 0xe3fdec — __ZNK4Ogre9Technique27isTransparentSortingEnabledEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::isTransparentSortingEnabled(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique27isTransparentSortingEnabledEv")]
// IDA 0xe3fdec: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fdec() {
}


// 0xe3fe04 — __ZNK4Ogre9Technique26isTransparentSortingForcedEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::isTransparentSortingForced(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique26isTransparentSortingForcedEv")]
// IDA 0xe3fe04: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fe04() {
}


// 0xe3fe1c — __ZNK4Ogre9Technique19isDepthWriteEnabledEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::isDepthWriteEnabled(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique19isDepthWriteEnabledEv")]
// IDA 0xe3fe1c: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fe1c() {
}


// 0xe3fe34 — __ZNK4Ogre9Technique19isDepthCheckEnabledEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::isDepthCheckEnabled(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique19isDepthCheckEnabledEv")]
// IDA 0xe3fe34: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fe34() {
}


// 0xe3fe4c — __ZNK4Ogre9Technique22hasColourWriteDisabledEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::hasColourWriteDisabled(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique22hasColourWriteDisabledEv")]
// IDA 0xe3fe4c: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fe4c() {
}


// 0xe3fe68 — __ZN4Ogre9Technique8_prepareEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::_prepare(void)")]
#[doc(alias = "__ZN4Ogre9Technique8_prepareEv")]
// IDA 0xe3fe68: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fe68() {
}


// 0xe3fea4 — __ZN4Ogre9Technique10_unprepareEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::_unprepare(void)")]
#[doc(alias = "__ZN4Ogre9Technique10_unprepareEv")]
// IDA 0xe3fea4: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fea4() {
}


// 0xe3fec0 — __ZN4Ogre9Technique5_loadEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::_load(void)")]
#[doc(alias = "__ZN4Ogre9Technique5_loadEv")]
// IDA 0xe3fec0: 391 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3fec0() {
}


// 0xe402ac — __ZN4Ogre9Technique7_unloadEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::_unload(void)")]
#[doc(alias = "__ZN4Ogre9Technique7_unloadEv")]
// IDA 0xe402ac: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe402ac() {
}


// 0xe402c8 — __ZNK4Ogre9Technique8isLoadedEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::isLoaded(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique8isLoadedEv")]
// IDA 0xe402c8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe402c8() {
}


// 0xe402ec — __ZN4Ogre9Technique20setDepthCheckEnabledEb
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, bool)
#[doc(alias = "Ogre::Technique::setDepthCheckEnabled(bool)")]
#[doc(alias = "__ZN4Ogre9Technique20setDepthCheckEnabledEb")]
// IDA 0xe402ec: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe402ec() {
}


// 0xe4030c — __ZN4Ogre9Technique20setDepthWriteEnabledEb
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, bool)
#[doc(alias = "Ogre::Technique::setDepthWriteEnabled(bool)")]
#[doc(alias = "__ZN4Ogre9Technique20setDepthWriteEnabledEb")]
// IDA 0xe4030c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4030c() {
}


// 0xe4032c — __ZN4Ogre9Technique18setLightingEnabledEb
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, bool)
#[doc(alias = "Ogre::Technique::setLightingEnabled(bool)")]
#[doc(alias = "__ZN4Ogre9Technique18setLightingEnabledEb")]
// IDA 0xe4032c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4032c() {
}


// 0xe4034c — __ZN4Ogre9Technique6setFogEbNS_7FogModeERKNS_11ColourValueEfff
// type: int __fastcall(int, int, int, int, float, float, float)
#[doc(alias = "Ogre::Technique::setFog(bool,Ogre::FogMode,Ogre::ColourValue const&,float,float,float)")]
#[doc(alias = "__ZN4Ogre9Technique6setFogEbNS_7FogModeERKNS_11ColourValueEfff")]
// IDA 0xe4034c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4034c() {
}


// 0xe403b4 — __ZN4Ogre9Technique16setSceneBlendingENS_14SceneBlendTypeE
#[doc(alias = "Ogre::Technique::setSceneBlending(Ogre::SceneBlendType)")]
#[doc(alias = "__ZN4Ogre9Technique16setSceneBlendingENS_14SceneBlendTypeE")]
// IDA 0xe403b4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe403b4() {
}


// 0xe403d4 — __ZN4Ogre9Technique16setSceneBlendingENS_16SceneBlendFactorES1_
#[doc(alias = "Ogre::Technique::setSceneBlending(Ogre::SceneBlendFactor,Ogre::SceneBlendFactor)")]
#[doc(alias = "__ZN4Ogre9Technique16setSceneBlendingENS_16SceneBlendFactorES1_")]
// IDA 0xe403d4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe403d4() {
}


// 0xe40400 — __ZN4Ogre9Technique7setNameERKSs
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, const std::string *)
#[doc(alias = "Ogre::Technique::setName(std::string const&)")]
#[doc(alias = "__ZN4Ogre9Technique7setNameERKSs")]
// IDA 0xe40400: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40400() {
}


// 0xe4040c — __ZN4Ogre9Technique21_notifyNeedsRecompileEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::_notifyNeedsRecompile(void)")]
#[doc(alias = "__ZN4Ogre9Technique21_notifyNeedsRecompileEv")]
// IDA 0xe4040c: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4040c() {
}


// 0xe40420 — __ZN4Ogre9Technique11setLodIndexEt
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::Technique::setLodIndex(unsigned short)")]
#[doc(alias = "__ZN4Ogre9Technique11setLodIndexEt")]
// IDA 0xe40420: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40420() {
}


// 0xe40438 — __ZN4Ogre9Technique13setSchemeNameERKSs
#[doc(alias = "Ogre::Technique::setSchemeName(std::string const&)")]
#[doc(alias = "__ZN4Ogre9Technique13setSchemeNameERKSs")]
// IDA 0xe40438: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40438() {
}


// 0xe40460 — __ZNK4Ogre9Technique15_getSchemeIndexEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::_getSchemeIndex(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique15_getSchemeIndexEv")]
// IDA 0xe40460: 2 insns (LDRH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40460() {
}


// 0xe40464 — __ZN4Ogre9Technique40checkManuallyOrganisedIlluminationPassesEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::checkManuallyOrganisedIlluminationPasses(void)")]
#[doc(alias = "__ZN4Ogre9Technique40checkManuallyOrganisedIlluminationPassesEv")]
// IDA 0xe40464: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40464() {
}


// 0xe404f0 — __ZN4Ogre9Technique26_compileIlluminationPassesEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::_compileIlluminationPasses(void)")]
#[doc(alias = "__ZN4Ogre9Technique26_compileIlluminationPassesEv")]
// IDA 0xe404f0: 791 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe404f0() {
}


// 0xe40cdc — __ZN4Ogre9Technique27getIlluminationPassIteratorEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::getIlluminationPassIterator(void)")]
#[doc(alias = "__ZN4Ogre9Technique27getIlluminationPassIteratorEv")]
// IDA 0xe40cdc: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40cdc() {
}


// 0xe40d08 — __ZNK4Ogre9Technique16getResourceGroupEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::getResourceGroup(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique16getResourceGroupEv")]
// IDA 0xe40d08: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40d08() {
}


// 0xe40d18 — __ZNK4Ogre9Technique19applyTextureAliasesERKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb
#[doc(alias = "Ogre::Technique::applyTextureAliases(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,bool)const")]
#[doc(alias = "__ZNK4Ogre9Technique19applyTextureAliasesERKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb")]
// IDA 0xe40d18: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40d18() {
}


// 0xe40d4c — __ZNK4Ogre9Technique23getShadowCasterMaterialEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::getShadowCasterMaterial(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique23getShadowCasterMaterialEv")]
// IDA 0xe40d4c: 22 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40d4c() {
}


// 0xe40d84 — __ZN4Ogre9Technique23setShadowCasterMaterialENS_11MaterialPtrE
#[doc(alias = "Ogre::Technique::setShadowCasterMaterial(Ogre::MaterialPtr)")]
#[doc(alias = "__ZN4Ogre9Technique23setShadowCasterMaterialENS_11MaterialPtrE")]
// IDA 0xe40d84: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40d84() {
}


// 0xe40de4 — __ZN4Ogre9Technique23setShadowCasterMaterialERKSs
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, const std::string *)
#[doc(alias = "Ogre::Technique::setShadowCasterMaterial(std::string const&)")]
#[doc(alias = "__ZN4Ogre9Technique23setShadowCasterMaterialERKSs")]
// IDA 0xe40de4: 185 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40de4() {
}


// 0xe40fc0 — __ZNK4Ogre9Technique25getShadowReceiverMaterialEv
// type: _DWORD __fastcall(Ogre::Technique *__hidden this)
#[doc(alias = "Ogre::Technique::getShadowReceiverMaterial(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique25getShadowReceiverMaterialEv")]
// IDA 0xe40fc0: 22 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40fc0() {
}


// 0xe40ff8 — __ZN4Ogre9Technique25setShadowReceiverMaterialERKSs
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, const std::string *)
#[doc(alias = "Ogre::Technique::setShadowReceiverMaterial(std::string const&)")]
#[doc(alias = "__ZN4Ogre9Technique25setShadowReceiverMaterialERKSs")]
// IDA 0xe40ff8: 185 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe40ff8() {
}


// 0xe411d4 — __ZN4Ogre9Technique16addGPUVendorRuleERKNS0_13GPUVendorRuleE
#[doc(alias = "Ogre::Technique::addGPUVendorRule(Ogre::Technique::GPUVendorRule const&)")]
#[doc(alias = "__ZN4Ogre9Technique16addGPUVendorRuleERKNS0_13GPUVendorRuleE")]
// IDA 0xe411d4: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe411d4() {
}


// 0xe413cc — __ZNSt6vectorIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
// type: int(void)
#[doc(alias = "std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_")]
// IDA 0xe413cc: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe413cc() {
}


// 0xe41cc4 — __ZNSt6vectorIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUVendorRule*,std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUVendorRule const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// IDA 0xe41cc4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xe41cc4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xe41de0 — __ZNSt6vectorIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::IlluminationPass **,std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::IlluminationPass * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// IDA 0xe41de0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xe41de0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xe42044 — __ZNSt6vectorIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Pass **,std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Pass * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// IDA 0xe42044: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xe42044() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xe42140 — __ZNSt12_Vector_baseIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// IDA 0xe42140: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xe42140() {
}


// 0xe42144 — __ZNSt12_Vector_baseIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// IDA 0xe42144: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xe42144() {
}


// 0xe42148 — __ZNSt12_Vector_baseIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// IDA 0xe42148: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xe42148() {
}


// 0xe42158 — __ZNSt12_Vector_baseIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// IDA 0xe42158: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe42158() {
}


// 0xe42164 — __ZNSt12_Vector_baseIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// IDA 0xe42164: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe42164() {
}


// 0xe42170 — __ZNSt12_Vector_baseIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// IDA 0xe42170: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe42170() {
}


// 0xe421b0 — __ZN4Ogre22TextAreaOverlayElementC1ERKSs
// type: _DWORD __fastcall(Ogre::TextAreaOverlayElement *__hidden this, const std::string *)
#[doc(alias = "Ogre::TextAreaOverlayElement::TextAreaOverlayElement(std::string const&)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElementC1ERKSs")]
// IDA 0xe421b0: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe421b0() {
}


// 0xe421bc — __ZN4Ogre22TextAreaOverlayElementC2ERKSs
// type: _DWORD __fastcall(Ogre::TextAreaOverlayElement *__hidden this, const std::string *)
#[doc(alias = "Ogre::TextAreaOverlayElement::TextAreaOverlayElement(std::string const&)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElementC2ERKSs")]
// IDA 0xe421bc: 254 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe421bc() {
}


// 0xe424ac — __ZN4Ogre22TextAreaOverlayElement10initialiseEv
// type: _DWORD __fastcall(Ogre::TextAreaOverlayElement *__hidden this)
#[doc(alias = "Ogre::TextAreaOverlayElement::initialise(void)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElement10initialiseEv")]
// IDA 0xe424ac: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe424ac() {
}


// 0xe42604 — __ZN4Ogre22TextAreaOverlayElement21checkMemoryAllocationEm
// type: _DWORD __fastcall(Ogre::TextAreaOverlayElement *__hidden this, unsigned int)
#[doc(alias = "Ogre::TextAreaOverlayElement::checkMemoryAllocation(unsigned long)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElement21checkMemoryAllocationEm")]
// IDA 0xe42604: 312 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe42604() {
}


// 0xe42910 — __ZN4Ogre22TextAreaOverlayElement22updatePositionGeometryEv
// type: _DWORD __fastcall(Ogre::TextAreaOverlayElement *__hidden this)
#[doc(alias = "Ogre::TextAreaOverlayElement::updatePositionGeometry(void)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElement22updatePositionGeometryEv")]
// IDA 0xe42910: 565 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe42910() {
}


// 0xe42f54 — __ZN4Ogre22TextAreaOverlayElement21updateTextureGeometryEv
// type: _DWORD __fastcall(Ogre::TextAreaOverlayElement *__hidden this)
#[doc(alias = "Ogre::TextAreaOverlayElement::updateTextureGeometry(void)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElement21updateTextureGeometryEv")]
// IDA 0xe42f54: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xe42f54() {
}


// 0xe42f58 — __ZN4Ogre22TextAreaOverlayElement10setCaptionERKNS_9UTFStringE
// type: _DWORD __fastcall(Ogre::TextAreaOverlayElement *__hidden this, const Ogre::UTFString *)
#[doc(alias = "Ogre::TextAreaOverlayElement::setCaption(Ogre::UTFString const&)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElement10setCaptionERKNS_9UTFStringE")]
// IDA 0xe42f58: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe42f58() {
}


// 0xe42f70 — __ZN4Ogre22TextAreaOverlayElement11setFontNameERKSs
// type: _DWORD __fastcall(Ogre::TextAreaOverlayElement *__hidden this, const std::string *)
#[doc(alias = "Ogre::TextAreaOverlayElement::setFontName(std::string const&)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElement11setFontNameERKSs")]
// IDA 0xe42f70: 349 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe42f70() {
}


// 0xe4332c — __ZN4Ogre22TextAreaOverlayElementD0Ev
// type: void __fastcall(Ogre::TextAreaOverlayElement *__hidden this)
#[doc(alias = "Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElementD0Ev")]
// IDA 0xe4332c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe4332c() {
}


// 0xe433bc — __ZN4Ogre22TextAreaOverlayElementD1Ev
// type: void __fastcall(Ogre::TextAreaOverlayElement *__hidden this)
#[doc(alias = "Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElementD1Ev")]
// IDA 0xe433bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe433bc() {
}


// 0xe433c8 — __ZThn12_N4Ogre22TextAreaOverlayElementD0Ev
// type: void __fastcall(Ogre::TextAreaOverlayElement *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
#[doc(alias = "__ZThn12_N4Ogre22TextAreaOverlayElementD0Ev")]
// IDA 0xe433c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe433c8() {
}


// 0xe4345c — __ZN4Ogre22TextAreaOverlayElementD2Ev
// type: void __fastcall(Ogre::TextAreaOverlayElement *__hidden this)
#[doc(alias = "Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElementD2Ev")]
// IDA 0xe4345c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe4345c() {
}


// 0xe43594 — __ZThn12_N4Ogre22TextAreaOverlayElementD1Ev
// type: void __fastcall(Ogre::TextAreaOverlayElement *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
#[doc(alias = "__ZThn12_N4Ogre22TextAreaOverlayElementD1Ev")]
// IDA 0xe43594: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe43594() {
}
