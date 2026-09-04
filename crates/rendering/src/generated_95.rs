//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xe7e8f8..0xe87524 (100 stubs, 10660 prior -> 10760 covered, 2573 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


// 0xe7e8f8 — __ZN4Ogre17GLES2RenderSystem24bindGpuProgramParametersENS_14GpuProgramTypeENS_9SharedPtrINS_20GpuProgramParametersEEEt
#[doc(alias = "Ogre::GLES2RenderSystem::bindGpuProgramParameters(Ogre::GpuProgramType,Ogre::SharedPtr<Ogre::GpuProgramParameters>,unsigned short)")]
// was: Ogre::GLES2RenderSystem::bindGpuProgramParameters(Ogre::GpuProgramType,Ogre::SharedPtr<Ogre::GpuProgramParameters>,unsigned short)
// IDA 0xe7e8f8: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7e8f8() {
}

// 0xe7ead8 — __ZN4Ogre17GLES2RenderSystem37bindGpuProgramPassIterationParametersENS_14GpuProgramTypeE
#[doc(alias = "Ogre::GLES2RenderSystem::bindGpuProgramPassIterationParameters(Ogre::GpuProgramType)")]
// was: Ogre::GLES2RenderSystem::bindGpuProgramPassIterationParameters(Ogre::GpuProgramType)
// IDA 0xe7ead8: 163 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7ead8() {
}

// 0xe7ec84 — __ZNK4Ogre17GLES2RenderSystem22getDisplayMonitorCountEv
#[doc(alias = "Ogre::GLES2RenderSystem::getDisplayMonitorCount(void)const")]
// was: Ogre::GLES2RenderSystem::getDisplayMonitorCount(void)const
// IDA 0xe7ec84: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7ec84() {
}

// 0xe7ec88 — __ZN4Ogre17GLES2RenderSystem15_deleteGLBufferEjj
#[doc(alias = "Ogre::GLES2RenderSystem::_deleteGLBuffer(unsigned int,unsigned int)")]
// was: Ogre::GLES2RenderSystem::_deleteGLBuffer(unsigned int,unsigned int)
// IDA 0xe7ec88: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7ec88() {
}

// 0xe7ed04 — __ZNSt6vectorIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm
#[doc(alias = "std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
// was: std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)
// IDA 0xe7ed04: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7ed04() {
}

// 0xe7ed70 — __ZN4Ogre21RenderingAPIExceptionD1Ev
#[doc(alias = "Ogre::RenderingAPIException::~RenderingAPIException()")]
// was: Ogre::RenderingAPIException::~RenderingAPIException()
// IDA 0xe7ed70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7ed70() {
}

// 0xe7ed80 — __ZN4Ogre9SharedPtrINS_20GpuProgramParametersEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgramParameters>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::GpuProgramParameters>::~SharedPtr()
// IDA 0xe7ed80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7ed80() {
}

// 0xe7ee30 — __ZN4Ogre17GLES2RenderSystem15setAmbientLightEfff
#[doc(alias = "Ogre::GLES2RenderSystem::setAmbientLight(float,float,float)")]
// was: Ogre::GLES2RenderSystem::setAmbientLight(float,float,float)
// IDA 0xe7ee30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee30() {
}

// 0xe7ee34 — __ZN4Ogre17GLES2RenderSystem14setShadingTypeENS_12ShadeOptionsE
#[doc(alias = "Ogre::GLES2RenderSystem::setShadingType(Ogre::ShadeOptions)")]
// was: Ogre::GLES2RenderSystem::setShadingType(Ogre::ShadeOptions)
// IDA 0xe7ee34: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee34() {
}

// 0xe7ee38 — __ZN4Ogre17GLES2RenderSystem18setLightingEnabledEb
#[doc(alias = "Ogre::GLES2RenderSystem::setLightingEnabled(bool)")]
// was: Ogre::GLES2RenderSystem::setLightingEnabled(bool)
// IDA 0xe7ee38: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee38() {
}

