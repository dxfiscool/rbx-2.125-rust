//! watchdog_core_wd3 — 100 core stubs EA-sorted global-dedup
//! Source: ida/export.json (85545 funcs) filtered core namespace
//! (memory/signal/containers/string/allocator: boost/rbx::signals/SharedPtr/allocator priority,
//! then EA-sorted global gap-fill per watchdog_core_wd2 precedent), only EAs absent from fresh global stub set.
//! Format: //0xADDR + #[doc(alias=mangled)] + pub fn stub_0xADDR() { todo!("0xADDR") }
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr,
//! boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes/backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

//0xe62dd8 — __ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias="__ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
#[doc(alias="std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::RenderTarget * const&)")]
// type: int __fastcall(char *)
pub fn stub_0xe62dd8() { todo!("0xe62dd8 __ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_") }

//0xe62ed0 — __ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias="__ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev")]
#[doc(alias="std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::RenderTarget *>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe62ed0() { todo!("0xe62ed0 __ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev") }

//0xe62ed4 — __ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias="__ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev")]
#[doc(alias="std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::RenderTarget *>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe62ed4() { todo!("0xe62ed4 __ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev") }

//0xe6c4cc — __ZNSt3mapISsN4Ogre11MaterialPtrESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
#[doc(alias="__ZNSt3mapISsN4Ogre11MaterialPtrESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_")]
#[doc(alias="std::map<std::string,Ogre::MaterialPtr,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// type: 
pub fn stub_0xe6c4cc() { todo!("0xe6c4cc __ZNSt3mapISsN4Ogre11MaterialPtrESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_") }

//0xe6c794 — __ZNSt6vectorIhN4Ogre12STLAllocatorIhNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPhS6_EEmRKh
#[doc(alias="__ZNSt6vectorIhN4Ogre12STLAllocatorIhNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPhS6_EEmRKh")]
#[doc(alias="std::vector<unsigned char,Ogre::STLAllocator<unsigned char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned char *,std::vector<unsigned char,Ogre::STLAllocator<unsigned char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,unsigned char const&)")]
// type: int __fastcall(int, void *__b)
pub fn stub_0xe6c794() { todo!("0xe6c794 __ZNSt6vectorIhN4Ogre12STLAllocatorIhNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPhS6_EEmRKh") }

//0xe6c908 — __ZNSt12_Vector_baseIhN4Ogre12STLAllocatorIhNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias="__ZNSt12_Vector_baseIhN4Ogre12STLAllocatorIhNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
#[doc(alias="std::_Vector_base<unsigned char,Ogre::STLAllocator<unsigned char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xe6c908() { todo!("0xe6c908 __ZNSt12_Vector_baseIhN4Ogre12STLAllocatorIhNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev") }

//0xe6c90c — __ZNSt12_Vector_baseIhN4Ogre12STLAllocatorIhNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias="__ZNSt12_Vector_baseIhN4Ogre12STLAllocatorIhNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
#[doc(alias="std::_Vector_base<unsigned char,Ogre::STLAllocator<unsigned char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xe6c90c() { todo!("0xe6c90c __ZNSt12_Vector_baseIhN4Ogre12STLAllocatorIhNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev") }

//0xe6c918 — __ZNSt8_Rb_treeIPN4Ogre7Matrix4ESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias="__ZNSt8_Rb_treeIPN4Ogre7Matrix4ESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
#[doc(alias="std::_Rb_tree<Ogre::Matrix4 *,std::pair<Ogre::Matrix4 * const,unsigned long>,std::_Select1st<std::pair<Ogre::Matrix4 * const,unsigned long>>,std::less<Ogre::Matrix4 *>,Ogre::STLAllocator<std::pair<Ogre::Matrix4 * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Matrix4 * const,unsigned long> const&)")]
// type: int __fastcall(char *)
pub fn stub_0xe6c918() { todo!("0xe6c918 __ZNSt8_Rb_treeIPN4Ogre7Matrix4ESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_") }

