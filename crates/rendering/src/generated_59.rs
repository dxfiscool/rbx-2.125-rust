//! rendering generated_59 — Ogre::|G3D:: strict 13333 total, 7069 prior, 100 this batch — 0xbc3c88..0xbd3a90
//! EA-sorted ascending uncovered gap 0xbc3c88..0xf6ad44 (rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0xbc3c88 — __ZNSt6vectorISt4pairIPN4Ogre14ParticleSystemEfESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
#[doc(alias = "std::vector<std::pair<Ogre::ParticleSystem *,float>,std::allocator<std::pair<Ogre::ParticleSystem *,float>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ParticleSystem *,float>*,std::vector<std::pair<Ogre::ParticleSystem *,float>,std::allocator<std::pair<Ogre::ParticleSystem *,float>>>>,std::pair<Ogre::ParticleSystem *,float> const&)")]
// was: std::vector<std::pair<Ogre::ParticleSystem *,float>,std::allocator<std::pair<Ogre::ParticleSystem *,float>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ParticleSystem *,float>*,std::vector<std::pair<Ogre::ParticleSystem *,float>,std::allocator<std::pair<Ogre::ParticleSystem *,float>>>>,std::pair<Ogre::ParticleSystem *,float> const&)
// IDA 0xbc3c88: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_bc3c88() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xbc44a8 — __ZN3RBX18RbxParticleManagerC1EPN4Ogre12VisualEngineE
// type: _DWORD __fastcall(RBX::RbxParticleManager *__hidden this, Ogre::VisualEngine *)
#[doc(alias = "RBX::RbxParticleManager::RbxParticleManager(Ogre::VisualEngine *)")]
// was: RBX::RbxParticleManager::RbxParticleManager(Ogre::VisualEngine *)
// IDA 0xbc44a8: 7 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc44a8() {
}

// 0xbc45b0 — __ZN3RBX18RbxParticleManager19FirstThrottleSystemEPN4Ogre14ParticleSystemE
// type: _DWORD __fastcall(RBX::RbxParticleManager *__hidden this, Ogre::ParticleSystem *)
#[doc(alias = "RBX::RbxParticleManager::FirstThrottleSystem(Ogre::ParticleSystem *)")]
// was: RBX::RbxParticleManager::FirstThrottleSystem(Ogre::ParticleSystem *)
// IDA 0xbc45b0: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc45b0() {
}

// 0xbc4cb4 — __ZN4Ogre15RbxSceneManagerC2ERKSs
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::RbxSceneManager::RbxSceneManager(std::string const&)")]
// was: Ogre::RbxSceneManager::RbxSceneManager(std::string const&)
// IDA 0xbc4cb4: 1073 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc4cb4() {
}

// 0xbc57b0 — __ZN4Ogre15RbxSceneManagerD0Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::~RbxSceneManager()")]
// was: Ogre::RbxSceneManager::~RbxSceneManager()
// IDA 0xbc57b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bc57b0() {
}

// 0xbc5864 — __ZN4Ogre15RbxSceneManagerD1Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::~RbxSceneManager()")]
// was: Ogre::RbxSceneManager::~RbxSceneManager()
// IDA 0xbc5864: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bc5864() {
}

// 0xbc5868 — __ZThn17800_N4Ogre15RbxSceneManagerD0Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RbxSceneManager::~RbxSceneManager()")]
// was: non-virtual thunk to Ogre::RbxSceneManager::~RbxSceneManager()
// IDA 0xbc5868: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bc5868() {
}

// 0xbc5924 — __ZN4Ogre15RbxSceneManagerD2Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::~RbxSceneManager()")]
// was: Ogre::RbxSceneManager::~RbxSceneManager()
// IDA 0xbc5924: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bc5924() {
}

// 0xbc5fbc — __ZThn17800_N4Ogre15RbxSceneManagerD1Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RbxSceneManager::~RbxSceneManager()")]
// was: non-virtual thunk to Ogre::RbxSceneManager::~RbxSceneManager()
// IDA 0xbc5fbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bc5fbc() {
}

// 0xbc5fc8 — __ZN4Ogre15RbxSceneManager15initSpatialHashEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::initSpatialHash(void)")]
// was: Ogre::RbxSceneManager::initSpatialHash(void)
// IDA 0xbc5fc8: 113 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc5fc8() {
}

// 0xbc6110 — __ZN4Ogre15RbxSceneManager10clearSceneEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::clearScene(void)")]
// was: Ogre::RbxSceneManager::clearScene(void)
// IDA 0xbc6110: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6110() {
}

// 0xbc61cc — __ZN4Ogre15RbxSceneManager17clearMegaClustersEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::clearMegaClusters(void)")]
// was: Ogre::RbxSceneManager::clearMegaClusters(void)
// IDA 0xbc61cc: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc61cc() {
}

// 0xbc6300 — __ZN4Ogre15RbxSceneManager17getSceneNodeCountEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::getSceneNodeCount(void)")]
// was: Ogre::RbxSceneManager::getSceneNodeCount(void)
// IDA 0xbc6300: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6300() {
}

// 0xbc6308 — __ZN4Ogre15RbxSceneManager24createRbxParticleEmitterEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::createRbxParticleEmitter(void)")]
// was: Ogre::RbxSceneManager::createRbxParticleEmitter(void)
// IDA 0xbc6308: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6308() {
}

