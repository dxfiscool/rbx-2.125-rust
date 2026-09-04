//! rendering — next 100 Ogre stubs (EA-sorted strict Ogre:: filter)
//! Filter: Ogre (9822 total, 2466 prior strict Ogre stubbed, +100 this batch) — 0xcbee38..0xcc658c after 0xcbee34 (remaining 7256 after batch)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xcbee38 — __ZN4Ogre8Resource14postUnloadImplEv
#[doc(alias = "Ogre::Resource::postUnloadImpl(void)")]
// was: Ogre::Resource::postUnloadImpl(void)
// IDA 0xcbee38: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cbee38() {
}

// 0xcbee3c — __ZN4Ogre8Resource13unprepareImplEv
#[doc(alias = "Ogre::Resource::unprepareImpl(void)")]
// was: Ogre::Resource::unprepareImpl(void)
// IDA 0xcbee3c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cbee3c() {
}

// 0xcbee40 — __ZNK4Ogre10GpuProgram13calculateSizeEv
#[doc(alias = "Ogre::GpuProgram::calculateSize(void)const")]
// was: Ogre::GpuProgram::calculateSize(void)const
// IDA 0xcbee40: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee40() {
}

// 0xcbee44 — __ZNK4Ogre10GpuProgram13getSyntaxCodeEv
#[doc(alias = "Ogre::GpuProgram::getSyntaxCode(void)const")]
// was: Ogre::GpuProgram::getSyntaxCode(void)const
// IDA 0xcbee44: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee44() {
}

// 0xcbee48 — __ZNK4Ogre10GpuProgram13getSourceFileEv
#[doc(alias = "Ogre::GpuProgram::getSourceFile(void)const")]
// was: Ogre::GpuProgram::getSourceFile(void)const
// IDA 0xcbee48: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee48() {
}

// 0xcbee4c — __ZNK4Ogre10GpuProgram9getSourceEv
#[doc(alias = "Ogre::GpuProgram::getSource(void)const")]
// was: Ogre::GpuProgram::getSource(void)const
// IDA 0xcbee4c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee4c() {
}

// 0xcbee50 — __ZNK4Ogre10GpuProgram7getTypeEv
#[doc(alias = "Ogre::GpuProgram::getType(void)const")]
// was: Ogre::GpuProgram::getType(void)const
// IDA 0xcbee50: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee50() {
}

// 0xcbee54 — __ZN4Ogre19HighLevelGpuProgram19_getBindingDelegateEv
#[doc(alias = "Ogre::HighLevelGpuProgram::_getBindingDelegate(void)")]
// was: Ogre::HighLevelGpuProgram::_getBindingDelegate(void)
// IDA 0xcbee54: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee54() {
}

// 0xcbee5c — __ZNK4Ogre11NullProgram11isSupportedEv
#[doc(alias = "Ogre::NullProgram::isSupported(void)const")]
// was: Ogre::NullProgram::isSupported(void)const
// IDA 0xcbee5c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee5c() {
}

// 0xcbee60 — __ZN4Ogre10GpuProgram28setSkeletalAnimationIncludedEb
#[doc(alias = "Ogre::GpuProgram::setSkeletalAnimationIncluded(bool)")]
// was: Ogre::GpuProgram::setSkeletalAnimationIncluded(bool)
// IDA 0xcbee60: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee60() {
}

// 0xcbee68 — __ZNK4Ogre10GpuProgram27isSkeletalAnimationIncludedEv
#[doc(alias = "Ogre::GpuProgram::isSkeletalAnimationIncluded(void)const")]
// was: Ogre::GpuProgram::isSkeletalAnimationIncluded(void)const
// IDA 0xcbee68: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee68() {
}

// 0xcbee70 — __ZN4Ogre10GpuProgram25setMorphAnimationIncludedEb
#[doc(alias = "Ogre::GpuProgram::setMorphAnimationIncluded(bool)")]
// was: Ogre::GpuProgram::setMorphAnimationIncluded(bool)
// IDA 0xcbee70: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee70() {
}