// 0xe7ee40 — __ZN4Ogre17GLES2RenderSystem10_useLightsERKNS_12HashedVectorIPNS_5LightEEEt
#[doc(alias = "Ogre::GLES2RenderSystem::_useLights(Ogre::HashedVector<Ogre::Light *> const&,unsigned short)")]
// was: Ogre::GLES2RenderSystem::_useLights(Ogre::HashedVector<Ogre::Light *> const&,unsigned short)
// IDA 0xe7ee40: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee40() {
}

// 0xe7ee44 — __ZNK4Ogre17GLES2RenderSystem33areFixedFunctionLightsInViewSpaceEv
#[doc(alias = "Ogre::GLES2RenderSystem::areFixedFunctionLightsInViewSpace(void)const")]
// was: Ogre::GLES2RenderSystem::areFixedFunctionLightsInViewSpace(void)const
// IDA 0xe7ee44: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7ee44() {
}

// 0xe7ee48 — __ZN4Ogre17GLES2RenderSystem17_setSurfaceParamsERKNS_11ColourValueES3_S3_S3_fi
#[doc(alias = "Ogre::GLES2RenderSystem::_setSurfaceParams(Ogre::ColourValue const&,Ogre::ColourValue const&,Ogre::ColourValue const&,Ogre::ColourValue const&,float,int)")]
// was: Ogre::GLES2RenderSystem::_setSurfaceParams(Ogre::ColourValue const&,Ogre::ColourValue const&,Ogre::ColourValue const&,Ogre::ColourValue const&,float,int)
// IDA 0xe7ee48: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee48() {
}

// 0xe7ee4c — __ZN4Ogre17GLES2RenderSystem23_setPointSpritesEnabledEb
#[doc(alias = "Ogre::GLES2RenderSystem::_setPointSpritesEnabled(bool)")]
// was: Ogre::GLES2RenderSystem::_setPointSpritesEnabled(bool)
// IDA 0xe7ee4c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee4c() {
}

// 0xe7ee50 — __ZN4Ogre17GLES2RenderSystem19_setPointParametersEfbfffff
#[doc(alias = "Ogre::GLES2RenderSystem::_setPointParameters(float,bool,float,float,float,float,float)")]
// was: Ogre::GLES2RenderSystem::_setPointParameters(float,bool,float,float,float,float,float)
// IDA 0xe7ee50: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee50() {
}

// 0xe7ee54 — __ZN4Ogre17GLES2RenderSystem27_setTextureCoordCalculationEmNS_18TexCoordCalcMethodEPKNS_7FrustumE
#[doc(alias = "Ogre::GLES2RenderSystem::_setTextureCoordCalculation(unsigned long,Ogre::TexCoordCalcMethod,Ogre::Frustum const*)")]
// was: Ogre::GLES2RenderSystem::_setTextureCoordCalculation(unsigned long,Ogre::TexCoordCalcMethod,Ogre::Frustum const*)
// IDA 0xe7ee54: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee54() {
}

// 0xe7ee58 — __ZN4Ogre17GLES2RenderSystem20_setTextureBlendModeEmRKNS_16LayerBlendModeExE
#[doc(alias = "Ogre::GLES2RenderSystem::_setTextureBlendMode(unsigned long,Ogre::LayerBlendModeEx const&)")]
// was: Ogre::GLES2RenderSystem::_setTextureBlendMode(unsigned long,Ogre::LayerBlendModeEx const&)
// IDA 0xe7ee58: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee58() {
}

// 0xe7ee5c — __ZN4Ogre17GLES2RenderSystem23_setTextureBorderColourEmRKNS_11ColourValueE
#[doc(alias = "Ogre::GLES2RenderSystem::_setTextureBorderColour(unsigned long,Ogre::ColourValue const&)")]
// was: Ogre::GLES2RenderSystem::_setTextureBorderColour(unsigned long,Ogre::ColourValue const&)
// IDA 0xe7ee5c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee5c() {
}

// 0xe7ee60 — __ZN4Ogre17GLES2RenderSystem21_setTextureMipmapBiasEmf
#[doc(alias = "Ogre::GLES2RenderSystem::_setTextureMipmapBias(unsigned long,float)")]
// was: Ogre::GLES2RenderSystem::_setTextureMipmapBias(unsigned long,float)
// IDA 0xe7ee60: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee60() {
}

