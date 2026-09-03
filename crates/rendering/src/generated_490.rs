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
pub fn stub_c93830() -> ! {
    todo!("0xc93830 std::_Vector_base<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}


// 0xc93834 — __ZNSt12_Vector_baseISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
pub fn stub_c93834() -> ! {
    todo!("0xc93834 std::_Vector_base<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}


// 0xc93840 — __ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// was: __ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
pub fn stub_c93840() -> ! {
    todo!("0xc93840 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")
}


// 0xc938e8 — __ZN4Ogre9SharedPtrINS_7TextureEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::SharedPtr<Ogre::Texture>::operator=(Ogre::SharedPtr<Ogre::Texture> const&)")]
// was: __ZN4Ogre9SharedPtrINS_7TextureEEaSERKS2_
pub fn stub_c938e8() -> ! {
    todo!("0xc938e8 Ogre::SharedPtr<Ogre::Texture>::operator=(Ogre::SharedPtr<Ogre::Texture> const&)")
}


// 0xc93a68 — __ZN4Ogre9SharedPtrINS_7TextureEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Texture>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_7TextureEED0Ev
pub fn stub_c93a68() -> ! {
    todo!("0xc93a68 Ogre::SharedPtr<Ogre::Texture>::~SharedPtr()")
}


// 0xc93b60 — __ZN4Ogre22InternalErrorExceptionD0Ev
// type: void __fastcall(Ogre::InternalErrorException *__hidden this)
#[doc(alias = "Ogre::InternalErrorException::~InternalErrorException()")]
// was: __ZN4Ogre22InternalErrorExceptionD0Ev
pub fn stub_c93b60() -> ! {
    todo!("0xc93b60 Ogre::InternalErrorException::~InternalErrorException()")
}


// 0xc93b78 — __ZN4Ogre9SharedPtrINS_8MaterialEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_8MaterialEED0Ev
pub fn stub_c93b78() -> ! {
    todo!("0xc93b78 Ogre::SharedPtr<Ogre::Material>::~SharedPtr()")
}


// 0xc93c70 — __ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::ParameterDef const&)")]
// was: __ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
pub fn stub_c93c70() -> ! {
    todo!("0xc93c70 std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::ParameterDef const&)")
}


// 0xc93d90 — __ZNSt3mapISsPN4Ogre12ParamCommandESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::ParamCommand *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: __ZNSt3mapISsPN4Ogre12ParamCommandESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
pub fn stub_c93d90() -> ! {
    todo!("0xc93d90 std::map<std::string,Ogre::ParamCommand *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")
}


// 0xc93f50 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamCommand *> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
pub fn stub_c93f50() -> ! {
    todo!("0xc93f50 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamCommand *> const&)")
}


// 0xc94038 — __ZSt22__uninitialized_copy_aIPN4Ogre12ParameterDefES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "Ogre::ParameterDef * std::__uninitialized_copy_a<Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: __ZSt22__uninitialized_copy_aIPN4Ogre12ParameterDefES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
pub fn stub_c94038() -> ! {
    todo!("0xc94038 Ogre::ParameterDef * std::__uninitialized_copy_a<Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")
}


// 0xc94238 — __ZN4Ogre12STLAllocatorINS_12ParameterDefENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: __ZN4Ogre12STLAllocatorINS_12ParameterDefENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
pub fn stub_c94238() -> ! {
    todo!("0xc94238 Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")
}


// 0xc94248 — __ZNSt12_Vector_baseIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
pub fn stub_c94248() -> ! {
    todo!("0xc94248 std::_Vector_base<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}


// 0xc94250 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
pub fn stub_c94250() -> ! {
    todo!("0xc94250 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")
}


// 0xc94260 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSH_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> const*,std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>>*)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSH_
pub fn stub_c94260() -> ! {
    todo!("0xc94260 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> const*,std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>>*)")
}


// 0xc94450 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamDictionary> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
pub fn stub_c94450() -> ! {
    todo!("0xc94450 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamDictionary> const&)")
}


// 0xc94538 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::ParamDictionary> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_
pub fn stub_c94538() -> ! {
    todo!("0xc94538 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::ParamDictionary> const&)")
}


// 0xc947c0 — __ZN4Ogre11FontManager12getSingletonEv
// type: _DWORD __fastcall(Ogre::FontManager *__hidden this)
#[doc(alias = "Ogre::FontManager::getSingleton(void)")]
// was: __ZN4Ogre11FontManager12getSingletonEv
pub fn stub_c947c0() -> ! {
    todo!("0xc947c0 Ogre::FontManager::getSingleton(void)")
}


// 0xc947d0 — __ZN4Ogre11FontManagerC1Ev
// type: _DWORD __fastcall(Ogre::FontManager *__hidden this)
#[doc(alias = "Ogre::FontManager::FontManager(void)")]
// was: __ZN4Ogre11FontManagerC1Ev
pub fn stub_c947d0() -> ! {
    todo!("0xc947d0 Ogre::FontManager::FontManager(void)")
}


// 0xc947dc — __ZN4Ogre11FontManagerC2Ev
// type: _DWORD __fastcall(Ogre::FontManager *__hidden this)
#[doc(alias = "Ogre::FontManager::FontManager(void)")]
// was: __ZN4Ogre11FontManagerC2Ev
pub fn stub_c947dc() -> ! {
    todo!("0xc947dc Ogre::FontManager::FontManager(void)")
}


// 0xc949c4 — __ZN4Ogre11FontManagerD0Ev
// type: void __fastcall(Ogre::FontManager *__hidden this)
#[doc(alias = "Ogre::FontManager::~FontManager()")]
// was: __ZN4Ogre11FontManagerD0Ev
pub fn stub_c949c4() -> ! {
    todo!("0xc949c4 Ogre::FontManager::~FontManager()")
}


// 0xc94abc — __ZN4Ogre11FontManagerD1Ev
// type: void __fastcall(Ogre::FontManager *__hidden this)
#[doc(alias = "Ogre::FontManager::~FontManager()")]
// was: __ZN4Ogre11FontManagerD1Ev
pub fn stub_c94abc() -> ! {
    todo!("0xc94abc Ogre::FontManager::~FontManager()")
}


// 0xc94ba4 — __ZN4Ogre11FontManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, Ogre::ManualResourceLoader *, int, int, int)
#[doc(alias = "Ogre::FontManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: __ZN4Ogre11FontManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
pub fn stub_c94ba4() -> ! {
    todo!("0xc94ba4 Ogre::FontManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")
}


// 0xc95368 — __ZN4Ogre11FontManager14parseAttributeERKSsRNS_7FontPtrE
// type: _DWORD __fastcall(Ogre::FontManager *__hidden this, const std::string *, Ogre::FontPtr *)
#[doc(alias = "Ogre::FontManager::parseAttribute(std::string const&,Ogre::FontPtr &)")]
// was: __ZN4Ogre11FontManager14parseAttributeERKSsRNS_7FontPtrE
pub fn stub_c95368() -> ! {
    todo!("0xc95368 Ogre::FontManager::parseAttribute(std::string const&,Ogre::FontPtr &)")
}


// 0xc95cfc — __ZN4Ogre11FontManager12logBadAttribERKSsRNS_7FontPtrE
#[doc(alias = "Ogre::FontManager::logBadAttrib(std::string const&,Ogre::FontPtr &)")]
// was: __ZN4Ogre11FontManager12logBadAttribERKSsRNS_7FontPtrE
pub fn stub_c95cfc() -> ! {
    todo!("0xc95cfc Ogre::FontManager::logBadAttrib(std::string const&,Ogre::FontPtr &)")
}


// 0xc96014 — __ZN4Ogre7FontPtrD1Ev
// type: void __fastcall(Ogre::FontPtr *__hidden this)
#[doc(alias = "Ogre::FontPtr::~FontPtr()")]
// was: __ZN4Ogre7FontPtrD1Ev
pub fn stub_c96014() -> ! {
    todo!("0xc96014 Ogre::FontPtr::~FontPtr()")
}


// 0xc96108 — __ZNK4Ogre15ResourceManager15getLoadingOrderEv
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this)
#[doc(alias = "Ogre::ResourceManager::getLoadingOrder(void)const")]
// was: __ZNK4Ogre15ResourceManager15getLoadingOrderEv
pub fn stub_c96108() -> ! {
    todo!("0xc96108 Ogre::ResourceManager::getLoadingOrder(void)const")
}


// 0xc9610c — __ZNK4Ogre15ResourceManager14getMemoryUsageEv
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this)
#[doc(alias = "Ogre::ResourceManager::getMemoryUsage(void)const")]
// was: __ZNK4Ogre15ResourceManager14getMemoryUsageEv
pub fn stub_c9610c() -> ! {
    todo!("0xc9610c Ogre::ResourceManager::getMemoryUsage(void)const")
}


// 0xc96110 — __ZN4Ogre15ResourceManager14resourceExistsERKSs
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::ResourceManager::resourceExists(std::string const&)")]
// was: __ZN4Ogre15ResourceManager14resourceExistsERKSs
pub fn stub_c96110() -> ! {
    todo!("0xc96110 Ogre::ResourceManager::resourceExists(std::string const&)")
}


// 0xc96220 — __ZN4Ogre15ResourceManager14resourceExistsEy
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, unsigned __int64)
#[doc(alias = "Ogre::ResourceManager::resourceExists(unsigned long long)")]
// was: __ZN4Ogre15ResourceManager14resourceExistsEy
pub fn stub_c96220() -> ! {
    todo!("0xc96220 Ogre::ResourceManager::resourceExists(unsigned long long)")
}


// 0xc96324 — __ZN4Ogre15ResourceManager10setVerboseEb
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, bool)
#[doc(alias = "Ogre::ResourceManager::setVerbose(bool)")]
// was: __ZN4Ogre15ResourceManager10setVerboseEb
pub fn stub_c96324() -> ! {
    todo!("0xc96324 Ogre::ResourceManager::setVerbose(bool)")
}


// 0xc9632c — __ZN4Ogre15ResourceManager10getVerboseEv
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this)
#[doc(alias = "Ogre::ResourceManager::getVerbose(void)")]
// was: __ZN4Ogre15ResourceManager10getVerboseEv
pub fn stub_c9632c() -> ! {
    todo!("0xc9632c Ogre::ResourceManager::getVerbose(void)")
}


// 0xc96338 — __ZSt22__uninitialized_copy_aIPSsS0_N4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEET0_T_S8_S7_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::string * std::__uninitialized_copy_a<std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::string *,std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: __ZSt22__uninitialized_copy_aIPSsS0_N4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEET0_T_S8_S7_T1_
pub fn stub_c96338() -> ! {
    todo!("0xc96338 std::string * std::__uninitialized_copy_a<std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::string *,std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")
}


// 0xc96488 — __ZNSt6vectorISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S8_EERKS1_
#[doc(alias = "std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<unsigned int,unsigned int>*,std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::pair<unsigned int,unsigned int> const&)")]
// was: __ZNSt6vectorISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S8_EERKS1_
pub fn stub_c96488() -> ! {
    todo!("0xc96488 std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<unsigned int,unsigned int>*,std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::pair<unsigned int,unsigned int> const&)")
}


// 0xc96a30 — __ZN4Ogre9SharedPtrINS_4FontEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_4FontEED1Ev
pub fn stub_c96a30() -> ! {
    todo!("0xc96a30 Ogre::SharedPtr<Ogre::Font>::~SharedPtr()")
}


// 0xc96ae0 — __ZN4Ogre9SharedPtrINS_4FontEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_4FontEED0Ev
pub fn stub_c96ae0() -> ! {
    todo!("0xc96ae0 Ogre::SharedPtr<Ogre::Font>::~SharedPtr()")
}


// 0xc96bd4 — __ZN4Ogre9SharedPtrINS_4FontEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::destroy(void)")]
// was: __ZN4Ogre9SharedPtrINS_4FontEE7destroyEv
pub fn stub_c96bd4() -> ! {
    todo!("0xc96bd4 Ogre::SharedPtr<Ogre::Font>::destroy(void)")
}


// 0xc96c0c — __ZN4Ogre9SharedPtrINS_4FontEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Font>::swap(Ogre::SharedPtr<Ogre::Font>&)")]
// was: __ZN4Ogre9SharedPtrINS_4FontEE4swapERS2_
pub fn stub_c96c0c() -> ! {
    todo!("0xc96c0c Ogre::SharedPtr<Ogre::Font>::swap(Ogre::SharedPtr<Ogre::Font>&)")
}


// 0xc96c28 — __ZN4Ogre7FontPtrD0Ev
// type: void __fastcall(Ogre::FontPtr *__hidden this)
#[doc(alias = "Ogre::FontPtr::~FontPtr()")]
// was: __ZN4Ogre7FontPtrD0Ev
pub fn stub_c96c28() -> ! {
    todo!("0xc96c28 Ogre::FontPtr::~FontPtr()")
}


// 0xc96d50 — __ZN4Ogre25FreeImageLoadErrorHandlerE17FREE_IMAGE_FORMATPKc
#[doc(alias = "Ogre::FreeImageLoadErrorHandler(FREE_IMAGE_FORMAT,char const*)")]
// was: __ZN4Ogre25FreeImageLoadErrorHandlerE17FREE_IMAGE_FORMATPKc
pub fn stub_c96d50() -> ! {
    todo!("0xc96d50 Ogre::FreeImageLoadErrorHandler(FREE_IMAGE_FORMAT,char const*)")
}


// 0xc96f14 — __ZN4Ogre14FreeImageCodec7startupEv
// type: _DWORD __fastcall(Ogre::FreeImageCodec *__hidden this)
#[doc(alias = "Ogre::FreeImageCodec::startup(void)")]
// was: __ZN4Ogre14FreeImageCodec7startupEv
pub fn stub_c96f14() -> ! {
    todo!("0xc96f14 Ogre::FreeImageCodec::startup(void)")
}


// 0xc9778c — __ZN4Ogre14FreeImageCodec8shutdownEv
// type: _DWORD __fastcall(Ogre::FreeImageCodec *__hidden this)
#[doc(alias = "Ogre::FreeImageCodec::shutdown(void)")]
// was: __ZN4Ogre14FreeImageCodec8shutdownEv
pub fn stub_c9778c() -> ! {
    todo!("0xc9778c Ogre::FreeImageCodec::shutdown(void)")
}


// 0xc988c4 — __ZNK4Ogre14FreeImageCodec7getTypeEv
// type: _DWORD __fastcall(Ogre::FreeImageCodec *__hidden this)
#[doc(alias = "Ogre::FreeImageCodec::getType(void)const")]
// was: __ZNK4Ogre14FreeImageCodec7getTypeEv
pub fn stub_c988c4() -> ! {
    todo!("0xc988c4 Ogre::FreeImageCodec::getType(void)const")
}


// 0xc988d0 — __ZNK4Ogre14FreeImageCodec20magicNumberToFileExtEPKcm
// type: _DWORD __fastcall(Ogre::FreeImageCodec *__hidden this, const char *, unsigned int)
#[doc(alias = "Ogre::FreeImageCodec::magicNumberToFileExt(char const*,unsigned long)const")]
// was: __ZNK4Ogre14FreeImageCodec20magicNumberToFileExtEPKcm
pub fn stub_c988d0() -> ! {
    todo!("0xc988d0 Ogre::FreeImageCodec::magicNumberToFileExt(char const*,unsigned long)const")
}


// 0xc98a48 — __ZNSt4listIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(alias = "std::list<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~list()")]
// was: __ZNSt4listIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
pub fn stub_c98a48() -> ! {
    todo!("0xc98a48 std::list<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~list()")
}


// 0xc98af0 — __ZN4Ogre14FreeImageCodecD1Ev
// type: void __fastcall(Ogre::FreeImageCodec *__hidden this)
#[doc(alias = "Ogre::FreeImageCodec::~FreeImageCodec()")]
// was: __ZN4Ogre14FreeImageCodecD1Ev
pub fn stub_c98af0() -> ! {
    todo!("0xc98af0 Ogre::FreeImageCodec::~FreeImageCodec()")
}


// 0xc98b50 — __ZN4Ogre14FreeImageCodecD0Ev
// type: void __fastcall(Ogre::FreeImageCodec *__hidden this)
#[doc(alias = "Ogre::FreeImageCodec::~FreeImageCodec()")]
// was: __ZN4Ogre14FreeImageCodecD0Ev
pub fn stub_c98b50() -> ! {
    todo!("0xc98b50 Ogre::FreeImageCodec::~FreeImageCodec()")
}


// 0xc98c30 — __ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: __ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
pub fn stub_c98c30() -> ! {
    todo!("0xc98c30 std::_List_base<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")
}


// 0xc98c34 — __ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: __ZNSt10_List_baseIPN4Ogre10ImageCodecENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
pub fn stub_c98c34() -> ! {
    todo!("0xc98c34 std::_List_base<Ogre::ImageCodec *,Ogre::STLAllocator<Ogre::ImageCodec *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")
}


// 0xc98cb4 — __ZN4Ogre7FrustumC1ERKSs
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, const std::string *)
#[doc(alias = "Ogre::Frustum::Frustum(std::string const&)")]
// was: __ZN4Ogre7FrustumC1ERKSs
pub fn stub_c98cb4() -> ! {
    todo!("0xc98cb4 Ogre::Frustum::Frustum(std::string const&)")
}


