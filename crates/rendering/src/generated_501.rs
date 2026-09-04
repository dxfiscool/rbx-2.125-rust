//! rendering — generated_501 — 100 stubs global dedup (rendering filtered, EA-sorted asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) NOT in /tmp/global_eas.txt — next 100 uncovered EA-sorted asc 0xe20d10..0xe34080 (2076 candidates remaining, 92102 global EAs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr). Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xe20d10 — __ZN4Ogre20ShadowTextureManager11clearUnusedEv
// type: _DWORD __fastcall(Ogre::ShadowTextureManager *__hidden this)
#[doc(alias = "Ogre::ShadowTextureManager::clearUnused(void)")]
#[doc(alias = "__ZN4Ogre20ShadowTextureManager11clearUnusedEv")]
// was: Ogre::ShadowTextureManager::clearUnused(void)
// IDA 0xe20d10: 122 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe20d10() {
}

// 0xe20e50 — __ZN4Ogre20ShadowTextureManager5clearEv
// type: _DWORD __fastcall(Ogre::ShadowTextureManager *__hidden this)
#[doc(alias = "Ogre::ShadowTextureManager::clear(void)")]
#[doc(alias = "__ZN4Ogre20ShadowTextureManager5clearEv")]
// was: Ogre::ShadowTextureManager::clear(void)
// IDA 0xe20e50: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe20e50() {
}

// 0xe20eb4 — __ZNSt8_Rb_treeIPN4Ogre7TextureES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Texture *>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeIPN4Ogre7TextureES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev")]
// was: std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Texture *>,false>::~_Rb_tree_impl()
// IDA 0xe20eb4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xe20eb4() {
}

// 0xe20eb8 — __ZNSt8_Rb_treeIPN4Ogre7TextureES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Texture *>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeIPN4Ogre7TextureES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev")]
// was: std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Texture *>,false>::~_Rb_tree_impl()
// IDA 0xe20eb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe20eb8() {
}

// 0xe20ec4 — __ZNSt8_Rb_treeIPN4Ogre7TextureES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Texture *> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN4Ogre7TextureES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// was: std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Texture *> *)
// IDA 0xe20ec4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe20ec4() {
}

// 0xe20f20 — __ZN4Ogre26ShadowVolumeExtrudeProgram10initialiseEv
// type: _DWORD __fastcall(Ogre::ShadowVolumeExtrudeProgram *__hidden this)
#[doc(alias = "Ogre::ShadowVolumeExtrudeProgram::initialise(void)")]
#[doc(alias = "__ZN4Ogre26ShadowVolumeExtrudeProgram10initialiseEv")]
// was: Ogre::ShadowVolumeExtrudeProgram::initialise(void)
// IDA 0xe20f20: 2195 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe20f20() {
}

// 0xe226a0 — __ZN4Ogre26ShadowVolumeExtrudeProgram16getProgramSourceENS_5Light10LightTypesESsbb
// type: int __fastcall(int, std::string *this)
#[doc(alias = "Ogre::ShadowVolumeExtrudeProgram::getProgramSource(Ogre::Light::LightTypes,std::string,bool,bool)")]
#[doc(alias = "__ZN4Ogre26ShadowVolumeExtrudeProgram16getProgramSourceENS_5Light10LightTypesESsbb")]
// was: Ogre::ShadowVolumeExtrudeProgram::getProgramSource(Ogre::Light::LightTypes,std::string,bool,bool)
// IDA 0xe226a0: 435 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe226a0() {
}

// 0xe22c20 — __ZN4Ogre26ShadowVolumeExtrudeProgram8shutdownEv
// type: _DWORD __fastcall(Ogre::ShadowVolumeExtrudeProgram *__hidden this)
#[doc(alias = "Ogre::ShadowVolumeExtrudeProgram::shutdown(void)")]
#[doc(alias = "__ZN4Ogre26ShadowVolumeExtrudeProgram8shutdownEv")]
// was: Ogre::ShadowVolumeExtrudeProgram::shutdown(void)
// IDA 0xe22c20: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe22c20() {
}

// 0xe22ccc — __ZN4Ogre26ShadowVolumeExtrudeProgram14getProgramNameENS_5Light10LightTypesEbb
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "Ogre::ShadowVolumeExtrudeProgram::getProgramName(Ogre::Light::LightTypes,bool,bool)")]
#[doc(alias = "__ZN4Ogre26ShadowVolumeExtrudeProgram14getProgramNameENS_5Light10LightTypesEbb")]
// was: Ogre::ShadowVolumeExtrudeProgram::getProgramName(Ogre::Light::LightTypes,bool,bool)
// IDA 0xe22ccc: 37 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe22ccc() {
}

// 0xe238c4 — __ZN4Ogre16SimpleRenderableC2Ev
// type: _DWORD __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "Ogre::SimpleRenderable::SimpleRenderable(void)")]
#[doc(alias = "__ZN4Ogre16SimpleRenderableC2Ev")]
// was: Ogre::SimpleRenderable::SimpleRenderable(void)
// IDA 0xe238c4: 637 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe238c4() {
}

// 0xe2400c — __ZN4Ogre16SimpleRenderable11setMaterialERKSs
// type: _DWORD __fastcall(Ogre::SimpleRenderable *__hidden this, const std::string *)
#[doc(alias = "Ogre::SimpleRenderable::setMaterial(std::string const&)")]
#[doc(alias = "__ZN4Ogre16SimpleRenderable11setMaterialERKSs")]
// was: Ogre::SimpleRenderable::setMaterial(std::string const&)
// IDA 0xe2400c: 356 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2400c() {
}

// 0xe243d4 — __ZNK4Ogre16SimpleRenderable11getMaterialEv
// type: _DWORD __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "Ogre::SimpleRenderable::getMaterial(void)const")]
#[doc(alias = "__ZNK4Ogre16SimpleRenderable11getMaterialEv")]
// was: Ogre::SimpleRenderable::getMaterial(void)const
// IDA 0xe243d4: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe243d4() {
}

