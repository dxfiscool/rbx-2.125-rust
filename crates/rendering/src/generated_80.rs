//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xe1e340..0xe27a70 (100 stubs, 9160 prior -> 9260 covered, 4073 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xe1e340 — __ZN4Ogre10Serializer20flipFromLittleEndianEPvmm
#[doc(alias = "Ogre::Serializer::flipFromLittleEndian(void *,unsigned long,unsigned long)")]
// was: Ogre::Serializer::flipFromLittleEndian(void *,unsigned long,unsigned long)
// IDA 0xe1e340: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e340() {
}

// 0xe1e35c — __ZN4Ogre10Serializer10flipEndianEPvmm
#[doc(alias = "Ogre::Serializer::flipEndian(void *,unsigned long,unsigned long)")]
// was: Ogre::Serializer::flipEndian(void *,unsigned long,unsigned long)
// IDA 0xe1e35c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e35c() {
}

// 0xe1e388 — __ZN4Ogre10Serializer10flipEndianEPvm
#[doc(alias = "Ogre::Serializer::flipEndian(void *,unsigned long)")]
// was: Ogre::Serializer::flipEndian(void *,unsigned long)
// IDA 0xe1e388: 16 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e388() {
}

// 0xe1e3e4 — __ZN4Ogre24DefaultShadowCameraSetupC1Ev
#[doc(alias = "Ogre::DefaultShadowCameraSetup::DefaultShadowCameraSetup(void)")]
// was: Ogre::DefaultShadowCameraSetup::DefaultShadowCameraSetup(void)
// IDA 0xe1e3e4: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e3e4() {
}

// 0xe1e3f4 — __ZN4Ogre24DefaultShadowCameraSetupD0Ev
#[doc(alias = "Ogre::DefaultShadowCameraSetup::~DefaultShadowCameraSetup()")]
// was: Ogre::DefaultShadowCameraSetup::~DefaultShadowCameraSetup()
// IDA 0xe1e3f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1e3f4() {
}

// 0xe1e480 — __ZN4Ogre24DefaultShadowCameraSetupD1Ev
#[doc(alias = "Ogre::DefaultShadowCameraSetup::~DefaultShadowCameraSetup()")]
// was: Ogre::DefaultShadowCameraSetup::~DefaultShadowCameraSetup()
// IDA 0xe1e480: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1e480() {
}

// 0xe1e484 — __ZNK4Ogre24DefaultShadowCameraSetup15getShadowCameraEPKNS_12SceneManagerEPKNS_6CameraEPKNS_8ViewportEPKNS_5LightEPS4_m
#[doc(alias = "Ogre::DefaultShadowCameraSetup::getShadowCamera(Ogre::SceneManager const*,Ogre::Camera const*,Ogre::Viewport const*,Ogre::Light const*,Ogre::Camera*,unsigned long)const")]
// was: Ogre::DefaultShadowCameraSetup::getShadowCamera(Ogre::SceneManager const*,Ogre::Camera const*,Ogre::Viewport const*,Ogre::Light const*,Ogre::Camera*,unsigned long)const
// IDA 0xe1e484: 567 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e484() {
}

// 0xe1ec84 — __ZNK4Ogre16ShadowRenderable9getLightsEv
#[doc(alias = "Ogre::ShadowRenderable::getLights(void)const")]
// was: Ogre::ShadowRenderable::getLights(void)const
// IDA 0xe1ec84: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1ec84() {
}

// 0xe1ecf0 — __ZN4Ogre12ShadowCaster25updateEdgeListLightFacingEPNS_8EdgeDataERKNS_7Vector4E
#[doc(alias = "Ogre::ShadowCaster::updateEdgeListLightFacing(Ogre::EdgeData *,Ogre::Vector4 const&)")]
// was: Ogre::ShadowCaster::updateEdgeListLightFacing(Ogre::EdgeData *,Ogre::Vector4 const&)
// IDA 0xe1ecf0: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1ecf0() {
}

