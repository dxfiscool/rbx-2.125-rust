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
pub fn stub_bb6e18() -> ! {
    todo!("0xbb6e18 Ogre::VertexStreamer::init(Ogre::VisualEngine *,Ogre::SceneManager *,Ogre::RenderWindow *,bool)")
}

// 0xbb79f0 — __ZN4Ogre14VertexStreamer12prepareChunkERKNS_10TexturePtrENS_15RenderOperation13OperationTypeENS0_15CoordinateSpaceENS0_10VextexTypeEbb
#[doc(alias = "Ogre::VertexStreamer::prepareChunk(Ogre::TexturePtr const&,Ogre::RenderOperation::OperationType,Ogre::VertexStreamer::CoordinateSpace,Ogre::VertexStreamer::VextexType,bool,bool)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer12prepareChunkERKNS_10TexturePtrENS_15RenderOperation13OperationTypeENS0_15CoordinateSpaceENS0_10VextexTypeEbb")]
pub fn stub_bb79f0() -> ! {
    todo!("0xbb79f0 Ogre::VertexStreamer::prepareChunk(Ogre::TexturePtr const&,Ogre::RenderOperation::OperationType,Ogre::VertexStreamer::CoordinateSpace,Ogre::VertexStreamer::VextexType,bool,bool)")
}

// 0xbb9540 — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE5setupEPNS_12RenderSystemE
// type: int(void)
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::setup(Ogre::RenderSystem *)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE5setupEPNS_12RenderSystemE")]
pub fn stub_bb9540() -> ! {
    todo!("0xbb9540 Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::setup(Ogre::RenderSystem *)")
}

// 0xbb97fc — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE5setupEPNS_12RenderSystemE
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::setup(Ogre::RenderSystem *)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE5setupEPNS_12RenderSystemE")]
pub fn stub_bb97fc() -> ! {
    todo!("0xbb97fc Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::setup(Ogre::RenderSystem *)")
}

// 0xbc8ebc — __ZN4Ogre15RbxSceneManager22renderQueueGroupSolidsEhNS_26QueuedRenderableCollection16OrganisationModeEb
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, char, char, char, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneManager::renderQueueGroupSolids(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode,bool)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager22renderQueueGroupSolidsEhNS_26QueuedRenderableCollection16OrganisationModeEb")]
pub fn stub_bc8ebc() -> ! {
    todo!("0xbc8ebc Ogre::RbxSceneManager::renderQueueGroupSolids(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode,bool)")
}

// 0xbc9490 — __ZN4Ogre15RbxSceneManager28renderQueueGroupTransparentsEhNS_26QueuedRenderableCollection16OrganisationModeE
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneManager::renderQueueGroupTransparents(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager28renderQueueGroupTransparentsEhNS_26QueuedRenderableCollection16OrganisationModeE")]
pub fn stub_bc9490() -> ! {
    todo!("0xbc9490 Ogre::RbxSceneManager::renderQueueGroupTransparents(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode)")
}

// 0xbc98e0 — __ZN4Ogre15RbxSceneManager18renderSingleObjectEPNS_10RenderableEPKNS_4PassEbbPKNS_12HashedVectorIPNS_5LightEEE
#[doc(alias = "Ogre::RbxSceneManager::renderSingleObject(Ogre::Renderable *,Ogre::Pass const*,bool,bool,Ogre::HashedVector<Ogre::Light *> const*)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager18renderSingleObjectEPNS_10RenderableEPKNS_4PassEbbPKNS_12HashedVectorIPNS_5LightEEE")]
pub fn stub_bc98e0() -> ! {
    todo!("0xbc98e0 Ogre::RbxSceneManager::renderSingleObject(Ogre::Renderable *,Ogre::Pass const*,bool,bool,Ogre::HashedVector<Ogre::Light *> const*)")
}

// 0xbc9bd0 — __ZN4Ogre15RbxSceneManager29updateRenderQueueSplitOptionsEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::updateRenderQueueSplitOptions(void)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager29updateRenderQueueSplitOptionsEv")]
pub fn stub_bc9bd0() -> ! {
    todo!("0xbc9bd0 Ogre::RbxSceneManager::updateRenderQueueSplitOptions(void)")
}

// 0xbc9c0c — __ZN4Ogre15RbxSceneManager23_queueSkiesForRenderingEPNS_6CameraE
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, Ogre::Camera *)
#[doc(alias = "Ogre::RbxSceneManager::_queueSkiesForRendering(Ogre::Camera *)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager23_queueSkiesForRenderingEPNS_6CameraE")]
pub fn stub_bc9c0c() -> ! {
    todo!("0xbc9c0c Ogre::RbxSceneManager::_queueSkiesForRendering(Ogre::Camera *)")
}

// 0xbca050 — __ZN4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE
#[doc(alias = "Ogre::RbxSceneManager::preRenderTargetUpdate(Ogre::RenderTargetEvent const&)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE")]
pub fn stub_bca050() -> ! {
    todo!("0xbca050 Ogre::RbxSceneManager::preRenderTargetUpdate(Ogre::RenderTargetEvent const&)")
}

// 0xbca17c — __ZThn17800_N4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE
#[doc = "`non-virtual thunk to'Ogre::RbxSceneManager::preRenderTargetUpdate(Ogre::RenderTargetEvent const&)"]
#[doc(alias = "__ZThn17800_N4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE")]
pub fn stub_bca17c() -> ! {
    todo!("0xbca17c `non-virtual thunk to'Ogre::RbxSceneManager::preRenderTargetUpdate(Ogre::RenderTargetEvent const&)")
}

