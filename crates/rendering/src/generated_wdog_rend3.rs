//! rendering — wdog cron rend3 — 120 stubs EA-sorted asc Rendering|Gfx|Render global-deduped
//! Range: 0xbb6e18..0xc354bc (120 stubs, EA-sorted asc, distinct not yet in /tmp/global_eas.txt)
//! Source: ida/export.json (85545 funcs, 2510 Rendering|Gfx|Render total, 1423 already stubbed, 1087 remaining before -> 967 after)
//! Each stub preserves IDA ea + mangled + demangled for rg. Uses rbx_core::SharedPtr not boost.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0xbb6e18 — __ZN4Ogre14VertexStreamer4initEPNS_12VisualEngineEPNS_12SceneManagerEPNS_12RenderWindowEb
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this, Ogre::VisualEngine *, Ogre::SceneManager *, Ogre::RenderWindow *, bool)
#[doc(alias = "Ogre::VertexStreamer::init(Ogre::VisualEngine *,Ogre::SceneManager *,Ogre::RenderWindow *,bool)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer4initEPNS_12VisualEngineEPNS_12SceneManagerEPNS_12RenderWindowEb")]
// IDA 0xbb6e18: 8 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb6e18() {
}

// 0xbb79f0 — __ZN4Ogre14VertexStreamer12prepareChunkERKNS_10TexturePtrENS_15RenderOperation13OperationTypeENS0_15CoordinateSpaceENS0_10VextexTypeEbb
#[doc(alias = "Ogre::VertexStreamer::prepareChunk(Ogre::TexturePtr const&,Ogre::RenderOperation::OperationType,Ogre::VertexStreamer::CoordinateSpace,Ogre::VertexStreamer::VextexType,bool,bool)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer12prepareChunkERKNS_10TexturePtrENS_15RenderOperation13OperationTypeENS0_15CoordinateSpaceENS0_10VextexTypeEbb")]
// IDA 0xbb79f0: 269 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb79f0() {
}

// 0xbb9540 — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE5setupEPNS_12RenderSystemE
// type: int(void)
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::setup(Ogre::RenderSystem *)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE5setupEPNS_12RenderSystemE")]
// IDA 0xbb9540: 170 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb9540() {
}

// 0xbb97fc — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE5setupEPNS_12RenderSystemE
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::setup(Ogre::RenderSystem *)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE5setupEPNS_12RenderSystemE")]
// IDA 0xbb97fc: 171 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb97fc() {
}

// 0xbc8ebc — __ZN4Ogre15RbxSceneManager22renderQueueGroupSolidsEhNS_26QueuedRenderableCollection16OrganisationModeEb
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, char, char, char, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneManager::renderQueueGroupSolids(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode,bool)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager22renderQueueGroupSolidsEhNS_26QueuedRenderableCollection16OrganisationModeEb")]
// IDA 0xbc8ebc: 452 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc8ebc() {
}

// 0xbc9490 — __ZN4Ogre15RbxSceneManager28renderQueueGroupTransparentsEhNS_26QueuedRenderableCollection16OrganisationModeE
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneManager::renderQueueGroupTransparents(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager28renderQueueGroupTransparentsEhNS_26QueuedRenderableCollection16OrganisationModeE")]
// IDA 0xbc9490: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9490() {
}

// 0xbc98e0 — __ZN4Ogre15RbxSceneManager18renderSingleObjectEPNS_10RenderableEPKNS_4PassEbbPKNS_12HashedVectorIPNS_5LightEEE
#[doc(alias = "Ogre::RbxSceneManager::renderSingleObject(Ogre::Renderable *,Ogre::Pass const*,bool,bool,Ogre::HashedVector<Ogre::Light *> const*)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager18renderSingleObjectEPNS_10RenderableEPKNS_4PassEbbPKNS_12HashedVectorIPNS_5LightEEE")]
// IDA 0xbc98e0: 283 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc98e0() {
}

// 0xbc9bd0 — __ZN4Ogre15RbxSceneManager29updateRenderQueueSplitOptionsEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::updateRenderQueueSplitOptions(void)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager29updateRenderQueueSplitOptionsEv")]
// IDA 0xbc9bd0: 23 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9bd0() {
}

// 0xbc9c0c — __ZN4Ogre15RbxSceneManager23_queueSkiesForRenderingEPNS_6CameraE
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, Ogre::Camera *)
#[doc(alias = "Ogre::RbxSceneManager::_queueSkiesForRendering(Ogre::Camera *)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager23_queueSkiesForRenderingEPNS_6CameraE")]
// IDA 0xbc9c0c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9c0c() {
}