// 0xe1ed00 — __ZN4Ogre12ShadowCaster20generateShadowVolumeEPNS_8EdgeDataERKNS_28HardwareIndexBufferSharedPtrEPKNS_5LightERSt6vectorIPNS_16ShadowRenderableENS_12STLAllocatorISB_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEm
#[doc(alias = "Ogre::ShadowCaster::generateShadowVolume(Ogre::EdgeData *,Ogre::HardwareIndexBufferSharedPtr const&,Ogre::Light const*,std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &,unsigned long)")]
// was: Ogre::ShadowCaster::generateShadowVolume(Ogre::EdgeData *,Ogre::HardwareIndexBufferSharedPtr const&,Ogre::Light const*,std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &,unsigned long)
// IDA 0xe1ed00: 566 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1ed00() {
}

// 0xe1f48c — __ZN4Ogre12ShadowCaster15extrudeVerticesERKNS_29HardwareVertexBufferSharedPtrEmRKNS_7Vector4Ef
#[doc(alias = "Ogre::ShadowCaster::extrudeVertices(Ogre::HardwareVertexBufferSharedPtr const&,unsigned long,Ogre::Vector4 const&,float)")]
// was: Ogre::ShadowCaster::extrudeVertices(Ogre::HardwareVertexBufferSharedPtr const&,unsigned long,Ogre::Vector4 const&,float)
// IDA 0xe1f48c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1f48c() {
}

// 0xe1f4e4 — __ZNK4Ogre12ShadowCaster13extrudeBoundsERNS_14AxisAlignedBoxERKNS_7Vector4Ef
#[doc(alias = "Ogre::ShadowCaster::extrudeBounds(Ogre::AxisAlignedBox &,Ogre::Vector4 const&,float)const")]
// was: Ogre::ShadowCaster::extrudeBounds(Ogre::AxisAlignedBox &,Ogre::Vector4 const&,float)const
// IDA 0xe1f4e4: 471 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1f4e4() {
}

// 0xe1fbb0 — __ZNK4Ogre12ShadowCaster20getExtrusionDistanceERKNS_7Vector3EPKNS_5LightE
#[doc(alias = "Ogre::ShadowCaster::getExtrusionDistance(Ogre::Vector3 const&,Ogre::Light const*)const")]
// was: Ogre::ShadowCaster::getExtrusionDistance(Ogre::Vector3 const&,Ogre::Light const*)const
// IDA 0xe1fbb0: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1fbb0() {
}

// 0xe1fc4c — __ZN4Ogre20ShadowTextureManager12getSingletonEv
#[doc(alias = "Ogre::ShadowTextureManager::getSingleton(void)")]
// was: Ogre::ShadowTextureManager::getSingleton(void)
// IDA 0xe1fc4c: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1fc4c() {
}

// 0xe1fc5c — __ZN4Ogre20ShadowTextureManagerC1Ev
#[doc(alias = "Ogre::ShadowTextureManager::ShadowTextureManager(void)")]
// was: Ogre::ShadowTextureManager::ShadowTextureManager(void)
// IDA 0xe1fc5c: 23 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1fc5c() {
}

// 0xe1fca0 — __ZN4Ogre20ShadowTextureManagerD0Ev
#[doc(alias = "Ogre::ShadowTextureManager::~ShadowTextureManager()")]
// was: Ogre::ShadowTextureManager::~ShadowTextureManager()
// IDA 0xe1fca0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1fca0() {
}

// 0xe1fd30 — __ZN4Ogre20ShadowTextureManagerD1Ev
#[doc(alias = "Ogre::ShadowTextureManager::~ShadowTextureManager()")]
// was: Ogre::ShadowTextureManager::~ShadowTextureManager()
// IDA 0xe1fd30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1fd30() {
}

// 0xe1fd3c — __ZN4Ogre20ShadowTextureManagerD2Ev
#[doc(alias = "Ogre::ShadowTextureManager::~ShadowTextureManager()")]
// was: Ogre::ShadowTextureManager::~ShadowTextureManager()
// IDA 0xe1fd3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1fd3c() {
}