// 0xbca188 — __ZN4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE
#[doc(alias = "Ogre::RbxSceneManager::postRenderTargetUpdate(Ogre::RenderTargetEvent const&)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE")]
pub fn stub_bca188() -> ! {
    todo!("0xbca188 Ogre::RbxSceneManager::postRenderTargetUpdate(Ogre::RenderTargetEvent const&)")
}

// 0xbca240 — __ZThn17800_N4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE
#[doc = "`non-virtual thunk to'Ogre::RbxSceneManager::postRenderTargetUpdate(Ogre::RenderTargetEvent const&)"]
#[doc(alias = "__ZThn17800_N4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE")]
pub fn stub_bca240() -> ! {
    todo!("0xbca240 `non-virtual thunk to'Ogre::RbxSceneManager::postRenderTargetUpdate(Ogre::RenderTargetEvent const&)")
}

// 0xbca8d0 — __ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE7reserveEm
// type: int(void)
#[doc(alias = "std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE7reserveEm")]
pub fn stub_bca8d0() -> ! {
    todo!("0xbca8d0 std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::reserve(unsigned long)")
}

// 0xbca94c — __ZN4Ogre20RenderTargetListener17preViewportUpdateERKNS_25RenderTargetViewportEventE
#[doc(alias = "Ogre::RenderTargetListener::preViewportUpdate(Ogre::RenderTargetViewportEvent const&)")]
#[doc(alias = "__ZN4Ogre20RenderTargetListener17preViewportUpdateERKNS_25RenderTargetViewportEventE")]
pub fn stub_bca94c() -> ! {
    todo!("0xbca94c Ogre::RenderTargetListener::preViewportUpdate(Ogre::RenderTargetViewportEvent const&)")
}

// 0xbca950 — __ZN4Ogre20RenderTargetListener18postViewportUpdateERKNS_25RenderTargetViewportEventE
#[doc(alias = "Ogre::RenderTargetListener::postViewportUpdate(Ogre::RenderTargetViewportEvent const&)")]
#[doc(alias = "__ZN4Ogre20RenderTargetListener18postViewportUpdateERKNS_25RenderTargetViewportEventE")]
pub fn stub_bca950() -> ! {
    todo!("0xbca950 Ogre::RenderTargetListener::postViewportUpdate(Ogre::RenderTargetViewportEvent const&)")
}

// 0xbcaa4c — __ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
#[doc(alias = "std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ShadowRenderable *,bool>*,std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>>,std::pair<Ogre::ShadowRenderable *,bool> const&)")]
#[doc(alias = "__ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_")]
pub fn stub_bcaa4c() -> ! {
    todo!("0xbcaa4c std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ShadowRenderable *,bool>*,std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>>,std::pair<Ogre::ShadowRenderable *,bool> const&)")
}

// 0xbcb4f0 — __ZN4Ogre12RbxSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb
// type: _DWORD __fastcall(Ogre::RbxSceneNode *__hidden this, Ogre::Camera *, Ogre::RenderQueue *, Ogre::VisibleObjectsBoundsInfo *, bool, bool, bool)
#[doc(alias = "Ogre::RbxSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")]
#[doc(alias = "__ZN4Ogre12RbxSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb")]
pub fn stub_bcb4f0() -> ! {
    todo!("0xbcb4f0 Ogre::RbxSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")
}

// 0xbcb6c0 — __ZN4Ogre12RbxSceneNode22setRenderableTransformERNS_10RenderableERKNS_7Matrix4E
// type: _DWORD __fastcall(Ogre::RbxSceneNode *__hidden this, Ogre::Renderable *, const Ogre::Matrix4 *)
#[doc(alias = "Ogre::RbxSceneNode::setRenderableTransform(Ogre::Renderable &,Ogre::Matrix4 const&)")]
#[doc(alias = "__ZN4Ogre12RbxSceneNode22setRenderableTransformERNS_10RenderableERKNS_7Matrix4E")]
pub fn stub_bcb6c0() -> ! {
    todo!("0xbcb6c0 Ogre::RbxSceneNode::setRenderableTransform(Ogre::Renderable &,Ogre::Matrix4 const&)")
}

// 0xbcb75c — __ZNK4Ogre12RbxSceneNode22getRenderableTransformEPKNS_10RenderableE
// type: _DWORD __fastcall(Ogre::RbxSceneNode *__hidden this, const Ogre::Renderable *)
#[doc(alias = "Ogre::RbxSceneNode::getRenderableTransform(Ogre::Renderable const*)const")]
#[doc(alias = "__ZNK4Ogre12RbxSceneNode22getRenderableTransformEPKNS_10RenderableE")]
pub fn stub_bcb75c() -> ! {
    todo!("0xbcb75c Ogre::RbxSceneNode::getRenderableTransform(Ogre::Renderable const*)const")
}

// 0xbcb91c — __ZNK4Ogre12RbxSceneNode25renderableTransformExistsEPKNS_10RenderableE
#[doc(alias = "Ogre::RbxSceneNode::renderableTransformExists(Ogre::Renderable const*)const")]
#[doc(alias = "__ZNK4Ogre12RbxSceneNode25renderableTransformExistsEPKNS_10RenderableE")]
pub fn stub_bcb91c() -> ! {
    todo!("0xbcb91c Ogre::RbxSceneNode::renderableTransformExists(Ogre::Renderable const*)const")
}

// 0xbcbb40 — __ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_bcbb40() -> ! {
    todo!("0xbcbb40 std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>> *)")
}

// 0xbcbb68 — __ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_insert_unique(std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_bcbb68() -> ! {
    todo!("0xbcbb68 std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_insert_unique(std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *> const&)")
}

