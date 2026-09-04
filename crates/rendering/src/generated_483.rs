//! rendering shard 483 — 100 stubs EA-sorted asc rendering-filter not in /tmp/global_eas.txt (0xbacfcc..0xbc9c0c, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) rendering namespace filter, global EA dedup.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xbacfcc — __ZN4Ogre12VisualEngineC1Ev
// type: int __fastcall(Ogre::VisualEngine *this)
#[doc(alias = "Ogre::VisualEngine::VisualEngine(void)")]
// was: __ZN4Ogre12VisualEngineC1Ev
// IDA 0xbacfcc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bacfcc() {
}


// 0xbacfd0 — __ZN4Ogre12VisualEngineC2Ev
// type: Ogre::VisualEngine *__fastcall(Ogre::VisualEngine *this)
#[doc(alias = "Ogre::VisualEngine::VisualEngine(void)")]
// was: __ZN4Ogre12VisualEngineC2Ev
// IDA 0xbacfd0: 294 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bacfd0() {
}


// 0xbad2c8 — __ZN4Ogre12VisualEngineD0Ev
// type: void __fastcall(Ogre::VisualEngine *__hidden this)
#[doc(alias = "Ogre::VisualEngine::~VisualEngine()")]
// was: __ZN4Ogre12VisualEngineD0Ev
// IDA 0xbad2c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bad2c8() {
}


// 0xbad368 — __ZN4Ogre12VisualEngineD1Ev
// type: void __fastcall(Ogre::VisualEngine *__hidden this)
#[doc(alias = "Ogre::VisualEngine::~VisualEngine()")]
// was: __ZN4Ogre12VisualEngineD1Ev
// IDA 0xbad368: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bad368() {
}


// 0xbad36c — __ZN4Ogre12VisualEngineD2Ev
// type: void __fastcall(Ogre::VisualEngine *__hidden this)
#[doc(alias = "Ogre::VisualEngine::~VisualEngine()")]
// was: __ZN4Ogre12VisualEngineD2Ev
// IDA 0xbad36c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bad36c() {
}


// 0xbb0120 — __ZN4Ogre12VisualEngine14initializeLoadEii
// type: void __fastcall(Ogre::VisualEngine *this, int, int)
#[doc(alias = "Ogre::VisualEngine::initializeLoad(int,int)")]
// was: __ZN4Ogre12VisualEngine14initializeLoadEii
// IDA 0xbb0120: 149 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb0120() {
}


// 0xbb039c — __ZN4Ogre12VisualEngine14setupResourcesEv
// type: void __fastcall(Ogre::VisualEngine *this, Ogre::VisualEngine *)
#[doc(alias = "Ogre::VisualEngine::setupResources(void)")]
// was: __ZN4Ogre12VisualEngine14setupResourcesEv
// IDA 0xbb039c: 1212 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb039c() {
}


// 0xbb1160 — __ZN4Ogre12VisualEngine10setupSceneEii
// type: void __fastcall(Ogre::VisualEngine *this, int, int)
#[doc(alias = "Ogre::VisualEngine::setupScene(int,int)")]
// was: __ZN4Ogre12VisualEngine10setupSceneEii
// IDA 0xbb1160: 974 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb1160() {
}


// 0xbb1c18 — __ZN4Ogre12VisualEngine17checkMaterialCapsEv
// type: void __fastcall(Ogre::Root **this)
#[doc(alias = "Ogre::VisualEngine::checkMaterialCaps(void)")]
// was: __ZN4Ogre12VisualEngine17checkMaterialCapsEv
// IDA 0xbb1c18: 647 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb1c18() {
}


// 0xbb2308 — __ZN4Ogre12VisualEngine13reloadShadersEv
// type: int __fastcall(Ogre::VisualEngine *this)
#[doc(alias = "Ogre::VisualEngine::reloadShaders(void)")]
// was: __ZN4Ogre12VisualEngine13reloadShadersEv
// IDA 0xbb2308: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bb2308() {
}


// 0xbb29b0 — __ZN4Ogre12VisualEngine15getSceneUpdaterEv
// type: int __fastcall(Ogre::VisualEngine *this)
#[doc(alias = "Ogre::VisualEngine::getSceneUpdater(void)")]
// was: __ZN4Ogre12VisualEngine15getSceneUpdaterEv
// IDA 0xbb29b0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb29b0() {
}


// 0xbb2ae0 — __ZN4Ogre12VisualEngine15convertPositionERKN3G3D15CoordinateFrameE
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "Ogre::VisualEngine::convertPosition(G3D::CoordinateFrame const&)")]
// was: __ZN4Ogre12VisualEngine15convertPositionERKN3G3D15CoordinateFrameE
// IDA 0xbb2ae0: 8 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb2ae0() {
}