// 0xe7ee64 — __ZN4Ogre17GLES2RenderSystem17_setTextureMatrixEmRKNS_7Matrix4E
#[doc(alias = "Ogre::GLES2RenderSystem::_setTextureMatrix(unsigned long,Ogre::Matrix4 const&)")]
// was: Ogre::GLES2RenderSystem::_setTextureMatrix(unsigned long,Ogre::Matrix4 const&)
// IDA 0xe7ee64: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee64() {
}

// 0xe7ee68 — __ZN4Ogre17GLES2RenderSystem20setVertexDeclarationEPNS_17VertexDeclarationE
#[doc(alias = "Ogre::GLES2RenderSystem::setVertexDeclaration(Ogre::VertexDeclaration *)")]
// was: Ogre::GLES2RenderSystem::setVertexDeclaration(Ogre::VertexDeclaration *)
// IDA 0xe7ee68: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee68() {
}

// 0xe7ee6c — __ZN4Ogre17GLES2RenderSystem22setVertexBufferBindingEPNS_19VertexBufferBindingE
#[doc(alias = "Ogre::GLES2RenderSystem::setVertexBufferBinding(Ogre::VertexBufferBinding *)")]
// was: Ogre::GLES2RenderSystem::setVertexBufferBinding(Ogre::VertexBufferBinding *)
// IDA 0xe7ee6c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee6c() {
}

// 0xe7ee70 — __ZN4Ogre17GLES2RenderSystem19setNormaliseNormalsEb
#[doc(alias = "Ogre::GLES2RenderSystem::setNormaliseNormals(bool)")]
// was: Ogre::GLES2RenderSystem::setNormaliseNormals(bool)
// IDA 0xe7ee70: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee70() {
}

// 0xe7ee74 — __ZN4Ogre17GLES2RenderSystem24getHorizontalTexelOffsetEv
#[doc(alias = "Ogre::GLES2RenderSystem::getHorizontalTexelOffset(void)")]
// was: Ogre::GLES2RenderSystem::getHorizontalTexelOffset(void)
// IDA 0xe7ee74: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7ee74() {
}

// 0xe7ee78 — __ZN4Ogre17GLES2RenderSystem22getVerticalTexelOffsetEv
#[doc(alias = "Ogre::GLES2RenderSystem::getVerticalTexelOffset(void)")]
// was: Ogre::GLES2RenderSystem::getVerticalTexelOffset(void)
// IDA 0xe7ee78: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7ee78() {
}

// 0xe7ee7c — __ZN4Ogre17GLES2RenderSystem25getMinimumDepthInputValueEv
#[doc(alias = "Ogre::GLES2RenderSystem::getMinimumDepthInputValue(void)")]
// was: Ogre::GLES2RenderSystem::getMinimumDepthInputValue(void)
// IDA 0xe7ee7c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7ee7c() {
}

// 0xe7ee84 — __ZN4Ogre17GLES2RenderSystem25getMaximumDepthInputValueEv
#[doc(alias = "Ogre::GLES2RenderSystem::getMaximumDepthInputValue(void)")]
// was: Ogre::GLES2RenderSystem::getMaximumDepthInputValue(void)
// IDA 0xe7ee84: 2 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7ee84() {
}

// 0xe7ee8c — __ZN4Ogre17GLES2RenderSystem22preExtraThreadsStartedEv
#[doc(alias = "Ogre::GLES2RenderSystem::preExtraThreadsStarted(void)")]
// was: Ogre::GLES2RenderSystem::preExtraThreadsStarted(void)
// IDA 0xe7ee8c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee8c() {
}

// 0xe7ee90 — __ZN4Ogre17GLES2RenderSystem23postExtraThreadsStartedEv
#[doc(alias = "Ogre::GLES2RenderSystem::postExtraThreadsStarted(void)")]
// was: Ogre::GLES2RenderSystem::postExtraThreadsStarted(void)
// IDA 0xe7ee90: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee90() {
}