// 0xbc63e4 — __ZN4Ogre15RbxSceneManager20createRbxMegaClusterERKN5boost10shared_ptrIN3RBX12PartInstanceEEE
// type: int __fastcall(int, char, int, int, int, void *, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneManager::createRbxMegaCluster(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// was: Ogre::RbxSceneManager::createRbxMegaCluster(boost::shared_ptr<RBX::PartInstance> const&)
// IDA 0xbc63e4: 291 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc63e4() {
}

// 0xbc6718 — __ZN4Ogre15RbxSceneManager16numSharedIBQuadsEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::numSharedIBQuads(void)")]
// was: Ogre::RbxSceneManager::numSharedIBQuads(void)
// IDA 0xbc6718: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6718() {
}

// 0xbc67d8 — __ZN4Ogre15RbxSceneManager23getOrCreateSharedQuadIBEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::getOrCreateSharedQuadIB(void)")]
// was: Ogre::RbxSceneManager::getOrCreateSharedQuadIB(void)
// IDA 0xbc67d8: 469 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc67d8() {
}

// 0xbc6c68 — __ZN4Ogre15RbxSceneManager8_setPassEPKNS_4PassEbb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const Ogre::Pass *, bool, bool)
#[doc(alias = "Ogre::RbxSceneManager::_setPass(Ogre::Pass const*,bool,bool)")]
// was: Ogre::RbxSceneManager::_setPass(Ogre::Pass const*,bool,bool)
// IDA 0xbc6c68: 418 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6c68() {
}

// 0xbc70b4 — __ZN4Ogre15RbxSceneManager28renderShadowVolumesToStencilEPKNS_5LightEPKNS_6CameraEb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const Ogre::Light *, const Ogre::Camera *, bool)
#[doc(alias = "Ogre::RbxSceneManager::renderShadowVolumesToStencil(Ogre::Light const*,Ogre::Camera const*,bool)")]
// was: Ogre::RbxSceneManager::renderShadowVolumesToStencil(Ogre::Light const*,Ogre::Camera const*,bool)
// IDA 0xbc70b4: 318 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc70b4() {
}

// 0xbc7404 — __ZN4Ogre15RbxSceneManager28findShadowCastersForLightRecERSt6vectorIPN3RBX27FastClusterShadowRenderableESaIS4_EEPNS_4NodeEPKNS_6CameraE
#[doc(alias = "Ogre::RbxSceneManager::findShadowCastersForLightRec(std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>> &,Ogre::Node *,Ogre::Camera const*)")]
// was: Ogre::RbxSceneManager::findShadowCastersForLightRec(std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>> &,Ogre::Node *,Ogre::Camera const*)
// IDA 0xbc7404: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc7404() {
}

// 0xbc74d8 — __ZN4Ogre15RbxSceneManager32renderFastShadowVolumesToStencilERKSt6vectorIPN3RBX27FastClusterShadowRenderableESaIS4_EEPKNS_4PassEPKNS_5LightEPKNS_6CameraE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, char, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneManager::renderFastShadowVolumesToStencil(std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>> const&,Ogre::Pass const*,Ogre::Light const*,Ogre::Camera const*)")]
// was: Ogre::RbxSceneManager::renderFastShadowVolumesToStencil(std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>> const&,Ogre::Pass const*,Ogre::Light const*,Ogre::Camera const*)
// IDA 0xbc74d8: 895 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc74d8() {
}

// 0xbc7d74 — __ZN4Ogre15RbxSceneManager16getDebugMaterialEPKvb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const void *, bool)
#[doc(alias = "Ogre::RbxSceneManager::getDebugMaterial(void const*,bool)")]
// was: Ogre::RbxSceneManager::getDebugMaterial(void const*,bool)
// IDA 0xbc7d74: 648 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc7d74() {
}

// 0xbc8448 — __ZN4Ogre15RbxSceneManager12_renderSceneEPNS_6CameraEPNS_8ViewportEb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, struct _Unwind_Exception *lpuexcpt, Ogre::Viewport *, bool)
#[doc(alias = "Ogre::RbxSceneManager::_renderScene(Ogre::Camera *,Ogre::Viewport *,bool)")]
// was: Ogre::RbxSceneManager::_renderScene(Ogre::Camera *,Ogre::Viewport *,bool)
// IDA 0xbc8448: 785 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc8448() {
}

// 0xbc8c98 — __ZN4Ogre15RbxSceneManager11renderBeginEPNS_8ViewportEPNS_6CameraE
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, Ogre::Viewport *, Ogre::Camera *)
#[doc(alias = "Ogre::RbxSceneManager::renderBegin(Ogre::Viewport *,Ogre::Camera *)")]
// was: Ogre::RbxSceneManager::renderBegin(Ogre::Viewport *,Ogre::Camera *)
// IDA 0xbc8c98: 205 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc8c98() {
}

// 0xbc8ebc — __ZN4Ogre15RbxSceneManager22renderQueueGroupSolidsEhNS_26QueuedRenderableCollection16OrganisationModeEb
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, char, char, char, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneManager::renderQueueGroupSolids(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode,bool)")]
// was: Ogre::RbxSceneManager::renderQueueGroupSolids(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode,bool)
// IDA 0xbc8ebc: 452 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc8ebc() {
}

