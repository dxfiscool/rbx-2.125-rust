//! rendering shard watchdog_rendering_w15d2 — 120 stubs EA-sorted asc rendering-filter not in /tmp/global_eas.txt (0xd4bf7c..0xf65274, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) rendering filter (Ogre::Material|Ogre::Light|G3D::), global EA dedup.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xd4bf7c — __ZN4Ogre4Pass18setIteratePerLightEbbNS_5Light10LightTypesE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "Ogre::Pass::setIteratePerLight(bool,bool,Ogre::Light::LightTypes)")]
// was: __ZN4Ogre4Pass18setIteratePerLightEbbNS_5Light10LightTypesE
// IDA 0xd4bf7c: 4 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d4bf7c() {
}

// 0xd9a2fc — __ZN4Ogre12LightFactoryD1Ev
// type: void __fastcall(Ogre::LightFactory *__hidden this)
#[doc(alias = "Ogre::LightFactory::~LightFactory()")]
// was: __ZN4Ogre12LightFactoryD1Ev
// IDA 0xd9a2fc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d9a2fc() {
}

// 0xd9a300 — __ZN4Ogre12LightFactoryD0Ev
// type: void __fastcall(Ogre::LightFactory *__hidden this)
#[doc(alias = "Ogre::LightFactory::~LightFactory()")]
// was: __ZN4Ogre12LightFactoryD0Ev
// IDA 0xd9a300: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d9a300() {
}

// 0xd9e30c — __ZN4Ogre12SceneManager12destroyLightEPNS_5LightE
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, Ogre::Light *)
#[doc(alias = "Ogre::SceneManager::destroyLight(Ogre::Light *)")]
// was: __ZN4Ogre12SceneManager12destroyLightEPNS_5LightE
// IDA 0xd9e30c: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d9e30c() {
}

// 0xd9e35c — __ZN4Ogre12SceneManager18_populateLightListERKNS_7Vector3EfRNS_12HashedVectorIPNS_5LightEEEj
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::_populateLightList(Ogre::Vector3 const&,float,Ogre::HashedVector<Ogre::Light *> &,unsigned int)")]
// was: __ZN4Ogre12SceneManager18_populateLightListERKNS_7Vector3EfRNS_12HashedVectorIPNS_5LightEEEj
// IDA 0xd9e35c: 195 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d9e35c() {
}

// 0xd9e570 — __ZN4Ogre12SceneManager18_populateLightListEPKNS_9SceneNodeEfRNS_12HashedVectorIPNS_5LightEEEj
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::_populateLightList(Ogre::SceneNode const*,float,Ogre::HashedVector<Ogre::Light *> &,unsigned int)")]
// was: __ZN4Ogre12SceneManager18_populateLightListEPKNS_9SceneNodeEfRNS_12HashedVectorIPNS_5LightEEEj
// IDA 0xd9e570: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d9e570() {
}

// 0xda8608 — __ZN4Ogre12SceneManager13renderObjectsERKNS_26QueuedRenderableCollectionENS1_16OrganisationModeEbbPKNS_12HashedVectorIPNS_5LightEEE
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::renderObjects(Ogre::QueuedRenderableCollection const&,Ogre::QueuedRenderableCollection::OrganisationMode,bool,bool,Ogre::HashedVector<Ogre::Light *> const*)")]
// was: __ZN4Ogre12SceneManager13renderObjectsERKNS_26QueuedRenderableCollectionENS1_16OrganisationModeEbbPKNS_12HashedVectorIPNS_5LightEEE
// IDA 0xda8608: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_da8608() {
}

// 0xda87c8 — __ZN4Ogre12SceneManager36renderTransparentShadowCasterObjectsERKNS_26QueuedRenderableCollectionENS1_16OrganisationModeEbbPKNS_12HashedVectorIPNS_5LightEEE
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::renderTransparentShadowCasterObjects(Ogre::QueuedRenderableCollection const&,Ogre::QueuedRenderableCollection::OrganisationMode,bool,bool,Ogre::HashedVector<Ogre::Light *> const*)")]
// was: __ZN4Ogre12SceneManager36renderTransparentShadowCasterObjectsERKNS_26QueuedRenderableCollectionENS1_16OrganisationModeEbbPKNS_12HashedVectorIPNS_5LightEEE
// IDA 0xda87c8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_da87c8() {
}

// 0xda8800 — __ZN4Ogre12SceneManager18renderSingleObjectEPNS_10RenderableEPKNS_4PassEbbPKNS_12HashedVectorIPNS_5LightEEE
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::renderSingleObject(Ogre::Renderable *,Ogre::Pass const*,bool,bool,Ogre::HashedVector<Ogre::Light *> const*)")]
// was: __ZN4Ogre12SceneManager18renderSingleObjectEPNS_10RenderableEPKNS_4PassEbbPKNS_12HashedVectorIPNS_5LightEEE
// IDA 0xda8800: 1473 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_da8800() {
}

// 0xdaaf5c — __ZN4Ogre12SceneManager22fireRenderSingleObjectEPNS_10RenderableEPKNS_4PassEPKNS_19AutoParamDataSourceEPKNS_12HashedVectorIPNS_5LightEEEb
// type: int __fastcall(int result, int, int, int, int, int)
#[doc(alias = "Ogre::SceneManager::fireRenderSingleObject(Ogre::Renderable *,Ogre::Pass const*,Ogre::AutoParamDataSource const*,Ogre::HashedVector<Ogre::Light *> const*,bool)")]
// was: __ZN4Ogre12SceneManager22fireRenderSingleObjectEPNS_10RenderableEPKNS_4PassEPKNS_19AutoParamDataSourceEPKNS_12HashedVectorIPNS_5LightEEEb
// IDA 0xdaaf5c: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_daaf5c() {
}

// 0xdaafcc — __ZN4Ogre12SceneManager27fireShadowTexturesPreCasterEPNS_5LightEPNS_6CameraEm
// type: int __fastcall(int this, Ogre::Light *, Ogre::Camera *, unsigned int)
#[doc(alias = "Ogre::SceneManager::fireShadowTexturesPreCaster(Ogre::Light *,Ogre::Camera *,unsigned long)")]
// was: __ZN4Ogre12SceneManager27fireShadowTexturesPreCasterEPNS_5LightEPNS_6CameraEm
// IDA 0xdaafcc: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_daafcc() {
}

// 0xdab004 — __ZN4Ogre12SceneManager29fireShadowTexturesPreReceiverEPNS_5LightEPNS_7FrustumE
// type: int __fastcall(int this, Ogre::Light *, Ogre::Frustum *)
#[doc(alias = "Ogre::SceneManager::fireShadowTexturesPreReceiver(Ogre::Light *,Ogre::Frustum *)")]
// was: __ZN4Ogre12SceneManager29fireShadowTexturesPreReceiverEPNS_5LightEPNS_7FrustumE
// IDA 0xdab004: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dab004() {
}