//0xe6ca14 — __ZNSt8_Rb_treeIPN4Ogre7Matrix4ESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias="__ZNSt8_Rb_treeIPN4Ogre7Matrix4ESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev")]
#[doc(alias="std::_Rb_tree<Ogre::Matrix4 *,std::pair<Ogre::Matrix4 * const,unsigned long>,std::_Select1st<std::pair<Ogre::Matrix4 * const,unsigned long>>,std::less<Ogre::Matrix4 *>,Ogre::STLAllocator<std::pair<Ogre::Matrix4 * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Matrix4 *>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe6ca14() { todo!("0xe6ca14 __ZNSt8_Rb_treeIPN4Ogre7Matrix4ESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev") }

//0xe6ca18 — __ZNSt8_Rb_treeIPN4Ogre7Matrix4ESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias="__ZNSt8_Rb_treeIPN4Ogre7Matrix4ESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev")]
#[doc(alias="std::_Rb_tree<Ogre::Matrix4 *,std::pair<Ogre::Matrix4 * const,unsigned long>,std::_Select1st<std::pair<Ogre::Matrix4 * const,unsigned long>>,std::less<Ogre::Matrix4 *>,Ogre::STLAllocator<std::pair<Ogre::Matrix4 * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Matrix4 *>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe6ca18() { todo!("0xe6ca18 __ZNSt8_Rb_treeIPN4Ogre7Matrix4ESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev") }

//0xe6ca24 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::MaterialPtr>>,std::pair<std::string const,Ogre::MaterialPtr> const&)")]
// type: int __fastcall(int)
pub fn stub_0xe6ca24() { todo!("0xe6ca24 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_") }

//0xe6cd6c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::MaterialPtr> const&)")]
// type: int __fastcall(int, int, int, int)
pub fn stub_0xe6cd6c() { todo!("0xe6cd6c __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_") }

//0xe6cde0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::MaterialPtr> const&)")]
// type: int __fastcall(int, int, int)
pub fn stub_0xe6cde0() { todo!("0xe6cde0 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_") }

//0xe6cec4 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::MaterialPtr> const&)")]
// type: _DWORD *__fastcall(int, const std::string *, int, int, void *, int)
pub fn stub_0xe6cec4() { todo!("0xe6cec4 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_") }

//0xe6cff0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// type: 
pub fn stub_0xe6cff0() { todo!("0xe6cff0 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_") }

//0xe6d094 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe6d094() { todo!("0xe6d094 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev") }

//0xe6d098 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe6d098() { todo!("0xe6d098 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev") }

//0xe6d0a4 — __ZNSt8_Rb_treeIPN4Ogre7Matrix4ESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias="__ZNSt8_Rb_treeIPN4Ogre7Matrix4ESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
#[doc(alias="std::_Rb_tree<Ogre::Matrix4 *,std::pair<Ogre::Matrix4 * const,unsigned long>,std::_Select1st<std::pair<Ogre::Matrix4 * const,unsigned long>>,std::less<Ogre::Matrix4 *>,Ogre::STLAllocator<std::pair<Ogre::Matrix4 * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Matrix4 * const,unsigned long>> *)")]
// type: 
pub fn stub_0xe6d0a4() { todo!("0xe6d0a4 __ZNSt8_Rb_treeIPN4Ogre7Matrix4ESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E") }

//0xe6d0cc — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::MaterialPtr>> *)")]
// type: 
pub fn stub_0xe6d0cc() { todo!("0xe6d0cc __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre11MaterialPtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E") }

//0xe6d0fc — __ZN4Ogre12STLAllocatorISt4pairIKSsNS_11MaterialPtrEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS4_
#[doc(alias="__ZN4Ogre12STLAllocatorISt4pairIKSsNS_11MaterialPtrEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS4_")]
#[doc(alias="Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::MaterialPtr>*)")]
// type: 
pub fn stub_0xe6d0fc() { todo!("0xe6d0fc __ZN4Ogre12STLAllocatorISt4pairIKSsNS_11MaterialPtrEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS4_") }

//0xe73574 — __ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findES7_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findES7_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// type: 
pub fn stub_0xe73574() { todo!("0xe73574 __ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findES7_") }

