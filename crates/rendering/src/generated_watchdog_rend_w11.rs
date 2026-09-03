//! Generated watchdog rendering w11 — 120 stubs (rendering filter RBX::Gfx|Ogre|RBX::Adorn|RBX::VisualEngine)
//! Source: ida/export.json (85545 funcs) rendering-filtered, global-deduped
//! Range: 0xc93830..0xc9c1bc (120 stubs, EA-sorted asc)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };


// 0xc93830 — __ZNSt12_Vector_baseISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// IDA 0xc93830: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xc93830() {
}


// 0xc93834 — __ZNSt12_Vector_baseISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// IDA 0xc93834: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc93834() {
}


// 0xc93840 — __ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
#[doc(alias = "__ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// IDA 0xc93840: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc93840() {
}


// 0xc938e8 — __ZN4Ogre9SharedPtrINS_7TextureEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Texture>::operator=(Ogre::SharedPtr<Ogre::Texture> const&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_7TextureEEaSERKS2_")]
// IDA 0xc938e8: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc938e8() {
}


// 0xc93a68 — __ZN4Ogre9SharedPtrINS_7TextureEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Texture>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_7TextureEED0Ev")]
// IDA 0xc93a68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc93a68() {
}


// 0xc93b60 — __ZN4Ogre22InternalErrorExceptionD0Ev
#[doc(alias = "Ogre::InternalErrorException::~InternalErrorException()")]
#[doc(alias = "__ZN4Ogre22InternalErrorExceptionD0Ev")]
// IDA 0xc93b60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc93b60() {
}


// 0xc93b78 — __ZN4Ogre9SharedPtrINS_8MaterialEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_8MaterialEED0Ev")]
// IDA 0xc93b78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc93b78() {
}


// 0xc93c70 — __ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
#[doc(alias = "std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::ParameterDef const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_")]
// IDA 0xc93c70: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_0xc93c70() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}


// 0xc93d90 — __ZNSt3mapISsPN4Ogre12ParamCommandESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::ParamCommand *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsPN4Ogre12ParamCommandESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
// IDA 0xc93d90: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc93d90() {
}


// 0xc93f50 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamCommand *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// IDA 0xc93f50: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc93f50() {
}


// 0xc94038 — __ZSt22__uninitialized_copy_aIPN4Ogre12ParameterDefES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
#[doc(alias = "Ogre::ParameterDef * std::__uninitialized_copy_a<Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
#[doc(alias = "__ZSt22__uninitialized_copy_aIPN4Ogre12ParameterDefES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_")]
// IDA 0xc94038: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc94038() {
}


// 0xc94238 — __ZN4Ogre12STLAllocatorINS_12ParameterDefENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
#[doc(alias = "__ZN4Ogre12STLAllocatorINS_12ParameterDefENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev")]
// IDA 0xc94238: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc94238() {
}


// 0xc94248 — __ZNSt12_Vector_baseIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// IDA 0xc94248: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xc94248() {
}


// 0xc94250 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev")]
// IDA 0xc94250: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc94250() {
}


// 0xc94260 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSH_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> const*,std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>>*)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSH_")]
// IDA 0xc94260: 195 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc94260() {
}


// 0xc94450 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamDictionary> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
// IDA 0xc94450: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc94450() {
}


// 0xc94538 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::ParamDictionary> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_")]
// IDA 0xc94538: 106 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc94538() {
}


// 0xc947c0 — __ZN4Ogre11FontManager12getSingletonEv
#[doc(alias = "Ogre::FontManager::getSingleton(void)")]
#[doc(alias = "__ZN4Ogre11FontManager12getSingletonEv")]
// IDA 0xc947c0: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc947c0() {
}


// 0xc947d0 — __ZN4Ogre11FontManagerC1Ev
#[doc(alias = "Ogre::FontManager::FontManager(void)")]
#[doc(alias = "__ZN4Ogre11FontManagerC1Ev")]
// IDA 0xc947d0: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc947d0() {
}


// 0xc947dc — __ZN4Ogre11FontManagerC2Ev
#[doc(alias = "Ogre::FontManager::FontManager(void)")]
#[doc(alias = "__ZN4Ogre11FontManagerC2Ev")]
// IDA 0xc947dc: 171 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc947dc() {
}


