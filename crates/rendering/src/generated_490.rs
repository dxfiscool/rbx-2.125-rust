//! rendering shard 490 — 100 stubs EA-sorted asc rendering-filter not in /tmp/global_eas.txt (0xc93830..0xc9b638, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) rendering namespace filter (Ogre|Gfx|Render|G3D), global EA dedup.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xc93830 — __ZNSt12_Vector_baseISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
// IDA 0xc93830: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c93830() {
}


// 0xc93834 — __ZNSt12_Vector_baseISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// IDA 0xc93834: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93834() {
}


// 0xc93840 — __ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// was: __ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// IDA 0xc93840: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c93840() {
}


// 0xc938e8 — __ZN4Ogre9SharedPtrINS_7TextureEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::SharedPtr<Ogre::Texture>::operator=(Ogre::SharedPtr<Ogre::Texture> const&)")]
// was: __ZN4Ogre9SharedPtrINS_7TextureEEaSERKS2_
// IDA 0xc938e8: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c938e8() {
}


// 0xc93a68 — __ZN4Ogre9SharedPtrINS_7TextureEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Texture>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_7TextureEED0Ev
// IDA 0xc93a68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93a68() {
}


// 0xc93b60 — __ZN4Ogre22InternalErrorExceptionD0Ev
// type: void __fastcall(Ogre::InternalErrorException *__hidden this)
#[doc(alias = "Ogre::InternalErrorException::~InternalErrorException()")]
// was: __ZN4Ogre22InternalErrorExceptionD0Ev
// IDA 0xc93b60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93b60() {
}


// 0xc93b78 — __ZN4Ogre9SharedPtrINS_8MaterialEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_8MaterialEED0Ev
// IDA 0xc93b78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93b78() {
}


// 0xc93c70 — __ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::ParameterDef const&)")]
// was: __ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
// IDA 0xc93c70: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_c93c70() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}


// 0xc93d90 — __ZNSt3mapISsPN4Ogre12ParamCommandESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::ParamCommand *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: __ZNSt3mapISsPN4Ogre12ParamCommandESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// IDA 0xc93d90: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c93d90() {
}


// 0xc93f50 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamCommand *> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// IDA 0xc93f50: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c93f50() {
}


// 0xc94038 — __ZSt22__uninitialized_copy_aIPN4Ogre12ParameterDefES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "Ogre::ParameterDef * std::__uninitialized_copy_a<Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: __ZSt22__uninitialized_copy_aIPN4Ogre12ParameterDefES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
// IDA 0xc94038: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c94038() {
}


// 0xc94238 — __ZN4Ogre12STLAllocatorINS_12ParameterDefENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: __ZN4Ogre12STLAllocatorINS_12ParameterDefENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
// IDA 0xc94238: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c94238() {
}


// 0xc94248 — __ZNSt12_Vector_baseIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
// IDA 0xc94248: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c94248() {
}


// 0xc94250 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
// IDA 0xc94250: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c94250() {
}


// 0xc94260 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSH_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> const*,std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>>*)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSH_
// IDA 0xc94260: 195 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c94260() {
}


// 0xc94450 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamDictionary> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// IDA 0xc94450: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c94450() {
}


// 0xc94538 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::ParamDictionary> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_
// IDA 0xc94538: 106 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c94538() {
}


// 0xc947c0 — __ZN4Ogre11FontManager12getSingletonEv
// type: _DWORD __fastcall(Ogre::FontManager *__hidden this)
#[doc(alias = "Ogre::FontManager::getSingleton(void)")]
// was: __ZN4Ogre11FontManager12getSingletonEv
// IDA 0xc947c0: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c947c0() {
}


// 0xc947d0 — __ZN4Ogre11FontManagerC1Ev
// type: _DWORD __fastcall(Ogre::FontManager *__hidden this)
#[doc(alias = "Ogre::FontManager::FontManager(void)")]
// was: __ZN4Ogre11FontManagerC1Ev
// IDA 0xc947d0: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c947d0() {
}