// 0xe243dc — __ZThn188_NK4Ogre16SimpleRenderable11getMaterialEv
// type: _DWORD __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::getMaterial(void)const")]
#[doc(alias = "__ZThn188_NK4Ogre16SimpleRenderable11getMaterialEv")]
// was: `non-virtual thunk toOgre::SimpleRenderable::getMaterial(void)const
// IDA 0xe243dc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe243dc() {
}

// 0xe243e0 — __ZN4Ogre16SimpleRenderable18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::SimpleRenderable::getRenderOperation(Ogre::RenderOperation &)")]
#[doc(alias = "__ZN4Ogre16SimpleRenderable18getRenderOperationERNS_15RenderOperationE")]
// was: Ogre::SimpleRenderable::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xe243e0: 8 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe243e0() {
}

// 0xe243fc — __ZThn188_N4Ogre16SimpleRenderable18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::getRenderOperation(Ogre::RenderOperation &)")]
#[doc(alias = "__ZThn188_N4Ogre16SimpleRenderable18getRenderOperationERNS_15RenderOperationE")]
// was: `non-virtual thunk toOgre::SimpleRenderable::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xe243fc: 8 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe243fc() {
}

// 0xe24418 — __ZN4Ogre16SimpleRenderable18setRenderOperationERKNS_15RenderOperationE
#[doc(alias = "Ogre::SimpleRenderable::setRenderOperation(Ogre::RenderOperation const&)")]
#[doc(alias = "__ZN4Ogre16SimpleRenderable18setRenderOperationERKNS_15RenderOperationE")]
// was: Ogre::SimpleRenderable::setRenderOperation(Ogre::RenderOperation const&)
// IDA 0xe24418: 8 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe24418() {
}

// 0xe244f4 — __ZN4Ogre16SimpleRenderable20_notifyCurrentCameraEPNS_6CameraE
// type: _DWORD __fastcall(Ogre::SimpleRenderable *__hidden this, Ogre::Camera *)
#[doc(alias = "Ogre::SimpleRenderable::_notifyCurrentCamera(Ogre::Camera *)")]
#[doc(alias = "__ZN4Ogre16SimpleRenderable20_notifyCurrentCameraEPNS_6CameraE")]
// was: Ogre::SimpleRenderable::_notifyCurrentCamera(Ogre::Camera *)
// IDA 0xe244f4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe244f4() {
}

// 0xe24508 — __ZN4Ogre16SimpleRenderable14setBoundingBoxERKNS_14AxisAlignedBoxE
#[doc(alias = "Ogre::SimpleRenderable::setBoundingBox(Ogre::AxisAlignedBox const&)")]
#[doc(alias = "__ZN4Ogre16SimpleRenderable14setBoundingBoxERKNS_14AxisAlignedBoxE")]
// was: Ogre::SimpleRenderable::setBoundingBox(Ogre::AxisAlignedBox const&)
// IDA 0xe24508: 26 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe24508() {
}

// 0xe24550 — __ZNK4Ogre16SimpleRenderable14getBoundingBoxEv
// type: _DWORD __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "Ogre::SimpleRenderable::getBoundingBox(void)const")]
#[doc(alias = "__ZNK4Ogre16SimpleRenderable14getBoundingBoxEv")]
// was: Ogre::SimpleRenderable::getBoundingBox(void)const
// IDA 0xe24550: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe24550() {
}

// 0xe24558 — __ZN4Ogre16SimpleRenderable18_updateRenderQueueEPNS_11RenderQueueE
// type: _DWORD __fastcall(Ogre::SimpleRenderable *__hidden this, Ogre::RenderQueue *)
#[doc(alias = "Ogre::SimpleRenderable::_updateRenderQueue(Ogre::RenderQueue *)")]
#[doc(alias = "__ZN4Ogre16SimpleRenderable18_updateRenderQueueEPNS_11RenderQueueE")]
// was: Ogre::SimpleRenderable::_updateRenderQueue(Ogre::RenderQueue *)
// IDA 0xe24558: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe24558() {
}

// 0xe24570 — __ZN4Ogre16SimpleRenderable16visitRenderablesEPNS_10Renderable7VisitorEb
#[doc(alias = "Ogre::SimpleRenderable::visitRenderables(Ogre::Renderable::Visitor *,bool)")]
#[doc(alias = "__ZN4Ogre16SimpleRenderable16visitRenderablesEPNS_10Renderable7VisitorEb")]
// was: Ogre::SimpleRenderable::visitRenderables(Ogre::Renderable::Visitor *,bool)
// IDA 0xe24570: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe24570() {
}

// 0xe24594 — __ZN4Ogre16SimpleRenderableD0Ev
// type: void __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "Ogre::SimpleRenderable::~SimpleRenderable()")]
#[doc(alias = "__ZN4Ogre16SimpleRenderableD0Ev")]
// was: Ogre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe24594: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe24594() {
}

// 0xe24624 — __ZN4Ogre16SimpleRenderableD1Ev
// type: void __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "Ogre::SimpleRenderable::~SimpleRenderable()")]
#[doc(alias = "__ZN4Ogre16SimpleRenderableD1Ev")]
// was: Ogre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe24624: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe24624() {
}

// 0xe24630 — __ZThn4_N4Ogre16SimpleRenderableD0Ev
// type: void __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::~SimpleRenderable()")]
#[doc(alias = "__ZThn4_N4Ogre16SimpleRenderableD0Ev")]
// was: `non-virtual thunk toOgre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe24630: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe24630() {
}

// 0xe246c4 — __ZThn188_N4Ogre16SimpleRenderableD0Ev
// type: void __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::~SimpleRenderable()")]
#[doc(alias = "__ZThn188_N4Ogre16SimpleRenderableD0Ev")]
// was: `non-virtual thunk toOgre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe246c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe246c4() {
}