// 0xcbee78 — __ZN4Ogre10GpuProgram24setPoseAnimationIncludedEt
#[doc(alias = "Ogre::GpuProgram::setPoseAnimationIncluded(unsigned short)")]
// was: Ogre::GpuProgram::setPoseAnimationIncluded(unsigned short)
// IDA 0xcbee78: 2 insns (STRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee78() {
}

// 0xcbee80 — __ZNK4Ogre10GpuProgram24isMorphAnimationIncludedEv
#[doc(alias = "Ogre::GpuProgram::isMorphAnimationIncluded(void)const")]
// was: Ogre::GpuProgram::isMorphAnimationIncluded(void)const
// IDA 0xcbee80: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee80() {
}

// 0xcbee88 — __ZNK4Ogre10GpuProgram23isPoseAnimationIncludedEv
#[doc(alias = "Ogre::GpuProgram::isPoseAnimationIncluded(void)const")]
// was: Ogre::GpuProgram::isPoseAnimationIncluded(void)const
// IDA 0xcbee88: 5 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee88() {
}

// 0xcbee94 — __ZNK4Ogre10GpuProgram24getNumberOfPosesIncludedEv
#[doc(alias = "Ogre::GpuProgram::getNumberOfPosesIncluded(void)const")]
// was: Ogre::GpuProgram::getNumberOfPosesIncluded(void)const
// IDA 0xcbee94: 2 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee94() {
}

// 0xcbee9c — __ZN4Ogre10GpuProgram29setVertexTextureFetchRequiredEb
#[doc(alias = "Ogre::GpuProgram::setVertexTextureFetchRequired(bool)")]
// was: Ogre::GpuProgram::setVertexTextureFetchRequired(bool)
// IDA 0xcbee9c: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee9c() {
}

// 0xcbeea4 — __ZNK4Ogre10GpuProgram28isVertexTextureFetchRequiredEv
#[doc(alias = "Ogre::GpuProgram::isVertexTextureFetchRequired(void)const")]
// was: Ogre::GpuProgram::isVertexTextureFetchRequired(void)const
// IDA 0xcbeea4: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbeea4() {
}

// 0xcbeeac — __ZN4Ogre10GpuProgram24setAdjacencyInfoRequiredEb
#[doc(alias = "Ogre::GpuProgram::setAdjacencyInfoRequired(bool)")]
// was: Ogre::GpuProgram::setAdjacencyInfoRequired(bool)
// IDA 0xcbeeac: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbeeac() {
}

// 0xcbeeb4 — __ZNK4Ogre10GpuProgram23isAdjacencyInfoRequiredEv
#[doc(alias = "Ogre::GpuProgram::isAdjacencyInfoRequired(void)const")]
// was: Ogre::GpuProgram::isAdjacencyInfoRequired(void)const
// IDA 0xcbeeb4: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbeeb4() {
}

// 0xcbeebc — __ZNK4Ogre10GpuProgram20hasDefaultParametersEv
#[doc(alias = "Ogre::GpuProgram::hasDefaultParameters(void)const")]
// was: Ogre::GpuProgram::hasDefaultParameters(void)const
// IDA 0xcbeebc: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbeebc() {
}

// 0xcbeec8 — __ZNK4Ogre10GpuProgram28getPassSurfaceAndLightStatesEv
#[doc(alias = "Ogre::GpuProgram::getPassSurfaceAndLightStates(void)const")]
// was: Ogre::GpuProgram::getPassSurfaceAndLightStates(void)const
// IDA 0xcbeec8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbeec8() {
}

// 0xcbeecc — __ZNK4Ogre10GpuProgram16getPassFogStatesEv
#[doc(alias = "Ogre::GpuProgram::getPassFogStates(void)const")]
// was: Ogre::GpuProgram::getPassFogStates(void)const
// IDA 0xcbeecc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbeecc() {
}

// 0xcbeed0 — __ZNK4Ogre10GpuProgram22getPassTransformStatesEv
#[doc(alias = "Ogre::GpuProgram::getPassTransformStates(void)const")]
// was: Ogre::GpuProgram::getPassTransformStates(void)const
// IDA 0xcbeed0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbeed0() {
}

// 0xcbeed4 — __ZNK4Ogre11NullProgram11getLanguageEv
#[doc(alias = "Ogre::NullProgram::getLanguage(void)const")]
// was: Ogre::NullProgram::getLanguage(void)const
// IDA 0xcbeed4: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbeed4() {
}