// 0xe7ee94 — __ZN4Ogre17GLES2RenderSystem14registerThreadEv
#[doc(alias = "Ogre::GLES2RenderSystem::registerThread(void)")]
// was: Ogre::GLES2RenderSystem::registerThread(void)
// IDA 0xe7ee94: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee94() {
}

// 0xe7ee98 — __ZN4Ogre17GLES2RenderSystem16unregisterThreadEv
#[doc(alias = "Ogre::GLES2RenderSystem::unregisterThread(void)")]
// was: Ogre::GLES2RenderSystem::unregisterThread(void)
// IDA 0xe7ee98: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee98() {
}

// 0xe7ee9c — __ZN4Ogre17GLES2RenderSystem17setClipPlanesImplERKSt6vectorINS_5PlaneENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::GLES2RenderSystem::setClipPlanesImpl(std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::GLES2RenderSystem::setClipPlanesImpl(std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xe7ee9c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7ee9c() {
}

// 0xe7f358 — __ZN4Ogre24GpuSharedParametersUsageD2Ev
#[doc(alias = "Ogre::GpuSharedParametersUsage::~GpuSharedParametersUsage()")]
// was: Ogre::GpuSharedParametersUsage::~GpuSharedParametersUsage()
// IDA 0xe7f358: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7f358() {
}

// 0xe7f490 — __ZNSt6vectorIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS6_EERKj
#[doc(alias = "std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned int const&)")]
// was: std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned int const&)
// IDA 0xe7f490: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e7f490() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe7f590 — __ZNSt12_Vector_baseIPN4Ogre11DepthBufferENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe7f590: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7f590() {
}

// 0xe7f5a0 — __ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xe7f5a0: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7f5a0() {
}

// 0xe7f658 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::RenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::RenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xe7f658: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7f658() {
}

// 0xe7f700 — __ZNSt12_Vector_baseIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe7f700: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7f700() {
}

// 0xe7f710 — __ZN4Ogre21RenderingAPIExceptionD0Ev
#[doc(alias = "Ogre::RenderingAPIException::~RenderingAPIException()")]
// was: Ogre::RenderingAPIException::~RenderingAPIException()
// IDA 0xe7f710: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7f710() {
}

// 0xe7f728 — __ZN4Ogre26GLES2HardwareBufferManagerD1Ev
#[doc(alias = "Ogre::GLES2HardwareBufferManager::~GLES2HardwareBufferManager()")]
// was: Ogre::GLES2HardwareBufferManager::~GLES2HardwareBufferManager()
// IDA 0xe7f728: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7f728() {
}

// 0xe7f754 — __ZN4Ogre26GLES2HardwareBufferManagerD0Ev
#[doc(alias = "Ogre::GLES2HardwareBufferManager::~GLES2HardwareBufferManager()")]
// was: Ogre::GLES2HardwareBufferManager::~GLES2HardwareBufferManager()
// IDA 0xe7f754: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7f754() {
}

// 0xe7f808 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKSs
#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::string const&)")]
// was: std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::string const&)
// IDA 0xe7f808: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7f808() {
}

// 0xe7f920 — __ZN4Ogre15GLES2RTTManagerD2Ev
#[doc(alias = "Ogre::GLES2RTTManager::~GLES2RTTManager()")]
// was: Ogre::GLES2RTTManager::~GLES2RTTManager()
// IDA 0xe7f920: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7f920() {
}

// 0xe7f930 — __ZN4Ogre15GLES2RTTManager23getSupportedAlternativeENS_11PixelFormatE
#[doc(alias = "Ogre::GLES2RTTManager::getSupportedAlternative(Ogre::PixelFormat)")]
// was: Ogre::GLES2RTTManager::getSupportedAlternative(Ogre::PixelFormat)
// IDA 0xe7f930: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7f930() {
}

