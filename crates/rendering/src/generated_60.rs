//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xbd3c88..0xbf94a8 (100 stubs, 7168 prior -> +100)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xbd3c88 — __ZN4Ogre6RbxSky17updateRenderQueueEPNS_6CameraEPNS_11RenderQueueENS_18RenderQueueGroupIDE
#[doc(alias = "Ogre::RbxSky::updateRenderQueue(Ogre::Camera *,Ogre::RenderQueue *,Ogre::RenderQueueGroupID)")]
// was: Ogre::RbxSky::updateRenderQueue(Ogre::Camera *,Ogre::RenderQueue *,Ogre::RenderQueueGroupID)
// IDA 0xbd3c88: 629 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd3c88() {
}

// 0xbd4448 — __ZNSt6vectorIPN4Ogre12ManualObjectESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<Ogre::ManualObject *,std::allocator<Ogre::ManualObject *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ManualObject **,std::vector<Ogre::ManualObject *,std::allocator<Ogre::ManualObject *>>>,Ogre::ManualObject * const&)")]
// was: std::vector<Ogre::ManualObject *,std::allocator<Ogre::ManualObject *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ManualObject **,std::vector<Ogre::ManualObject *,std::allocator<Ogre::ManualObject *>>>,Ogre::ManualObject * const&)
// IDA 0xbd4448: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_bd4448() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xbd4540 — __ZNSt6vectorIN4Ogre7Vector3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
#[doc(alias = "std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,unsigned long,Ogre::Vector3 const&)")]
// was: std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,unsigned long,Ogre::Vector3 const&)
// IDA 0xbd4540: 224 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd4540() {
}

// 0xbd4e74 — __ZN4Ogre12RbxSubEntityC1EPNS_6EntityE
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntity(Ogre::Entity *)")]
// was: Ogre::RbxSubEntity::RbxSubEntity(Ogre::Entity *)
// IDA 0xbd4e74: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd4e74() {
}

// 0xbd4ecc — __ZN4Ogre12RbxSubEntity10setSubMeshEPNS_7SubMeshE
#[doc(alias = "Ogre::RbxSubEntity::setSubMesh(Ogre::SubMesh *)")]
// was: Ogre::RbxSubEntity::setSubMesh(Ogre::SubMesh *)
// IDA 0xbd4ecc: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd4ecc() {
}

// 0xbd4ed0 — __ZN4Ogre12RbxSubEntity18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::RbxSubEntity::getRenderOperation(Ogre::RenderOperation &)")]
// was: Ogre::RbxSubEntity::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xbd4ed0: 10 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd4ed0() {
}

// 0xbd4ef0 — __ZNK4Ogre12RbxSubEntity19getSquaredViewDepthEPKNS_6CameraE
#[doc(alias = "Ogre::RbxSubEntity::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: Ogre::RbxSubEntity::getSquaredViewDepth(Ogre::Camera const*)const
// IDA 0xbd4ef0: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd4ef0() {
}

// 0xbd4f68 — __ZN4Ogre12RbxSubEntity11setMaterialERKNS_11MaterialPtrE
#[doc(alias = "Ogre::RbxSubEntity::setMaterial(Ogre::MaterialPtr const&)")]
// was: Ogre::RbxSubEntity::setMaterial(Ogre::MaterialPtr const&)
// IDA 0xbd4f68: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd4f68() {
}

// 0xbd5050 — __ZN4Ogre12RbxSubEntity13getSubMeshLODEi
#[doc(alias = "Ogre::RbxSubEntity::getSubMeshLOD(int)")]
// was: Ogre::RbxSubEntity::getSubMeshLOD(int)
// IDA 0xbd5050: 7 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd5050() {
}

// 0xbd5064 — __ZNK4Ogre12RbxSubEntity19getWorldBoundingBoxEb
#[doc(alias = "Ogre::RbxSubEntity::getWorldBoundingBox(bool)const")]
// was: Ogre::RbxSubEntity::getWorldBoundingBox(bool)const
// IDA 0xbd5064: 86 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd5064() {
}

// 0xbd5188 — __ZThn264_NK4Ogre12RbxSubEntity19getWorldBoundingBoxEb
#[doc(alias = "non-virtual thunk toOgre::RbxSubEntity::getWorldBoundingBox(bool)const")]
// was: non-virtual thunk to Ogre::RbxSubEntity::getWorldBoundingBox(bool)const
// IDA 0xbd5188: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd5188() {
}

// 0xbd51a4 — __ZNK4Ogre12RbxSubEntity17getLightCapBoundsEv
#[doc(alias = "Ogre::RbxSubEntity::getLightCapBounds(void)const")]
// was: Ogre::RbxSubEntity::getLightCapBounds(void)const
// IDA 0xbd51a4: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd51a4() {
}