// 0xbd3c88 — __ZN4Ogre6RbxSky17updateRenderQueueEPNS_6CameraEPNS_11RenderQueueENS_18RenderQueueGroupIDE
// type: int __fastcall(int, Ogre::Camera *this)
#[doc(alias = "Ogre::RbxSky::updateRenderQueue(Ogre::Camera *,Ogre::RenderQueue *,Ogre::RenderQueueGroupID)")]
#[doc(alias = "__ZN4Ogre6RbxSky17updateRenderQueueEPNS_6CameraEPNS_11RenderQueueENS_18RenderQueueGroupIDE")]
pub fn stub_bd3c88() -> ! {
    todo!("0xbd3c88 Ogre::RbxSky::updateRenderQueue(Ogre::Camera *,Ogre::RenderQueue *,Ogre::RenderQueueGroupID)")
}

// 0xbd4ed0 — __ZN4Ogre12RbxSubEntity18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::RbxSubEntity::getRenderOperation(Ogre::RenderOperation &)")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity18getRenderOperationERNS_15RenderOperationE")]
pub fn stub_bd4ed0() -> ! {
    todo!("0xbd4ed0 Ogre::RbxSubEntity::getRenderOperation(Ogre::RenderOperation &)")
}

// 0xbd55cc — __ZN4Ogre12RbxSubEntity33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm
// type: int __fastcall(int, int, int, int, int, int, float, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::RbxSubEntity::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm")]
pub fn stub_bd55cc() -> ! {
    todo!("0xbd55cc Ogre::RbxSubEntity::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")
}

// 0xbd59e8 — __ZThn264_N4Ogre12RbxSubEntity33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm
// type: int __fastcall(int, int, int, int, int, int, float, int)
#[doc = "`non-virtual thunk to'Ogre::RbxSubEntity::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)"]
#[doc(alias = "__ZThn264_N4Ogre12RbxSubEntity33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm")]
pub fn stub_bd59e8() -> ! {
    todo!("0xbd59e8 `non-virtual thunk to'Ogre::RbxSubEntity::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")
}

// 0xbd5cc4 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableC2EPNS_6EntityEPNS_28HardwareIndexBufferSharedPtrEPKNS_10VertexDataEbPNS_9SubEntityEb
// type: _DWORD __fastcall(Ogre::RbxSubEntity::RbxSubEntityShadowRenderable *__hidden this, Ogre::Entity *, Ogre::HardwareIndexBufferSharedPtr *, const Ogre::VertexData *, bool, Ogre::SubEntity *, bool)
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::RbxSubEntityShadowRenderable(Ogre::Entity *,Ogre::HardwareIndexBufferSharedPtr *,Ogre::VertexData const*,bool,Ogre::SubEntity *,bool)")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableC2EPNS_6EntityEPNS_28HardwareIndexBufferSharedPtrEPKNS_10VertexDataEbPNS_9SubEntityEb")]
pub fn stub_bd5cc4() -> ! {
    todo!("0xbd5cc4 Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::RbxSubEntityShadowRenderable(Ogre::Entity *,Ogre::HardwareIndexBufferSharedPtr *,Ogre::VertexData const*,bool,Ogre::SubEntity *,bool)")
}

// 0xbd6240 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD0Ev
// type: void __fastcall(Ogre::RbxSubEntity::RbxSubEntityShadowRenderable *__hidden this)
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD0Ev")]
pub fn stub_bd6240() -> ! {
    todo!("0xbd6240 Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()")
}

// 0xbd62f4 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD1Ev
// type: void __fastcall(Ogre::RbxSubEntity::RbxSubEntityShadowRenderable *__hidden this)
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD1Ev")]
pub fn stub_bd62f4() -> ! {
    todo!("0xbd62f4 Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()")
}

// 0xbd62f8 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD2Ev
// type: void __fastcall(Ogre::RbxSubEntity::RbxSubEntityShadowRenderable *__hidden this)
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderableD2Ev")]
pub fn stub_bd62f8() -> ! {
    todo!("0xbd62f8 Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::~RbxSubEntityShadowRenderable()")
}

// 0xbd661c — __ZNK4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")]
#[doc(alias = "__ZNK4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable18getWorldTransformsEPNS_7Matrix4E")]
pub fn stub_bd661c() -> ! {
    todo!("0xbd661c Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")
}

// 0xbd6628 — __ZNK4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable9isVisibleEv
// type: _DWORD __fastcall(Ogre::RbxSubEntity::RbxSubEntityShadowRenderable *__hidden this)
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::isVisible(void)const")]
#[doc(alias = "__ZNK4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable9isVisibleEv")]
pub fn stub_bd6628() -> ! {
    todo!("0xbd6628 Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::isVisible(void)const")
}

// 0xbd6638 — __ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable17rebindIndexBufferERKNS_28HardwareIndexBufferSharedPtrE
// type: _DWORD __fastcall(Ogre::RbxSubEntity::RbxSubEntityShadowRenderable *__hidden this, const Ogre::HardwareIndexBufferSharedPtr *)
#[doc(alias = "Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)")]
#[doc(alias = "__ZN4Ogre12RbxSubEntity28RbxSubEntityShadowRenderable17rebindIndexBufferERKNS_28HardwareIndexBufferSharedPtrE")]
pub fn stub_bd6638() -> ! {
    todo!("0xbd6638 Ogre::RbxSubEntity::RbxSubEntityShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)")
}

// 0xbec65c — __ZN3RBX10ViewRbxGfx10printSceneEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::printScene(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx10printSceneEv")]
pub fn stub_bec65c() -> ! {
    todo!("0xbec65c RBX::ViewRbxGfx::printScene(void)")
}