// 0xe24758 — __ZN4Ogre16SimpleRenderableD2Ev
// type: void __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "Ogre::SimpleRenderable::~SimpleRenderable()")]
#[doc(alias = "__ZN4Ogre16SimpleRenderableD2Ev")]
// was: Ogre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe24758: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe24758() {
}

// 0xe24918 — __ZThn4_N4Ogre16SimpleRenderableD1Ev
// type: void __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::~SimpleRenderable()")]
#[doc(alias = "__ZThn4_N4Ogre16SimpleRenderableD1Ev")]
// was: `non-virtual thunk toOgre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe24918: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe24918() {
}

// 0xe24924 — __ZThn188_N4Ogre16SimpleRenderableD1Ev
// type: void __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::~SimpleRenderable()")]
#[doc(alias = "__ZThn188_N4Ogre16SimpleRenderableD1Ev")]
// was: `non-virtual thunk toOgre::SimpleRenderable::~SimpleRenderable()
// IDA 0xe24924: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe24924() {
}

// 0xe24930 — __ZNK4Ogre16SimpleRenderable14getMovableTypeEv
// type: _DWORD __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "Ogre::SimpleRenderable::getMovableType(void)const")]
#[doc(alias = "__ZNK4Ogre16SimpleRenderable14getMovableTypeEv")]
// was: Ogre::SimpleRenderable::getMovableType(void)const
// IDA 0xe24930: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe24930() {
}

// 0xe24a24 — __ZNK4Ogre16SimpleRenderable9getLightsEv
// type: _DWORD __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "Ogre::SimpleRenderable::getLights(void)const")]
#[doc(alias = "__ZNK4Ogre16SimpleRenderable9getLightsEv")]
// was: Ogre::SimpleRenderable::getLights(void)const
// IDA 0xe24a24: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe24a24() {
}

// 0xe24a34 — __ZThn188_NK4Ogre16SimpleRenderable9getLightsEv
// type: _DWORD __fastcall(Ogre::SimpleRenderable *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::SimpleRenderable::getLights(void)const")]
#[doc(alias = "__ZThn188_NK4Ogre16SimpleRenderable9getLightsEv")]
// was: `non-virtual thunk toOgre::SimpleRenderable::getLights(void)const
// IDA 0xe24a34: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe24a34() {
}

// 0xe24a78 — __ZN4Ogre12SimpleSplineC1Ev
// type: _DWORD __fastcall(Ogre::SimpleSpline *__hidden this)
#[doc(alias = "Ogre::SimpleSpline::SimpleSpline(void)")]
#[doc(alias = "__ZN4Ogre12SimpleSplineC1Ev")]
// was: Ogre::SimpleSpline::SimpleSpline(void)
// IDA 0xe24a78: 40 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe24a78() {
}

// 0xe24ae4 — __ZN4Ogre12SimpleSplineD1Ev
// type: void __fastcall(Ogre::SimpleSpline *__hidden this)
#[doc(alias = "Ogre::SimpleSpline::~SimpleSpline()")]
#[doc(alias = "__ZN4Ogre12SimpleSplineD1Ev")]
// was: Ogre::SimpleSpline::~SimpleSpline()
// IDA 0xe24ae4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe24ae4() {
}

// 0xe24b88 — __ZN4Ogre12SimpleSpline8addPointERKNS_7Vector3E
// type: _DWORD __fastcall(Ogre::SimpleSpline *__hidden this, const Ogre::Vector3 *)
#[doc(alias = "Ogre::SimpleSpline::addPoint(Ogre::Vector3 const&)")]
#[doc(alias = "__ZN4Ogre12SimpleSpline8addPointERKNS_7Vector3E")]
// was: Ogre::SimpleSpline::addPoint(Ogre::Vector3 const&)
// IDA 0xe24b88: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe24b88() {
}

// 0xe2dbe0 — __ZN4Ogre14StaticGeometry9addEntityEPNS_6EntityERKNS_7Vector3ERKNS_10QuaternionES5_
// type: _DWORD __fastcall(Ogre::StaticGeometry *__hidden this, Ogre::Entity *, const Ogre::Vector3 *, const Ogre::Quaternion *, const Ogre::Vector3 *)
#[doc(alias = "Ogre::StaticGeometry::addEntity(Ogre::Entity *,Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::Vector3 const&)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry9addEntityEPNS_6EntityERKNS_7Vector3ERKNS_10QuaternionES5_")]
// was: Ogre::StaticGeometry::addEntity(Ogre::Entity *,Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::Vector3 const&)
// IDA 0xe2dbe0: 356 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2dbe0() {
}

// 0xe2df88 — __ZN4Ogre14StaticGeometry17determineGeometryEPNS_7SubMeshE
// type: _DWORD __fastcall(Ogre::StaticGeometry *__hidden this, Ogre::SubMesh *)
#[doc(alias = "Ogre::StaticGeometry::determineGeometry(Ogre::SubMesh *)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry17determineGeometryEPNS_7SubMeshE")]
// was: Ogre::StaticGeometry::determineGeometry(Ogre::SubMesh *)
// IDA 0xe2df88: 151 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2df88() {
}

// 0xe2e108 — __ZN4Ogre14StaticGeometry13splitGeometryEPNS_10VertexDataEPNS_9IndexDataEPNS0_22SubMeshLodGeometryLinkE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, char, int, int, int, int, int, int, int, int, int, int, int, in
#[doc(alias = "Ogre::StaticGeometry::splitGeometry(Ogre::VertexData *,Ogre::IndexData *,Ogre::StaticGeometry::SubMeshLodGeometryLink *)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry13splitGeometryEPNS_10VertexDataEPNS_9IndexDataEPNS0_22SubMeshLodGeometryLinkE")]
// was: Ogre::StaticGeometry::splitGeometry(Ogre::VertexData *,Ogre::IndexData *,Ogre::StaticGeometry::SubMeshLodGeometryLink *)
// IDA 0xe2e108: 832 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2e108() {
}