// 0xbd51b0 — __ZThn264_NK4Ogre12RbxSubEntity17getLightCapBoundsEv
#[doc(alias = "non-virtual thunk toOgre::RbxSubEntity::getLightCapBounds(void)const")]
// was: non-virtual thunk to Ogre::RbxSubEntity::getLightCapBounds(void)const
// IDA 0xbd51b0: 6 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd51b0() {
}

// 0xbd51c8 — __ZNK4Ogre12RbxSubEntity16getDarkCapBoundsERKNS_5LightEf
#[doc(alias = "Ogre::RbxSubEntity::getDarkCapBounds(Ogre::Light const&,float)const")]
// was: Ogre::RbxSubEntity::getDarkCapBounds(Ogre::Light const&,float)const
// IDA 0xbd51c8: 110 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd51c8() {
}

// 0xbd5300 — __ZThn264_NK4Ogre12RbxSubEntity16getDarkCapBoundsERKNS_5LightEf
#[doc(alias = "non-virtual thunk toOgre::RbxSubEntity::getDarkCapBounds(Ogre::Light const&,float)const")]
// was: non-virtual thunk to Ogre::RbxSubEntity::getDarkCapBounds(Ogre::Light const&,float)const
// IDA 0xbd5300: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd5300() {
}

// 0xbd5318 — __ZNK4Ogre12RbxSubEntity25getPointExtrusionDistanceEPKNS_5LightE
#[doc(alias = "Ogre::RbxSubEntity::getPointExtrusionDistance(Ogre::Light const*)const")]
// was: Ogre::RbxSubEntity::getPointExtrusionDistance(Ogre::Light const*)const
// IDA 0xbd5318: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd5318() {
}

// 0xbd534c — __ZThn264_NK4Ogre12RbxSubEntity25getPointExtrusionDistanceEPKNS_5LightE
#[doc(alias = "non-virtual thunk toOgre::RbxSubEntity::getPointExtrusionDistance(Ogre::Light const*)const")]
// was: non-virtual thunk to Ogre::RbxSubEntity::getPointExtrusionDistance(Ogre::Light const*)const
// IDA 0xbd534c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd534c() {
}

// 0xbd537c — __ZN4Ogre12RbxSubEntity11getEdgeListEv
#[doc(alias = "Ogre::RbxSubEntity::getEdgeList(void)")]
// was: Ogre::RbxSubEntity::getEdgeList(void)
// IDA 0xbd537c: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd537c() {
}

// 0xbd549c — __ZThn264_N4Ogre12RbxSubEntity11getEdgeListEv
#[doc(alias = "non-virtual thunk toOgre::RbxSubEntity::getEdgeList(void)")]
// was: non-virtual thunk to Ogre::RbxSubEntity::getEdgeList(void)
// IDA 0xbd549c: 3 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd549c() {
}

// 0xbd54a4 — __ZN4Ogre12RbxSubEntity11hasEdgeListEv
#[doc(alias = "Ogre::RbxSubEntity::hasEdgeList(void)")]
// was: Ogre::RbxSubEntity::hasEdgeList(void)
// IDA 0xbd54a4: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd54a4() {
}

// 0xbd55c4 — __ZThn264_N4Ogre12RbxSubEntity11hasEdgeListEv
#[doc(alias = "non-virtual thunk toOgre::RbxSubEntity::hasEdgeList(void)")]
// was: non-virtual thunk to Ogre::RbxSubEntity::hasEdgeList(void)
// IDA 0xbd55c4: 3 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd55c4() {
}

// 0xbd55cc — __ZN4Ogre12RbxSubEntity33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm
#[doc(alias = "Ogre::RbxSubEntity::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")]
// was: Ogre::RbxSubEntity::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)
// IDA 0xbd55cc: 388 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd55cc() {
}

// 0xbd59e8 — __ZThn264_N4Ogre12RbxSubEntity33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm
#[doc(alias = "non-virtual thunk toOgre::RbxSubEntity::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")]
// was: non-virtual thunk to Ogre::RbxSubEntity::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)
// IDA 0xbd59e8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd59e8() {
}

// 0xbd5a18 — __ZN4Ogre12RbxSubEntityD0Ev
#[doc(alias = "Ogre::RbxSubEntity::~RbxSubEntity()")]
// was: Ogre::RbxSubEntity::~RbxSubEntity()
// IDA 0xbd5a18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bd5a18() {
}