//0xe73618 — __ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeISD_E
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeISD_E")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>> *)")]
// type: 
pub fn stub_0xe73618() { todo!("0xe73618 __ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeISD_E") }

//0xe73690 — __ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISH_Lb0EED1Ev
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISH_Lb0EED1Ev")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe73690() { todo!("0xe73690 __ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISH_Lb0EED1Ev") }

//0xe73694 — __ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISH_Lb0EED0Ev
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISH_Lb0EED0Ev")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe73694() { todo!("0xe73694 __ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISH_Lb0EED0Ev") }

//0xe73d74 — __ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias="__ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
#[doc(alias="std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::HardwareIndexBuffer * const&)")]
// type: int __fastcall(char *)
pub fn stub_0xe73d74() { todo!("0xe73d74 __ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_") }

//0xe73e6c — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias="__ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
#[doc(alias="std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::HardwareVertexBuffer * const&)")]
// type: _DWORD *__fastcall(char *, _DWORD *, int *)
pub fn stub_0xe73e6c() { todo!("0xe73e6c __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_") }

//0xe78194 — __ZNSt6vectorIPN4Ogre13RenderTextureENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm
#[doc(alias="__ZNSt6vectorIPN4Ogre13RenderTextureENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm")]
#[doc(alias="std::vector<Ogre::RenderTexture *,Ogre::STLAllocator<Ogre::RenderTexture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
// type: 
pub fn stub_0xe78194() { todo!("0xe78194 __ZNSt6vectorIPN4Ogre13RenderTextureENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm") }

//0xe7820c — __ZNSt12_Vector_baseIPN4Ogre13RenderTextureENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias="__ZNSt12_Vector_baseIPN4Ogre13RenderTextureENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
#[doc(alias="std::_Vector_base<Ogre::RenderTexture *,Ogre::STLAllocator<Ogre::RenderTexture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xe7820c() { todo!("0xe7820c __ZNSt12_Vector_baseIPN4Ogre13RenderTextureENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev") }

//0xe78210 — __ZNSt12_Vector_baseIPN4Ogre13RenderTextureENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias="__ZNSt12_Vector_baseIPN4Ogre13RenderTextureENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
#[doc(alias="std::_Vector_base<Ogre::RenderTexture *,Ogre::STLAllocator<Ogre::RenderTexture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xe78210() { todo!("0xe78210 __ZNSt12_Vector_baseIPN4Ogre13RenderTextureENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev") }

//0xe7821c — __ZNSt6vectorIPN4Ogre13RenderTextureENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias="__ZNSt6vectorIPN4Ogre13RenderTextureENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
#[doc(alias="std::vector<Ogre::RenderTexture *,Ogre::STLAllocator<Ogre::RenderTexture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderTexture **,std::vector<Ogre::RenderTexture *,Ogre::STLAllocator<Ogre::RenderTexture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderTexture * const&)")]
// type: int __fastcall(int, void *__src)
pub fn stub_0xe7821c() { todo!("0xe7821c __ZNSt6vectorIPN4Ogre13RenderTextureENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_") }

//0xe7ed04 — __ZNSt6vectorIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm
#[doc(alias="__ZNSt6vectorIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm")]
#[doc(alias="std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
// type: 
pub fn stub_0xe7ed04() { todo!("0xe7ed04 __ZNSt6vectorIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm") }

//0xe7f490 — __ZNSt6vectorIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS6_EERKj
#[doc(alias="__ZNSt6vectorIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS6_EERKj")]
#[doc(alias="std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned int const&)")]
// type: int __fastcall(int, void *__src)
pub fn stub_0xe7f490() { todo!("0xe7f490 __ZNSt6vectorIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS6_EERKj") }

//0xe7f590 — __ZNSt12_Vector_baseIPN4Ogre11DepthBufferENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias="__ZNSt12_Vector_baseIPN4Ogre11DepthBufferENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
#[doc(alias="std::_Vector_base<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xe7f590() { todo!("0xe7f590 __ZNSt12_Vector_baseIPN4Ogre11DepthBufferENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev") }