// 0xc98cc0 — __ZN4Ogre7FrustumC2ERKSs
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, const std::string *)
#[doc(alias = "Ogre::Frustum::Frustum(std::string const&)")]
// was: __ZN4Ogre7FrustumC2ERKSs
pub fn stub_c98cc0() -> ! {
    todo!("0xc98cc0 Ogre::Frustum::Frustum(std::string const&)")
}


// 0xc99398 — __ZN4Ogre7FrustumD0Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::~Frustum()")]
// was: __ZN4Ogre7FrustumD0Ev
pub fn stub_c99398() -> ! {
    todo!("0xc99398 Ogre::Frustum::~Frustum()")
}


// 0xc99428 — __ZN4Ogre7FrustumD1Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::~Frustum()")]
// was: __ZN4Ogre7FrustumD1Ev
pub fn stub_c99428() -> ! {
    todo!("0xc99428 Ogre::Frustum::~Frustum()")
}


// 0xc99434 — __ZThn4_N4Ogre7FrustumD0Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
// was: __ZThn4_N4Ogre7FrustumD0Ev
pub fn stub_c99434() -> ! {
    todo!("0xc99434 `non-virtual thunk to'Ogre::Frustum::~Frustum()")
}


// 0xc994c8 — __ZThn188_N4Ogre7FrustumD0Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
// was: __ZThn188_N4Ogre7FrustumD0Ev
pub fn stub_c994c8() -> ! {
    todo!("0xc994c8 `non-virtual thunk to'Ogre::Frustum::~Frustum()")
}


