//! rendering — generated_507 — 100 stubs global dedup (rendering filtered, EA-sorted asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) NOT in /tmp/global_eas.txt — 100 uncovered EA-sorted asc 0xe486e8..0xf67bf4 (359 candidates remaining, 94160 global EAs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr). Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xe486e8 — __ZNK4Ogre14TextureManager27getPreferredIntegerBitDepthEv
// type: _DWORD __fastcall(Ogre::TextureManager *__hidden this)
#[doc(alias = "Ogre::TextureManager::getPreferredIntegerBitDepth(void)const")]
#[doc(alias = "__ZNK4Ogre14TextureManager27getPreferredIntegerBitDepthEv")]
// was: Ogre::TextureManager::getPreferredIntegerBitDepth(void)const
// IDA 0xe486e8: 2 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe486e8() {
}

// 0xe486f0 — __ZN4Ogre14TextureManager25setPreferredFloatBitDepthEtb
// type: _DWORD __fastcall(Ogre::TextureManager *__hidden this, unsigned __int16, bool)
#[doc(alias = "Ogre::TextureManager::setPreferredFloatBitDepth(unsigned short,bool)")]
#[doc(alias = "__ZN4Ogre14TextureManager25setPreferredFloatBitDepthEtb")]
// was: Ogre::TextureManager::setPreferredFloatBitDepth(unsigned short,bool)
// IDA 0xe486f0: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe486f0() {
}

// 0xe48764 — __ZNK4Ogre14TextureManager25getPreferredFloatBitDepthEv
// type: _DWORD __fastcall(Ogre::TextureManager *__hidden this)
#[doc(alias = "Ogre::TextureManager::getPreferredFloatBitDepth(void)const")]
#[doc(alias = "__ZNK4Ogre14TextureManager25getPreferredFloatBitDepthEv")]
// was: Ogre::TextureManager::getPreferredFloatBitDepth(void)const
// IDA 0xe48764: 2 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48764() {
}

// 0xe4876c — __ZN4Ogre14TextureManager21setPreferredBitDepthsEttb
// type: _DWORD __fastcall(Ogre::TextureManager *__hidden this, unsigned __int16, unsigned __int16, bool)
#[doc(alias = "Ogre::TextureManager::setPreferredBitDepths(unsigned short,unsigned short,bool)")]
#[doc(alias = "__ZN4Ogre14TextureManager21setPreferredBitDepthsEttb")]
// was: Ogre::TextureManager::setPreferredBitDepths(unsigned short,unsigned short,bool)
// IDA 0xe4876c: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4876c() {
}

// 0xe487f0 — __ZN4Ogre14TextureManager20setDefaultNumMipmapsEm
// type: _DWORD __fastcall(Ogre::TextureManager *__hidden this, unsigned int)
#[doc(alias = "Ogre::TextureManager::setDefaultNumMipmaps(unsigned long)")]
#[doc(alias = "__ZN4Ogre14TextureManager20setDefaultNumMipmapsEm")]
// was: Ogre::TextureManager::setDefaultNumMipmaps(unsigned long)
// IDA 0xe487f0: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe487f0() {
}

// 0xe487f8 — __ZN4Ogre14TextureManager17isFormatSupportedENS_11TextureTypeENS_11PixelFormatEi
#[doc(alias = "Ogre::TextureManager::isFormatSupported(Ogre::TextureType,Ogre::PixelFormat,int)")]
#[doc(alias = "__ZN4Ogre14TextureManager17isFormatSupportedENS_11TextureTypeENS_11PixelFormatEi")]
// was: Ogre::TextureManager::isFormatSupported(Ogre::TextureType,Ogre::PixelFormat,int)
// IDA 0xe487f8: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe487f8() {
}

// 0xe48814 — __ZN4Ogre14TextureManager27isEquivalentFormatSupportedENS_11TextureTypeENS_11PixelFormatEi
#[doc(alias = "Ogre::TextureManager::isEquivalentFormatSupported(Ogre::TextureType,Ogre::PixelFormat,int)")]
#[doc(alias = "__ZN4Ogre14TextureManager27isEquivalentFormatSupportedENS_11TextureTypeENS_11PixelFormatEi")]
// was: Ogre::TextureManager::isEquivalentFormatSupported(Ogre::TextureType,Ogre::PixelFormat,int)
// IDA 0xe48814: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48814() {
}