// 0xe1fec0 — __ZN4Ogre20ShadowTextureManager17getShadowTexturesERKSt6vectorINS_19ShadowTextureConfigENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEERS1_INS_10TexturePtrENS3_ISB_S6_EEE
#[doc(alias = "Ogre::ShadowTextureManager::getShadowTextures(std::vector<Ogre::ShadowTextureConfig,Ogre::STLAllocator<Ogre::ShadowTextureConfig,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,std::vector&<Ogre::TexturePtr,Ogre::STLAllocator<std::vector&,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)")]
// was: Ogre::ShadowTextureManager::getShadowTextures(std::vector<Ogre::ShadowTextureConfig,Ogre::STLAllocator<Ogre::ShadowTextureConfig,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,std::vector&<Ogre::TexturePtr,Ogre::STLAllocator<std::vector&,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)
// IDA 0xe1fec0: 644 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1fec0() {
}

// 0xe20528 — __ZN4Ogre20ShadowTextureManager20getNullShadowTextureENS_11PixelFormatE
#[doc(alias = "Ogre::ShadowTextureManager::getNullShadowTexture(Ogre::PixelFormat)")]
// was: Ogre::ShadowTextureManager::getNullShadowTexture(Ogre::PixelFormat)
// IDA 0xe20528: 766 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e20528() {
}

// 0xe20d10 — __ZN4Ogre20ShadowTextureManager11clearUnusedEv
#[doc(alias = "Ogre::ShadowTextureManager::clearUnused(void)")]
// was: Ogre::ShadowTextureManager::clearUnused(void)
// IDA 0xe20d10: 122 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e20d10() {
}

// 0xe20e50 — __ZN4Ogre20ShadowTextureManager5clearEv
#[doc(alias = "Ogre::ShadowTextureManager::clear(void)")]
// was: Ogre::ShadowTextureManager::clear(void)
// IDA 0xe20e50: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e20e50() {
}

// 0xe20eb4 — __ZNSt8_Rb_treeIPN4Ogre7TextureES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Texture *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Texture *>,false>::~_Rb_tree_impl()
// IDA 0xe20eb4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e20eb4() {
}

// 0xe20eb8 — __ZNSt8_Rb_treeIPN4Ogre7TextureES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Texture *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Texture *>,false>::~_Rb_tree_impl()
// IDA 0xe20eb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e20eb8() {
}

// 0xe20ec4 — __ZNSt8_Rb_treeIPN4Ogre7TextureES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Texture *> *)")]
// was: std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Texture *> *)
// IDA 0xe20ec4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e20ec4() {
}

// 0xe20f20 — __ZN4Ogre26ShadowVolumeExtrudeProgram10initialiseEv
#[doc(alias = "Ogre::ShadowVolumeExtrudeProgram::initialise(void)")]
// was: Ogre::ShadowVolumeExtrudeProgram::initialise(void)
// IDA 0xe20f20: 2195 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e20f20() {
}

// 0xe226a0 — __ZN4Ogre26ShadowVolumeExtrudeProgram16getProgramSourceENS_5Light10LightTypesESsbb
#[doc(alias = "Ogre::ShadowVolumeExtrudeProgram::getProgramSource(Ogre::Light::LightTypes,std::string,bool,bool)")]
// was: Ogre::ShadowVolumeExtrudeProgram::getProgramSource(Ogre::Light::LightTypes,std::string,bool,bool)
// IDA 0xe226a0: 435 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e226a0() {
}

// 0xe22c20 — __ZN4Ogre26ShadowVolumeExtrudeProgram8shutdownEv
#[doc(alias = "Ogre::ShadowVolumeExtrudeProgram::shutdown(void)")]
// was: Ogre::ShadowVolumeExtrudeProgram::shutdown(void)
// IDA 0xe22c20: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e22c20() {
}

// 0xe22ccc — __ZN4Ogre26ShadowVolumeExtrudeProgram14getProgramNameENS_5Light10LightTypesEbb
#[doc(alias = "Ogre::ShadowVolumeExtrudeProgram::getProgramName(Ogre::Light::LightTypes,bool,bool)")]
// was: Ogre::ShadowVolumeExtrudeProgram::getProgramName(Ogre::Light::LightTypes,bool,bool)
// IDA 0xe22ccc: 37 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e22ccc() {
}

// 0xe22d34 — __ZN4Ogre9SharedPtrINS_10GpuProgramEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgram>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::GpuProgram>::~SharedPtr()
// IDA 0xe22d34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e22d34() {
}

// 0xe22de4 — __ZN4Ogre9SharedPtrINS_10GpuProgramEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgram>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::GpuProgram>::~SharedPtr()
// IDA 0xe22de4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e22de4() {
}