// 0xdabee4 — __ZN4Ogre12SceneManager25findShadowCastersForLightEPKNS_5LightEPKNS_6CameraE
// type: char *__fastcall(Ogre::SceneManager *this, const Ogre::Light *, const Ogre::Camera *)
#[doc(alias = "Ogre::SceneManager::findShadowCastersForLight(Ogre::Light const*,Ogre::Camera const*)")]
// was: __ZN4Ogre12SceneManager25findShadowCastersForLightEPKNS_5LightEPKNS_6CameraE
// IDA 0xdabee4: 379 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dabee4() {
}

// 0xdaf850 — __ZN4Ogre12SceneManager19getLightScissorRectEPNS_5LightEPKNS_6CameraE
// type: char *__fastcall(Ogre::SceneManager *this, Ogre::Light *, const Ogre::Camera *)
#[doc(alias = "Ogre::SceneManager::getLightScissorRect(Ogre::Light *,Ogre::Camera const*)")]
// was: __ZN4Ogre12SceneManager19getLightScissorRectEPNS_5LightEPKNS_6CameraE
// IDA 0xdaf850: 171 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_daf850() {
}

// 0xdafa0c — __ZN4Ogre12SceneManager18buildAndSetScissorERKNS_12HashedVectorIPNS_5LightEEEPKNS_6CameraE
// type: int __fastcall(int, int, int)
#[doc(alias = "Ogre::SceneManager::buildAndSetScissor(Ogre::HashedVector<Ogre::Light *> const&,Ogre::Camera const*)")]
// was: __ZN4Ogre12SceneManager18buildAndSetScissorERKNS_12HashedVectorIPNS_5LightEEEPKNS_6CameraE
// IDA 0xdafa0c: 158 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dafa0c() {
}

// 0xdafc28 — __ZN4Ogre12SceneManager12buildScissorEPKNS_5LightEPKNS_6CameraERNS_5TRectIfEE
// type: int __fastcall(int, Ogre::Light *this, int, int)
#[doc(alias = "Ogre::SceneManager::buildScissor(Ogre::Light const*,Ogre::Camera const*,Ogre::TRect<float> &)")]
// was: __ZN4Ogre12SceneManager12buildScissorEPKNS_5LightEPKNS_6CameraERNS_5TRectIfEE
// IDA 0xdafc28: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dafc28() {
}

// 0xdafd08 — __ZN4Ogre12SceneManager22getLightClippingPlanesEPNS_5LightE
// type: char *__fastcall(Ogre::SceneManager *this, Ogre::Light *)
#[doc(alias = "Ogre::SceneManager::getLightClippingPlanes(Ogre::Light *)")]
// was: __ZN4Ogre12SceneManager22getLightClippingPlanesEPNS_5LightE
// IDA 0xdafd08: 169 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dafd08() {
}

// 0xdafebc — __ZN4Ogre12SceneManager20buildAndSetLightClipERKNS_12HashedVectorIPNS_5LightEEE
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "Ogre::SceneManager::buildAndSetLightClip(Ogre::HashedVector<Ogre::Light *> const&)")]
// was: __ZN4Ogre12SceneManager20buildAndSetLightClipERKNS_12HashedVectorIPNS_5LightEEE
// IDA 0xdafebc: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dafebc() {
}

// 0xdaff2c — __ZN4Ogre12SceneManager14buildLightClipEPKNS_5LightERSt6vectorINS_5PlaneENS_12STLAllocatorIS5_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, Ogre::Light *this, int)
#[doc(alias = "Ogre::SceneManager::buildLightClip(Ogre::Light const*,std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)")]
// was: __ZN4Ogre12SceneManager14buildLightClipEPKNS_5LightERSt6vectorINS_5PlaneENS_12STLAllocatorIS5_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// IDA 0xdaff2c: 719 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_daff2c() {
}

// 0xdb08b0 — __ZN4Ogre12SceneManager28renderShadowVolumesToStencilEPKNS_5LightEPKNS_6CameraEb
// type: void __fastcall(Ogre::SceneManager *this, const Ogre::Light *, const Ogre::Camera *, int)
#[doc(alias = "Ogre::SceneManager::renderShadowVolumesToStencil(Ogre::Light const*,Ogre::Camera const*,bool)")]
// was: __ZN4Ogre12SceneManager28renderShadowVolumesToStencilEPKNS_5LightEPKNS_6CameraEb
// IDA 0xdb08b0: 1124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db08b0() {
}

// 0xdb13c8 — __ZN4Ogre12SceneManager25renderShadowVolumeObjectsENS_14VectorIteratorISt6vectorIPNS_16ShadowRenderableENS_12STLAllocatorIS4_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEEPNS_4PassEPKNS_12HashedVectorIPNS_5LightEEEmbbb
// type: _DWORD *__fastcall(_DWORD *result, int, _DWORD *, _DWORD *, int, int, char, int, int, int)
#[doc(alias = "Ogre::SceneManager::renderShadowVolumeObjects(Ogre::VectorIterator<std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Pass *,Ogre::HashedVector<Ogre::Light *> const*,unsigned long,bool,bool,bool)")]
// was: __ZN4Ogre12SceneManager25renderShadowVolumeObjectsENS_14VectorIteratorISt6vectorIPNS_16ShadowRenderableENS_12STLAllocatorIS4_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEEPNS_4PassEPKNS_12HashedVectorIPNS_5LightEEEmbbb
// IDA 0xdb13c8: 139 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db13c8() {
}

// 0xdb7480 — __ZN4Ogre12SceneManager21_injectRenderWithPassEPNS_4PassEPNS_10RenderableEbbPKNS_12HashedVectorIPNS_5LightEEE
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::_injectRenderWithPass(Ogre::Pass *,Ogre::Renderable *,bool,bool,Ogre::HashedVector<Ogre::Light *> const*)")]
// was: __ZN4Ogre12SceneManager21_injectRenderWithPassEPNS_4PassEPNS_10RenderableEbbPKNS_12HashedVectorIPNS_5LightEEE
// IDA 0xdb7480: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db7480() {
}

// 0xdb77a8 — __ZN4Ogre12SceneManager9useLightsERKNS_12HashedVectorIPNS_5LightEEEt
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::useLights(Ogre::HashedVector<Ogre::Light *> const&,unsigned short)")]
// was: __ZN4Ogre12SceneManager9useLightsERKNS_12HashedVectorIPNS_5LightEEEt
// IDA 0xdb77a8: 70 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db77a8() {
}

// 0xdb7850 — __ZN4Ogre12SceneManager19useLightsGpuProgramEPKNS_4PassEPKNS_12HashedVectorIPNS_5LightEEE
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneManager::useLightsGpuProgram(Ogre::Pass const*,Ogre::HashedVector<Ogre::Light *> const*)")]
// was: __ZN4Ogre12SceneManager19useLightsGpuProgramEPKNS_4PassEPKNS_12HashedVectorIPNS_5LightEEE
// IDA 0xdb7850: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db7850() {
}