// 0xe4883c — __ZN4Ogre14TextureManager20getDefaultNumMipmapsEv
// type: _DWORD __fastcall(Ogre::TextureManager *__hidden this)
#[doc(alias = "Ogre::TextureManager::getDefaultNumMipmaps(void)")]
#[doc(alias = "__ZN4Ogre14TextureManager20getDefaultNumMipmapsEv")]
// was: Ogre::TextureManager::getDefaultNumMipmaps(void)
// IDA 0xe4883c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4883c() {
}

// 0xe48878 — __ZN4Ogre16TextureUnitStateC1EPNS_4PassE
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, Ogre::Pass *)
#[doc(alias = "Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *)")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateC1EPNS_4PassE")]
// was: Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *)
// IDA 0xe48878: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48878() {
}

// 0xe48884 — __ZN4Ogre16TextureUnitStateC2EPNS_4PassE
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, Ogre::Pass *)
#[doc(alias = "Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *)")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateC2EPNS_4PassE")]
// was: Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *)
// IDA 0xe48884: 425 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48884() {
}

// 0xe48d44 — __ZN4Ogre16TextureUnitState18setColourOperationENS_19LayerBlendOperationE
// type: int __fastcall(int result, int)
#[doc(alias = "Ogre::TextureUnitState::setColourOperation(Ogre::LayerBlendOperation)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState18setColourOperationENS_19LayerBlendOperationE")]
// was: Ogre::TextureUnitState::setColourOperation(Ogre::LayerBlendOperation)
// IDA 0xe48d44: 83 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48d44() {
}

// 0xe48e4c — __ZN4Ogre16TextureUnitState24setTextureAddressingModeENS0_21TextureAddressingModeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::TextureUnitState::setTextureAddressingMode(Ogre::TextureUnitState::TextureAddressingMode)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState24setTextureAddressingModeENS0_21TextureAddressingModeE")]
// was: Ogre::TextureUnitState::setTextureAddressingMode(Ogre::TextureUnitState::TextureAddressingMode)
// IDA 0xe48e4c: 4 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48e4c() {
}

// 0xe48e54 — __ZN4Ogre16TextureUnitStateC1EPNS_4PassERKS0_
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, Ogre::Pass *, const Ogre::TextureUnitState *)
#[doc(alias = "Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *,Ogre::TextureUnitState const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateC1EPNS_4PassERKS0_")]
// was: Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *,Ogre::TextureUnitState const&)
// IDA 0xe48e54: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48e54() {
}

// 0xe48e60 — __ZN4Ogre16TextureUnitStateC2EPNS_4PassERKS0_
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, Ogre::Pass *, const Ogre::TextureUnitState *)
#[doc(alias = "Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *,Ogre::TextureUnitState const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateC2EPNS_4PassERKS0_")]
// was: Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *,Ogre::TextureUnitState const&)
// IDA 0xe48e60: 297 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48e60() {
}

// 0xe491a8 — __ZN4Ogre16TextureUnitStateaSERKS0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::TextureUnitState::operator=(Ogre::TextureUnitState const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateaSERKS0_")]
// was: Ogre::TextureUnitState::operator=(Ogre::TextureUnitState const&)
// IDA 0xe491a8: 87 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe491a8() {
}

// 0xe492bc — __ZN4Ogre16TextureUnitState14setTextureNameERKSsNS_11TextureTypeE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, int, Ogre::NedPoolingImpl *, int, int, int, int, char, int, int, int, int)
#[doc(alias = "Ogre::TextureUnitState::setTextureName(std::string const&,Ogre::TextureType)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState14setTextureNameERKSsNS_11TextureTypeE")]
// was: Ogre::TextureUnitState::setTextureName(std::string const&,Ogre::TextureType)
// IDA 0xe492bc: 344 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe492bc() {
}