// 0xe2e8dc — __ZN4Ogre14StaticGeometry12addSceneNodeEPKNS_9SceneNodeE
// type: _DWORD __fastcall(Ogre::StaticGeometry *__hidden this, const Ogre::SceneNode *)
#[doc(alias = "Ogre::StaticGeometry::addSceneNode(Ogre::SceneNode const*)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry12addSceneNodeEPKNS_9SceneNodeE")]
// was: Ogre::StaticGeometry::addSceneNode(Ogre::SceneNode const*)
// IDA 0xe2e8dc: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2e8dc() {
}

// 0xe2e9a8 — __ZN4Ogre14StaticGeometry5buildEv
// type: _DWORD __fastcall(Ogre::StaticGeometry *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::build(void)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry5buildEv")]
// was: Ogre::StaticGeometry::build(void)
// IDA 0xe2e9a8: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2e9a8() {
}

// 0xe2ea28 — __ZN4Ogre14StaticGeometry6Region6assignEPNS0_13QueuedSubMeshE
#[doc(alias = "Ogre::StaticGeometry::Region::assign(Ogre::StaticGeometry::QueuedSubMesh *)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry6Region6assignEPNS0_13QueuedSubMeshE")]
// was: Ogre::StaticGeometry::Region::assign(Ogre::StaticGeometry::QueuedSubMesh *)
// IDA 0xe2ea28: 356 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2ea28() {
}

// 0xe2ee70 — __ZN4Ogre14StaticGeometry6Region5buildEb
// type: _DWORD __fastcall(Ogre::StaticGeometry::Region *__hidden this, bool)
#[doc(alias = "Ogre::StaticGeometry::Region::build(bool)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry6Region5buildEb")]
// was: Ogre::StaticGeometry::Region::build(bool)
// IDA 0xe2ee70: 129 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2ee70() {
}

// 0xe2efe8 — __ZN4Ogre14StaticGeometry7destroyEv
// type: _DWORD __fastcall(Ogre::StaticGeometry *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::destroy(void)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry7destroyEv")]
// was: Ogre::StaticGeometry::destroy(void)
// IDA 0xe2efe8: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2efe8() {
}

// 0xe2f040 — __ZN4Ogre14StaticGeometry5resetEv
// type: _DWORD __fastcall(Ogre::StaticGeometry *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::reset(void)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry5resetEv")]
// was: Ogre::StaticGeometry::reset(void)
// IDA 0xe2f040: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2f040() {
}

// 0xe2f234 — __ZN4Ogre14StaticGeometry10setVisibleEb
// type: _DWORD __fastcall(Ogre::StaticGeometry *__hidden this, bool)
#[doc(alias = "Ogre::StaticGeometry::setVisible(bool)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry10setVisibleEb")]
// was: Ogre::StaticGeometry::setVisible(bool)
// IDA 0xe2f234: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2f234() {
}

// 0xe2f264 — __ZN4Ogre14StaticGeometry14setCastShadowsEb
// type: _DWORD __fastcall(Ogre::StaticGeometry *__hidden this, bool)
#[doc(alias = "Ogre::StaticGeometry::setCastShadows(bool)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14setCastShadowsEb")]
// was: Ogre::StaticGeometry::setCastShadows(bool)
// IDA 0xe2f264: 15 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2f264() {
}

// 0xe2f28c — __ZN4Ogre14StaticGeometry19setRenderQueueGroupEh
// type: _DWORD __fastcall(Ogre::StaticGeometry *__hidden this, unsigned __int8)
#[doc(alias = "Ogre::StaticGeometry::setRenderQueueGroup(unsigned char)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry19setRenderQueueGroupEh")]
// was: Ogre::StaticGeometry::setRenderQueueGroup(unsigned char)
// IDA 0xe2f28c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2f28c() {
}

// 0xe2f2c0 — __ZNK4Ogre14StaticGeometry19getRenderQueueGroupEv
// type: _DWORD __fastcall(Ogre::StaticGeometry *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::getRenderQueueGroup(void)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry19getRenderQueueGroupEv")]
// was: Ogre::StaticGeometry::getRenderQueueGroup(void)const
// IDA 0xe2f2c0: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2f2c0() {
}

// 0xe2f2c8 — __ZNK4Ogre14StaticGeometry4dumpERKSs
// type: _DWORD __fastcall(Ogre::StaticGeometry *__hidden this, const std::string *)
#[doc(alias = "Ogre::StaticGeometry::dump(std::string const&)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry4dumpERKSs")]
// was: Ogre::StaticGeometry::dump(std::string const&)const
// IDA 0xe2f2c8: 528 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2f2c8() {
}

// 0xe2f8ac — __ZNK4Ogre14StaticGeometry6Region4dumpERSt14basic_ofstreamIcSt11char_traitsIcEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::StaticGeometry::Region::dump(std::basic_ofstream<char,std::char_traits<char>> &)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry6Region4dumpERSt14basic_ofstreamIcSt11char_traitsIcEE")]
// was: Ogre::StaticGeometry::Region::dump(std::basic_ofstream<char,std::char_traits<char>> &)const
// IDA 0xe2f8ac: 303 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2f8ac() {
}

// 0xe2fc38 — __ZN4Ogre14StaticGeometry6RegionD0Ev
// type: void __fastcall(Ogre::StaticGeometry::Region *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::Region::~Region()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry6RegionD0Ev")]
// was: Ogre::StaticGeometry::Region::~Region()
// IDA 0xe2fc38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe2fc38() {
}

