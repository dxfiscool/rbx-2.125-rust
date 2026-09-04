//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xe47714..0xe4d1f0 (100 stubs, 9760 prior -> 9860 covered, 3473 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xe47714 — __ZN4Ogre7Texture8setDepthEm
#[doc(alias = "Ogre::Texture::setDepth(unsigned long)")]
// was: Ogre::Texture::setDepth(unsigned long)
// IDA 0xe47714: 3 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e47714() {
}

// 0xe4771c — __ZNK4Ogre7Texture8getUsageEv
#[doc(alias = "Ogre::Texture::getUsage(void)const")]
// was: Ogre::Texture::getUsage(void)const
// IDA 0xe4771c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4771c() {
}

// 0xe47724 — __ZN4Ogre7Texture8setUsageEi
#[doc(alias = "Ogre::Texture::setUsage(int)")]
// was: Ogre::Texture::setUsage(int)
// IDA 0xe47724: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e47724() {
}

// 0xe4772c — __ZNK4Ogre7Texture9getFormatEv
#[doc(alias = "Ogre::Texture::getFormat(void)const")]
// was: Ogre::Texture::getFormat(void)const
// IDA 0xe4772c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4772c() {
}

// 0xe47734 — __ZNK4Ogre7Texture16getDesiredFormatEv
#[doc(alias = "Ogre::Texture::getDesiredFormat(void)const")]
// was: Ogre::Texture::getDesiredFormat(void)const
// IDA 0xe47734: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e47734() {
}

// 0xe4773c — __ZNK4Ogre7Texture12getSrcFormatEv
#[doc(alias = "Ogre::Texture::getSrcFormat(void)const")]
// was: Ogre::Texture::getSrcFormat(void)const
// IDA 0xe4773c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4773c() {
}

// 0xe47744 — __ZN4Ogre7Texture18getCustomAttributeERKSsPv
#[doc(alias = "Ogre::Texture::getCustomAttribute(std::string const&,void *)")]
// was: Ogre::Texture::getCustomAttribute(std::string const&,void *)
// IDA 0xe47744: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e47744() {
}

// 0xe4777c — __ZN4Ogre14TextureManager15getSingletonPtrEv
#[doc(alias = "Ogre::TextureManager::getSingletonPtr(void)")]
// was: Ogre::TextureManager::getSingletonPtr(void)
// IDA 0xe4777c: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4777c() {
}

// 0xe4778c — __ZN4Ogre14TextureManager12getSingletonEv
#[doc(alias = "Ogre::TextureManager::getSingleton(void)")]
// was: Ogre::TextureManager::getSingleton(void)
// IDA 0xe4778c: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4778c() {
}

// 0xe4779c — __ZN4Ogre14TextureManagerC2Ev
#[doc(alias = "Ogre::TextureManager::TextureManager(void)")]
// was: Ogre::TextureManager::TextureManager(void)
// IDA 0xe4779c: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4779c() {
}

// 0xe4789c — __ZN4Ogre14TextureManagerD0Ev
#[doc(alias = "Ogre::TextureManager::~TextureManager()")]
// was: Ogre::TextureManager::~TextureManager()
// IDA 0xe4789c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e4789c() {
}

// 0xe4793c — __ZN4Ogre14TextureManagerD1Ev
#[doc(alias = "Ogre::TextureManager::~TextureManager()")]
// was: Ogre::TextureManager::~TextureManager()
// IDA 0xe4793c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e4793c() {
}

// 0xe47954 — __ZN4Ogre14TextureManagerD2Ev
#[doc(alias = "Ogre::TextureManager::~TextureManager()")]
// was: Ogre::TextureManager::~TextureManager()
// IDA 0xe47954: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e47954() {
}

// 0xe4796c — __ZN4Ogre14TextureManager16createOrRetrieveERKSsS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEENS_11TextureTypeEifbNS_11PixelFormatEb
#[doc(alias = "Ogre::TextureManager::createOrRetrieve(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,Ogre::TextureType,int,float,bool,Ogre::PixelFormat,bool)")]
// was: Ogre::TextureManager::createOrRetrieve(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,Ogre::TextureType,int,float,bool,Ogre::PixelFormat,bool)
// IDA 0xe4796c: 220 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4796c() {
}

// 0xe47b8c — __ZN4Ogre14TextureManager7prepareERKSsS2_NS_11TextureTypeEifbNS_11PixelFormatEb
#[doc(alias = "Ogre::TextureManager::prepare(std::string const&,std::string const&,Ogre::TextureType,int,float,bool,Ogre::PixelFormat,bool)")]
// was: Ogre::TextureManager::prepare(std::string const&,std::string const&,Ogre::TextureType,int,float,bool,Ogre::PixelFormat,bool)
// IDA 0xe47b8c: 214 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e47b8c() {
}