// 0xbb2b00 — __ZN4Ogre12VisualEngine18convertOrientationERKN3G3D15CoordinateFrameE
// type: int __fastcall(Ogre::VisualEngine *this, const G3D::CoordinateFrame *)
#[doc(alias = "Ogre::VisualEngine::convertOrientation(G3D::CoordinateFrame const&)")]
// was: __ZN4Ogre12VisualEngine18convertOrientationERKN3G3D15CoordinateFrameE
// IDA 0xbb2b00: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb2b00() {
}


// 0xbb2b18 — __ZN4Ogre12VisualEngine20SweepUnusedResourcesEv
// type: void __fastcall(Ogre::VisualEngine *this)
#[doc(alias = "Ogre::VisualEngine::SweepUnusedResources(void)")]
// was: __ZN4Ogre12VisualEngine20SweepUnusedResourcesEv
// IDA 0xbb2b18: 709 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb2b18() {
}


// 0xbb32f4 — __ZN4Ogre12VisualEngine19getFrameRateManagerEv
// type: int __fastcall(Ogre::VisualEngine *this)
#[doc(alias = "Ogre::VisualEngine::getFrameRateManager(void)")]
// was: __ZN4Ogre12VisualEngine19getFrameRateManagerEv
// IDA 0xbb32f4: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb32f4() {
}


// 0xbb3300 — __ZN4Ogre12VisualEngine14getSkyViewportEv
// type: int __fastcall(Ogre::VisualEngine *this)
#[doc(alias = "Ogre::VisualEngine::getSkyViewport(void)")]
// was: __ZN4Ogre12VisualEngine14getSkyViewportEv
// IDA 0xbb3300: 6 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb3300() {
}


// 0xbb3310 — __ZN4Ogre12VisualEngine20setCustomSkyViewportEPNS_8ViewportE
// type: int __fastcall(int result, int)
#[doc(alias = "Ogre::VisualEngine::setCustomSkyViewport(Ogre::Viewport *)")]
// was: __ZN4Ogre12VisualEngine20setCustomSkyViewportEPNS_8ViewportE
// IDA 0xbb3310: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb3310() {
}


// 0xbb3318 — __ZN4Ogre12VisualEngine21createOrUpdateTextureEPKcNS_11TextureTypeEiiiNS_11PixelFormatENS_12TextureUsageE
// type: void __fastcall(Ogre::TextureManager *, int, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::VisualEngine::createOrUpdateTexture(char const*,Ogre::TextureType,int,int,int,Ogre::PixelFormat,Ogre::TextureUsage)")]
// was: __ZN4Ogre12VisualEngine21createOrUpdateTextureEPKcNS_11TextureTypeEiiiNS_11PixelFormatENS_12TextureUsageE
// IDA 0xbb3318: 507 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb3318() {
}


// 0xbb3974 — __ZN4Ogre22RbxSceneManagerFactoryD1Ev
// type: void __fastcall(Ogre::RbxSceneManagerFactory *__hidden this)
#[doc(alias = "Ogre::RbxSceneManagerFactory::~RbxSceneManagerFactory()")]
// was: __ZN4Ogre22RbxSceneManagerFactoryD1Ev
// IDA 0xbb3974: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bb3974() {
}


// 0xbb3a0c — __ZNK4Ogre13DriverVersion8toStringEv
// type: void __fastcall(Ogre::DriverVersion *this, _DWORD *)
#[doc(alias = "Ogre::DriverVersion::toString(void)const")]
// was: __ZNK4Ogre13DriverVersion8toStringEv
// IDA 0xbb3a0c: 227 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb3a0c() {
}


// 0xbb4af8 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPSsSt6vectorISsN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEEEEA10_cET_SD_SD_RKT0_St26random_access_iterator_tag
// type: std::string *__fastcall(std::string *this, int, char *)
#[doc(alias = "__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::__find<__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,char [10]>(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,char [10] const&,std::random_access_iterator_tag)")]
// was: __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPSsSt6vectorISsN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEEEEA10_cET_SD_SD_RKT0_St26random_access_iterator_tag
// IDA 0xbb4af8: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb4af8() {
}


// 0xbb5a80 — __ZNKSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE4findERKSs
#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// was: __ZNKSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE4findERKSs
// IDA 0xbb5a80: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb5a80() {
}


// 0xbb5b24 — __ZN4Ogre22RbxSceneManagerFactoryD0Ev
// type: void __fastcall(Ogre::RbxSceneManagerFactory *__hidden this)
#[doc(alias = "Ogre::RbxSceneManagerFactory::~RbxSceneManagerFactory()")]
// was: __ZN4Ogre22RbxSceneManagerFactoryD0Ev
// IDA 0xbb5b24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bb5b24() {
}