// 0xc949c4 — __ZN4Ogre11FontManagerD0Ev
#[doc(alias = "Ogre::FontManager::~FontManager()")]
#[doc(alias = "__ZN4Ogre11FontManagerD0Ev")]
// IDA 0xc949c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc949c4() {
}


// 0xc94abc — __ZN4Ogre11FontManagerD1Ev
#[doc(alias = "Ogre::FontManager::~FontManager()")]
#[doc(alias = "__ZN4Ogre11FontManagerD1Ev")]
// IDA 0xc94abc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc94abc() {
}


// 0xc94ba4 — __ZN4Ogre11FontManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::FontManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
#[doc(alias = "__ZN4Ogre11FontManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE")]
// IDA 0xc94ba4: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc94ba4() {
}


// 0xc95368 — __ZN4Ogre11FontManager14parseAttributeERKSsRNS_7FontPtrE
#[doc(alias = "Ogre::FontManager::parseAttribute(std::string const&,Ogre::FontPtr &)")]
#[doc(alias = "__ZN4Ogre11FontManager14parseAttributeERKSsRNS_7FontPtrE")]
// IDA 0xc95368: 891 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc95368() {
}


// 0xc95cfc — __ZN4Ogre11FontManager12logBadAttribERKSsRNS_7FontPtrE
#[doc(alias = "Ogre::FontManager::logBadAttrib(std::string const&,Ogre::FontPtr &)")]
#[doc(alias = "__ZN4Ogre11FontManager12logBadAttribERKSsRNS_7FontPtrE")]
// IDA 0xc95cfc: 285 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc95cfc() {
}


// 0xc96014 — __ZN4Ogre7FontPtrD1Ev
#[doc(alias = "Ogre::FontPtr::~FontPtr()")]
#[doc(alias = "__ZN4Ogre7FontPtrD1Ev")]
// IDA 0xc96014: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc96014() {
}


// 0xc96108 — __ZNK4Ogre15ResourceManager15getLoadingOrderEv
#[doc(alias = "Ogre::ResourceManager::getLoadingOrder(void)const")]
#[doc(alias = "__ZNK4Ogre15ResourceManager15getLoadingOrderEv")]
// IDA 0xc96108: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc96108() {
}


// 0xc9610c — __ZNK4Ogre15ResourceManager14getMemoryUsageEv
#[doc(alias = "Ogre::ResourceManager::getMemoryUsage(void)const")]
#[doc(alias = "__ZNK4Ogre15ResourceManager14getMemoryUsageEv")]
// IDA 0xc9610c: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9610c() {
}


// 0xc96110 — __ZN4Ogre15ResourceManager14resourceExistsERKSs
#[doc(alias = "Ogre::ResourceManager::resourceExists(std::string const&)")]
#[doc(alias = "__ZN4Ogre15ResourceManager14resourceExistsERKSs")]
// IDA 0xc96110: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc96110() {
}


// 0xc96220 — __ZN4Ogre15ResourceManager14resourceExistsEy
#[doc(alias = "Ogre::ResourceManager::resourceExists(unsigned long long)")]
#[doc(alias = "__ZN4Ogre15ResourceManager14resourceExistsEy")]
// IDA 0xc96220: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc96220() {
}


// 0xc96324 — __ZN4Ogre15ResourceManager10setVerboseEb
#[doc(alias = "Ogre::ResourceManager::setVerbose(bool)")]
#[doc(alias = "__ZN4Ogre15ResourceManager10setVerboseEb")]
// IDA 0xc96324: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc96324() {
}


// 0xc9632c — __ZN4Ogre15ResourceManager10getVerboseEv
#[doc(alias = "Ogre::ResourceManager::getVerbose(void)")]
#[doc(alias = "__ZN4Ogre15ResourceManager10getVerboseEv")]
// IDA 0xc9632c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9632c() {
}


// 0xc96338 — __ZSt22__uninitialized_copy_aIPSsS0_N4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEET0_T_S8_S7_T1_
#[doc(alias = "std::string * std::__uninitialized_copy_a<std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::string *,std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
#[doc(alias = "__ZSt22__uninitialized_copy_aIPSsS0_N4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEET0_T_S8_S7_T1_")]
// IDA 0xc96338: 67 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc96338() {
}