// 0xbd5acc — __ZN4Ogre12RbxSubEntityD1Ev
#[doc(alias = "Ogre::RbxSubEntity::~RbxSubEntity()")]
// was: Ogre::RbxSubEntity::~RbxSubEntity()
// IDA 0xbd5acc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bd5acc() {
}

// 0xbd5ad0 — __ZThn264_N4Ogre12RbxSubEntityD0Ev
#[doc(alias = "non-virtual thunk toOgre::RbxSubEntity::~RbxSubEntity()")]
// was: non-virtual thunk to Ogre::RbxSubEntity::~RbxSubEntity()
// IDA 0xbd5ad0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bd5ad0() {
}

// 0xbd5b88 — __ZN4Ogre12RbxSubEntityD2Ev
#[doc(alias = "Ogre::RbxSubEntity::~RbxSubEntity()")]
// was: Ogre::RbxSubEntity::~RbxSubEntity()
// IDA 0xbd5b88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bd5b88() {
}

// 0xbd5cbc — __ZThn264_N4Ogre12RbxSubEntityD1Ev
#[doc(alias = "non-virtual thunk toOgre::RbxSubEntity::~RbxSubEntity()")]
// was: non-virtual thunk to Ogre::RbxSubEntity::~RbxSubEntity()
// IDA 0xbd5cbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bd5cbc() {
}

// 0xbd5cc4 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableC2EPNS_6EntityEPNS_28HardwareIndexBufferSharedPtrEPKNS_10VertexDataEbPNS_9SubEntityEb
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::RbxSubEntityShadowRenderable(Ogre::Entity *,Ogre::HardwareIndexBufferSharedPtr *,Ogre::VertexData const*,bool,Ogre::SubEntity *,bool)")]
// was: Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::RbxSubEntityShadowRenderable(Ogre::Entity *,Ogre::HardwareIndexBufferSharedPtr *,Ogre::VertexData const*,bool,Ogre::SubEntity *,bool)
// IDA 0xbd5cc4: 569 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd5cc4() {
}

// 0xbd6240 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD0Ev
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()")]
// was: Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()
// IDA 0xbd6240: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bd6240() {
}

// 0xbd62f4 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD1Ev
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()")]
// was: Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()
// IDA 0xbd62f4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bd62f4() {
}

// 0xbd62f8 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD2Ev
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()")]
// was: Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()
// IDA 0xbd62f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bd62f8() {
}

// 0xbd661c — __ZNK4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xbd661c: 4 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd661c() {
}

// 0xbd6628 — __ZNK4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable9isVisibleEv
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::isVisible(void)const")]
// was: Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::isVisible(void)const
// IDA 0xbd6628: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd6628() {
}

// 0xbd6638 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable17rebindIndexBufferERKNS_28HardwareIndexBufferSharedPtrE
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)")]
// was: Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)
// IDA 0xbd6638: 95 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd6638() {
}

// 0xbd6738 — __ZNK4Ogre12RbxSubEntity18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::RbxSubEntity::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::RbxSubEntity::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xbd6738: 16 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd6738() {
}

// 0xbd6778 — __ZNK4Ogre12RbxSubEntity14getCastShadowsEv
#[doc(alias = "Ogre::RbxSubEntity::getCastShadows(void)const")]
// was: Ogre::RbxSubEntity::getCastShadows(void)const
// IDA 0xbd6778: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd6778() {
}

// 0xbd6780 — __ZN4Ogre12RbxSubEntity13setImportanceEf
#[doc(alias = "Ogre::RbxSubEntity::setImportance(float)")]
// was: Ogre::RbxSubEntity::setImportance(float)
// IDA 0xbd6780: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd6780() {
}

// 0xbd6788 — __ZN4Ogre12RbxSubEntity13getImportanceEv
#[doc(alias = "Ogre::RbxSubEntity::getImportance(void)")]
// was: Ogre::RbxSubEntity::getImportance(void)
// IDA 0xbd6788: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd6788() {
}

// 0xbd6790 — __ZThn264_NK4Ogre12RbxSubEntity14getCastShadowsEv
#[doc(alias = "non-virtual thunk toOgre::RbxSubEntity::getCastShadows(void)const")]
// was: non-virtual thunk to Ogre::RbxSubEntity::getCastShadows(void)const
// IDA 0xbd6790: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd6790() {
}

// 0xbd6e1c — __ZN4Ogre10RbxSubMesh22prepareForShadowVolumeEv
#[doc(alias = "Ogre::RbxSubMesh::prepareForShadowVolume(void)")]
// was: Ogre::RbxSubMesh::prepareForShadowVolume(void)
// IDA 0xbd6e1c: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd6e1c() {
}