// 0xe2fcc8 — __ZN4Ogre14StaticGeometry6RegionD1Ev
// type: void __fastcall(Ogre::StaticGeometry::Region *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::Region::~Region()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry6RegionD1Ev")]
// was: Ogre::StaticGeometry::Region::~Region()
// IDA 0xe2fcc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe2fcc8() {
}

// 0xe2fcd4 — __ZThn4_N4Ogre14StaticGeometry6RegionD0Ev
// type: void __fastcall(Ogre::StaticGeometry::Region *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::StaticGeometry::Region::~Region()")]
#[doc(alias = "__ZThn4_N4Ogre14StaticGeometry6RegionD0Ev")]
// was: `non-virtual thunk toOgre::StaticGeometry::Region::~Region()
// IDA 0xe2fcd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe2fcd4() {
}

// 0xe2fd68 — __ZN4Ogre14StaticGeometry6RegionD2Ev
// type: void __fastcall(Ogre::StaticGeometry::Region *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::Region::~Region()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry6RegionD2Ev")]
// was: Ogre::StaticGeometry::Region::~Region()
// IDA 0xe2fd68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe2fd68() {
}

// 0xe2ff6c — __ZThn4_N4Ogre14StaticGeometry6RegionD1Ev
// type: void __fastcall(Ogre::StaticGeometry::Region *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::StaticGeometry::Region::~Region()")]
#[doc(alias = "__ZThn4_N4Ogre14StaticGeometry6RegionD1Ev")]
// was: `non-virtual thunk toOgre::StaticGeometry::Region::~Region()
// IDA 0xe2ff6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe2ff6c() {
}

// 0xe2ff78 — __ZNK4Ogre14StaticGeometry6Region12getTypeFlagsEv
// type: _DWORD __fastcall(Ogre::StaticGeometry::Region *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::Region::getTypeFlags(void)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry6Region12getTypeFlagsEv")]
// was: Ogre::StaticGeometry::Region::getTypeFlags(void)const
// IDA 0xe2ff78: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2ff78() {
}

// 0xe2ff88 — __ZN4Ogre14StaticGeometry9LODBucket6assignEPNS0_13QueuedSubMeshEt
// type: int __fastcall(int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::StaticGeometry::LODBucket::assign(Ogre::StaticGeometry::QueuedSubMesh *,unsigned short)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry9LODBucket6assignEPNS0_13QueuedSubMeshEt")]
// was: Ogre::StaticGeometry::LODBucket::assign(Ogre::StaticGeometry::QueuedSubMesh *,unsigned short)
// IDA 0xe2ff88: 210 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe2ff88() {
}

// 0xe301b0 — __ZN4Ogre14StaticGeometry9LODBucket5buildEb
// type: _DWORD __fastcall(Ogre::StaticGeometry::LODBucket *__hidden this, bool)
#[doc(alias = "Ogre::StaticGeometry::LODBucket::build(bool)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry9LODBucket5buildEb")]
// was: Ogre::StaticGeometry::LODBucket::build(bool)
// IDA 0xe301b0: 147 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe301b0() {
}

// 0xe30324 — __ZNK4Ogre14StaticGeometry6Region14getMovableTypeEv
// type: _DWORD __fastcall(Ogre::StaticGeometry::Region *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::Region::getMovableType(void)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry6Region14getMovableTypeEv")]
// was: Ogre::StaticGeometry::Region::getMovableType(void)const
// IDA 0xe30324: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe30324() {
}

// 0xe30418 — __ZN4Ogre14StaticGeometry6Region20_notifyCurrentCameraEPNS_6CameraE
// type: _DWORD __fastcall(Ogre::StaticGeometry::Region *__hidden this, Ogre::Camera *)
#[doc(alias = "Ogre::StaticGeometry::Region::_notifyCurrentCamera(Ogre::Camera *)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry6Region20_notifyCurrentCameraEPNS_6CameraE")]
// was: Ogre::StaticGeometry::Region::_notifyCurrentCamera(Ogre::Camera *)
// IDA 0xe30418: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe30418() {
}

// 0xe30470 — __ZNK4Ogre14StaticGeometry6Region14getBoundingBoxEv
// type: _DWORD __fastcall(Ogre::StaticGeometry::Region *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::Region::getBoundingBox(void)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry6Region14getBoundingBoxEv")]
// was: Ogre::StaticGeometry::Region::getBoundingBox(void)const
// IDA 0xe30470: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe30470() {
}

// 0xe30474 — __ZNK4Ogre14StaticGeometry6Region17getBoundingRadiusEv
// type: _DWORD __fastcall(Ogre::StaticGeometry::Region *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::Region::getBoundingRadius(void)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry6Region17getBoundingRadiusEv")]
// was: Ogre::StaticGeometry::Region::getBoundingRadius(void)const
// IDA 0xe30474: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe30474() {
}

// 0xe3047c — __ZN4Ogre14StaticGeometry6Region18_updateRenderQueueEPNS_11RenderQueueE
// type: _DWORD __fastcall(Ogre::StaticGeometry::Region *__hidden this, Ogre::RenderQueue *)
#[doc(alias = "Ogre::StaticGeometry::Region::_updateRenderQueue(Ogre::RenderQueue *)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry6Region18_updateRenderQueueEPNS_11RenderQueueE")]
// was: Ogre::StaticGeometry::Region::_updateRenderQueue(Ogre::RenderQueue *)
// IDA 0xe3047c: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3047c() {
}

// 0xe304c8 — __ZN4Ogre14StaticGeometry6Region16visitRenderablesEPNS_10Renderable7VisitorEb
#[doc(alias = "Ogre::StaticGeometry::Region::visitRenderables(Ogre::Renderable::Visitor *,bool)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry6Region16visitRenderablesEPNS_10Renderable7VisitorEb")]
// was: Ogre::StaticGeometry::Region::visitRenderables(Ogre::Renderable::Visitor *,bool)
// IDA 0xe304c8: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe304c8() {
}