// 0xbc9358 — __ZN4Ogre15RbxSceneManager9renderEndEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::renderEnd(void)")]
// was: Ogre::RbxSceneManager::renderEnd(void)
// IDA 0xbc9358: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9358() {
}

// 0xbc9490 — __ZN4Ogre15RbxSceneManager28renderQueueGroupTransparentsEhNS_26QueuedRenderableCollection16OrganisationModeE
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneManager::renderQueueGroupTransparents(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode)")]
// was: Ogre::RbxSceneManager::renderQueueGroupTransparents(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode)
// IDA 0xbc9490: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9490() {
}

// 0xbc9640 — __ZN4Ogre15RbxSceneManager10_setSkyBoxEbRKSsfhRKNS_10QuaternionES2_
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, bool, const std::string *, float, unsigned __int8, const Ogre::Quaternion *, const std::string *)
#[doc(alias = "Ogre::RbxSceneManager::_setSkyBox(bool,std::string const&,float,unsigned char,Ogre::Quaternion const&,std::string const&)")]
// was: Ogre::RbxSceneManager::_setSkyBox(bool,std::string const&,float,unsigned char,Ogre::Quaternion const&,std::string const&)
// IDA 0xbc9640: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9640() {
}

// 0xbc97f4 — __ZN4Ogre15RbxSceneManager15recordPassStatsEj
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, unsigned int)
#[doc(alias = "Ogre::RbxSceneManager::recordPassStats(unsigned int)")]
// was: Ogre::RbxSceneManager::recordPassStats(unsigned int)
// IDA 0xbc97f4: 88 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc97f4() {
}

// 0xbc98e0 — __ZN4Ogre15RbxSceneManager18renderSingleObjectEPNS_10RenderableEPKNS_4PassEbbPKNS_12HashedVectorIPNS_5LightEEE
#[doc(alias = "Ogre::RbxSceneManager::renderSingleObject(Ogre::Renderable *,Ogre::Pass const*,bool,bool,Ogre::HashedVector<Ogre::Light *> const*)")]
// was: Ogre::RbxSceneManager::renderSingleObject(Ogre::Renderable *,Ogre::Pass const*,bool,bool,Ogre::HashedVector<Ogre::Light *> const*)
// IDA 0xbc98e0: 283 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc98e0() {
}

// 0xbc9bd0 — __ZN4Ogre15RbxSceneManager29updateRenderQueueSplitOptionsEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::updateRenderQueueSplitOptions(void)")]
// was: Ogre::RbxSceneManager::updateRenderQueueSplitOptions(void)
// IDA 0xbc9bd0: 23 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9bd0() {
}

// 0xbc9c0c — __ZN4Ogre15RbxSceneManager23_queueSkiesForRenderingEPNS_6CameraE
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, Ogre::Camera *)
#[doc(alias = "Ogre::RbxSceneManager::_queueSkiesForRendering(Ogre::Camera *)")]
// was: Ogre::RbxSceneManager::_queueSkiesForRendering(Ogre::Camera *)
// IDA 0xbc9c0c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9c0c() {
}

// 0xbc9c48 — __ZN4Ogre15RbxSceneManager19createSceneNodeImplEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::createSceneNodeImpl(void)")]
// was: Ogre::RbxSceneManager::createSceneNodeImpl(void)
// IDA 0xbc9c48: 67 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9c48() {
}

// 0xbc9d08 — __ZN4Ogre15RbxSceneManager19createSceneNodeImplERKSs
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::RbxSceneManager::createSceneNodeImpl(std::string const&)")]
// was: Ogre::RbxSceneManager::createSceneNodeImpl(std::string const&)
// IDA 0xbc9d08: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9d08() {
}

// 0xbc9dcc — __ZNK4Ogre15RbxSceneManager11getTypeNameEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::getTypeName(void)const")]
// was: Ogre::RbxSceneManager::getTypeName(void)const
// IDA 0xbc9dcc: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9dcc() {
}

// 0xbc9dd8 — __ZN4Ogre15RbxSceneManager19_findVisibleObjectsEPNS_6CameraEPNS_24VisibleObjectsBoundsInfoEb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, Ogre::Camera *, Ogre::VisibleObjectsBoundsInfo *, bool)
#[doc(alias = "Ogre::RbxSceneManager::_findVisibleObjects(Ogre::Camera *,Ogre::VisibleObjectsBoundsInfo *,bool)")]
// was: Ogre::RbxSceneManager::_findVisibleObjects(Ogre::Camera *,Ogre::VisibleObjectsBoundsInfo *,bool)
// IDA 0xbc9dd8: 223 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9dd8() {
}

// 0xbca050 — __ZN4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE
#[doc(alias = "Ogre::RbxSceneManager::preRenderTargetUpdate(Ogre::RenderTargetEvent const&)")]
// was: Ogre::RbxSceneManager::preRenderTargetUpdate(Ogre::RenderTargetEvent const&)
// IDA 0xbca050: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca050() {
}