// 0xc947dc — __ZN4Ogre11FontManagerC2Ev
// type: _DWORD __fastcall(Ogre::FontManager *__hidden this)
#[doc(alias = "Ogre::FontManager::FontManager(void)")]
// was: __ZN4Ogre11FontManagerC2Ev
// IDA 0xc947dc: 171 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c947dc() {
}


// 0xc949c4 — __ZN4Ogre11FontManagerD0Ev
// type: void __fastcall(Ogre::FontManager *__hidden this)
#[doc(alias = "Ogre::FontManager::~FontManager()")]
// was: __ZN4Ogre11FontManagerD0Ev
// IDA 0xc949c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c949c4() {
}


// 0xc94abc — __ZN4Ogre11FontManagerD1Ev
// type: void __fastcall(Ogre::FontManager *__hidden this)
#[doc(alias = "Ogre::FontManager::~FontManager()")]
// was: __ZN4Ogre11FontManagerD1Ev
// IDA 0xc94abc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c94abc() {
}


// 0xc94ba4 — __ZN4Ogre11FontManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, Ogre::ManualResourceLoader *, int, int, int)
#[doc(alias = "Ogre::FontManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: __ZN4Ogre11FontManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// IDA 0xc94ba4: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c94ba4() {
}


// 0xc95368 — __ZN4Ogre11FontManager14parseAttributeERKSsRNS_7FontPtrE
// type: _DWORD __fastcall(Ogre::FontManager *__hidden this, const std::string *, Ogre::FontPtr *)
#[doc(alias = "Ogre::FontManager::parseAttribute(std::string const&,Ogre::FontPtr &)")]
// was: __ZN4Ogre11FontManager14parseAttributeERKSsRNS_7FontPtrE
// IDA 0xc95368: 891 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c95368() {
}


// 0xc95cfc — __ZN4Ogre11FontManager12logBadAttribERKSsRNS_7FontPtrE
#[doc(alias = "Ogre::FontManager::logBadAttrib(std::string const&,Ogre::FontPtr &)")]
// was: __ZN4Ogre11FontManager12logBadAttribERKSsRNS_7FontPtrE
// IDA 0xc95cfc: 285 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c95cfc() {
}


// 0xc96014 — __ZN4Ogre7FontPtrD1Ev
// type: void __fastcall(Ogre::FontPtr *__hidden this)
#[doc(alias = "Ogre::FontPtr::~FontPtr()")]
// was: __ZN4Ogre7FontPtrD1Ev
// IDA 0xc96014: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c96014() {
}


// 0xc96108 — __ZNK4Ogre15ResourceManager15getLoadingOrderEv
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this)
#[doc(alias = "Ogre::ResourceManager::getLoadingOrder(void)const")]
// was: __ZNK4Ogre15ResourceManager15getLoadingOrderEv
// IDA 0xc96108: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96108() {
}


// 0xc9610c — __ZNK4Ogre15ResourceManager14getMemoryUsageEv
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this)
#[doc(alias = "Ogre::ResourceManager::getMemoryUsage(void)const")]
// was: __ZNK4Ogre15ResourceManager14getMemoryUsageEv
// IDA 0xc9610c: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9610c() {
}


// 0xc96110 — __ZN4Ogre15ResourceManager14resourceExistsERKSs
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::ResourceManager::resourceExists(std::string const&)")]
// was: __ZN4Ogre15ResourceManager14resourceExistsERKSs
// IDA 0xc96110: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96110() {
}


// 0xc96220 — __ZN4Ogre15ResourceManager14resourceExistsEy
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, unsigned __int64)
#[doc(alias = "Ogre::ResourceManager::resourceExists(unsigned long long)")]
// was: __ZN4Ogre15ResourceManager14resourceExistsEy
// IDA 0xc96220: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96220() {
}


