//! reflection — generated_watchdog_refl_wd2 — 120 stubs EA-sorted asc global gap filler 0xe3adb4..0xe43594 global-dedup //0xADDR
//! Source: ida/export.json (85545 funcs) EA asc not in crates — next 120 uncovered sorted asc (RBX::Reflection 19829 filtered exhausted, fallback global gap 1876 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Shard watchdog_refl_wd2 — watchdog shard wd2 (global gap filler EA-sorted asc after reflection exhausted)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xe3adb4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
pub fn stub_0xe3adb4() -> ! {
    todo!("0xe3adb4 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")
}

// 0xe3aed0 — __ZN4Ogre9SubEntityC1EPNS_6EntityEPNS_7SubMeshE
#[doc(alias = "Ogre::SubEntity::SubEntity(Ogre::Entity *,Ogre::SubMesh *)")]
#[doc(alias = "__ZN4Ogre9SubEntityC1EPNS_6EntityEPNS_7SubMeshE")]
pub fn stub_0xe3aed0() -> ! {
    todo!("0xe3aed0 Ogre::SubEntity::SubEntity(Ogre::Entity *,Ogre::SubMesh *)")
}

// 0xe3aedc — __ZN4Ogre9SubEntityC2EPNS_6EntityEPNS_7SubMeshE
#[doc(alias = "Ogre::SubEntity::SubEntity(Ogre::Entity *,Ogre::SubMesh *)")]
#[doc(alias = "__ZN4Ogre9SubEntityC2EPNS_6EntityEPNS_7SubMeshE")]
pub fn stub_0xe3aedc() -> ! {
    todo!("0xe3aedc Ogre::SubEntity::SubEntity(Ogre::Entity *,Ogre::SubMesh *)")
}