// 0xc9955c — __ZN4Ogre7FrustumD2Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::~Frustum()")]
// was: __ZN4Ogre7FrustumD2Ev
pub fn stub_c9955c() -> ! {
    todo!("0xc9955c Ogre::Frustum::~Frustum()")
}


// 0xc996e0 — __ZThn4_N4Ogre7FrustumD1Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
// was: __ZThn4_N4Ogre7FrustumD1Ev
pub fn stub_c996e0() -> ! {
    todo!("0xc996e0 `non-virtual thunk to'Ogre::Frustum::~Frustum()")
}


// 0xc996ec — __ZThn188_N4Ogre7FrustumD1Ev
// type: void __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::Frustum::~Frustum()")]
// was: __ZThn188_N4Ogre7FrustumD1Ev
pub fn stub_c996ec() -> ! {
    todo!("0xc996ec `non-virtual thunk to'Ogre::Frustum::~Frustum()")
}


// 0xc996f8 — __ZN4Ogre7Frustum7setFOVyERKNS_6RadianE
#[doc(alias = "Ogre::Frustum::setFOVy(Ogre::Radian const&)")]
// was: __ZN4Ogre7Frustum7setFOVyERKNS_6RadianE
pub fn stub_c996f8() -> ! {
    todo!("0xc996f8 Ogre::Frustum::setFOVy(Ogre::Radian const&)")
}