// 0xcbeee0 — __ZNK4Ogre10GpuProgram15hasCompileErrorEv
#[doc(alias = "Ogre::GpuProgram::hasCompileError(void)const")]
// was: Ogre::GpuProgram::hasCompileError(void)const
// IDA 0xcbeee0: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbeee0() {
}

// 0xcbeee8 — __ZN4Ogre10GpuProgram17resetCompileErrorEv
#[doc(alias = "Ogre::GpuProgram::resetCompileError(void)")]
// was: Ogre::GpuProgram::resetCompileError(void)
// IDA 0xcbeee8: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbeee8() {
}

// 0xcbeef0 — __ZNK4Ogre19HighLevelGpuProgram17getNamedConstantsEv
#[doc(alias = "Ogre::HighLevelGpuProgram::getNamedConstants(void)const")]
// was: Ogre::HighLevelGpuProgram::getNamedConstants(void)const
// IDA 0xcbeef0: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbeef0() {
}

// 0xcbef00 — __ZNK4Ogre10GpuProgram27getManualNamedConstantsFileEv
#[doc(alias = "Ogre::GpuProgram::getManualNamedConstantsFile(void)const")]
// was: Ogre::GpuProgram::getManualNamedConstantsFile(void)const
// IDA 0xcbef00: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbef00() {
}

// 0xcbef04 — __ZN4Ogre11NullProgram14loadFromSourceEv
#[doc(alias = "Ogre::NullProgram::loadFromSource(void)")]
// was: Ogre::NullProgram::loadFromSource(void)
// IDA 0xcbef04: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cbef04() {
}

// 0xcbef08 — __ZN4Ogre11NullProgram18createLowLevelImplEv
#[doc(alias = "Ogre::NullProgram::createLowLevelImpl(void)")]
// was: Ogre::NullProgram::createLowLevelImpl(void)
// IDA 0xcbef08: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cbef08() {
}

// 0xcbef0c — __ZN4Ogre11NullProgram19unloadHighLevelImplEv
#[doc(alias = "Ogre::NullProgram::unloadHighLevelImpl(void)")]
// was: Ogre::NullProgram::unloadHighLevelImpl(void)
// IDA 0xcbef0c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cbef0c() {
}

// 0xcbef10 — __ZN4Ogre11NullProgram22populateParameterNamesENS_9SharedPtrINS_20GpuProgramParametersEEE
#[doc(alias = "Ogre::NullProgram::populateParameterNames(Ogre::SharedPtr<Ogre::GpuProgramParameters>)")]
// was: Ogre::NullProgram::populateParameterNames(Ogre::SharedPtr<Ogre::GpuProgramParameters>)
// IDA 0xcbef10: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbef10() {
}

// 0xcbef1c — __ZNK4Ogre11NullProgram24buildConstantDefinitionsEv
#[doc(alias = "Ogre::NullProgram::buildConstantDefinitions(void)const")]
// was: Ogre::NullProgram::buildConstantDefinitions(void)const
// IDA 0xcbef1c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cbef1c() {
}

// 0xcbef8c — __ZN4Ogre10ImageCodecD2Ev
#[doc(alias = "Ogre::ImageCodec::~ImageCodec()")]
// was: Ogre::ImageCodec::~ImageCodec()
// IDA 0xcbef8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbef8c() {
}

// 0xcbef98 — __ZN4Ogre5ImageC1Ev
#[doc(alias = "Ogre::Image::Image(void)")]
// was: Ogre::Image::Image(void)
// IDA 0xcbef98: 15 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbef98() {
}

// 0xcbefc4 — __ZN4Ogre5ImageC1ERKS0_
#[doc(alias = "Ogre::Image::Image(Ogre::Image const&)")]
// was: Ogre::Image::Image(Ogre::Image const&)
// IDA 0xcbefc4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbefc4() {
}

// 0xcbefec — __ZN4Ogre5ImageaSERKS0_
#[doc(alias = "Ogre::Image::operator=(Ogre::Image const&)")]
// was: Ogre::Image::operator=(Ogre::Image const&)
// IDA 0xcbefec: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbefec() {
}