// 0xe3b554 — __ZN4Ogre9SubEntityD0Ev
#[doc(alias = "Ogre::SubEntity::~SubEntity()")]
#[doc(alias = "__ZN4Ogre9SubEntityD0Ev")]
pub fn stub_0xe3b554() {
    // IDA 0xe3b554: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xe3b5e4 — __ZN4Ogre9SubEntityD1Ev
#[doc(alias = "Ogre::SubEntity::~SubEntity()")]
#[doc(alias = "__ZN4Ogre9SubEntityD1Ev")]
pub fn stub_0xe3b5e4() {
    // IDA 0xe3b5e4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xe3b5f0 — __ZN4Ogre9SubEntityD2Ev
#[doc(alias = "Ogre::SubEntity::~SubEntity()")]
#[doc(alias = "__ZN4Ogre9SubEntityD2Ev")]
pub fn stub_0xe3b5f0() {
    // IDA 0xe3b5f0: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xe3b7cc — __ZN4Ogre9SubEntity10getSubMeshEv
#[doc(alias = "Ogre::SubEntity::getSubMesh(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity10getSubMeshEv")]
pub fn stub_0xe3b7cc() -> ! {
    todo!("0xe3b7cc Ogre::SubEntity::getSubMesh(void)")
}

// 0xe3b7d0 — __ZNK4Ogre9SubEntity15getMaterialNameEv
#[doc(alias = "Ogre::SubEntity::getMaterialName(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity15getMaterialNameEv")]
pub fn stub_0xe3b7d0() -> ! {
    todo!("0xe3b7d0 Ogre::SubEntity::getMaterialName(void)const")
}

// 0xe3b7d4 — __ZN4Ogre9SubEntity15setMaterialNameERKSsS2_
#[doc(alias = "Ogre::SubEntity::setMaterialName(std::string const&,std::string const&)")]
#[doc(alias = "__ZN4Ogre9SubEntity15setMaterialNameERKSsS2_")]
pub fn stub_0xe3b7d4() -> ! {
    todo!("0xe3b7d4 Ogre::SubEntity::setMaterialName(std::string const&,std::string const&)")
}

// 0xe3c25c — __ZN4Ogre9SubEntity11setMaterialERKNS_11MaterialPtrE
#[doc(alias = "Ogre::SubEntity::setMaterial(Ogre::MaterialPtr const&)")]
#[doc(alias = "__ZN4Ogre9SubEntity11setMaterialERKNS_11MaterialPtrE")]
pub fn stub_0xe3c25c() -> ! {
    todo!("0xe3c25c Ogre::SubEntity::setMaterial(Ogre::MaterialPtr const&)")
}

// 0xe3c968 — __ZNK4Ogre9SubEntity11getMaterialEv
#[doc(alias = "Ogre::SubEntity::getMaterial(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity11getMaterialEv")]
pub fn stub_0xe3c968() -> ! {
    todo!("0xe3c968 Ogre::SubEntity::getMaterial(void)const")
}

// 0xe3c96c — __ZNK4Ogre9SubEntity12getTechniqueEv
#[doc(alias = "Ogre::SubEntity::getTechnique(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity12getTechniqueEv")]
pub fn stub_0xe3c96c() -> ! {
    todo!("0xe3c96c Ogre::SubEntity::getTechnique(void)const")
}

// 0xe3c980 — __ZN4Ogre9SubEntity18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::SubEntity::getRenderOperation(Ogre::RenderOperation &)")]
#[doc(alias = "__ZN4Ogre9SubEntity18getRenderOperationERNS_15RenderOperationE")]
pub fn stub_0xe3c980() -> ! {
    todo!("0xe3c980 Ogre::SubEntity::getRenderOperation(Ogre::RenderOperation &)")
}

// 0xe3cd18 — __ZNK4Ogre9SubEntity9getLightsEv
#[doc(alias = "Ogre::SubEntity::getLights(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity9getLightsEv")]
pub fn stub_0xe3cd18() -> ! {
    todo!("0xe3cd18 Ogre::SubEntity::getLights(void)const")
}

// 0xe3cd28 — __ZN4Ogre9SubEntity10setVisibleEb
#[doc(alias = "Ogre::SubEntity::setVisible(bool)")]
#[doc(alias = "__ZN4Ogre9SubEntity10setVisibleEb")]
pub fn stub_0xe3cd28() -> ! {
    todo!("0xe3cd28 Ogre::SubEntity::setVisible(bool)")
}

// 0xe3cd30 — __ZNK4Ogre9SubEntity9isVisibleEv
#[doc(alias = "Ogre::SubEntity::isVisible(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity9isVisibleEv")]
pub fn stub_0xe3cd30() -> ! {
    todo!("0xe3cd30 Ogre::SubEntity::isVisible(void)const")
}

// 0xe3cd38 — __ZN4Ogre9SubEntity23prepareTempBlendBuffersEv
#[doc(alias = "Ogre::SubEntity::prepareTempBlendBuffers(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity23prepareTempBlendBuffersEv")]
pub fn stub_0xe3cd38() -> ! {
    todo!("0xe3cd38 Ogre::SubEntity::prepareTempBlendBuffers(void)")
}

// 0xe3ce9c — __ZNK4Ogre9SubEntity15getCastsShadowsEv
#[doc(alias = "Ogre::SubEntity::getCastsShadows(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity15getCastsShadowsEv")]
pub fn stub_0xe3ce9c() -> ! {
    todo!("0xe3ce9c Ogre::SubEntity::getCastsShadows(void)const")
}

// 0xe3ceac — __ZN4Ogre9SubEntity22_getSkelAnimVertexDataEv
#[doc(alias = "Ogre::SubEntity::_getSkelAnimVertexData(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity22_getSkelAnimVertexDataEv")]
pub fn stub_0xe3ceac() -> ! {
    todo!("0xe3ceac Ogre::SubEntity::_getSkelAnimVertexData(void)")
}

// 0xe3ceb0 — __ZN4Ogre9SubEntity32_getSoftwareVertexAnimVertexDataEv
#[doc(alias = "Ogre::SubEntity::_getSoftwareVertexAnimVertexData(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity32_getSoftwareVertexAnimVertexDataEv")]
pub fn stub_0xe3ceb0() -> ! {
    todo!("0xe3ceb0 Ogre::SubEntity::_getSoftwareVertexAnimVertexData(void)")
}

// 0xe3ceb8 — __ZN4Ogre9SubEntity32_getHardwareVertexAnimVertexDataEv
#[doc(alias = "Ogre::SubEntity::_getHardwareVertexAnimVertexData(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity32_getHardwareVertexAnimVertexDataEv")]
pub fn stub_0xe3ceb8() -> ! {
    todo!("0xe3ceb8 Ogre::SubEntity::_getHardwareVertexAnimVertexData(void)")
}

// 0xe3cec0 — __ZN4Ogre9SubEntity28_getVertexAnimTempBufferInfoEv
#[doc(alias = "Ogre::SubEntity::_getVertexAnimTempBufferInfo(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity28_getVertexAnimTempBufferInfoEv")]
pub fn stub_0xe3cec0() -> ! {
    todo!("0xe3cec0 Ogre::SubEntity::_getVertexAnimTempBufferInfo(void)")
}

// 0xe3cec4 — __ZNK4Ogre9SubEntity25_updateCustomGpuParameterERKNS_20GpuProgramParameters17AutoConstantEntryEPS1_
#[doc(alias = "Ogre::SubEntity::_updateCustomGpuParameter(Ogre::GpuProgramParameters::AutoConstantEntry const&,Ogre::GpuProgramParameters*)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity25_updateCustomGpuParameterERKNS_20GpuProgramParameters17AutoConstantEntryEPS1_")]
pub fn stub_0xe3cec4() -> ! {
    todo!("0xe3cec4 Ogre::SubEntity::_updateCustomGpuParameter(Ogre::GpuProgramParameters::AutoConstantEntry const&,Ogre::GpuProgramParameters*)const")
}

// 0xe3cf64 — __ZN4Ogre9SubEntity30_markBuffersUnusedForAnimationEv
#[doc(alias = "Ogre::SubEntity::_markBuffersUnusedForAnimation(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity30_markBuffersUnusedForAnimationEv")]
pub fn stub_0xe3cf64() -> ! {
    todo!("0xe3cf64 Ogre::SubEntity::_markBuffersUnusedForAnimation(void)")
}

// 0xe3cf6c — __ZN4Ogre9SubEntity28_markBuffersUsedForAnimationEv
#[doc(alias = "Ogre::SubEntity::_markBuffersUsedForAnimation(void)")]
#[doc(alias = "__ZN4Ogre9SubEntity28_markBuffersUsedForAnimationEv")]
pub fn stub_0xe3cf6c() -> ! {
    todo!("0xe3cf6c Ogre::SubEntity::_markBuffersUsedForAnimation(void)")
}

// 0xe3cf74 — __ZN4Ogre9SubEntity33_restoreBuffersForUnusedAnimationEb
#[doc(alias = "Ogre::SubEntity::_restoreBuffersForUnusedAnimation(bool)")]
#[doc(alias = "__ZN4Ogre9SubEntity33_restoreBuffersForUnusedAnimationEb")]
pub fn stub_0xe3cf74() -> ! {
    todo!("0xe3cf74 Ogre::SubEntity::_restoreBuffersForUnusedAnimation(bool)")
}

// 0xe3d1c0 — __ZN4Ogre9SubEntity19setRenderQueueGroupEh
#[doc(alias = "Ogre::SubEntity::setRenderQueueGroup(unsigned char)")]
#[doc(alias = "__ZN4Ogre9SubEntity19setRenderQueueGroupEh")]
pub fn stub_0xe3d1c0() -> ! {
    todo!("0xe3d1c0 Ogre::SubEntity::setRenderQueueGroup(unsigned char)")
}

// 0xe3d1cc — __ZN4Ogre9SubEntity30setRenderQueueGroupAndPriorityEht
#[doc(alias = "Ogre::SubEntity::setRenderQueueGroupAndPriority(unsigned char,unsigned short)")]
#[doc(alias = "__ZN4Ogre9SubEntity30setRenderQueueGroupAndPriorityEht")]
pub fn stub_0xe3d1cc() -> ! {
    todo!("0xe3d1cc Ogre::SubEntity::setRenderQueueGroupAndPriority(unsigned char,unsigned short)")
}

// 0xe3d1e8 — __ZNK4Ogre9SubEntity19getRenderQueueGroupEv
#[doc(alias = "Ogre::SubEntity::getRenderQueueGroup(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity19getRenderQueueGroupEv")]
pub fn stub_0xe3d1e8() -> ! {
    todo!("0xe3d1e8 Ogre::SubEntity::getRenderQueueGroup(void)const")
}

// 0xe3d1f0 — __ZNK4Ogre9SubEntity22getRenderQueuePriorityEv
#[doc(alias = "Ogre::SubEntity::getRenderQueuePriority(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity22getRenderQueuePriorityEv")]
pub fn stub_0xe3d1f0() -> ! {
    todo!("0xe3d1f0 Ogre::SubEntity::getRenderQueuePriority(void)const")
}

// 0xe3d1f8 — __ZNK4Ogre9SubEntity21isRenderQueueGroupSetEv
#[doc(alias = "Ogre::SubEntity::isRenderQueueGroupSet(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity21isRenderQueueGroupSetEv")]
pub fn stub_0xe3d1f8() -> ! {
    todo!("0xe3d1f8 Ogre::SubEntity::isRenderQueueGroupSet(void)const")
}

// 0xe3d200 — __ZNK4Ogre9SubEntity24isRenderQueuePrioritySetEv
#[doc(alias = "Ogre::SubEntity::isRenderQueuePrioritySet(void)const")]
#[doc(alias = "__ZNK4Ogre9SubEntity24isRenderQueuePrioritySetEv")]
pub fn stub_0xe3d200() -> ! {
    todo!("0xe3d200 Ogre::SubEntity::isRenderQueuePrioritySet(void)const")
}

// 0xe3d23c — __ZN4Ogre7SubMeshC1Ev
#[doc(alias = "Ogre::SubMesh::SubMesh(void)")]
#[doc(alias = "__ZN4Ogre7SubMeshC1Ev")]
pub fn stub_0xe3d23c() -> ! {
    todo!("0xe3d23c Ogre::SubMesh::SubMesh(void)")
}

// 0xe3d248 — __ZN4Ogre7SubMeshC2Ev
#[doc(alias = "Ogre::SubMesh::SubMesh(void)")]
#[doc(alias = "__ZN4Ogre7SubMeshC2Ev")]
pub fn stub_0xe3d248() -> ! {
    todo!("0xe3d248 Ogre::SubMesh::SubMesh(void)")
}

// 0xe3d498 — __ZN4Ogre7SubMeshD1Ev
#[doc(alias = "Ogre::SubMesh::~SubMesh()")]
#[doc(alias = "__ZN4Ogre7SubMeshD1Ev")]
pub fn stub_0xe3d498() {
    // IDA 0xe3d498: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xe3d4a4 — __ZN4Ogre7SubMeshD2Ev
#[doc(alias = "Ogre::SubMesh::~SubMesh()")]
#[doc(alias = "__ZN4Ogre7SubMeshD2Ev")]
pub fn stub_0xe3d4a4() {
    // IDA 0xe3d4a4: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xe3d620 — __ZN4Ogre7SubMesh15removeLodLevelsEv
#[doc(alias = "Ogre::SubMesh::removeLodLevels(void)")]
#[doc(alias = "__ZN4Ogre7SubMesh15removeLodLevelsEv")]
pub fn stub_0xe3d620() -> ! {
    todo!("0xe3d620 Ogre::SubMesh::removeLodLevels(void)")
}

// 0xe3d6d4 — __ZN4Ogre7SubMesh15setMaterialNameERKSsS2_
#[doc(alias = "Ogre::SubMesh::setMaterialName(std::string const&,std::string const&)")]
#[doc(alias = "__ZN4Ogre7SubMesh15setMaterialNameERKSsS2_")]
pub fn stub_0xe3d6d4() -> ! {
    todo!("0xe3d6d4 Ogre::SubMesh::setMaterialName(std::string const&,std::string const&)")
}

// 0xe3d6ec — __ZNK4Ogre7SubMesh15getMaterialNameEv
#[doc(alias = "Ogre::SubMesh::getMaterialName(void)const")]
#[doc(alias = "__ZNK4Ogre7SubMesh15getMaterialNameEv")]
pub fn stub_0xe3d6ec() -> ! {
    todo!("0xe3d6ec Ogre::SubMesh::getMaterialName(void)const")
}

// 0xe3d6f0 — __ZNK4Ogre7SubMesh16isMatInitialisedEv
#[doc(alias = "Ogre::SubMesh::isMatInitialised(void)const")]
#[doc(alias = "__ZNK4Ogre7SubMesh16isMatInitialisedEv")]
pub fn stub_0xe3d6f0() -> ! {
    todo!("0xe3d6f0 Ogre::SubMesh::isMatInitialised(void)const")
}

// 0xe3d6f8 — __ZN4Ogre7SubMesh19_getRenderOperationERNS_15RenderOperationEt
#[doc(alias = "Ogre::SubMesh::_getRenderOperation(Ogre::RenderOperation &,unsigned short)")]
#[doc(alias = "__ZN4Ogre7SubMesh19_getRenderOperationERNS_15RenderOperationEt")]
pub fn stub_0xe3d6f8() -> ! {
    todo!("0xe3d6f8 Ogre::SubMesh::_getRenderOperation(Ogre::RenderOperation &,unsigned short)")
}

// 0xe3d73c — __ZN4Ogre7SubMesh17addBoneAssignmentERKNS_22VertexBoneAssignment_sE
#[doc(alias = "Ogre::SubMesh::addBoneAssignment(Ogre::VertexBoneAssignment_s const&)")]
#[doc(alias = "__ZN4Ogre7SubMesh17addBoneAssignmentERKNS_22VertexBoneAssignment_sE")]
pub fn stub_0xe3d73c() -> ! {
    todo!("0xe3d73c Ogre::SubMesh::addBoneAssignment(Ogre::VertexBoneAssignment_s const&)")
}

// 0xe3d994 — __ZN4Ogre7SubMesh23_compileBoneAssignmentsEv
#[doc(alias = "Ogre::SubMesh::_compileBoneAssignments(void)")]
#[doc(alias = "__ZN4Ogre7SubMesh23_compileBoneAssignmentsEv")]
pub fn stub_0xe3d994() -> ! {
    todo!("0xe3d994 Ogre::SubMesh::_compileBoneAssignments(void)")
}

// 0xe3d9cc — __ZN4Ogre7SubMesh15addTextureAliasERKSsS2_
#[doc(alias = "Ogre::SubMesh::addTextureAlias(std::string const&,std::string const&)")]
#[doc(alias = "__ZN4Ogre7SubMesh15addTextureAliasERKSsS2_")]
pub fn stub_0xe3d9cc() -> ! {
    todo!("0xe3d9cc Ogre::SubMesh::addTextureAlias(std::string const&,std::string const&)")
}

// 0xe3d9e0 — __ZN4Ogre7SubMesh33updateMaterialUsingTextureAliasesEv
#[doc(alias = "Ogre::SubMesh::updateMaterialUsingTextureAliases(void)")]
#[doc(alias = "__ZN4Ogre7SubMesh33updateMaterialUsingTextureAliasesEv")]
pub fn stub_0xe3d9e0() -> ! {
    todo!("0xe3d9e0 Ogre::SubMesh::updateMaterialUsingTextureAliases(void)")
}

// 0xe3e0e0 — __ZNK4Ogre7SubMesh22getVertexAnimationTypeEv
#[doc(alias = "Ogre::SubMesh::getVertexAnimationType(void)const")]
#[doc(alias = "__ZNK4Ogre7SubMesh22getVertexAnimationTypeEv")]
pub fn stub_0xe3e0e0() -> ! {
    todo!("0xe3e0e0 Ogre::SubMesh::getVertexAnimationType(void)const")
}

// 0xe3e0fc — __ZNSt12_Vector_baseIPN4Ogre9IndexDataENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::IndexData *,Ogre::STLAllocator<Ogre::IndexData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre9IndexDataENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
pub fn stub_0xe3e0fc() {
    // IDA 0xe3e0fc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xe3e100 — __ZNSt12_Vector_baseIPN4Ogre9IndexDataENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::IndexData *,Ogre::STLAllocator<Ogre::IndexData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre9IndexDataENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
pub fn stub_0xe3e100() {
    // IDA 0xe3e100: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xe3e140 — __ZNK4Ogre8TagPoint15getParentEntityEv
#[doc(alias = "Ogre::TagPoint::getParentEntity(void)const")]
#[doc(alias = "__ZNK4Ogre8TagPoint15getParentEntityEv")]
pub fn stub_0xe3e140() -> ! {
    todo!("0xe3e140 Ogre::TagPoint::getParentEntity(void)const")
}

// 0xe3e148 — __ZNK4Ogre8TagPoint22_getFullLocalTransformEv
#[doc(alias = "Ogre::TagPoint::_getFullLocalTransform(void)const")]
#[doc(alias = "__ZNK4Ogre8TagPoint22_getFullLocalTransformEv")]
pub fn stub_0xe3e148() -> ! {
    todo!("0xe3e148 Ogre::TagPoint::_getFullLocalTransform(void)const")
}

// 0xe3e1b8 — __ZN4Ogre9TechniqueC1EPNS_8MaterialE
#[doc(alias = "Ogre::Technique::Technique(Ogre::Material *)")]
#[doc(alias = "__ZN4Ogre9TechniqueC1EPNS_8MaterialE")]
pub fn stub_0xe3e1b8() -> ! {
    todo!("0xe3e1b8 Ogre::Technique::Technique(Ogre::Material *)")
}

// 0xe3e1c4 — __ZN4Ogre9TechniqueC2EPNS_8MaterialE
#[doc(alias = "Ogre::Technique::Technique(Ogre::Material *)")]
#[doc(alias = "__ZN4Ogre9TechniqueC2EPNS_8MaterialE")]
pub fn stub_0xe3e1c4() -> ! {
    todo!("0xe3e1c4 Ogre::Technique::Technique(Ogre::Material *)")
}

// 0xe3e4e8 — __ZN4Ogre9TechniqueaSERKS0_
#[doc(alias = "Ogre::Technique::operator=(Ogre::Technique const&)")]
#[doc(alias = "__ZN4Ogre9TechniqueaSERKS0_")]
pub fn stub_0xe3e4e8() -> ! {
    todo!("0xe3e4e8 Ogre::Technique::operator=(Ogre::Technique const&)")
}

// 0xe3e6c8 — __ZN4Ogre9TechniqueD1Ev
#[doc(alias = "Ogre::Technique::~Technique()")]
#[doc(alias = "__ZN4Ogre9TechniqueD1Ev")]
pub fn stub_0xe3e6c8() {
    // IDA 0xe3e6c8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xe3e6d4 — __ZN4Ogre9TechniqueD2Ev
#[doc(alias = "Ogre::Technique::~Technique()")]
#[doc(alias = "__ZN4Ogre9TechniqueD2Ev")]
pub fn stub_0xe3e6d4() {
    // IDA 0xe3e6d4: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xe3ec68 — __ZNK4Ogre9Technique11isSupportedEv
#[doc(alias = "Ogre::Technique::isSupported(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique11isSupportedEv")]
pub fn stub_0xe3ec68() -> ! {
    todo!("0xe3ec68 Ogre::Technique::isSupported(void)const")
}

// 0xe3ec70 — __ZN4Ogre9Technique8_compileEb
#[doc(alias = "Ogre::Technique::_compile(bool)")]
#[doc(alias = "__ZN4Ogre9Technique8_compileEb")]
pub fn stub_0xe3ec70() -> ! {
    todo!("0xe3ec70 Ogre::Technique::_compile(bool)")
}

// 0xe3eee0 — __ZN4Ogre9Technique13checkGPURulesERSt18basic_stringstreamIcSt11char_traitsIcESaIcEE
#[doc(alias = "Ogre::Technique::checkGPURules(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> &)")]
#[doc(alias = "__ZN4Ogre9Technique13checkGPURulesERSt18basic_stringstreamIcSt11char_traitsIcESaIcEE")]
pub fn stub_0xe3eee0() -> ! {
    todo!("0xe3eee0 Ogre::Technique::checkGPURules(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> &)")
}

// 0xe3f704 — __ZN4Ogre9Technique20checkHardwareSupportEbRSt18basic_stringstreamIcSt11char_traitsIcESaIcEE
#[doc(alias = "Ogre::Technique::checkHardwareSupport(bool,std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> &)")]
#[doc(alias = "__ZN4Ogre9Technique20checkHardwareSupportEbRSt18basic_stringstreamIcSt11char_traitsIcESaIcEE")]
pub fn stub_0xe3f704() -> ! {
    todo!("0xe3f704 Ogre::Technique::checkHardwareSupport(bool,std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> &)")
}

// 0xe3fc54 — __ZN4Ogre9Technique10createPassEv
#[doc(alias = "Ogre::Technique::createPass(void)")]
#[doc(alias = "__ZN4Ogre9Technique10createPassEv")]
pub fn stub_0xe3fc54() -> ! {
    todo!("0xe3fc54 Ogre::Technique::createPass(void)")
}

// 0xe3fd54 — __ZN4Ogre9Technique7getPassEt
#[doc(alias = "Ogre::Technique::getPass(unsigned short)")]
#[doc(alias = "__ZN4Ogre9Technique7getPassEt")]
pub fn stub_0xe3fd54() -> ! {
    todo!("0xe3fd54 Ogre::Technique::getPass(unsigned short)")
}

// 0xe3fd5c — __ZN4Ogre9Technique7getPassERKSs
#[doc(alias = "Ogre::Technique::getPass(std::string const&)")]
#[doc(alias = "__ZN4Ogre9Technique7getPassERKSs")]
pub fn stub_0xe3fd5c() -> ! {
    todo!("0xe3fd5c Ogre::Technique::getPass(std::string const&)")
}

// 0xe3fdbc — __ZNK4Ogre9Technique12getNumPassesEv
#[doc(alias = "Ogre::Technique::getNumPasses(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique12getNumPassesEv")]
pub fn stub_0xe3fdbc() -> ! {
    todo!("0xe3fdbc Ogre::Technique::getNumPasses(void)const")
}

// 0xe3fdc8 — __ZN4Ogre9Technique15getPassIteratorEv
#[doc(alias = "Ogre::Technique::getPassIterator(void)")]
#[doc(alias = "__ZN4Ogre9Technique15getPassIteratorEv")]
pub fn stub_0xe3fdc8() -> ! {
    todo!("0xe3fdc8 Ogre::Technique::getPassIterator(void)")
}

// 0xe3fdd4 — __ZNK4Ogre9Technique13isTransparentEv
#[doc(alias = "Ogre::Technique::isTransparent(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique13isTransparentEv")]
pub fn stub_0xe3fdd4() -> ! {
    todo!("0xe3fdd4 Ogre::Technique::isTransparent(void)const")
}

// 0xe3fdec — __ZNK4Ogre9Technique27isTransparentSortingEnabledEv
#[doc(alias = "Ogre::Technique::isTransparentSortingEnabled(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique27isTransparentSortingEnabledEv")]
pub fn stub_0xe3fdec() -> ! {
    todo!("0xe3fdec Ogre::Technique::isTransparentSortingEnabled(void)const")
}

// 0xe3fe04 — __ZNK4Ogre9Technique26isTransparentSortingForcedEv
#[doc(alias = "Ogre::Technique::isTransparentSortingForced(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique26isTransparentSortingForcedEv")]
pub fn stub_0xe3fe04() -> ! {
    todo!("0xe3fe04 Ogre::Technique::isTransparentSortingForced(void)const")
}

// 0xe3fe1c — __ZNK4Ogre9Technique19isDepthWriteEnabledEv
#[doc(alias = "Ogre::Technique::isDepthWriteEnabled(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique19isDepthWriteEnabledEv")]
pub fn stub_0xe3fe1c() -> ! {
    todo!("0xe3fe1c Ogre::Technique::isDepthWriteEnabled(void)const")
}

// 0xe3fe34 — __ZNK4Ogre9Technique19isDepthCheckEnabledEv
#[doc(alias = "Ogre::Technique::isDepthCheckEnabled(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique19isDepthCheckEnabledEv")]
pub fn stub_0xe3fe34() -> ! {
    todo!("0xe3fe34 Ogre::Technique::isDepthCheckEnabled(void)const")
}

// 0xe3fe4c — __ZNK4Ogre9Technique22hasColourWriteDisabledEv
#[doc(alias = "Ogre::Technique::hasColourWriteDisabled(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique22hasColourWriteDisabledEv")]
pub fn stub_0xe3fe4c() -> ! {
    todo!("0xe3fe4c Ogre::Technique::hasColourWriteDisabled(void)const")
}

// 0xe3fe68 — __ZN4Ogre9Technique8_prepareEv
#[doc(alias = "Ogre::Technique::_prepare(void)")]
#[doc(alias = "__ZN4Ogre9Technique8_prepareEv")]
pub fn stub_0xe3fe68() -> ! {
    todo!("0xe3fe68 Ogre::Technique::_prepare(void)")
}

// 0xe3fea4 — __ZN4Ogre9Technique10_unprepareEv
#[doc(alias = "Ogre::Technique::_unprepare(void)")]
#[doc(alias = "__ZN4Ogre9Technique10_unprepareEv")]
pub fn stub_0xe3fea4() -> ! {
    todo!("0xe3fea4 Ogre::Technique::_unprepare(void)")
}

// 0xe3fec0 — __ZN4Ogre9Technique5_loadEv
#[doc(alias = "Ogre::Technique::_load(void)")]
#[doc(alias = "__ZN4Ogre9Technique5_loadEv")]
pub fn stub_0xe3fec0() -> ! {
    todo!("0xe3fec0 Ogre::Technique::_load(void)")
}

// 0xe402ac — __ZN4Ogre9Technique7_unloadEv
#[doc(alias = "Ogre::Technique::_unload(void)")]
#[doc(alias = "__ZN4Ogre9Technique7_unloadEv")]
pub fn stub_0xe402ac() -> ! {
    todo!("0xe402ac Ogre::Technique::_unload(void)")
}

// 0xe402c8 — __ZNK4Ogre9Technique8isLoadedEv
#[doc(alias = "Ogre::Technique::isLoaded(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique8isLoadedEv")]
pub fn stub_0xe402c8() -> ! {
    todo!("0xe402c8 Ogre::Technique::isLoaded(void)const")
}

// 0xe402ec — __ZN4Ogre9Technique20setDepthCheckEnabledEb
#[doc(alias = "Ogre::Technique::setDepthCheckEnabled(bool)")]
#[doc(alias = "__ZN4Ogre9Technique20setDepthCheckEnabledEb")]
pub fn stub_0xe402ec() -> ! {
    todo!("0xe402ec Ogre::Technique::setDepthCheckEnabled(bool)")
}

// 0xe4030c — __ZN4Ogre9Technique20setDepthWriteEnabledEb
#[doc(alias = "Ogre::Technique::setDepthWriteEnabled(bool)")]
#[doc(alias = "__ZN4Ogre9Technique20setDepthWriteEnabledEb")]
pub fn stub_0xe4030c() -> ! {
    todo!("0xe4030c Ogre::Technique::setDepthWriteEnabled(bool)")
}

// 0xe4032c — __ZN4Ogre9Technique18setLightingEnabledEb
#[doc(alias = "Ogre::Technique::setLightingEnabled(bool)")]
#[doc(alias = "__ZN4Ogre9Technique18setLightingEnabledEb")]
pub fn stub_0xe4032c() -> ! {
    todo!("0xe4032c Ogre::Technique::setLightingEnabled(bool)")
}

// 0xe4034c — __ZN4Ogre9Technique6setFogEbNS_7FogModeERKNS_11ColourValueEfff
#[doc(alias = "Ogre::Technique::setFog(bool,Ogre::FogMode,Ogre::ColourValue const&,float,float,float)")]
#[doc(alias = "__ZN4Ogre9Technique6setFogEbNS_7FogModeERKNS_11ColourValueEfff")]
pub fn stub_0xe4034c() -> ! {
    todo!("0xe4034c Ogre::Technique::setFog(bool,Ogre::FogMode,Ogre::ColourValue const&,float,float,float)")
}

// 0xe403b4 — __ZN4Ogre9Technique16setSceneBlendingENS_14SceneBlendTypeE
#[doc(alias = "Ogre::Technique::setSceneBlending(Ogre::SceneBlendType)")]
#[doc(alias = "__ZN4Ogre9Technique16setSceneBlendingENS_14SceneBlendTypeE")]
pub fn stub_0xe403b4() -> ! {
    todo!("0xe403b4 Ogre::Technique::setSceneBlending(Ogre::SceneBlendType)")
}

// 0xe403d4 — __ZN4Ogre9Technique16setSceneBlendingENS_16SceneBlendFactorES1_
#[doc(alias = "Ogre::Technique::setSceneBlending(Ogre::SceneBlendFactor,Ogre::SceneBlendFactor)")]
#[doc(alias = "__ZN4Ogre9Technique16setSceneBlendingENS_16SceneBlendFactorES1_")]
pub fn stub_0xe403d4() -> ! {
    todo!("0xe403d4 Ogre::Technique::setSceneBlending(Ogre::SceneBlendFactor,Ogre::SceneBlendFactor)")
}

// 0xe40400 — __ZN4Ogre9Technique7setNameERKSs
#[doc(alias = "Ogre::Technique::setName(std::string const&)")]
#[doc(alias = "__ZN4Ogre9Technique7setNameERKSs")]
pub fn stub_0xe40400() -> ! {
    todo!("0xe40400 Ogre::Technique::setName(std::string const&)")
}

// 0xe4040c — __ZN4Ogre9Technique21_notifyNeedsRecompileEv
#[doc(alias = "Ogre::Technique::_notifyNeedsRecompile(void)")]
#[doc(alias = "__ZN4Ogre9Technique21_notifyNeedsRecompileEv")]
pub fn stub_0xe4040c() -> ! {
    todo!("0xe4040c Ogre::Technique::_notifyNeedsRecompile(void)")
}

// 0xe40420 — __ZN4Ogre9Technique11setLodIndexEt
#[doc(alias = "Ogre::Technique::setLodIndex(unsigned short)")]
#[doc(alias = "__ZN4Ogre9Technique11setLodIndexEt")]
pub fn stub_0xe40420() -> ! {
    todo!("0xe40420 Ogre::Technique::setLodIndex(unsigned short)")
}

// 0xe40438 — __ZN4Ogre9Technique13setSchemeNameERKSs
#[doc(alias = "Ogre::Technique::setSchemeName(std::string const&)")]
#[doc(alias = "__ZN4Ogre9Technique13setSchemeNameERKSs")]
pub fn stub_0xe40438() -> ! {
    todo!("0xe40438 Ogre::Technique::setSchemeName(std::string const&)")
}

// 0xe40460 — __ZNK4Ogre9Technique15_getSchemeIndexEv
#[doc(alias = "Ogre::Technique::_getSchemeIndex(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique15_getSchemeIndexEv")]
pub fn stub_0xe40460() -> ! {
    todo!("0xe40460 Ogre::Technique::_getSchemeIndex(void)const")
}

// 0xe40464 — __ZN4Ogre9Technique40checkManuallyOrganisedIlluminationPassesEv
#[doc(alias = "Ogre::Technique::checkManuallyOrganisedIlluminationPasses(void)")]
#[doc(alias = "__ZN4Ogre9Technique40checkManuallyOrganisedIlluminationPassesEv")]
pub fn stub_0xe40464() -> ! {
    todo!("0xe40464 Ogre::Technique::checkManuallyOrganisedIlluminationPasses(void)")
}

// 0xe404f0 — __ZN4Ogre9Technique26_compileIlluminationPassesEv
#[doc(alias = "Ogre::Technique::_compileIlluminationPasses(void)")]
#[doc(alias = "__ZN4Ogre9Technique26_compileIlluminationPassesEv")]
pub fn stub_0xe404f0() -> ! {
    todo!("0xe404f0 Ogre::Technique::_compileIlluminationPasses(void)")
}

// 0xe40cdc — __ZN4Ogre9Technique27getIlluminationPassIteratorEv
#[doc(alias = "Ogre::Technique::getIlluminationPassIterator(void)")]
#[doc(alias = "__ZN4Ogre9Technique27getIlluminationPassIteratorEv")]
pub fn stub_0xe40cdc() -> ! {
    todo!("0xe40cdc Ogre::Technique::getIlluminationPassIterator(void)")
}

// 0xe40d08 — __ZNK4Ogre9Technique16getResourceGroupEv
#[doc(alias = "Ogre::Technique::getResourceGroup(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique16getResourceGroupEv")]
pub fn stub_0xe40d08() -> ! {
    todo!("0xe40d08 Ogre::Technique::getResourceGroup(void)const")
}

// 0xe40d18 — __ZNK4Ogre9Technique19applyTextureAliasesERKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb
#[doc(alias = "Ogre::Technique::applyTextureAliases(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,bool)const")]
#[doc(alias = "__ZNK4Ogre9Technique19applyTextureAliasesERKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb")]
pub fn stub_0xe40d18() -> ! {
    todo!("0xe40d18 Ogre::Technique::applyTextureAliases(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,bool)const")
}

// 0xe40d4c — __ZNK4Ogre9Technique23getShadowCasterMaterialEv
#[doc(alias = "Ogre::Technique::getShadowCasterMaterial(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique23getShadowCasterMaterialEv")]
pub fn stub_0xe40d4c() -> ! {
    todo!("0xe40d4c Ogre::Technique::getShadowCasterMaterial(void)const")
}

// 0xe40d84 — __ZN4Ogre9Technique23setShadowCasterMaterialENS_11MaterialPtrE
#[doc(alias = "Ogre::Technique::setShadowCasterMaterial(Ogre::MaterialPtr)")]
#[doc(alias = "__ZN4Ogre9Technique23setShadowCasterMaterialENS_11MaterialPtrE")]
pub fn stub_0xe40d84() -> ! {
    todo!("0xe40d84 Ogre::Technique::setShadowCasterMaterial(Ogre::MaterialPtr)")
}

// 0xe40de4 — __ZN4Ogre9Technique23setShadowCasterMaterialERKSs
#[doc(alias = "Ogre::Technique::setShadowCasterMaterial(std::string const&)")]
#[doc(alias = "__ZN4Ogre9Technique23setShadowCasterMaterialERKSs")]
pub fn stub_0xe40de4() -> ! {
    todo!("0xe40de4 Ogre::Technique::setShadowCasterMaterial(std::string const&)")
}

// 0xe40fc0 — __ZNK4Ogre9Technique25getShadowReceiverMaterialEv
#[doc(alias = "Ogre::Technique::getShadowReceiverMaterial(void)const")]
#[doc(alias = "__ZNK4Ogre9Technique25getShadowReceiverMaterialEv")]
pub fn stub_0xe40fc0() -> ! {
    todo!("0xe40fc0 Ogre::Technique::getShadowReceiverMaterial(void)const")
}

// 0xe40ff8 — __ZN4Ogre9Technique25setShadowReceiverMaterialERKSs
#[doc(alias = "Ogre::Technique::setShadowReceiverMaterial(std::string const&)")]
#[doc(alias = "__ZN4Ogre9Technique25setShadowReceiverMaterialERKSs")]
pub fn stub_0xe40ff8() -> ! {
    todo!("0xe40ff8 Ogre::Technique::setShadowReceiverMaterial(std::string const&)")
}

// 0xe411d4 — __ZN4Ogre9Technique16addGPUVendorRuleERKNS0_13GPUVendorRuleE
#[doc(alias = "Ogre::Technique::addGPUVendorRule(Ogre::Technique::GPUVendorRule const&)")]
#[doc(alias = "__ZN4Ogre9Technique16addGPUVendorRuleERKNS0_13GPUVendorRuleE")]
pub fn stub_0xe411d4() -> ! {
    todo!("0xe411d4 Ogre::Technique::addGPUVendorRule(Ogre::Technique::GPUVendorRule const&)")
}

// 0xe413cc — __ZNSt6vectorIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
#[doc(alias = "std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_")]
pub fn stub_0xe413cc() -> ! {
    todo!("0xe413cc std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

// 0xe41cc4 — __ZNSt6vectorIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUVendorRule*,std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUVendorRule const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
pub fn stub_0xe41cc4() -> ! {
    todo!("0xe41cc4 std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUVendorRule*,std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUVendorRule const&)")
}

// 0xe41de0 — __ZNSt6vectorIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::IlluminationPass **,std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::IlluminationPass * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
pub fn stub_0xe41de0() -> ! {
    todo!("0xe41de0 std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::IlluminationPass **,std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::IlluminationPass * const&)")
}

// 0xe42044 — __ZNSt6vectorIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Pass **,std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Pass * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
pub fn stub_0xe42044() -> ! {
    todo!("0xe42044 std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Pass **,std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Pass * const&)")
}

// 0xe42140 — __ZNSt12_Vector_baseIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
pub fn stub_0xe42140() {
    // IDA 0xe42140: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xe42144 — __ZNSt12_Vector_baseIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
pub fn stub_0xe42144() {
    // IDA 0xe42144: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xe42148 — __ZNSt12_Vector_baseIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
pub fn stub_0xe42148() {
    // IDA 0xe42148: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xe42158 — __ZNSt12_Vector_baseIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
pub fn stub_0xe42158() {
    // IDA 0xe42158: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xe42164 — __ZNSt12_Vector_baseIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
pub fn stub_0xe42164() {
    // IDA 0xe42164: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xe42170 — __ZNSt12_Vector_baseIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
pub fn stub_0xe42170() {
    // IDA 0xe42170: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xe421b0 — __ZN4Ogre22TextAreaOverlayElementC1ERKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::TextAreaOverlayElement(std::string const&)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElementC1ERKSs")]
pub fn stub_0xe421b0() -> ! {
    todo!("0xe421b0 Ogre::TextAreaOverlayElement::TextAreaOverlayElement(std::string const&)")
}

// 0xe421bc — __ZN4Ogre22TextAreaOverlayElementC2ERKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::TextAreaOverlayElement(std::string const&)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElementC2ERKSs")]
pub fn stub_0xe421bc() -> ! {
    todo!("0xe421bc Ogre::TextAreaOverlayElement::TextAreaOverlayElement(std::string const&)")
}

// 0xe424ac — __ZN4Ogre22TextAreaOverlayElement10initialiseEv
#[doc(alias = "Ogre::TextAreaOverlayElement::initialise(void)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElement10initialiseEv")]
pub fn stub_0xe424ac() -> ! {
    todo!("0xe424ac Ogre::TextAreaOverlayElement::initialise(void)")
}

// 0xe42604 — __ZN4Ogre22TextAreaOverlayElement21checkMemoryAllocationEm
#[doc(alias = "Ogre::TextAreaOverlayElement::checkMemoryAllocation(unsigned long)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElement21checkMemoryAllocationEm")]
pub fn stub_0xe42604() -> ! {
    todo!("0xe42604 Ogre::TextAreaOverlayElement::checkMemoryAllocation(unsigned long)")
}

// 0xe42910 — __ZN4Ogre22TextAreaOverlayElement22updatePositionGeometryEv
#[doc(alias = "Ogre::TextAreaOverlayElement::updatePositionGeometry(void)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElement22updatePositionGeometryEv")]
pub fn stub_0xe42910() -> ! {
    todo!("0xe42910 Ogre::TextAreaOverlayElement::updatePositionGeometry(void)")
}

// 0xe42f54 — __ZN4Ogre22TextAreaOverlayElement21updateTextureGeometryEv
#[doc(alias = "Ogre::TextAreaOverlayElement::updateTextureGeometry(void)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElement21updateTextureGeometryEv")]
pub fn stub_0xe42f54() -> ! {
    todo!("0xe42f54 Ogre::TextAreaOverlayElement::updateTextureGeometry(void)")
}

// 0xe42f58 — __ZN4Ogre22TextAreaOverlayElement10setCaptionERKNS_9UTFStringE
#[doc(alias = "Ogre::TextAreaOverlayElement::setCaption(Ogre::UTFString const&)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElement10setCaptionERKNS_9UTFStringE")]
pub fn stub_0xe42f58() -> ! {
    todo!("0xe42f58 Ogre::TextAreaOverlayElement::setCaption(Ogre::UTFString const&)")
}

// 0xe42f70 — __ZN4Ogre22TextAreaOverlayElement11setFontNameERKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::setFontName(std::string const&)")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElement11setFontNameERKSs")]
pub fn stub_0xe42f70() -> ! {
    todo!("0xe42f70 Ogre::TextAreaOverlayElement::setFontName(std::string const&)")
}

// 0xe4332c — __ZN4Ogre22TextAreaOverlayElementD0Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElementD0Ev")]
pub fn stub_0xe4332c() {
    // IDA 0xe4332c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xe433bc — __ZN4Ogre22TextAreaOverlayElementD1Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElementD1Ev")]
pub fn stub_0xe433bc() {
    // IDA 0xe433bc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xe433c8 — __ZThn12_N4Ogre22TextAreaOverlayElementD0Ev
#[doc(alias = "non-virtual thunk toOgre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
#[doc(alias = "__ZThn12_N4Ogre22TextAreaOverlayElementD0Ev")]
pub fn stub_0xe433c8() {
    // IDA 0xe433c8: __ZThn12 thunk (D0 deleting dtor): `this -= 12`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xe4345c — __ZN4Ogre22TextAreaOverlayElementD2Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
#[doc(alias = "__ZN4Ogre22TextAreaOverlayElementD2Ev")]
pub fn stub_0xe4345c() {
    // IDA 0xe4345c: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xe43594 — __ZThn12_N4Ogre22TextAreaOverlayElementD1Ev
#[doc(alias = "non-virtual thunk toOgre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
#[doc(alias = "__ZThn12_N4Ogre22TextAreaOverlayElementD1Ev")]
pub fn stub_0xe43594() {
    // IDA 0xe43594: __ZThn12 thunk (D1 base dtor): `this -= 12`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}