// 0xe47da8 — __ZN4Ogre14TextureManager4loadERKSsS2_NS_11TextureTypeEifbNS_11PixelFormatEb
#[doc(alias = "Ogre::TextureManager::load(std::string const&,std::string const&,Ogre::TextureType,int,float,bool,Ogre::PixelFormat,bool)")]
// was: Ogre::TextureManager::load(std::string const&,std::string const&,Ogre::TextureType,int,float,bool,Ogre::PixelFormat,bool)
// IDA 0xe47da8: 214 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e47da8() {
}

// 0xe47fc4 — __ZN4Ogre14TextureManager9loadImageERKSsS2_RKNS_5ImageENS_11TextureTypeEifbNS_11PixelFormatEb
#[doc(alias = "Ogre::TextureManager::loadImage(std::string const&,std::string const&,Ogre::Image const&,Ogre::TextureType,int,float,bool,Ogre::PixelFormat,bool)")]
// was: Ogre::TextureManager::loadImage(std::string const&,std::string const&,Ogre::Image const&,Ogre::TextureType,int,float,bool,Ogre::PixelFormat,bool)
// IDA 0xe47fc4: 222 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e47fc4() {
}

// 0xe481f8 — __ZN4Ogre14TextureManager11loadRawDataERKSsS2_RNS_9SharedPtrINS_10DataStreamEEEttNS_11PixelFormatENS_11TextureTypeEifb
#[doc(alias = "Ogre::TextureManager::loadRawData(std::string const&,std::string const&,Ogre::SharedPtr<Ogre::DataStream> &,unsigned short,unsigned short,Ogre::PixelFormat,Ogre::TextureType,int,float,bool)")]
// was: Ogre::TextureManager::loadRawData(std::string const&,std::string const&,Ogre::SharedPtr<Ogre::DataStream> &,unsigned short,unsigned short,Ogre::PixelFormat,Ogre::TextureType,int,float,bool)
// IDA 0xe481f8: 209 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e481f8() {
}

// 0xe4840c — __ZN4Ogre14TextureManager12createManualERKSsS2_NS_11TextureTypeEjjjiNS_11PixelFormatEiPNS_20ManualResourceLoaderEbjS2_
#[doc(alias = "Ogre::TextureManager::createManual(std::string const&,std::string const&,Ogre::TextureType,unsigned int,unsigned int,unsigned int,int,Ogre::PixelFormat,int,Ogre::ManualResourceLoader *,bool,unsigned int,std::string const&)")]
// was: Ogre::TextureManager::createManual(std::string const&,std::string const&,Ogre::TextureType,unsigned int,unsigned int,unsigned int,int,Ogre::PixelFormat,int,Ogre::ManualResourceLoader *,bool,unsigned int,std::string const&)
// IDA 0xe4840c: 246 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4840c() {
}

// 0xe48674 — __ZN4Ogre14TextureManager27setPreferredIntegerBitDepthEtb
#[doc(alias = "Ogre::TextureManager::setPreferredIntegerBitDepth(unsigned short,bool)")]
// was: Ogre::TextureManager::setPreferredIntegerBitDepth(unsigned short,bool)
// IDA 0xe48674: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e48674() {
}

// 0xe486e8 — __ZNK4Ogre14TextureManager27getPreferredIntegerBitDepthEv
#[doc(alias = "Ogre::TextureManager::getPreferredIntegerBitDepth(void)const")]
// was: Ogre::TextureManager::getPreferredIntegerBitDepth(void)const
// IDA 0xe486e8: 2 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e486e8() {
}

// 0xe486f0 — __ZN4Ogre14TextureManager25setPreferredFloatBitDepthEtb
#[doc(alias = "Ogre::TextureManager::setPreferredFloatBitDepth(unsigned short,bool)")]
// was: Ogre::TextureManager::setPreferredFloatBitDepth(unsigned short,bool)
// IDA 0xe486f0: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e486f0() {
}

// 0xe48764 — __ZNK4Ogre14TextureManager25getPreferredFloatBitDepthEv
#[doc(alias = "Ogre::TextureManager::getPreferredFloatBitDepth(void)const")]
// was: Ogre::TextureManager::getPreferredFloatBitDepth(void)const
// IDA 0xe48764: 2 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e48764() {
}

// 0xe4876c — __ZN4Ogre14TextureManager21setPreferredBitDepthsEttb
#[doc(alias = "Ogre::TextureManager::setPreferredBitDepths(unsigned short,unsigned short,bool)")]
// was: Ogre::TextureManager::setPreferredBitDepths(unsigned short,unsigned short,bool)
// IDA 0xe4876c: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4876c() {
}