// 0xe30548 — __ZNK4Ogre14StaticGeometry6Region9isVisibleEv
// type: _DWORD __fastcall(Ogre::StaticGeometry::Region *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::Region::isVisible(void)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry6Region9isVisibleEv")]
// was: Ogre::StaticGeometry::Region::isVisible(void)const
// IDA 0xe30548: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe30548() {
}

// 0xe30aa0 — __ZN4Ogre14StaticGeometry6Region11getEdgeListEv
// type: _DWORD __fastcall(Ogre::StaticGeometry::Region *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::Region::getEdgeList(void)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry6Region11getEdgeListEv")]
// was: Ogre::StaticGeometry::Region::getEdgeList(void)
// IDA 0xe30aa0: 5 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe30aa0() {
}

// 0xe30ab0 — __ZN4Ogre14StaticGeometry6Region11hasEdgeListEv
// type: _DWORD __fastcall(Ogre::StaticGeometry::Region *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::Region::hasEdgeList(void)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry6Region11hasEdgeListEv")]
// was: Ogre::StaticGeometry::Region::hasEdgeList(void)
// IDA 0xe30ab0: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe30ab0() {
}

// 0xe30ac4 — __ZNK4Ogre14StaticGeometry9LODBucket4dumpERSt14basic_ofstreamIcSt11char_traitsIcEE
#[doc(alias = "Ogre::StaticGeometry::LODBucket::dump(std::basic_ofstream<char,std::char_traits<char>> &)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry9LODBucket4dumpERSt14basic_ofstreamIcSt11char_traitsIcEE")]
// was: Ogre::StaticGeometry::LODBucket::dump(std::basic_ofstream<char,std::char_traits<char>> &)const
// IDA 0xe30ac4: 104 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe30ac4() {
}

// 0xe31028 — __ZN4Ogre14StaticGeometry9LODBucket19LODShadowRenderableD0Ev
// type: void __fastcall(Ogre::StaticGeometry::LODBucket::LODShadowRenderable *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::LODBucket::LODShadowRenderable::~LODShadowRenderable()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry9LODBucket19LODShadowRenderableD0Ev")]
// was: Ogre::StaticGeometry::LODBucket::LODShadowRenderable::~LODShadowRenderable()
// IDA 0xe31028: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe31028() {
}

// 0xe310b8 — __ZN4Ogre14StaticGeometry9LODBucket19LODShadowRenderableD1Ev
// type: void __fastcall(Ogre::StaticGeometry::LODBucket::LODShadowRenderable *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::LODBucket::LODShadowRenderable::~LODShadowRenderable()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry9LODBucket19LODShadowRenderableD1Ev")]
// was: Ogre::StaticGeometry::LODBucket::LODShadowRenderable::~LODShadowRenderable()
// IDA 0xe310b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe310b8() {
}

// 0xe310c4 — __ZN4Ogre14StaticGeometry9LODBucket19LODShadowRenderableD2Ev
// type: void __fastcall(Ogre::StaticGeometry::LODBucket::LODShadowRenderable *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::LODBucket::LODShadowRenderable::~LODShadowRenderable()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry9LODBucket19LODShadowRenderableD2Ev")]
// was: Ogre::StaticGeometry::LODBucket::LODShadowRenderable::~LODShadowRenderable()
// IDA 0xe310c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe310c4() {
}

// 0xe312d0 — __ZN4Ogre14StaticGeometry9LODBucketD0Ev
// type: void __fastcall(Ogre::StaticGeometry::LODBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::LODBucket::~LODBucket()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry9LODBucketD0Ev")]
// was: Ogre::StaticGeometry::LODBucket::~LODBucket()
// IDA 0xe312d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe312d0() {
}

// 0xe31360 — __ZN4Ogre14StaticGeometry9LODBucketD1Ev
// type: void __fastcall(Ogre::StaticGeometry::LODBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::LODBucket::~LODBucket()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry9LODBucketD1Ev")]
// was: Ogre::StaticGeometry::LODBucket::~LODBucket()
// IDA 0xe31360: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe31360() {
}

// 0xe3136c — __ZN4Ogre14StaticGeometry9LODBucketD2Ev
// type: void __fastcall(Ogre::StaticGeometry::LODBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::LODBucket::~LODBucket()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry9LODBucketD2Ev")]
// was: Ogre::StaticGeometry::LODBucket::~LODBucket()
// IDA 0xe3136c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe3136c() {
}

// 0xe3153c — __ZN4Ogre14StaticGeometry14MaterialBucket6assignEPNS0_14QueuedGeometryE
#[doc(alias = "Ogre::StaticGeometry::MaterialBucket::assign(Ogre::StaticGeometry::QueuedGeometry *)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14MaterialBucket6assignEPNS0_14QueuedGeometryE")]
// was: Ogre::StaticGeometry::MaterialBucket::assign(Ogre::StaticGeometry::QueuedGeometry *)
// IDA 0xe3153c: 291 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe3153c() {
}

// 0xe31868 — __ZN4Ogre14StaticGeometry14MaterialBucket5buildEb
// type: _DWORD __fastcall(Ogre::StaticGeometry::MaterialBucket *__hidden this, bool)
#[doc(alias = "Ogre::StaticGeometry::MaterialBucket::build(bool)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14MaterialBucket5buildEb")]
// was: Ogre::StaticGeometry::MaterialBucket::build(bool)
// IDA 0xe31868: 434 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe31868() {
}