// 0xbca050 — __ZN4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE
#[doc(alias = "Ogre::RbxSceneManager::preRenderTargetUpdate(Ogre::RenderTargetEvent const&)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE")]
// IDA 0xbca050: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca050() {
}

// 0xbca17c — __ZThn17800_N4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE
#[doc = "`non-virtual thunk to'Ogre::RbxSceneManager::preRenderTargetUpdate(Ogre::RenderTargetEvent const&)"]
#[doc(alias = "__ZThn17800_N4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE")]
// IDA 0xbca17c: 3 insns (MOVW..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca17c() {
}

// 0xbca188 — __ZN4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE
#[doc(alias = "Ogre::RbxSceneManager::postRenderTargetUpdate(Ogre::RenderTargetEvent const&)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE")]
// IDA 0xbca188: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca188() {
}

// 0xbca240 — __ZThn17800_N4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE
#[doc = "`non-virtual thunk to'Ogre::RbxSceneManager::postRenderTargetUpdate(Ogre::RenderTargetEvent const&)"]
#[doc(alias = "__ZThn17800_N4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE")]
// IDA 0xbca240: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca240() {
}

// 0xbca8d0 — __ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE7reserveEm
// type: int(void)
#[doc(alias = "std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE7reserveEm")]
// IDA 0xbca8d0: 46 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca8d0() {
}

// 0xbca94c — __ZN4Ogre20RenderTargetListener17preViewportUpdateERKNS_25RenderTargetViewportEventE
#[doc(alias = "Ogre::RenderTargetListener::preViewportUpdate(Ogre::RenderTargetViewportEvent const&)")]
#[doc(alias = "__ZN4Ogre20RenderTargetListener17preViewportUpdateERKNS_25RenderTargetViewportEventE")]
// IDA 0xbca94c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_bca94c() {
}

// 0xbca950 — __ZN4Ogre20RenderTargetListener18postViewportUpdateERKNS_25RenderTargetViewportEventE
#[doc(alias = "Ogre::RenderTargetListener::postViewportUpdate(Ogre::RenderTargetViewportEvent const&)")]
#[doc(alias = "__ZN4Ogre20RenderTargetListener18postViewportUpdateERKNS_25RenderTargetViewportEventE")]
// IDA 0xbca950: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_bca950() {
}

// 0xbcaa4c — __ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
#[doc(alias = "std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ShadowRenderable *,bool>*,std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>>,std::pair<Ogre::ShadowRenderable *,bool> const&)")]
#[doc(alias = "__ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_")]
// IDA 0xbcaa4c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_bcaa4c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xbcb4f0 — __ZN4Ogre12RbxSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb
// type: _DWORD __fastcall(Ogre::RbxSceneNode *__hidden this, Ogre::Camera *, Ogre::RenderQueue *, Ogre::VisibleObjectsBoundsInfo *, bool, bool, bool)
#[doc(alias = "Ogre::RbxSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")]
#[doc(alias = "__ZN4Ogre12RbxSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb")]
// IDA 0xbcb4f0: 138 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcb4f0() {
}

// 0xbcb6c0 — __ZN4Ogre12RbxSceneNode22setRenderableTransformERNS_10RenderableERKNS_7Matrix4E
// type: _DWORD __fastcall(Ogre::RbxSceneNode *__hidden this, Ogre::Renderable *, const Ogre::Matrix4 *)
#[doc(alias = "Ogre::RbxSceneNode::setRenderableTransform(Ogre::Renderable &,Ogre::Matrix4 const&)")]
#[doc(alias = "__ZN4Ogre12RbxSceneNode22setRenderableTransformERNS_10RenderableERKNS_7Matrix4E")]
// IDA 0xbcb6c0: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcb6c0() {
}

// 0xbcb75c — __ZNK4Ogre12RbxSceneNode22getRenderableTransformEPKNS_10RenderableE
// type: _DWORD __fastcall(Ogre::RbxSceneNode *__hidden this, const Ogre::Renderable *)
#[doc(alias = "Ogre::RbxSceneNode::getRenderableTransform(Ogre::Renderable const*)const")]
#[doc(alias = "__ZNK4Ogre12RbxSceneNode22getRenderableTransformEPKNS_10RenderableE")]
// IDA 0xbcb75c: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcb75c() {
}