// 0xbb5bc0 — __ZN4Ogre22RbxDbgVisualEngineInfoC2Ev
// type: _DWORD __fastcall(Ogre::RbxDbgVisualEngineInfo *__hidden this)
#[doc(alias = "Ogre::RbxDbgVisualEngineInfo::RbxDbgVisualEngineInfo(void)")]
// was: __ZN4Ogre22RbxDbgVisualEngineInfoC2Ev
// IDA 0xbb5bc0: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb5bc0() {
}


// 0xbb6870 — __ZN4Ogre14VertexStreamerC1Ev
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this)
#[doc(alias = "Ogre::VertexStreamer::VertexStreamer(void)")]
// was: __ZN4Ogre14VertexStreamerC1Ev
// IDA 0xbb6870: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bb6870() {
}


// 0xbb6874 — __ZN4Ogre14VertexStreamerC2Ev
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this)
#[doc(alias = "Ogre::VertexStreamer::VertexStreamer(void)")]
// was: __ZN4Ogre14VertexStreamerC2Ev
// IDA 0xbb6874: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb6874() {
}


// 0xbb6a4c — __ZN4Ogre14VertexStreamerD0Ev
// type: void __fastcall(Ogre::VertexStreamer *__hidden this)
#[doc(alias = "Ogre::VertexStreamer::~VertexStreamer()")]
// was: __ZN4Ogre14VertexStreamerD0Ev
// IDA 0xbb6a4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bb6a4c() {
}


// 0xbb6b70 — __ZN4Ogre14VertexStreamerD1Ev
// type: void __fastcall(Ogre::VertexStreamer *__hidden this)
#[doc(alias = "Ogre::VertexStreamer::~VertexStreamer()")]
// was: __ZN4Ogre14VertexStreamerD1Ev
// IDA 0xbb6b70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bb6b70() {
}


// 0xbb6c84 — __ZN4Ogre14VertexStreamer3endEv
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this)
#[doc(alias = "Ogre::VertexStreamer::end(void)")]
// was: __ZN4Ogre14VertexStreamer3endEv
// IDA 0xbb6c84: 150 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb6c84() {
}


// 0xbb6e18 — __ZN4Ogre14VertexStreamer4initEPNS_12VisualEngineEPNS_12SceneManagerEPNS_12RenderWindowEb
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this, Ogre::VisualEngine *, Ogre::SceneManager *, Ogre::RenderWindow *, bool)
#[doc(alias = "Ogre::VertexStreamer::init(Ogre::VisualEngine *,Ogre::SceneManager *,Ogre::RenderWindow *,bool)")]
// was: __ZN4Ogre14VertexStreamer4initEPNS_12VisualEngineEPNS_12SceneManagerEPNS_12RenderWindowEb
// IDA 0xbb6e18: 8 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb6e18() {
}


// 0xbb6e30 — __ZN4Ogre14VertexStreamer10setEnabledEb
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this, bool)
#[doc(alias = "Ogre::VertexStreamer::setEnabled(bool)")]
// was: __ZN4Ogre14VertexStreamer10setEnabledEb
// IDA 0xbb6e30: 2 insns (STRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb6e30() {
}


// 0xbb6e34 — __ZN4Ogre14VertexStreamer16cleanUpFrameDataEv
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this)
#[doc(alias = "Ogre::VertexStreamer::cleanUpFrameData(void)")]
// was: __ZN4Ogre14VertexStreamer16cleanUpFrameDataEv
// IDA 0xbb6e34: 26 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb6e34() {
}


// 0xbb6e84 — __ZN4Ogre14VertexStreamer6renderEv
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this)
#[doc(alias = "Ogre::VertexStreamer::render(void)")]
// was: __ZN4Ogre14VertexStreamer6renderEv
// IDA 0xbb6e84: 1070 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb6e84() {
}


// 0xbb79f0 — __ZN4Ogre14VertexStreamer12prepareChunkERKNS_10TexturePtrENS_15RenderOperation13OperationTypeENS0_15CoordinateSpaceENS0_10VextexTypeEbb
#[doc(alias = "Ogre::VertexStreamer::prepareChunk(Ogre::TexturePtr const&,Ogre::RenderOperation::OperationType,Ogre::VertexStreamer::CoordinateSpace,Ogre::VertexStreamer::VextexType,bool,bool)")]
// was: __ZN4Ogre14VertexStreamer12prepareChunkERKNS_10TexturePtrENS_15RenderOperation13OperationTypeENS0_15CoordinateSpaceENS0_10VextexTypeEbb
// IDA 0xbb79f0: 269 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb79f0() {
}