// 0xe4964c — __ZN4Ogre16TextureUnitState18setTextureCoordSetEj
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, unsigned int)
#[doc(alias = "Ogre::TextureUnitState::setTextureCoordSet(unsigned int)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState18setTextureCoordSetEj")]
// was: Ogre::TextureUnitState::setTextureCoordSet(unsigned int)
// IDA 0xe4964c: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4964c() {
}

// 0xe49650 — __ZN4Ogre16TextureUnitStateD1Ev
// type: void __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::~TextureUnitState()")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateD1Ev")]
// was: Ogre::TextureUnitState::~TextureUnitState()
// IDA 0xe49650: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe49650() {
}

// 0xe4965c — __ZN4Ogre16TextureUnitStateD2Ev
// type: void __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::~TextureUnitState()")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateD2Ev")]
// was: Ogre::TextureUnitState::~TextureUnitState()
// IDA 0xe4965c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe4965c() {
}

// 0xf67554 — j___ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexDeclaration * const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
// was: std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexDeclaration * const&)
// IDA 0xf67554: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67554() {
}

// 0xf67564 — j___ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexDeclaration *>,std::_Rb_tree_iterator<Ogre::VertexDeclaration *>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_")]
// was: std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexDeclaration *>,std::_Rb_tree_iterator<Ogre::VertexDeclaration *>)
// IDA 0xf67564: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67564() {
}

// 0xf67574 — j___ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexDeclaration *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// was: std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexDeclaration *> *)
// IDA 0xf67574: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67574() {
}

// 0xf67584 — j___ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareIndexBuffer *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// was: std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareIndexBuffer *> *)
// IDA 0xf67584: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67584() {
}

// 0xf67594 — j___ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexBufferBinding * const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
// was: std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexBufferBinding * const&)
// IDA 0xf67594: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67594() {
}

// 0xf675a4 — j___ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>,std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_")]
// was: std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>,std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>)
// IDA 0xf675a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf675a4() {
}

// 0xf675b4 — j___ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexBufferBinding *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// was: std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexBufferBinding *> *)
// IDA 0xf675b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf675b4() {
}

// 0xf675c4 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareVertexBuffer *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareVertexBuffer *> *)
// IDA 0xf675c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf675c4() {
}

// 0xf675d4 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)
// IDA 0xf675d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf675d4() {
}

// 0xf675e4 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)
// IDA 0xf675e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf675e4() {
}

// 0xf675f4 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)
// IDA 0xf675f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf675f4() {
}

// 0xf67604 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKS7_
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKS7_")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)
// IDA 0xf67604: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67604() {
}

// 0xf67614 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS6_E")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)
// IDA 0xf67614: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67614() {
}

// 0xf67624 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS6_ESI_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS6_ESI_")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>)
// IDA 0xf67624: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67624() {
}

// 0xf67634 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)
// IDA 0xf67634: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67634() {
}

// 0xf67644 — j___ZNSt3mapItN4Ogre29HardwareVertexBufferSharedPtrESt4lessItENS0_12STLAllocatorISt4pairIKtS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<unsigned short,Ogre::HardwareVertexBufferSharedPtr,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](unsigned short const&)")]
#[doc(alias = "j___ZNSt3mapItN4Ogre29HardwareVertexBufferSharedPtrESt4lessItENS0_12STLAllocatorISt4pairIKtS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_")]
// was: std::map<unsigned short,Ogre::HardwareVertexBufferSharedPtr,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](unsigned short const&)
// IDA 0xf67644: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67644() {
}

// 0xf67654 — j___ZNSt4listIN4Ogre13VertexElementENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE4sortIPFbRKS1_SA_EEEvT_
#[doc(alias = "void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::sort<bool (*)(Ogre::VertexElement const&,Ogre::VertexElement const&)>(bool (*)(Ogre::VertexElement const&,Ogre::VertexElement const&))")]
#[doc(alias = "j___ZNSt4listIN4Ogre13VertexElementENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE4sortIPFbRKS1_SA_EEEvT_")]
// was: void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::sort<bool (*)(Ogre::VertexElement const&,Ogre::VertexElement const&)>(bool (*)(Ogre::VertexElement const&,Ogre::VertexElement const&))
// IDA 0xf67654: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67654() {
}

// 0xf67664 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>> *)
// IDA 0xf67664: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67664() {
}