// 0xc96324 — __ZN4Ogre15ResourceManager10setVerboseEb
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, bool)
#[doc(alias = "Ogre::ResourceManager::setVerbose(bool)")]
// was: __ZN4Ogre15ResourceManager10setVerboseEb
// IDA 0xc96324: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96324() {
}


// 0xc9632c — __ZN4Ogre15ResourceManager10getVerboseEv
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this)
#[doc(alias = "Ogre::ResourceManager::getVerbose(void)")]
// was: __ZN4Ogre15ResourceManager10getVerboseEv
// IDA 0xc9632c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9632c() {
}


// 0xc96338 — __ZSt22__uninitialized_copy_aIPSsS0_N4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEET0_T_S8_S7_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::string * std::__uninitialized_copy_a<std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::string *,std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: __ZSt22__uninitialized_copy_aIPSsS0_N4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEET0_T_S8_S7_T1_
// IDA 0xc96338: 67 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96338() {
}


// 0xc96488 — __ZNSt6vectorISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S8_EERKS1_
#[doc(alias = "std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<unsigned int,unsigned int>*,std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::pair<unsigned int,unsigned int> const&)")]
// was: __ZNSt6vectorISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S8_EERKS1_
// IDA 0xc96488: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_c96488() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xc96a30 — __ZN4Ogre9SharedPtrINS_4FontEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_4FontEED1Ev
// IDA 0xc96a30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c96a30() {
}


// 0xc96ae0 — __ZN4Ogre9SharedPtrINS_4FontEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_4FontEED0Ev
// IDA 0xc96ae0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c96ae0() {
}


// 0xc96bd4 — __ZN4Ogre9SharedPtrINS_4FontEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::destroy(void)")]
// was: __ZN4Ogre9SharedPtrINS_4FontEE7destroyEv
// IDA 0xc96bd4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96bd4() {
}


// 0xc96c0c — __ZN4Ogre9SharedPtrINS_4FontEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::swap(Ogre::SharedPtr<Ogre::Font>&)")]
// was: __ZN4Ogre9SharedPtrINS_4FontEE4swapERS2_
// IDA 0xc96c0c: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96c0c() {
}


// 0xc96c28 — __ZN4Ogre7FontPtrD0Ev
// type: void __fastcall(Ogre::FontPtr *__hidden this)
#[doc(alias = "Ogre::FontPtr::~FontPtr()")]
// was: __ZN4Ogre7FontPtrD0Ev
// IDA 0xc96c28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c96c28() {
}


// 0xc96d50 — __ZN4Ogre25FreeImageLoadErrorHandlerE17FREE_IMAGE_FORMATPKc
#[doc(alias = "Ogre::FreeImageLoadErrorHandler(FREE_IMAGE_FORMAT,char const*)")]
// was: __ZN4Ogre25FreeImageLoadErrorHandlerE17FREE_IMAGE_FORMATPKc
// IDA 0xc96d50: 162 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96d50() {
}


// 0xc96f14 — __ZN4Ogre14FreeImageCodec7startupEv
// type: _DWORD __fastcall(Ogre::FreeImageCodec *__hidden this)
#[doc(alias = "Ogre::FreeImageCodec::startup(void)")]
// was: __ZN4Ogre14FreeImageCodec7startupEv
// IDA 0xc96f14: 761 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96f14() {
}


// 0xc9778c — __ZN4Ogre14FreeImageCodec8shutdownEv
// type: _DWORD __fastcall(Ogre::FreeImageCodec *__hidden this)
#[doc(alias = "Ogre::FreeImageCodec::shutdown(void)")]
// was: __ZN4Ogre14FreeImageCodec8shutdownEv
// IDA 0xc9778c: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9778c() {
}


// 0xc988c4 — __ZNK4Ogre14FreeImageCodec7getTypeEv
// type: _DWORD __fastcall(Ogre::FreeImageCodec *__hidden this)
#[doc(alias = "Ogre::FreeImageCodec::getType(void)const")]
// was: __ZNK4Ogre14FreeImageCodec7getTypeEv
// IDA 0xc988c4: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c988c4() {
}