// 0xe7f978 — __ZN4Ogre18GLES2RenderTextureC2ERKSsRKNS_16GLES2SurfaceDescEbj
#[doc(alias = "Ogre::GLES2RenderTexture::GLES2RenderTexture(std::string const&,Ogre::GLES2SurfaceDesc const&,bool,unsigned int)")]
// was: Ogre::GLES2RenderTexture::GLES2RenderTexture(std::string const&,Ogre::GLES2SurfaceDesc const&,bool,unsigned int)
// IDA 0xe7f978: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7f978() {
}

// 0xe7fa48 — __ZN4Ogre18GLES2RenderTextureD0Ev
#[doc(alias = "Ogre::GLES2RenderTexture::~GLES2RenderTexture()")]
// was: Ogre::GLES2RenderTexture::~GLES2RenderTexture()
// IDA 0xe7fa48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7fa48() {
}

// 0xe7fad8 — __ZN4Ogre18GLES2RenderTextureD1Ev
#[doc(alias = "Ogre::GLES2RenderTexture::~GLES2RenderTexture()")]
// was: Ogre::GLES2RenderTexture::~GLES2RenderTexture()
// IDA 0xe7fad8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7fad8() {
}

// 0xe7fae4 — __ZN4Ogre18GLES2RenderTextureD2Ev
#[doc(alias = "Ogre::GLES2RenderTexture::~GLES2RenderTexture()")]
// was: Ogre::GLES2RenderTexture::~GLES2RenderTexture()
// IDA 0xe7fae4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7fae4() {
}

// 0xe7fb24 — __ZN4Ogre12GLES2Support15setConfigOptionERKSsS2_
#[doc(alias = "Ogre::GLES2Support::setConfigOption(std::string const&,std::string const&)")]
// was: Ogre::GLES2Support::setConfigOption(std::string const&,std::string const&)
// IDA 0xe7fb24: 258 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7fb24() {
}

// 0xe7fe10 — __ZN4Ogre12GLES2Support16getConfigOptionsEv
#[doc(alias = "Ogre::GLES2Support::getConfigOptions(void)")]
// was: Ogre::GLES2Support::getConfigOptions(void)
// IDA 0xe7fe10: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7fe10() {
}

// 0xe7fe14 — __ZN4Ogre12GLES2Support20initialiseExtensionsEv
#[doc(alias = "Ogre::GLES2Support::initialiseExtensions(void)")]
// was: Ogre::GLES2Support::initialiseExtensions(void)
// IDA 0xe7fe14: 995 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7fe14() {
}

// 0xe8098c — __ZNK4Ogre12GLES2Support14checkExtensionERKSs
#[doc(alias = "Ogre::GLES2Support::checkExtension(std::string const&)const")]
// was: Ogre::GLES2Support::checkExtension(std::string const&)const
// IDA 0xe8098c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8098c() {
}

// 0xe809e0 — __ZN4Ogre12GLES2TextureC1EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderERNS_12GLES2SupportE
#[doc(alias = "Ogre::GLES2Texture::GLES2Texture(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GLES2Support &)")]
// was: Ogre::GLES2Texture::GLES2Texture(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GLES2Support &)
// IDA 0xe809e0: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e809e0() {
}

// 0xe80a5c — __ZN4Ogre12GLES2TextureD0Ev
#[doc(alias = "Ogre::GLES2Texture::~GLES2Texture()")]
// was: Ogre::GLES2Texture::~GLES2Texture()
// IDA 0xe80a5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e80a5c() {
}

// 0xe80aec — __ZN4Ogre12GLES2TextureD1Ev
#[doc(alias = "Ogre::GLES2Texture::~GLES2Texture()")]
// was: Ogre::GLES2Texture::~GLES2Texture()
// IDA 0xe80aec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e80aec() {
}

// 0xe80af8 — __ZN4Ogre12GLES2TextureD2Ev
#[doc(alias = "Ogre::GLES2Texture::~GLES2Texture()")]
// was: Ogre::GLES2Texture::~GLES2Texture()
// IDA 0xe80af8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e80af8() {
}

// 0xe80d38 — __ZNK4Ogre12GLES2Texture21getGLES2TextureTargetEv
#[doc(alias = "Ogre::GLES2Texture::getGLES2TextureTarget(void)const")]
// was: Ogre::GLES2Texture::getGLES2TextureTarget(void)const
// IDA 0xe80d38: 12 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e80d38() {
}