// 0xc9970c — __ZNK4Ogre7Frustum7getFOVyEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getFOVy(void)const")]
// was: __ZNK4Ogre7Frustum7getFOVyEv
pub fn stub_c9970c() -> ! {
    todo!("0xc9970c Ogre::Frustum::getFOVy(void)const")
}


// 0xc99710 — __ZN4Ogre7Frustum18setFarClipDistanceEf
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float)
#[doc(alias = "Ogre::Frustum::setFarClipDistance(float)")]
// was: __ZN4Ogre7Frustum18setFarClipDistanceEf
pub fn stub_c99710() -> ! {
    todo!("0xc99710 Ogre::Frustum::setFarClipDistance(float)")
}


// 0xc99724 — __ZNK4Ogre7Frustum18getFarClipDistanceEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getFarClipDistance(void)const")]
// was: __ZNK4Ogre7Frustum18getFarClipDistanceEv
pub fn stub_c99724() -> ! {
    todo!("0xc99724 Ogre::Frustum::getFarClipDistance(void)const")
}


// 0xc9972c — __ZN4Ogre7Frustum19setNearClipDistanceEf
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float)
#[doc(alias = "Ogre::Frustum::setNearClipDistance(float)")]
// was: __ZN4Ogre7Frustum19setNearClipDistanceEf
pub fn stub_c9972c() -> ! {
    todo!("0xc9972c Ogre::Frustum::setNearClipDistance(float)")
}