// 0xbcb91c — __ZNK4Ogre12RbxSceneNode25renderableTransformExistsEPKNS_10RenderableE
#[doc(alias = "Ogre::RbxSceneNode::renderableTransformExists(Ogre::Renderable const*)const")]
#[doc(alias = "__ZNK4Ogre12RbxSceneNode25renderableTransformExistsEPKNS_10RenderableE")]
// IDA 0xbcb91c: 26 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcb91c() {
}

// 0xbcbb40 — __ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// IDA 0xbcbb40: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcbb40() {
}

// 0xbcbb68 — __ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_insert_unique(std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// IDA 0xbcbb68: 93 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcbb68() {
}

// 0xbd3c88 — __ZN4Ogre6RbxSky17updateRenderQueueEPNS_6CameraEPNS_11RenderQueueENS_18RenderQueueGroupIDE
// type: int __fastcall(int, Ogre::Camera *this)
#[doc(alias = "Ogre::RbxSky::updateRenderQueue(Ogre::Camera *,Ogre::RenderQueue *,Ogre::RenderQueueGroupID)")]
#[doc(alias = "__ZN4Ogre6RbxSky17updateRenderQueueEPNS_6CameraEPNS_11RenderQueueENS_18RenderQueueGroupIDE")]
// IDA 0xbd3c88: 629 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd3c88() {
}

// 0xbd4ed0 — __ZN4Ogre12RbxSubEntity18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::RbxSubEntity::getRenderOperation(Ogre::RenderOperation &)")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity18getRenderOperationERNS_15RenderOperationE")]
// IDA 0xbd4ed0: 10 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd4ed0() {
}

// 0xbd55cc — __ZN4Ogre12RbxSubEntity33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm
// type: int __fastcall(int, int, int, int, int, int, float, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::RbxSubEntity::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm")]
// IDA 0xbd55cc: 388 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd55cc() {
}

// 0xbd59e8 — __ZThn264_N4Ogre12RbxSubEntity33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm
// type: int __fastcall(int, int, int, int, int, int, float, int)
#[doc = "`non-virtual thunk to'Ogre::RbxSubEntity::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)"]
#[doc(alias = "__ZThn264_N4Ogre12RbxSubEntity33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm")]
// IDA 0xbd59e8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd59e8() {
}

// 0xbd5cc4 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableC2EPNS_6EntityEPNS_28HardwareIndexBufferSharedPtrEPKNS_10VertexDataEbPNS_9SubEntityEb
// type: _DWORD __fastcall(Ogre::RbxSubEntity::RbxSubEntityShadowRenderable *__hidden this, Ogre::Entity *, Ogre::HardwareIndexBufferSharedPtr *, const Ogre::VertexData *, bool, Ogre::SubEntity *, bool)
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::RbxSubEntityShadowRenderable(Ogre::Entity *,Ogre::HardwareIndexBufferSharedPtr *,Ogre::VertexData const*,bool,Ogre::SubEntity *,bool)")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableC2EPNS_6EntityEPNS_28HardwareIndexBufferSharedPtrEPKNS_10VertexDataEbPNS_9SubEntityEb")]
// IDA 0xbd5cc4: 569 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd5cc4() {
}

// 0xbd6240 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD0Ev
// type: void __fastcall(Ogre::RbxSubEntity::RbxSubEntityShadowRenderable *__hidden this)
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD0Ev")]
// IDA 0xbd6240: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bd6240() {
}

// 0xbd62f4 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD1Ev
// type: void __fastcall(Ogre::RbxSubEntity::RbxSubEntityShadowRenderable *__hidden this)
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD1Ev")]
// IDA 0xbd62f4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bd62f4() {
}

// 0xbd62f8 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD2Ev
// type: void __fastcall(Ogre::RbxSubEntity::RbxSubEntityShadowRenderable *__hidden this)
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD2Ev")]
// IDA 0xbd62f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bd62f8() {
}

// 0xbd661c — __ZNK4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")]
#[doc(alias = "__ZNK4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable18getWorldTransformsEPNS_7Matrix4E")]
// IDA 0xbd661c: 4 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd661c() {
}

// 0xbd6628 — __ZNK4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable9isVisibleEv
// type: _DWORD __fastcall(Ogre::RbxSubEntity::RbxSubEntityShadowRenderable *__hidden this)
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::isVisible(void)const")]
#[doc(alias = "__ZNK4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable9isVisibleEv")]
// IDA 0xbd6628: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd6628() {
}