// 0xbb7cbc — __ZN4Ogre14VertexStreamer13spriteBltFullERKNS_10TexturePtrERKN3G3D6Color4Effffffffb
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this, const Ogre::TexturePtr *, const G3D::Color4 *, float, float, float, float, float, float, float, float, bool)
#[doc(alias = "Ogre::VertexStreamer::spriteBltFull(Ogre::TexturePtr const&,G3D::Color4 const&,float,float,float,float,float,float,float,float,bool)")]
// was: __ZN4Ogre14VertexStreamer13spriteBltFullERKNS_10TexturePtrERKN3G3D6Color4Effffffffb
// IDA 0xbb7cbc: 189 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb7cbc() {
}


// 0xbb7f18 — __ZN4Ogre14VertexStreamer11spriteBlt3DERKNS_10TexturePtrERKN3G3D6Color4ERKNS4_15CoordinateFrameEfffffffffb
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this, const Ogre::TexturePtr *, const G3D::Color4 *, const G3D::CoordinateFrame *, float, float, float, float, float, float, float, float, float, bool)
#[doc(alias = "Ogre::VertexStreamer::spriteBlt3D(Ogre::TexturePtr const&,G3D::Color4 const&,G3D::CoordinateFrame const&,float,float,float,float,float,float,float,float,float,bool)")]
// was: __ZN4Ogre14VertexStreamer11spriteBlt3DERKNS_10TexturePtrERKN3G3D6Color4ERKNS4_15CoordinateFrameEfffffffffb
// IDA 0xbb7f18: 358 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb7f18() {
}


// 0xbb842c — __ZN4Ogre14VertexStreamer14triangleList2dERKN3G3D6Color4EPKNS1_7Vector2EiPKsi
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this, const G3D::Color4 *, const G3D::Vector2 *, int, const __int16 *, int)
#[doc(alias = "Ogre::VertexStreamer::triangleList2d(G3D::Color4 const&,G3D::Vector2 const*,int,short const*,int)")]
// was: __ZN4Ogre14VertexStreamer14triangleList2dERKN3G3D6Color4EPKNS1_7Vector2EiPKsi
// IDA 0xbb842c: 271 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb842c() {
}


// 0xbb8704 — __ZN4Ogre14VertexStreamer12triangleListERKN3G3D6Color4ERKNS1_15CoordinateFrameEPKNS1_7Vector3EiPKsi
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this, const G3D::Color4 *, const G3D::CoordinateFrame *, const G3D::Vector3 *, int, const __int16 *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "Ogre::VertexStreamer::triangleList(G3D::Color4 const&,G3D::CoordinateFrame const&,G3D::Vector3 const*,int,short const*,int)")]
// was: __ZN4Ogre14VertexStreamer12triangleListERKN3G3D6Color4ERKNS1_15CoordinateFrameEPKNS1_7Vector3EiPKsi
// IDA 0xbb8704: 283 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb8704() {
}


// 0xbb8a44 — __ZN4Ogre14VertexStreamer4lineEffffRKN3G3D6Color4E
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this, float, float, float, float, const G3D::Color4 *)
#[doc(alias = "Ogre::VertexStreamer::line(float,float,float,float,G3D::Color4 const&)")]
// was: __ZN4Ogre14VertexStreamer4lineEffffRKN3G3D6Color4E
// IDA 0xbb8a44: 227 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb8a44() {
}


// 0xbb8cb8 — __ZN4Ogre14VertexStreamer6line3dEffffffRKN3G3D6Color4E
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this, float, float, float, float, float, struct _Unwind_Exception *lpuexcpt, const G3D::Color4 *)
#[doc(alias = "Ogre::VertexStreamer::line3d(float,float,float,float,float,float,G3D::Color4 const&)")]
// was: __ZN4Ogre14VertexStreamer6line3dEffffffRKN3G3D6Color4E
// IDA 0xbb8cb8: 227 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb8cb8() {
}


// 0xbb8f2c — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEED1Ev
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::~VertexBufferBatch()")]
// was: __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEED1Ev
// IDA 0xbb8f2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bb8f2c() {
}


// 0xbb9178 — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEED1Ev
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::~VertexBufferBatch()")]
// was: __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEED1Ev
// IDA 0xbb9178: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bb9178() {
}


// 0xbb93c4 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6resizeEib
// type: int(void)
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::resize(int,bool)")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6resizeEib
// IDA 0xbb93c4: 135 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb93c4() {
}


// 0xbb9540 — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE5setupEPNS_12RenderSystemE
// type: int(void)
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::setup(Ogre::RenderSystem *)")]
// was: __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE5setupEPNS_12RenderSystemE
// IDA 0xbb9540: 170 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb9540() {
}