// 0xc9991c — __ZNK4Ogre7Frustum19getNearClipDistanceEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getNearClipDistance(void)const")]
// was: __ZNK4Ogre7Frustum19getNearClipDistanceEv
pub fn stub_c9991c() -> ! {
    todo!("0xc9991c Ogre::Frustum::getNearClipDistance(void)const")
}


// 0xc99924 — __ZN4Ogre7Frustum16setFrustumOffsetERKNS_7Vector2E
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, const Vector2 *)
#[doc(alias = "Ogre::Frustum::setFrustumOffset(Ogre::Vector2 const&)")]
// was: __ZN4Ogre7Frustum16setFrustumOffsetERKNS_7Vector2E
pub fn stub_c99924() -> ! {
    todo!("0xc99924 Ogre::Frustum::setFrustumOffset(Ogre::Vector2 const&)")
}


// 0xc99940 — __ZN4Ogre7Frustum16setFrustumOffsetEff
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float, float)
#[doc(alias = "Ogre::Frustum::setFrustumOffset(float,float)")]
// was: __ZN4Ogre7Frustum16setFrustumOffsetEff
pub fn stub_c99940() -> ! {
    todo!("0xc99940 Ogre::Frustum::setFrustumOffset(float,float)")
}


// 0xc99958 — __ZNK4Ogre7Frustum16getFrustumOffsetEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getFrustumOffset(void)const")]
// was: __ZNK4Ogre7Frustum16getFrustumOffsetEv
pub fn stub_c99958() -> ! {
    todo!("0xc99958 Ogre::Frustum::getFrustumOffset(void)const")
}