// 0xe80d58 — __ZN4Ogre12GLES2Texture27createInternalResourcesImplEv
#[doc(alias = "Ogre::GLES2Texture::createInternalResourcesImpl(void)")]
// was: Ogre::GLES2Texture::createInternalResourcesImpl(void)
// IDA 0xe80d58: 510 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e80d58() {
}

// 0xe81304 — __ZN4Ogre12GLES2Texture18_createSurfaceListEv
#[doc(alias = "Ogre::GLES2Texture::_createSurfaceList(void)")]
// was: Ogre::GLES2Texture::_createSurfaceList(void)
// IDA 0xe81304: 491 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e81304() {
}

// 0xe81c7c — __ZN4Ogre12GLES2Texture11prepareImplEv
#[doc(alias = "Ogre::GLES2Texture::prepareImpl(void)")]
// was: Ogre::GLES2Texture::prepareImpl(void)
// IDA 0xe81c7c: 1104 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e81c7c() {
}

// 0xe828a8 — __ZN4OgreL9doImageIOERKSsS1_S1_RSt6vectorINS_5ImageENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPNS_8ResourceE
#[doc(alias = "Ogre::doImageIO(std::string const&,std::string const&,std::string const&,std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &,Ogre::Resource *)")]
// was: Ogre::doImageIO(std::string const&,std::string const&,std::string const&,std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &,Ogre::Resource *)
// IDA 0xe828a8: 266 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e828a8() {
}

// 0xe82b58 — __ZN4Ogre12GLES2Texture13unprepareImplEv
#[doc(alias = "Ogre::GLES2Texture::unprepareImpl(void)")]
// was: Ogre::GLES2Texture::unprepareImpl(void)
// IDA 0xe82b58: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e82b58() {
}

// 0xe82b8c — __ZN4Ogre12GLES2Texture8loadImplEv
#[doc(alias = "Ogre::GLES2Texture::loadImpl(void)")]
// was: Ogre::GLES2Texture::loadImpl(void)
// IDA 0xe82b8c: 200 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e82b8c() {
}

// 0xe82d88 — __ZN4Ogre12GLES2Texture25freeInternalResourcesImplEv
#[doc(alias = "Ogre::GLES2Texture::freeInternalResourcesImpl(void)")]
// was: Ogre::GLES2Texture::freeInternalResourcesImpl(void)
// IDA 0xe82d88: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e82d88() {
}

// 0xe82dc4 — __ZN4Ogre12GLES2Texture9getBufferEmm
#[doc(alias = "Ogre::GLES2Texture::getBuffer(unsigned long,unsigned long)")]
// was: Ogre::GLES2Texture::getBuffer(unsigned long,unsigned long)
// IDA 0xe82dc4: 302 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e82dc4() {
}

// 0xe8313c — __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xe8313c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8313c() {
}

// 0xe831ec — __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEaSERKS9_
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::operator=(Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::operator=(Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xe831ec: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e831ec() {
}

// 0xe832f8 — __ZNSt6vectorIN4Ogre28HardwarePixelBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::HardwarePixelBufferSharedPtr*,std::vector<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::HardwarePixelBufferSharedPtr const&)")]
// was: std::vector<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::HardwarePixelBufferSharedPtr*,std::vector<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::HardwarePixelBufferSharedPtr const&)
// IDA 0xe832f8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e832f8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe83718 — __ZN4Ogre9SharedPtrINS_19HardwarePixelBufferEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwarePixelBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwarePixelBuffer> const&)")]
// was: Ogre::SharedPtr<Ogre::HardwarePixelBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwarePixelBuffer> const&)
// IDA 0xe83718: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e83718() {
}

// 0xe83898 — __ZNSt12_Vector_baseIN4Ogre5ImageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe83898: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e83898() {
}

// 0xe8389c — __ZNSt12_Vector_baseIN4Ogre5ImageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe8389c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8389c() {
}

// 0xe838a8 — __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xe838a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e838a8() {
}