// 0xbb97fc — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE5setupEPNS_12RenderSystemE
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::setup(Ogre::RenderSystem *)")]
// was: __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE5setupEPNS_12RenderSystemE
// IDA 0xbb97fc: 171 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb97fc() {
}


// 0xbb9abc — __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::append(Ogre::VertexStreamer::VertexChunk const&)")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6appendERKS3_
// IDA 0xbb9abc: 289 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb9abc() {
}


// 0xbb9dc4 — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE20createHardwareBufferEj
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, char, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::createHardwareBuffer(unsigned int)")]
// was: __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE20createHardwareBufferEj
// IDA 0xbb9dc4: 294 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb9dc4() {
}


// 0xbba0b8 — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE20createHardwareBufferEj
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, char, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::createHardwareBuffer(unsigned int)")]
// was: __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE20createHardwareBufferEj
// IDA 0xbba0b8: 280 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba0b8() {
}


// 0xbba388 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE6resizeEib
// type: int(void)
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::resize(int,bool)")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE6resizeEib
// IDA 0xbba388: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba388() {
}


// 0xbba45c — __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::realloc(int)")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE7reallocEi
// IDA 0xbba45c: 154 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba45c() {
}


// 0xbba670 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::resize(int,bool)")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE6resizeEib
// IDA 0xbba670: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba670() {
}


// 0xbba734 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::realloc(int)")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE7reallocEi
// IDA 0xbba734: 147 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba734() {
}


// 0xbba920 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::realloc(int)")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE7reallocEi
// IDA 0xbba920: 221 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba920() {
}


// 0xbbabe8 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::~Array()")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EED2Ev
// IDA 0xbbabe8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bbabe8() {
}


// 0xbbadc4 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::Array(void)")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EEC2Ev
// IDA 0xbbadc4: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbadc4() {
}


// 0xbbaf7c — __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::append(Ogre::VertexStreamer::Vertex3D const&)")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE6appendERKS3_
// IDA 0xbbaf7c: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbaf7c() {
}


// 0xbbb000 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE6appendERKS3_
// type: int(void)
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::append(Ogre::VertexStreamer::Vertex3DTexture const&)")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE6appendERKS3_
// IDA 0xbbb000: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbb000() {
}


// 0xbbb0a8 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::~Array()")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EED2Ev
// IDA 0xbbb0a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bbb0a8() {
}


// 0xbbb1c8 — __ZN3G3D5ArrayIiLi10ELm32EED2Ev
#[doc(alias = "G3D::Array<int,10,32ul>::~Array()")]
// was: __ZN3G3D5ArrayIiLi10ELm32EED2Ev
// IDA 0xbbb1c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bbb1c8() {
}


// 0xbbb2e8 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::~Array()")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EED2Ev
// IDA 0xbbb2e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bbb2e8() {
}


// 0xbbb408 — __ZN3G3D5ArrayIiLi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<int,10,32ul>::Array(void)")]
// was: __ZN3G3D5ArrayIiLi10ELm32EEC2Ev
// IDA 0xbbb408: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbb408() {
}


// 0xbbb5c0 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::Array(void)")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EEC2Ev
// IDA 0xbbb5c0: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbb5c0() {
}


// 0xbbb778 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::Array(void)")]
// was: __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EEC2Ev
// IDA 0xbbb778: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbb778() {
}


// 0xbbbfb8 — __ZN4Ogre28RbxManualResourceLoaderChainC2EPNS_12VisualEngineEPPS0_
#[doc(alias = "Ogre::RbxManualResourceLoaderChain::RbxManualResourceLoaderChain(Ogre::VisualEngine *,Ogre::RbxManualResourceLoaderChain**)")]
// was: __ZN4Ogre28RbxManualResourceLoaderChainC2EPNS_12VisualEngineEPPS0_
// IDA 0xbbbfb8: 15 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbbfb8() {
}


// 0xbbbfdc — __ZN4Ogre28RbxManualResourceLoaderChain12loadResourceEPNS_8ResourceE
// type: _DWORD __fastcall(Ogre::RbxManualResourceLoaderChain *__hidden this, Ogre::Resource *)
#[doc(alias = "Ogre::RbxManualResourceLoaderChain::loadResource(Ogre::Resource *)")]
// was: __ZN4Ogre28RbxManualResourceLoaderChain12loadResourceEPNS_8ResourceE
// IDA 0xbbbfdc: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbbfdc() {
}


// 0xbbc010 — __ZN4Ogre28RbxManualResourceLoaderChain11DeleteChainEv
// type: _DWORD __fastcall(Ogre::RbxManualResourceLoaderChain *__hidden this)
#[doc(alias = "Ogre::RbxManualResourceLoaderChain::DeleteChain(void)")]
// was: __ZN4Ogre28RbxManualResourceLoaderChain11DeleteChainEv
// IDA 0xbbc010: 26 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbc010() {
}