// 0xc988d0 — __ZNK4Ogre14FreeImageCodec20magicNumberToFileExtEPKcm
// type: _DWORD __fastcall(Ogre::FreeImageCodec *__hidden this, const char *, unsigned int)
#[doc(alias = "Ogre::FreeImageCodec::magicNumberToFileExt(char const*,unsigned long)const")]
// was: __ZNK4Ogre14FreeImageCodec20magicNumberToFileExtEPKcm
// IDA 0xc988d0: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c988d0() {
}


// 0xc98a48 — __ZNSt4listIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(alias = "std::list<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~list()")]
// was: __ZNSt4listIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
// IDA 0xc98a48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c98a48() {
}


// 0xc98af0 — __ZN4Ogre14FreeImageCodecD1Ev
// type: void __fastcall(Ogre::FreeImageCodec *__hidden this)
#[doc(alias = "Ogre::FreeImageCodec::~FreeImageCodec()")]
// was: __ZN4Ogre14FreeImageCodecD1Ev
// IDA 0xc98af0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c98af0() {
}


// 0xc98b50 — __ZN4Ogre14FreeImageCodecD0Ev
// type: void __fastcall(Ogre::FreeImageCodec *__hidden this)
#[doc(alias = "Ogre::FreeImageCodec::~FreeImageCodec()")]
// was: __ZN4Ogre14FreeImageCodecD0Ev
// IDA 0xc98b50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c98b50() {
}


// 0xc98c30 — __ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: __ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
// IDA 0xc98c30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c98c30() {
}


// 0xc98c34 — __ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: __ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
// IDA 0xc98c34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c98c34() {
}


// 0xc98cb4 — __ZN4Ogre7FrustumC1ERKSs
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, const std::string *)
#[doc(alias = "Ogre::Frustum::Frustum(std::string const&)")]
// was: __ZN4Ogre7FrustumC1ERKSs
// IDA 0xc98cb4: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c98cb4() {
}


// 0xc98cc0 — __ZN4Ogre7FrustumC2ERKSs
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, const std::string *)
#[doc(alias = "Ogre::Frustum::Frustum(std::string const&)")]
// was: __ZN4Ogre7FrustumC2ERKSs
// IDA 0xc98cc0: 617 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c98cc0() {
}


// 0xc99398 — __ZN4Ogre7FrustumD0Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::~Frustum()")]
// was: __ZN4Ogre7FrustumD0Ev
// IDA 0xc99398: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c99398() {
}


// 0xc99428 — __ZN4Ogre7FrustumD1Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::~Frustum()")]
// was: __ZN4Ogre7FrustumD1Ev
// IDA 0xc99428: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c99428() {
}


// 0xc99434 — __ZThn4_N4Ogre7FrustumD0Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
// was: __ZThn4_N4Ogre7FrustumD0Ev
// IDA 0xc99434: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c99434() {
}


// 0xc994c8 — __ZThn188_N4Ogre7FrustumD0Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
// was: __ZThn188_N4Ogre7FrustumD0Ev
// IDA 0xc994c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c994c8() {
}


// 0xc9955c — __ZN4Ogre7FrustumD2Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::~Frustum()")]
// was: __ZN4Ogre7FrustumD2Ev
// IDA 0xc9955c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c9955c() {
}


// 0xc996e0 — __ZThn4_N4Ogre7FrustumD1Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
// was: __ZThn4_N4Ogre7FrustumD1Ev
// IDA 0xc996e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c996e0() {
}


// 0xc996ec — __ZThn188_N4Ogre7FrustumD1Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
// was: __ZThn188_N4Ogre7FrustumD1Ev
// IDA 0xc996ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c996ec() {
}


// 0xc996f8 — __ZN4Ogre7Frustum7setFOVyERKNS_6RadianE
#[doc(alias = "Ogre::Frustum::setFOVy(Ogre::Radian const&)")]
// was: __ZN4Ogre7Frustum7setFOVyERKNS_6RadianE
// IDA 0xc996f8: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c996f8() {
}