// 0xe31d0c — __ZN4Ogre14StaticGeometry14MaterialBucket14addRenderablesEPNS_11RenderQueueEhf
// type: _DWORD __fastcall(Ogre::StaticGeometry::MaterialBucket *__hidden this, Ogre::RenderQueue *, unsigned __int8, float)
#[doc(alias = "Ogre::StaticGeometry::MaterialBucket::addRenderables(Ogre::RenderQueue *,unsigned char,float)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14MaterialBucket14addRenderablesEPNS_11RenderQueueEhf")]
// was: Ogre::StaticGeometry::MaterialBucket::addRenderables(Ogre::RenderQueue *,unsigned char,float)
// IDA 0xe31d0c: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe31d0c() {
}

// 0xe31d84 — __ZNK4Ogre14StaticGeometry14MaterialBucket4dumpERSt14basic_ofstreamIcSt11char_traitsIcEE
#[doc(alias = "Ogre::StaticGeometry::MaterialBucket::dump(std::basic_ofstream<char,std::char_traits<char>> &)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry14MaterialBucket4dumpERSt14basic_ofstreamIcSt11char_traitsIcEE")]
// was: Ogre::StaticGeometry::MaterialBucket::dump(std::basic_ofstream<char,std::char_traits<char>> &)const
// IDA 0xe31d84: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe31d84() {
}

// 0xe31e74 — __ZN4Ogre14StaticGeometry14MaterialBucketD0Ev
// type: void __fastcall(Ogre::StaticGeometry::MaterialBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::MaterialBucket::~MaterialBucket()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14MaterialBucketD0Ev")]
// was: Ogre::StaticGeometry::MaterialBucket::~MaterialBucket()
// IDA 0xe31e74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe31e74() {
}

// 0xe31f04 — __ZN4Ogre14StaticGeometry14MaterialBucketD1Ev
// type: void __fastcall(Ogre::StaticGeometry::MaterialBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::MaterialBucket::~MaterialBucket()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14MaterialBucketD1Ev")]
// was: Ogre::StaticGeometry::MaterialBucket::~MaterialBucket()
// IDA 0xe31f04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe31f04() {
}

// 0xe31f10 — __ZN4Ogre14StaticGeometry14MaterialBucketD2Ev
// type: void __fastcall(Ogre::StaticGeometry::MaterialBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::MaterialBucket::~MaterialBucket()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14MaterialBucketD2Ev")]
// was: Ogre::StaticGeometry::MaterialBucket::~MaterialBucket()
// IDA 0xe31f10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe31f10() {
}

// 0xe32090 — __ZN4Ogre14StaticGeometry14MaterialBucket23getGeometryFormatStringEPNS0_22SubMeshLodGeometryLinkE
#[doc(alias = "Ogre::StaticGeometry::MaterialBucket::getGeometryFormatString(Ogre::StaticGeometry::SubMeshLodGeometryLink *)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14MaterialBucket23getGeometryFormatStringEPNS0_22SubMeshLodGeometryLinkE")]
// was: Ogre::StaticGeometry::MaterialBucket::getGeometryFormatString(Ogre::StaticGeometry::SubMeshLodGeometryLink *)
// IDA 0xe32090: 252 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe32090() {
}

// 0xe32364 — __ZN4Ogre14StaticGeometry14GeometryBucket6assignEPNS0_14QueuedGeometryE
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::assign(Ogre::StaticGeometry::QueuedGeometry *)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14GeometryBucket6assignEPNS0_14QueuedGeometryE")]
// was: Ogre::StaticGeometry::GeometryBucket::assign(Ogre::StaticGeometry::QueuedGeometry *)
// IDA 0xe32364: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe32364() {
}

// 0xe323d8 — __ZN4Ogre14StaticGeometry14GeometryBucket5buildEb
// type: _DWORD __fastcall(Ogre::StaticGeometry::GeometryBucket *__hidden this, bool)
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::build(bool)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14GeometryBucket5buildEb")]
// was: Ogre::StaticGeometry::GeometryBucket::build(bool)
// IDA 0xe323d8: 1036 insns (PUSH..TBH.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe323d8() {
}

// 0xe33118 — __ZNK4Ogre14StaticGeometry14GeometryBucket4dumpERSt14basic_ofstreamIcSt11char_traitsIcEE
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::dump(std::basic_ofstream<char,std::char_traits<char>> &)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry14GeometryBucket4dumpERSt14basic_ofstreamIcSt11char_traitsIcEE")]
// was: Ogre::StaticGeometry::GeometryBucket::dump(std::basic_ofstream<char,std::char_traits<char>> &)const
// IDA 0xe33118: 131 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe33118() {
}

// 0xe33290 — __ZN4Ogre14StaticGeometry14GeometryBucketC2EPNS0_14MaterialBucketERKSsPKNS_10VertexDataEPKNS_9IndexDataE
// type: _DWORD __fastcall(Ogre::StaticGeometry::GeometryBucket *__hidden this, Ogre::StaticGeometry::MaterialBucket *, const std::string *, const Ogre::VertexData *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::GeometryBucket(Ogre::StaticGeometry::MaterialBucket *,std::string const&,Ogre::VertexData const*,Ogre::IndexData const*)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14GeometryBucketC2EPNS0_14MaterialBucketERKSsPKNS_10VertexDataEPKNS_9IndexDataE")]
// was: Ogre::StaticGeometry::GeometryBucket::GeometryBucket(Ogre::StaticGeometry::MaterialBucket *,std::string const&,Ogre::VertexData const*,Ogre::IndexData const*)
// IDA 0xe33290: 280 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe33290() {
}

// 0xe33554 — __ZN4Ogre14StaticGeometry14GeometryBucketD0Ev
// type: void __fastcall(Ogre::StaticGeometry::GeometryBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::~GeometryBucket()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14GeometryBucketD0Ev")]
// was: Ogre::StaticGeometry::GeometryBucket::~GeometryBucket()
// IDA 0xe33554: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe33554() {
}