// 0xbd6f18 — __ZN4Ogre10RbxSubMesh17setSubMeshBSphereERNS_7SubMeshERKNS_6SphereE
#[doc(alias = "Ogre::RbxSubMesh::setSubMeshBSphere(Ogre::SubMesh &,Ogre::Sphere const&)")]
// was: Ogre::RbxSubMesh::setSubMeshBSphere(Ogre::SubMesh &,Ogre::Sphere const&)
// IDA 0xbd6f18: 69 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd6f18() {
}

// 0xbd6fcc — __ZN4Ogre10RbxSubMesh17getSubMeshBSphereEPKNS_7SubMeshE
#[doc(alias = "Ogre::RbxSubMesh::getSubMeshBSphere(Ogre::SubMesh const*)")]
// was: Ogre::RbxSubMesh::getSubMeshBSphere(Ogre::SubMesh const*)
// IDA 0xbd6fcc: 129 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd6fcc() {
}

// 0xbd7144 — __ZN5boost10scoped_ptrISt3mapIPKN4Ogre7SubMeshENS2_6SphereESt4lessIS5_ESaISt4pairIKS5_S6_EEEED1Ev
#[doc(alias = "boost::scoped_ptr<std::map<Ogre::SubMesh const*,Ogre::Sphere,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>>::~scoped_ptr()")]
// was: boost::scoped_ptr<std::map<Ogre::SubMesh const*,Ogre::Sphere,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>>::~scoped_ptr()
// IDA 0xbd7144: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bd7144() {
}

// 0xbd7160 — __ZNSt8_Rb_treeIPKN4Ogre7SubMeshESt4pairIKS3_NS0_6SphereEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_insert_unique(std::pair<Ogre::SubMesh const* const,Ogre::Sphere> const&)")]
// was: std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_insert_unique(std::pair<Ogre::SubMesh const* const,Ogre::Sphere> const&)
// IDA 0xbd7160: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd7160() {
}

// 0xbd7264 — __ZNSt8_Rb_treeIPKN4Ogre7SubMeshESt4pairIKS3_NS0_6SphereEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>> *)")]
// was: std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>> *)
// IDA 0xbd7264: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd7264() {
}

// 0xbd791c — __ZN3RBX20TextureCompositorJobC2EPN4Ogre12VisualEngineERKN3G3D7Vector2ERKSt6vectorINS_22TextureCompositorLayerESaIS9_EEf
#[doc(alias = "RBX::TextureCompositorJob::TextureCompositorJob(Ogre::VisualEngine *,G3D::Vector2 const&,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&,float)")]
// was: RBX::TextureCompositorJob::TextureCompositorJob(Ogre::VisualEngine *,G3D::Vector2 const&,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&,float)
// IDA 0xbd791c: 140 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd791c() {
}

// 0xbd94dc — __ZN3RBX20TextureCompositorJob6renderERKN4Ogre10TexturePtrE
#[doc(alias = "RBX::TextureCompositorJob::render(Ogre::TexturePtr const&)")]
// was: RBX::TextureCompositorJob::render(Ogre::TexturePtr const&)
// IDA 0xbd94dc: 387 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd94dc() {
}

// 0xbd98fc — __ZN3RBX17TextureCompositorC1EPN4Ogre12VisualEngineE
#[doc(alias = "RBX::TextureCompositor::TextureCompositor(Ogre::VisualEngine *)")]
// was: RBX::TextureCompositor::TextureCompositor(Ogre::VisualEngine *)
// IDA 0xbd98fc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bd98fc() {
}

// 0xbd9900 — __ZN3RBX17TextureCompositorC2EPN4Ogre12VisualEngineE
#[doc(alias = "RBX::TextureCompositor::TextureCompositor(Ogre::VisualEngine *)")]
// was: RBX::TextureCompositor::TextureCompositor(Ogre::VisualEngine *)
// IDA 0xbd9900: 736 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd9900() {
}

// 0xbda788 — __ZN3RBX17TextureCompositor6getJobERKSsRKN3G3D7Vector2ERKSt6vectorINS_22TextureCompositorLayerESaIS8_EE
#[doc(alias = "RBX::TextureCompositor::getJob(std::string const&,G3D::Vector2 const&,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&)")]
// was: RBX::TextureCompositor::getJob(std::string const&,G3D::Vector2 const&,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&)
// IDA 0xbda788: 483 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bda788() {
}

// 0xbdae14 — __ZN3RBX17TextureCompositor14attachMaterialERKN5boost10shared_ptrINS0_3JobEEERKN4Ogre11MaterialPtrE
#[doc(alias = "RBX::TextureCompositor::attachMaterial(rbx_core::SharedPtr<RBX::TextureCompositor::Job> const&,Ogre::MaterialPtr const&)")]
// was: RBX::TextureCompositor::attachMaterial(boost::shared_ptr<RBX::TextureCompositor::Job> const&,Ogre::MaterialPtr const&)
// IDA 0xbdae14: 144 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdae14() {
}