// 0xc9970c — __ZNK4Ogre7Frustum7getFOVyEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getFOVy(void)const")]
// was: __ZNK4Ogre7Frustum7getFOVyEv
// IDA 0xc9970c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9970c() {
}


// 0xc99710 — __ZN4Ogre7Frustum18setFarClipDistanceEf
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float)
#[doc(alias = "Ogre::Frustum::setFarClipDistance(float)")]
// was: __ZN4Ogre7Frustum18setFarClipDistanceEf
// IDA 0xc99710: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99710() {
}


// 0xc99724 — __ZNK4Ogre7Frustum18getFarClipDistanceEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getFarClipDistance(void)const")]
// was: __ZNK4Ogre7Frustum18getFarClipDistanceEv
// IDA 0xc99724: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99724() {
}


// 0xc9972c — __ZN4Ogre7Frustum19setNearClipDistanceEf
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float)
#[doc(alias = "Ogre::Frustum::setNearClipDistance(float)")]
// was: __ZN4Ogre7Frustum19setNearClipDistanceEf
// IDA 0xc9972c: 164 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9972c() {
}


// 0xc9991c — __ZNK4Ogre7Frustum19getNearClipDistanceEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getNearClipDistance(void)const")]
// was: __ZNK4Ogre7Frustum19getNearClipDistanceEv
// IDA 0xc9991c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9991c() {
}


// 0xc99924 — __ZN4Ogre7Frustum16setFrustumOffsetERKNS_7Vector2E
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, const Vector2 *)
#[doc(alias = "Ogre::Frustum::setFrustumOffset(Ogre::Vector2 const&)")]
// was: __ZN4Ogre7Frustum16setFrustumOffsetERKNS_7Vector2E
// IDA 0xc99924: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99924() {
}


// 0xc99940 — __ZN4Ogre7Frustum16setFrustumOffsetEff
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float, float)
#[doc(alias = "Ogre::Frustum::setFrustumOffset(float,float)")]
// was: __ZN4Ogre7Frustum16setFrustumOffsetEff
// IDA 0xc99940: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99940() {
}


// 0xc99958 — __ZNK4Ogre7Frustum16getFrustumOffsetEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getFrustumOffset(void)const")]
// was: __ZNK4Ogre7Frustum16getFrustumOffsetEv
// IDA 0xc99958: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99958() {
}


// 0xc99960 — __ZN4Ogre7Frustum14setFocalLengthEf
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float)
#[doc(alias = "Ogre::Frustum::setFocalLength(float)")]
// was: __ZN4Ogre7Frustum14setFocalLengthEf
// IDA 0xc99960: 164 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99960() {
}


// 0xc99b50 — __ZNK4Ogre7Frustum14getFocalLengthEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getFocalLength(void)const")]
// was: __ZNK4Ogre7Frustum14getFocalLengthEv
// IDA 0xc99b50: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99b50() {
}


// 0xc99b58 — __ZNK4Ogre7Frustum19getProjectionMatrixEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getProjectionMatrix(void)const")]
// was: __ZNK4Ogre7Frustum19getProjectionMatrixEv
// IDA 0xc99b58: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99b58() {
}


// 0xc99b70 — __ZNK4Ogre7Frustum30getProjectionMatrixWithRSDepthEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getProjectionMatrixWithRSDepth(void)const")]
// was: __ZNK4Ogre7Frustum30getProjectionMatrixWithRSDepthEv
// IDA 0xc99b70: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99b70() {
}


// 0xc99b88 — __ZNK4Ogre7Frustum21getProjectionMatrixRSEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getProjectionMatrixRS(void)const")]
// was: __ZNK4Ogre7Frustum21getProjectionMatrixRSEv
// IDA 0xc99b88: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99b88() {
}