//0xe7f5a0 — __ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
#[doc(alias="__ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_")]
#[doc(alias="std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// type: struct _Unwind_Exception *__fastcall(_DWORD *, int, unsigned __int16 *)
pub fn stub_0xe7f5a0() { todo!("0xe7f5a0 __ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_") }

//0xe7f658 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::RenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// type: 
pub fn stub_0xe7f658() { todo!("0xe7f658 __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_") }

//0xe7f700 — __ZNSt12_Vector_baseIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias="__ZNSt12_Vector_baseIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
#[doc(alias="std::_Vector_base<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xe7f700() { todo!("0xe7f700 __ZNSt12_Vector_baseIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev") }

//0xe7f808 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKSs
#[doc(alias="__ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKSs")]
#[doc(alias="std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::string const&)")]
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xe7f808() { todo!("0xe7f808 __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKSs") }

//0xe828a8 — __ZN4OgreL9doImageIOERKSsS1_S1_RSt6vectorINS_5ImageENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPNS_8ResourceE
#[doc(alias="__ZN4OgreL9doImageIOERKSsS1_S1_RSt6vectorINS_5ImageENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPNS_8ResourceE")]
#[doc(alias="Ogre::doImageIO(std::string const&,std::string const&,std::string const&,std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &,Ogre::Resource *)")]
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, char, int, int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
pub fn stub_0xe828a8() { todo!("0xe828a8 __ZN4OgreL9doImageIOERKSsS1_S1_RSt6vectorINS_5ImageENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPNS_8ResourceE") }

//0xe8313c — __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev
#[doc(alias="__ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev")]
#[doc(alias="Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// type: 
pub fn stub_0xe8313c() { todo!("0xe8313c __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev") }

//0xe831ec — __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEaSERKS9_
#[doc(alias="__ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEaSERKS9_")]
#[doc(alias="Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::operator=(Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// type: 
pub fn stub_0xe831ec() { todo!("0xe831ec __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEaSERKS9_") }

//0xe832f8 — __ZNSt6vectorIN4Ogre28HardwarePixelBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias="__ZNSt6vectorIN4Ogre28HardwarePixelBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
#[doc(alias="std::vector<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::HardwarePixelBufferSharedPtr*,std::vector<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::HardwarePixelBufferSharedPtr const&)")]
// type: int __fastcall(int, int)
pub fn stub_0xe832f8() { todo!("0xe832f8 __ZNSt6vectorIN4Ogre28HardwarePixelBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_") }

//0xe83898 — __ZNSt12_Vector_baseIN4Ogre5ImageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias="__ZNSt12_Vector_baseIN4Ogre5ImageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
#[doc(alias="std::_Vector_base<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xe83898() { todo!("0xe83898 __ZNSt12_Vector_baseIN4Ogre5ImageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev") }

//0xe8389c — __ZNSt12_Vector_baseIN4Ogre5ImageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias="__ZNSt12_Vector_baseIN4Ogre5ImageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
#[doc(alias="std::_Vector_base<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xe8389c() { todo!("0xe8389c __ZNSt12_Vector_baseIN4Ogre5ImageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev") }

//0xe838a8 — __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev
#[doc(alias="__ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev")]
#[doc(alias="Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// type: 
pub fn stub_0xe838a8() { todo!("0xe838a8 __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev") }

//0xe8395c — __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv
#[doc(alias="__ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv")]
#[doc(alias="Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
// type: 
pub fn stub_0xe8395c() { todo!("0xe8395c __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv") }

//0xe83a6c — __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERS9_
#[doc(alias="__ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERS9_")]
#[doc(alias="Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)")]
// type: 
pub fn stub_0xe83a6c() { todo!("0xe83a6c __ZN4Ogre9SharedPtrISt6vectorINS_5ImageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERS9_") }