// 0xe22ed8 — __ZN4Ogre9SharedPtrINS_10GpuProgramEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgram>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::GpuProgram>::destroy(void)
// IDA 0xe22ed8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e22ed8() {
}

// 0xe22f10 — __ZN4Ogre9SharedPtrINS_10GpuProgramEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgram>::swap(Ogre::SharedPtr<Ogre::GpuProgram>&)")]
// was: Ogre::SharedPtr<Ogre::GpuProgram>::swap(Ogre::SharedPtr<Ogre::GpuProgram>&)
// IDA 0xe22f10: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e22f10() {
}

// 0xe238c4 — __ZN4Ogre16SimpleRenderableC2Ev
#[doc(alias = "Ogre::SimpleRenderable::SimpleRenderable(void)")]
// was: Ogre::SimpleRenderable::SimpleRenderable(void)
// IDA 0xe238c4: 637 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e238c4() {
}

// 0xe2400c — __ZN4Ogre16SimpleRenderable11setMaterialERKSs
#[doc(alias = "Ogre::SimpleRenderable::setMaterial(std::string const&)")]
// was: Ogre::SimpleRenderable::setMaterial(std::string const&)
// IDA 0xe2400c: 356 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e2400c() {
}

// 0xe243d4 — __ZNK4Ogre16SimpleRenderable11getMaterialEv
#[doc(alias = "Ogre::SimpleRenderable::getMaterial(void)const")]
// was: Ogre::SimpleRenderable::getMaterial(void)const
// IDA 0xe243d4: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e243d4() {
}

// 0xe243dc — __ZThn188_NK4Ogre16SimpleRenderable11getMaterialEv
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::getMaterial(void)const")]
// was: non-virtual thunk to Ogre::SimpleRenderable::getMaterial(void)const
// IDA 0xe243dc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e243dc() {
}

// 0xe243e0 — __ZN4Ogre16SimpleRenderable18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::SimpleRenderable::getRenderOperation(Ogre::RenderOperation &)")]
// was: Ogre::SimpleRenderable::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xe243e0: 8 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e243e0() {
}

// 0xe243fc — __ZThn188_N4Ogre16SimpleRenderable18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::getRenderOperation(Ogre::RenderOperation &)")]
// was: non-virtual thunk to Ogre::SimpleRenderable::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xe243fc: 8 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e243fc() {
}

// 0xe24418 — __ZN4Ogre16SimpleRenderable18setRenderOperationERKNS_15RenderOperationE
#[doc(alias = "Ogre::SimpleRenderable::setRenderOperation(Ogre::RenderOperation const&)")]
// was: Ogre::SimpleRenderable::setRenderOperation(Ogre::RenderOperation const&)
// IDA 0xe24418: 8 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24418() {
}

// 0xe24434 — __ZNK4Ogre16SimpleRenderable18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::SimpleRenderable::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::SimpleRenderable::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xe24434: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24434() {
}

// 0xe24494 — __ZThn188_NK4Ogre16SimpleRenderable18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: non-virtual thunk to Ogre::SimpleRenderable::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xe24494: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24494() {
}

// 0xe244f4 — __ZN4Ogre16SimpleRenderable20_notifyCurrentCameraEPNS_6CameraE
#[doc(alias = "Ogre::SimpleRenderable::_notifyCurrentCamera(Ogre::Camera *)")]
// was: Ogre::SimpleRenderable::_notifyCurrentCamera(Ogre::Camera *)
// IDA 0xe244f4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e244f4() {
}

// 0xe24508 — __ZN4Ogre16SimpleRenderable14setBoundingBoxERKNS_14AxisAlignedBoxE
#[doc(alias = "Ogre::SimpleRenderable::setBoundingBox(Ogre::AxisAlignedBox const&)")]
// was: Ogre::SimpleRenderable::setBoundingBox(Ogre::AxisAlignedBox const&)
// IDA 0xe24508: 26 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24508() {
}

// 0xe24550 — __ZNK4Ogre16SimpleRenderable14getBoundingBoxEv
#[doc(alias = "Ogre::SimpleRenderable::getBoundingBox(void)const")]
// was: Ogre::SimpleRenderable::getBoundingBox(void)const
// IDA 0xe24550: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24550() {
}