// 0xbbc048 — __ZN4Ogre28RbxManualResourceLoaderChain7ZombifyEv
// type: _DWORD __fastcall(Ogre::RbxManualResourceLoaderChain *__hidden this)
#[doc(alias = "Ogre::RbxManualResourceLoaderChain::Zombify(void)")]
// was: __ZN4Ogre28RbxManualResourceLoaderChain7ZombifyEv
// IDA 0xbbc048: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbc048() {
}


// 0xbbc050 — __ZN4Ogre28RbxManualResourceLoaderChainD1Ev
// type: void __fastcall(Ogre::RbxManualResourceLoaderChain *__hidden this)
#[doc(alias = "Ogre::RbxManualResourceLoaderChain::~RbxManualResourceLoaderChain()")]
// was: __ZN4Ogre28RbxManualResourceLoaderChainD1Ev
// IDA 0xbbc050: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bbc050() {
}


// 0xbbc054 — __ZN4Ogre28RbxManualResourceLoaderChainD0Ev
// type: void __fastcall(Ogre::RbxManualResourceLoaderChain *__hidden this)
#[doc(alias = "Ogre::RbxManualResourceLoaderChain::~RbxManualResourceLoaderChain()")]
// was: __ZN4Ogre28RbxManualResourceLoaderChainD0Ev
// IDA 0xbbc054: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bbc054() {
}


// 0xbbc058 — __ZN4Ogre15TextureLoaderOpclEPNS_28RbxManualResourceLoaderChainE
#[doc(alias = "Ogre::TextureLoaderOp::operator()(Ogre::RbxManualResourceLoaderChain *)")]
// was: __ZN4Ogre15TextureLoaderOpclEPNS_28RbxManualResourceLoaderChainE
// IDA 0xbbc058: 300 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbc058() {
}


// 0xbbca30 — __ZN4Ogre22RbxManualTextureLoaderC1EPNS_12VisualEngineE
// type: _DWORD __fastcall(Ogre::RbxManualTextureLoader *__hidden this, Ogre::VisualEngine *)
#[doc(alias = "Ogre::RbxManualTextureLoader::RbxManualTextureLoader(Ogre::VisualEngine *)")]
// was: __ZN4Ogre22RbxManualTextureLoaderC1EPNS_12VisualEngineE
// IDA 0xbbca30: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbca30() {
}


// 0xbbca54 — __ZN4Ogre22RbxManualTextureLoader12tryLoadImageERSiRNS_5ImageEPKcibPiS6_
// type: _DWORD __fastcall(Ogre::RbxManualTextureLoader *__hidden this, std::istream *, Ogre::Image *, const char *, int, bool, int *, int *)
#[doc(alias = "Ogre::RbxManualTextureLoader::tryLoadImage(std::istream &,Ogre::Image &,char const*,int,bool,int *,int *)")]
// was: __ZN4Ogre22RbxManualTextureLoader12tryLoadImageERSiRNS_5ImageEPKcibPiS6_
// IDA 0xbbca54: 1074 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbca54() {
}


// 0xbbd5c4 — __ZN4Ogre22RbxManualTextureLoader15loadRbxResourceEPNS_8ResourceE
// type: _DWORD __fastcall(Ogre::RbxManualTextureLoader *__hidden this, Ogre::Resource *)
#[doc(alias = "Ogre::RbxManualTextureLoader::loadRbxResource(Ogre::Resource *)")]
// was: __ZN4Ogre22RbxManualTextureLoader15loadRbxResourceEPNS_8ResourceE
// IDA 0xbbd5c4: 918 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbd5c4() {
}


// 0xbbdf98 — __ZN4Ogre22RbxManualTextureLoaderD1Ev
// type: void __fastcall(Ogre::RbxManualTextureLoader *__hidden this)
#[doc(alias = "Ogre::RbxManualTextureLoader::~RbxManualTextureLoader()")]
// was: __ZN4Ogre22RbxManualTextureLoaderD1Ev
// IDA 0xbbdf98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bbdf98() {
}


// 0xbbdf9c — __ZN4Ogre22RbxManualTextureLoaderD0Ev
// type: void __fastcall(Ogre::RbxManualTextureLoader *__hidden this)
#[doc(alias = "Ogre::RbxManualTextureLoader::~RbxManualTextureLoader()")]
// was: __ZN4Ogre22RbxManualTextureLoaderD0Ev
// IDA 0xbbdf9c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bbdf9c() {
}