// 0xbec7fc — __ZThn4_N3RBX10ViewRbxGfx10printSceneEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc = "`non-virtual thunk to'RBX::ViewRbxGfx::printScene(void)"]
#[doc(alias = "__ZThn4_N3RBX10ViewRbxGfx10printSceneEv")]
pub fn stub_bec7fc() -> ! {
    todo!("0xbec7fc `non-virtual thunk to'RBX::ViewRbxGfx::printScene(void)")
}

// 0xbec808 — __ZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricE
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, RBX::IMetric *)
#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricE")]
pub fn stub_bec808() -> ! {
    todo!("0xbec808 RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)")
}

// 0xbed5b8 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD1Ev
#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::~ProxyMetric()")]
#[doc(alias = "__ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD1Ev")]
pub fn stub_bed5b8() -> ! {
    todo!("0xbed5b8 RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::~ProxyMetric()")
}

// 0xbed5c0 — __ZN3RBX10ViewRbxGfx13renderPerformEd
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, double)
#[doc(alias = "RBX::ViewRbxGfx::renderPerform(double)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx13renderPerformEd")]
pub fn stub_bed5c0() -> ! {
    todo!("0xbed5c0 RBX::ViewRbxGfx::renderPerform(double)")
}

// 0xbee4c0 — __ZN3RBX10ViewRbxGfx20saveScreenshotToFileERSs
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, std::string *)
#[doc(alias = "RBX::ViewRbxGfx::saveScreenshotToFile(std::string &)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx20saveScreenshotToFileERSs")]
pub fn stub_bee4c0() -> ! {
    todo!("0xbee4c0 RBX::ViewRbxGfx::saveScreenshotToFile(std::string &)")
}

// 0xbee96c — __ZN3RBX10ViewRbxGfx6updateEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::update(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx6updateEv")]
pub fn stub_bee96c() -> ! {
    todo!("0xbee96c RBX::ViewRbxGfx::update(void)")
}

// 0xbeea8c — __ZN3RBX10ViewRbxGfx8buildGuiEb
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, bool)
#[doc(alias = "RBX::ViewRbxGfx::buildGui(bool)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx8buildGuiEb")]
pub fn stub_beea8c() -> ! {
    todo!("0xbeea8c RBX::ViewRbxGfx::buildGui(bool)")
}

// 0xbeeaac — __ZN3RBX10ViewRbxGfx14getRenderStatsEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::getRenderStats(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx14getRenderStatsEv")]
pub fn stub_beeaac() -> ! {
    todo!("0xbeeaac RBX::ViewRbxGfx::getRenderStats(void)")
}

// 0xbeeab8 — __ZN3RBX10ViewRbxGfx11renderThumbEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::renderThumb(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx11renderThumbEv")]
pub fn stub_beeab8() -> ! {
    todo!("0xbeeab8 RBX::ViewRbxGfx::renderThumb(void)")
}

// 0xbeef64 — __ZN3RBX10ViewRbxGfx13writeRTToFileERKSs
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, const std::string *)
#[doc(alias = "RBX::ViewRbxGfx::writeRTToFile(std::string const&)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx13writeRTToFileERKSs")]
pub fn stub_beef64() -> ! {
    todo!("0xbeef64 RBX::ViewRbxGfx::writeRTToFile(std::string const&)")
}

// 0xbeefb0 — __ZN3RBX10ViewRbxGfx15writeRTToBufferEPhiii
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, unsigned __int8 *, int, int, int)
#[doc(alias = "RBX::ViewRbxGfx::writeRTToBuffer(unsigned char *,int,int,int)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx15writeRTToBufferEPhiii")]
pub fn stub_beefb0() -> ! {
    todo!("0xbeefb0 RBX::ViewRbxGfx::writeRTToBuffer(unsigned char *,int,int,int)")
}

// 0xbef0f8 — __ZN3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, std::string *this)
#[doc(alias = "RBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE")]
pub fn stub_bef0f8() -> ! {
    todo!("0xbef0f8 RBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")
}

// 0xbef138 — __ZThn8_N3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, std::string *this)
#[doc = "`non-virtual thunk to'RBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)"]
#[doc(alias = "__ZThn8_N3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE")]
pub fn stub_bef138() -> ! {
    todo!("0xbef138 `non-virtual thunk to'RBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")
}

// 0xbef174 — __ZN3RBX10ViewRbxGfx10startFrameEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::startFrame(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx10startFrameEv")]
pub fn stub_bef174() -> ! {
    todo!("0xbef174 RBX::ViewRbxGfx::startFrame(void)")
}

// 0xbef1f0 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::~ProxyMetric()")]
#[doc(alias = "__ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD0Ev")]
pub fn stub_bef1f0() -> ! {
    todo!("0xbef1f0 RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::~ProxyMetric()")
}

// 0xbef1f4 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric9getMetricERKSs
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::getMetric(std::string const&)const")]
#[doc(alias = "__ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric9getMetricERKSs")]
pub fn stub_bef1f4() -> ! {
    todo!("0xbef1f4 RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::getMetric(std::string const&)const")
}

// 0xbef230 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric14getMetricValueERKSs
// type: int __fastcall(int, std::string *this)
#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::getMetricValue(std::string const&)const")]
#[doc(alias = "__ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric14getMetricValueERKSs")]
pub fn stub_bef230() -> ! {
    todo!("0xbef230 RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::getMetricValue(std::string const&)const")
}