// 0xe24558 — __ZN4Ogre16SimpleRenderable18_updateRenderQueueEPNS_11RenderQueueE
#[doc(alias = "Ogre::SimpleRenderable::_updateRenderQueue(Ogre::RenderQueue *)")]
// was: Ogre::SimpleRenderable::_updateRenderQueue(Ogre::RenderQueue *)
// IDA 0xe24558: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24558() {
}

// 0xe24570 — __ZN4Ogre16SimpleRenderable16visitRenderablesEPNS_10Renderable7VisitorEb
#[doc(alias = "Ogre::SimpleRenderable::visitRenderables(Ogre::Renderable::Visitor *,bool)")]
// was: Ogre::SimpleRenderable::visitRenderables(Ogre::Renderable::Visitor *,bool)
// IDA 0xe24570: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24570() {
}

// 0xe24594 — __ZN4Ogre16SimpleRenderableD0Ev
#[doc(alias = "Ogre::SimpleRenderable::~SimpleRenderable()")]
// was: Ogre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe24594: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e24594() {
}

// 0xe24624 — __ZN4Ogre16SimpleRenderableD1Ev
#[doc(alias = "Ogre::SimpleRenderable::~SimpleRenderable()")]
// was: Ogre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe24624: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e24624() {
}

// 0xe24630 — __ZThn4_N4Ogre16SimpleRenderableD0Ev
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::~SimpleRenderable()")]
// was: non-virtual thunk to Ogre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe24630: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e24630() {
}

// 0xe246c4 — __ZThn188_N4Ogre16SimpleRenderableD0Ev
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::~SimpleRenderable()")]
// was: non-virtual thunk to Ogre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe246c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e246c4() {
}

// 0xe24758 — __ZN4Ogre16SimpleRenderableD2Ev
#[doc(alias = "Ogre::SimpleRenderable::~SimpleRenderable()")]
// was: Ogre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe24758: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e24758() {
}

// 0xe24918 — __ZThn4_N4Ogre16SimpleRenderableD1Ev
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::~SimpleRenderable()")]
// was: non-virtual thunk to Ogre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe24918: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e24918() {
}

// 0xe24924 — __ZThn188_N4Ogre16SimpleRenderableD1Ev
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::~SimpleRenderable()")]
// was: non-virtual thunk to Ogre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe24924: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e24924() {
}

// 0xe24930 — __ZNK4Ogre16SimpleRenderable14getMovableTypeEv
#[doc(alias = "Ogre::SimpleRenderable::getMovableType(void)const")]
// was: Ogre::SimpleRenderable::getMovableType(void)const
// IDA 0xe24930: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24930() {
}

// 0xe24a24 — __ZNK4Ogre16SimpleRenderable9getLightsEv
#[doc(alias = "Ogre::SimpleRenderable::getLights(void)const")]
// was: Ogre::SimpleRenderable::getLights(void)const
// IDA 0xe24a24: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24a24() {
}

// 0xe24a34 — __ZThn188_NK4Ogre16SimpleRenderable9getLightsEv
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::getLights(void)const")]
// was: non-virtual thunk to Ogre::SimpleRenderable::getLights(void)const
// IDA 0xe24a34: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24a34() {
}

// 0xe24a78 — __ZN4Ogre12SimpleSplineC1Ev
#[doc(alias = "Ogre::SimpleSpline::SimpleSpline(void)")]
// was: Ogre::SimpleSpline::SimpleSpline(void)
// IDA 0xe24a78: 40 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24a78() {
}

// 0xe24ae4 — __ZN4Ogre12SimpleSplineD1Ev
#[doc(alias = "Ogre::SimpleSpline::~SimpleSpline()")]
// was: Ogre::SimpleSpline::~SimpleSpline()
// IDA 0xe24ae4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e24ae4() {
}

// 0xe24b88 — __ZN4Ogre12SimpleSpline8addPointERKNS_7Vector3E
#[doc(alias = "Ogre::SimpleSpline::addPoint(Ogre::Vector3 const&)")]
// was: Ogre::SimpleSpline::addPoint(Ogre::Vector3 const&)
// IDA 0xe24b88: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24b88() {
}