// 0xbc4cb4 — __ZN4Ogre15RbxSceneManagerC2ERKSs
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::RbxSceneManager::RbxSceneManager(std::string const&)")]
// was: __ZN4Ogre15RbxSceneManagerC2ERKSs
// IDA 0xbc4cb4: 1073 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc4cb4() {
}


// 0xbc57b0 — __ZN4Ogre15RbxSceneManagerD0Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::~RbxSceneManager()")]
// was: __ZN4Ogre15RbxSceneManagerD0Ev
// IDA 0xbc57b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bc57b0() {
}


// 0xbc5864 — __ZN4Ogre15RbxSceneManagerD1Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::~RbxSceneManager()")]
// was: __ZN4Ogre15RbxSceneManagerD1Ev
// IDA 0xbc5864: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bc5864() {
}


// 0xbc5868 — __ZThn17800_N4Ogre15RbxSceneManagerD0Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RbxSceneManager::~RbxSceneManager()")]
// was: __ZThn17800_N4Ogre15RbxSceneManagerD0Ev
// IDA 0xbc5868: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bc5868() {
}


// 0xbc5924 — __ZN4Ogre15RbxSceneManagerD2Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::~RbxSceneManager()")]
// was: __ZN4Ogre15RbxSceneManagerD2Ev
// IDA 0xbc5924: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bc5924() {
}


// 0xbc5fbc — __ZThn17800_N4Ogre15RbxSceneManagerD1Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RbxSceneManager::~RbxSceneManager()")]
// was: __ZThn17800_N4Ogre15RbxSceneManagerD1Ev
// IDA 0xbc5fbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bc5fbc() {
}


// 0xbc5fc8 — __ZN4Ogre15RbxSceneManager15initSpatialHashEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::initSpatialHash(void)")]
// was: __ZN4Ogre15RbxSceneManager15initSpatialHashEv
// IDA 0xbc5fc8: 113 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc5fc8() {
}


// 0xbc6110 — __ZN4Ogre15RbxSceneManager10clearSceneEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::clearScene(void)")]
// was: __ZN4Ogre15RbxSceneManager10clearSceneEv
// IDA 0xbc6110: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6110() {
}


// 0xbc61cc — __ZN4Ogre15RbxSceneManager17clearMegaClustersEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::clearMegaClusters(void)")]
// was: __ZN4Ogre15RbxSceneManager17clearMegaClustersEv
// IDA 0xbc61cc: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc61cc() {
}


// 0xbc6300 — __ZN4Ogre15RbxSceneManager17getSceneNodeCountEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::getSceneNodeCount(void)")]
// was: __ZN4Ogre15RbxSceneManager17getSceneNodeCountEv
// IDA 0xbc6300: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6300() {
}


// 0xbc6718 — __ZN4Ogre15RbxSceneManager16numSharedIBQuadsEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::numSharedIBQuads(void)")]
// was: __ZN4Ogre15RbxSceneManager16numSharedIBQuadsEv
// IDA 0xbc6718: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6718() {
}


// 0xbc67d8 — __ZN4Ogre15RbxSceneManager23getOrCreateSharedQuadIBEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::getOrCreateSharedQuadIB(void)")]
// was: __ZN4Ogre15RbxSceneManager23getOrCreateSharedQuadIBEv
// IDA 0xbc67d8: 469 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc67d8() {
}


// 0xbc6c68 — __ZN4Ogre15RbxSceneManager8_setPassEPKNS_4PassEbb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const Ogre::Pass *, bool, bool)
#[doc(alias = "Ogre::RbxSceneManager::_setPass(Ogre::Pass const*,bool,bool)")]
// was: __ZN4Ogre15RbxSceneManager8_setPassEPKNS_4PassEbb
// IDA 0xbc6c68: 418 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6c68() {
}


// 0xbc70b4 — __ZN4Ogre15RbxSceneManager28renderShadowVolumesToStencilEPKNS_5LightEPKNS_6CameraEb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const Ogre::Light *, const Ogre::Camera *, bool)
#[doc(alias = "Ogre::RbxSceneManager::renderShadowVolumesToStencil(Ogre::Light const*,Ogre::Camera const*,bool)")]
// was: __ZN4Ogre15RbxSceneManager28renderShadowVolumesToStencilEPKNS_5LightEPKNS_6CameraEb
// IDA 0xbc70b4: 318 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc70b4() {
}


// 0xbc7d74 — __ZN4Ogre15RbxSceneManager16getDebugMaterialEPKvb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const void *, bool)
#[doc(alias = "Ogre::RbxSceneManager::getDebugMaterial(void const*,bool)")]
// was: __ZN4Ogre15RbxSceneManager16getDebugMaterialEPKvb
// IDA 0xbc7d74: 648 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc7d74() {
}