// 0xbef270 — __ZZN3RBX21ViewRbxGfx_InitModuleEvEN17ViewRbxGfxFactory6CreateENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_
#[doc(alias = "RBX::ViewRbxGfx_InitModule(void)::ViewRbxGfxFactory::Create(RBX::CRenderSettings::GraphicsMode,RBX::OSContext *,RBX::CRenderSettings*)")]
#[doc(alias = "__ZZN3RBX21ViewRbxGfx_InitModuleEvEN17ViewRbxGfxFactory6CreateENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_")]
pub fn stub_bef270() -> ! {
    todo!("0xbef270 RBX::ViewRbxGfx_InitModule(void)::ViewRbxGfxFactory::Create(RBX::CRenderSettings::GraphicsMode,RBX::OSContext *,RBX::CRenderSettings*)")
}

// 0xbef8c0 — __ZN3RBX10ViewRbxGfx8throttleEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::throttle(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx8throttleEv")]
pub fn stub_bef8c0() -> ! {
    todo!("0xbef8c0 RBX::ViewRbxGfx::throttle(void)")
}

// 0xbef8cc — __ZN3RBX10ViewRbxGfx8getAdornEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "RBX::ViewRbxGfx::getAdorn(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx8getAdornEv")]
pub fn stub_bef8cc() -> ! {
    todo!("0xbef8cc RBX::ViewRbxGfx::getAdorn(void)")
}

// 0xbf0e94 — __ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")]
pub fn stub_bf0e94() -> ! {
    todo!("0xbf0e94 rbx::signals::signal<void ()(int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")
}

// 0xbf0ef0 — __ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")]
pub fn stub_bf0ef0() -> ! {
    todo!("0xbf0ef0 rbx::signals::signal<void ()(int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")
}

// 0xbf10f4 — __ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(int)>::call(int)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi")]
pub fn stub_bf10f4() -> ! {
    todo!("0xbf10f4 rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(int)>::call(int)")
}

// 0xbf110c — __ZThn4_N3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi
#[doc = "`non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(int)>::call(int)"]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi")]
pub fn stub_bf110c() -> ! {
    todo!("0xbf110c `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(int)>::call(int)")
}

// 0xbf18ac — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")]
pub fn stub_bf18ac() -> ! {
    todo!("0xbf18ac rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>>::~callable_slot()")
}

// 0xbf1908 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")]
pub fn stub_bf1908() -> ! {
    todo!("0xbf1908 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>>::~callable_slot()")
}

// 0xbf1a14 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
pub fn stub_bf1a14() -> ! {
    todo!("0xbf1a14 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>,0,void ()(void)>::call(void)")
}

// 0xbf1a2c — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// type: int __fastcall(_DWORD *)
#[doc = "`non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>,0,void ()(void)>::call(void)"]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
pub fn stub_bf1a2c() -> ! {
    todo!("0xbf1a2c `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>,0,void ()(void)>::call(void)")
}

// 0xbf1afc — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")]
pub fn stub_bf1afc() -> ! {
    todo!("0xbf1afc rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")
}

// 0xbf1b58 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")]
pub fn stub_bf1b58() -> ! {
    todo!("0xbf1b58 rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")
}

// 0xbf1d5c — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")]
pub fn stub_bf1d5c() -> ! {
    todo!("0xbf1d5c rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")
}

// 0xbf1d74 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
#[doc = "`non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)"]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")]
pub fn stub_bf1d74() -> ! {
    todo!("0xbf1d74 `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")
}

// 0xbfb48c — __ZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb
// type: _DWORD __fastcall(Ogre::RbxSpatialHashedSceneNode *__hidden this, Ogre::Camera *, Ogre::RenderQueue *, Ogre::VisibleObjectsBoundsInfo *, bool, bool, bool)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")]
#[doc(alias = "__ZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb")]
pub fn stub_bfb48c() -> ! {
    todo!("0xbfb48c Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")
}

// 0xbfb568 — __ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter10IntersectsERKN3RBX7ExtentsE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)::NodeVisiter::Intersects(RBX::Extents const&)")]
#[doc(alias = "__ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter10IntersectsERKN3RBX7ExtentsE")]
pub fn stub_bfb568() -> ! {
    todo!("0xbfb568 Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)::NodeVisiter::Intersects(RBX::Extents const&)")
}

// 0xbfb69c — __ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter8DistanceERKN3RBX7ExtentsE
// type: int __fastcall(int, RBX::Extents *this)
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)::NodeVisiter::Distance(RBX::Extents const&)")]
#[doc(alias = "__ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter8DistanceERKN3RBX7ExtentsE")]
pub fn stub_bfb69c() -> ! {
    todo!("0xbfb69c Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)::NodeVisiter::Distance(RBX::Extents const&)")
}

// 0xbfb6a8 — __ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter11onPrimitiveEPNS_20RbxCullableSceneNodeEN3RBX15IntersectResultEf
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)::NodeVisiter::onPrimitive(Ogre::RbxCullableSceneNode *,RBX::IntersectResult,float)")]
#[doc(alias = "__ZZN4Ogre25RbxSpatialHashedSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbEN11NodeVisiter11onPrimitiveEPNS_20RbxCullableSceneNodeEN3RBX15IntersectResultEf")]
pub fn stub_bfb6a8() -> ! {
    todo!("0xbfb6a8 Ogre::RbxSpatialHashedSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)::NodeVisiter::onPrimitive(Ogre::RbxCullableSceneNode *,RBX::IntersectResult,float)")
}