// 0xc99bb8 — __ZNK4Ogre7Frustum16getFrustumPlanesEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getFrustumPlanes(void)const")]
// was: __ZNK4Ogre7Frustum16getFrustumPlanesEv
// IDA 0xc99bb8: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99bb8() {
}


// 0xc99bd0 — __ZNK4Ogre7Frustum15getFrustumPlaneEt
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::Frustum::getFrustumPlane(unsigned short)const")]
// was: __ZNK4Ogre7Frustum15getFrustumPlaneEt
// IDA 0xc99bd0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99bd0() {
}


// 0xc99bec — __ZNK4Ogre7Frustum9isVisibleERKNS_14AxisAlignedBoxEPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum::isVisible(Ogre::AxisAlignedBox const&,Ogre::FrustumPlane *)const")]
// was: __ZNK4Ogre7Frustum9isVisibleERKNS_14AxisAlignedBoxEPNS_12FrustumPlaneE
// IDA 0xc99bec: 102 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99bec() {
}


// 0xc99d28 — __ZNK4Ogre7Frustum9isVisibleERKNS_7Vector3EPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum::isVisible(Ogre::Vector3 const&,Ogre::FrustumPlane *)const")]
// was: __ZNK4Ogre7Frustum9isVisibleERKNS_7Vector3EPNS_12FrustumPlaneE
// IDA 0xc99d28: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99d28() {
}


// 0xc99d90 — __ZNK4Ogre7Frustum9isVisibleERKNS_6SphereEPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum::isVisible(Ogre::Sphere const&,Ogre::FrustumPlane *)const")]
// was: __ZNK4Ogre7Frustum9isVisibleERKNS_6SphereEPNS_12FrustumPlaneE
// IDA 0xc99d90: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99d90() {
}


// 0xc99e0c — __ZNK4Ogre7Frustum12getTypeFlagsEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getTypeFlags(void)const")]
// was: __ZNK4Ogre7Frustum12getTypeFlagsEv
// IDA 0xc99e0c: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99e0c() {
}


// 0xc99e1c — __ZNK4Ogre7Frustum24calcProjectionParametersERfS1_S1_S1_
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float *, float *, float *, float *)
#[doc(alias = "Ogre::Frustum::calcProjectionParameters(float &,float &,float &,float &)const")]
// was: __ZNK4Ogre7Frustum24calcProjectionParametersERfS1_S1_S1_
// IDA 0xc99e1c: 157 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c99e1c() {
}


// 0xc9a054 — __ZNK4Ogre7Frustum17updateFrustumImplEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateFrustumImpl(void)const")]
// was: __ZNK4Ogre7Frustum17updateFrustumImplEv
// IDA 0xc9a054: 394 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9a054() {
}


// 0xc9a5c0 — __ZNK4Ogre7Frustum13updateFrustumEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateFrustum(void)const")]
// was: __ZNK4Ogre7Frustum13updateFrustumEv
// IDA 0xc9a5c0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9a5c0() {
}


// 0xc9a5e4 — __ZNK4Ogre7Frustum16updateVertexDataEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateVertexData(void)const")]
// was: __ZNK4Ogre7Frustum16updateVertexDataEv
// IDA 0xc9a5e4: 512 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9a5e4() {
}


// 0xc9ad5c — __ZNK4Ogre7Frustum18isFrustumOutOfDateEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::isFrustumOutOfDate(void)const")]
// was: __ZNK4Ogre7Frustum18isFrustumOutOfDateEv
// IDA 0xc9ad5c: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9ad5c() {
}


// 0xc9af84 — __ZNK4Ogre7Frustum23updateFrustumPlanesImplEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateFrustumPlanesImpl(void)const")]
// was: __ZNK4Ogre7Frustum23updateFrustumPlanesImplEv
// IDA 0xc9af84: 110 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9af84() {
}


// 0xc9b11c — __ZNK4Ogre7Frustum19updateFrustumPlanesEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateFrustumPlanes(void)const")]
// was: __ZNK4Ogre7Frustum19updateFrustumPlanesEv
// IDA 0xc9b11c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b11c() {
}