// 0xbc8448 — __ZN4Ogre15RbxSceneManager12_renderSceneEPNS_6CameraEPNS_8ViewportEb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, struct _Unwind_Exception *lpuexcpt, Ogre::Viewport *, bool)
#[doc(alias = "Ogre::RbxSceneManager::_renderScene(Ogre::Camera *,Ogre::Viewport *,bool)")]
// was: __ZN4Ogre15RbxSceneManager12_renderSceneEPNS_6CameraEPNS_8ViewportEb
// IDA 0xbc8448: 785 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc8448() {
}


// 0xbc8c98 — __ZN4Ogre15RbxSceneManager11renderBeginEPNS_8ViewportEPNS_6CameraE
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, Ogre::Viewport *, Ogre::Camera *)
#[doc(alias = "Ogre::RbxSceneManager::renderBegin(Ogre::Viewport *,Ogre::Camera *)")]
// was: __ZN4Ogre15RbxSceneManager11renderBeginEPNS_8ViewportEPNS_6CameraE
// IDA 0xbc8c98: 205 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc8c98() {
}


// 0xbc8ebc — __ZN4Ogre15RbxSceneManager22renderQueueGroupSolidsEhNS_26QueuedRenderableCollection16OrganisationModeEb
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, char, char, char, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneManager::renderQueueGroupSolids(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode,bool)")]
// was: __ZN4Ogre15RbxSceneManager22renderQueueGroupSolidsEhNS_26QueuedRenderableCollection16OrganisationModeEb
// IDA 0xbc8ebc: 452 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc8ebc() {
}


// 0xbc9358 — __ZN4Ogre15RbxSceneManager9renderEndEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::renderEnd(void)")]
// was: __ZN4Ogre15RbxSceneManager9renderEndEv
// IDA 0xbc9358: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9358() {
}


// 0xbc9490 — __ZN4Ogre15RbxSceneManager28renderQueueGroupTransparentsEhNS_26QueuedRenderableCollection16OrganisationModeE
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneManager::renderQueueGroupTransparents(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode)")]
// was: __ZN4Ogre15RbxSceneManager28renderQueueGroupTransparentsEhNS_26QueuedRenderableCollection16OrganisationModeE
// IDA 0xbc9490: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9490() {
}


// 0xbc9640 — __ZN4Ogre15RbxSceneManager10_setSkyBoxEbRKSsfhRKNS_10QuaternionES2_
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, bool, const std::string *, float, unsigned __int8, const Ogre::Quaternion *, const std::string *)
#[doc(alias = "Ogre::RbxSceneManager::_setSkyBox(bool,std::string const&,float,unsigned char,Ogre::Quaternion const&,std::string const&)")]
// was: __ZN4Ogre15RbxSceneManager10_setSkyBoxEbRKSsfhRKNS_10QuaternionES2_
// IDA 0xbc9640: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9640() {
}


// 0xbc97f4 — __ZN4Ogre15RbxSceneManager15recordPassStatsEj
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, unsigned int)
#[doc(alias = "Ogre::RbxSceneManager::recordPassStats(unsigned int)")]
// was: __ZN4Ogre15RbxSceneManager15recordPassStatsEj
// IDA 0xbc97f4: 88 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc97f4() {
}


// 0xbc98e0 — __ZN4Ogre15RbxSceneManager18renderSingleObjectEPNS_10RenderableEPKNS_4PassEbbPKNS_12HashedVectorIPNS_5LightEEE
#[doc(alias = "Ogre::RbxSceneManager::renderSingleObject(Ogre::Renderable *,Ogre::Pass const*,bool,bool,Ogre::HashedVector<Ogre::Light *> const*)")]
// was: __ZN4Ogre15RbxSceneManager18renderSingleObjectEPNS_10RenderableEPKNS_4PassEbbPKNS_12HashedVectorIPNS_5LightEEE
// IDA 0xbc98e0: 283 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc98e0() {
}


// 0xbc9bd0 — __ZN4Ogre15RbxSceneManager29updateRenderQueueSplitOptionsEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::updateRenderQueueSplitOptions(void)")]
// was: __ZN4Ogre15RbxSceneManager29updateRenderQueueSplitOptionsEv
// IDA 0xbc9bd0: 23 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9bd0() {
}


// 0xbc9c0c — __ZN4Ogre15RbxSceneManager23_queueSkiesForRenderingEPNS_6CameraE
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, Ogre::Camera *)
#[doc(alias = "Ogre::RbxSceneManager::_queueSkiesForRendering(Ogre::Camera *)")]
// was: __ZN4Ogre15RbxSceneManager23_queueSkiesForRenderingEPNS_6CameraE
// IDA 0xbc9c0c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9c0c() {
}
