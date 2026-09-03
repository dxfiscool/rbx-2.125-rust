//! rendering — generated_watchdog_rend_w4 — 120 stubs global gap filler EA-sorted asc
//! Source: ida/export.json (85545 funcs) global EA asc next 120 uncovered after 0xbb8a44..
//! Range: 0xbb8a44..0xbf9194 (Ogre|G3D|rendering filtered, global-deduped)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xbb8a44 — __ZN4Ogre14VertexStreamer4lineEffffRKN3G3D6Color4E
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this, float, float, float, float, const G3D::Color4 *)
#[doc(alias = "Ogre::VertexStreamer::line(float,float,float,float,G3D::Color4 const&)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer4lineEffffRKN3G3D6Color4E")]
// IDA 0xbb8a44: 227 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb8a44() {
}

// 0xbb8cb8 — __ZN4Ogre14VertexStreamer6line3dEffffffRKN3G3D6Color4E
// type: _DWORD __fastcall(Ogre::VertexStreamer *__hidden this, float, float, float, float, float, struct _Unwind_Exception *lpuexcpt, const G3D::Color4 *)
#[doc(alias = "Ogre::VertexStreamer::line3d(float,float,float,float,float,float,G3D::Color4 const&)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer6line3dEffffffRKN3G3D6Color4E")]
// IDA 0xbb8cb8: 227 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb8cb8() {
}

// 0xbb8f2c — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEED1Ev
// type: 
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::~VertexBufferBatch()")]
#[doc(alias = "__ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEED1Ev")]
// IDA 0xbb8f2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bb8f2c() {
}

// 0xbb9178 — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEED1Ev
// type: 
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::~VertexBufferBatch()")]
#[doc(alias = "__ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEED1Ev")]
// IDA 0xbb9178: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bb9178() {
}

// 0xbb93c4 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6resizeEib
// type: int(void)
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6resizeEib")]
// IDA 0xbb93c4: 135 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb93c4() {
}

// 0xbb9540 — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE5setupEPNS_12RenderSystemE
// type: int(void)
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::setup(Ogre::RenderSystem *)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE5setupEPNS_12RenderSystemE")]
// IDA 0xbb9540: 170 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb9540() {
}

// 0xbb97fc — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE5setupEPNS_12RenderSystemE
// type: 
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::setup(Ogre::RenderSystem *)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE5setupEPNS_12RenderSystemE")]
// IDA 0xbb97fc: 171 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb97fc() {
}

// 0xbb9abc — __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6appendERKS3_
// type: 
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::append(Ogre::VertexStreamer::VertexChunk const&)")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE6appendERKS3_")]
// IDA 0xbb9abc: 289 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb9abc() {
}

// 0xbb9dc4 — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE20createHardwareBufferEj
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, char, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3DTexture>::createHardwareBuffer(unsigned int)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_15Vertex3DTextureEE20createHardwareBufferEj")]
// IDA 0xbb9dc4: 294 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb9dc4() {
}

// 0xbba0b8 — __ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE20createHardwareBufferEj
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, char, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::VertexStreamer::VertexBufferBatch<Ogre::VertexStreamer::Vertex3D>::createHardwareBuffer(unsigned int)")]
#[doc(alias = "__ZN4Ogre14VertexStreamer17VertexBufferBatchINS0_8Vertex3DEE20createHardwareBufferEj")]
// IDA 0xbba0b8: 280 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba0b8() {
}

// 0xbba388 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE6resizeEib
// type: int(void)
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE6resizeEib")]
// IDA 0xbba388: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba388() {
}

// 0xbba45c — __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE7reallocEi
// type: 
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE7reallocEi")]
// IDA 0xbba45c: 154 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba45c() {
}

// 0xbba670 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE6resizeEib
// type: 
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE6resizeEib")]
// IDA 0xbba670: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba670() {
}

// 0xbba734 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE7reallocEi
// type: 
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE7reallocEi")]
// IDA 0xbba734: 147 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba734() {
}

// 0xbba920 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE7reallocEi
// type: 
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EE7reallocEi")]
// IDA 0xbba920: 221 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba920() {
}

// 0xbbabe8 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EED2Ev
// type: 
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EED2Ev")]
// IDA 0xbbabe8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bbabe8() {
}

// 0xbbadc4 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EEC2Ev
// type: 
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::VertexChunk,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer11VertexChunkELi10ELm32EEC2Ev")]
// IDA 0xbbadc4: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbadc4() {
}

// 0xbbaf7c — __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE6appendERKS3_
// type: 
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::append(Ogre::VertexStreamer::Vertex3D const&)")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EE6appendERKS3_")]
// IDA 0xbbaf7c: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbaf7c() {
}

// 0xbbb000 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE6appendERKS3_
// type: int(void)
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::append(Ogre::VertexStreamer::Vertex3DTexture const&)")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EE6appendERKS3_")]
// IDA 0xbbb000: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbb000() {
}

// 0xbbb0a8 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EED2Ev
// type: 
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EED2Ev")]
// IDA 0xbbb0a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bbb0a8() {
}

// 0xbbb1c8 — __ZN3G3D5ArrayIiLi10ELm32EED2Ev
// type: 
#[doc(alias = "G3D::Array<int,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIiLi10ELm32EED2Ev")]
// IDA 0xbbb1c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bbb1c8() {
}

// 0xbbb2e8 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EED2Ev
// type: 
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EED2Ev")]
// IDA 0xbbb2e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bbb2e8() {
}