// 0xbd6638 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable17rebindIndexBufferERKNS_28HardwareIndexBufferSharedPtrE
// type: _DWORD __fastcall(Ogre::RbxSubEntity::RbxSubEntityShadowRenderable *__hidden this, const Ogre::HardwareIndexBufferSharedPtr *)
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable17rebindIndexBufferERKNS_28HardwareIndexBufferSharedPtrE")]
// IDA 0xbd6638: 95 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd6638() {
}

// 0xbec65c — __ZN3RBX10ViewRbxGfx10printSceneEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::printScene(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx10printSceneEv")]
// IDA 0xbec65c: 145 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bec65c() {
}

// 0xbec7fc — __ZThn4_N3RBX10ViewRbxGfx10printSceneEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc = "`non-virtual thunk to'RBX::ViewRbxGfx::printScene(void)"]
#[doc(alias = "__ZThn4_N3RBX10ViewRbxGfx10printSceneEv")]
// IDA 0xbec7fc: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bec7fc() {
}

// 0xbec808 — __ZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricE
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, RBX::IMetric *)
#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricE")]
// IDA 0xbec808: 1240 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bec808() {
}

// 0xbed5b8 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD1Ev
#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::~ProxyMetric()")]
#[doc(alias = "__ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD1Ev")]
// IDA 0xbed5b8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_bed5b8() {
}

// 0xbed5c0 — __ZN3RBX10ViewRbxGfx13renderPerformEd
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, double)
#[doc(alias = "RBX::ViewRbxGfx::renderPerform(double)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx13renderPerformEd")]
// IDA 0xbed5c0: 956 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bed5c0() {
}

// 0xbee4c0 — __ZN3RBX10ViewRbxGfx20saveScreenshotToFileERSs
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, std::string *)
#[doc(alias = "RBX::ViewRbxGfx::saveScreenshotToFile(std::string &)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx20saveScreenshotToFileERSs")]
// IDA 0xbee4c0: 425 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bee4c0() {
}

// 0xbee96c — __ZN3RBX10ViewRbxGfx6updateEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::update(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx6updateEv")]
// IDA 0xbee96c: 100 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bee96c() {
}

// 0xbeea8c — __ZN3RBX10ViewRbxGfx8buildGuiEb
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, bool)
#[doc(alias = "RBX::ViewRbxGfx::buildGui(bool)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx8buildGuiEb")]
// IDA 0xbeea8c: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_beea8c() {
}

// 0xbeeaac — __ZN3RBX10ViewRbxGfx14getRenderStatsEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::getRenderStats(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx14getRenderStatsEv")]
// IDA 0xbeeaac: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_beeaac() {
}

// 0xbeeab8 — __ZN3RBX10ViewRbxGfx11renderThumbEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::renderThumb(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx11renderThumbEv")]
// IDA 0xbeeab8: 419 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_beeab8() {
}

// 0xbeef64 — __ZN3RBX10ViewRbxGfx13writeRTToFileERKSs
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, const std::string *)
#[doc(alias = "RBX::ViewRbxGfx::writeRTToFile(std::string const&)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx13writeRTToFileERKSs")]
// IDA 0xbeef64: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_beef64() {
}

// 0xbeefb0 — __ZN3RBX10ViewRbxGfx15writeRTToBufferEPhiii
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, unsigned __int8 *, int, int, int)
#[doc(alias = "RBX::ViewRbxGfx::writeRTToBuffer(unsigned char *,int,int,int)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx15writeRTToBufferEPhiii")]
// IDA 0xbeefb0: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_beefb0() {
}

// 0xbef0f8 — __ZN3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, std::string *this)
#[doc(alias = "RBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE")]
// IDA 0xbef0f8: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef0f8() {
}

// 0xbef138 — __ZThn8_N3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, std::string *this)
#[doc = "`non-virtual thunk to'RBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)"]
#[doc(alias = "__ZThn8_N3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE")]
// IDA 0xbef138: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef138() {
}

// 0xbef174 — __ZN3RBX10ViewRbxGfx10startFrameEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::startFrame(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx10startFrameEv")]
// IDA 0xbef174: 39 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef174() {
}

// 0xbef1f0 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::~ProxyMetric()")]
#[doc(alias = "__ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD0Ev")]
// IDA 0xbef1f0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bef1f0() {
}

// 0xbef1f4 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric9getMetricERKSs
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::getMetric(std::string const&)const")]
#[doc(alias = "__ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric9getMetricERKSs")]
// IDA 0xbef1f4: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef1f4() {
}