// 0xdbb4e8 — __ZNSt8_Rb_treeIPKN4Ogre6CameraESt4pairIKS3_PKNS0_5LightEESt10_Select1stIS9_ESt4lessIS3_ENS0_12STLAllocatorIS9_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, const char **)
#[doc(alias = "std::_Rb_tree<Ogre::Camera const*,std::pair<Ogre::Camera const* const,Ogre::Light const*>,std::_Select1st<std::pair<Ogre::Camera const* const,Ogre::Light const*>>,std::less<Ogre::Camera const*>,Ogre::STLAllocator<std::pair<Ogre::Camera const* const,Ogre::Light const*>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::Camera const* const,Ogre::Light const*>>,std::pair<Ogre::Camera const* const,Ogre::Light const*> const&)")]
// was: __ZNSt8_Rb_treeIPKN4Ogre6CameraESt4pairIKS3_PKNS0_5LightEESt10_Select1stIS9_ESt4lessIS3_ENS0_12STLAllocatorIS9_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// IDA 0xdbb4e8: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbb4e8() {
}

// 0xdbb6f0 — __ZNSt8_Rb_treeIPKN4Ogre6CameraESt4pairIKS3_PKNS0_5LightEESt10_Select1stIS9_ESt4lessIS3_ENS0_12STLAllocatorIS9_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS9_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<Ogre::Camera const*,std::pair<Ogre::Camera const* const,Ogre::Light const*>,std::_Select1st<std::pair<Ogre::Camera const* const,Ogre::Light const*>>,std::less<Ogre::Camera const*>,Ogre::STLAllocator<std::pair<Ogre::Camera const* const,Ogre::Light const*>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Camera const* const,Ogre::Light const*> const&)")]
// was: __ZNSt8_Rb_treeIPKN4Ogre6CameraESt4pairIKS3_PKNS0_5LightEESt10_Select1stIS9_ESt4lessIS3_ENS0_12STLAllocatorIS9_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS9_
// IDA 0xdbb6f0: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbb6f0() {
}

// 0xdbb8e4 — __ZNSt8_Rb_treeIPKN4Ogre6CameraESt4pairIKS3_PKNS0_5LightEESt10_Select1stIS9_ESt4lessIS3_ENS0_12STLAllocatorIS9_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: void __fastcall(void)
#[doc(alias = "std::_Rb_tree<Ogre::Camera const*,std::pair<Ogre::Camera const* const,Ogre::Light const*>,std::_Select1st<std::pair<Ogre::Camera const* const,Ogre::Light const*>>,std::less<Ogre::Camera const*>,Ogre::STLAllocator<std::pair<Ogre::Camera const* const,Ogre::Light const*>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Camera const* const,Ogre::Light const*>> *)")]
// was: __ZNSt8_Rb_treeIPKN4Ogre6CameraESt4pairIKS3_PKNS0_5LightEESt10_Select1stIS9_ESt4lessIS3_ENS0_12STLAllocatorIS9_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// IDA 0xdbb8e4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbb8e4() {
}

// 0xdbbcd0 — __ZNSt8_Rb_treeIPN4Ogre5LightESt4pairIKS2_NS0_12SceneManager17LightClippingInfoEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: void __fastcall(void)
#[doc(alias = "std::_Rb_tree<Ogre::Light *,std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>,std::_Select1st<std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>>,std::less<Ogre::Light *>,Ogre::STLAllocator<std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>> *)")]
// was: __ZNSt8_Rb_treeIPN4Ogre5LightESt4pairIKS2_NS0_12SceneManager17LightClippingInfoEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// IDA 0xdbbcd0: 66 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbbcd0() {
}

// 0xdbbd8c — __ZNSt8_Rb_treeIPN4Ogre5LightESt4pairIKS2_NS0_12SceneManager17LightClippingInfoEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<Ogre::Light *,std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>,std::_Select1st<std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>>,std::less<Ogre::Light *>,Ogre::STLAllocator<std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo> const&)")]
// was: __ZNSt8_Rb_treeIPN4Ogre5LightESt4pairIKS2_NS0_12SceneManager17LightClippingInfoEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_
// IDA 0xdbbd8c: 70 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbbd8c() {
}

// 0xdbbe40 — __ZNSt8_Rb_treeIPN4Ogre5LightESt4pairIKS2_NS0_12SceneManager17LightClippingInfoEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS7_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<Ogre::Light *,std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>,std::_Select1st<std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>>,std::less<Ogre::Light *>,Ogre::STLAllocator<std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo> const&)")]
// was: __ZNSt8_Rb_treeIPN4Ogre5LightESt4pairIKS2_NS0_12SceneManager17LightClippingInfoEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS7_
// IDA 0xdbbe40: 103 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbbe40() {
}

// 0xdbc198 — __ZSt21__inplace_stable_sortIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_12SceneManager26lightsForShadowTextureLessEEvT_SG_T0_
// type: void __fastcall(void)
#[doc(alias = "void std::__inplace_stable_sort<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightsForShadowTextureLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightsForShadowTextureLess)")]
// was: __ZSt21__inplace_stable_sortIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_12SceneManager26lightsForShadowTextureLessEEvT_SG_T0_
// IDA 0xdbc198: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbc198() {
}

// 0xdbc1f0 — __ZSt22__stable_sort_adaptiveIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES5_iNS2_12SceneManager26lightsForShadowTextureLessEEvT_SG_T0_T1_T2_
// type: void __fastcall(void)
#[doc(alias = "void std::__stable_sort_adaptive<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,int,Ogre::SceneManager::lightsForShadowTextureLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,int,Ogre::SceneManager::lightsForShadowTextureLess)")]
// was: __ZSt22__stable_sort_adaptiveIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES5_iNS2_12SceneManager26lightsForShadowTextureLessEEvT_SG_T0_T1_T2_
// IDA 0xdbc1f0: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbc1f0() {
}

// 0xdbc268 — __ZSt24__merge_sort_with_bufferIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES5_NS2_12SceneManager26lightsForShadowTextureLessEEvT_SG_T0_T1_
// type: void __fastcall(void)
#[doc(alias = "void std::__merge_sort_with_buffer<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,Ogre::SceneManager::lightsForShadowTextureLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,Ogre::SceneManager::lightsForShadowTextureLess)")]
// was: __ZSt24__merge_sort_with_bufferIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES5_NS2_12SceneManager26lightsForShadowTextureLessEEvT_SG_T0_T1_
// IDA 0xdbc268: 112 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbc268() {
}

// 0xdbc38c — __ZSt16__merge_adaptiveIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS5_NS2_12SceneManager26lightsForShadowTextureLessEEvT_SG_SG_T0_SH_T1_SH_T2_
// type: int __fastcall(int, int, int, int, int, void *__dst, int)
#[doc(alias = "void std::__merge_adaptive<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::Light **,Ogre::SceneManager::lightsForShadowTextureLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::Light **,int,Ogre::SceneManager::lightsForShadowTextureLess)")]
// was: __ZSt16__merge_adaptiveIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS5_NS2_12SceneManager26lightsForShadowTextureLessEEvT_SG_SG_T0_SH_T1_SH_T2_
// IDA 0xdbc38c: 269 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbc38c() {
}