// 0xbbb408 — __ZN3G3D5ArrayIiLi10ELm32EEC2Ev
// type: 
#[doc(alias = "G3D::Array<int,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIiLi10ELm32EEC2Ev")]
// IDA 0xbbb408: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbb408() {
}

// 0xbbb5c0 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EEC2Ev
// type: 
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3DTexture,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer15Vertex3DTextureELi10ELm32EEC2Ev")]
// IDA 0xbbb5c0: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbb5c0() {
}

// 0xbbb778 — __ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EEC2Ev
// type: 
#[doc(alias = "G3D::Array<Ogre::VertexStreamer::Vertex3D,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIN4Ogre14VertexStreamer8Vertex3DELi10ELm32EEC2Ev")]
// IDA 0xbbb778: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbb778() {
}

// 0xbbbfb8 — __ZN4Ogre28RbxManualResourceLoaderChainC2EPNS_12VisualEngineEPPS0_
// type: 
#[doc(alias = "Ogre::RbxManualResourceLoaderChain::RbxManualResourceLoaderChain(Ogre::VisualEngine *,Ogre::RbxManualResourceLoaderChain**)")]
#[doc(alias = "__ZN4Ogre28RbxManualResourceLoaderChainC2EPNS_12VisualEngineEPPS0_")]
// IDA 0xbbbfb8: 15 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbbfb8() {
}

// 0xbbbfdc — __ZN4Ogre28RbxManualResourceLoaderChain12loadResourceEPNS_8ResourceE
// type: _DWORD __fastcall(Ogre::RbxManualResourceLoaderChain *__hidden this, Ogre::Resource *)
#[doc(alias = "Ogre::RbxManualResourceLoaderChain::loadResource(Ogre::Resource *)")]
#[doc(alias = "__ZN4Ogre28RbxManualResourceLoaderChain12loadResourceEPNS_8ResourceE")]
// IDA 0xbbbfdc: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbbfdc() {
}

// 0xbbc010 — __ZN4Ogre28RbxManualResourceLoaderChain11DeleteChainEv
// type: _DWORD __fastcall(Ogre::RbxManualResourceLoaderChain *__hidden this)
#[doc(alias = "Ogre::RbxManualResourceLoaderChain::DeleteChain(void)")]
#[doc(alias = "__ZN4Ogre28RbxManualResourceLoaderChain11DeleteChainEv")]
// IDA 0xbbc010: 26 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbc010() {
}

// 0xbbc048 — __ZN4Ogre28RbxManualResourceLoaderChain7ZombifyEv
// type: _DWORD __fastcall(Ogre::RbxManualResourceLoaderChain *__hidden this)
#[doc(alias = "Ogre::RbxManualResourceLoaderChain::Zombify(void)")]
#[doc(alias = "__ZN4Ogre28RbxManualResourceLoaderChain7ZombifyEv")]
// IDA 0xbbc048: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbc048() {
}

// 0xbbc050 — __ZN4Ogre28RbxManualResourceLoaderChainD1Ev
// type: void __fastcall(Ogre::RbxManualResourceLoaderChain *__hidden this)
#[doc(alias = "Ogre::RbxManualResourceLoaderChain::~RbxManualResourceLoaderChain()")]
#[doc(alias = "__ZN4Ogre28RbxManualResourceLoaderChainD1Ev")]
// IDA 0xbbc050: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bbc050() {
}

// 0xbbc054 — __ZN4Ogre28RbxManualResourceLoaderChainD0Ev
// type: void __fastcall(Ogre::RbxManualResourceLoaderChain *__hidden this)
#[doc(alias = "Ogre::RbxManualResourceLoaderChain::~RbxManualResourceLoaderChain()")]
#[doc(alias = "__ZN4Ogre28RbxManualResourceLoaderChainD0Ev")]
// IDA 0xbbc054: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bbc054() {
}

// 0xbbc058 — __ZN4Ogre15TextureLoaderOpclEPNS_28RbxManualResourceLoaderChainE
// type: 
#[doc(alias = "Ogre::TextureLoaderOp::operator()(Ogre::RbxManualResourceLoaderChain *)")]
#[doc(alias = "__ZN4Ogre15TextureLoaderOpclEPNS_28RbxManualResourceLoaderChainE")]
// IDA 0xbbc058: 300 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbc058() {
}

// 0xbbca30 — __ZN4Ogre22RbxManualTextureLoaderC1EPNS_12VisualEngineE
// type: _DWORD __fastcall(Ogre::RbxManualTextureLoader *__hidden this, Ogre::VisualEngine *)
#[doc(alias = "Ogre::RbxManualTextureLoader::RbxManualTextureLoader(Ogre::VisualEngine *)")]
#[doc(alias = "__ZN4Ogre22RbxManualTextureLoaderC1EPNS_12VisualEngineE")]
// IDA 0xbbca30: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbca30() {
}

// 0xbbca54 — __ZN4Ogre22RbxManualTextureLoader12tryLoadImageERSiRNS_5ImageEPKcibPiS6_
// type: _DWORD __fastcall(Ogre::RbxManualTextureLoader *__hidden this, std::istream *, Ogre::Image *, const char *, int, bool, int *, int *)
#[doc(alias = "Ogre::RbxManualTextureLoader::tryLoadImage(std::istream &,Ogre::Image &,char const*,int,bool,int *,int *)")]
#[doc(alias = "__ZN4Ogre22RbxManualTextureLoader12tryLoadImageERSiRNS_5ImageEPKcibPiS6_")]
// IDA 0xbbca54: 1074 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbca54() {
}