// 0xbca17c — __ZThn17800_N4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE
#[doc(alias = "non-virtual thunk toOgre::RbxSceneManager::preRenderTargetUpdate(Ogre::RenderTargetEvent const&)")]
// was: non-virtual thunk to Ogre::RbxSceneManager::preRenderTargetUpdate(Ogre::RenderTargetEvent const&)
// IDA 0xbca17c: 3 insns (MOVW..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca17c() {
}

// 0xbca188 — __ZN4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE
#[doc(alias = "Ogre::RbxSceneManager::postRenderTargetUpdate(Ogre::RenderTargetEvent const&)")]
// was: Ogre::RbxSceneManager::postRenderTargetUpdate(Ogre::RenderTargetEvent const&)
// IDA 0xbca188: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca188() {
}

// 0xbca240 — __ZThn17800_N4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE
#[doc(alias = "non-virtual thunk toOgre::RbxSceneManager::postRenderTargetUpdate(Ogre::RenderTargetEvent const&)")]
// was: non-virtual thunk to Ogre::RbxSceneManager::postRenderTargetUpdate(Ogre::RenderTargetEvent const&)
// IDA 0xbca240: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca240() {
}

// 0xbca2f4 — __ZN4Ogre15RbxSceneManager15setShadowColourERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::RbxSceneManager::setShadowColour(Ogre::ColourValue const&)")]
// was: Ogre::RbxSceneManager::setShadowColour(Ogre::ColourValue const&)
// IDA 0xbca2f4: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca2f4() {
}

// 0xbca504 — __ZN4Ogre15RbxSceneManager21processSqPartDistanceEf
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, float)
#[doc(alias = "Ogre::RbxSceneManager::processSqPartDistance(float)")]
// was: Ogre::RbxSceneManager::processSqPartDistance(float)
// IDA 0xbca504: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca504() {
}

// 0xbca528 — __ZN4Ogre15RbxSceneManager18setPointOfInterestERKN3G3D7Vector3E
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const Vector3 *)
#[doc(alias = "Ogre::RbxSceneManager::setPointOfInterest(G3D::Vector3 const&)")]
// was: Ogre::RbxSceneManager::setPointOfInterest(G3D::Vector3 const&)
// IDA 0xbca528: 12 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca528() {
}

// 0xbca548 — __ZNK4Ogre22RbxSceneManagerFactory12initMetaDataEv
// type: _DWORD __fastcall(Ogre::RbxSceneManagerFactory *__hidden this)
#[doc(alias = "Ogre::RbxSceneManagerFactory::initMetaData(void)const")]
// was: Ogre::RbxSceneManagerFactory::initMetaData(void)const
// IDA 0xbca548: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca548() {
}

// 0xbca57c — __ZN4Ogre22RbxSceneManagerFactory14createInstanceERKSs
// type: _DWORD __fastcall(Ogre::RbxSceneManagerFactory *__hidden this, const std::string *)
#[doc(alias = "Ogre::RbxSceneManagerFactory::createInstance(std::string const&)")]
// was: Ogre::RbxSceneManagerFactory::createInstance(std::string const&)
// IDA 0xbca57c: 67 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca57c() {
}

// 0xbca63c — __ZN4Ogre22RbxSceneManagerFactory15destroyInstanceEPNS_12SceneManagerE
#[doc(alias = "Ogre::RbxSceneManagerFactory::destroyInstance(Ogre::SceneManager *)")]
// was: Ogre::RbxSceneManagerFactory::destroyInstance(Ogre::SceneManager *)
// IDA 0xbca63c: 7 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca63c() {
}

// 0xbca64c — __ZN5boost12scoped_arrayIN4Ogre11MaterialPtrEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
#[doc(alias = "boost::scoped_array<Ogre::MaterialPtr>::~scoped_array()")]
// was: boost::scoped_array<Ogre::MaterialPtr>::~scoped_array()
// IDA 0xbca64c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bca64c() {
}

// 0xbca87c — __ZN4Ogre9SharedPtrINS_6RbxSkyEED1Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "Ogre::SharedPtr<Ogre::RbxSky>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::RbxSky>::~SharedPtr()
// IDA 0xbca87c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bca87c() {
}

// 0xbca8d0 — __ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE7reserveEm
// type: int(void)
#[doc(alias = "std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::reserve(unsigned long)")]
// was: std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::reserve(unsigned long)
// IDA 0xbca8d0: 46 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca8d0() {
}

// 0xbca94c — __ZN4Ogre20RenderTargetListener17preViewportUpdateERKNS_25RenderTargetViewportEventE
#[doc(alias = "Ogre::RenderTargetListener::preViewportUpdate(Ogre::RenderTargetViewportEvent const&)")]
// was: Ogre::RenderTargetListener::preViewportUpdate(Ogre::RenderTargetViewportEvent const&)
// IDA 0xbca94c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_bca94c() {
}

// 0xbca950 — __ZN4Ogre20RenderTargetListener18postViewportUpdateERKNS_25RenderTargetViewportEventE
#[doc(alias = "Ogre::RenderTargetListener::postViewportUpdate(Ogre::RenderTargetViewportEvent const&)")]
// was: Ogre::RenderTargetListener::postViewportUpdate(Ogre::RenderTargetViewportEvent const&)
// IDA 0xbca950: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_bca950() {
}