// 0xbdb344 — __ZN3RBX17TextureCompositor29updatePrioritiesAndOrphanJobsERKN3G3D7Vector3E
#[doc(alias = "RBX::TextureCompositor::updatePrioritiesAndOrphanJobs(G3D::Vector3 const&)")]
// was: RBX::TextureCompositor::updatePrioritiesAndOrphanJobs(G3D::Vector3 const&)
// IDA 0xbdb344: 583 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdb344() {
}

// 0xbdc510 — __ZN3RBX17TextureCompositor6updateERKN3G3D7Vector3E
#[doc(alias = "RBX::TextureCompositor::update(G3D::Vector3 const&)")]
// was: RBX::TextureCompositor::update(G3D::Vector3 const&)
// IDA 0xbdc510: 330 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdc510() {
}

// 0xbdd9d4 — __ZN3RBX17TextureCompositor15getRenderTargetERKN4Ogre10TexturePtrE
#[doc(alias = "RBX::TextureCompositor::getRenderTarget(Ogre::TexturePtr const&)")]
// was: RBX::TextureCompositor::getRenderTarget(Ogre::TexturePtr const&)
// IDA 0xbdd9d4: 876 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdd9d4() {
}

// 0xbde2d4 — __ZN12_GLOBAL__N_123replaceMaterialTexturesERKyN4Ogre10TexturePtrE
#[doc(alias = "anonymous namespace::replaceMaterialTextures(unsigned long long const&,Ogre::TexturePtr)")]
// was: anonymous namespace::replaceMaterialTextures(unsigned long long const&,Ogre::TexturePtr)
// IDA 0xbde2d4: 216 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bde2d4() {
}

// 0xbdee88 — __ZN3RBX17TextureCompositor13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
#[doc(alias = "RBX::TextureCompositor::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: RBX::TextureCompositor::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xbdee88: 446 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdee88() {
}

// 0xbe0ba8 — __ZNSt6vectorIN4Ogre10TexturePtrESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>>,Ogre::TexturePtr const&)")]
// was: std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>>,Ogre::TexturePtr const&)
// IDA 0xbe0ba8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_be0ba8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xbe0ffc — __ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN4Ogre10TexturePtrES4_EET0_T_S6_S5_
#[doc(alias = "Ogre::TexturePtr * std::__copy_backward_normal<false,false>::__copy_b_n<Ogre::TexturePtr *,Ogre::TexturePtr *>(Ogre::TexturePtr *,Ogre::TexturePtr *,Ogre::TexturePtr *)")]
// was: Ogre::TexturePtr * std::__copy_backward_normal<false,false>::__copy_b_n<Ogre::TexturePtr *,Ogre::TexturePtr *>(Ogre::TexturePtr *,Ogre::TexturePtr *,Ogre::TexturePtr *)
// IDA 0xbe0ffc: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_be0ffc() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xbe552c — __ZN3RBX15RbxTextureProxyC1ERKN4Ogre10TexturePtrEii
#[doc(alias = "RBX::RbxTextureProxy::RbxTextureProxy(Ogre::TexturePtr const&,int,int)")]
// was: RBX::RbxTextureProxy::RbxTextureProxy(Ogre::TexturePtr const&,int,int)
// IDA 0xbe552c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_be552c() {
}

// 0xbe5530 — __ZN3RBX15RbxTextureProxyC2ERKN4Ogre10TexturePtrEii
#[doc(alias = "RBX::RbxTextureProxy::RbxTextureProxy(Ogre::TexturePtr const&,int,int)")]
// was: RBX::RbxTextureProxy::RbxTextureProxy(Ogre::TexturePtr const&,int,int)
// IDA 0xbe5530: 259 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be5530() {
}

// 0xbe618c — __ZN4Ogre17SaveTextureToFileEPNS_7TextureERKSs
#[doc(alias = "Ogre::SaveTextureToFile(Ogre::Texture *,std::string const&)")]
// was: Ogre::SaveTextureToFile(Ogre::Texture *,std::string const&)
// IDA 0xbe618c: 243 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be618c() {
}

// 0xbe63e8 — __ZN4Ogre17Frustum_IntersectEPKNS_7FrustumERKNS_14AxisAlignedBoxEPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum_Intersect(Ogre::Frustum const*,Ogre::AxisAlignedBox const&,Ogre::FrustumPlane *)")]
// was: Ogre::Frustum_Intersect(Ogre::Frustum const*,Ogre::AxisAlignedBox const&,Ogre::FrustumPlane *)
// IDA 0xbe63e8: 120 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be63e8() {
}