// 0xbbd5c4 — __ZN4Ogre22RbxManualTextureLoader15loadRbxResourceEPNS_8ResourceE
// type: _DWORD __fastcall(Ogre::RbxManualTextureLoader *__hidden this, Ogre::Resource *)
#[doc(alias = "Ogre::RbxManualTextureLoader::loadRbxResource(Ogre::Resource *)")]
#[doc(alias = "__ZN4Ogre22RbxManualTextureLoader15loadRbxResourceEPNS_8ResourceE")]
// IDA 0xbbd5c4: 918 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbd5c4() {
}

// 0xbbdf98 — __ZN4Ogre22RbxManualTextureLoaderD1Ev
// type: void __fastcall(Ogre::RbxManualTextureLoader *__hidden this)
#[doc(alias = "Ogre::RbxManualTextureLoader::~RbxManualTextureLoader()")]
#[doc(alias = "__ZN4Ogre22RbxManualTextureLoaderD1Ev")]
// IDA 0xbbdf98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bbdf98() {
}

// 0xbbdf9c — __ZN4Ogre22RbxManualTextureLoaderD0Ev
// type: void __fastcall(Ogre::RbxManualTextureLoader *__hidden this)
#[doc(alias = "Ogre::RbxManualTextureLoader::~RbxManualTextureLoader()")]
#[doc(alias = "__ZN4Ogre22RbxManualTextureLoaderD0Ev")]
// IDA 0xbbdf9c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bbdf9c() {
}

// 0xbc4cb4 — __ZN4Ogre15RbxSceneManagerC2ERKSs
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::RbxSceneManager::RbxSceneManager(std::string const&)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManagerC2ERKSs")]
// IDA 0xbc4cb4: 1073 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc4cb4() {
}

// 0xbc57b0 — __ZN4Ogre15RbxSceneManagerD0Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::~RbxSceneManager()")]
#[doc(alias = "__ZN4Ogre15RbxSceneManagerD0Ev")]
// IDA 0xbc57b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bc57b0() {
}

// 0xbc5864 — __ZN4Ogre15RbxSceneManagerD1Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::~RbxSceneManager()")]
#[doc(alias = "__ZN4Ogre15RbxSceneManagerD1Ev")]
// IDA 0xbc5864: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bc5864() {
}

// 0xbc5868 — __ZThn17800_N4Ogre15RbxSceneManagerD0Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RbxSceneManager::~RbxSceneManager()")]
#[doc(alias = "__ZThn17800_N4Ogre15RbxSceneManagerD0Ev")]
// IDA 0xbc5868: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bc5868() {
}

// 0xbc5924 — __ZN4Ogre15RbxSceneManagerD2Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::~RbxSceneManager()")]
#[doc(alias = "__ZN4Ogre15RbxSceneManagerD2Ev")]
// IDA 0xbc5924: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bc5924() {
}

// 0xbc5fbc — __ZThn17800_N4Ogre15RbxSceneManagerD1Ev
// type: void __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RbxSceneManager::~RbxSceneManager()")]
#[doc(alias = "__ZThn17800_N4Ogre15RbxSceneManagerD1Ev")]
// IDA 0xbc5fbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bc5fbc() {
}

// 0xbc5fc8 — __ZN4Ogre15RbxSceneManager15initSpatialHashEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::initSpatialHash(void)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager15initSpatialHashEv")]
// IDA 0xbc5fc8: 113 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc5fc8() {
}

// 0xbc6110 — __ZN4Ogre15RbxSceneManager10clearSceneEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::clearScene(void)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager10clearSceneEv")]
// IDA 0xbc6110: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6110() {
}

// 0xbc61cc — __ZN4Ogre15RbxSceneManager17clearMegaClustersEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::clearMegaClusters(void)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager17clearMegaClustersEv")]
// IDA 0xbc61cc: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc61cc() {
}

// 0xbc6300 — __ZN4Ogre15RbxSceneManager17getSceneNodeCountEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::getSceneNodeCount(void)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager17getSceneNodeCountEv")]
// IDA 0xbc6300: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6300() {
}

// 0xbc6718 — __ZN4Ogre15RbxSceneManager16numSharedIBQuadsEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::numSharedIBQuads(void)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager16numSharedIBQuadsEv")]
// IDA 0xbc6718: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6718() {
}

// 0xbc67d8 — __ZN4Ogre15RbxSceneManager23getOrCreateSharedQuadIBEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::getOrCreateSharedQuadIB(void)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager23getOrCreateSharedQuadIBEv")]
// IDA 0xbc67d8: 469 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc67d8() {
}

// 0xbc6c68 — __ZN4Ogre15RbxSceneManager8_setPassEPKNS_4PassEbb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const Ogre::Pass *, bool, bool)
#[doc(alias = "Ogre::RbxSceneManager::_setPass(Ogre::Pass const*,bool,bool)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager8_setPassEPKNS_4PassEbb")]
// IDA 0xbc6c68: 418 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc6c68() {
}

// 0xbc70b4 — __ZN4Ogre15RbxSceneManager28renderShadowVolumesToStencilEPKNS_5LightEPKNS_6CameraEb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const Ogre::Light *, const Ogre::Camera *, bool)
#[doc(alias = "Ogre::RbxSceneManager::renderShadowVolumesToStencil(Ogre::Light const*,Ogre::Camera const*,bool)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager28renderShadowVolumesToStencilEPKNS_5LightEPKNS_6CameraEb")]
// IDA 0xbc70b4: 318 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc70b4() {
}