// 0xcbf058 — __ZN4Ogre5ImageD0Ev
#[doc(alias = "Ogre::Image::~Image()")]
// was: Ogre::Image::~Image()
// IDA 0xcbf058: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbf058() {
}

// 0xcbf114 — __ZN4Ogre5ImageD1Ev
#[doc(alias = "Ogre::Image::~Image()")]
// was: Ogre::Image::~Image()
// IDA 0xcbf114: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbf114() {
}

// 0xcbf1c8 — __ZN4Ogre5Image16loadDynamicImageEPhmmmNS_11PixelFormatEbmm
#[doc(alias = "Ogre::Image::loadDynamicImage(unsigned char *,unsigned long,unsigned long,unsigned long,Ogre::PixelFormat,bool,unsigned long,unsigned long)")]
// was: Ogre::Image::loadDynamicImage(unsigned char *,unsigned long,unsigned long,unsigned long,Ogre::PixelFormat,bool,unsigned long,unsigned long)
// IDA 0xcbf1c8: 239 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbf1c8() {
}

// 0xcbf490 — __ZN4Ogre5Image13calculateSizeEmmmmmNS_11PixelFormatE
#[doc(alias = "Ogre::Image::calculateSize(unsigned long,unsigned long,unsigned long,unsigned long,unsigned long,Ogre::PixelFormat)")]
// was: Ogre::Image::calculateSize(unsigned long,unsigned long,unsigned long,unsigned long,unsigned long,Ogre::PixelFormat)
// IDA 0xcbf490: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbf490() {
}

// 0xcbf4e8 — __ZN4Ogre5Image11loadRawDataERNS_9SharedPtrINS_10DataStreamEEEmmmNS_11PixelFormatEmm
#[doc(alias = "Ogre::Image::loadRawData(Ogre::SharedPtr<Ogre::DataStream> &,unsigned long,unsigned long,unsigned long,Ogre::PixelFormat,unsigned long,unsigned long)")]
// was: Ogre::Image::loadRawData(Ogre::SharedPtr<Ogre::DataStream> &,unsigned long,unsigned long,unsigned long,Ogre::PixelFormat,unsigned long,unsigned long)
// IDA 0xcbf4e8: 219 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbf4e8() {
}

// 0xcbf768 — __ZN4Ogre5Image4loadERKSsS2_
#[doc(alias = "Ogre::Image::load(std::string const&,std::string const&)")]
// was: Ogre::Image::load(std::string const&,std::string const&)
// IDA 0xcbf768: 180 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbf768() {
}

// 0xcbfa6c — __ZN4Ogre5Image4loadERNS_9SharedPtrINS_10DataStreamEEERKSs
#[doc(alias = "Ogre::Image::load(Ogre::SharedPtr<Ogre::DataStream> &,std::string const&)")]
// was: Ogre::Image::load(Ogre::SharedPtr<Ogre::DataStream> &,std::string const&)
// IDA 0xcbfa6c: 190 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbfa6c() {
}

// 0xcbfd3c — __ZN4Ogre5Image4saveERKSs
#[doc(alias = "Ogre::Image::save(std::string const&)")]
// was: Ogre::Image::save(std::string const&)
// IDA 0xcbfd3c: 1003 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbfd3c() {
}

// 0xcc081c — __ZN4Ogre5Image19getFileExtFromMagicENS_9SharedPtrINS_10DataStreamEEE
#[doc(alias = "Ogre::Image::getFileExtFromMagic(Ogre::SharedPtr<Ogre::DataStream>)")]
// was: Ogre::Image::getFileExtFromMagic(Ogre::SharedPtr<Ogre::DataStream>)
// IDA 0xcc081c: 50 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc081c() {
}

// 0xcc089c — __ZNK4Ogre5Image7getSizeEv
#[doc(alias = "Ogre::Image::getSize(void)const")]
// was: Ogre::Image::getSize(void)const
// IDA 0xcc089c: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc089c() {
}

// 0xcc08a0 — __ZNK4Ogre5Image13getNumMipmapsEv
#[doc(alias = "Ogre::Image::getNumMipmaps(void)const")]
// was: Ogre::Image::getNumMipmaps(void)const
// IDA 0xcc08a0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc08a0() {
}