// 0xe487f0 — __ZN4Ogre14TextureManager20setDefaultNumMipmapsEm
#[doc(alias = "Ogre::TextureManager::setDefaultNumMipmaps(unsigned long)")]
// was: Ogre::TextureManager::setDefaultNumMipmaps(unsigned long)
// IDA 0xe487f0: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e487f0() {
}

// 0xe487f8 — __ZN4Ogre14TextureManager17isFormatSupportedENS_11TextureTypeENS_11PixelFormatEi
#[doc(alias = "Ogre::TextureManager::isFormatSupported(Ogre::TextureType,Ogre::PixelFormat,int)")]
// was: Ogre::TextureManager::isFormatSupported(Ogre::TextureType,Ogre::PixelFormat,int)
// IDA 0xe487f8: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e487f8() {
}

// 0xe48814 — __ZN4Ogre14TextureManager27isEquivalentFormatSupportedENS_11TextureTypeENS_11PixelFormatEi
#[doc(alias = "Ogre::TextureManager::isEquivalentFormatSupported(Ogre::TextureType,Ogre::PixelFormat,int)")]
// was: Ogre::TextureManager::isEquivalentFormatSupported(Ogre::TextureType,Ogre::PixelFormat,int)
// IDA 0xe48814: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e48814() {
}

// 0xe4883c — __ZN4Ogre14TextureManager20getDefaultNumMipmapsEv
#[doc(alias = "Ogre::TextureManager::getDefaultNumMipmaps(void)")]
// was: Ogre::TextureManager::getDefaultNumMipmaps(void)
// IDA 0xe4883c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4883c() {
}

// 0xe48878 — __ZN4Ogre16TextureUnitStateC1EPNS_4PassE
#[doc(alias = "Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *)")]
// was: Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *)
// IDA 0xe48878: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e48878() {
}

// 0xe48884 — __ZN4Ogre16TextureUnitStateC2EPNS_4PassE
#[doc(alias = "Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *)")]
// was: Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *)
// IDA 0xe48884: 425 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e48884() {
}

// 0xe48d44 — __ZN4Ogre16TextureUnitState18setColourOperationENS_19LayerBlendOperationE
#[doc(alias = "Ogre::TextureUnitState::setColourOperation(Ogre::LayerBlendOperation)")]
// was: Ogre::TextureUnitState::setColourOperation(Ogre::LayerBlendOperation)
// IDA 0xe48d44: 83 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e48d44() {
}

// 0xe48e4c — __ZN4Ogre16TextureUnitState24setTextureAddressingModeENS0_21TextureAddressingModeE
#[doc(alias = "Ogre::TextureUnitState::setTextureAddressingMode(Ogre::TextureUnitState::TextureAddressingMode)")]
// was: Ogre::TextureUnitState::setTextureAddressingMode(Ogre::TextureUnitState::TextureAddressingMode)
// IDA 0xe48e4c: 4 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e48e4c() {
}

// 0xe48e54 — __ZN4Ogre16TextureUnitStateC1EPNS_4PassERKS0_
#[doc(alias = "Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *,Ogre::TextureUnitState const&)")]
// was: Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *,Ogre::TextureUnitState const&)
// IDA 0xe48e54: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e48e54() {
}

// 0xe48e60 — __ZN4Ogre16TextureUnitStateC2EPNS_4PassERKS0_
#[doc(alias = "Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *,Ogre::TextureUnitState const&)")]
// was: Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *,Ogre::TextureUnitState const&)
// IDA 0xe48e60: 297 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e48e60() {
}

// 0xe491a8 — __ZN4Ogre16TextureUnitStateaSERKS0_
#[doc(alias = "Ogre::TextureUnitState::operator=(Ogre::TextureUnitState const&)")]
// was: Ogre::TextureUnitState::operator=(Ogre::TextureUnitState const&)
// IDA 0xe491a8: 87 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e491a8() {
}

// 0xe492bc — __ZN4Ogre16TextureUnitState14setTextureNameERKSsNS_11TextureTypeE
#[doc(alias = "Ogre::TextureUnitState::setTextureName(std::string const&,Ogre::TextureType)")]
// was: Ogre::TextureUnitState::setTextureName(std::string const&,Ogre::TextureType)
// IDA 0xe492bc: 344 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e492bc() {
}

// 0xe4964c — __ZN4Ogre16TextureUnitState18setTextureCoordSetEj
#[doc(alias = "Ogre::TextureUnitState::setTextureCoordSet(unsigned int)")]
// was: Ogre::TextureUnitState::setTextureCoordSet(unsigned int)
// IDA 0xe4964c: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4964c() {
}