// 0xbe6554 — __ZN4Ogre16ToAxisAlignedBoxERKN3RBX7ExtentsE
#[doc(alias = "Ogre::ToAxisAlignedBox(RBX::Extents const&)")]
// was: Ogre::ToAxisAlignedBox(RBX::Extents const&)
// IDA 0xbe6554: 28 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be6554() {
}

// 0xbe65b4 — __ZN4Ogre9ToExtentsERKNS_14AxisAlignedBoxE
#[doc(alias = "Ogre::ToExtents(Ogre::AxisAlignedBox const&)")]
// was: Ogre::ToExtents(Ogre::AxisAlignedBox const&)
// IDA 0xbe65b4: 9 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be65b4() {
}

// 0xbe65d0 — __ZN4Ogre9ToVector3ERKNS_7Vector3E
#[doc(alias = "Ogre::ToVector3(Ogre::Vector3 const&)")]
// was: Ogre::ToVector3(Ogre::Vector3 const&)
// IDA 0xbe65d0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_be65d0() {
}

// 0xbe65d4 — __ZN4Ogre9ToVector3ERKN3G3D7Vector3E
#[doc(alias = "Ogre::ToVector3(G3D::Vector3 const&)")]
// was: Ogre::ToVector3(G3D::Vector3 const&)
// IDA 0xbe65d4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_be65d4() {
}

// 0xbe65d8 — __ZN4Ogre19CircumscribedSphereERKN3RBX7ExtentsE
#[doc(alias = "Ogre::CircumscribedSphere(RBX::Extents const&)")]
// was: Ogre::CircumscribedSphere(RBX::Extents const&)
// IDA 0xbe65d8: 28 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be65d8() {
}

// 0xbe6648 — __ZN4Ogre12ToColorValueERKN3G3D6Color4E
#[doc(alias = "Ogre::ToColorValue(G3D::Color4 const&)")]
// was: Ogre::ToColorValue(G3D::Color4 const&)
// IDA 0xbe6648: 15 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be6648() {
}

// 0xbe6684 — __ZN4Ogre12ToColorValueERKN3G3D6Color3E
#[doc(alias = "Ogre::ToColorValue(G3D::Color3 const&)")]
// was: Ogre::ToColorValue(G3D::Color3 const&)
// IDA 0xbe6684: 14 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be6684() {
}

// 0xbe66b8 — __ZN4Ogre8MaxCoordERKNS_7Vector3E
#[doc(alias = "Ogre::MaxCoord(Ogre::Vector3 const&)")]
// was: Ogre::MaxCoord(Ogre::Vector3 const&)
// IDA 0xbe66b8: 13 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be66b8() {
}

// 0xbe8f80 — __ZN3RBX10ViewRbxGfx14presetLightingEPNS_8LightingERKN3G3D6Color3Ef
#[doc(alias = "RBX::ViewRbxGfx::presetLighting(RBX::Lighting *,G3D::Color3 const&,float)")]
// was: RBX::ViewRbxGfx::presetLighting(RBX::Lighting *,G3D::Color3 const&,float)
// IDA 0xbe8f80: 513 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be8f80() {
}

// 0xbec248 — __ZN3RBX14visitPrintNodeEPN4Ogre9SceneNodeERSs
#[doc(alias = "RBX::visitPrintNode(Ogre::SceneNode *,std::string &)")]
// was: RBX::visitPrintNode(Ogre::SceneNode *,std::string &)
// IDA 0xbec248: 400 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bec248() {
}

// 0xbef0f8 — __ZN3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
#[doc(alias = "RBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: RBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xbef0f8: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef0f8() {
}

// 0xbef138 — __ZThn8_N3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
#[doc(alias = "non-virtual thunk to RBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: non-virtual thunk to RBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xbef138: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef138() {
}

// 0xbef328 — __ZN4Ogre9SharedPtrINS_8ResourceEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Resource>::~SharedPtr()
// IDA 0xbef328: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bef328() {
}

// 0xbf29a8 — __ZN4Ogre9SharedPtrINS_8ResourceEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Resource>::~SharedPtr()
// IDA 0xbf29a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf29a8() {
}

// 0xbf2a48 — __ZN4Ogre9SharedPtrINS_8ResourceEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::Resource>::destroy(void)
// IDA 0xbf2a48: 25 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf2a48() {
}

// 0xbf2a80 — __ZN4Ogre9SharedPtrINS_8ResourceEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::swap(Ogre::SharedPtr<Ogre::Resource>&)")]
// was: Ogre::SharedPtr<Ogre::Resource>::swap(Ogre::SharedPtr<Ogre::Resource>&)
// IDA 0xbf2a80: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf2a80() {
}