// 0xbc7d74 — __ZN4Ogre15RbxSceneManager16getDebugMaterialEPKvb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const void *, bool)
#[doc(alias = "Ogre::RbxSceneManager::getDebugMaterial(void const*,bool)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager16getDebugMaterialEPKvb")]
// IDA 0xbc7d74: 648 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc7d74() {
}

// 0xbc8448 — __ZN4Ogre15RbxSceneManager12_renderSceneEPNS_6CameraEPNS_8ViewportEb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, struct _Unwind_Exception *lpuexcpt, Ogre::Viewport *, bool)
#[doc(alias = "Ogre::RbxSceneManager::_renderScene(Ogre::Camera *,Ogre::Viewport *,bool)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager12_renderSceneEPNS_6CameraEPNS_8ViewportEb")]
// IDA 0xbc8448: 785 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc8448() {
}

// 0xbc8c98 — __ZN4Ogre15RbxSceneManager11renderBeginEPNS_8ViewportEPNS_6CameraE
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, Ogre::Viewport *, Ogre::Camera *)
#[doc(alias = "Ogre::RbxSceneManager::renderBegin(Ogre::Viewport *,Ogre::Camera *)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager11renderBeginEPNS_8ViewportEPNS_6CameraE")]
// IDA 0xbc8c98: 205 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc8c98() {
}

// 0xbc8ebc — __ZN4Ogre15RbxSceneManager22renderQueueGroupSolidsEhNS_26QueuedRenderableCollection16OrganisationModeEb
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, char, char, char, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneManager::renderQueueGroupSolids(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode,bool)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager22renderQueueGroupSolidsEhNS_26QueuedRenderableCollection16OrganisationModeEb")]
// IDA 0xbc8ebc: 452 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc8ebc() {
}

// 0xbc9358 — __ZN4Ogre15RbxSceneManager9renderEndEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::renderEnd(void)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager9renderEndEv")]
// IDA 0xbc9358: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9358() {
}

// 0xbc9490 — __ZN4Ogre15RbxSceneManager28renderQueueGroupTransparentsEhNS_26QueuedRenderableCollection16OrganisationModeE
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int)
#[doc(alias = "Ogre::RbxSceneManager::renderQueueGroupTransparents(unsigned char,Ogre::QueuedRenderableCollection::OrganisationMode)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager28renderQueueGroupTransparentsEhNS_26QueuedRenderableCollection16OrganisationModeE")]
// IDA 0xbc9490: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9490() {
}

// 0xbc9640 — __ZN4Ogre15RbxSceneManager10_setSkyBoxEbRKSsfhRKNS_10QuaternionES2_
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, bool, const std::string *, float, unsigned __int8, const Ogre::Quaternion *, const std::string *)
#[doc(alias = "Ogre::RbxSceneManager::_setSkyBox(bool,std::string const&,float,unsigned char,Ogre::Quaternion const&,std::string const&)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager10_setSkyBoxEbRKSsfhRKNS_10QuaternionES2_")]
// IDA 0xbc9640: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9640() {
}

// 0xbc97f4 — __ZN4Ogre15RbxSceneManager15recordPassStatsEj
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, unsigned int)
#[doc(alias = "Ogre::RbxSceneManager::recordPassStats(unsigned int)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager15recordPassStatsEj")]
// IDA 0xbc97f4: 88 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc97f4() {
}

// 0xbc98e0 — __ZN4Ogre15RbxSceneManager18renderSingleObjectEPNS_10RenderableEPKNS_4PassEbbPKNS_12HashedVectorIPNS_5LightEEE
// type: 
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

// 0xbc9c48 — __ZN4Ogre15RbxSceneManager19createSceneNodeImplEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::createSceneNodeImpl(void)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager19createSceneNodeImplEv")]
// IDA 0xbc9c48: 67 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9c48() {
}

// 0xbc9d08 — __ZN4Ogre15RbxSceneManager19createSceneNodeImplERKSs
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::RbxSceneManager::createSceneNodeImpl(std::string const&)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager19createSceneNodeImplERKSs")]
// IDA 0xbc9d08: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9d08() {
}

// 0xbc9dcc — __ZNK4Ogre15RbxSceneManager11getTypeNameEv
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this)
#[doc(alias = "Ogre::RbxSceneManager::getTypeName(void)const")]
#[doc(alias = "__ZNK4Ogre15RbxSceneManager11getTypeNameEv")]
// IDA 0xbc9dcc: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9dcc() {
}

// 0xbc9dd8 — __ZN4Ogre15RbxSceneManager19_findVisibleObjectsEPNS_6CameraEPNS_24VisibleObjectsBoundsInfoEb
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, Ogre::Camera *, Ogre::VisibleObjectsBoundsInfo *, bool)
#[doc(alias = "Ogre::RbxSceneManager::_findVisibleObjects(Ogre::Camera *,Ogre::VisibleObjectsBoundsInfo *,bool)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager19_findVisibleObjectsEPNS_6CameraEPNS_24VisibleObjectsBoundsInfoEb")]
// IDA 0xbc9dd8: 223 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc9dd8() {
}

// 0xbca050 — __ZN4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE
// type: 
#[doc(alias = "Ogre::RbxSceneManager::preRenderTargetUpdate(Ogre::RenderTargetEvent const&)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE")]
// IDA 0xbca050: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca050() {
}

// 0xbca17c — __ZThn17800_N4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE
// type: 
#[doc(alias = "non-virtual thunk toOgre::RbxSceneManager::preRenderTargetUpdate(Ogre::RenderTargetEvent const&)")]
#[doc(alias = "__ZThn17800_N4Ogre15RbxSceneManager21preRenderTargetUpdateERKNS_17RenderTargetEventE")]
// IDA 0xbca17c: 3 insns (MOVW..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca17c() {
}