// 0xe49650 — __ZN4Ogre16TextureUnitStateD1Ev
#[doc(alias = "Ogre::TextureUnitState::~TextureUnitState()")]
// was: Ogre::TextureUnitState::~TextureUnitState()
// IDA 0xe49650: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e49650() {
}

// 0xe4965c — __ZN4Ogre16TextureUnitStateD2Ev
#[doc(alias = "Ogre::TextureUnitState::~TextureUnitState()")]
// was: Ogre::TextureUnitState::~TextureUnitState()
// IDA 0xe4965c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e4965c() {
}

// 0xe49a3c — __ZN4Ogre16TextureUnitState7_unloadEv
#[doc(alias = "Ogre::TextureUnitState::_unload(void)")]
// was: Ogre::TextureUnitState::_unload(void)
// IDA 0xe49a3c: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e49a3c() {
}

// 0xe49ac4 — __ZNK4Ogre16TextureUnitState8isLoadedEv
#[doc(alias = "Ogre::TextureUnitState::isLoaded(void)const")]
// was: Ogre::TextureUnitState::isLoaded(void)const
// IDA 0xe49ac4: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e49ac4() {
}

// 0xe49ad4 — __ZN4Ogre16TextureUnitState5_loadEv
#[doc(alias = "Ogre::TextureUnitState::_load(void)")]
// was: Ogre::TextureUnitState::_load(void)
// IDA 0xe49ad4: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e49ad4() {
}

// 0xe49b54 — __ZNK4Ogre16TextureUnitState14getTextureNameEv
#[doc(alias = "Ogre::TextureUnitState::getTextureName(void)const")]
// was: Ogre::TextureUnitState::getTextureName(void)const
// IDA 0xe49b54: 12 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e49b54() {
}

// 0xe49b7c — __ZN4Ogre16TextureUnitState14setContentTypeENS0_11ContentTypeE
#[doc(alias = "Ogre::TextureUnitState::setContentType(Ogre::TextureUnitState::ContentType)")]
// was: Ogre::TextureUnitState::setContentType(Ogre::TextureUnitState::ContentType)
// IDA 0xe49b7c: 236 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e49b7c() {
}

// 0xe49dec — __ZN4Ogre16TextureUnitState19setCubicTextureNameERKSsb
#[doc(alias = "Ogre::TextureUnitState::setCubicTextureName(std::string const&,bool)")]
// was: Ogre::TextureUnitState::setCubicTextureName(std::string const&,bool)
// IDA 0xe49dec: 570 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e49dec() {
}

// 0xe4a90c — __ZN4Ogre16TextureUnitState14setBindingTypeENS0_11BindingTypeE
#[doc(alias = "Ogre::TextureUnitState::setBindingType(Ogre::TextureUnitState::BindingType)")]
// was: Ogre::TextureUnitState::setBindingType(Ogre::TextureUnitState::BindingType)
// IDA 0xe4a90c: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4a90c() {
}

// 0xe4a914 — __ZNK4Ogre16TextureUnitState14getBindingTypeEv
#[doc(alias = "Ogre::TextureUnitState::getBindingType(void)const")]
// was: Ogre::TextureUnitState::getBindingType(void)const
// IDA 0xe4a914: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4a914() {
}

// 0xe4a91c — __ZNK4Ogre16TextureUnitState14getContentTypeEv
#[doc(alias = "Ogre::TextureUnitState::getContentType(void)const")]
// was: Ogre::TextureUnitState::getContentType(void)const
// IDA 0xe4a91c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4a91c() {
}

// 0xe4a924 — __ZN4Ogre16TextureUnitState19setCubicTextureNameEPKSsb
#[doc(alias = "Ogre::TextureUnitState::setCubicTextureName(std::string const*,bool)")]
// was: Ogre::TextureUnitState::setCubicTextureName(std::string const*,bool)
// IDA 0xe4a924: 346 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4a924() {
}

// 0xe4aca8 — __ZNK4Ogre16TextureUnitState4is3DEv
#[doc(alias = "Ogre::TextureUnitState::is3D(void)const")]
// was: Ogre::TextureUnitState::is3D(void)const
// IDA 0xe4aca8: 6 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4aca8() {
}

// 0xe4acb4 — __ZNK4Ogre16TextureUnitState14getTextureTypeEv
#[doc(alias = "Ogre::TextureUnitState::getTextureType(void)const")]
// was: Ogre::TextureUnitState::getTextureType(void)const
// IDA 0xe4acb4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4acb4() {
}