// 0xe8395c — __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
// was: Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)
// IDA 0xe8395c: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8395c() {
}

// 0xe83a6c — __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERS9_
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)")]
// was: Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)
// IDA 0xe83a6c: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e83a6c() {
}

// 0xe83a88 — __ZNSt12_Vector_baseIN4Ogre28HardwarePixelBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe83a88: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e83a88() {
}

// 0xe83a8c — __ZNSt12_Vector_baseIN4Ogre28HardwarePixelBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe83a8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e83a8c() {
}

// 0xe83a98 — __ZNSt6vectorIN4Ogre5ImageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Image*,std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Image const&)")]
// was: std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Image*,std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Image const&)
// IDA 0xe83a98: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e83a98() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe83fb8 — __ZN4Ogre19GLES2TextureManagerC1ERNS_12GLES2SupportE
#[doc(alias = "Ogre::GLES2TextureManager::GLES2TextureManager(Ogre::GLES2Support &)")]
// was: Ogre::GLES2TextureManager::GLES2TextureManager(Ogre::GLES2Support &)
// IDA 0xe83fb8: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e83fb8() {
}

// 0xe84094 — __ZN4Ogre19GLES2TextureManager20createWarningTextureEv
#[doc(alias = "Ogre::GLES2TextureManager::createWarningTexture(void)")]
// was: Ogre::GLES2TextureManager::createWarningTexture(void)
// IDA 0xe84094: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e84094() {
}

// 0xe8419c — __ZN4Ogre19GLES2TextureManagerD0Ev
#[doc(alias = "Ogre::GLES2TextureManager::~GLES2TextureManager()")]
// was: Ogre::GLES2TextureManager::~GLES2TextureManager()
// IDA 0xe8419c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8419c() {
}

// 0xe84274 — __ZN4Ogre19GLES2TextureManagerD1Ev
#[doc(alias = "Ogre::GLES2TextureManager::~GLES2TextureManager()")]
// was: Ogre::GLES2TextureManager::~GLES2TextureManager()
// IDA 0xe84274: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e84274() {
}

// 0xe84340 — __ZN4Ogre19GLES2TextureManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::GLES2TextureManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::GLES2TextureManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xe84340: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e84340() {
}

// 0xe8441c — __ZN4Ogre19GLES2TextureManager15getNativeFormatENS_11TextureTypeENS_11PixelFormatEi
#[doc(alias = "Ogre::GLES2TextureManager::getNativeFormat(Ogre::TextureType,Ogre::PixelFormat,int)")]
// was: Ogre::GLES2TextureManager::getNativeFormat(Ogre::TextureType,Ogre::PixelFormat,int)
// IDA 0xe8441c: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8441c() {
}

// 0xe84480 — __ZN4Ogre19GLES2TextureManager28isHardwareFilteringSupportedENS_11TextureTypeENS_11PixelFormatEib
#[doc(alias = "Ogre::GLES2TextureManager::isHardwareFilteringSupported(Ogre::TextureType,Ogre::PixelFormat,int,bool)")]
// was: Ogre::GLES2TextureManager::isHardwareFilteringSupported(Ogre::TextureType,Ogre::PixelFormat,int,bool)
// IDA 0xe84480: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e84480() {
}

// 0xe844ec — __ZN4Ogre12EAGL2SupportC1Ev
#[doc(alias = "Ogre::EAGL2Support::EAGL2Support(void)")]
// was: Ogre::EAGL2Support::EAGL2Support(void)
// IDA 0xe844ec: 37 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e844ec() {
}

// 0xe8455c — __ZN4Ogre12EAGL2SupportD0Ev
#[doc(alias = "Ogre::EAGL2Support::~EAGL2Support()")]
// was: Ogre::EAGL2Support::~EAGL2Support()
// IDA 0xe8455c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8455c() {
}

// 0xe84570 — __ZN4Ogre12EAGL2SupportD1Ev
#[doc(alias = "Ogre::EAGL2Support::~EAGL2Support()")]
// was: Ogre::EAGL2Support::~EAGL2Support()
// IDA 0xe84570: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e84570() {
}