// 0xc99960 — __ZN4Ogre7Frustum14setFocalLengthEf
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float)
#[doc(alias = "Ogre::Frustum::setFocalLength(float)")]
// was: __ZN4Ogre7Frustum14setFocalLengthEf
pub fn stub_c99960() -> ! {
    todo!("0xc99960 Ogre::Frustum::setFocalLength(float)")
}


// 0xc99b50 — __ZNK4Ogre7Frustum14getFocalLengthEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getFocalLength(void)const")]
// was: __ZNK4Ogre7Frustum14getFocalLengthEv
pub fn stub_c99b50() -> ! {
    todo!("0xc99b50 Ogre::Frustum::getFocalLength(void)const")
}


// 0xc99b58 — __ZNK4Ogre7Frustum19getProjectionMatrixEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getProjectionMatrix(void)const")]
// was: __ZNK4Ogre7Frustum19getProjectionMatrixEv
pub fn stub_c99b58() -> ! {
    todo!("0xc99b58 Ogre::Frustum::getProjectionMatrix(void)const")
}


// 0xc99b70 — __ZNK4Ogre7Frustum30getProjectionMatrixWithRSDepthEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getProjectionMatrixWithRSDepth(void)const")]
// was: __ZNK4Ogre7Frustum30getProjectionMatrixWithRSDepthEv
pub fn stub_c99b70() -> ! {
    todo!("0xc99b70 Ogre::Frustum::getProjectionMatrixWithRSDepth(void)const")
}


// 0xc99b88 — __ZNK4Ogre7Frustum21getProjectionMatrixRSEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getProjectionMatrixRS(void)const")]
// was: __ZNK4Ogre7Frustum21getProjectionMatrixRSEv
pub fn stub_c99b88() -> ! {
    todo!("0xc99b88 Ogre::Frustum::getProjectionMatrixRS(void)const")
}


// 0xc99bb8 — __ZNK4Ogre7Frustum16getFrustumPlanesEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getFrustumPlanes(void)const")]
// was: __ZNK4Ogre7Frustum16getFrustumPlanesEv
pub fn stub_c99bb8() -> ! {
    todo!("0xc99bb8 Ogre::Frustum::getFrustumPlanes(void)const")
}


// 0xc99bd0 — __ZNK4Ogre7Frustum15getFrustumPlaneEt
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::Frustum::getFrustumPlane(unsigned short)const")]
// was: __ZNK4Ogre7Frustum15getFrustumPlaneEt
pub fn stub_c99bd0() -> ! {
    todo!("0xc99bd0 Ogre::Frustum::getFrustumPlane(unsigned short)const")
}


// 0xc99bec — __ZNK4Ogre7Frustum9isVisibleERKNS_14AxisAlignedBoxEPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum::isVisible(Ogre::AxisAlignedBox const&,Ogre::FrustumPlane *)const")]
// was: __ZNK4Ogre7Frustum9isVisibleERKNS_14AxisAlignedBoxEPNS_12FrustumPlaneE
pub fn stub_c99bec() -> ! {
    todo!("0xc99bec Ogre::Frustum::isVisible(Ogre::AxisAlignedBox const&,Ogre::FrustumPlane *)const")
}


// 0xc99d28 — __ZNK4Ogre7Frustum9isVisibleERKNS_7Vector3EPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum::isVisible(Ogre::Vector3 const&,Ogre::FrustumPlane *)const")]
// was: __ZNK4Ogre7Frustum9isVisibleERKNS_7Vector3EPNS_12FrustumPlaneE
pub fn stub_c99d28() -> ! {
    todo!("0xc99d28 Ogre::Frustum::isVisible(Ogre::Vector3 const&,Ogre::FrustumPlane *)const")
}