// 0xf67674 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)
// IDA 0xf67674: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67674() {
}

// 0xf67684 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)
// IDA 0xf67684: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67684() {
}

// 0xf67694 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>> *)
// IDA 0xf67694: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67694() {
}

// 0xf676a4 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)
// IDA 0xf676a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf676a4() {
}

// 0xf676b4 — j___ZNSt8_Rb_treeItSt4pairIKttESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,unsigned short> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKttESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,unsigned short> const&)
// IDA 0xf676b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf676b4() {
}

// 0xf676c4 — j___ZNSt8_Rb_treeItSt4pairIKttESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,unsigned short>>,std::pair<unsigned short const,unsigned short> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKttESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,unsigned short>>,std::pair<unsigned short const,unsigned short> const&)
// IDA 0xf676c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf676c4() {
}

// 0xf676d4 — j___ZN4Ogre9SharedPtrINS_19HighLevelGpuProgramEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::destroy(void)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_19HighLevelGpuProgramEE7destroyEv")]
// was: Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::destroy(void)
// IDA 0xf676d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf676d4() {
}

// 0xf676e4 — j___ZNSt3mapISsPN4Ogre26HighLevelGpuProgramFactoryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::HighLevelGpuProgramFactory *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsPN4Ogre26HighLevelGpuProgramFactoryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
// was: std::map<std::string,Ogre::HighLevelGpuProgramFactory *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xf676e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf676e4() {
}

// 0xf676f4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)
// IDA 0xf676f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf676f4() {
}

// 0xf67704 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)
// IDA 0xf67704: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67704() {
}

// 0xf67714 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xf67714: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67714() {
}

// 0xf67724 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>> *)
// IDA 0xf67724: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67724() {
}

// 0xf67734 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)
// IDA 0xf67734: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67734() {
}

// 0xf67744 — j___ZN4Ogre15LinearResampler5scaleERKNS_8PixelBoxES3_
// type: _DWORD __fastcall(Ogre::LinearResampler *__hidden this, const Ogre::PixelBox *, const Ogre::PixelBox *)
#[doc(alias = "Ogre::LinearResampler::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre15LinearResampler5scaleERKNS_8PixelBoxES3_")]
// was: Ogre::LinearResampler::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67744: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67744() {
}

// 0xf67754 — j___ZN4Ogre16NearestResamplerILj12EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<12u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj12EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<12u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67754: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67754() {
}

// 0xf67764 — j___ZN4Ogre16NearestResamplerILj16EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<16u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj16EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<16u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67764: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67764() {
}

// 0xf67774 — j___ZN4Ogre16NearestResamplerILj1EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<1u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj1EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<1u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67774: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67774() {
}

// 0xf67784 — j___ZN4Ogre16NearestResamplerILj2EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<2u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj2EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<2u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67784: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67784() {
}

// 0xf67794 — j___ZN4Ogre16NearestResamplerILj3EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<3u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj3EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<3u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67794: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67794() {
}

// 0xf677a4 — j___ZN4Ogre16NearestResamplerILj4EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<4u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj4EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<4u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf677a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf677a4() {
}

// 0xf677b4 — j___ZN4Ogre16NearestResamplerILj6EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<6u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj6EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<6u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf677b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf677b4() {
}

// 0xf677c4 — j___ZN4Ogre16NearestResamplerILj8EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<8u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj8EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<8u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf677c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf677c4() {
}

// 0xf677d4 — j___ZN4Ogre20LinearResampler_ByteILj1EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::LinearResampler_Byte<1u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre20LinearResampler_ByteILj1EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::LinearResampler_Byte<1u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf677d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf677d4() {
}

// 0xf677e4 — j___ZN4Ogre20LinearResampler_ByteILj2EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::LinearResampler_Byte<2u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre20LinearResampler_ByteILj2EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::LinearResampler_Byte<2u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf677e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf677e4() {
}

// 0xf677f4 — j___ZN4Ogre20LinearResampler_ByteILj3EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::LinearResampler_Byte<3u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre20LinearResampler_ByteILj3EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::LinearResampler_Byte<3u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf677f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf677f4() {
}