// 0xdbc634 — __ZSt5mergeIPPN4Ogre5LightEN9__gnu_cxx17__normal_iteratorIS3_St6vectorIS2_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEEESD_NS0_12SceneManager26lightsForShadowTextureLessEET1_T_SH_T0_SI_SG_T2_
// type: int __fastcall(void *__src, int, int, int, void *__dst)
#[doc(alias = "__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::merge<Ogre::Light **,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightsForShadowTextureLess>(Ogre::Light **,Ogre::Light **,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightsForShadowTextureLess)")]
// was: __ZSt5mergeIPPN4Ogre5LightEN9__gnu_cxx17__normal_iteratorIS3_St6vectorIS2_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEEESD_NS0_12SceneManager26lightsForShadowTextureLessEET1_T_SH_T0_SI_SG_T2_
// IDA 0xdbc634: 69 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbc634() {
}

// 0xdbc6e4 — __ZSt16__merge_backwardIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES5_SD_NS2_12SceneManager26lightsForShadowTextureLessEET1_T_SH_T0_SI_SG_T2_
// type: void __fastcall(void)
#[doc(alias = "__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::__merge_backward<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightsForShadowTextureLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,Ogre::Light **,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightsForShadowTextureLess)")]
// was: __ZSt16__merge_backwardIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES5_SD_NS2_12SceneManager26lightsForShadowTextureLessEET1_T_SH_T0_SI_SG_T2_
// IDA 0xdbc6e4: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbc6e4() {
}

// 0xdbc7c8 — __ZSt8__rotateIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEEvT_SE_SE_St26random_access_iterator_tag
// type: int __fastcall(int result, int, int)
#[doc(alias = "void std::__rotate<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::random_access_iterator_tag)")]
// was: __ZSt8__rotateIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEEvT_SE_SE_St26random_access_iterator_tag
// IDA 0xdbc7c8: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbc7c8() {
}

// 0xdbc8bc — __ZSt5mergeIPPN4Ogre5LightES3_N9__gnu_cxx17__normal_iteratorIS3_St6vectorIS2_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEEENS0_12SceneManager26lightsForShadowTextureLessEET1_T_SH_T0_SI_SG_T2_
// type: int __fastcall(void *__src, int, int, int, void *__dst)
#[doc(alias = "__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::merge<Ogre::Light **,Ogre::Light **,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightsForShadowTextureLess>(Ogre::Light **,Ogre::Light **,Ogre::Light **,Ogre::Light **,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightsForShadowTextureLess)")]
// was: __ZSt5mergeIPPN4Ogre5LightES3_N9__gnu_cxx17__normal_iteratorIS3_St6vectorIS2_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEEENS0_12SceneManager26lightsForShadowTextureLessEET1_T_SH_T0_SI_SG_T2_
// IDA 0xdbc8bc: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbc8bc() {
}

// 0xdbc968 — __ZSt5mergeIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEESD_S5_NS2_12SceneManager26lightsForShadowTextureLessEET1_T_SH_T0_SI_SG_T2_
// type: int __fastcall(void *__src, int, int, int, void *__dst)
#[doc(alias = "Ogre::Light ** std::merge<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,Ogre::SceneManager::lightsForShadowTextureLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,Ogre::SceneManager::lightsForShadowTextureLess)")]
// was: __ZSt5mergeIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEESD_S5_NS2_12SceneManager26lightsForShadowTextureLessEET1_T_SH_T0_SI_SG_T2_
// IDA 0xdbc968: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbc968() {
}

// 0xdbca18 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_12SceneManager26lightsForShadowTextureLessEEvT_SG_T0_
// type: void __fastcall(void)
#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightsForShadowTextureLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightsForShadowTextureLess)")]
// was: __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_12SceneManager26lightsForShadowTextureLessEEvT_SG_T0_
// IDA 0xdbca18: 101 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbca18() {
}

// 0xdbcb18 — __ZSt22__merge_without_bufferIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_12SceneManager26lightsForShadowTextureLessEEvT_SG_SG_T0_SH_T1_
// type: void __fastcall(void)
#[doc(alias = "void std::__merge_without_buffer<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::SceneManager::lightsForShadowTextureLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::SceneManager::lightsForShadowTextureLess)")]
// was: __ZSt22__merge_without_bufferIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_12SceneManager26lightsForShadowTextureLessEEvT_SG_SG_T0_SH_T1_
// IDA 0xdbcb18: 201 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbcb18() {
}

// 0xdbd7b0 — __ZNSt6vectorIPN4Ogre5LightENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Light * const&)")]
// was: __ZNSt6vectorIPN4Ogre5LightENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// IDA 0xdbd7b0: 159 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbd7b0() {
}

// 0xdbe704 — __ZSt21__inplace_stable_sortIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_12SceneManager9lightLessEEvT_SG_T0_
// type: int __fastcall(void *__src)
#[doc(alias = "void std::__inplace_stable_sort<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightLess)")]
// was: __ZSt21__inplace_stable_sortIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_12SceneManager9lightLessEEvT_SG_T0_
// IDA 0xdbe704: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbe704() {
}

// 0xdbe7c4 — __ZSt22__stable_sort_adaptiveIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES5_iNS2_12SceneManager9lightLessEEvT_SG_T0_T1_T2_
// type: int __fastcall(void *__src)
#[doc(alias = "void std::__stable_sort_adaptive<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,int,Ogre::SceneManager::lightLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,int,Ogre::SceneManager::lightLess)")]
// was: __ZSt22__stable_sort_adaptiveIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES5_iNS2_12SceneManager9lightLessEEvT_SG_T0_T1_T2_
// IDA 0xdbe7c4: 100 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbe7c4() {
}

// 0xdbe8d0 — __ZSt16__merge_adaptiveIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS5_NS2_12SceneManager9lightLessEEvT_SG_SG_T0_SH_T1_SH_T2_
// type: int __fastcall(void *__src, void *, int, int, int, void *__dst, int)
#[doc(alias = "void std::__merge_adaptive<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::Light **,Ogre::SceneManager::lightLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::Light **,int,Ogre::SceneManager::lightLess)")]
// was: __ZSt16__merge_adaptiveIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS5_NS2_12SceneManager9lightLessEEvT_SG_SG_T0_SH_T1_SH_T2_
// IDA 0xdbe8d0: 223 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbe8d0() {
}

// 0xdbeb20 — __ZSt16__merge_backwardIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES5_SD_NS2_12SceneManager9lightLessEET1_T_SH_T0_SI_SG_T2_
// type: int __fastcall(void *__src, int, int, int, int)
#[doc(alias = "__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::__merge_backward<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,Ogre::Light **,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SceneManager::lightLess)")]
// was: __ZSt16__merge_backwardIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES5_SD_NS2_12SceneManager9lightLessEET1_T_SH_T0_SI_SG_T2_
// IDA 0xdbeb20: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbeb20() {
}

// 0xdbebb8 — __ZSt22__chunk_insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_12SceneManager9lightLessEEvT_SG_T0_T1_
// type: int __fastcall(void *__src)
#[doc(alias = "void std::__chunk_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::SceneManager::lightLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::SceneManager::lightLess)")]
// was: __ZSt22__chunk_insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_12SceneManager9lightLessEEvT_SG_T0_T1_
// IDA 0xdbebb8: 104 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbebb8() {
}