// 0xcc08a4 — __ZNK4Ogre5Image7hasFlagENS_10ImageFlagsE
#[doc(alias = "Ogre::Image::hasFlag(Ogre::ImageFlags)const")]
// was: Ogre::Image::hasFlag(Ogre::ImageFlags)const
// IDA 0xcc08a4: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc08a4() {
}

// 0xcc08b0 — __ZNK4Ogre5Image8getDepthEv
#[doc(alias = "Ogre::Image::getDepth(void)const")]
// was: Ogre::Image::getDepth(void)const
// IDA 0xcc08b0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc08b0() {
}

// 0xcc08b4 — __ZNK4Ogre5Image8getWidthEv
#[doc(alias = "Ogre::Image::getWidth(void)const")]
// was: Ogre::Image::getWidth(void)const
// IDA 0xcc08b4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc08b4() {
}

// 0xcc08b8 — __ZNK4Ogre5Image9getHeightEv
#[doc(alias = "Ogre::Image::getHeight(void)const")]
// was: Ogre::Image::getHeight(void)const
// IDA 0xcc08b8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc08b8() {
}

// 0xcc08bc — __ZNK4Ogre5Image11getNumFacesEv
#[doc(alias = "Ogre::Image::getNumFaces(void)const")]
// was: Ogre::Image::getNumFaces(void)const
// IDA 0xcc08bc: 6 insns (LDRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc08bc() {
}

// 0xcc08cc — __ZNK4Ogre5Image9getFormatEv
#[doc(alias = "Ogre::Image::getFormat(void)const")]
// was: Ogre::Image::getFormat(void)const
// IDA 0xcc08cc: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc08cc() {
}

// 0xcc08d0 — __ZN4Ogre5Image10applyGammaEPhfmh
#[doc(alias = "Ogre::Image::applyGamma(unsigned char *,float,unsigned long,unsigned char)")]
// was: Ogre::Image::applyGamma(unsigned char *,float,unsigned long,unsigned char)
// IDA 0xcc08d0: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc08d0() {
}

// 0xcc0978 — __ZN4Ogre5Image6resizeEttNS0_6FilterE
#[doc(alias = "Ogre::Image::resize(unsigned short,unsigned short,Ogre::Image::Filter)")]
// was: Ogre::Image::resize(unsigned short,unsigned short,Ogre::Image::Filter)
// IDA 0xcc0978: 154 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc0978() {
}

// 0xcc0b14 — __ZN4Ogre5Image5scaleERKNS_8PixelBoxES3_NS0_6FilterE
#[doc(alias = "Ogre::Image::scale(Ogre::PixelBox const&,Ogre::PixelBox const&,Ogre::Image::Filter)")]
// was: Ogre::Image::scale(Ogre::PixelBox const&,Ogre::PixelBox const&,Ogre::Image::Filter)
// IDA 0xcc0b14: 411 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc0b14() {
}

// 0xcc0f28 — __ZNK4Ogre5Image11getPixelBoxEmm
#[doc(alias = "Ogre::Image::getPixelBox(unsigned long,unsigned long)const")]
// was: Ogre::Image::getPixelBox(unsigned long,unsigned long)const
// IDA 0xcc0f28: 351 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc0f28() {
}

// 0xcc131c — __ZNK4Ogre5Image11getColourAtEmmm
#[doc(alias = "Ogre::Image::getColourAt(unsigned long,unsigned long,unsigned long)const")]
// was: Ogre::Image::getColourAt(unsigned long,unsigned long,unsigned long)const
// IDA 0xcc131c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc131c() {
}

// 0xcc135c — __ZN4Ogre5Image4swapERS0_
#[doc(alias = "Ogre::Image::swap(Ogre::Image&)")]
// was: Ogre::Image::swap(Ogre::Image&)
// IDA 0xcc135c: 41 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc135c() {
}

// 0xcc13c0 — __ZN4Ogre16NearestResamplerILj1EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<1u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::NearestResampler<1u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc13c0: 157 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc13c0() {
}

// 0xcc1550 — __ZN4Ogre16NearestResamplerILj2EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<2u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::NearestResampler<2u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc1550: 141 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc1550() {
}