// 0xc96488 — __ZNSt6vectorISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S8_EERKS1_
#[doc(alias = "std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<unsigned int,unsigned int>*,std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::pair<unsigned int,unsigned int> const&)")]
#[doc(alias = "__ZNSt6vectorISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S8_EERKS1_")]
// IDA 0xc96488: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xc96488() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xc96a30 — __ZN4Ogre9SharedPtrINS_4FontEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4FontEED1Ev")]
// IDA 0xc96a30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc96a30() {
}


// 0xc96ae0 — __ZN4Ogre9SharedPtrINS_4FontEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4FontEED0Ev")]
// IDA 0xc96ae0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc96ae0() {
}


// 0xc96bd4 — __ZN4Ogre9SharedPtrINS_4FontEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4FontEE7destroyEv")]
// IDA 0xc96bd4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc96bd4() {
}


// 0xc96c0c — __ZN4Ogre9SharedPtrINS_4FontEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::swap(Ogre::SharedPtr<Ogre::Font>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4FontEE4swapERS2_")]
// IDA 0xc96c0c: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc96c0c() {
}


// 0xc96c28 — __ZN4Ogre7FontPtrD0Ev
#[doc(alias = "Ogre::FontPtr::~FontPtr()")]
#[doc(alias = "__ZN4Ogre7FontPtrD0Ev")]
// IDA 0xc96c28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc96c28() {
}


// 0xc96d50 — __ZN4Ogre25FreeImageLoadErrorHandlerE17FREE_IMAGE_FORMATPKc
#[doc(alias = "Ogre::FreeImageLoadErrorHandler(FREE_IMAGE_FORMAT,char const*)")]
#[doc(alias = "__ZN4Ogre25FreeImageLoadErrorHandlerE17FREE_IMAGE_FORMATPKc")]
// IDA 0xc96d50: 162 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc96d50() {
}


// 0xc96f14 — __ZN4Ogre14FreeImageCodec7startupEv
#[doc(alias = "Ogre::FreeImageCodec::startup(void)")]
#[doc(alias = "__ZN4Ogre14FreeImageCodec7startupEv")]
// IDA 0xc96f14: 761 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc96f14() {
}


// 0xc9778c — __ZN4Ogre14FreeImageCodec8shutdownEv
#[doc(alias = "Ogre::FreeImageCodec::shutdown(void)")]
#[doc(alias = "__ZN4Ogre14FreeImageCodec8shutdownEv")]
// IDA 0xc9778c: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9778c() {
}


// 0xc988c4 — __ZNK4Ogre14FreeImageCodec7getTypeEv
#[doc(alias = "Ogre::FreeImageCodec::getType(void)const")]
#[doc(alias = "__ZNK4Ogre14FreeImageCodec7getTypeEv")]
// IDA 0xc988c4: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc988c4() {
}


// 0xc988d0 — __ZNK4Ogre14FreeImageCodec20magicNumberToFileExtEPKcm
#[doc(alias = "Ogre::FreeImageCodec::magicNumberToFileExt(char const*,unsigned long)const")]
#[doc(alias = "__ZNK4Ogre14FreeImageCodec20magicNumberToFileExtEPKcm")]
// IDA 0xc988d0: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc988d0() {
}


// 0xc98a48 — __ZNSt4listIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(alias = "std::list<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~list()")]
#[doc(alias = "__ZNSt4listIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev")]
// IDA 0xc98a48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc98a48() {
}


// 0xc98af0 — __ZN4Ogre14FreeImageCodecD1Ev
#[doc(alias = "Ogre::FreeImageCodec::~FreeImageCodec()")]
#[doc(alias = "__ZN4Ogre14FreeImageCodecD1Ev")]
// IDA 0xc98af0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc98af0() {
}


// 0xc98b50 — __ZN4Ogre14FreeImageCodecD0Ev
#[doc(alias = "Ogre::FreeImageCodec::~FreeImageCodec()")]
#[doc(alias = "__ZN4Ogre14FreeImageCodecD0Ev")]
// IDA 0xc98b50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc98b50() {
}


// 0xc98c30 — __ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
#[doc(alias = "__ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev")]
// IDA 0xc98c30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xc98c30() {
}


// 0xc98c34 — __ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
#[doc(alias = "__ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev")]
// IDA 0xc98c34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc98c34() {
}


// 0xc98cb4 — __ZN4Ogre7FrustumC1ERKSs
#[doc(alias = "Ogre::Frustum::Frustum(std::string const&)")]
#[doc(alias = "__ZN4Ogre7FrustumC1ERKSs")]
// IDA 0xc98cb4: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc98cb4() {
}