// 0xf67804 — j___ZN4Ogre20LinearResampler_ByteILj4EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::LinearResampler_Byte<4u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre20LinearResampler_ByteILj4EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::LinearResampler_Byte<4u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67804: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67804() {
}

// 0xf67814 — j___ZN4Ogre23LinearResampler_Float325scaleERKNS_8PixelBoxES3_
#[doc(alias = "Ogre::LinearResampler_Float32::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre23LinearResampler_Float325scaleERKNS_8PixelBoxES3_")]
// was: Ogre::LinearResampler_Float32::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67814: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67814() {
}

// 0xf67854 — j___ZNSt6vectorIPN4Ogre15RenderOperationENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderOperation **,std::vector<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderOperation * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre15RenderOperationENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// was: std::vector<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderOperation **,std::vector<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderOperation * const&)
// IDA 0xf67854: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf67854() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf679a4 — j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long const,Ogre::Vector4> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long const,Ogre::Vector4> const&)
// IDA 0xf679a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf679a4() {
}

// 0xf679b4 — j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::Vector4>>,std::pair<unsigned long const,Ogre::Vector4> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::Vector4>>,std::pair<unsigned long const,Ogre::Vector4> const&)
// IDA 0xf679b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf679b4() {
}

// 0xf679f4 — j___ZNSt6vectorIN4Ogre18VertexPoseKeyFrame7PoseRefENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::VertexPoseKeyFrame::PoseRef*,std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::VertexPoseKeyFrame::PoseRef const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre18VertexPoseKeyFrame7PoseRefENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// was: std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::VertexPoseKeyFrame::PoseRef*,std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::VertexPoseKeyFrame::PoseRef const&)
// IDA 0xf679f4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf679f4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf67a04 — j___ZNSt6vectorIN4Ogre18VertexPoseKeyFrame7PoseRefENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
#[doc(alias = "std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre18VertexPoseKeyFrame7PoseRefENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_")]
// was: std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xf67a04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a04() {
}

// 0xf67a14 — j___ZN4Ogre9SharedPtrINS_13AnimableValueEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::destroy(void)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_13AnimableValueEE7destroyEv")]
// was: Ogre::SharedPtr<Ogre::AnimableValue>::destroy(void)
// IDA 0xf67a14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a14() {
}

// 0xf67a24 — j___ZNSt6vectorIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: int __fastcall(int, int, void *)
#[doc(alias = "std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PlaneBoundedVolume*,std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PlaneBoundedVolume const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
// was: std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PlaneBoundedVolume*,std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PlaneBoundedVolume const&)
// IDA 0xf67a24: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf67a24() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf67a34 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::Vector4> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::Vector4> const&)
// IDA 0xf67a34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a34() {
}

// 0xf67a44 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::Vector4>>,std::pair<unsigned short const,Ogre::Vector4> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::Vector4>>,std::pair<unsigned short const,Ogre::Vector4> const&)
// IDA 0xf67a44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a44() {
}

// 0xf67a54 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::Vector4>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::Vector4>> *)
// IDA 0xf67a54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a54() {
}

// 0xf67a64 — j___ZSt22__uninitialized_copy_aIPN4Ogre18PlaneBoundedVolumeES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "Ogre::PlaneBoundedVolume * std::__uninitialized_copy_a<Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
#[doc(alias = "j___ZSt22__uninitialized_copy_aIPN4Ogre18PlaneBoundedVolumeES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_")]
// was: Ogre::PlaneBoundedVolume * std::__uninitialized_copy_a<Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
// IDA 0xf67a64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a64() {
}

// 0xf67a74 — j___ZNSt11__iter_swapILb1EE9iter_swapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS5_NS4_12STLAllocatorIS5_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEEEESE_EEvT_T0_
#[doc(alias = "void std::__iter_swap<true>::iter_swap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>)")]
#[doc(alias = "j___ZNSt11__iter_swapILb1EE9iter_swapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS5_NS4_12STLAllocatorIS5_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEEEESE_EEvT_T0_")]
// was: void std::__iter_swap<true>::iter_swap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>)
// IDA 0xf67a74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a74() {
}