//0xe83a88 — __ZNSt12_Vector_baseIN4Ogre28HardwarePixelBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias="__ZNSt12_Vector_baseIN4Ogre28HardwarePixelBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
#[doc(alias="std::_Vector_base<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xe83a88() { todo!("0xe83a88 __ZNSt12_Vector_baseIN4Ogre28HardwarePixelBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev") }

//0xe83a8c — __ZNSt12_Vector_baseIN4Ogre28HardwarePixelBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias="__ZNSt12_Vector_baseIN4Ogre28HardwarePixelBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
#[doc(alias="std::_Vector_base<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xe83a8c() { todo!("0xe83a8c __ZNSt12_Vector_baseIN4Ogre28HardwarePixelBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev") }

//0xe83a98 — __ZNSt6vectorIN4Ogre5ImageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias="__ZNSt6vectorIN4Ogre5ImageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
#[doc(alias="std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Image*,std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Image const&)")]
// type: 
pub fn stub_0xe83a98() { todo!("0xe83a98 __ZNSt6vectorIN4Ogre5ImageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_") }

//0xe86f24 — __ZNSt3mapISsN4Ogre13_ConfigOptionESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
#[doc(alias="__ZNSt3mapISsN4Ogre13_ConfigOptionESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_")]
#[doc(alias="std::map<std::string,Ogre::_ConfigOption,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// type: 
pub fn stub_0xe86f24() { todo!("0xe86f24 __ZNSt3mapISsN4Ogre13_ConfigOptionESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_") }

//0xe87524 — __ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS6_
#[doc(alias="__ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS6_")]
#[doc(alias="std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xe87524() { todo!("0xe87524 __ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS6_") }

//0xe87644 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::_ConfigOption>>,std::pair<std::string const,Ogre::_ConfigOption> const&)")]
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xe87644() { todo!("0xe87644 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_") }

//0xe87824 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::_ConfigOption> const&)")]
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xe87824() { todo!("0xe87824 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_") }

//0xe8796c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::_ConfigOption> const&)")]
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xe8796c() { todo!("0xe8796c __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_") }

//0xe87d74 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::_ConfigOption>> *)")]
// type: 
pub fn stub_0xe87d74() { todo!("0xe87d74 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E") }

//0xe87df4 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe87df4() { todo!("0xe87df4 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev") }

//0xe87df8 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// type: void __fastcall(void *)
pub fn stub_0xe87df8() { todo!("0xe87df8 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev") }

//0xe8a5c0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// type: 
pub fn stub_0xe8a5c0() { todo!("0xe8a5c0 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_") }

//0xe8b890 — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias="__ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
#[doc(alias="std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *> const&)")]
// type: int __fastcall(char *, _Rb_tree_node_base *)
pub fn stub_0xe8b890() { todo!("0xe8b890 __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_") }

//0xe8bb94 — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias="__ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
#[doc(alias="std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *> const&)")]
// type: int __fastcall(int, char *)
pub fn stub_0xe8bb94() { todo!("0xe8bb94 __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_") }

//0xe8bd04 — __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe8bd04() { todo!("0xe8bd04 __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev") }

//0xe8bd08 — __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe8bd08() { todo!("0xe8bd08 __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev") }

//0xe8bd14 — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias="__ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev")]
#[doc(alias="std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long long>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe8bd14() { todo!("0xe8bd14 __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev") }

//0xe8bd18 — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias="__ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev")]
#[doc(alias="std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long long>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe8bd18() { todo!("0xe8bd18 __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev") }

//0xe8bd24 — __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned int>> *)")]
// type: 
pub fn stub_0xe8bd24() { todo!("0xe8bd24 __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E") }

//0xe8bd9c — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias="__ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
#[doc(alias="std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>> *)")]
// type: int __fastcall(int result, Ogre::NedPoolingImpl *)
pub fn stub_0xe8bd9c() { todo!("0xe8bd9c __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E") }

//0xe8d72c — __ZNSt3mapISsN4Ogre21VertexElementSemanticESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
#[doc(alias="__ZNSt3mapISsN4Ogre21VertexElementSemanticESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_")]
#[doc(alias="std::map<std::string,Ogre::VertexElementSemantic,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// type: 
pub fn stub_0xe8d72c() { todo!("0xe8d72c __ZNSt3mapISsN4Ogre21VertexElementSemanticESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_") }