// 0xe4acb8 — __ZN4Ogre16TextureUnitState22setAnimatedTextureNameERKSsjf
#[doc(alias = "Ogre::TextureUnitState::setAnimatedTextureName(std::string const&,unsigned int,float)")]
// was: Ogre::TextureUnitState::setAnimatedTextureName(std::string const&,unsigned int,float)
// IDA 0xe4acb8: 542 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4acb8() {
}

// 0xe4b538 — __ZN4Ogre16TextureUnitState22setAnimatedTextureNameEPKSsjf
#[doc(alias = "Ogre::TextureUnitState::setAnimatedTextureName(std::string const*,unsigned int,float)")]
// was: Ogre::TextureUnitState::setAnimatedTextureName(std::string const*,unsigned int,float)
// IDA 0xe4b538: 355 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4b538() {
}

// 0xe4b8f0 — __ZNK4Ogre16TextureUnitState14_getTexturePtrEm
#[doc(alias = "Ogre::TextureUnitState::_getTexturePtr(unsigned long)const")]
// was: Ogre::TextureUnitState::_getTexturePtr(unsigned long)const
// IDA 0xe4b8f0: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4b8f0() {
}

// 0xe4b98c — __ZN4Ogre16TextureUnitState15setCurrentFrameEj
#[doc(alias = "Ogre::TextureUnitState::setCurrentFrame(unsigned int)")]
// was: Ogre::TextureUnitState::setCurrentFrame(unsigned int)
// IDA 0xe4b98c: 173 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4b98c() {
}

// 0xe4bb98 — __ZNK4Ogre16TextureUnitState15getCurrentFrameEv
#[doc(alias = "Ogre::TextureUnitState::getCurrentFrame(void)const")]
// was: Ogre::TextureUnitState::getCurrentFrame(void)const
// IDA 0xe4bb98: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bb98() {
}

// 0xe4bb9c — __ZNK4Ogre16TextureUnitState12getNumFramesEv
#[doc(alias = "Ogre::TextureUnitState::getNumFrames(void)const")]
// was: Ogre::TextureUnitState::getNumFrames(void)const
// IDA 0xe4bb9c: 4 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bb9c() {
}

// 0xe4bba8 — __ZN4Ogre16TextureUnitState16setDesiredFormatENS_11PixelFormatE
#[doc(alias = "Ogre::TextureUnitState::setDesiredFormat(Ogre::PixelFormat)")]
// was: Ogre::TextureUnitState::setDesiredFormat(Ogre::PixelFormat)
// IDA 0xe4bba8: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bba8() {
}

// 0xe4bbac — __ZN4Ogre16TextureUnitState13setNumMipmapsEi
#[doc(alias = "Ogre::TextureUnitState::setNumMipmaps(int)")]
// was: Ogre::TextureUnitState::setNumMipmaps(int)
// IDA 0xe4bbac: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bbac() {
}

// 0xe4bbb0 — __ZN4Ogre16TextureUnitState10setIsAlphaEb
#[doc(alias = "Ogre::TextureUnitState::setIsAlpha(bool)")]
// was: Ogre::TextureUnitState::setIsAlpha(bool)
// IDA 0xe4bbb0: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bbb0() {
}

// 0xe4bbb8 — __ZN4Ogre16TextureUnitState23setHardwareGammaEnabledEb
#[doc(alias = "Ogre::TextureUnitState::setHardwareGammaEnabled(bool)")]
// was: Ogre::TextureUnitState::setHardwareGammaEnabled(bool)
// IDA 0xe4bbb8: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bbb8() {
}

// 0xe4bbc0 — __ZNK4Ogre16TextureUnitState18getTextureCoordSetEv
#[doc(alias = "Ogre::TextureUnitState::getTextureCoordSet(void)const")]
// was: Ogre::TextureUnitState::getTextureCoordSet(void)const
// IDA 0xe4bbc0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bbc0() {
}

// 0xe4bbc4 — __ZN4Ogre16TextureUnitState20setColourOperationExENS_21LayerBlendOperationExENS_16LayerBlendSourceES2_RKNS_11ColourValueES5_f
#[doc(alias = "Ogre::TextureUnitState::setColourOperationEx(Ogre::LayerBlendOperationEx,Ogre::LayerBlendSource,Ogre::LayerBlendSource,Ogre::ColourValue const&,Ogre::ColourValue const&,float)")]
// was: Ogre::TextureUnitState::setColourOperationEx(Ogre::LayerBlendOperationEx,Ogre::LayerBlendSource,Ogre::LayerBlendSource,Ogre::ColourValue const&,Ogre::ColourValue const&,float)
// IDA 0xe4bbc4: 15 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bbc4() {
}