// 0xbcaa4c — __ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
#[doc(alias = "std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ShadowRenderable *,bool>*,std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>>,std::pair<Ogre::ShadowRenderable *,bool> const&)")]
// was: std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ShadowRenderable *,bool>*,std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>>,std::pair<Ogre::ShadowRenderable *,bool> const&)
// IDA 0xbcaa4c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_bcaa4c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xbcab68 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvN4Ogre15RbxSceneManagerEEENSA_5list1INSA_5valueIPSF_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,Ogre::RbxSceneManager>,boost::_bi::list1<boost::_bi::value<Ogre::RbxSceneManager*>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,Ogre::RbxSceneManager>,boost::_bi::list1<boost::_bi::value<Ogre::RbxSceneManager*>>>>::~callable_slot()
// IDA 0xbcab68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bcab68() {
}

// 0xbcabc4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvN4Ogre15RbxSceneManagerEEENSA_5list1INSA_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,Ogre::RbxSceneManager>,boost::_bi::list1<boost::_bi::value<Ogre::RbxSceneManager*>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,Ogre::RbxSceneManager>,boost::_bi::list1<boost::_bi::value<Ogre::RbxSceneManager*>>>>::~callable_slot()
// IDA 0xbcabc4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bcabc4() {
}

// 0xbcaccc — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvN4Ogre15RbxSceneManagerEEENSB_5list1INSB_5valueIPSG_EEEEEELi2ES8_E4callES7_S7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,Ogre::RbxSceneManager>,boost::_bi::list1<boost::_bi::value<Ogre::RbxSceneManager*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,Ogre::RbxSceneManager>,boost::_bi::list1<boost::_bi::value<Ogre::RbxSceneManager*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
// IDA 0xbcaccc: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcaccc() {
}

// 0xbcace4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvN4Ogre15RbxSceneManagerEEENSB_5list1INSB_5valueIPSG_EEEEEELi2ES8_E4callES7_S7_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,Ogre::RbxSceneManager>,boost::_bi::list1<boost::_bi::value<Ogre::RbxSceneManager*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,Ogre::RbxSceneManager>,boost::_bi::list1<boost::_bi::value<Ogre::RbxSceneManager*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
// IDA 0xbcace4: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcace4() {
}

// 0xbcacfc — __ZN4Ogre9SharedPtrINS_6RbxSkyEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::RbxSky>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::RbxSky>::~SharedPtr()
// IDA 0xbcacfc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bcacfc() {
}

// 0xbcad9c — __ZN4Ogre9SharedPtrINS_6RbxSkyEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::RbxSky>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::RbxSky>::destroy(void)
// IDA 0xbcad9c: 25 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcad9c() {
}

// 0xbcadd4 — __ZN4Ogre9SharedPtrINS_6RbxSkyEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::RbxSky>::swap(Ogre::SharedPtr<Ogre::RbxSky>&)")]
// was: Ogre::SharedPtr<Ogre::RbxSky>::swap(Ogre::SharedPtr<Ogre::RbxSky>&)
// IDA 0xbcadd4: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcadd4() {
}

// 0xbcb48c — __ZN4Ogre12RbxSceneNode7_updateEbb
// type: _DWORD __fastcall(Ogre::RbxSceneNode *__hidden this, bool, bool)
#[doc(alias = "Ogre::RbxSceneNode::_update(bool,bool)")]
// was: Ogre::RbxSceneNode::_update(bool,bool)
// IDA 0xbcb48c: 33 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcb48c() {
}

// 0xbcb4f0 — __ZN4Ogre12RbxSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb
// type: _DWORD __fastcall(Ogre::RbxSceneNode *__hidden this, Ogre::Camera *, Ogre::RenderQueue *, Ogre::VisibleObjectsBoundsInfo *, bool, bool, bool)
#[doc(alias = "Ogre::RbxSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")]
// was: Ogre::RbxSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)
// IDA 0xbcb4f0: 138 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcb4f0() {
}

// 0xbcb64c — __ZN4Ogre12RbxSceneNode13_updateBoundsEv
// type: _DWORD __fastcall(Ogre::RbxSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxSceneNode::_updateBounds(void)")]
// was: Ogre::RbxSceneNode::_updateBounds(void)
// IDA 0xbcb64c: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcb64c() {
}

// 0xbcb6c0 — __ZN4Ogre12RbxSceneNode22setRenderableTransformERNS_10RenderableERKNS_7Matrix4E
// type: _DWORD __fastcall(Ogre::RbxSceneNode *__hidden this, Ogre::Renderable *, const Ogre::Matrix4 *)
#[doc(alias = "Ogre::RbxSceneNode::setRenderableTransform(Ogre::Renderable &,Ogre::Matrix4 const&)")]
// was: Ogre::RbxSceneNode::setRenderableTransform(Ogre::Renderable &,Ogre::Matrix4 const&)
// IDA 0xbcb6c0: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcb6c0() {
}

// 0xbcb75c — __ZNK4Ogre12RbxSceneNode22getRenderableTransformEPKNS_10RenderableE
// type: _DWORD __fastcall(Ogre::RbxSceneNode *__hidden this, const Ogre::Renderable *)
#[doc(alias = "Ogre::RbxSceneNode::getRenderableTransform(Ogre::Renderable const*)const")]
// was: Ogre::RbxSceneNode::getRenderableTransform(Ogre::Renderable const*)const
// IDA 0xbcb75c: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcb75c() {
}