//0xe8d8e8 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::pair<std::string const,Ogre::VertexElementSemantic> const&)")]
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xe8d8e8() { todo!("0xe8d8e8 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_") }

//0xe8dac8 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::VertexElementSemantic> const&)")]
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xe8dac8() { todo!("0xe8dac8 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_") }

//0xe8dc1c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::VertexElementSemantic> const&)")]
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xe8dc1c() { todo!("0xe8dc1c __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_") }

//0xe8dd00 — __ZNSt12_Vector_baseIN4Ogre18GLUniformReferenceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias="__ZNSt12_Vector_baseIN4Ogre18GLUniformReferenceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
#[doc(alias="std::_Vector_base<Ogre::GLUniformReference,Ogre::STLAllocator<Ogre::GLUniformReference,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xe8dd00() { todo!("0xe8dd00 __ZNSt12_Vector_baseIN4Ogre18GLUniformReferenceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev") }

//0xe8dd04 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe8dd04() { todo!("0xe8dd04 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev") }

//0xe8dd08 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe8dd08() { todo!("0xe8dd08 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev") }

//0xe8dd14 — __ZNSt12_Vector_baseIN4Ogre18GLUniformReferenceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias="__ZNSt12_Vector_baseIN4Ogre18GLUniformReferenceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
#[doc(alias="std::_Vector_base<Ogre::GLUniformReference,Ogre::STLAllocator<Ogre::GLUniformReference,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xe8dd14() { todo!("0xe8dd14 __ZNSt12_Vector_baseIN4Ogre18GLUniformReferenceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev") }

//0xe8dd20 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::VertexElementSemantic>> *)")]
// type: int __fastcall(int result, Ogre::NedPoolingImpl *)
pub fn stub_0xe8dd20() { todo!("0xe8dd20 __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E") }

//0xe8e594 — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias="__ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
#[doc(alias="std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>>,std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *> const&)")]
// type: int __fastcall(char *, _Rb_tree_node_base *)
pub fn stub_0xe8e594() { todo!("0xe8e594 __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_") }

//0xe8e898 — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias="__ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
#[doc(alias="std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *> const&)")]
// type: int __fastcall(int, char *)
pub fn stub_0xe8e898() { todo!("0xe8e898 __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_") }

//0xe8ea08 — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias="__ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev")]
#[doc(alias="std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long long>,false>::~_Rb_tree_impl()")]
// type: void()
pub fn stub_0xe8ea08() { todo!("0xe8ea08 __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev") }

//0xe8ea0c — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias="__ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev")]
#[doc(alias="std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long long>,false>::~_Rb_tree_impl()")]
// type: 
pub fn stub_0xe8ea0c() { todo!("0xe8ea0c __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev") }

//0xe8ea18 — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias="__ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
#[doc(alias="std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>> *)")]
// type: 
pub fn stub_0xe8ea18() { todo!("0xe8ea18 __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre21GLSLESProgramPipelineEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E") }

//0xe96b80 — __ZN4Ogre26GLSLESProgramManagerCommon15extractUniformsEjPKSt3mapISsNS_21GpuConstantDefinitionESt4lessISsENS_12STLAllocatorISt4pairIKSsS2_ENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEESF_RSt6vectorINS_18GLUniformReferenceENS5_ISH_SB_EEE
#[doc(alias="__ZN4Ogre26GLSLESProgramManagerCommon15extractUniformsEjPKSt3mapISsNS_21GpuConstantDefinitionESt4lessISsENS_12STLAllocatorISt4pairIKSsS2_ENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEESF_RSt6vectorINS_18GLUniformReferenceENS5_ISH_SB_EEE")]
#[doc(alias="Ogre::GLSLESProgramManagerCommon::extractUniforms(unsigned int,std::map<std::string,Ogre::GpuConstantDefinition,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,std::map<std::string,Ogre::GpuConstantDefinition,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,std::vector<Ogre::GLUniformReference,Ogre::STLAllocator<Ogre::GLUniformReference,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)")]
// type: int __fastcall(int, GLuint program, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xe96b80() { todo!("0xe96b80 __ZN4Ogre26GLSLESProgramManagerCommon15extractUniformsEjPKSt3mapISsNS_21GpuConstantDefinitionESt4lessISsENS_12STLAllocatorISt4pairIKSsS2_ENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEESF_RSt6vectorINS_18GLUniformReferenceENS5_ISH_SB_EEE") }