// 0xe4bbf8 — __ZN4Ogre16TextureUnitState28setColourOpMultipassFallbackENS_16SceneBlendFactorES1_
#[doc(alias = "Ogre::TextureUnitState::setColourOpMultipassFallback(Ogre::SceneBlendFactor,Ogre::SceneBlendFactor)")]
// was: Ogre::TextureUnitState::setColourOpMultipassFallback(Ogre::SceneBlendFactor,Ogre::SceneBlendFactor)
// IDA 0xe4bbf8: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bbf8() {
}

// 0xe4bc04 — __ZN4Ogre16TextureUnitState17setAlphaOperationENS_21LayerBlendOperationExENS_16LayerBlendSourceES2_fff
#[doc(alias = "Ogre::TextureUnitState::setAlphaOperation(Ogre::LayerBlendOperationEx,Ogre::LayerBlendSource,Ogre::LayerBlendSource,float,float,float)")]
// was: Ogre::TextureUnitState::setAlphaOperation(Ogre::LayerBlendOperationEx,Ogre::LayerBlendSource,Ogre::LayerBlendSource,float,float,float)
// IDA 0xe4bc04: 10 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bc04() {
}

// 0xe4bc2c — __ZN4Ogre16TextureUnitState9addEffectERNS0_13TextureEffectE
#[doc(alias = "Ogre::TextureUnitState::addEffect(Ogre::TextureUnitState::TextureEffect &)")]
// was: Ogre::TextureUnitState::addEffect(Ogre::TextureUnitState::TextureEffect &)
// IDA 0xe4bc2c: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bc2c() {
}

// 0xe4bd68 — __ZN4Ogre16TextureUnitState22createEffectControllerERNS0_13TextureEffectE
#[doc(alias = "Ogre::TextureUnitState::createEffectController(Ogre::TextureUnitState::TextureEffect &)")]
// was: Ogre::TextureUnitState::createEffectController(Ogre::TextureUnitState::TextureEffect &)
// IDA 0xe4bd68: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bd68() {
}

// 0xe4bdf8 — __ZNK4Ogre16TextureUnitState25getColourBlendFallbackSrcEv
#[doc(alias = "Ogre::TextureUnitState::getColourBlendFallbackSrc(void)const")]
// was: Ogre::TextureUnitState::getColourBlendFallbackSrc(void)const
// IDA 0xe4bdf8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bdf8() {
}

// 0xe4bdfc — __ZNK4Ogre16TextureUnitState26getColourBlendFallbackDestEv
#[doc(alias = "Ogre::TextureUnitState::getColourBlendFallbackDest(void)const")]
// was: Ogre::TextureUnitState::getColourBlendFallbackDest(void)const
// IDA 0xe4bdfc: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bdfc() {
}

// 0xe4be00 — __ZNK4Ogre16TextureUnitState18getColourBlendModeEv
#[doc(alias = "Ogre::TextureUnitState::getColourBlendMode(void)const")]
// was: Ogre::TextureUnitState::getColourBlendMode(void)const
// IDA 0xe4be00: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4be00() {
}

// 0xe4be04 — __ZNK4Ogre16TextureUnitState17getAlphaBlendModeEv
#[doc(alias = "Ogre::TextureUnitState::getAlphaBlendMode(void)const")]
// was: Ogre::TextureUnitState::getAlphaBlendMode(void)const
// IDA 0xe4be04: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4be04() {
}

// 0xe4be08 — __ZNK4Ogre16TextureUnitState24getTextureAddressingModeEv
#[doc(alias = "Ogre::TextureUnitState::getTextureAddressingMode(void)const")]
// was: Ogre::TextureUnitState::getTextureAddressingMode(void)const
// IDA 0xe4be08: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4be08() {
}

// 0xe4be0c — __ZN4Ogre16TextureUnitState24setTextureAddressingModeERKNS0_17UVWAddressingModeE
#[doc(alias = "Ogre::TextureUnitState::setTextureAddressingMode(Ogre::TextureUnitState::UVWAddressingMode const&)")]
// was: Ogre::TextureUnitState::setTextureAddressingMode(Ogre::TextureUnitState::UVWAddressingMode const&)
// IDA 0xe4be0c: 5 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4be0c() {
}

// 0xe4be1c — __ZN4Ogre16TextureUnitState22setTextureBorderColourERKNS_11ColourValueE
#[doc(alias = "Ogre::TextureUnitState::setTextureBorderColour(Ogre::ColourValue const&)")]
// was: Ogre::TextureUnitState::setTextureBorderColour(Ogre::ColourValue const&)
// IDA 0xe4be1c: 4 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4be1c() {
}

// 0xe4be28 — __ZNK4Ogre16TextureUnitState22getTextureBorderColourEv
#[doc(alias = "Ogre::TextureUnitState::getTextureBorderColour(void)const")]
// was: Ogre::TextureUnitState::getTextureBorderColour(void)const
// IDA 0xe4be28: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4be28() {
}