// 0xbef230 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric14getMetricValueERKSs
// type: int __fastcall(int, std::string *this)
#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::getMetricValue(std::string const&)const")]
#[doc(alias = "__ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric14getMetricValueERKSs")]
// IDA 0xbef230: 26 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef230() {
}

// 0xbef270 — __ZZN3RBX21ViewRbxGfx_InitModuleEvEN17ViewRbxGfxFactory6CreateENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_
#[doc(alias = "RBX::ViewRbxGfx_InitModule(void)::ViewRbxGfxFactory::Create(RBX::CRenderSettings::GraphicsMode,RBX::OSContext *,RBX::CRenderSettings*)")]
#[doc(alias = "__ZZN3RBX21ViewRbxGfx_InitModuleEvEN17ViewRbxGfxFactory6CreateENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_")]
// IDA 0xbef270: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef270() {
}

// 0xbef8c0 — __ZN3RBX10ViewRbxGfx8throttleEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::throttle(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx8throttleEv")]
// IDA 0xbef8c0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_bef8c0() {
}

// 0xbef8cc — __ZN3RBX10ViewRbxGfx8getAdornEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::getAdorn(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx8getAdornEv")]
// IDA 0xbef8cc: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef8cc() {
}

// 0xbf0e94 — __ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")]
// IDA 0xbf0e94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf0e94() {
}

// 0xbf0ef0 — __ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")]
// IDA 0xbf0ef0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf0ef0() {
}

// 0xbf10f4 — __ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(int)>::call(int)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi")]
// IDA 0xbf10f4: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf10f4() {
}

// 0xbf110c — __ZThn4_N3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi
#[doc = "`non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(int)>::call(int)"]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi")]
// IDA 0xbf110c: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf110c() {
}

// 0xbf18ac — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")]
// IDA 0xbf18ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf18ac() {
}

// 0xbf1908 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")]
// IDA 0xbf1908: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf1908() {
}

// 0xbf1a14 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
// IDA 0xbf1a14: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf1a14() {
}

// 0xbf1a2c — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// type: int __fastcall(_DWORD *)
#[doc = "`non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>,0,void ()(void)>::call(void)"]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
// IDA 0xbf1a2c: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf1a2c() {
}

// 0xbf1afc — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")]
// IDA 0xbf1afc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf1afc() {
}

// 0xbf1b58 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")]
// IDA 0xbf1b58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf1b58() {
}

// 0xbf1d5c — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")]
// IDA 0xbf1d5c: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf1d5c() {
}

// 0xbf1d74 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
#[doc = "`non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)"]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")]
// IDA 0xbf1d74: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf1d74() {
}

// 0xbfb48c — __ZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this, Ogre::Camera *, Ogre::RenderQueue *, Ogre::VisibleObjectsBoundsInfo *, bool, bool, bool)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")]
#[doc(alias = "__ZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb")]
// IDA 0xbfb48c: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb48c() {
}

// 0xbfb568 — __ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter10IntersectsERKN3RBX7ExtentsE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)::NodeVisiter::Intersects(RBX::Extents const&)")]
#[doc(alias = "__ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter10IntersectsERKN3RBX7ExtentsE")]
// IDA 0xbfb568: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb568() {
}

// 0xbfb69c — __ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter8DistanceERKN3RBX7ExtentsE
// type: int __fastcall(int, RBX::Extents *this)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)::NodeVisiter::Distance(RBX::Extents const&)")]
#[doc(alias = "__ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter8DistanceERKN3RBX7ExtentsE")]
// IDA 0xbfb69c: 4 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb69c() {
}

// 0xbfb6a8 — __ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter11onPrimitiveEPNS_20RbxCullableSceneNodeEN3RBX15IntersectResultEf
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)::NodeVisiter::onPrimitive(Ogre::RbxCullableSceneNode *,RBX::IntersectResult,float)")]
#[doc(alias = "__ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter11onPrimitiveEPNS_20RbxCullableSceneNodeEN3RBX15IntersectResultEf")]
// IDA 0xbfb6a8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb6a8() {
}

// 0xc02d80 — __ZN19ResourceGroupHelper31UpdateMaterialRenderableVisitor5visitEPN4Ogre10RenderableEtbPNS1_3AnyE
// type: _DWORD __fastcall(ResourceGroupHelper::UpdateMaterialRenderableVisitor *__hidden this, Ogre::Renderable *, unsigned __int16, bool, Ogre::Any *)
#[doc(alias = "ResourceGroupHelper::UpdateMaterialRenderableVisitor::visit(Ogre::Renderable *,unsigned short,bool,Ogre::Any *)")]
#[doc(alias = "__ZN19ResourceGroupHelper31UpdateMaterialRenderableVisitor5visitEPN4Ogre10RenderableEtbPNS1_3AnyE")]
// IDA 0xc02d80: 439 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c02d80() {
}