// 0xdbecd4 — __ZSt17__merge_sort_loopIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES5_iNS2_12SceneManager9lightLessEEvT_SG_T0_T1_T2_
// type: int __fastcall(void *__src, int, void *__dst)
#[doc(alias = "void std::__merge_sort_loop<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,int,Ogre::SceneManager::lightLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light **,int,Ogre::SceneManager::lightLess)")]
// was: __ZSt17__merge_sort_loopIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES5_iNS2_12SceneManager9lightLessEEvT_SG_T0_T1_T2_
// IDA 0xdbecd4: 95 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbecd4() {
}

// 0xdbedcc — __ZSt17__merge_sort_loopIPPN4Ogre5LightEN9__gnu_cxx17__normal_iteratorIS3_St6vectorIS2_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEEEiNS0_12SceneManager9lightLessEEvT_SG_T0_T1_T2_
// type: int __fastcall(void *__src, int, void *__dst)
#[doc(alias = "void std::__merge_sort_loop<Ogre::Light **,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::SceneManager::lightLess>(Ogre::Light **,Ogre::Light **,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::SceneManager::lightLess)")]
// was: __ZSt17__merge_sort_loopIPPN4Ogre5LightEN9__gnu_cxx17__normal_iteratorIS3_St6vectorIS2_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEEEiNS0_12SceneManager9lightLessEEvT_SG_T0_T1_T2_
// IDA 0xdbedcc: 108 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbedcc() {
}

// 0xdbeee0 — __ZSt22__merge_without_bufferIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_12SceneManager9lightLessEEvT_SG_SG_T0_SH_T1_
// type: void __fastcall(void)
#[doc(alias = "void std::__merge_without_buffer<__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::SceneManager::lightLess>(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::SceneManager::lightLess)")]
// was: __ZSt22__merge_without_bufferIN9__gnu_cxx17__normal_iteratorIPPN4Ogre5LightESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_12SceneManager9lightLessEEvT_SG_SG_T0_SH_T1_
// IDA 0xdbeee0: 116 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbeee0() {
}

// 0xdbf010 — __ZNSt6vectorIPN4Ogre5LightENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Light **,std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Light * const&)")]
// was: __ZNSt6vectorIPN4Ogre5LightENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// IDA 0xdbf010: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_dbf010() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xdbf108 — __ZNSt6vectorIPN4Ogre5LightENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm
// type: void __fastcall(void)
#[doc(alias = "std::vector<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
// was: __ZNSt6vectorIPN4Ogre5LightENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm
// IDA 0xdbf108: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbf108() {
}

// 0xdbfb3c — __ZNSt8_Rb_treeIPN4Ogre5LightESt4pairIKS2_NS0_12SceneManager17LightClippingInfoEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED1Ev
// type: void __fastcall(void)
#[doc(alias = "std::_Rb_tree<Ogre::Light *,std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>,std::_Select1st<std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>>,std::less<Ogre::Light *>,Ogre::STLAllocator<std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Light *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre5LightESt4pairIKS2_NS0_12SceneManager17LightClippingInfoEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED1Ev
// IDA 0xdbfb3c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_dbfb3c() {
}

// 0xdbfb40 — __ZNSt8_Rb_treeIPN4Ogre5LightESt4pairIKS2_NS0_12SceneManager17LightClippingInfoEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED0Ev
// type: void __fastcall(void)
#[doc(alias = "std::_Rb_tree<Ogre::Light *,std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>,std::_Select1st<std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>>,std::less<Ogre::Light *>,Ogre::STLAllocator<std::pair<Ogre::Light * const,Ogre::SceneManager::LightClippingInfo>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Light *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre5LightESt4pairIKS2_NS0_12SceneManager17LightClippingInfoEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED0Ev
// IDA 0xdbfb40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dbfb40() {
}

// 0xdbfbbc — __ZNSt12_Vector_baseIPN4Ogre5LightENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// type: void __fastcall(void)
#[doc(alias = "std::_Vector_base<Ogre::Light *,Ogre::STLAllocator<Ogre::Light *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIPN4Ogre5LightENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// IDA 0xdbfbbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dbfbbc() {
}

// 0xdbfbd4 — __ZNSt8_Rb_treeIPKN4Ogre6CameraESt4pairIKS3_PKNS0_5LightEESt10_Select1stIS9_ESt4lessIS3_ENS0_12STLAllocatorIS9_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISD_Lb0EED1Ev
// type: void __fastcall(void)
#[doc(alias = "std::_Rb_tree<Ogre::Camera const*,std::pair<Ogre::Camera const* const,Ogre::Light const*>,std::_Select1st<std::pair<Ogre::Camera const* const,Ogre::Light const*>>,std::less<Ogre::Camera const*>,Ogre::STLAllocator<std::pair<Ogre::Camera const* const,Ogre::Light const*>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Camera const*>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPKN4Ogre6CameraESt4pairIKS3_PKNS0_5LightEESt10_Select1stIS9_ESt4lessIS3_ENS0_12STLAllocatorIS9_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISD_Lb0EED1Ev
// IDA 0xdbfbd4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_dbfbd4() {
}

// 0xdbfbd8 — __ZNSt8_Rb_treeIPKN4Ogre6CameraESt4pairIKS3_PKNS0_5LightEESt10_Select1stIS9_ESt4lessIS3_ENS0_12STLAllocatorIS9_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISD_Lb0EED0Ev
// type: void __fastcall(void)
#[doc(alias = "std::_Rb_tree<Ogre::Camera const*,std::pair<Ogre::Camera const* const,Ogre::Light const*>,std::_Select1st<std::pair<Ogre::Camera const* const,Ogre::Light const*>>,std::less<Ogre::Camera const*>,Ogre::STLAllocator<std::pair<Ogre::Camera const* const,Ogre::Light const*>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Camera const*>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPKN4Ogre6CameraESt4pairIKS3_PKNS0_5LightEESt10_Select1stIS9_ESt4lessIS3_ENS0_12STLAllocatorIS9_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISD_Lb0EED0Ev
// IDA 0xdbfbd8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dbfbd8() {
}

// 0xdbfc74 — __ZN4Ogre9SharedPtrINS_8MaterialEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::operator=(Ogre::SharedPtr<Ogre::Material> const&)")]
// was: __ZN4Ogre9SharedPtrINS_8MaterialEEaSERKS2_
// IDA 0xdbfc74: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbfc74() {
}

// 0xdc3f1c — __ZNK4Ogre9SceneNode10findLightsERNS_12HashedVectorIPNS_5LightEEEfj
// type: void __fastcall(void)
#[doc(alias = "Ogre::SceneNode::findLights(Ogre::HashedVector<Ogre::Light *> &,float,unsigned int)const")]
// was: __ZNK4Ogre9SceneNode10findLightsERNS_12HashedVectorIPNS_5LightEEEfj
// IDA 0xdc3f1c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dc3f1c() {
}

// 0xe187f4 — __ZN4Ogre8any_castIPNS_8MaterialEEET_RKNS_3AnyE
// type: int(void)
#[doc(alias = "Ogre::Material * Ogre::any_cast<Ogre::Material *>(Ogre::Any const&)")]
// was: __ZN4Ogre8any_castIPNS_8MaterialEEET_RKNS_3AnyE
// IDA 0xe187f4: 316 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e187f4() {
}