// 0xc02d80 — __ZN19ResourceGroupHelper31UpdateMaterialRenderableVisitor5visitEPN4Ogre10RenderableEtbPNS1_3AnyE
// type: _DWORD __fastcall(ResourceGroupHelper::UpdateMaterialRenderableVisitor *__hidden this, Ogre::Renderable *, unsigned __int16, bool, Ogre::Any *)
#[doc(alias = "ResourceGroupHelper::UpdateMaterialRenderableVisitor::visit(Ogre::Renderable *,unsigned short,bool,Ogre::Any *)")]
#[doc(alias = "__ZN19ResourceGroupHelper31UpdateMaterialRenderableVisitor5visitEPN4Ogre10RenderableEtbPNS1_3AnyE")]
pub fn stub_c02d80() -> ! {
    todo!("0xc02d80 ResourceGroupHelper::UpdateMaterialRenderableVisitor::visit(Ogre::Renderable *,unsigned short,bool,Ogre::Any *)")
}

// 0xc03db8 — __ZL28updateMaterialsOnRenderNodesPKN4Ogre9SceneNodeE
// type: _DWORD __fastcall(const Ogre::SceneNode *)
#[doc(alias = "updateMaterialsOnRenderNodes(Ogre::SceneNode const*)")]
#[doc(alias = "__ZL28updateMaterialsOnRenderNodesPKN4Ogre9SceneNodeE")]
pub fn stub_c03db8() -> ! {
    todo!("0xc03db8 updateMaterialsOnRenderNodes(Ogre::SceneNode const*)")
}

// 0xc04658 — __ZN19ResourceGroupHelper31visitRecursivelyRenderablesFromEPN4Ogre16OverlayContainerERNS0_10Renderable7VisitorEb
#[doc(alias = "ResourceGroupHelper::visitRecursivelyRenderablesFrom(Ogre::OverlayContainer *,Ogre::Renderable::Visitor &,bool)")]
#[doc(alias = "__ZN19ResourceGroupHelper31visitRecursivelyRenderablesFromEPN4Ogre16OverlayContainerERNS0_10Renderable7VisitorEb")]
pub fn stub_c04658() -> ! {
    todo!("0xc04658 ResourceGroupHelper::visitRecursivelyRenderablesFrom(Ogre::OverlayContainer *,Ogre::Renderable::Visitor &,bool)")
}

// 0xc08ddc — __ZN3RBX11MegaCluster19createSolidGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::MegaCluster::createSolidGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)")]
#[doc(alias = "__ZN3RBX11MegaCluster19createSolidGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj")]
pub fn stub_c08ddc() -> ! {
    todo!("0xc08ddc RBX::MegaCluster::createSolidGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)")
}

// 0xc08fec — __ZN3RBX11MegaCluster19createWaterGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::MegaCluster::createWaterGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)")]
#[doc(alias = "__ZN3RBX11MegaCluster19createWaterGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj")]
pub fn stub_c08fec() -> ! {
    todo!("0xc08fec RBX::MegaCluster::createWaterGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)")
}

// 0xc0923c — __ZN3RBX11MegaCluster14createGeometryEPNS_10RenderNodeERKN4Ogre29HardwareVertexBufferSharedPtrEPKch
// type: _DWORD __fastcall(RBX::MegaCluster *__hidden this, RBX::RenderNode *, const Ogre::HardwareVertexBufferSharedPtr *, const char *, unsigned __int8)
#[doc(alias = "RBX::MegaCluster::createGeometry(RBX::RenderNode *,Ogre::HardwareVertexBufferSharedPtr const&,char const*,unsigned char)")]
#[doc(alias = "__ZN3RBX11MegaCluster14createGeometryEPNS_10RenderNodeERKN4Ogre29HardwareVertexBufferSharedPtrEPKch")]
pub fn stub_c0923c() -> ! {
    todo!("0xc0923c RBX::MegaCluster::createGeometry(RBX::RenderNode *,Ogre::HardwareVertexBufferSharedPtr const&,char const*,unsigned char)")
}

// 0xc0a4ec — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_20SolidTerrainRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_20SolidTerrainRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")]
pub fn stub_c0a4ec() -> ! {
    todo!("0xc0a4ec RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc0acec — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_17WaterFaceRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_17WaterFaceRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")]