// 0xc03db8 — __ZL28updateMaterialsOnRenderNodesPKN4Ogre9SceneNodeE
// type: _DWORD __fastcall(const Ogre::SceneNode *)
#[doc(alias = "updateMaterialsOnRenderNodes(Ogre::SceneNode const*)")]
#[doc(alias = "__ZL28updateMaterialsOnRenderNodesPKN4Ogre9SceneNodeE")]
// IDA 0xc03db8: 852 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c03db8() {
}

// 0xc04658 — __ZN19ResourceGroupHelper31visitRecursivelyRenderablesFromEPN4Ogre16OverlayContainerERNS0_10Renderable7VisitorEb
#[doc(alias = "ResourceGroupHelper::visitRecursivelyRenderablesFrom(Ogre::OverlayContainer *,Ogre::Renderable::Visitor &,bool)")]
#[doc(alias = "__ZN19ResourceGroupHelper31visitRecursivelyRenderablesFromEPN4Ogre16OverlayContainerERNS0_10Renderable7VisitorEb")]
// IDA 0xc04658: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c04658() {
}

// 0xc08ddc — __ZN3RBX11MegaCluster19createSolidGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::MegaCluster::createSolidGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)")]
#[doc(alias = "__ZN3RBX11MegaCluster19createSolidGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj")]
// IDA 0xc08ddc: 208 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c08ddc() {
}

// 0xc08fec — __ZN3RBX11MegaCluster19createWaterGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::MegaCluster::createWaterGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)")]
#[doc(alias = "__ZN3RBX11MegaCluster19createWaterGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj")]
// IDA 0xc08fec: 230 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c08fec() {
}

// 0xc0923c — __ZN3RBX11MegaCluster14createGeometryEPNS_10RenderNodeERKN4Ogre29HardwareVertexBufferSharedPtrEPKch
// type: _DWORD __fastcall(RBX::MegaCluster *__hidden this, RBX::RenderNode *, const Ogre::HardwareVertexBufferSharedPtr *, const char *, unsigned __int8)
#[doc(alias = "RBX::MegaCluster::createGeometry(RBX::RenderNode *,Ogre::HardwareVertexBufferSharedPtr const&,char const*,unsigned char)")]
#[doc(alias = "__ZN3RBX11MegaCluster14createGeometryEPNS_10RenderNodeERKN4Ogre29HardwareVertexBufferSharedPtrEPKch")]
// IDA 0xc0923c: 535 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0923c() {
}

// 0xc0a4ec — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_20SolidTerrainRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_20SolidTerrainRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")]
// IDA 0xc0a4ec: 759 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0a4ec() {
}

// 0xc0acec — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_17WaterFaceRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_17WaterFaceRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")]
// IDA 0xc0acec: 696 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0acec() {
}

// 0xc0b430 — __ZN3RBX10GfxBinding16invalidateEntityEv
// type: _DWORD __fastcall(RBX::GfxBinding *__hidden this)
#[doc(alias = "RBX::GfxBinding::invalidateEntity(void)")]
#[doc(alias = "__ZN3RBX10GfxBinding16invalidateEntityEv")]
// IDA 0xc0b430: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c0b430() {
}

// 0xc0b434 — __ZN3RBX10GfxBinding24onCoordinateFrameChangedEv
// type: _DWORD __fastcall(RBX::GfxBinding *__hidden this)
#[doc(alias = "RBX::GfxBinding::onCoordinateFrameChanged(void)")]
#[doc(alias = "__ZN3RBX10GfxBinding24onCoordinateFrameChangedEv")]
// IDA 0xc0b434: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c0b434() {
}

// 0xc0b438 — __ZN3RBX7GfxPart21updateCoordinateFrameEb
// type: _DWORD __fastcall(RBX::GfxPart *__hidden this, bool)
#[doc(alias = "RBX::GfxPart::updateCoordinateFrame(bool)")]
#[doc(alias = "__ZN3RBX7GfxPart21updateCoordinateFrameEb")]
// IDA 0xc0b438: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c0b438() {
}

// 0xc0b43c — __ZN3RBX7GfxPart19getFastFuzzyExtentsEv
// type: _DWORD __fastcall(RBX::GfxPart *__hidden this)
#[doc(alias = "RBX::GfxPart::getFastFuzzyExtents(void)")]
#[doc(alias = "__ZN3RBX7GfxPart19getFastFuzzyExtentsEv")]
// IDA 0xc0b43c: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0b43c() {
}