// 0xc99d90 — __ZNK4Ogre7Frustum9isVisibleERKNS_6SphereEPNS_12FrustumPlaneE
#[doc(alias = "Ogre::Frustum::isVisible(Ogre::Sphere const&,Ogre::FrustumPlane *)const")]
// was: __ZNK4Ogre7Frustum9isVisibleERKNS_6SphereEPNS_12FrustumPlaneE
pub fn stub_c99d90() -> ! {
    todo!("0xc99d90 Ogre::Frustum::isVisible(Ogre::Sphere const&,Ogre::FrustumPlane *)const")
}


// 0xc99e0c — __ZNK4Ogre7Frustum12getTypeFlagsEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getTypeFlags(void)const")]
// was: __ZNK4Ogre7Frustum12getTypeFlagsEv
pub fn stub_c99e0c() -> ! {
    todo!("0xc99e0c Ogre::Frustum::getTypeFlags(void)const")
}


// 0xc99e1c — __ZNK4Ogre7Frustum24calcProjectionParametersERfS1_S1_S1_
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float *, float *, float *, float *)
#[doc(alias = "Ogre::Frustum::calcProjectionParameters(float &,float &,float &,float &)const")]
// was: __ZNK4Ogre7Frustum24calcProjectionParametersERfS1_S1_S1_
pub fn stub_c99e1c() -> ! {
    todo!("0xc99e1c Ogre::Frustum::calcProjectionParameters(float &,float &,float &,float &)const")
}


// 0xc9a054 — __ZNK4Ogre7Frustum17updateFrustumImplEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateFrustumImpl(void)const")]
// was: __ZNK4Ogre7Frustum17updateFrustumImplEv
pub fn stub_c9a054() -> ! {
    todo!("0xc9a054 Ogre::Frustum::updateFrustumImpl(void)const")
}


// 0xc9a5c0 — __ZNK4Ogre7Frustum13updateFrustumEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateFrustum(void)const")]
// was: __ZNK4Ogre7Frustum13updateFrustumEv
pub fn stub_c9a5c0() -> ! {
    todo!("0xc9a5c0 Ogre::Frustum::updateFrustum(void)const")
}


// 0xc9a5e4 — __ZNK4Ogre7Frustum16updateVertexDataEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateVertexData(void)const")]
// was: __ZNK4Ogre7Frustum16updateVertexDataEv
pub fn stub_c9a5e4() -> ! {
    todo!("0xc9a5e4 Ogre::Frustum::updateVertexData(void)const")
}


// 0xc9ad5c — __ZNK4Ogre7Frustum18isFrustumOutOfDateEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::isFrustumOutOfDate(void)const")]
// was: __ZNK4Ogre7Frustum18isFrustumOutOfDateEv
pub fn stub_c9ad5c() -> ! {
    todo!("0xc9ad5c Ogre::Frustum::isFrustumOutOfDate(void)const")
}


// 0xc9af84 — __ZNK4Ogre7Frustum23updateFrustumPlanesImplEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateFrustumPlanesImpl(void)const")]
// was: __ZNK4Ogre7Frustum23updateFrustumPlanesImplEv
pub fn stub_c9af84() -> ! {
    todo!("0xc9af84 Ogre::Frustum::updateFrustumPlanesImpl(void)const")
}


// 0xc9b11c — __ZNK4Ogre7Frustum19updateFrustumPlanesEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateFrustumPlanes(void)const")]
// was: __ZNK4Ogre7Frustum19updateFrustumPlanesEv
pub fn stub_c9b11c() -> ! {
    todo!("0xc9b11c Ogre::Frustum::updateFrustumPlanes(void)const")
}


// 0xc9b14c — __ZNK4Ogre7Frustum27updateWorldSpaceCornersImplEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateWorldSpaceCornersImpl(void)const")]
// was: __ZNK4Ogre7Frustum27updateWorldSpaceCornersImplEv
pub fn stub_c9b14c() -> ! {
    todo!("0xc9b14c Ogre::Frustum::updateWorldSpaceCornersImpl(void)const")
}


// 0xc9b438 — __ZNK4Ogre7Frustum23updateWorldSpaceCornersEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::updateWorldSpaceCorners(void)const")]
// was: __ZNK4Ogre7Frustum23updateWorldSpaceCornersEv
pub fn stub_c9b438() -> ! {
    todo!("0xc9b438 Ogre::Frustum::updateWorldSpaceCorners(void)const")
}


// 0xc9b460 — __ZNK4Ogre7Frustum14getAspectRatioEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getAspectRatio(void)const")]
// was: __ZNK4Ogre7Frustum14getAspectRatioEv
pub fn stub_c9b460() -> ! {
    todo!("0xc9b460 Ogre::Frustum::getAspectRatio(void)const")
}