// 0xc98cc0 — __ZN4Ogre7FrustumC2ERKSs
#[doc(alias = "Ogre::Frustum::Frustum(std::string const&)")]
#[doc(alias = "__ZN4Ogre7FrustumC2ERKSs")]
// IDA 0xc98cc0: 617 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc98cc0() {
}


// 0xc99398 — __ZN4Ogre7FrustumD0Ev
#[doc(alias = "Ogre::Frustum::~Frustum()")]
#[doc(alias = "__ZN4Ogre7FrustumD0Ev")]
// IDA 0xc99398: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc99398() {
}


// 0xc99428 — __ZN4Ogre7FrustumD1Ev
#[doc(alias = "Ogre::Frustum::~Frustum()")]
#[doc(alias = "__ZN4Ogre7FrustumD1Ev")]
// IDA 0xc99428: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc99428() {
}


// 0xc99434 — __ZThn4_N4Ogre7FrustumD0Ev
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
#[doc(alias = "__ZThn4_N4Ogre7FrustumD0Ev")]
// IDA 0xc99434: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc99434() {
}


// 0xc994c8 — __ZThn188_N4Ogre7FrustumD0Ev
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
#[doc(alias = "__ZThn188_N4Ogre7FrustumD0Ev")]
// IDA 0xc994c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc994c8() {
}


// 0xc9955c — __ZN4Ogre7FrustumD2Ev
#[doc(alias = "Ogre::Frustum::~Frustum()")]
#[doc(alias = "__ZN4Ogre7FrustumD2Ev")]
// IDA 0xc9955c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc9955c() {
}


// 0xc996e0 — __ZThn4_N4Ogre7FrustumD1Ev
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
#[doc(alias = "__ZThn4_N4Ogre7FrustumD1Ev")]
// IDA 0xc996e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc996e0() {
}


// 0xc996ec — __ZThn188_N4Ogre7FrustumD1Ev
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
#[doc(alias = "__ZThn188_N4Ogre7FrustumD1Ev")]
// IDA 0xc996ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc996ec() {
}


// 0xc996f8 — __ZN4Ogre7Frustum7setFOVyERKNS_6RadianE
#[doc(alias = "Ogre::Frustum::setFOVy(Ogre::Radian const&)")]
#[doc(alias = "__ZN4Ogre7Frustum7setFOVyERKNS_6RadianE")]
// IDA 0xc996f8: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc996f8() {
}


// 0xc9970c — __ZNK4Ogre7Frustum7getFOVyEv
#[doc(alias = "Ogre::Frustum::getFOVy(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum7getFOVyEv")]
// IDA 0xc9970c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9970c() {
}


// 0xc99710 — __ZN4Ogre7Frustum18setFarClipDistanceEf
#[doc(alias = "Ogre::Frustum::setFarClipDistance(float)")]
#[doc(alias = "__ZN4Ogre7Frustum18setFarClipDistanceEf")]
// IDA 0xc99710: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99710() {
}


// 0xc99724 — __ZNK4Ogre7Frustum18getFarClipDistanceEv
#[doc(alias = "Ogre::Frustum::getFarClipDistance(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum18getFarClipDistanceEv")]
// IDA 0xc99724: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99724() {
}


// 0xc9972c — __ZN4Ogre7Frustum19setNearClipDistanceEf
#[doc(alias = "Ogre::Frustum::setNearClipDistance(float)")]
#[doc(alias = "__ZN4Ogre7Frustum19setNearClipDistanceEf")]
// IDA 0xc9972c: 164 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9972c() {
}


// 0xc9991c — __ZNK4Ogre7Frustum19getNearClipDistanceEv
#[doc(alias = "Ogre::Frustum::getNearClipDistance(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum19getNearClipDistanceEv")]
// IDA 0xc9991c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9991c() {
}


// 0xc99924 — __ZN4Ogre7Frustum16setFrustumOffsetERKNS_7Vector2E
#[doc(alias = "Ogre::Frustum::setFrustumOffset(Ogre::Vector2 const&)")]
#[doc(alias = "__ZN4Ogre7Frustum16setFrustumOffsetERKNS_7Vector2E")]
// IDA 0xc99924: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99924() {
}