// 0xbca188 — __ZN4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE
// type: 
#[doc(alias = "Ogre::RbxSceneManager::postRenderTargetUpdate(Ogre::RenderTargetEvent const&)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE")]
// IDA 0xbca188: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca188() {
}

// 0xbca240 — __ZThn17800_N4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE
// type: 
#[doc(alias = "non-virtual thunk toOgre::RbxSceneManager::postRenderTargetUpdate(Ogre::RenderTargetEvent const&)")]
#[doc(alias = "__ZThn17800_N4Ogre15RbxSceneManager22postRenderTargetUpdateERKNS_17RenderTargetEventE")]
// IDA 0xbca240: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca240() {
}

// 0xbca2f4 — __ZN4Ogre15RbxSceneManager15setShadowColourERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::RbxSceneManager::setShadowColour(Ogre::ColourValue const&)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager15setShadowColourERKNS_11ColourValueE")]
// IDA 0xbca2f4: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca2f4() {
}

// 0xbca528 — __ZN4Ogre15RbxSceneManager18setPointOfInterestERKN3G3D7Vector3E
// type: _DWORD __fastcall(Ogre::RbxSceneManager *__hidden this, const Vector3 *)
#[doc(alias = "Ogre::RbxSceneManager::setPointOfInterest(G3D::Vector3 const&)")]
#[doc(alias = "__ZN4Ogre15RbxSceneManager18setPointOfInterestERKN3G3D7Vector3E")]
// IDA 0xbca528: 12 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca528() {
}

// 0xbca548 — __ZNK4Ogre22RbxSceneManagerFactory12initMetaDataEv
// type: _DWORD __fastcall(Ogre::RbxSceneManagerFactory *__hidden this)
#[doc(alias = "Ogre::RbxSceneManagerFactory::initMetaData(void)const")]
#[doc(alias = "__ZNK4Ogre22RbxSceneManagerFactory12initMetaDataEv")]
// IDA 0xbca548: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca548() {
}

// 0xbca87c — __ZN4Ogre9SharedPtrINS_6RbxSkyEED1Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "Ogre::SharedPtr<Ogre::RbxSky>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_6RbxSkyEED1Ev")]
// IDA 0xbca87c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bca87c() {
}

// 0xbca8d0 — __ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE7reserveEm
// type: int(void)
#[doc(alias = "std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE7reserveEm")]
// IDA 0xbca8d0: 46 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca8d0() {
}

// 0xbca94c — __ZN4Ogre20RenderTargetListener17preViewportUpdateERKNS_25RenderTargetViewportEventE
// type: 
#[doc(alias = "Ogre::RenderTargetListener::preViewportUpdate(Ogre::RenderTargetViewportEvent const&)")]
#[doc(alias = "__ZN4Ogre20RenderTargetListener17preViewportUpdateERKNS_25RenderTargetViewportEventE")]
// IDA 0xbca94c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_bca94c() {
}

// 0xbca950 — __ZN4Ogre20RenderTargetListener18postViewportUpdateERKNS_25RenderTargetViewportEventE
// type: 
#[doc(alias = "Ogre::RenderTargetListener::postViewportUpdate(Ogre::RenderTargetViewportEvent const&)")]
#[doc(alias = "__ZN4Ogre20RenderTargetListener18postViewportUpdateERKNS_25RenderTargetViewportEventE")]
// IDA 0xbca950: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_bca950() {
}

// 0xbcaa4c — __ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// type: 
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

// 0xbcacfc — __ZN4Ogre9SharedPtrINS_6RbxSkyEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::RbxSky>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_6RbxSkyEED0Ev")]
// IDA 0xbcacfc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bcacfc() {
}

// 0xbcad9c — __ZN4Ogre9SharedPtrINS_6RbxSkyEE7destroyEv
// type: 
#[doc(alias = "Ogre::SharedPtr<Ogre::RbxSky>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_6RbxSkyEE7destroyEv")]
// IDA 0xbcad9c: 25 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcad9c() {
}

// 0xbcadd4 — __ZN4Ogre9SharedPtrINS_6RbxSkyEE4swapERS2_
// type: 
#[doc(alias = "Ogre::SharedPtr<Ogre::RbxSky>::swap(Ogre::SharedPtr<Ogre::RbxSky>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_6RbxSkyEE4swapERS2_")]
// IDA 0xbcadd4: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcadd4() {
}

// 0xbcb48c — __ZN4Ogre12RbxSceneNode7_updateEbb
// type: _DWORD __fastcall(Ogre::RbxSceneNode *__hidden this, bool, bool)
#[doc(alias = "Ogre::RbxSceneNode::_update(bool,bool)")]
#[doc(alias = "__ZN4Ogre12RbxSceneNode7_updateEbb")]
// IDA 0xbcb48c: 33 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcb48c() {
}

// 0xbd7264 — __ZNSt8_Rb_treeIPKN4Ogre7SubMeshESt4pairIKS3_NS0_6SphereEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: 
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN4Ogre7SubMeshESt4pairIKS3_NS0_6SphereEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// IDA 0xbd7264: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd7264() {
}