//0xe97c3c — __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// type: 
pub fn stub_0xe97c3c() { todo!("0xe97c3c __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE4findERS1_") }

//0xe97ce0 — __ZNSt6vectorIN4Ogre18GLUniformReferenceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias="__ZNSt6vectorIN4Ogre18GLUniformReferenceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
#[doc(alias="std::vector<Ogre::GLUniformReference,Ogre::STLAllocator<Ogre::GLUniformReference,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GLUniformReference*,std::vector<Ogre::GLUniformReference,Ogre::STLAllocator<Ogre::GLUniformReference,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GLUniformReference const&)")]
// type: 
pub fn stub_0xe97ce0() { todo!("0xe97ce0 __ZNSt6vectorIN4Ogre18GLUniformReferenceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_") }

//0xe97e50 — __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,unsigned int> const&)")]
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xe97e50() { todo!("0xe97e50 __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_") }

//0xe97f34 — __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS2_
#[doc(alias="__ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS2_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,unsigned int> const&)")]
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xe97f34() { todo!("0xe97f34 __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS2_") }

//0xea3728 — __ZNSt6vectorIPN4Ogre23ParticleAffectorFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias="__ZNSt6vectorIPN4Ogre23ParticleAffectorFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
#[doc(alias="std::vector<Ogre::ParticleAffectorFactory *,Ogre::STLAllocator<Ogre::ParticleAffectorFactory *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParticleAffectorFactory **,std::vector<Ogre::ParticleAffectorFactory *,Ogre::STLAllocator<Ogre::ParticleAffectorFactory *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParticleAffectorFactory * const&)")]
// type: int __fastcall(int, void *__src)
pub fn stub_0xea3728() { todo!("0xea3728 __ZNSt6vectorIPN4Ogre23ParticleAffectorFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_") }

//0xea3820 — __ZNSt6vectorIPN4Ogre22ParticleEmitterFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias="__ZNSt6vectorIPN4Ogre22ParticleEmitterFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
#[doc(alias="std::vector<Ogre::ParticleEmitterFactory *,Ogre::STLAllocator<Ogre::ParticleEmitterFactory *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParticleEmitterFactory **,std::vector<Ogre::ParticleEmitterFactory *,Ogre::STLAllocator<Ogre::ParticleEmitterFactory *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParticleEmitterFactory * const&)")]
// type: _DWORD *__fastcall(int, char *__src, _DWORD *)
pub fn stub_0xea3820() { todo!("0xea3820 __ZNSt6vectorIPN4Ogre22ParticleEmitterFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_") }

//0xea3918 — __ZNSt12_Vector_baseIPN4Ogre22ParticleEmitterFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias="__ZNSt12_Vector_baseIPN4Ogre22ParticleEmitterFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
#[doc(alias="std::_Vector_base<Ogre::ParticleEmitterFactory *,Ogre::STLAllocator<Ogre::ParticleEmitterFactory *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xea3918() { todo!("0xea3918 __ZNSt12_Vector_baseIPN4Ogre22ParticleEmitterFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev") }

//0xea391c — __ZNSt12_Vector_baseIPN4Ogre23ParticleAffectorFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias="__ZNSt12_Vector_baseIPN4Ogre23ParticleAffectorFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
#[doc(alias="std::_Vector_base<Ogre::ParticleAffectorFactory *,Ogre::STLAllocator<Ogre::ParticleAffectorFactory *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xea391c() { todo!("0xea391c __ZNSt12_Vector_baseIPN4Ogre23ParticleAffectorFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev") }