// 0xe1a6a0 — __ZN4Ogre18MaterialTranslatorD1Ev
// type: void __fastcall(Ogre::MaterialTranslator *__hidden this)
#[doc(alias = "Ogre::MaterialTranslator::~MaterialTranslator()")]
// was: __ZN4Ogre18MaterialTranslatorD1Ev
// IDA 0xe1a6a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1a6a0() {
}

// 0xe1a744 — __ZN4Ogre18MaterialTranslatorD0Ev
// type: void __fastcall(Ogre::MaterialTranslator *__hidden this)
#[doc(alias = "Ogre::MaterialTranslator::~MaterialTranslator()")]
// was: __ZN4Ogre18MaterialTranslatorD0Ev
// IDA 0xe1a744: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1a744() {
}

// 0xe1b794 — __ZN4Ogre3Any6holderIPNS_8MaterialEED1Ev
// type: void __fastcall(void)
#[doc(alias = "Ogre::Any::holder<Ogre::Material *>::~holder()")]
// was: __ZN4Ogre3Any6holderIPNS_8MaterialEED1Ev
// IDA 0xe1b794: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b794() {
}

// 0xe1b798 — __ZN4Ogre3Any6holderIPNS_8MaterialEED0Ev
// type: void __fastcall(void)
#[doc(alias = "Ogre::Any::holder<Ogre::Material *>::~holder()")]
// was: __ZN4Ogre3Any6holderIPNS_8MaterialEED0Ev
// IDA 0xe1b798: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b798() {
}

// 0xe1b7a4 — __ZNK4Ogre3Any6holderIPNS_8MaterialEE7getTypeEv
// type: void __fastcall(void)
#[doc(alias = "Ogre::Any::holder<Ogre::Material *>::getType(void)const")]
// was: __ZNK4Ogre3Any6holderIPNS_8MaterialEE7getTypeEv
// IDA 0xe1b7a4: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b7a4() {
}

// 0xe1b7b4 — __ZNK4Ogre3Any6holderIPNS_8MaterialEE5cloneEv
// type: void __fastcall(void)
#[doc(alias = "Ogre::Any::holder<Ogre::Material *>::clone(void)const")]
// was: __ZNK4Ogre3Any6holderIPNS_8MaterialEE5cloneEv
// IDA 0xe1b7b4: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b7b4() {
}

// 0xe1b7e4 — __ZN4Ogre3Any6holderIPNS_8MaterialEE13writeToStreamERSo
// type: void __fastcall(void)
#[doc(alias = "Ogre::Any::holder<Ogre::Material *>::writeToStream(std::ostream &)")]
// was: __ZN4Ogre3Any6holderIPNS_8MaterialEE13writeToStreamERSo
// IDA 0xe1b7e4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b7e4() {
}

// 0xe1ed00 — __ZN4Ogre12ShadowCaster20generateShadowVolumeEPNS_8EdgeDataERKNS_28HardwareIndexBufferSharedPtrEPKNS_5LightERSt6vectorIPNS_16ShadowRenderableENS_12STLAllocatorISB_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEm
// type: int __fastcall(int, int, int, Ogre::Light *this, int, int)
#[doc(alias = "Ogre::ShadowCaster::generateShadowVolume(Ogre::EdgeData *,Ogre::HardwareIndexBufferSharedPtr const&,Ogre::Light const*,std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &,unsigned long)")]
// was: __ZN4Ogre12ShadowCaster20generateShadowVolumeEPNS_8EdgeDataERKNS_28HardwareIndexBufferSharedPtrEPKNS_5LightERSt6vectorIPNS_16ShadowRenderableENS_12STLAllocatorISB_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEm
// IDA 0xe1ed00: 566 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1ed00() {
}

// 0xe1fbb0 — __ZNK4Ogre12ShadowCaster20getExtrusionDistanceERKNS_7Vector3EPKNS_5LightE
// type: _DWORD __fastcall(Ogre::ShadowCaster *__hidden this, const Vector3 *, const Ogre::Light *)
#[doc(alias = "Ogre::ShadowCaster::getExtrusionDistance(Ogre::Vector3 const&,Ogre::Light const*)const")]
// was: __ZNK4Ogre12ShadowCaster20getExtrusionDistanceERKNS_7Vector3EPKNS_5LightE
// IDA 0xe1fbb0: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1fbb0() {
}

// 0xe226a0 — __ZN4Ogre26ShadowVolumeExtrudeProgram16getProgramSourceENS_5Light10LightTypesESsbb
// type: int __fastcall(int, std::string *this)
#[doc(alias = "Ogre::ShadowVolumeExtrudeProgram::getProgramSource(Ogre::Light::LightTypes,std::string,bool,bool)")]
// was: __ZN4Ogre26ShadowVolumeExtrudeProgram16getProgramSourceENS_5Light10LightTypesESsbb
// IDA 0xe226a0: 435 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e226a0() {
}

// 0xe22ccc — __ZN4Ogre26ShadowVolumeExtrudeProgram14getProgramNameENS_5Light10LightTypesEbb
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "Ogre::ShadowVolumeExtrudeProgram::getProgramName(Ogre::Light::LightTypes,bool,bool)")]
// was: __ZN4Ogre26ShadowVolumeExtrudeProgram14getProgramNameENS_5Light10LightTypesEbb
// IDA 0xe22ccc: 37 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e22ccc() {
}

// 0xe30580 — __ZN4Ogre14StaticGeometry6Region33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm
// type: int __fastcall(int, int, int, bool, int, int, float, int)
#[doc(alias = "Ogre::StaticGeometry::Region::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")]
// was: __ZN4Ogre14StaticGeometry6Region33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm
// IDA 0xe30580: 100 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e30580() {
}

// 0xe3c25c — __ZN4Ogre9SubEntity11setMaterialERKNS_11MaterialPtrE
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this, const Ogre::MaterialPtr *)
#[doc(alias = "Ogre::SubEntity::setMaterial(Ogre::MaterialPtr const&)")]
// was: __ZN4Ogre9SubEntity11setMaterialERKNS_11MaterialPtrE
// IDA 0xe3c25c: 373 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3c25c() {
}

// 0xe3e1b8 — __ZN4Ogre9TechniqueC1EPNS_8MaterialE
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, Ogre::Material *)
#[doc(alias = "Ogre::Technique::Technique(Ogre::Material *)")]
// was: __ZN4Ogre9TechniqueC1EPNS_8MaterialE
// IDA 0xe3e1b8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3e1b8() {
}

// 0xe3e1c4 — __ZN4Ogre9TechniqueC2EPNS_8MaterialE
// type: _DWORD __fastcall(Ogre::Technique *__hidden this, Ogre::Material *)
#[doc(alias = "Ogre::Technique::Technique(Ogre::Material *)")]
// was: __ZN4Ogre9TechniqueC2EPNS_8MaterialE
// IDA 0xe3e1c4: 313 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3e1c4() {
}