// 0xbcb91c — __ZNK4Ogre12RbxSceneNode25renderableTransformExistsEPKNS_10RenderableE
#[doc(alias = "Ogre::RbxSceneNode::renderableTransformExists(Ogre::Renderable const*)const")]
// was: Ogre::RbxSceneNode::renderableTransformExists(Ogre::Renderable const*)const
// IDA 0xbcb91c: 26 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcb91c() {
}

// 0xbcb95c — __ZN4Ogre12RbxSceneNodeD0Ev
// type: void __fastcall(Ogre::RbxSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxSceneNode::~RbxSceneNode()")]
// was: Ogre::RbxSceneNode::~RbxSceneNode()
// IDA 0xbcb95c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bcb95c() {
}

// 0xbcba10 — __ZN4Ogre12RbxSceneNodeD1Ev
// type: void __fastcall(Ogre::RbxSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxSceneNode::~RbxSceneNode()")]
// was: Ogre::RbxSceneNode::~RbxSceneNode()
// IDA 0xbcba10: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bcba10() {
}

// 0xbcba14 — __ZN4Ogre12RbxSceneNodeD2Ev
// type: void __fastcall(Ogre::RbxSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxSceneNode::~RbxSceneNode()")]
// was: Ogre::RbxSceneNode::~RbxSceneNode()
// IDA 0xbcba14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bcba14() {
}

// 0xbcbb40 — __ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>> *)")]
// was: std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>> *)
// IDA 0xbcbb40: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcbb40() {
}

// 0xbcbb68 — __ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_insert_unique(std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *> const&)")]
// was: std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_insert_unique(std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *> const&)
// IDA 0xbcbb68: 93 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcbb68() {
}

// 0xbcc2e0 — __ZN4Ogre15RbxSceneUpdaterC1EN5boost10shared_ptrIN3RBX9DataModelEEEPNS_12VisualEngineE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "Ogre::RbxSceneUpdater::RbxSceneUpdater(rbx_core::SharedPtr<RBX::DataModel>,Ogre::VisualEngine *)")]
// was: Ogre::RbxSceneUpdater::RbxSceneUpdater(boost::shared_ptr<RBX::DataModel>,Ogre::VisualEngine *)
// IDA 0xbcc2e0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bcc2e0() {
}

// 0xbcc2e4 — __ZN4Ogre15RbxSceneUpdaterC2EN5boost10shared_ptrIN3RBX9DataModelEEEPNS_12VisualEngineE
// type: int __fastcall(rbx::signals::scoped_connection *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneUpdater::RbxSceneUpdater(rbx_core::SharedPtr<RBX::DataModel>,Ogre::VisualEngine *)")]
// was: Ogre::RbxSceneUpdater::RbxSceneUpdater(boost::shared_ptr<RBX::DataModel>,Ogre::VisualEngine *)
// IDA 0xbcc2e4: 194 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcc2e4() {
}

// 0xbcc50c — __ZN4Ogre15RbxSceneUpdaterD1Ev
// type: void __fastcall(Ogre::RbxSceneUpdater *__hidden this)
#[doc(alias = "Ogre::RbxSceneUpdater::~RbxSceneUpdater()")]
// was: Ogre::RbxSceneUpdater::~RbxSceneUpdater()
// IDA 0xbcc50c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bcc50c() {
}

// 0xbcc510 — __ZN4Ogre15RbxSceneUpdaterD2Ev
// type: void __fastcall(Ogre::RbxSceneUpdater *__hidden this)
#[doc(alias = "Ogre::RbxSceneUpdater::~RbxSceneUpdater()")]
// was: Ogre::RbxSceneUpdater::~RbxSceneUpdater()
// IDA 0xbcc510: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bcc510() {
}

// 0xbcc644 — __ZN4Ogre15RbxSceneUpdater4bindEv
// type: _DWORD __fastcall(Ogre::RbxSceneUpdater *__hidden this)
#[doc(alias = "Ogre::RbxSceneUpdater::bind(void)")]
// was: Ogre::RbxSceneUpdater::bind(void)
// IDA 0xbcc644: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcc644() {
}

// 0xbcc718 — __ZN4Ogre15RbxSceneUpdater6unbindEv
// type: _DWORD __fastcall(Ogre::RbxSceneUpdater *__hidden this)
#[doc(alias = "Ogre::RbxSceneUpdater::unbind(void)")]
// was: Ogre::RbxSceneUpdater::unbind(void)
// IDA 0xbcc718: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcc718() {
}

// 0xbcc77c — __ZN4Ogre15RbxSceneUpdater18terrainCellChangedERKN3RBX5Voxel14CellChangeInfoE
#[doc(alias = "Ogre::RbxSceneUpdater::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// was: Ogre::RbxSceneUpdater::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)
// IDA 0xbcc77c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcc77c() {
}

// 0xbcc7f0 — __ZN4Ogre15RbxSceneUpdater27lightingInvalidateOccupancyERKN3RBX7ExtentsERKN3G3D7Vector3Eb
// type: _DWORD __fastcall(Ogre::RbxSceneUpdater *__hidden this, const RBX::Extents *, const G3D::Vector3 *, bool)
#[doc(alias = "Ogre::RbxSceneUpdater::lightingInvalidateOccupancy(RBX::Extents const&,G3D::Vector3 const&,bool)")]
// was: Ogre::RbxSceneUpdater::lightingInvalidateOccupancy(RBX::Extents const&,G3D::Vector3 const&,bool)
// IDA 0xbcc7f0: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcc7f0() {
}