pub fn stub_c0acec() -> ! {
    todo!("0xc0acec RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc0b430 — __ZN3RBX10GfxBinding16invalidateEntityEv
// type: _DWORD __fastcall(RBX::GfxBinding *__hidden this)
#[doc(alias = "RBX::GfxBinding::invalidateEntity(void)")]
#[doc(alias = "__ZN3RBX10GfxBinding16invalidateEntityEv")]
pub fn stub_c0b430() -> ! {
    todo!("0xc0b430 RBX::GfxBinding::invalidateEntity(void)")
}

// 0xc0b434 — __ZN3RBX10GfxBinding24onCoordinateFrameChangedEv
// type: _DWORD __fastcall(RBX::GfxBinding *__hidden this)
#[doc(alias = "RBX::GfxBinding::onCoordinateFrameChanged(void)")]
#[doc(alias = "__ZN3RBX10GfxBinding24onCoordinateFrameChangedEv")]
pub fn stub_c0b434() -> ! {
    todo!("0xc0b434 RBX::GfxBinding::onCoordinateFrameChanged(void)")
}

// 0xc0b438 — __ZN3RBX7GfxPart21updateCoordinateFrameEb
// type: _DWORD __fastcall(RBX::GfxPart *__hidden this, bool)
#[doc(alias = "RBX::GfxPart::updateCoordinateFrame(bool)")]
#[doc(alias = "__ZN3RBX7GfxPart21updateCoordinateFrameEb")]
pub fn stub_c0b438() -> ! {
    todo!("0xc0b438 RBX::GfxPart::updateCoordinateFrame(bool)")
}

// 0xc0b43c — __ZN3RBX7GfxPart19getFastFuzzyExtentsEv
// type: _DWORD __fastcall(RBX::GfxPart *__hidden this)
#[doc(alias = "RBX::GfxPart::getFastFuzzyExtents(void)")]
#[doc(alias = "__ZN3RBX7GfxPart19getFastFuzzyExtentsEv")]
pub fn stub_c0b43c() -> ! {
    todo!("0xc0b43c RBX::GfxPart::getFastFuzzyExtents(void)")
}

// 0xc0b4cc — __ZN3RBX7GfxPart12getPartCountEv
// type: _DWORD __fastcall(RBX::GfxPart *__hidden this)
#[doc(alias = "RBX::GfxPart::getPartCount(void)")]
#[doc(alias = "__ZN3RBX7GfxPart12getPartCountEv")]
pub fn stub_c0b4cc() -> ! {
    todo!("0xc0b4cc RBX::GfxPart::getPartCount(void)")
}

// 0xc0b4d4 — __ZN3RBX7GfxPart14onClumpChangedEv
// type: _DWORD __fastcall(RBX::GfxPart *__hidden this)
#[doc(alias = "RBX::GfxPart::onClumpChanged(void)")]
#[doc(alias = "__ZN3RBX7GfxPart14onClumpChangedEv")]
pub fn stub_c0b4d4() -> ! {
    todo!("0xc0b4d4 RBX::GfxPart::onClumpChanged(void)")
}

// 0xc0b4d8 — __ZNK3RBX20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE8internalERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionE
#[doc(alias = "RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
#[doc(alias = "__ZNK3RBX20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE8internalERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionE")]
pub fn stub_c0b4d8() -> ! {
    todo!("0xc0b4d8 RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")
}

// 0xc0b66c — __ZN3RBX17WaterFaceRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX17WaterFaceRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE")]
pub fn stub_c0b66c() -> ! {
    todo!("0xc0b66c RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc0bf18 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")]
pub fn stub_c0bf18() -> ! {
    todo!("0xc0bf18 RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc0c648 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE")]
pub fn stub_c0c648() -> ! {
    todo!("0xc0c648 RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc0c904 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE9wedgeFaceERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE9wedgeFaceERKNS1_6RegionINS3_5ChunkEE8iteratorE")]
pub fn stub_c0c904() -> ! {
    todo!("0xc0c904 RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")
}

// 0xc0cd30 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE19detectWedgeOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE19detectWedgeOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorE")]
pub fn stub_c0cd30() -> ! {
    todo!("0xc0cd30 RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")
}

// 0xc0cf1c — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12wedgeUpEmptyERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12wedgeUpEmptyERKNS1_6RegionINS3_5ChunkEE8iteratorE")]
pub fn stub_c0cf1c() -> ! {
    todo!("0xc0cf1c RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")
}

// 0xc0d000 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE14detectOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE14detectOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE")]
pub fn stub_c0d000() -> ! {
    todo!("0xc0d000 RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc0d190 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12renderHelperENS1_4CellENS1_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS1_13FaceDirectionEh
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12renderHelperENS1_4CellENS1_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS1_13FaceDirectionEh")]
pub fn stub_c0d190() -> ! {
    todo!("0xc0d190 RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)")
}

// 0xc0d418 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
// type: int __fastcall(unsigned int *, _WORD *)
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")]
pub fn stub_c0d418() -> ! {
    todo!("0xc0d418 RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc0dbd8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED1Ev")]
pub fn stub_c0dbd8() -> ! {
    todo!("0xc0dbd8 boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")
}

// 0xc0dbdc — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED0Ev")]
pub fn stub_c0dbdc() -> ! {
    todo!("0xc0dbdc boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")
}

// 0xc0dbe0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE7disposeEv")]
pub fn stub_c0dbe0() -> ! {
    todo!("0xc0dbe0 boost::detail::sp_counted_impl_p<RBX::RenderNode>::dispose(void)")
}

// 0xc0dbf0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE11get_deleterERKSt9type_info")]
pub fn stub_c0dbf0() -> ! {
    todo!("0xc0dbf0 boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_deleter(std::type_info const&)")
}

// 0xc0dbf4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE19get_untyped_deleterEv")]
pub fn stub_c0dbf4() -> ! {
    todo!("0xc0dbf4 boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_untyped_deleter(void)")
}

// 0xc14f78 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_20SolidTerrainRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_20SolidTerrainRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")]
pub fn stub_c14f78() -> ! {
    todo!("0xc14f78 RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc15780 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE")]
pub fn stub_c15780() -> ! {
    todo!("0xc15780 RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc15a3c — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE9wedgeFaceERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE9wedgeFaceERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE")]
pub fn stub_c15a3c() -> ! {
    todo!("0xc15a3c RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")
}

// 0xc15e68 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE")]
pub fn stub_c15e68() -> ! {
    todo!("0xc15e68 RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")
}

// 0xc16054 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12wedgeUpEmptyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12wedgeUpEmptyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE")]
pub fn stub_c16054() -> ! {
    todo!("0xc16054 RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")
}

// 0xc16138 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE14detectOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE14detectOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE")]
pub fn stub_c16138() -> ! {
    todo!("0xc16138 RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc162c8 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12renderHelperENS_5Voxel4CellENS4_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS4_13FaceDirectionEh
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12renderHelperENS_5Voxel4CellENS4_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS4_13FaceDirectionEh")]
pub fn stub_c162c8() -> ! {
    todo!("0xc162c8 RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)")
}

// 0xc16550 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
// type: int __fastcall(unsigned int *, _WORD *)
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")]
pub fn stub_c16550() -> ! {
    todo!("0xc16550 RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc16d18 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_17WaterFaceRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_17WaterFaceRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")]
pub fn stub_c16d18() -> ! {
    todo!("0xc16d18 RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc17464 — __ZNK3RBX20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEE8internalERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionE
#[doc(alias = "RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
#[doc(alias = "__ZNK3RBX20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEE8internalERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionE")]
pub fn stub_c17464() -> ! {
    todo!("0xc17464 RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")
}

// 0xc175f8 — __ZN3RBX17WaterFaceRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX17WaterFaceRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE")]
pub fn stub_c175f8() -> ! {
    todo!("0xc175f8 RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc1799c — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")]
pub fn stub_c1799c() -> ! {
    todo!("0xc1799c RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc18ea8 — __ZN3RBX10GfxBinding12updateEntityEb
// type: _DWORD __fastcall(RBX::GfxBinding *__hidden this, bool)
#[doc(alias = "RBX::GfxBinding::updateEntity(bool)")]
#[doc(alias = "__ZN3RBX10GfxBinding12updateEntityEb")]
pub fn stub_c18ea8() -> ! {
    todo!("0xc18ea8 RBX::GfxBinding::updateEntity(bool)")
}

// 0xc1d970 — __ZN3RBX9LightGrid24lightingUpdateChunkLocalERNS_14LightGridChunkEPN4Ogre14GfxSpatialHashE
// type: int __fastcall(int, int, int, int, char *, FLog *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, char, int, int, double, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::LightGrid::lightingUpdateChunkLocal(RBX::LightGridChunk &,Ogre::GfxSpatialHash *)")]
#[doc(alias = "__ZN3RBX9LightGrid24lightingUpdateChunkLocalERNS_14LightGridChunkEPN4Ogre14GfxSpatialHashE")]
pub fn stub_c1d970() -> ! {
    todo!("0xc1d970 RBX::LightGrid::lightingUpdateChunkLocal(RBX::LightGridChunk &,Ogre::GfxSpatialHash *)")
}

// 0xc1dea0 — __ZN3RBX9LightGrid17lightingGetLightsERKNS_7ExtentsEPN4Ogre14GfxSpatialHashE
// type: void __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char *, FLog *, int, int, int, int, int, int, int, char, int, int, double, char, int, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::LightGrid::lightingGetLights(RBX::Extents const&,Ogre::GfxSpatialHash *)")]
#[doc(alias = "__ZN3RBX9LightGrid17lightingGetLightsERKNS_7ExtentsEPN4Ogre14GfxSpatialHashE")]
pub fn stub_c1dea0() -> ! {
    todo!("0xc1dea0 RBX::LightGrid::lightingGetLights(RBX::Extents const&,Ogre::GfxSpatialHash *)")
}

// 0xc350ac — __ZN3RBX12RenderEntityC1EPNS_10RenderNodeEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrEhh
// type: _DWORD __fastcall(RBX::RenderEntity *__hidden this, RBX::RenderNode *, Ogre::VertexData *, Ogre::IndexData *, const Ogre::MaterialPtr *, unsigned __int8, unsigned __int8)
#[doc(alias = "RBX::RenderEntity::RenderEntity(RBX::RenderNode *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,unsigned char,unsigned char)")]
#[doc(alias = "__ZN3RBX12RenderEntityC1EPNS_10RenderNodeEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrEhh")]
pub fn stub_c350ac() -> ! {
    todo!("0xc350ac RBX::RenderEntity::RenderEntity(RBX::RenderNode *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,unsigned char,unsigned char)")
}

// 0xc350c8 — __ZN3RBX12RenderEntityC2EPNS_10RenderNodeEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrEhh
// type: _DWORD __fastcall(RBX::RenderEntity *__hidden this, RBX::RenderNode *, Ogre::VertexData *, Ogre::IndexData *, struct _Unwind_Exception *lpuexcpt, unsigned __int8, unsigned __int8)
#[doc(alias = "RBX::RenderEntity::RenderEntity(RBX::RenderNode *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,unsigned char,unsigned char)")]
#[doc(alias = "__ZN3RBX12RenderEntityC2EPNS_10RenderNodeEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrEhh")]
pub fn stub_c350c8() -> ! {
    todo!("0xc350c8 RBX::RenderEntity::RenderEntity(RBX::RenderNode *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,unsigned char,unsigned char)")
}

// 0xc35418 — __ZN3RBX12RenderEntityD0Ev
// type: void __fastcall(RBX::RenderEntity *__hidden this)
#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
#[doc(alias = "__ZN3RBX12RenderEntityD0Ev")]
pub fn stub_c35418() -> ! {
    todo!("0xc35418 RBX::RenderEntity::~RenderEntity()")
}

// 0xc354b8 — __ZN3RBX12RenderEntityD1Ev
// type: void __fastcall(RBX::RenderEntity *__hidden this)
#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
#[doc(alias = "__ZN3RBX12RenderEntityD1Ev")]
pub fn stub_c354b8() -> ! {
    todo!("0xc354b8 RBX::RenderEntity::~RenderEntity()")
}

// 0xc354bc — __ZN3RBX12RenderEntityD2Ev
// type: void __fastcall(RBX::RenderEntity *__hidden this)
#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
#[doc(alias = "__ZN3RBX12RenderEntityD2Ev")]
pub fn stub_c354bc() -> ! {
    todo!("0xc354bc RBX::RenderEntity::~RenderEntity()")
}