// 0xbde2d4 — __ZN12_GLOBAL__N_123replaceMaterialTexturesERKyN4Ogre10TexturePtrE
// type: 
#[doc(alias = "anonymous namespace::replaceMaterialTextures(unsigned long long const&,Ogre::TexturePtr)")]
#[doc(alias = "__ZN12_GLOBAL__N_123replaceMaterialTexturesERKyN4Ogre10TexturePtrE")]
// IDA 0xbde2d4: 216 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bde2d4() {
}

// 0xbe0ba8 — __ZNSt6vectorIN4Ogre10TexturePtrESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: 
#[doc(alias = "std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>>,Ogre::TexturePtr const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre10TexturePtrESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
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
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::TexturePtr * std::__copy_backward_normal<false,false>::__copy_b_n<Ogre::TexturePtr *,Ogre::TexturePtr *>(Ogre::TexturePtr *,Ogre::TexturePtr *,Ogre::TexturePtr *)")]
#[doc(alias = "__ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN4Ogre10TexturePtrES4_EET0_T_S6_S5_")]
// IDA 0xbe0ffc: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_be0ffc() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xbe618c — __ZN4Ogre17SaveTextureToFileEPNS_7TextureERKSs
// type: _DWORD __fastcall(Ogre *__hidden this, Ogre::Texture *, const std::string *)
#[doc(alias = "Ogre::SaveTextureToFile(Ogre::Texture *,std::string const&)")]
#[doc(alias = "__ZN4Ogre17SaveTextureToFileEPNS_7TextureERKSs")]
// IDA 0xbe618c: 243 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be618c() {
}

// 0xbe63e8 — __ZN4Ogre17Frustum_IntersectEPKNS_7FrustumERKNS_14AxisAlignedBoxEPNS_12FrustumPlaneE
// type: 
#[doc(alias = "Ogre::Frustum_Intersect(Ogre::Frustum const*,Ogre::AxisAlignedBox const&,Ogre::FrustumPlane *)")]
#[doc(alias = "__ZN4Ogre17Frustum_IntersectEPKNS_7FrustumERKNS_14AxisAlignedBoxEPNS_12FrustumPlaneE")]
// IDA 0xbe63e8: 120 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be63e8() {
}

// 0xbe65b4 — __ZN4Ogre9ToExtentsERKNS_14AxisAlignedBoxE
// type: _DWORD __fastcall(Ogre *__hidden this, const Ogre::AxisAlignedBox *)
#[doc(alias = "Ogre::ToExtents(Ogre::AxisAlignedBox const&)")]
#[doc(alias = "__ZN4Ogre9ToExtentsERKNS_14AxisAlignedBoxE")]
// IDA 0xbe65b4: 9 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be65b4() {
}

// 0xbe65d0 — __ZN4Ogre9ToVector3ERKNS_7Vector3E
// type: _DWORD __fastcall(Ogre *__hidden this, const Vector3 *)
#[doc(alias = "Ogre::ToVector3(Ogre::Vector3 const&)")]
#[doc(alias = "__ZN4Ogre9ToVector3ERKNS_7Vector3E")]
// IDA 0xbe65d0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_be65d0() {
}

// 0xbe65d4 — __ZN4Ogre9ToVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(Ogre *__hidden this, const Vector3 *)
#[doc(alias = "Ogre::ToVector3(G3D::Vector3 const&)")]
#[doc(alias = "__ZN4Ogre9ToVector3ERKN3G3D7Vector3E")]
// IDA 0xbe65d4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_be65d4() {
}

// 0xbe6648 — __ZN4Ogre12ToColorValueERKN3G3D6Color4E
// type: _DWORD __fastcall(Ogre *__hidden this, const G3D::Color4 *)
#[doc(alias = "Ogre::ToColorValue(G3D::Color4 const&)")]
#[doc(alias = "__ZN4Ogre12ToColorValueERKN3G3D6Color4E")]
// IDA 0xbe6648: 15 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be6648() {
}

// 0xbe6684 — __ZN4Ogre12ToColorValueERKN3G3D6Color3E
// type: 
#[doc(alias = "Ogre::ToColorValue(G3D::Color3 const&)")]
#[doc(alias = "__ZN4Ogre12ToColorValueERKN3G3D6Color3E")]
// IDA 0xbe6684: 14 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be6684() {
}

// 0xbe66b8 — __ZN4Ogre8MaxCoordERKNS_7Vector3E
// type: _DWORD __fastcall(Ogre *__hidden this, const Vector3 *)
#[doc(alias = "Ogre::MaxCoord(Ogre::Vector3 const&)")]
#[doc(alias = "__ZN4Ogre8MaxCoordERKNS_7Vector3E")]
// IDA 0xbe66b8: 13 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be66b8() {
}

// 0xbec248 — __ZN3RBX14visitPrintNodeEPN4Ogre9SceneNodeERSs
// type: _DWORD __fastcall(RBX *__hidden this, Ogre::SceneNode *, std::string *)
#[doc(alias = "RBX::visitPrintNode(Ogre::SceneNode *,std::string &)")]
#[doc(alias = "__ZN3RBX14visitPrintNodeEPN4Ogre9SceneNodeERSs")]
// IDA 0xbec248: 400 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bec248() {
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
#[doc(alias = "non-virtual thunk toRBX::ViewRbxGfx::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
#[doc(alias = "__ZThn8_N3RBX10ViewRbxGfx13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE")]
// IDA 0xbef138: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef138() {
}

// 0xbef328 — __ZN4Ogre9SharedPtrINS_8ResourceEED1Ev
// type: 
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_8ResourceEED1Ev")]
// IDA 0xbef328: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bef328() {
}