// 0xc99940 — __ZN4Ogre7Frustum16setFrustumOffsetEff
#[doc(alias = "Ogre::Frustum::setFrustumOffset(float,float)")]
#[doc(alias = "__ZN4Ogre7Frustum16setFrustumOffsetEff")]
// IDA 0xc99940: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99940() {
}


// 0xc99958 — __ZNK4Ogre7Frustum16getFrustumOffsetEv
#[doc(alias = "Ogre::Frustum::getFrustumOffset(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum16getFrustumOffsetEv")]
// IDA 0xc99958: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99958() {
}


// 0xc99960 — __ZN4Ogre7Frustum14setFocalLengthEf
#[doc(alias = "Ogre::Frustum::setFocalLength(float)")]
#[doc(alias = "__ZN4Ogre7Frustum14setFocalLengthEf")]
// IDA 0xc99960: 164 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99960() {
}


// 0xc99b50 — __ZNK4Ogre7Frustum14getFocalLengthEv
#[doc(alias = "Ogre::Frustum::getFocalLength(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum14getFocalLengthEv")]
// IDA 0xc99b50: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99b50() {
}


// 0xc99b58 — __ZNK4Ogre7Frustum19getProjectionMatrixEv
#[doc(alias = "Ogre::Frustum::getProjectionMatrix(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum19getProjectionMatrixEv")]
// IDA 0xc99b58: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99b58() {
}


// 0xc99b70 — __ZNK4Ogre7Frustum30getProjectionMatrixWithRSDepthEv
#[doc(alias = "Ogre::Frustum::getProjectionMatrixWithRSDepth(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum30getProjectionMatrixWithRSDepthEv")]
// IDA 0xc99b70: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99b70() {
}


// 0xc99b88 — __ZNK4Ogre7Frustum21getProjectionMatrixRSEv
#[doc(alias = "Ogre::Frustum::getProjectionMatrixRS(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum21getProjectionMatrixRSEv")]
// IDA 0xc99b88: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99b88() {
}


// 0xc99bb8 — __ZNK4Ogre7Frustum16getFrustumPlanesEv
#[doc(alias = "Ogre::Frustum::getFrustumPlanes(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum16getFrustumPlanesEv")]
// IDA 0xc99bb8: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99bb8() {
}


// 0xc99bd0 — __ZNK4Ogre7Frustum15getFrustumPlaneEt
#[doc(alias = "Ogre::Frustum::getFrustumPlane(unsigned short)const")]
#[doc(alias = "__ZNK4Ogre7Frustum15getFrustumPlaneEt")]
// IDA 0xc99bd0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99bd0() {
}


// 0xc99bec — __ZNK4Ogre7Frustum9isVisibleERKNS_14AxisAlignedBoxEPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum::isVisible(Ogre::AxisAlignedBox const&,Ogre::FrustumPlane *)const")]
#[doc(alias = "__ZNK4Ogre7Frustum9isVisibleERKNS_14AxisAlignedBoxEPNS_12FrustumPlaneE")]
// IDA 0xc99bec: 102 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99bec() {
}


// 0xc99d28 — __ZNK4Ogre7Frustum9isVisibleERKNS_7Vector3EPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum::isVisible(Ogre::Vector3 const&,Ogre::FrustumPlane *)const")]
#[doc(alias = "__ZNK4Ogre7Frustum9isVisibleERKNS_7Vector3EPNS_12FrustumPlaneE")]
// IDA 0xc99d28: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99d28() {
}


// 0xc99d90 — __ZNK4Ogre7Frustum9isVisibleERKNS_6SphereEPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum::isVisible(Ogre::Sphere const&,Ogre::FrustumPlane *)const")]
#[doc(alias = "__ZNK4Ogre7Frustum9isVisibleERKNS_6SphereEPNS_12FrustumPlaneE")]
// IDA 0xc99d90: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99d90() {
}


// 0xc99e0c — __ZNK4Ogre7Frustum12getTypeFlagsEv
#[doc(alias = "Ogre::Frustum::getTypeFlags(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum12getTypeFlagsEv")]
// IDA 0xc99e0c: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99e0c() {
}


// 0xc99e1c — __ZNK4Ogre7Frustum24calcProjectionParametersERfS1_S1_S1_
#[doc(alias = "Ogre::Frustum::calcProjectionParameters(float &,float &,float &,float &)const")]
#[doc(alias = "__ZNK4Ogre7Frustum24calcProjectionParametersERfS1_S1_S1_")]
// IDA 0xc99e1c: 157 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc99e1c() {
}