// 0xe4be2c — __ZN4Ogre16TextureUnitState17setEnvironmentMapEbNS0_10EnvMapTypeE
#[doc(alias = "Ogre::TextureUnitState::setEnvironmentMap(bool,Ogre::TextureUnitState::EnvMapType)")]
// was: Ogre::TextureUnitState::setEnvironmentMap(bool,Ogre::TextureUnitState::EnvMapType)
// IDA 0xe4be2c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4be2c() {
}

// 0xe4be50 — __ZN4Ogre16TextureUnitState12removeEffectENS0_17TextureEffectTypeE
#[doc(alias = "Ogre::TextureUnitState::removeEffect(Ogre::TextureUnitState::TextureEffectType)")]
// was: Ogre::TextureUnitState::removeEffect(Ogre::TextureUnitState::TextureEffectType)
// IDA 0xe4be50: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4be50() {
}

// 0xe4bec8 — __ZN4Ogre16TextureUnitState19setTextureTransformERKNS_7Matrix4E
#[doc(alias = "Ogre::TextureUnitState::setTextureTransform(Ogre::Matrix4 const&)")]
// was: Ogre::TextureUnitState::setTextureTransform(Ogre::Matrix4 const&)
// IDA 0xe4bec8: 18 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bec8() {
}

// 0xe4bf0c — __ZN4Ogre16TextureUnitState16setTextureScrollEff
#[doc(alias = "Ogre::TextureUnitState::setTextureScroll(float,float)")]
// was: Ogre::TextureUnitState::setTextureScroll(float,float)
// IDA 0xe4bf0c: 6 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bf0c() {
}

// 0xe4bf1c — __ZN4Ogre16TextureUnitState15setTextureScaleEff
#[doc(alias = "Ogre::TextureUnitState::setTextureScale(float,float)")]
// was: Ogre::TextureUnitState::setTextureScale(float,float)
// IDA 0xe4bf1c: 6 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bf1c() {
}

// 0xe4bf2c — __ZN4Ogre16TextureUnitState16setTextureRotateERKNS_6RadianE
#[doc(alias = "Ogre::TextureUnitState::setTextureRotate(Ogre::Radian const&)")]
// was: Ogre::TextureUnitState::setTextureRotate(Ogre::Radian const&)
// IDA 0xe4bf2c: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bf2c() {
}

// 0xe4bf3c — __ZNK4Ogre16TextureUnitState19getTextureTransformEv
#[doc(alias = "Ogre::TextureUnitState::getTextureTransform(void)const")]
// was: Ogre::TextureUnitState::getTextureTransform(void)const
// IDA 0xe4bf3c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bf3c() {
}

// 0xe4bf58 — __ZNK4Ogre16TextureUnitState19recalcTextureMatrixEv
#[doc(alias = "Ogre::TextureUnitState::recalcTextureMatrix(void)const")]
// was: Ogre::TextureUnitState::recalcTextureMatrix(void)const
// IDA 0xe4bf58: 183 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4bf58() {
}

// 0xe4c1dc — __ZN4Ogre16TextureUnitState17setTextureUScrollEf
#[doc(alias = "Ogre::TextureUnitState::setTextureUScroll(float)")]
// was: Ogre::TextureUnitState::setTextureUScroll(float)
// IDA 0xe4c1dc: 4 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4c1dc() {
}

// 0xe4c1e8 — __ZN4Ogre16TextureUnitState17setTextureVScrollEf
#[doc(alias = "Ogre::TextureUnitState::setTextureVScroll(float)")]
// was: Ogre::TextureUnitState::setTextureVScroll(float)
// IDA 0xe4c1e8: 4 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4c1e8() {
}

// 0xe4c1f4 — __ZN4Ogre16TextureUnitState16setTextureUScaleEf
#[doc(alias = "Ogre::TextureUnitState::setTextureUScale(float)")]
// was: Ogre::TextureUnitState::setTextureUScale(float)
// IDA 0xe4c1f4: 4 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4c1f4() {
}

// 0xe4c200 — __ZN4Ogre16TextureUnitState16setTextureVScaleEf
#[doc(alias = "Ogre::TextureUnitState::setTextureVScale(float)")]
// was: Ogre::TextureUnitState::setTextureVScale(float)
// IDA 0xe4c200: 4 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4c200() {
}

// 0xe4c20c — __ZN4Ogre16TextureUnitState18setScrollAnimationEff
#[doc(alias = "Ogre::TextureUnitState::setScrollAnimation(float,float)")]
// was: Ogre::TextureUnitState::setScrollAnimation(float,float)
// IDA 0xe4c20c: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4c20c() {
}