// 0xbf29a8 — __ZN4Ogre9SharedPtrINS_8ResourceEED0Ev
// type: 
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_8ResourceEED0Ev")]
// IDA 0xbf29a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf29a8() {
}

// 0xbf2a48 — __ZN4Ogre9SharedPtrINS_8ResourceEE7destroyEv
// type: 
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_8ResourceEE7destroyEv")]
// IDA 0xbf2a48: 25 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf2a48() {
}

// 0xbf2a80 — __ZN4Ogre9SharedPtrINS_8ResourceEE4swapERS2_
// type: 
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::swap(Ogre::SharedPtr<Ogre::Resource>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_8ResourceEE4swapERS2_")]
// IDA 0xbf2a80: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf2a80() {
}

// 0xbf2c80 — __ZN4Ogre9SharedPtrINS_8MaterialEED1Ev
// type: 
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_8MaterialEED1Ev")]
// IDA 0xbf2c80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf2c80() {
}

// 0xbf2cb0 — __ZN4Ogre11MaterialPtrD0Ev
// type: void __fastcall(Ogre::MaterialPtr *__hidden this)
#[doc(alias = "Ogre::MaterialPtr::~MaterialPtr()")]
#[doc(alias = "__ZN4Ogre11MaterialPtrD0Ev")]
// IDA 0xbf2cb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf2cb0() {
}

// 0xbf5b30 — __ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEED1Ev
// type: 
#[doc(alias = "rbx_core::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEED1Ev")]
// was: boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::~sp_counted_impl_p()
// IDA 0xbf5b30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_bf5b30() {
}

// 0xbf5b34 — __ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEED0Ev
// type: 
#[doc(alias = "rbx_core::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEED0Ev")]
// was: boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::~sp_counted_impl_p()
// IDA 0xbf5b34: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bf5b34() {
}

// 0xbf5b38 — __ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEE7disposeEv
// type: 
#[doc(alias = "rbx_core::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEE7disposeEv")]
// was: boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::dispose(void)
// IDA 0xbf5b38: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf5b38() {
}

// 0xbf5b48 — __ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEE11get_deleterERKSt9type_info
// type: 
#[doc(alias = "rbx_core::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEE11get_deleterERKSt9type_info")]
// was: boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::get_deleter(std::type_info const&)
// IDA 0xbf5b48: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf5b48() {
}

// 0xbf5b4c — __ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEE19get_untyped_deleterEv
// type: 
#[doc(alias = "rbx_core::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEE19get_untyped_deleterEv")]
// was: boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::get_untyped_deleter(void)
// IDA 0xbf5b4c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf5b4c() {
}

// 0xbf61f4 — __ZN4Ogre13RbxTypesetterC1ENS_7FontPtrES1_fff
// type: int __fastcall(int, int, int, int, float, float)
#[doc(alias = "Ogre::RbxTypesetter::RbxTypesetter(Ogre::FontPtr,Ogre::FontPtr,float,float,float)")]
#[doc(alias = "__ZN4Ogre13RbxTypesetterC1ENS_7FontPtrES1_fff")]
// IDA 0xbf61f4: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf61f4() {
}

// 0xbf62a8 — __ZNK4Ogre13RbxTypesetter12computeArrayERKSsfffN3RBX4Text6XAlignEPN3G3D7Vector2EmiN9__gnu_cxx17__normal_iteratorIPKSt4pairIiNS0_7SpacingEESt6vectorISD_SaISD_EEEESJ_RKNS_7FontPtrEb
// type: int __fastcall(int, int, int, int, float, float, int, int, int, float, int, int, int, int)
#[doc(alias = "Ogre::RbxTypesetter::computeArray(std::string const&,float,float,float,RBX::Text::XAlign,G3D::Vector2 *,unsigned long,int,__gnu_cxx::__normal_iterator<std::pair<int,Ogre::RbxTypesetter::Spacing> const*,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>>,__gnu_cxx::__normal_iterator<std::pair<int,Ogre::RbxTypesetter::Spacing> const*,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>>,Ogre::FontPtr const&,bool)const")]
#[doc(alias = "__ZNK4Ogre13RbxTypesetter12computeArrayERKSsfffN3RBX4Text6XAlignEPN3G3D7Vector2EmiN9__gnu_cxx17__normal_iteratorIPKSt4pairIiNS0_7SpacingEESt6vectorISD_SaISD_EEEESJ_RKNS_7FontPtrEb")]
// IDA 0xbf62a8: 365 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf62a8() {
}

// 0xbf66dc — __ZNK4Ogre13RbxTypesetter13getTexturePtrEf
// type: _DWORD __fastcall(Ogre::RbxTypesetter *__hidden this, float)
#[doc(alias = "Ogre::RbxTypesetter::getTexturePtr(float)const")]
#[doc(alias = "__ZNK4Ogre13RbxTypesetter13getTexturePtrEf")]
// IDA 0xbf66dc: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf66dc() {
}

// 0xbf6790 — __ZNK4Ogre13RbxTypesetter4drawEPN3RBX5AdornERKSsRKN3G3D7Vector2EfRKNS6_6Color4ESC_NS1_4Text6XAlignENSD_6YAlignES9_RKNS6_6Rect2DE
// type: int __fastcall(int, int, int, int, int, float, int, int, int, int, int, int)
#[doc(alias = "Ogre::RbxTypesetter::draw(RBX::Adorn *,std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)const")]
#[doc(alias = "__ZNK4Ogre13RbxTypesetter4drawEPN3RBX5AdornERKSsRKN3G3D7Vector2EfRKNS6_6Color4ESC_NS1_4Text6XAlignENSD_6YAlignES9_RKNS6_6Rect2DE")]
// IDA 0xbf6790: 844 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf6790() {
}