// 0xc9a054 — __ZNK4Ogre7Frustum17updateFrustumImplEv
#[doc(alias = "Ogre::Frustum::updateFrustumImpl(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum17updateFrustumImplEv")]
// IDA 0xc9a054: 394 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9a054() {
}


// 0xc9a5c0 — __ZNK4Ogre7Frustum13updateFrustumEv
#[doc(alias = "Ogre::Frustum::updateFrustum(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum13updateFrustumEv")]
// IDA 0xc9a5c0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9a5c0() {
}


// 0xc9a5e4 — __ZNK4Ogre7Frustum16updateVertexDataEv
#[doc(alias = "Ogre::Frustum::updateVertexData(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum16updateVertexDataEv")]
// IDA 0xc9a5e4: 512 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9a5e4() {
}


// 0xc9ad5c — __ZNK4Ogre7Frustum18isFrustumOutOfDateEv
#[doc(alias = "Ogre::Frustum::isFrustumOutOfDate(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum18isFrustumOutOfDateEv")]
// IDA 0xc9ad5c: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9ad5c() {
}


// 0xc9af84 — __ZNK4Ogre7Frustum23updateFrustumPlanesImplEv
#[doc(alias = "Ogre::Frustum::updateFrustumPlanesImpl(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum23updateFrustumPlanesImplEv")]
// IDA 0xc9af84: 110 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9af84() {
}


// 0xc9b11c — __ZNK4Ogre7Frustum19updateFrustumPlanesEv
#[doc(alias = "Ogre::Frustum::updateFrustumPlanes(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum19updateFrustumPlanesEv")]
// IDA 0xc9b11c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b11c() {
}


// 0xc9b14c — __ZNK4Ogre7Frustum27updateWorldSpaceCornersImplEv
#[doc(alias = "Ogre::Frustum::updateWorldSpaceCornersImpl(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum27updateWorldSpaceCornersImplEv")]
// IDA 0xc9b14c: 197 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b14c() {
}


// 0xc9b438 — __ZNK4Ogre7Frustum23updateWorldSpaceCornersEv
#[doc(alias = "Ogre::Frustum::updateWorldSpaceCorners(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum23updateWorldSpaceCornersEv")]
// IDA 0xc9b438: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b438() {
}


// 0xc9b460 — __ZNK4Ogre7Frustum14getAspectRatioEv
#[doc(alias = "Ogre::Frustum::getAspectRatio(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum14getAspectRatioEv")]
// IDA 0xc9b460: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b460() {
}


// 0xc9b468 — __ZN4Ogre7Frustum14setAspectRatioEf
#[doc(alias = "Ogre::Frustum::setAspectRatio(float)")]
#[doc(alias = "__ZN4Ogre7Frustum14setAspectRatioEf")]
// IDA 0xc9b468: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b468() {
}


// 0xc9b47c — __ZNK4Ogre7Frustum14getBoundingBoxEv
#[doc(alias = "Ogre::Frustum::getBoundingBox(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum14getBoundingBoxEv")]
// IDA 0xc9b47c: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b47c() {
}


// 0xc9b484 — __ZN4Ogre7Frustum18_updateRenderQueueEPNS_11RenderQueueE
#[doc(alias = "Ogre::Frustum::_updateRenderQueue(Ogre::RenderQueue *)")]
#[doc(alias = "__ZN4Ogre7Frustum18_updateRenderQueueEPNS_11RenderQueueE")]
// IDA 0xc9b484: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b484() {
}


// 0xc9b4a0 — __ZNK4Ogre7Frustum14getMovableTypeEv
#[doc(alias = "Ogre::Frustum::getMovableType(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum14getMovableTypeEv")]
// IDA 0xc9b4a0: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b4a0() {
}


// 0xc9b4ac — __ZNK4Ogre7Frustum17getBoundingRadiusEv
#[doc(alias = "Ogre::Frustum::getBoundingRadius(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum17getBoundingRadiusEv")]
// IDA 0xc9b4ac: 8 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b4ac() {
}


// 0xc9b4cc — __ZNK4Ogre7Frustum11getMaterialEv
#[doc(alias = "Ogre::Frustum::getMaterial(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum11getMaterialEv")]
// IDA 0xc9b4cc: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b4cc() {
}