// 0xc9b468 — __ZN4Ogre7Frustum14setAspectRatioEf
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float)
#[doc(alias = "Ogre::Frustum::setAspectRatio(float)")]
// was: __ZN4Ogre7Frustum14setAspectRatioEf
pub fn stub_c9b468() -> ! {
    todo!("0xc9b468 Ogre::Frustum::setAspectRatio(float)")
}


// 0xc9b47c — __ZNK4Ogre7Frustum14getBoundingBoxEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getBoundingBox(void)const")]
// was: __ZNK4Ogre7Frustum14getBoundingBoxEv
pub fn stub_c9b47c() -> ! {
    todo!("0xc9b47c Ogre::Frustum::getBoundingBox(void)const")
}


// 0xc9b484 — __ZN4Ogre7Frustum18_updateRenderQueueEPNS_11RenderQueueE
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, Ogre::RenderQueue *)
#[doc(alias = "Ogre::Frustum::_updateRenderQueue(Ogre::RenderQueue *)")]
// was: __ZN4Ogre7Frustum18_updateRenderQueueEPNS_11RenderQueueE
pub fn stub_c9b484() -> ! {
    todo!("0xc9b484 Ogre::Frustum::_updateRenderQueue(Ogre::RenderQueue *)")
}


// 0xc9b4a0 — __ZNK4Ogre7Frustum14getMovableTypeEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getMovableType(void)const")]
// was: __ZNK4Ogre7Frustum14getMovableTypeEv
pub fn stub_c9b4a0() -> ! {
    todo!("0xc9b4a0 Ogre::Frustum::getMovableType(void)const")
}


// 0xc9b4ac — __ZNK4Ogre7Frustum17getBoundingRadiusEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getBoundingRadius(void)const")]
// was: __ZNK4Ogre7Frustum17getBoundingRadiusEv
pub fn stub_c9b4ac() -> ! {
    todo!("0xc9b4ac Ogre::Frustum::getBoundingRadius(void)const")
}


// 0xc9b4cc — __ZNK4Ogre7Frustum11getMaterialEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "Ogre::Frustum::getMaterial(void)const")]
// was: __ZNK4Ogre7Frustum11getMaterialEv
pub fn stub_c9b4cc() -> ! {
    todo!("0xc9b4cc Ogre::Frustum::getMaterial(void)const")
}


// 0xc9b4d4 — __ZThn188_NK4Ogre7Frustum11getMaterialEv
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::Frustum::getMaterial(void)const")]
// was: __ZThn188_NK4Ogre7Frustum11getMaterialEv
pub fn stub_c9b4d4() -> ! {
    todo!("0xc9b4d4 `non-virtual thunk to'Ogre::Frustum::getMaterial(void)const")
}


// 0xc9b4dc — __ZN4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::Frustum::getRenderOperation(Ogre::RenderOperation &)")]
// was: __ZN4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE
pub fn stub_c9b4dc() -> ! {
    todo!("0xc9b4dc Ogre::Frustum::getRenderOperation(Ogre::RenderOperation &)")
}


// 0xc9b500 — __ZThn188_N4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "non-virtual thunk toOgre::Frustum::getRenderOperation(Ogre::RenderOperation &)")]
// was: __ZThn188_N4Ogre7Frustum18getRenderOperationERNS_15RenderOperationE
pub fn stub_c9b500() -> ! {
    todo!("0xc9b500 `non-virtual thunk to'Ogre::Frustum::getRenderOperation(Ogre::RenderOperation &)")
}


// 0xc9b524 — __ZNK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::Frustum::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: __ZNK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E
pub fn stub_c9b524() -> ! {
    todo!("0xc9b524 Ogre::Frustum::getWorldTransforms(Ogre::Matrix4 *)const")
}


// 0xc9b57c — __ZThn188_NK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "non-virtual thunk toOgre::Frustum::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: __ZThn188_NK4Ogre7Frustum18getWorldTransformsEPNS_7Matrix4E
pub fn stub_c9b57c() -> ! {
    todo!("0xc9b57c `non-virtual thunk to'Ogre::Frustum::getWorldTransforms(Ogre::Matrix4 *)const")
}


// 0xc9b638 — __ZThn188_NK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, const Ogre::Camera *)
#[doc(alias = "non-virtual thunk toOgre::Frustum::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: __ZThn188_NK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE
pub fn stub_c9b638() -> ! {
    todo!("0xc9b638 `non-virtual thunk to'Ogre::Frustum::getSquaredViewDepth(Ogre::Camera const*)const")
}