// 0xbcc82c — __ZN4Ogre15RbxSceneUpdater23lightingInvalidateLocalERKN3RBX7ExtentsE
// type: _DWORD __fastcall(Ogre::RbxSceneUpdater *__hidden this, const RBX::Extents *)
#[doc(alias = "Ogre::RbxSceneUpdater::lightingInvalidateLocal(RBX::Extents const&)")]
// was: Ogre::RbxSceneUpdater::lightingInvalidateLocal(RBX::Extents const&)
// IDA 0xbcc82c: 21 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcc82c() {
}

// 0xbcc868 — __ZN4Ogre15RbxSceneUpdater15computeLightingEb
// type: _DWORD __fastcall(Ogre::RbxSceneUpdater *__hidden this, Ogre::NedPoolingImpl *)
#[doc(alias = "Ogre::RbxSceneUpdater::computeLighting(bool)")]
// was: Ogre::RbxSceneUpdater::computeLighting(bool)
// IDA 0xbcc868: 944 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcc868() {
}

// 0xbcd3f4 — __ZN4Ogre15RbxSceneUpdater17checkFastClustersEv
// type: _DWORD __fastcall(Ogre::RbxSceneUpdater *__hidden this)
#[doc(alias = "Ogre::RbxSceneUpdater::checkFastClusters(void)")]
// was: Ogre::RbxSceneUpdater::checkFastClusters(void)
// IDA 0xbcd3f4: 225 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcd3f4() {
}

// 0xbcd68c — __ZN4Ogre15RbxSceneUpdater24checkAddSeenFastClustersERKN3RBX16SpatialGridIndexE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::RbxSceneUpdater::checkAddSeenFastClusters(RBX::SpatialGridIndex const&)")]
// was: Ogre::RbxSceneUpdater::checkAddSeenFastClusters(RBX::SpatialGridIndex const&)
// IDA 0xbcd68c: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcd68c() {
}

// 0xbcd6f8 — __ZN4Ogre15RbxSceneUpdater14addMegaClusterERKN5boost10shared_ptrIN3RBX12PartInstanceEEE
#[doc(alias = "Ogre::RbxSceneUpdater::addMegaCluster(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// was: Ogre::RbxSceneUpdater::addMegaCluster(boost::shared_ptr<RBX::PartInstance> const&)
// IDA 0xbcd6f8: 87 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcd6f8() {
}

// 0xbcd7f4 — __ZN4Ogre15RbxSceneUpdater11getHumanoidEPN3RBX12PartInstanceE
// type: _DWORD __fastcall(Ogre::RbxSceneUpdater *__hidden this, RBX::PartInstance *)
#[doc(alias = "Ogre::RbxSceneUpdater::getHumanoid(RBX::PartInstance *)")]
// was: Ogre::RbxSceneUpdater::getHumanoid(RBX::PartInstance *)
// IDA 0xbcd7f4: 187 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcd7f4() {
}

// 0xbcda1c — __ZN4Ogre15RbxSceneUpdater20getLightingTimeStatsEv
// type: _DWORD __fastcall(Ogre::RbxSceneUpdater *__hidden this)
#[doc(alias = "Ogre::RbxSceneUpdater::getLightingTimeStats(void)")]
// was: Ogre::RbxSceneUpdater::getLightingTimeStats(void)
// IDA 0xbcda1c: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcda1c() {
}

// 0xbcda30 — __ZN4Ogre15RbxSceneUpdater17getLightOldestAgeEv
// type: _DWORD __fastcall(Ogre::RbxSceneUpdater *__hidden this)
#[doc(alias = "Ogre::RbxSceneUpdater::getLightOldestAge(void)")]
// was: Ogre::RbxSceneUpdater::getLightOldestAge(void)
// IDA 0xbcda30: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcda30() {
}

// 0xbcda54 — __ZN4Ogre15RbxSceneUpdater11addFastPartERKN5boost10shared_ptrIN3RBX12PartInstanceEEEbb
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneUpdater::addFastPart(rbx_core::SharedPtr<RBX::PartInstance> const&,bool,bool)")]
// was: Ogre::RbxSceneUpdater::addFastPart(boost::shared_ptr<RBX::PartInstance> const&,bool,bool)
// IDA 0xbcda54: 192 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcda54() {
}

// 0xbcdc54 — __ZN4Ogre15RbxSceneUpdater18destroyFastClusterEPN3RBX11FastClusterE
// type: _DWORD __fastcall(Ogre::RbxSceneUpdater *__hidden this, RBX::FastCluster *)
#[doc(alias = "Ogre::RbxSceneUpdater::destroyFastCluster(RBX::FastCluster *)")]
// was: Ogre::RbxSceneUpdater::destroyFastCluster(RBX::FastCluster *)
// IDA 0xbcdc54: 100 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcdc54() {
}