// 0xc9b4d4 — __ZThn188_NK4Ogre7Frustum11getMaterialEv
#[doc(alias = "non-virtual thunk toOgre::Frustum::getMaterial(void)const")]
#[doc(alias = "__ZThn188_NK4Ogre7Frustum11getMaterialEv")]
// IDA 0xc9b4d4: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b4d4() {
}


// 0xc9b4dc — __ZN4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::Frustum::getRenderOperation(Ogre::RenderOperation &)")]
#[doc(alias = "__ZN4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE")]
// IDA 0xc9b4dc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b4dc() {
}


// 0xc9b500 — __ZThn188_N4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "non-virtual thunk toOgre::Frustum::getRenderOperation(Ogre::RenderOperation &)")]
#[doc(alias = "__ZThn188_N4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE")]
// IDA 0xc9b500: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b500() {
}


// 0xc9b524 — __ZNK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::Frustum::getWorldTransforms(Ogre::Matrix4 *)const")]
#[doc(alias = "__ZNK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E")]
// IDA 0xc9b524: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b524() {
}


// 0xc9b57c — __ZThn188_NK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "non-virtual thunk toOgre::Frustum::getWorldTransforms(Ogre::Matrix4 *)const")]
#[doc(alias = "__ZThn188_NK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E")]
// IDA 0xc9b57c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b57c() {
}


// 0xc9b638 — __ZThn188_NK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE
#[doc(alias = "non-virtual thunk toOgre::Frustum::getSquaredViewDepth(Ogre::Camera const*)const")]
#[doc(alias = "__ZThn188_NK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE")]
// IDA 0xc9b638: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b638() {
}


// 0xc9b69c — __ZNK4Ogre7Frustum9getLightsEv
#[doc(alias = "Ogre::Frustum::getLights(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum9getLightsEv")]
// IDA 0xc9b69c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b69c() {
}


// 0xc9b784 — __ZThn188_NK4Ogre7Frustum9getLightsEv
#[doc(alias = "non-virtual thunk toOgre::Frustum::getLights(void)const")]
#[doc(alias = "__ZThn188_NK4Ogre7Frustum9getLightsEv")]
// IDA 0xc9b784: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b784() {
}


// 0xc9b86c — __ZN4Ogre7Frustum20_notifyCurrentCameraEPNS_6CameraE
#[doc(alias = "Ogre::Frustum::_notifyCurrentCamera(Ogre::Camera *)")]
#[doc(alias = "__ZN4Ogre7Frustum20_notifyCurrentCameraEPNS_6CameraE")]
// IDA 0xc9b86c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b86c() {
}


// 0xc9b888 — __ZNK4Ogre7Frustum17invalidateFrustumEv
#[doc(alias = "Ogre::Frustum::invalidateFrustum(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum17invalidateFrustumEv")]
// IDA 0xc9b888: 6 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b888() {
}


// 0xc9b8ac — __ZNK4Ogre7Frustum20getWorldSpaceCornersEv
#[doc(alias = "Ogre::Frustum::getWorldSpaceCorners(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum20getWorldSpaceCornersEv")]
// IDA 0xc9b8ac: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b8ac() {
}


// 0xc9b8c4 — __ZN4Ogre7Frustum17setProjectionTypeENS_14ProjectionTypeE
#[doc(alias = "Ogre::Frustum::setProjectionType(Ogre::ProjectionType)")]
#[doc(alias = "__ZN4Ogre7Frustum17setProjectionTypeENS_14ProjectionTypeE")]
// IDA 0xc9b8c4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b8c4() {
}


// 0xc9b8d8 — __ZNK4Ogre7Frustum17getProjectionTypeEv
#[doc(alias = "Ogre::Frustum::getProjectionType(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum17getProjectionTypeEv")]
// IDA 0xc9b8d8: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9b8d8() {
}


// 0xc9ba50 — __ZNK4Ogre7Frustum13projectSphereERKNS_6SphereEPfS4_S4_S4_
#[doc(alias = "Ogre::Frustum::projectSphere(Ogre::Sphere const&,float *,float *,float *,float *)const")]
#[doc(alias = "__ZNK4Ogre7Frustum13projectSphereERKNS_6SphereEPfS4_S4_S4_")]
// IDA 0xc9ba50: 375 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9ba50() {
}