// 0xe40d84 — __ZN4Ogre9Technique23setShadowCasterMaterialENS_11MaterialPtrE
// type: void __fastcall(void)
#[doc(alias = "Ogre::Technique::setShadowCasterMaterial(Ogre::MaterialPtr)")]
// was: __ZN4Ogre9Technique23setShadowCasterMaterialENS_11MaterialPtrE
// IDA 0xe40d84: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e40d84() {
}

// 0xe66948 — __ZN4Ogre15InstanceBatchHWC1EPNS_15InstanceManagerERNS_7MeshPtrERKNS_11MaterialPtrEmPKSt6vectorItNS_12STLAllocatorItNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEERKSs
// type: int __fastcall(int, int, int, int, char)
#[doc(alias = "Ogre::InstanceBatchHW::InstanceBatchHW(Ogre::InstanceManager *,Ogre::MeshPtr &,Ogre::MaterialPtr const&,unsigned long,std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,std::string const&)")]
// was: __ZN4Ogre15InstanceBatchHWC1EPNS_15InstanceManagerERNS_7MeshPtrERKNS_11MaterialPtrEmPKSt6vectorItNS_12STLAllocatorItNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEERKSs
// IDA 0xe66948: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e66948() {
}

// 0xe675d8 — __ZN4Ogre19InstanceBatchShaderC1EPNS_15InstanceManagerERNS_7MeshPtrERKNS_11MaterialPtrEmPKSt6vectorItNS_12STLAllocatorItNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEERKSs
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *, std::string *)
#[doc(alias = "Ogre::InstanceBatchShader::InstanceBatchShader(Ogre::InstanceManager *,Ogre::MeshPtr &,Ogre::MaterialPtr const&,unsigned long,std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,std::string const&)")]
// was: __ZN4Ogre19InstanceBatchShaderC1EPNS_15InstanceManagerERNS_7MeshPtrERKNS_11MaterialPtrEmPKSt6vectorItNS_12STLAllocatorItNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEERKSs
// IDA 0xe675d8: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e675d8() {
}

// 0xe68e54 — __ZN4Ogre20BaseInstanceBatchVTFC2EPNS_15InstanceManagerERNS_7MeshPtrERKNS_11MaterialPtrEmPKSt6vectorItNS_12STLAllocatorItNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEERKSs
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, struct _Unwind_Exception *, std::string *, int, int, int)
#[doc(alias = "Ogre::BaseInstanceBatchVTF::BaseInstanceBatchVTF(Ogre::InstanceManager *,Ogre::MeshPtr &,Ogre::MaterialPtr const&,unsigned long,std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,std::string const&)")]
// was: __ZN4Ogre20BaseInstanceBatchVTFC2EPNS_15InstanceManagerERNS_7MeshPtrERKNS_11MaterialPtrEmPKSt6vectorItNS_12STLAllocatorItNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEERKSs
// IDA 0xe68e54: 165 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e68e54() {
}

// 0xe6c4cc — __ZNSt3mapISsN4Ogre11MaterialPtrESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
// type: void __fastcall(void)
#[doc(alias = "std::map<std::string,Ogre::MaterialPtr,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: __ZNSt3mapISsN4Ogre11MaterialPtrESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
// IDA 0xe6c4cc: 259 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6c4cc() {
}

// 0xe6ca24 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::MaterialPtr>>,std::pair<std::string const,Ogre::MaterialPtr> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// IDA 0xe6ca24: 341 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6ca24() {
}

// 0xe6cd6c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::MaterialPtr> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
// IDA 0xe6cd6c: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6cd6c() {
}

// 0xe6cde0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::MaterialPtr> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// IDA 0xe6cde0: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6cde0() {
}

// 0xe6cec4 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_
// type: _DWORD *__fastcall(int, const std::string *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::MaterialPtr> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_
// IDA 0xe6cec4: 112 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6cec4() {
}

// 0xe6cff0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: void __fastcall(void)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// IDA 0xe6cff0: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6cff0() {
}

// 0xe6d094 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
// type: void __fastcall(void)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
// IDA 0xe6d094: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e6d094() {
}

// 0xe6d098 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
// type: void __fastcall(void)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
// IDA 0xe6d098: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e6d098() {
}

// 0xe6d0cc — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: void __fastcall(void)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::MaterialPtr>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// IDA 0xe6d0cc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6d0cc() {
}

// 0xe6d0fc — __ZN4Ogre12STLAllocatorISt4pairIKSsNS_11MaterialPtrEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS4_
// type: void __fastcall(void)
#[doc(alias = "Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::MaterialPtr>*)")]
// was: __ZN4Ogre12STLAllocatorISt4pairIKSsNS_11MaterialPtrEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS4_
// IDA 0xe6d0fc: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6d0fc() {
}

// 0xf4afc4 — j___ZN3G3D5ArrayIbLi10ELm32EE6appendERKb
// type: void __fastcall(void)
#[doc(alias = "G3D::Array<bool,10,32ul>::append(bool const&)")]
// was: j___ZN3G3D5ArrayIbLi10ELm32EE6appendERKb
// IDA 0xf4afc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4afc4() {
}

// 0xf4afd4 — j___ZN3G3D5ArrayIbLi10ELm32EEC2Ev
// type: void __fastcall(void)
#[doc(alias = "G3D::Array<bool,10,32ul>::Array(void)")]
// was: j___ZN3G3D5ArrayIbLi10ELm32EEC2Ev
// IDA 0xf4afd4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4afd4() {
}

// 0xf4afe4 — j___ZN3G3D5ArrayIbLi10ELm32EED2Ev
// type: void __fastcall(void)
#[doc(alias = "G3D::Array<bool,10,32ul>::~Array()")]
// was: j___ZN3G3D5ArrayIbLi10ELm32EED2Ev
// IDA 0xf4afe4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4afe4() {
}

// 0xf4c574 — j___ZN3G3D4Line13closestPointsERKS0_S2_RNS_7Vector3ES4_
// type: _DWORD __fastcall(G3D::Line *__hidden this, const G3D::Line *, const G3D::Line *, G3D::Vector3 *, G3D::Vector3 *)
#[doc(alias = "G3D::Line::closestPoints(G3D::Line const&,G3D::Line const&,G3D::Vector3 &,G3D::Vector3 &)")]
// was: j___ZN3G3D4Line13closestPointsERKS0_S2_RNS_7Vector3ES4_
// IDA 0xf4c574: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c574() {
}

// 0xf4ea64 — j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,G3D::Vector3int16 *>> *)")]
// was: j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// IDA 0xf4ea64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ea64() {
}

// 0xf57934 — j___ZNK3G3D12Vector3int1618isBetweenInclusiveERKS0_S2_
// type: _DWORD __fastcall(G3D::Vector3int16 *__hidden this, const G3D::Vector3int16 *, const G3D::Vector3int16 *)
#[doc(alias = "G3D::Vector3int16::isBetweenInclusive(G3D::Vector3int16 const&,G3D::Vector3int16 const&)const")]
// was: j___ZNK3G3D12Vector3int1618isBetweenInclusiveERKS0_S2_
// IDA 0xf57934: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f57934() {
}