// 0xcc16d4 — __ZN4Ogre16NearestResamplerILj3EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<3u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::NearestResampler<3u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc16d4: 160 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc16d4() {
}

// 0xcc1870 — __ZN4Ogre16NearestResamplerILj4EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<4u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::NearestResampler<4u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc1870: 157 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc1870() {
}

// 0xcc1a04 — __ZN4Ogre16NearestResamplerILj6EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<6u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::NearestResampler<6u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc1a04: 160 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc1a04() {
}

// 0xcc1ba8 — __ZN4Ogre16NearestResamplerILj8EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<8u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::NearestResampler<8u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc1ba8: 157 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc1ba8() {
}

// 0xcc1d40 — __ZN4Ogre16NearestResamplerILj12EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<12u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::NearestResampler<12u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc1d40: 148 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc1d40() {
}

// 0xcc1ed8 — __ZN4Ogre16NearestResamplerILj16EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<16u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::NearestResampler<16u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc1ed8: 145 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc1ed8() {
}

// 0xcc2060 — __ZN4Ogre20LinearResampler_ByteILj1EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::LinearResampler_Byte<1u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::LinearResampler_Byte<1u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc2060: 186 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc2060() {
}

// 0xcc2254 — __ZN4Ogre20LinearResampler_ByteILj2EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::LinearResampler_Byte<2u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::LinearResampler_Byte<2u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc2254: 212 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc2254() {
}

// 0xcc24a8 — __ZN4Ogre20LinearResampler_ByteILj3EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::LinearResampler_Byte<3u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::LinearResampler_Byte<3u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc24a8: 222 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc24a8() {
}

// 0xcc2710 — __ZN4Ogre20LinearResampler_ByteILj4EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::LinearResampler_Byte<4u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::LinearResampler_Byte<4u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc2710: 242 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc2710() {
}

// 0xcc29cc — __ZN4Ogre23LinearResampler_Float325scaleERKNS_8PixelBoxES3_
#[doc(alias = "Ogre::LinearResampler_Float32::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::LinearResampler_Float32::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc29cc: 571 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc29cc() {
}

// 0xcc3118 — __ZN4Ogre15LinearResampler5scaleERKNS_8PixelBoxES3_
#[doc(alias = "Ogre::LinearResampler::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
// was: Ogre::LinearResampler::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xcc3118: 496 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc3118() {
}

// 0xcc3740 — __ZN4Ogre17InstancedGeometryC1EPNS_12SceneManagerERKSs
#[doc(alias = "Ogre::InstancedGeometry::InstancedGeometry(Ogre::SceneManager *,std::string const&)")]
// was: Ogre::InstancedGeometry::InstancedGeometry(Ogre::SceneManager *,std::string const&)
// IDA 0xcc3740: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc3740() {
}

// 0xcc374c — __ZN4Ogre17InstancedGeometryC2EPNS_12SceneManagerERKSs
#[doc(alias = "Ogre::InstancedGeometry::InstancedGeometry(Ogre::SceneManager *,std::string const&)")]
// was: Ogre::InstancedGeometry::InstancedGeometry(Ogre::SceneManager *,std::string const&)
// IDA 0xcc374c: 140 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc374c() {
}

// 0xcc38fc — __ZN4Ogre17InstancedGeometryD0Ev
#[doc(alias = "Ogre::InstancedGeometry::~InstancedGeometry()")]
// was: Ogre::InstancedGeometry::~InstancedGeometry()
// IDA 0xcc38fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc38fc() {
}

// 0xcc398c — __ZN4Ogre17InstancedGeometryD1Ev
#[doc(alias = "Ogre::InstancedGeometry::~InstancedGeometry()")]
// was: Ogre::InstancedGeometry::~InstancedGeometry()
// IDA 0xcc398c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc398c() {
}

// 0xcc3998 — __ZN4Ogre17InstancedGeometryD2Ev
#[doc(alias = "Ogre::InstancedGeometry::~InstancedGeometry()")]
// was: Ogre::InstancedGeometry::~InstancedGeometry()
// IDA 0xcc3998: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc3998() {
}