// 0xc9b14c — __ZNK4Ogre7Frustum27updateWorldSpaceCornersImplEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateWorldSpaceCornersImpl(void)const")]
// was: __ZNK4Ogre7Frustum27updateWorldSpaceCornersImplEv
// IDA 0xc9b14c: 197 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b14c() {
}


// 0xc9b438 — __ZNK4Ogre7Frustum23updateWorldSpaceCornersEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateWorldSpaceCorners(void)const")]
// was: __ZNK4Ogre7Frustum23updateWorldSpaceCornersEv
// IDA 0xc9b438: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b438() {
}


// 0xc9b460 — __ZNK4Ogre7Frustum14getAspectRatioEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getAspectRatio(void)const")]
// was: __ZNK4Ogre7Frustum14getAspectRatioEv
// IDA 0xc9b460: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b460() {
}


// 0xc9b468 — __ZN4Ogre7Frustum14setAspectRatioEf
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float)
#[doc(alias = "Ogre::Frustum::setAspectRatio(float)")]
// was: __ZN4Ogre7Frustum14setAspectRatioEf
// IDA 0xc9b468: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b468() {
}


// 0xc9b47c — __ZNK4Ogre7Frustum14getBoundingBoxEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getBoundingBox(void)const")]
// was: __ZNK4Ogre7Frustum14getBoundingBoxEv
// IDA 0xc9b47c: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b47c() {
}


// 0xc9b484 — __ZN4Ogre7Frustum18_updateRenderQueueEPNS_11RenderQueueE
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, Ogre::RenderQueue *)
#[doc(alias = "Ogre::Frustum::_updateRenderQueue(Ogre::RenderQueue *)")]
// was: __ZN4Ogre7Frustum18_updateRenderQueueEPNS_11RenderQueueE
// IDA 0xc9b484: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b484() {
}


// 0xc9b4a0 — __ZNK4Ogre7Frustum14getMovableTypeEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getMovableType(void)const")]
// was: __ZNK4Ogre7Frustum14getMovableTypeEv
// IDA 0xc9b4a0: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b4a0() {
}


// 0xc9b4ac — __ZNK4Ogre7Frustum17getBoundingRadiusEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getBoundingRadius(void)const")]
// was: __ZNK4Ogre7Frustum17getBoundingRadiusEv
// IDA 0xc9b4ac: 8 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b4ac() {
}


// 0xc9b4cc — __ZNK4Ogre7Frustum11getMaterialEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getMaterial(void)const")]
// was: __ZNK4Ogre7Frustum11getMaterialEv
// IDA 0xc9b4cc: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b4cc() {
}


// 0xc9b4d4 — __ZThn188_NK4Ogre7Frustum11getMaterialEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::Frustum::getMaterial(void)const")]
// was: __ZThn188_NK4Ogre7Frustum11getMaterialEv
// IDA 0xc9b4d4: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b4d4() {
}


// 0xc9b4dc — __ZN4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::Frustum::getRenderOperation(Ogre::RenderOperation &)")]
// was: __ZN4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE
// IDA 0xc9b4dc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b4dc() {
}


// 0xc9b500 — __ZThn188_N4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "non-virtual thunk toOgre::Frustum::getRenderOperation(Ogre::RenderOperation &)")]
// was: __ZThn188_N4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE
// IDA 0xc9b500: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b500() {
}


// 0xc9b524 — __ZNK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::Frustum::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: __ZNK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E
// IDA 0xc9b524: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b524() {
}


// 0xc9b57c — __ZThn188_NK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "non-virtual thunk toOgre::Frustum::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: __ZThn188_NK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E
// IDA 0xc9b57c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b57c() {
}


// 0xc9b638 — __ZThn188_NK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, const Ogre::Camera *)
#[doc(alias = "non-virtual thunk toOgre::Frustum::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: __ZThn188_NK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE
// IDA 0xc9b638: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9b638() {
}