// 0xbf7268 — __ZNK4Ogre13RbxTypesetter15measureInternalERKSsfRKN3G3D7Vector2EPSt6vectorISt4pairIiNS0_7SpacingEESaISA_EEPb
// type: 
#[doc(alias = "Ogre::RbxTypesetter::measureInternal(std::string const&,float,G3D::Vector2 const&,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>> *,bool *)const")]
#[doc(alias = "__ZNK4Ogre13RbxTypesetter15measureInternalERKSsfRKN3G3D7Vector2EPSt6vectorISt4pairIiNS0_7SpacingEESaISA_EEPb")]
// IDA 0xbf7268: 460 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf7268() {
}

// 0xbf7794 — __ZNK4Ogre13RbxTypesetter23getCursorPositionInTextERKSsRKN3G3D7Vector2EfN3RBX4Text6XAlignENS8_6YAlignES6_S4_
// type: int __fastcall(int, int, int, int, int, int, int, G3D::Vector2 *)
#[doc(alias = "Ogre::RbxTypesetter::getCursorPositionInText(std::string const&,G3D::Vector2 const&,float,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Vector2)const")]
#[doc(alias = "__ZNK4Ogre13RbxTypesetter23getCursorPositionInTextERKSsRKN3G3D7Vector2EfN3RBX4Text6XAlignENS8_6YAlignES6_S4_")]
// IDA 0xbf7794: 494 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf7794() {
}

// 0xbf7dcc — __ZNK4Ogre13RbxTypesetter7measureERKSsfRKN3G3D7Vector2EPb
// type: _DWORD __fastcall(Ogre::RbxTypesetter *__hidden this, const std::string *, float, const G3D::Vector2 *, bool *)
#[doc(alias = "Ogre::RbxTypesetter::measure(std::string const&,float,G3D::Vector2 const&,bool *)const")]
#[doc(alias = "__ZNK4Ogre13RbxTypesetter7measureERKSsfRKN3G3D7Vector2EPb")]
// IDA 0xbf7dcc: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf7dcc() {
}

// 0xbf8504 — __ZNK3RBX16TypesetterBitmap4drawEPNS_5AdornERKSsRKN3G3D7Vector2EfRKNS5_6Color4ESB_NS_4Text6XAlignENSC_6YAlignES8_RKNS5_6Rect2DE
// type: int __fastcall(int, int, int, int, int, float, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
#[doc(alias = "RBX::TypesetterBitmap::draw(RBX::Adorn *,std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)const")]
#[doc(alias = "__ZNK3RBX16TypesetterBitmap4drawEPNS_5AdornERKSsRKN3G3D7Vector2EfRKNS5_6Color4ESB_NS_4Text6XAlignENSC_6YAlignES8_RKNS5_6Rect2DE")]
// IDA 0xbf8504: 445 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf8504() {
}

// 0xbf8a1c — __ZNK3RBX16TypesetterBitmap6layoutERKSsPSt6vectorINS0_9GlyphLineESaIS4_EEiRKN3G3D12Vector2int16EbPb
// type: 
#[doc(alias = "RBX::TypesetterBitmap::layout(std::string const&,std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>> *,int,G3D::Vector2int16 const&,bool,bool *)const")]
#[doc(alias = "__ZNK3RBX16TypesetterBitmap6layoutERKSsPSt6vectorINS0_9GlyphLineESaIS4_EEiRKN3G3D12Vector2int16EbPb")]
// IDA 0xbf8a1c: 312 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf8a1c() {
}

// 0xbf8d24 — __ZN3RBXL8drawRectEPNS_5AdornEbRKN3G3D6Rect2DES5_RKNS2_7Vector2ES8_RKNS2_6Color4E
// type: 
#[doc(alias = "RBX::drawRect(RBX::Adorn *,bool,G3D::Rect2D const&,G3D::Rect2D const&,G3D::Vector2 const&,G3D::Vector2 const&,G3D::Color4 const&)")]
#[doc(alias = "__ZN3RBXL8drawRectEPNS_5AdornEbRKN3G3D6Rect2DES5_RKNS2_7Vector2ES8_RKNS2_6Color4E")]
// IDA 0xbf8d24: 126 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf8d24() {
}

// 0xbf8ecc — __ZNK3RBX16TypesetterBitmap23getCursorPositionInTextERKSsRKN3G3D7Vector2EfNS_4Text6XAlignENS7_6YAlignES6_S4_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::TypesetterBitmap::getCursorPositionInText(std::string const&,G3D::Vector2 const&,float,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Vector2)const")]
#[doc(alias = "__ZNK3RBX16TypesetterBitmap23getCursorPositionInTextERKSsRKN3G3D7Vector2EfNS_4Text6XAlignENS7_6YAlignES6_S4_")]
// IDA 0xbf8ecc: 244 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf8ecc() {
}

// 0xbf9194 — __ZNK3RBX16TypesetterBitmap7measureERKSsfRKN3G3D7Vector2EPb
// type: _DWORD __fastcall(RBX::TypesetterBitmap *__hidden this, const std::string *, float, const G3D::Vector2 *, bool *)
#[doc(alias = "RBX::TypesetterBitmap::measure(std::string const&,float,G3D::Vector2 const&,bool *)const")]
#[doc(alias = "__ZNK3RBX16TypesetterBitmap7measureERKSsfRKN3G3D7Vector2EPb")]
// IDA 0xbf9194: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf9194() {
}