// 0xcc3ca4 — __ZN4Ogre17InstancedGeometry28getInstancedGeometryInstanceEv
#[doc(alias = "Ogre::InstancedGeometry::getInstancedGeometryInstance(void)")]
// was: Ogre::InstancedGeometry::getInstancedGeometryInstance(void)
// IDA 0xcc3ca4: 421 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc3ca4() {
}

// 0xcc417c — __ZN4Ogre17InstancedGeometry16getBatchInstanceERKNS_14AxisAlignedBoxEb
#[doc(alias = "Ogre::InstancedGeometry::getBatchInstance(Ogre::AxisAlignedBox const&,bool)")]
// was: Ogre::InstancedGeometry::getBatchInstance(Ogre::AxisAlignedBox const&,bool)
// IDA 0xcc417c: 112 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc417c() {
}

// 0xcc42a0 — __ZN4Ogre17InstancedGeometry21getVolumeIntersectionERKNS_14AxisAlignedBoxEttt
#[doc(alias = "Ogre::InstancedGeometry::getVolumeIntersection(Ogre::AxisAlignedBox const&,unsigned short,unsigned short,unsigned short)")]
// was: Ogre::InstancedGeometry::getVolumeIntersection(Ogre::AxisAlignedBox const&,unsigned short,unsigned short,unsigned short)
// IDA 0xcc42a0: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc42a0() {
}

// 0xcc43fc — __ZN4Ogre17InstancedGeometry22getBatchInstanceBoundsEttt
#[doc(alias = "Ogre::InstancedGeometry::getBatchInstanceBounds(unsigned short,unsigned short,unsigned short)")]
// was: Ogre::InstancedGeometry::getBatchInstanceBounds(unsigned short,unsigned short,unsigned short)
// IDA 0xcc43fc: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc43fc() {
}

// 0xcc44b4 — __ZN4Ogre17InstancedGeometry22getBatchInstanceCentreEttt
#[doc(alias = "Ogre::InstancedGeometry::getBatchInstanceCentre(unsigned short,unsigned short,unsigned short)")]
// was: Ogre::InstancedGeometry::getBatchInstanceCentre(unsigned short,unsigned short,unsigned short)
// IDA 0xcc44b4: 34 insns (VMOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc44b4() {
}

// 0xcc4538 — __ZN4Ogre17InstancedGeometry16getBatchInstanceEtttb
#[doc(alias = "Ogre::InstancedGeometry::getBatchInstance(unsigned short,unsigned short,unsigned short,bool)")]
// was: Ogre::InstancedGeometry::getBatchInstance(unsigned short,unsigned short,unsigned short,bool)
// IDA 0xcc4538: 448 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc4538() {
}

// 0xcc4a4c — __ZN4Ogre17InstancedGeometry16getBatchInstanceEj
#[doc(alias = "Ogre::InstancedGeometry::getBatchInstance(unsigned int)")]
// was: Ogre::InstancedGeometry::getBatchInstance(unsigned int)
// IDA 0xcc4a4c: 26 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc4a4c() {
}

// 0xcc4a8c — __ZN4Ogre17InstancedGeometry23getBatchInstanceIndexesERKNS_7Vector3ERtS4_S4_
#[doc(alias = "Ogre::InstancedGeometry::getBatchInstanceIndexes(Ogre::Vector3 const&,unsigned short &,unsigned short &,unsigned short &)")]
// was: Ogre::InstancedGeometry::getBatchInstanceIndexes(Ogre::Vector3 const&,unsigned short &,unsigned short &,unsigned short &)
// IDA 0xcc4a8c: 201 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc4a8c() {
}

// 0xcc4d08 — __ZN4Ogre17InstancedGeometry9packIndexEttt
#[doc(alias = "Ogre::InstancedGeometry::packIndex(unsigned short,unsigned short,unsigned short)")]
// was: Ogre::InstancedGeometry::packIndex(unsigned short,unsigned short,unsigned short)
// IDA 0xcc4d08: 3 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc4d08() {
}

// 0xcc4d14 — __ZN4Ogre17InstancedGeometry16getBatchInstanceERKNS_7Vector3Eb
#[doc(alias = "Ogre::InstancedGeometry::getBatchInstance(Ogre::Vector3 const&,bool)")]
// was: Ogre::InstancedGeometry::getBatchInstance(Ogre::Vector3 const&,bool)
// IDA 0xcc4d14: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc4d14() {
}