// 0xc0b4cc — __ZN3RBX7GfxPart12getPartCountEv
// type: _DWORD __fastcall(RBX::GfxPart *__hidden this)
#[doc(alias = "RBX::GfxPart::getPartCount(void)")]
#[doc(alias = "__ZN3RBX7GfxPart12getPartCountEv")]
// IDA 0xc0b4cc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0b4cc() {
}

// 0xc0b4d4 — __ZN3RBX7GfxPart14onClumpChangedEv
// type: _DWORD __fastcall(RBX::GfxPart *__hidden this)
#[doc(alias = "RBX::GfxPart::onClumpChanged(void)")]
#[doc(alias = "__ZN3RBX7GfxPart14onClumpChangedEv")]
// IDA 0xc0b4d4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c0b4d4() {
}

// 0xc0b4d8 — __ZNK3RBX20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE8internalERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionE
#[doc(alias = "RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
#[doc(alias = "__ZNK3RBX20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE8internalERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionE")]
// IDA 0xc0b4d8: 139 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0b4d8() {
}

// 0xc0b66c — __ZN3RBX17WaterFaceRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX17WaterFaceRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE")]
// IDA 0xc0b66c: 274 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0b66c() {
}

// 0xc0bf18 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")]
// IDA 0xc0bf18: 683 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0bf18() {
}

// 0xc0c648 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE")]
// IDA 0xc0c648: 218 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0c648() {
}

// 0xc0c904 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE9wedgeFaceERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE9wedgeFaceERKNS1_6RegionINS3_5ChunkEE8iteratorE")]
// IDA 0xc0c904: 330 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0c904() {
}

// 0xc0cd30 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE19detectWedgeOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE19detectWedgeOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorE")]
// IDA 0xc0cd30: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0cd30() {
}

// 0xc0cf1c — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12wedgeUpEmptyERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12wedgeUpEmptyERKNS1_6RegionINS3_5ChunkEE8iteratorE")]
// IDA 0xc0cf1c: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0cf1c() {
}

// 0xc0d000 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE14detectOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE14detectOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE")]
// IDA 0xc0d000: 138 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0d000() {
}

// 0xc0d190 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12renderHelperENS1_4CellENS1_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS1_13FaceDirectionEh
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12renderHelperENS1_4CellENS1_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS1_13FaceDirectionEh")]
// IDA 0xc0d190: 188 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0d190() {
}

// 0xc0d418 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
// type: int __fastcall(unsigned int *, _WORD *)
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")]
// IDA 0xc0d418: 728 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0d418() {
}

// 0xc0dbd8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED1Ev")]
// IDA 0xc0dbd8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c0dbd8() {
}

// 0xc0dbdc — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED0Ev")]
// IDA 0xc0dbdc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c0dbdc() {
}

// 0xc0dbe0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE7disposeEv")]
// IDA 0xc0dbe0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0dbe0() {
}

// 0xc0dbf0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE11get_deleterERKSt9type_info")]
// IDA 0xc0dbf0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0dbf0() {
}

// 0xc0dbf4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE19get_untyped_deleterEv")]
// IDA 0xc0dbf4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0dbf4() {
}

// 0xc14f78 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_20SolidTerrainRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_20SolidTerrainRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")]
// IDA 0xc14f78: 761 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c14f78() {
}

// 0xc15780 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE")]
// IDA 0xc15780: 218 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c15780() {
}

// 0xc15a3c — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE9wedgeFaceERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE9wedgeFaceERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE")]
// IDA 0xc15a3c: 330 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c15a3c() {
}

// 0xc15e68 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE")]
// IDA 0xc15e68: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c15e68() {
}

// 0xc16054 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12wedgeUpEmptyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12wedgeUpEmptyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE")]
// IDA 0xc16054: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c16054() {
}

// 0xc16138 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE14detectOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE14detectOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE")]
// IDA 0xc16138: 138 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c16138() {
}

// 0xc162c8 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12renderHelperENS_5Voxel4CellENS4_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS4_13FaceDirectionEh
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12renderHelperENS_5Voxel4CellENS4_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS4_13FaceDirectionEh")]
// IDA 0xc162c8: 188 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c162c8() {
}

// 0xc16550 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
// type: int __fastcall(unsigned int *, _WORD *)
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")]
// IDA 0xc16550: 730 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c16550() {
}