// 0xe24bd8 — __ZN4Ogre12SimpleSpline14recalcTangentsEv
#[doc(alias = "Ogre::SimpleSpline::recalcTangents(void)")]
// was: Ogre::SimpleSpline::recalcTangents(void)
// IDA 0xe24bd8: 157 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24bd8() {
}

// 0xe24de4 — __ZNK4Ogre12SimpleSpline11interpolateEjf
#[doc(alias = "Ogre::SimpleSpline::interpolate(unsigned int,float)const")]
// was: Ogre::SimpleSpline::interpolate(unsigned int,float)const
// IDA 0xe24de4: 117 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24de4() {
}

// 0xe24fa4 — __ZN4Ogre12SimpleSpline5clearEv
#[doc(alias = "Ogre::SimpleSpline::clear(void)")]
// was: Ogre::SimpleSpline::clear(void)
// IDA 0xe24fa4: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24fa4() {
}

// 0xe24fb0 — __ZN4Ogre12SimpleSpline16setAutoCalculateEb
#[doc(alias = "Ogre::SimpleSpline::setAutoCalculate(bool)")]
// was: Ogre::SimpleSpline::setAutoCalculate(bool)
// IDA 0xe24fb0: 2 insns (STRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24fb0() {
}

// 0xe24fb4 — __ZNSt6vectorIN4Ogre7Vector3ENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_
#[doc(alias = "std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Vector3 const&)")]
// was: std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Vector3 const&)
// IDA 0xe24fb4: 221 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24fb4() {
}

// 0xe25278 — __ZN4Ogre8SkeletonC2Ev
#[doc(alias = "Ogre::Skeleton::Skeleton(void)")]
// was: Ogre::Skeleton::Skeleton(void)
// IDA 0xe25278: 98 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e25278() {
}

// 0xe253c4 — __ZN4Ogre8SkeletonC1EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::Skeleton::Skeleton(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::Skeleton::Skeleton(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xe253c4: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e253c4() {
}

// 0xe253e0 — __ZN4Ogre8SkeletonC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::Skeleton::Skeleton(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::Skeleton::Skeleton(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xe253e0: 268 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e253e0() {
}

// 0xe256d4 — __ZN4Ogre8SkeletonD0Ev
#[doc(alias = "Ogre::Skeleton::~Skeleton()")]
// was: Ogre::Skeleton::~Skeleton()
// IDA 0xe256d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e256d4() {
}

// 0xe25764 — __ZN4Ogre8SkeletonD1Ev
#[doc(alias = "Ogre::Skeleton::~Skeleton()")]
// was: Ogre::Skeleton::~Skeleton()
// IDA 0xe25764: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e25764() {
}

// 0xe25770 — __ZN4Ogre8SkeletonD2Ev
#[doc(alias = "Ogre::Skeleton::~Skeleton()")]
// was: Ogre::Skeleton::~Skeleton()
// IDA 0xe25770: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e25770() {
}

// 0xe2595c — __ZN4Ogre8Skeleton8loadImplEv
#[doc(alias = "Ogre::Skeleton::loadImpl(void)")]
// was: Ogre::Skeleton::loadImpl(void)
// IDA 0xe2595c: 348 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e2595c() {
}

// 0xe25cb8 — __ZN4Ogre8Skeleton10unloadImplEv
#[doc(alias = "Ogre::Skeleton::unloadImpl(void)")]
// was: Ogre::Skeleton::unloadImpl(void)
// IDA 0xe25cb8: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e25cb8() {
}

// 0xe25d94 — __ZN4Ogre8Skeleton10createBoneEv
#[doc(alias = "Ogre::Skeleton::createBone(void)")]
// was: Ogre::Skeleton::createBone(void)
// IDA 0xe25d94: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e25d94() {
}

// 0xe25dac — __ZN4Ogre8Skeleton10createBoneERKSs
#[doc(alias = "Ogre::Skeleton::createBone(std::string const&)")]
// was: Ogre::Skeleton::createBone(std::string const&)
// IDA 0xe25dac: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e25dac() {
}

// 0xe25dc4 — __ZN4Ogre8Skeleton10createBoneEt
#[doc(alias = "Ogre::Skeleton::createBone(unsigned short)")]
// was: Ogre::Skeleton::createBone(unsigned short)
// IDA 0xe25dc4: 474 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e25dc4() {
}