// 0xe4c2a0 — __ZN4Ogre16TextureUnitState18setRotateAnimationEf
#[doc(alias = "Ogre::TextureUnitState::setRotateAnimation(float)")]
// was: Ogre::TextureUnitState::setRotateAnimation(float)
// IDA 0xe4c2a0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4c2a0() {
}

// 0xe4c2d4 — __ZN4Ogre16TextureUnitState21setTransformAnimationENS0_20TextureTransformTypeENS_12WaveformTypeEffff
#[doc(alias = "Ogre::TextureUnitState::setTransformAnimation(Ogre::TextureUnitState::TextureTransformType,Ogre::WaveformType,float,float,float,float)")]
// was: Ogre::TextureUnitState::setTransformAnimation(Ogre::TextureUnitState::TextureTransformType,Ogre::WaveformType,float,float,float,float)
// IDA 0xe4c2d4: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4c2d4() {
}

// 0xe4c390 — __ZN4Ogre16TextureUnitState8_prepareEv
#[doc(alias = "Ogre::TextureUnitState::_prepare(void)")]
// was: Ogre::TextureUnitState::_prepare(void)
// IDA 0xe4c390: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4c390() {
}

// 0xe4c3bc — __ZNK4Ogre16TextureUnitState14ensurePreparedEm
#[doc(alias = "Ogre::TextureUnitState::ensurePrepared(unsigned long)const")]
// was: Ogre::TextureUnitState::ensurePrepared(unsigned long)const
// IDA 0xe4c3bc: 609 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4c3bc() {
}

// 0xe4ca5c — __ZNK4Ogre16TextureUnitState12ensureLoadedEm
#[doc(alias = "Ogre::TextureUnitState::ensureLoaded(unsigned long)const")]
// was: Ogre::TextureUnitState::ensureLoaded(unsigned long)const
// IDA 0xe4ca5c: 609 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4ca5c() {
}

// 0xe4d0fc — __ZNK4Ogre16TextureUnitState14_getTexturePtrEv
#[doc(alias = "Ogre::TextureUnitState::_getTexturePtr(void)const")]
// was: Ogre::TextureUnitState::_getTexturePtr(void)const
// IDA 0xe4d0fc: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4d0fc() {
}

// 0xe4d108 — __ZN4Ogre16TextureUnitState14_setTexturePtrERKNS_10TexturePtrE
#[doc(alias = "Ogre::TextureUnitState::_setTexturePtr(Ogre::TexturePtr const&)")]
// was: Ogre::TextureUnitState::_setTexturePtr(Ogre::TexturePtr const&)
// IDA 0xe4d108: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4d108() {
}

// 0xe4d11c — __ZNK4Ogre16TextureUnitState10getEffectsEv
#[doc(alias = "Ogre::TextureUnitState::getEffects(void)const")]
// was: Ogre::TextureUnitState::getEffects(void)const
// IDA 0xe4d11c: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4d11c() {
}

// 0xe4d124 — __ZN4Ogre16TextureUnitState19setTextureFilteringENS_20TextureFilterOptionsE
#[doc(alias = "Ogre::TextureUnitState::setTextureFiltering(Ogre::TextureFilterOptions)")]
// was: Ogre::TextureUnitState::setTextureFiltering(Ogre::TextureFilterOptions)
// IDA 0xe4d124: 28 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4d124() {
}

// 0xe4d178 — __ZN4Ogre16TextureUnitState19setTextureFilteringENS_13FilterOptionsES1_S1_
#[doc(alias = "Ogre::TextureUnitState::setTextureFiltering(Ogre::FilterOptions,Ogre::FilterOptions,Ogre::FilterOptions)")]
// was: Ogre::TextureUnitState::setTextureFiltering(Ogre::FilterOptions,Ogre::FilterOptions,Ogre::FilterOptions)
// IDA 0xe4d178: 5 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4d178() {
}

// 0xe4d188 — __ZNK4Ogre16TextureUnitState19getTextureFilteringENS_10FilterTypeE
#[doc(alias = "Ogre::TextureUnitState::getTextureFiltering(Ogre::FilterType)const")]
// was: Ogre::TextureUnitState::getTextureFiltering(Ogre::FilterType)const
// IDA 0xe4d188: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4d188() {
}

// 0xe4d1f0 — __ZN4Ogre16TextureUnitState20setTextureAnisotropyEj
#[doc(alias = "Ogre::TextureUnitState::setTextureAnisotropy(unsigned int)")]
// was: Ogre::TextureUnitState::setTextureAnisotropy(unsigned int)
// IDA 0xe4d1f0: 4 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4d1f0() {
}