// 0xf67a84 — j___ZSt10__pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_16LodUsageSortLessEEvT_SE_SE_T0_T1_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void std::__pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt10__pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_16LodUsageSortLessEEvT_SE_SE_T0_T1_")]
// was: void std::__pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)
// IDA 0xf67a84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a84() {
}

// 0xf67a94 — j___ZSt10__pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_19LodUsageSortGreaterEEvT_SE_SE_T0_T1_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void std::__pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt10__pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_19LodUsageSortGreaterEEvT_SE_SE_T0_T1_")]
// was: void std::__pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)
// IDA 0xf67a94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a94() {
}

// 0xf67aa4 — j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_16LodUsageSortLessEEvT_T0_SF_T1_T2_
#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_16LodUsageSortLessEEvT_T0_SF_T1_T2_")]
// was: void std::__push_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)
// IDA 0xf67aa4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67aa4() {
}

// 0xf67ab4 — j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_19LodUsageSortGreaterEEvT_T0_SF_T1_T2_
#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_19LodUsageSortGreaterEEvT_T0_SF_T1_T2_")]
// was: void std::__push_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)
// IDA 0xf67ab4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67ab4() {
}

// 0xf67ac4 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_16LodUsageSortLessEEvT_T0_SF_T1_T2_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_16LodUsageSortLessEEvT_T0_SF_T1_T2_")]
// was: void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)
// IDA 0xf67ac4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67ac4() {
}

// 0xf67ad4 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_19LodUsageSortGreaterEEvT_T0_SF_T1_T2_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_19LodUsageSortGreaterEEvT_T0_SF_T1_T2_")]
// was: void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)
// IDA 0xf67ad4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67ad4() {
}

// 0xf67ae4 — j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_SE_T0_")]
// was: void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xf67ae4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67ae4() {
}

// 0xf67af4 — j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_SE_T0_")]
// was: void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xf67af4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67af4() {
}

// 0xf67b04 — j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, char, char, int, int, int, int, int)
#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_")]
// was: void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xf67b04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b04() {
}

// 0xf67b14 — j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, char, char, int, int, int, int, int)
#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_")]
// was: void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xf67b14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b14() {
}

// 0xf67b24 — j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_16LodUsageSortLessEEvT_SE_T0_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_16LodUsageSortLessEEvT_SE_T0_T1_")]
// was: void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortLess)
// IDA 0xf67b24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b24() {
}

// 0xf67b34 — j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_19LodUsageSortGreaterEEvT_SE_T0_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_19LodUsageSortGreaterEEvT_SE_T0_T1_")]
// was: void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortGreater)
// IDA 0xf67b34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b34() {
}

// 0xf67b44 — j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_")]
// was: void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xf67b44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b44() {
}

// 0xf67b54 — j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_")]
// was: void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xf67b54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b54() {
}

// 0xf67b64 — j___ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_16LodUsageSortLessEEvT_T0_T1_
#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_16LodUsageSortLessEEvT_T0_T1_")]
// was: void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)
// IDA 0xf67b64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b64() {
}

// 0xf67b74 — j___ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_19LodUsageSortGreaterEEvT_T0_T1_
#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_19LodUsageSortGreaterEEvT_T0_T1_")]
// was: void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)
// IDA 0xf67b74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b74() {
}

// 0xf67b84 — j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_
#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_")]
// was: void std::pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xf67b84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b84() {
}

// 0xf67b94 — j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_
#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_")]
// was: void std::pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xf67b94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b94() {
}

// 0xf67ba4 — j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::make_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_")]
// was: void std::make_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xf67ba4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67ba4() {
}

// 0xf67bb4 — j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::make_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_")]
// was: void std::make_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xf67bb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67bb4() {
}

// 0xf67bc4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::LodStrategy *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::LodStrategy *> const&)
// IDA 0xf67bc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67bc4() {
}

// 0xf67bd4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xf67bd4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67bd4() {
}

// 0xf67be4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::LodStrategy *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::LodStrategy *>> *)
// IDA 0xf67be4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67be4() {
}

// 0xf67bf4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::LodStrategy *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::LodStrategy *> const&)
// IDA 0xf67bf4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67bf4() {
}