// 0xbf2c80 — __ZN4Ogre9SharedPtrINS_8MaterialEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Material>::~SharedPtr()
// IDA 0xbf2c80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf2c80() {
}

// 0xbf2cb0 — __ZN4Ogre11MaterialPtrD0Ev
#[doc(alias = "Ogre::MaterialPtr::~MaterialPtr()")]
// was: Ogre::MaterialPtr::~MaterialPtr()
// IDA 0xbf2cb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf2cb0() {
}

// 0xbf5b30 — __ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::~sp_counted_impl_p()
// IDA 0xbf5b30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_bf5b30() {
}

// 0xbf5b34 — __ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::~sp_counted_impl_p()
// IDA 0xbf5b34: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bf5b34() {
}

// 0xbf5b38 — __ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::dispose(void)
// IDA 0xbf5b38: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf5b38() {
}

// 0xbf5b48 — __ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::get_deleter(std::type_info const&)
// IDA 0xbf5b48: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf5b48() {
}

// 0xbf61f4 — __ZN4Ogre13RbxTypesetterC1ENS_7FontPtrES1_fff
#[doc(alias = "Ogre::RbxTypesetter::RbxTypesetter(Ogre::FontPtr,Ogre::FontPtr,float,float,float)")]
// was: Ogre::RbxTypesetter::RbxTypesetter(Ogre::FontPtr,Ogre::FontPtr,float,float,float)
// IDA 0xbf61f4: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf61f4() {
}

// 0xbf62a8 — __ZNK4Ogre13RbxTypesetter12computeArrayERKSsfffN3RBX4Text6XAlignEPN3G3D7Vector2EmiN9__gnu_cxx17__normal_iteratorIPKSt4pairIiNS0_7SpacingEESt6vectorISD_SaISD_EEEESJ_RKNS_7FontPtrEb
#[doc(alias = "Ogre::RbxTypesetter::computeArray(std::string const&,float,float,float,RBX::Text::XAlign,G3D::Vector2 *,unsigned long,int,__gnu_cxx::__normal_iterator<std::pair<int,Ogre::RbxTypesetter::Spacing> const*,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>>,__gnu_cxx::__normal_iterator<std::pair<int,Ogre::RbxTypesetter::Spacing> const*,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>>,Ogre::FontPtr const&,bool)const")]
// was: Ogre::RbxTypesetter::computeArray(std::string const&,float,float,float,RBX::Text::XAlign,G3D::Vector2 *,unsigned long,int,__gnu_cxx::__normal_iterator<std::pair<int,Ogre::RbxTypesetter::Spacing> const*,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>>,__gnu_cxx::__normal_iterator<std::pair<int,Ogre::RbxTypesetter::Spacing> const*,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>>,Ogre::FontPtr const&,bool)const
// IDA 0xbf62a8: 365 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf62a8() {
}

// 0xbf66dc — __ZNK4Ogre13RbxTypesetter13getTexturePtrEf
#[doc(alias = "Ogre::RbxTypesetter::getTexturePtr(float)const")]
// was: Ogre::RbxTypesetter::getTexturePtr(float)const
// IDA 0xbf66dc: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf66dc() {
}

// 0xbf6790 — __ZNK4Ogre13RbxTypesetter4drawEPN3RBX5AdornERKSsRKN3G3D7Vector2EfRKNS6_6Color4ESC_NS1_4Text6XAlignENSD_6YAlignES9_RKNS6_6Rect2DE
#[doc(alias = "Ogre::RbxTypesetter::draw(RBX::Adorn *,std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)const")]
// was: Ogre::RbxTypesetter::draw(RBX::Adorn *,std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)const
// IDA 0xbf6790: 844 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf6790() {
}

// 0xbf7268 — __ZNK4Ogre13RbxTypesetter15measureInternalERKSsfRKN3G3D7Vector2EPSt6vectorISt4pairIiNS0_7SpacingEESaISA_EEPb
#[doc(alias = "Ogre::RbxTypesetter::measureInternal(std::string const&,float,G3D::Vector2 const&,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>> *,bool *)const")]
// was: Ogre::RbxTypesetter::measureInternal(std::string const&,float,G3D::Vector2 const&,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>> *,bool *)const
// IDA 0xbf7268: 460 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf7268() {
}