// 0xc16d18 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_17WaterFaceRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_17WaterFaceRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")]
// IDA 0xc16d18: 698 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c16d18() {
}

// 0xc17464 — __ZNK3RBX20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEE8internalERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionE
#[doc(alias = "RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
#[doc(alias = "__ZNK3RBX20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEE8internalERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionE")]
// IDA 0xc17464: 139 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c17464() {
}

// 0xc175f8 — __ZN3RBX17WaterFaceRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX17WaterFaceRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE")]
// IDA 0xc175f8: 274 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c175f8() {
}

// 0xc1799c — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")]
// IDA 0xc1799c: 685 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c1799c() {
}

// 0xc18ea8 — __ZN3RBX10GfxBinding12updateEntityEb
// type: _DWORD __fastcall(RBX::GfxBinding *__hidden this, bool)
#[doc(alias = "RBX::GfxBinding::updateEntity(bool)")]
#[doc(alias = "__ZN3RBX10GfxBinding12updateEntityEb")]
// IDA 0xc18ea8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c18ea8() {
}

// 0xc1d970 — __ZN3RBX9LightGrid24lightingUpdateChunkLocalERNS_14LightGridChunkEPN4Ogre14GfxSpatialHashE
// type: int __fastcall(int, int, int, int, char *, FLog *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, char, int, int, double, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::LightGrid::lightingUpdateChunkLocal(RBX::LightGridChunk &,Ogre::GfxSpatialHash *)")]
#[doc(alias = "__ZN3RBX9LightGrid24lightingUpdateChunkLocalERNS_14LightGridChunkEPN4Ogre14GfxSpatialHashE")]
// IDA 0xc1d970: 442 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c1d970() {
}

// 0xc1dea0 — __ZN3RBX9LightGrid17lightingGetLightsERKNS_7ExtentsEPN4Ogre14GfxSpatialHashE
// type: void __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char *, FLog *, int, int, int, int, int, int, int, char, int, int, double, char, int, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::LightGrid::lightingGetLights(RBX::Extents const&,Ogre::GfxSpatialHash *)")]
#[doc(alias = "__ZN3RBX9LightGrid17lightingGetLightsERKNS_7ExtentsEPN4Ogre14GfxSpatialHashE")]
// IDA 0xc1dea0: 305 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c1dea0() {
}

// 0xc350ac — __ZN3RBX12RenderEntityC1EPNS_10RenderNodeEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrEhh
// type: _DWORD __fastcall(RBX::RenderEntity *__hidden this, RBX::RenderNode *, Ogre::VertexData *, Ogre::IndexData *, const Ogre::MaterialPtr *, unsigned __int8, unsigned __int8)
#[doc(alias = "RBX::RenderEntity::RenderEntity(RBX::RenderNode *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,unsigned char,unsigned char)")]
#[doc(alias = "__ZN3RBX12RenderEntityC1EPNS_10RenderNodeEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrEhh")]
// IDA 0xc350ac: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c350ac() {
}

// 0xc350c8 — __ZN3RBX12RenderEntityC2EPNS_10RenderNodeEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrEhh
// type: _DWORD __fastcall(RBX::RenderEntity *__hidden this, RBX::RenderNode *, Ogre::VertexData *, Ogre::IndexData *, struct _Unwind_Exception *lpuexcpt, unsigned __int8, unsigned __int8)
#[doc(alias = "RBX::RenderEntity::RenderEntity(RBX::RenderNode *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,unsigned char,unsigned char)")]
#[doc(alias = "__ZN3RBX12RenderEntityC2EPNS_10RenderNodeEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrEhh")]
// IDA 0xc350c8: 328 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c350c8() {
}

// 0xc35418 — __ZN3RBX12RenderEntityD0Ev
// type: void __fastcall(RBX::RenderEntity *__hidden this)
#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
#[doc(alias = "__ZN3RBX12RenderEntityD0Ev")]
// IDA 0xc35418: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c35418() {
}

// 0xc354b8 — __ZN3RBX12RenderEntityD1Ev
// type: void __fastcall(RBX::RenderEntity *__hidden this)
#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
#[doc(alias = "__ZN3RBX12RenderEntityD1Ev")]
// IDA 0xc354b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c354b8() {
}

// 0xc354bc — __ZN3RBX12RenderEntityD2Ev
// type: void __fastcall(RBX::RenderEntity *__hidden this)
#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
#[doc(alias = "__ZN3RBX12RenderEntityD2Ev")]
// IDA 0xc354bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c354bc() {
}