// 0xf57eb4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3G3D7Vector3EPS4_EET0_T_S9_S8_
// type: void __fastcall(void)
#[doc(alias = "G3D::Vector3* std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 const*,G3D::Vector3*>(G3D::Vector3 const*,G3D::Vector3 const*,G3D::Vector3*)")]
// was: j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3G3D7Vector3EPS4_EET0_T_S9_S8_
// IDA 0xf57eb4: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_f57eb4() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xf57ec4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3G3D7Vector3ES5_EET0_T_S7_S6_
// type: void __fastcall(void)
#[doc(alias = "G3D::Vector3 * std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 *,G3D::Vector3 *>(G3D::Vector3 *,G3D::Vector3 *,G3D::Vector3 *)")]
// was: j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3G3D7Vector3ES5_EET0_T_S7_S6_
// IDA 0xf57ec4: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_f57ec4() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xf57ed4 — j___ZNSt6vectorIN3G3D7Vector3ESaIS1_EEaSERKS3_
// type: void __fastcall(void)
#[doc(alias = "std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>::operator=(std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&)")]
// was: j___ZNSt6vectorIN3G3D7Vector3ESaIS1_EEaSERKS3_
// IDA 0xf57ed4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f57ed4() {
}

// 0xf580a4 — j___ZNK3G3D5Plane15pointOnOrBehindENS_7Vector3E
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Plane::pointOnOrBehind(G3D::Vector3)const")]
// was: j___ZNK3G3D5Plane15pointOnOrBehindENS_7Vector3E
// IDA 0xf580a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f580a4() {
}

// 0xf58604 — j___ZN3G3D4Line13fromTwoPointsERKNS_7Vector3ES3_
// type: _DWORD __fastcall(G3D::Line *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "G3D::Line::fromTwoPoints(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: j___ZN3G3D4Line13fromTwoPointsERKNS_7Vector3ES3_
// IDA 0xf58604: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f58604() {
}

// 0xf58764 — j___ZNK3G3D4Line8distanceERKNS_7Vector3E
// type: _DWORD __fastcall(Vector3 *this, const G3D::Vector3 *)
#[doc(alias = "G3D::Line::distance(G3D::Vector3 const&)const")]
// was: j___ZNK3G3D4Line8distanceERKNS_7Vector3E
// IDA 0xf58764: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f58764() {
}

// 0xf58904 — j___ZNSt12_Vector_baseIN3G3D12Vector3int16ESaIS1_EE11_M_allocateEm
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Vector_base<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::_M_allocate(unsigned long)")]
// was: j___ZNSt12_Vector_baseIN3G3D12Vector3int16ESaIS1_EE11_M_allocateEm
// IDA 0xf58904: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_f58904() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0xf58924 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector3int16ES5_EET0_T_S7_S6_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "G3D::Vector3int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3int16 *,G3D::Vector3int16 *>(G3D::Vector3int16 *,G3D::Vector3int16 *,G3D::Vector3int16 *)")]
// was: j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector3int16ES5_EET0_T_S7_S6_
// IDA 0xf58924: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_f58924() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xf58944 — j___ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Vector3int16*,std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>>,unsigned long,G3D::Vector3int16 const&)")]
// was: j___ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// IDA 0xf58944: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f58944() {
}

// 0xf58954 — j___ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE6resizeEmS1_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::resize(unsigned long,G3D::Vector3int16)")]
// was: j___ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE6resizeEmS1_
// IDA 0xf58954: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f58954() {
}

// 0xf589e4 — j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_insert_unique(std::pair<int const,G3D::Vector3int16 *> const&)")]
// was: j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_
// IDA 0xf589e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f589e4() {
}

// 0xf589f4 — j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,G3D::Vector3int16 *> const&)")]
// was: j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// IDA 0xf589f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f589f4() {
}

// 0xf5b8c4 — j___ZNSt12_Vector_baseIN3G3D7Matrix3ESaIS1_EE11_M_allocateEm
// type: void __fastcall(void)
#[doc(alias = "std::_Vector_base<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_allocate(unsigned long)")]
// was: j___ZNSt12_Vector_baseIN3G3D7Matrix3ESaIS1_EE11_M_allocateEm
// IDA 0xf5b8c4: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_f5b8c4() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0xf5b904 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Matrix3ES5_EET0_T_S7_S6_
// type: void __fastcall(void)
#[doc(alias = "G3D::Matrix3 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Matrix3 *,G3D::Matrix3 *>(G3D::Matrix3 *,G3D::Matrix3 *,G3D::Matrix3 *)")]
// was: j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Matrix3ES5_EET0_T_S7_S6_
// IDA 0xf5b904: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_f5b904() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xf5b924 — j___ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int(void)
#[doc(alias = "std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Matrix3*,std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>>,unsigned long,G3D::Matrix3 const&)")]
// was: j___ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// IDA 0xf5b924: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5b924() {
}

// 0xf5b934 — j___ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE6resizeEmS1_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::resize(unsigned long,G3D::Matrix3)")]
// was: j___ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE6resizeEmS1_
// IDA 0xf5b934: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5b934() {
}

// 0xf5ba34 — j___ZSt26__uninitialized_fill_n_auxIPN3G3D7Matrix3EmS1_EvT_T0_RKT1_St12__false_type
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_aux<G3D::Matrix3 *,unsigned long,G3D::Matrix3>(G3D::Matrix3 *,unsigned long,G3D::Matrix3 const&,std::__false_type)")]
// was: j___ZSt26__uninitialized_fill_n_auxIPN3G3D7Matrix3EmS1_EvT_T0_RKT1_St12__false_type
// IDA 0xf5ba34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5ba34() {
}

// 0xf5dd74 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6resizeEib
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::resize(int,bool)")]
// was: j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6resizeEib
// IDA 0xf5dd74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5dd74() {
}

// 0xf5dd84 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::Array(void)")]
// was: j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EEC2Ev
// IDA 0xf5dd84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5dd84() {
}

// 0xf5e284 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6appendERKS1_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::append(G3D::Plane const&)")]
// was: j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6appendERKS1_
// IDA 0xf5e284: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e284() {
}

// 0xf5e694 — j___ZNK3G3D7Vector38isFiniteEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::isFinite(void)const")]
// was: j___ZNK3G3D7Vector38isFiniteEv
// IDA 0xf5e694: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e694() {
}

// 0xf64c74 — j___ZNK3G3D15CoordinateFrame13toObjectSpaceERKS0_
// type: _DWORD __fastcall(G3D::CoordinateFrame *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "G3D::CoordinateFrame::toObjectSpace(G3D::CoordinateFrame const&)const")]
// was: j___ZNK3G3D15CoordinateFrame13toObjectSpaceERKS0_
// IDA 0xf64c74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64c74() {
}

// 0xf65254 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6appendERKS3_
// type: void __fastcall(void)
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::append(Ogre::VertexStreamer::VertexChunk const&)")]
// was: j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6appendERKS3_
// IDA 0xf65254: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65254() {
}

// 0xf65264 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6resizeEib
// type: int()
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::resize(int,bool)")]
// was: j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6resizeEib
// IDA 0xf65264: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65264() {
}

// 0xf65274 — j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE7reallocEi
// type: void __fastcall(void)
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::realloc(int)")]
// was: j___ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE7reallocEi
// IDA 0xf65274: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65274() {
}