// 0xcc4d48 — __ZN4Ogre17InstancedGeometry15calculateBoundsEPNS_10VertexDataERKNS_7Vector3ERKNS_10QuaternionES5_
#[doc(alias = "Ogre::InstancedGeometry::calculateBounds(Ogre::VertexData *,Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::Vector3 const&)")]
// was: Ogre::InstancedGeometry::calculateBounds(Ogre::VertexData *,Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::Vector3 const&)
// IDA 0xcc4d48: 308 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc4d48() {
}

// 0xcc508c — __ZN4Ogre17InstancedGeometry9addEntityEPNS_6EntityERKNS_7Vector3ERKNS_10QuaternionES5_
#[doc(alias = "Ogre::InstancedGeometry::addEntity(Ogre::Entity *,Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::Vector3 const&)")]
// was: Ogre::InstancedGeometry::addEntity(Ogre::Entity *,Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::Vector3 const&)
// IDA 0xcc508c: 434 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc508c() {
}

// 0xcc54f8 — __ZN4Ogre17InstancedGeometry17determineGeometryEPNS_7SubMeshE
#[doc(alias = "Ogre::InstancedGeometry::determineGeometry(Ogre::SubMesh *)")]
// was: Ogre::InstancedGeometry::determineGeometry(Ogre::SubMesh *)
// IDA 0xcc54f8: 151 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc54f8() {
}

// 0xcc5678 — __ZN4Ogre17InstancedGeometry13splitGeometryEPNS_10VertexDataEPNS_9IndexDataEPNS0_22SubMeshLodGeometryLinkE
#[doc(alias = "Ogre::InstancedGeometry::splitGeometry(Ogre::VertexData *,Ogre::IndexData *,Ogre::InstancedGeometry::SubMeshLodGeometryLink *)")]
// was: Ogre::InstancedGeometry::splitGeometry(Ogre::VertexData *,Ogre::IndexData *,Ogre::InstancedGeometry::SubMeshLodGeometryLink *)
// IDA 0xcc5678: 832 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc5678() {
}

// 0xcc5e4c — __ZN4Ogre17InstancedGeometry12addSceneNodeEPKNS_9SceneNodeE
#[doc(alias = "Ogre::InstancedGeometry::addSceneNode(Ogre::SceneNode const*)")]
// was: Ogre::InstancedGeometry::addSceneNode(Ogre::SceneNode const*)
// IDA 0xcc5e4c: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc5e4c() {
}

// 0xcc5f18 — __ZN4Ogre17InstancedGeometry5buildEv
#[doc(alias = "Ogre::InstancedGeometry::build(void)")]
// was: Ogre::InstancedGeometry::build(void)
// IDA 0xcc5f18: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc5f18() {
}

// 0xcc5f64 — __ZN4Ogre17InstancedGeometry13BatchInstance6assignEPNS0_13QueuedSubMeshE
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::assign(Ogre::InstancedGeometry::QueuedSubMesh *)")]
// was: Ogre::InstancedGeometry::BatchInstance::assign(Ogre::InstancedGeometry::QueuedSubMesh *)
// IDA 0xcc5f64: 347 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc5f64() {
}

// 0xcc638c — __ZN4Ogre17InstancedGeometry13BatchInstance5buildEv
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::build(void)")]
// was: Ogre::InstancedGeometry::BatchInstance::build(void)
// IDA 0xcc638c: 126 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc638c() {
}

// 0xcc6504 — __ZN4Ogre17InstancedGeometry13BatchInstance24isInstancedObjectPresentEt
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::isInstancedObjectPresent(unsigned short)")]
// was: Ogre::InstancedGeometry::BatchInstance::isInstancedObjectPresent(unsigned short)
// IDA 0xcc6504: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc6504() {
}

// 0xcc658c — __ZN4Ogre17InstancedGeometry7destroyEv
#[doc(alias = "Ogre::InstancedGeometry::destroy(void)")]
// was: Ogre::InstancedGeometry::destroy(void)
// IDA 0xcc658c: 108 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc658c() {
}