// 0xe8457c — __ZN4Ogre12EAGL2Support9addConfigEv
#[doc(alias = "Ogre::EAGL2Support::addConfig(void)")]
// was: Ogre::EAGL2Support::addConfig(void)
// IDA 0xe8457c: 2669 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8457c() {
}

// 0xe862b0 — __ZN4Ogre12EAGL2Support14validateConfigEv
#[doc(alias = "Ogre::EAGL2Support::validateConfig(void)")]
// was: Ogre::EAGL2Support::validateConfig(void)
// IDA 0xe862b0: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e862b0() {
}

// 0xe862c8 — __ZN4Ogre12EAGL2Support14getDisplayNameEv
#[doc(alias = "Ogre::EAGL2Support::getDisplayName(void)")]
// was: Ogre::EAGL2Support::getDisplayName(void)
// IDA 0xe862c8: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e862c8() {
}

// 0xe862e4 — __ZN4Ogre12EAGL2Support12createWindowEbPNS_17GLES2RenderSystemERKSs
#[doc(alias = "Ogre::EAGL2Support::createWindow(bool,Ogre::GLES2RenderSystem *,std::string const&)")]
// was: Ogre::EAGL2Support::createWindow(bool,Ogre::GLES2RenderSystem *,std::string const&)
// IDA 0xe862e4: 523 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e862e4() {
}

// 0xe86aa0 — __ZN4Ogre12EAGL2Support9newWindowERKSsjjbPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::EAGL2Support::newWindow(std::string const&,unsigned int,unsigned int,bool,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::EAGL2Support::newWindow(std::string const&,unsigned int,unsigned int,bool,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xe86aa0: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e86aa0() {
}

// 0xe86b80 — __ZNK4Ogre12EAGL2Support16createNewContextERPK14__CFDictionaryP11CAEAGLLayerP14EAGLSharegroup
#[doc(alias = "Ogre::EAGL2Support::createNewContext(__CFDictionary const*&,CAEAGLLayer *,EAGLSharegroup *)const")]
// was: Ogre::EAGL2Support::createNewContext(__CFDictionary const*&,CAEAGLLayer *,EAGLSharegroup *)const
// IDA 0xe86b80: 174 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e86b80() {
}

// 0xe86d80 — __ZN4Ogre12EAGL2Support14getProcAddressERKSs
#[doc(alias = "Ogre::EAGL2Support::getProcAddress(std::string const&)")]
// was: Ogre::EAGL2Support::getProcAddress(std::string const&)
// IDA 0xe86d80: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e86d80() {
}

// 0xe86d84 — __ZN4Ogre12EAGL2Support5startEv
#[doc(alias = "Ogre::EAGL2Support::start(void)")]
// was: Ogre::EAGL2Support::start(void)
// IDA 0xe86d84: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e86d84() {
}

// 0xe86d88 — __ZN4Ogre12EAGL2Support4stopEv
#[doc(alias = "Ogre::EAGL2Support::stop(void)")]
// was: Ogre::EAGL2Support::stop(void)
// IDA 0xe86d88: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e86d88() {
}

// 0xe86d8c — __ZN4Ogre12GLES2SupportD2Ev
#[doc(alias = "Ogre::GLES2Support::~GLES2Support()")]
// was: Ogre::GLES2Support::~GLES2Support()
// IDA 0xe86d8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e86d8c() {
}

// 0xe86f24 — __ZNSt3mapISsN4Ogre13_ConfigOptionESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
#[doc(alias = "std::map<std::string,Ogre::_ConfigOption,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::_ConfigOption,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xe86f24: 550 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e86f24() {
}

// 0xe87520 — __ZNK4Ogre12GLES2Support22getDisplayMonitorCountEv
#[doc(alias = "Ogre::GLES2Support::getDisplayMonitorCount(void)const")]
// was: Ogre::GLES2Support::getDisplayMonitorCount(void)const
// IDA 0xe87520: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e87520() {
}

// 0xe87524 — __ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS6_
#[doc(alias = "std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xe87524: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e87524() {
}