// 0xbcdd84 — __ZN4Ogre15RbxSceneUpdater17createAttachementERKN5boost10shared_ptrIN3RBX8InstanceEEE
#[doc(alias = "Ogre::RbxSceneUpdater::createAttachement(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: Ogre::RbxSceneUpdater::createAttachement(boost::shared_ptr<RBX::Instance> const&)
// IDA 0xbcdd84: 469 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcdd84() {
}

// 0xbcf688 — __ZN3RBX11SpatialGridINS_11FastClusterEEC2ERKN3G3D7Vector3Ef
#[doc(alias = "RBX::SpatialGrid<RBX::FastCluster>::SpatialGrid(G3D::Vector3 const&,float)")]
// was: RBX::SpatialGrid<RBX::FastCluster>::SpatialGrid(G3D::Vector3 const&,float)
// IDA 0xbcf688: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcf688() {
}

// 0xbd0390 — __ZN4Ogre6RbxSkyC1EPNS_12SceneManagerERKSsb
// type: _DWORD __fastcall(Ogre::RbxSky *__hidden this, Ogre::SceneManager *, const std::string *, bool)
#[doc(alias = "Ogre::RbxSky::RbxSky(Ogre::SceneManager *,std::string const&,bool)")]
// was: Ogre::RbxSky::RbxSky(Ogre::SceneManager *,std::string const&,bool)
// IDA 0xbd0390: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bd0390() {
}

// 0xbd0398 — __ZN4Ogre6RbxSkyC2EPNS_12SceneManagerERKSsb
// type: _DWORD __fastcall(Ogre::RbxSky *__hidden this, Ogre::SceneManager *, const std::string *, bool)
#[doc(alias = "Ogre::RbxSky::RbxSky(Ogre::SceneManager *,std::string const&,bool)")]
// was: Ogre::RbxSky::RbxSky(Ogre::SceneManager *,std::string const&,bool)
// IDA 0xbd0398: 3121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd0398() {
}

// 0xbd25b0 — __ZN4Ogre6RbxSky15createStarFieldEi
// type: _DWORD __fastcall(Ogre::RbxSky *__hidden this, int)
#[doc(alias = "Ogre::RbxSky::createStarField(int)")]
// was: Ogre::RbxSky::createStarField(int)
// IDA 0xbd25b0: 268 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd25b0() {
}

// 0xbd2960 — __ZN4Ogre6RbxSkyD0Ev
// type: void __fastcall(Ogre::RbxSky *__hidden this)
#[doc(alias = "Ogre::RbxSky::~RbxSky()")]
// was: Ogre::RbxSky::~RbxSky()
// IDA 0xbd2960: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bd2960() {
}

// 0xbd2a00 — __ZN4Ogre6RbxSkyD1Ev
// type: void __fastcall(Ogre::RbxSky *__hidden this)
#[doc(alias = "Ogre::RbxSky::~RbxSky()")]
// was: Ogre::RbxSky::~RbxSky()
// IDA 0xbd2a00: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bd2a00() {
}

// 0xbd2a04 — __ZN4Ogre6RbxSkyD2Ev
// type: void __fastcall(Ogre::RbxSky *__hidden this)
#[doc(alias = "Ogre::RbxSky::~RbxSky()")]
// was: Ogre::RbxSky::~RbxSky()
// IDA 0xbd2a04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bd2a04() {
}

// 0xbd341c — __ZN4Ogre6RbxSky11setNumStarsEi
// type: _DWORD __fastcall(Ogre::RbxSky *__hidden this, int)
#[doc(alias = "Ogre::RbxSky::setNumStars(int)")]
// was: Ogre::RbxSky::setNumStars(int)
// IDA 0xbd341c: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd341c() {
}

// 0xbd3460 — __ZN4Ogre6RbxSky6updateEPNS_12VisualEngineERKN3G3D18LightingParametersE
// type: _DWORD __fastcall(Ogre::RbxSky *__hidden this, Ogre::VisualEngine *, const G3D::LightingParameters *)
#[doc(alias = "Ogre::RbxSky::update(Ogre::VisualEngine *,G3D::LightingParameters const&)")]
// was: Ogre::RbxSky::update(Ogre::VisualEngine *,G3D::LightingParameters const&)
// IDA 0xbd3460: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd3460() {
}

// 0xbd3528 — __ZN4Ogre6RbxSky16drawMoonAndStarsEPNS_12VisualEngineERKN3G3D18LightingParametersE
// type: _DWORD __fastcall(Ogre::RbxSky *__hidden this, Ogre::VisualEngine *, const G3D::LightingParameters *)
#[doc(alias = "Ogre::RbxSky::drawMoonAndStars(Ogre::VisualEngine *,G3D::LightingParameters const&)")]
// was: Ogre::RbxSky::drawMoonAndStars(Ogre::VisualEngine *,G3D::LightingParameters const&)
// IDA 0xbd3528: 363 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd3528() {
}

// 0xbd3a90 — __ZN4Ogre6RbxSky7drawSunEPNS_12VisualEngineERKN3G3D18LightingParametersE
#[doc(alias = "Ogre::RbxSky::drawSun(Ogre::VisualEngine *,G3D::LightingParameters const&)")]
// was: Ogre::RbxSky::drawSun(Ogre::VisualEngine *,G3D::LightingParameters const&)
// IDA 0xbd3a90: 140 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd3a90() {
}