// 0xe26334 — __ZN4Ogre8Skeleton10createBoneERKSst
#[doc(alias = "Ogre::Skeleton::createBone(std::string const&,unsigned short)")]
// was: Ogre::Skeleton::createBone(std::string const&,unsigned short)
// IDA 0xe26334: 695 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e26334() {
}

// 0xe26b40 — __ZNK4Ogre8Skeleton11getRootBoneEv
#[doc(alias = "Ogre::Skeleton::getRootBone(void)const")]
// was: Ogre::Skeleton::getRootBone(void)const
// IDA 0xe26b40: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e26b40() {
}

// 0xe26b5c — __ZNK4Ogre8Skeleton14deriveRootBoneEv
#[doc(alias = "Ogre::Skeleton::deriveRootBone(void)const")]
// was: Ogre::Skeleton::deriveRootBone(void)const
// IDA 0xe26b5c: 188 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e26b5c() {
}

// 0xe26d88 — __ZN4Ogre8Skeleton17setAnimationStateERKNS_17AnimationStateSetE
#[doc(alias = "Ogre::Skeleton::setAnimationState(Ogre::AnimationStateSet const&)")]
// was: Ogre::Skeleton::setAnimationState(Ogre::AnimationStateSet const&)
// IDA 0xe26d88: 125 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e26d88() {
}

// 0xe26ee4 — __ZN4Ogre8Skeleton14setBindingPoseEv
#[doc(alias = "Ogre::Skeleton::setBindingPose(void)")]
// was: Ogre::Skeleton::setBindingPose(void)
// IDA 0xe26ee4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e26ee4() {
}

// 0xe26f10 — __ZN4Ogre8Skeleton5resetEb
#[doc(alias = "Ogre::Skeleton::reset(bool)")]
// was: Ogre::Skeleton::reset(bool)
// IDA 0xe26f10: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e26f10() {
}

// 0xe26f40 — __ZN4Ogre8Skeleton15createAnimationERKSsf
#[doc(alias = "Ogre::Skeleton::createAnimation(std::string const&,float)")]
// was: Ogre::Skeleton::createAnimation(std::string const&,float)
// IDA 0xe26f40: 306 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e26f40() {
}

// 0xe272bc — __ZThn88_N4Ogre8Skeleton15createAnimationERKSsf
#[doc(alias = "non-virtual thunk toOgre::Skeleton::createAnimation(std::string const&,float)")]
// was: non-virtual thunk to Ogre::Skeleton::createAnimation(std::string const&,float)
// IDA 0xe272bc: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e272bc() {
}

// 0xe272c8 — __ZNK4Ogre8Skeleton12getAnimationERKSsPPKNS_29LinkedSkeletonAnimationSourceE
#[doc(alias = "Ogre::Skeleton::getAnimation(std::string const&,Ogre::LinkedSkeletonAnimationSource const**)const")]
// was: Ogre::Skeleton::getAnimation(std::string const&,Ogre::LinkedSkeletonAnimationSource const**)const
// IDA 0xe272c8: 195 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e272c8() {
}

// 0xe27500 — __ZNK4Ogre8Skeleton12getAnimationERKSs
#[doc(alias = "Ogre::Skeleton::getAnimation(std::string const&)const")]
// was: Ogre::Skeleton::getAnimation(std::string const&)const
// IDA 0xe27500: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e27500() {
}

// 0xe27510 — __ZThn88_NK4Ogre8Skeleton12getAnimationERKSs
#[doc(alias = "non-virtual thunk toOgre::Skeleton::getAnimation(std::string const&)const")]
// was: non-virtual thunk to Ogre::Skeleton::getAnimation(std::string const&)const
// IDA 0xe27510: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e27510() {
}

// 0xe27524 — __ZNK4Ogre8Skeleton12hasAnimationERKSs
#[doc(alias = "Ogre::Skeleton::hasAnimation(std::string const&)const")]
// was: Ogre::Skeleton::hasAnimation(std::string const&)const
// IDA 0xe27524: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e27524() {
}

// 0xe2753c — __ZThn88_NK4Ogre8Skeleton12hasAnimationERKSs
#[doc(alias = "non-virtual thunk toOgre::Skeleton::hasAnimation(std::string const&)const")]
// was: non-virtual thunk to Ogre::Skeleton::hasAnimation(std::string const&)const
// IDA 0xe2753c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e2753c() {
}