//0xea3920 — __ZNSt12_Vector_baseIPN4Ogre23ParticleAffectorFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias="__ZNSt12_Vector_baseIPN4Ogre23ParticleAffectorFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
#[doc(alias="std::_Vector_base<Ogre::ParticleAffectorFactory *,Ogre::STLAllocator<Ogre::ParticleAffectorFactory *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xea3920() { todo!("0xea3920 __ZNSt12_Vector_baseIPN4Ogre23ParticleAffectorFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev") }

//0xea392c — __ZNSt12_Vector_baseIPN4Ogre22ParticleEmitterFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias="__ZNSt12_Vector_baseIPN4Ogre22ParticleEmitterFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
#[doc(alias="std::_Vector_base<Ogre::ParticleEmitterFactory *,Ogre::STLAllocator<Ogre::ParticleEmitterFactory *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// type: 
pub fn stub_0xea392c() { todo!("0xea392c __ZNSt12_Vector_baseIPN4Ogre22ParticleEmitterFactoryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev") }

//0xf4ea64 — j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias="j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
#[doc(alias="std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,G3D::Vector3int16 *>> *)")]
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf4ea64() { todo!("0xf4ea64 j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E") }

//0xf57ed4 — j___ZNSt6vectorIN3G3D7Vector3ESaIS1_EEaSERKS3_
#[doc(alias="j___ZNSt6vectorIN3G3D7Vector3ESaIS1_EEaSERKS3_")]
#[doc(alias="std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>::operator=(std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&)")]
// type: 
pub fn stub_0xf57ed4() { todo!("0xf57ed4 j___ZNSt6vectorIN3G3D7Vector3ESaIS1_EEaSERKS3_") }

//0xf58904 — j___ZNSt12_Vector_baseIN3G3D12Vector3int16ESaIS1_EE11_M_allocateEm
#[doc(alias="j___ZNSt12_Vector_baseIN3G3D12Vector3int16ESaIS1_EE11_M_allocateEm")]
#[doc(alias="std::_Vector_base<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::_M_allocate(unsigned long)")]
// type: int __fastcall(_DWORD)
pub fn stub_0xf58904() { todo!("0xf58904 j___ZNSt12_Vector_baseIN3G3D12Vector3int16ESaIS1_EE11_M_allocateEm") }

//0xf58944 — j___ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
#[doc(alias="j___ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
#[doc(alias="std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Vector3int16*,std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>>,unsigned long,G3D::Vector3int16 const&)")]
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf58944() { todo!("0xf58944 j___ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_") }

//0xf58954 — j___ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE6resizeEmS1_
#[doc(alias="j___ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE6resizeEmS1_")]
#[doc(alias="std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::resize(unsigned long,G3D::Vector3int16)")]
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf58954() { todo!("0xf58954 j___ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE6resizeEmS1_") }

//0xf589e4 — j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_
#[doc(alias="j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_")]
#[doc(alias="std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_insert_unique(std::pair<int const,G3D::Vector3int16 *> const&)")]
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf589e4() { todo!("0xf589e4 j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_") }

//0xf589f4 — j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
#[doc(alias="j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
#[doc(alias="std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,G3D::Vector3int16 *> const&)")]
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf589f4() { todo!("0xf589f4 j___ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_") }

//0xf5b8c4 — j___ZNSt12_Vector_baseIN3G3D7Matrix3ESaIS1_EE11_M_allocateEm
#[doc(alias="j___ZNSt12_Vector_baseIN3G3D7Matrix3ESaIS1_EE11_M_allocateEm")]
#[doc(alias="std::_Vector_base<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_allocate(unsigned long)")]
// type: 
pub fn stub_0xf5b8c4() { todo!("0xf5b8c4 j___ZNSt12_Vector_baseIN3G3D7Matrix3ESaIS1_EE11_M_allocateEm") }

//0xf5b924 — j___ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
#[doc(alias="j___ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
#[doc(alias="std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Matrix3*,std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>>,unsigned long,G3D::Matrix3 const&)")]
// type: int(void)
pub fn stub_0xf5b924() { todo!("0xf5b924 j___ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_") }