// 0xbf7794 — __ZNK4Ogre13RbxTypesetter23getCursorPositionInTextERKSsRKN3G3D7Vector2EfN3RBX4Text6XAlignENS8_6YAlignES6_S4_
#[doc(alias = "Ogre::RbxTypesetter::getCursorPositionInText(std::string const&,G3D::Vector2 const&,float,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Vector2)const")]
// was: Ogre::RbxTypesetter::getCursorPositionInText(std::string const&,G3D::Vector2 const&,float,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Vector2)const
// IDA 0xbf7794: 494 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf7794() {
}

// 0xbf7dcc — __ZNK4Ogre13RbxTypesetter7measureERKSsfRKN3G3D7Vector2EPb
#[doc(alias = "Ogre::RbxTypesetter::measure(std::string const&,float,G3D::Vector2 const&,bool *)const")]
// was: Ogre::RbxTypesetter::measure(std::string const&,float,G3D::Vector2 const&,bool *)const
// IDA 0xbf7dcc: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf7dcc() {
}

// 0xbf8504 — __ZNK3RBX16TypesetterBitmap4drawEPNS_5AdornERKSsRKN3G3D7Vector2EfRKNS5_6Color4ESB_NS_4Text6XAlignENSC_6YAlignES8_RKNS5_6Rect2DE
#[doc(alias = "RBX::TypesetterBitmap::draw(RBX::Adorn *,std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)const")]
// was: RBX::TypesetterBitmap::draw(RBX::Adorn *,std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)const
// IDA 0xbf8504: 445 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf8504() {
}

// 0xbf8a1c — __ZNK3RBX16TypesetterBitmap6layoutERKSsPSt6vectorINS0_9GlyphLineESaIS4_EEiRKN3G3D12Vector2int16EbPb
#[doc(alias = "RBX::TypesetterBitmap::layout(std::string const&,std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>> *,int,G3D::Vector2int16 const&,bool,bool *)const")]
// was: RBX::TypesetterBitmap::layout(std::string const&,std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>> *,int,G3D::Vector2int16 const&,bool,bool *)const
// IDA 0xbf8a1c: 312 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf8a1c() {
}

// 0xbf8d24 — __ZN3RBXL8drawRectEPNS_5AdornEbRKN3G3D6Rect2DES5_RKNS2_7Vector2ES8_RKNS2_6Color4E
#[doc(alias = "RBX::drawRect(RBX::Adorn *,bool,G3D::Rect2D const&,G3D::Rect2D const&,G3D::Vector2 const&,G3D::Vector2 const&,G3D::Color4 const&)")]
// was: RBX::drawRect(RBX::Adorn *,bool,G3D::Rect2D const&,G3D::Rect2D const&,G3D::Vector2 const&,G3D::Vector2 const&,G3D::Color4 const&)
// IDA 0xbf8d24: 126 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf8d24() {
}

// 0xbf8ecc — __ZNK3RBX16TypesetterBitmap23getCursorPositionInTextERKSsRKN3G3D7Vector2EfNS_4Text6XAlignENS7_6YAlignES6_S4_
#[doc(alias = "RBX::TypesetterBitmap::getCursorPositionInText(std::string const&,G3D::Vector2 const&,float,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Vector2)const")]
// was: RBX::TypesetterBitmap::getCursorPositionInText(std::string const&,G3D::Vector2 const&,float,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Vector2)const
// IDA 0xbf8ecc: 244 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf8ecc() {
}

// 0xbf9194 — __ZNK3RBX16TypesetterBitmap7measureERKSsfRKN3G3D7Vector2EPb
#[doc(alias = "RBX::TypesetterBitmap::measure(std::string const&,float,G3D::Vector2 const&,bool *)const")]
// was: RBX::TypesetterBitmap::measure(std::string const&,float,G3D::Vector2 const&,bool *)const
// IDA 0xbf9194: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf9194() {
}

// 0xbf9278 — __ZN4Ogre13RbxTypesetter12getCharWidthERKNS_7FontPtrEfcf
#[doc(alias = "Ogre::RbxTypesetter::getCharWidth(Ogre::FontPtr const&,float,char,float)")]
// was: Ogre::RbxTypesetter::getCharWidth(Ogre::FontPtr const&,float,char,float)
// IDA 0xbf9278: 121 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf9278() {
}

// 0xbf93d0 — __ZN4Ogre13RbxTypesetterD1Ev
#[doc(alias = "Ogre::RbxTypesetter::~RbxTypesetter()")]
// was: Ogre::RbxTypesetter::~RbxTypesetter()
// IDA 0xbf93d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf93d0() {
}

// 0xbf94a8 — __ZN4Ogre13RbxTypesetterD0Ev
#[doc(alias = "Ogre::RbxTypesetter::~RbxTypesetter()")]
// was: Ogre::RbxTypesetter::~RbxTypesetter()
// IDA 0xbf94a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf94a8() {
}