// 0xe335e4 — __ZN4Ogre14StaticGeometry14GeometryBucketD1Ev
// type: void __fastcall(Ogre::StaticGeometry::GeometryBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::~GeometryBucket()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14GeometryBucketD1Ev")]
// was: Ogre::StaticGeometry::GeometryBucket::~GeometryBucket()
// IDA 0xe335e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe335e4() {
}

// 0xe335f0 — __ZN4Ogre14StaticGeometry14GeometryBucketD2Ev
// type: void __fastcall(Ogre::StaticGeometry::GeometryBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::~GeometryBucket()")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14GeometryBucketD2Ev")]
// was: Ogre::StaticGeometry::GeometryBucket::~GeometryBucket()
// IDA 0xe335f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe335f0() {
}

// 0xe33738 — __ZNK4Ogre14StaticGeometry14GeometryBucket11getMaterialEv
// type: _DWORD __fastcall(Ogre::StaticGeometry::GeometryBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::getMaterial(void)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry14GeometryBucket11getMaterialEv")]
// was: Ogre::StaticGeometry::GeometryBucket::getMaterial(void)const
// IDA 0xe33738: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe33738() {
}

// 0xe33740 — __ZNK4Ogre14StaticGeometry14GeometryBucket12getTechniqueEv
// type: _DWORD __fastcall(Ogre::StaticGeometry::GeometryBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::getTechnique(void)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry14GeometryBucket12getTechniqueEv")]
// was: Ogre::StaticGeometry::GeometryBucket::getTechnique(void)const
// IDA 0xe33740: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe33740() {
}

// 0xe33748 — __ZN4Ogre14StaticGeometry14GeometryBucket18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::getRenderOperation(Ogre::RenderOperation &)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry14GeometryBucket18getRenderOperationERNS_15RenderOperationE")]
// was: Ogre::StaticGeometry::GeometryBucket::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xe33748: 10 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe33748() {
}

// 0xe337f0 — __ZNK4Ogre14StaticGeometry14GeometryBucket9getLightsEv
// type: _DWORD __fastcall(Ogre::StaticGeometry::GeometryBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::getLights(void)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry14GeometryBucket9getLightsEv")]
// was: Ogre::StaticGeometry::GeometryBucket::getLights(void)const
// IDA 0xe337f0: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe337f0() {
}

// 0xe33804 — __ZNK4Ogre14StaticGeometry14GeometryBucket15getCastsShadowsEv
// type: _DWORD __fastcall(Ogre::StaticGeometry::GeometryBucket *__hidden this)
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::getCastsShadows(void)const")]
#[doc(alias = "__ZNK4Ogre14StaticGeometry14GeometryBucket15getCastsShadowsEv")]
// was: Ogre::StaticGeometry::GeometryBucket::getCastsShadows(void)const
// IDA 0xe33804: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe33804() {
}

// 0xe33818 — __ZNK4Ogre14AxisAlignedBox12intersectionERKS0_
#[doc(alias = "Ogre::AxisAlignedBox::intersection(Ogre::AxisAlignedBox const&)const")]
#[doc(alias = "__ZNK4Ogre14AxisAlignedBox12intersectionERKS0_")]
// was: Ogre::AxisAlignedBox::intersection(Ogre::AxisAlignedBox const&)const
// IDA 0xe33818: 203 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe33818() {
}

// 0xe33a70 — __ZN4OgrelsERSoNS_14AxisAlignedBoxE
// type: int(void)
#[doc(alias = "Ogre::operator<<(std::ostream &,Ogre::AxisAlignedBox)")]
#[doc(alias = "__ZN4OgrelsERSoNS_14AxisAlignedBoxE")]
// was: Ogre::operator<<(std::ostream &,Ogre::AxisAlignedBox)
// IDA 0xe33a70: 111 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe33a70() {
}

// 0xe33bb8 — __ZNSt3mapISsPN4Ogre14StaticGeometry14MaterialBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
// type: int(void)
#[doc(alias = "std::map<std::string,Ogre::StaticGeometry::MaterialBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsPN4Ogre14StaticGeometry14MaterialBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_")]
// was: std::map<std::string,Ogre::StaticGeometry::MaterialBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xe33bb8: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe33bb8() {
}

// 0xe33d74 — __ZNSt3mapISsPN4Ogre14StaticGeometry14GeometryBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
// type: int(void)
#[doc(alias = "std::map<std::string,Ogre::StaticGeometry::GeometryBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsPN4Ogre14StaticGeometry14GeometryBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_")]
// was: std::map<std::string,Ogre::StaticGeometry::GeometryBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xe33d74: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe33d74() {
}

// 0xe33f30 — __ZN4Ogre12STLAllocatorISt4listINS_13VertexElementENS0_IS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES5_ED1Ev
#[doc(alias = "Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
#[doc(alias = "__ZN4Ogre12STLAllocatorISt4listINS_13VertexElementENS0_IS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES5_ED1Ev")]
// was: Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xe33f30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xe33f30() {
}

// 0xe33f34 — __ZNSt6vectorISt4listIN4Ogre13VertexElementENS1_12STLAllocatorIS2_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEENS3_IS8_S6_EEE9push_backERKS8_
#[doc(alias = "std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "__ZNSt6vectorISt4listIN4Ogre13VertexElementENS1_12STLAllocatorIS2_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEENS3_IS8_S6_EEE9push_backERKS8_")]
// was: std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAl
// IDA 0xe33f34: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_0xe33f34() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0xe34080 — __ZN4Ogre14StaticGeometry20setRenderingDistanceEf
// type: _DWORD __fastcall(Ogre::StaticGeometry *__hidden this, float)
#[doc(alias = "Ogre::StaticGeometry::setRenderingDistance(float)")]
#[doc(alias = "__ZN4Ogre14StaticGeometry20setRenderingDistanceEf")]
// was: Ogre::StaticGeometry::setRenderingDistance(float)
// IDA 0xe34080: 6 insns (VMOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe34080() {
}