// 0xc9bfc4 — __ZN4Ogre7Frustum25enableCustomNearClipPlaneEPKNS_12MovablePlaneE
#[doc(alias = "Ogre::Frustum::enableCustomNearClipPlane(Ogre::MovablePlane const*)")]
#[doc(alias = "__ZN4Ogre7Frustum25enableCustomNearClipPlaneEPKNS_12MovablePlaneE")]
// IDA 0xc9bfc4: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9bfc4() {
}


// 0xc9c000 — __ZN4Ogre7Frustum25enableCustomNearClipPlaneERKNS_5PlaneE
#[doc(alias = "Ogre::Frustum::enableCustomNearClipPlane(Ogre::Plane const&)")]
#[doc(alias = "__ZN4Ogre7Frustum25enableCustomNearClipPlaneERKNS_5PlaneE")]
// IDA 0xc9c000: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9c000() {
}


// 0xc9c034 — __ZN4Ogre7Frustum26disableCustomNearClipPlaneEv
#[doc(alias = "Ogre::Frustum::disableCustomNearClipPlane(void)")]
#[doc(alias = "__ZN4Ogre7Frustum26disableCustomNearClipPlaneEv")]
// IDA 0xc9c034: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9c034() {
}


// 0xc9c0a0 — __ZN4Ogre7Frustum25setCustomProjectionMatrixEbRKNS_7Matrix4E
#[doc(alias = "Ogre::Frustum::setCustomProjectionMatrix(bool,Ogre::Matrix4 const&)")]
#[doc(alias = "__ZN4Ogre7Frustum25setCustomProjectionMatrixEbRKNS_7Matrix4E")]
// IDA 0xc9c0a0: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9c0a0() {
}


// 0xc9c0f4 — __ZN4Ogre7Frustum14setOrthoWindowEff
#[doc(alias = "Ogre::Frustum::setOrthoWindow(float,float)")]
#[doc(alias = "__ZN4Ogre7Frustum14setOrthoWindowEff")]
// IDA 0xc9c0f4: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9c0f4() {
}


// 0xc9c118 — __ZN4Ogre7Frustum20setOrthoWindowHeightEf
#[doc(alias = "Ogre::Frustum::setOrthoWindowHeight(float)")]
#[doc(alias = "__ZN4Ogre7Frustum20setOrthoWindowHeightEf")]
// IDA 0xc9c118: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9c118() {
}


// 0xc9c12c — __ZN4Ogre7Frustum19setOrthoWindowWidthEf
#[doc(alias = "Ogre::Frustum::setOrthoWindowWidth(float)")]
#[doc(alias = "__ZN4Ogre7Frustum19setOrthoWindowWidthEf")]
// IDA 0xc9c12c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9c12c() {
}


// 0xc9c14c — __ZNK4Ogre7Frustum20getOrthoWindowHeightEv
#[doc(alias = "Ogre::Frustum::getOrthoWindowHeight(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum20getOrthoWindowHeightEv")]
// IDA 0xc9c14c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9c14c() {
}


// 0xc9c154 — __ZNK4Ogre7Frustum19getOrthoWindowWidthEv
#[doc(alias = "Ogre::Frustum::getOrthoWindowWidth(void)const")]
#[doc(alias = "__ZNK4Ogre7Frustum19getOrthoWindowWidthEv")]
// IDA 0xc9c154: 5 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9c154() {
}


// 0xc9c168 — __ZN4Ogre7Frustum16visitRenderablesEPNS_10Renderable7VisitorEb
#[doc(alias = "Ogre::Frustum::visitRenderables(Ogre::Renderable::Visitor *,bool)")]
#[doc(alias = "__ZN4Ogre7Frustum16visitRenderablesEPNS_10Renderable7VisitorEb")]
// IDA 0xc9c168: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9c168() {
}


// 0xc9c190 — __ZN4Ogre7Frustum17setFrustumExtentsEffff
#[doc(alias = "Ogre::Frustum::setFrustumExtents(float,float,float,float)")]
#[doc(alias = "__ZN4Ogre7Frustum17setFrustumExtentsEffff")]
// IDA 0xc9c190: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9c190() {
}


// 0xc9c1bc — __ZN4Ogre7Frustum19resetFrustumExtentsEv
#[doc(alias = "Ogre::Frustum::resetFrustumExtents(void)")]
#[doc(alias = "__ZN4Ogre7Frustum19resetFrustumExtentsEv")]
// IDA 0xc9c1bc: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc9c1bc() {
}