// 0xe27554 — __ZNK4Ogre8Skeleton17_getAnimationImplERKSsPPKNS_29LinkedSkeletonAnimationSourceE
#[doc(alias = "Ogre::Skeleton::_getAnimationImpl(std::string const&,Ogre::LinkedSkeletonAnimationSource const**)const")]
// was: Ogre::Skeleton::_getAnimationImpl(std::string const&,Ogre::LinkedSkeletonAnimationSource const**)const
// IDA 0xe27554: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e27554() {
}

// 0xe275bc — __ZN4Ogre8Skeleton15removeAnimationERKSs
#[doc(alias = "Ogre::Skeleton::removeAnimation(std::string const&)")]
// was: Ogre::Skeleton::removeAnimation(std::string const&)
// IDA 0xe275bc: 231 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e275bc() {
}

// 0xe2785c — __ZThn88_N4Ogre8Skeleton15removeAnimationERKSs
#[doc(alias = "non-virtual thunk toOgre::Skeleton::removeAnimation(std::string const&)")]
// was: non-virtual thunk to Ogre::Skeleton::removeAnimation(std::string const&)
// IDA 0xe2785c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e2785c() {
}

// 0xe27868 — __ZN4Ogre8Skeleton19_initAnimationStateEPNS_17AnimationStateSetE
#[doc(alias = "Ogre::Skeleton::_initAnimationState(Ogre::AnimationStateSet *)")]
// was: Ogre::Skeleton::_initAnimationState(Ogre::AnimationStateSet *)
// IDA 0xe27868: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e27868() {
}

// 0xe278e8 — __ZN4Ogre8Skeleton22_refreshAnimationStateEPNS_17AnimationStateSetE
#[doc(alias = "Ogre::Skeleton::_refreshAnimationState(Ogre::AnimationStateSet *)")]
// was: Ogre::Skeleton::_refreshAnimationState(Ogre::AnimationStateSet *)
// IDA 0xe278e8: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e278e8() {
}

// 0xe279b8 — __ZN4Ogre8Skeleton23_notifyManualBonesDirtyEv
#[doc(alias = "Ogre::Skeleton::_notifyManualBonesDirty(void)")]
// was: Ogre::Skeleton::_notifyManualBonesDirty(void)
// IDA 0xe279b8: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e279b8() {
}

// 0xe279c0 — __ZN4Ogre8Skeleton28_notifyManualBoneStateChangeEPNS_4BoneE
#[doc(alias = "Ogre::Skeleton::_notifyManualBoneStateChange(Ogre::Bone *)")]
// was: Ogre::Skeleton::_notifyManualBoneStateChange(Ogre::Bone *)
// IDA 0xe279c0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e279c0() {
}

// 0xe27a2c — __ZNK4Ogre8Skeleton11getNumBonesEv
#[doc(alias = "Ogre::Skeleton::getNumBones(void)const")]
// was: Ogre::Skeleton::getNumBones(void)const
// IDA 0xe27a2c: 4 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e27a2c() {
}

// 0xe27a38 — __ZN4Ogre8Skeleton16_getBoneMatricesEPNS_7Matrix4E
#[doc(alias = "Ogre::Skeleton::_getBoneMatrices(Ogre::Matrix4 *)")]
// was: Ogre::Skeleton::_getBoneMatrices(Ogre::Matrix4 *)
// IDA 0xe27a38: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e27a38() {
}

// 0xe27a68 — __ZNK4Ogre8Skeleton16getNumAnimationsEv
#[doc(alias = "Ogre::Skeleton::getNumAnimations(void)const")]
// was: Ogre::Skeleton::getNumAnimations(void)const
// IDA 0xe27a68: 2 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e27a68() {
}

// 0xe27a70 — __ZThn88_NK4Ogre8Skeleton16getNumAnimationsEv
#[doc(alias = "non-virtual thunk toOgre::Skeleton::getNumAnimations(void)const")]
// was: non-virtual thunk to Ogre::Skeleton::getNumAnimations(void)const
// IDA 0xe27a70: 2 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e27a70() {
}