//! watchdog_core_wdA — 100 core stubs EA-sorted global-dedup
//! Source: ida/export.json (85545 funcs) filtered core fallback excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound
//! Gap-fill EA-sorted asc from global missing set (656 remaining before this file), 100 stubs.
//! Format: //0xADDR — mangled + #[doc(alias=mangled)] + pub fn stub_0xADDR() { todo!("0xADDR") }
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr,
//! boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes/backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf66ec4 — j___ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E
#[doc(alias="j___ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E")]
#[doc(alias="std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Controller<float> *> *)")]
// type: 
pub fn stub_0xf66ec4() { todo!("0xf66ec4 j___ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E") }

// 0xf66ed4 — j___ZNSt6vectorIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(alias="j___ZNSt6vectorIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_")]
#[doc(alias="std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Polygon **,std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Polygon * const&)")]
// type: int __fastcall(int, void *__src)
pub fn stub_0xf66ed4() { todo!("0xf66ed4 j___ZNSt6vectorIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_") }

// 0xf66ee4 — j___ZNSt8_Rb_treeIN4Ogre10SceneQuery17WorldFragmentTypeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias="j___ZNSt8_Rb_treeIN4Ogre10SceneQuery17WorldFragmentTypeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
#[doc(alias="std::_Rb_tree<Ogre::SceneQuery::WorldFragmentType,Ogre::SceneQuery::WorldFragmentType,std::_Identity<Ogre::SceneQuery::WorldFragmentType>,std::less<Ogre::SceneQuery::WorldFragmentType>,Ogre::STLAllocator<Ogre::SceneQuery::WorldFragmentType,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::SceneQuery::WorldFragmentType const&)")]
// type: int __fastcall(char *)
pub fn stub_0xf66ee4() { todo!("0xf66ee4 j___ZNSt8_Rb_treeIN4Ogre10SceneQuery17WorldFragmentTypeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_") }

// 0xf66ef4 — j___ZNSt3mapISsPN4Ogre6DynLibESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias="j___ZNSt3mapISsPN4Ogre6DynLibESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
#[doc(alias="std::map<std::string,Ogre::DynLib *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// type: 
pub fn stub_0xf66ef4() { todo!("0xf66ef4 j___ZNSt3mapISsPN4Ogre6DynLibESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_") }

// 0xf66f04 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::DynLib *> const&)")]
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf66f04() { todo!("0xf66f04 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_") }

// 0xf66f14 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::DynLib *>>,std::pair<std::string const,Ogre::DynLib *> const&)")]
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf66f14() { todo!("0xf66f14 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_") }

// 0xf66f24 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// type: 
pub fn stub_0xf66f24() { todo!("0xf66f24 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_") }

// 0xf66f34 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::DynLib *>> *)")]
// type: 
pub fn stub_0xf66f34() { todo!("0xf66f34 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E") }

// 0xf66f44 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::DynLib *> const&)")]
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf66f44() { todo!("0xf66f44 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_") }

// 0xf66f54 — j___ZNSt6vectorIN4Ogre15EdgeListBuilder12CommonVertexENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias="j___ZNSt6vectorIN4Ogre15EdgeListBuilder12CommonVertexENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
#[doc(alias="std::vector<Ogre::EdgeListBuilder::CommonVertex,Ogre::STLAllocator<Ogre::EdgeListBuilder::CommonVertex,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::CommonVertex*,std::vector<Ogre::EdgeListBuilder::CommonVertex,Ogre::STLAllocator<Ogre::EdgeListBuilder::CommonVertex,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::CommonVertex const&)")]
// type: 
pub fn stub_0xf66f54() { todo!("0xf66f54 j___ZNSt6vectorIN4Ogre15EdgeListBuilder12CommonVertexENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_") }

// 0xf66f64 — j___ZNSt6vectorIN4Ogre15EdgeListBuilder8GeometryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias="j___ZNSt6vectorIN4Ogre15EdgeListBuilder8GeometryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
#[doc(alias="std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry*,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::Geometry const&)")]
// type: 
pub fn stub_0xf66f64() { todo!("0xf66f64 j___ZNSt6vectorIN4Ogre15EdgeListBuilder8GeometryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_") }

// 0xf66f74 — j___ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias="j___ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
#[doc(alias="std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector4*,std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>>,Ogre::Vector4 const&)")]
// type: 
pub fn stub_0xf66f74() { todo!("0xf66f74 j___ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_") }

// 0xf66f84 — j___ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE7reserveEm
#[doc(alias="j___ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE7reserveEm")]
#[doc(alias="std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::reserve(unsigned long)")]
// type: 
pub fn stub_0xf66f84() { todo!("0xf66f84 j___ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE7reserveEm") }

// 0xf66f94 — j___ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias="j___ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
#[doc(alias="std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EdgeData::Edge*,std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeData::Edge const&)")]
// type: 
pub fn stub_0xf66f94() { todo!("0xf66f94 j___ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_") }

// 0xf66fa4 — j___ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_
#[doc(alias="j___ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_")]
#[doc(alias="std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// type: 
pub fn stub_0xf66fa4() { todo!("0xf66fa4 j___ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_") }

// 0xf66fb4 — j___ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
#[doc(alias="j___ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_")]
#[doc(alias="std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// type: 
pub fn stub_0xf66fb4() { todo!("0xf66fb4 j___ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_") }

// 0xf66fc4 — j___ZNSt6vectorIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias="j___ZNSt6vectorIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
#[doc(alias="std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EdgeData::Triangle*,std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeData::Triangle const&)")]
// type: 
pub fn stub_0xf66fc4() { todo!("0xf66fc4 j___ZNSt6vectorIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_") }

// 0xf66fd4 — j___ZNSt6vectorIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm
#[doc(alias="j___ZNSt6vectorIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm")]
#[doc(alias="std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
// type: 
pub fn stub_0xf66fd4() { todo!("0xf66fd4 j___ZNSt6vectorIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm") }

// 0xf66fe4 — j___ZNSt6vectorIN4Ogre8EdgeData9EdgeGroupENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(alias="j___ZNSt6vectorIN4Ogre8EdgeData9EdgeGroupENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_")]
#[doc(alias="std::vector<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::EdgeData::EdgeGroup*,std::vector<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::EdgeData::EdgeGroup const&)")]
// type: int __fastcall(int, int)
pub fn stub_0xf66fe4() { todo!("0xf66fe4 j___ZNSt6vectorIN4Ogre8EdgeData9EdgeGroupENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_") }

// 0xf66ff4 — j___ZNSt6vectorIPKN4Ogre10VertexDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias="j___ZNSt6vectorIPKN4Ogre10VertexDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_")]
#[doc(alias="std::vector<Ogre::VertexData const*,Ogre::STLAllocator<Ogre::VertexData const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::VertexData const**,std::vector<Ogre::VertexData const*,Ogre::STLAllocator<Ogre::VertexData const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::VertexData const* const&)")]
// type: int __fastcall(int, void *__src)
pub fn stub_0xf66ff4() { todo!("0xf66ff4 j___ZNSt6vectorIPKN4Ogre10VertexDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_") }

// 0xf67004 — j___ZNSt6vectorIcN4Ogre12STLAllocatorIcNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPcS6_EEmRKc
#[doc(alias="j___ZNSt6vectorIcN4Ogre12STLAllocatorIcNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPcS6_EEmRKc")]
#[doc(alias="std::vector<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<char *,std::vector<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,char const&)")]
// type: int __fastcall(int, void *__b)
pub fn stub_0xf67004() { todo!("0xf67004 j___ZNSt6vectorIcN4Ogre12STLAllocatorIcNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPcS6_EEmRKc") }

// 0xf67014 — j___ZNSt8_Rb_treeIN4Ogre7Vector3ESt4pairIKS1_mESt10_Select1stIS4_ENS0_15EdgeListBuilder10vectorLessENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias="j___ZNSt8_Rb_treeIN4Ogre7Vector3ESt4pairIKS1_mESt10_Select1stIS4_ENS0_15EdgeListBuilder10vectorLessENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
#[doc(alias="std::_Rb_tree<Ogre::Vector3,std::pair<Ogre::Vector3 const,unsigned long>,std::_Select1st<std::pair<Ogre::Vector3 const,unsigned long>>,Ogre::EdgeListBuilder::vectorLess,Ogre::STLAllocator<std::pair<Ogre::Vector3 const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Vector3 const,unsigned long> const&)")]
// type: 
pub fn stub_0xf67014() { todo!("0xf67014 j___ZNSt8_Rb_treeIN4Ogre7Vector3ESt4pairIKS1_mESt10_Select1stIS4_ENS0_15EdgeListBuilder10vectorLessENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_") }

// 0xf67024 — j___ZNSt8_Rb_treeIN4Ogre7Vector3ESt4pairIKS1_mESt10_Select1stIS4_ENS0_15EdgeListBuilder10vectorLessENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias="j___ZNSt8_Rb_treeIN4Ogre7Vector3ESt4pairIKS1_mESt10_Select1stIS4_ENS0_15EdgeListBuilder10vectorLessENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
#[doc(alias="std::_Rb_tree<Ogre::Vector3,std::pair<Ogre::Vector3 const,unsigned long>,std::_Select1st<std::pair<Ogre::Vector3 const,unsigned long>>,Ogre::EdgeListBuilder::vectorLess,Ogre::STLAllocator<std::pair<Ogre::Vector3 const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Vector3 const,unsigned long>> *)")]
// type: 
pub fn stub_0xf67024() { todo!("0xf67024 j___ZNSt8_Rb_treeIN4Ogre7Vector3ESt4pairIKS1_mESt10_Select1stIS4_ENS0_15EdgeListBuilder10vectorLessENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E") }

// 0xf67034 — j___ZNSt8_Rb_treeISt4pairImmES0_IKS1_S1_ESt10_Select1stIS3_ESt4lessIS1_EN4Ogre12STLAllocatorIS3_NS8_22CategorisedAllocPolicyILNS8_14MemoryCategoryE0EEEEEE15_M_insert_equalERKS3_
#[doc(alias="j___ZNSt8_Rb_treeISt4pairImmES0_IKS1_S1_ESt10_Select1stIS3_ESt4lessIS1_EN4Ogre12STLAllocatorIS3_NS8_22CategorisedAllocPolicyILNS8_14MemoryCategoryE0EEEEEE15_M_insert_equalERKS3_")]
#[doc(alias="std::_Rb_tree<std::pair<unsigned long,unsigned long>,std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,std::_Select1st<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>>,std::less<std::pair<unsigned long,unsigned long>>,Ogre::STLAllocator<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_equal(std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>> const&)")]
// type: 
pub fn stub_0xf67034() { todo!("0xf67034 j___ZNSt8_Rb_treeISt4pairImmES0_IKS1_S1_ESt10_Select1stIS3_ESt4lessIS1_EN4Ogre12STLAllocatorIS3_NS8_22CategorisedAllocPolicyILNS8_14MemoryCategoryE0EEEEEE15_M_insert_equalERKS3_") }

// 0xf67044 — j___ZNSt8_Rb_treeISt4pairImmES0_IKS1_S1_ESt10_Select1stIS3_ESt4lessIS1_EN4Ogre12STLAllocatorIS3_NS8_22CategorisedAllocPolicyILNS8_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E
#[doc(alias="j___ZNSt8_Rb_treeISt4pairImmES0_IKS1_S1_ESt10_Select1stIS3_ESt4lessIS1_EN4Ogre12STLAllocatorIS3_NS8_22CategorisedAllocPolicyILNS8_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E")]
#[doc(alias="std::_Rb_tree<std::pair<unsigned long,unsigned long>,std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,std::_Select1st<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>>,std::less<std::pair<unsigned long,unsigned long>>,Ogre::STLAllocator<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>> *)")]
// type: 
pub fn stub_0xf67044() { todo!("0xf67044 j___ZNSt8_Rb_treeISt4pairImmES0_IKS1_S1_ESt10_Select1stIS3_ESt4lessIS1_EN4Ogre12STLAllocatorIS3_NS8_22CategorisedAllocPolicyILNS8_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E") }

// 0xf67054 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS4_NS3_12geometryLessEEvT_T0_SG_T1_T2_
#[doc(alias="j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS4_NS3_12geometryLessEEvT_T0_SG_T1_T2_")]
#[doc(alias="void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::EdgeListBuilder::Geometry,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::EdgeListBuilder::Geometry,Ogre::EdgeListBuilder::geometryLess)")]
// type: 
pub fn stub_0xf67054() { todo!("0xf67054 j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS4_NS3_12geometryLessEEvT_T0_SG_T1_T2_") }

// 0xf67064 — j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS3_12geometryLessEEvT_SF_SF_T0_
#[doc(alias="j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS3_12geometryLessEEvT_SF_SF_T0_")]
#[doc(alias="void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess)")]
// type: 
pub fn stub_0xf67064() { todo!("0xf67064 j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS3_12geometryLessEEvT_SF_SF_T0_") }

// 0xf67074 — j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS3_12geometryLessEEvT_SF_T0_
#[doc(alias="j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS3_12geometryLessEEvT_SF_T0_")]
#[doc(alias="void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess)")]
// type: 
pub fn stub_0xf67074() { todo!("0xf67074 j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS3_12geometryLessEEvT_SF_T0_") }

// 0xf67084 — j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS3_12geometryLessEEvT_SF_T0_T1_
#[doc(alias="j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS3_12geometryLessEEvT_SF_T0_T1_")]
#[doc(alias="void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::EdgeListBuilder::geometryLess)")]
// type: 
pub fn stub_0xf67084() { todo!("0xf67084 j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS3_12geometryLessEEvT_SF_T0_T1_") }

// 0xf67094 — j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS3_12geometryLessEEvT_SF_T0_
#[doc(alias="j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS3_12geometryLessEEvT_SF_T0_")]
#[doc(alias="void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess)")]
// type: 
pub fn stub_0xf67094() { todo!("0xf67094 j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS3_12geometryLessEEvT_SF_T0_") }

// 0xf670a4 — j___ZSt22__uninitialized_copy_aIPN4Ogre8EdgeData9EdgeGroupES3_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_SA_S9_T1_
#[doc(alias="j___ZSt22__uninitialized_copy_aIPN4Ogre8EdgeData9EdgeGroupES3_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_SA_S9_T1_")]
#[doc(alias="Ogre::EdgeData::EdgeGroup * std::__uninitialized_copy_a<Ogre::EdgeData::EdgeGroup *,Ogre::EdgeData::EdgeGroup *,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::EdgeData::EdgeGroup *,Ogre::EdgeData::EdgeGroup *,Ogre::EdgeData::EdgeGroup *,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf670a4() { todo!("0xf670a4 j___ZSt22__uninitialized_copy_aIPN4Ogre8EdgeData9EdgeGroupES3_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_SA_S9_T1_") }

// 0xf670b4 — j___ZSt24__uninitialized_fill_n_aIPN4Ogre8EdgeData9EdgeGroupEmS2_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEvT_T0_RKT1_T2_
#[doc(alias="j___ZSt24__uninitialized_fill_n_aIPN4Ogre8EdgeData9EdgeGroupEmS2_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEvT_T0_RKT1_T2_")]
#[doc(alias="void std::__uninitialized_fill_n_a<Ogre::EdgeData::EdgeGroup *,unsigned long,Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::EdgeData::EdgeGroup *,unsigned long,Ogre::EdgeData::EdgeGroup const&,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf670b4() { todo!("0xf670b4 j___ZSt24__uninitialized_fill_n_aIPN4Ogre8EdgeData9EdgeGroupEmS2_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEvT_T0_RKT1_T2_") }

// 0xf670c4 — j___ZN4Ogre14AxisAlignedBox15transformAffineERKNS_7Matrix4E
#[doc(alias="j___ZN4Ogre14AxisAlignedBox15transformAffineERKNS_7Matrix4E")]
#[doc(alias="Ogre::AxisAlignedBox::transformAffine(Ogre::Matrix4 const&)")]
// type: _DWORD __fastcall(Ogre::AxisAlignedBox *__hidden this, const Ogre::Matrix4 *)
pub fn stub_0xf670c4() { todo!("0xf670c4 j___ZN4Ogre14AxisAlignedBox15transformAffineERKNS_7Matrix4E") }

// 0xf670d4 — j___ZNSt6vectorIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(alias="j___ZNSt6vectorIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_")]
#[doc(alias="std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::ShadowRenderable **,std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::ShadowRenderable * const&)")]
// type: int __fastcall(int, void *__src)
pub fn stub_0xf670d4() { todo!("0xf670d4 j___ZNSt6vectorIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_") }

// 0xf670e4 — j___ZNSt6vectorIPN4Ogre6EntityENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias="j___ZNSt6vectorIPN4Ogre6EntityENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
#[doc(alias="std::vector<Ogre::Entity *,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Entity **,std::vector<Ogre::Entity *,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Entity * const&)")]
// type: int __fastcall(int, void *__src)
pub fn stub_0xf670e4() { todo!("0xf670e4 j___ZNSt6vectorIPN4Ogre6EntityENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_") }

// 0xf670f4 — j___ZNSt6vectorIPN4Ogre9SubEntityENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias="j___ZNSt6vectorIPN4Ogre9SubEntityENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
#[doc(alias="std::vector<Ogre::SubEntity *,Ogre::STLAllocator<Ogre::SubEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SubEntity **,std::vector<Ogre::SubEntity *,Ogre::STLAllocator<Ogre::SubEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SubEntity * const&)")]
// type: int __fastcall(int, void *__src)
pub fn stub_0xf670f4() { todo!("0xf670f4 j___ZNSt6vectorIPN4Ogre9SubEntityENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_") }

// 0xf67104 — j___ZNSt8_Rb_treeIPN4Ogre6EntityES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
#[doc(alias="j___ZNSt8_Rb_treeIPN4Ogre6EntityES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_")]
#[doc(alias="std::_Rb_tree<Ogre::Entity *,Ogre::Entity *,std::_Identity<Ogre::Entity *>,std::less<Ogre::Entity *>,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Entity *>,std::_Rb_tree_iterator<Ogre::Entity *>)")]
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf67104() { todo!("0xf67104 j___ZNSt8_Rb_treeIPN4Ogre6EntityES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_") }

// 0xf67114 — j___ZNSt8_Rb_treeIPN4Ogre6EntityES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias="j___ZNSt8_Rb_treeIPN4Ogre6EntityES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
#[doc(alias="std::_Rb_tree<Ogre::Entity *,Ogre::Entity *,std::_Identity<Ogre::Entity *>,std::less<Ogre::Entity *>,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Entity *> *)")]
// type: 
pub fn stub_0xf67114() { todo!("0xf67114 j___ZNSt8_Rb_treeIPN4Ogre6EntityES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E") }

// 0xf67124 — j___ZNSt8_Rb_treeItSt4pairIKtbESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias="j___ZNSt8_Rb_treeItSt4pairIKtbESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
#[doc(alias="std::_Rb_tree<unsigned short,std::pair<unsigned short const,bool>,std::_Select1st<std::pair<unsigned short const,bool>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,bool> const&)")]
// type: int __fastcall(char *)
pub fn stub_0xf67124() { todo!("0xf67124 j___ZNSt8_Rb_treeItSt4pairIKtbESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_") }

// 0xf67134 — j___ZNSt8_Rb_treeItSt4pairIKtbESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias="j___ZNSt8_Rb_treeItSt4pairIKtbESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
#[doc(alias="std::_Rb_tree<unsigned short,std::pair<unsigned short const,bool>,std::_Select1st<std::pair<unsigned short const,bool>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,bool>> *)")]
// type: 
pub fn stub_0xf67134() { todo!("0xf67134 j___ZNSt8_Rb_treeItSt4pairIKtbESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E") }

// 0xf67144 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21ExternalTextureSourceEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21ExternalTextureSourceEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ExternalTextureSource *>,std::_Select1st<std::pair<std::string const,Ogre::ExternalTextureSource *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ExternalTextureSource *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ExternalTextureSource *>> *)")]
// type: 
pub fn stub_0xf67144() { todo!("0xf67144 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21ExternalTextureSourceEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E") }

// 0xf67154 — j___ZN4Ogre9SharedPtrINS_7TextureEEaSERKS2_
#[doc(alias="j___ZN4Ogre9SharedPtrINS_7TextureEEaSERKS2_")]
#[doc(alias="Ogre::SharedPtr<Ogre::Texture>::operator=(Ogre::SharedPtr<Ogre::Texture> const&)")]
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
pub fn stub_0xf67154() { todo!("0xf67154 j___ZN4Ogre9SharedPtrINS_7TextureEEaSERKS2_") }

// 0xf67164 — j___ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias="j___ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// type: 
pub fn stub_0xf67164() { todo!("0xf67164 j___ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_") }

// 0xf67174 — j___ZNSt3mapISsPN4Ogre12ParamCommandESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias="j___ZNSt3mapISsPN4Ogre12ParamCommandESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
#[doc(alias="std::map<std::string,Ogre::ParamCommand *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf67174() { todo!("0xf67174 j___ZNSt3mapISsPN4Ogre12ParamCommandESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_") }

// 0xf67184 — j___ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
#[doc(alias="j___ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_")]
#[doc(alias="std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::ParameterDef const&)")]
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf67184() { todo!("0xf67184 j___ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_") }

// 0xf67194 — j___ZNSt6vectorIPKN4Ogre5ImageENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias="j___ZNSt6vectorIPKN4Ogre5ImageENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_")]
#[doc(alias="std::vector<Ogre::Image const*,Ogre::STLAllocator<Ogre::Image const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Image const**,std::vector<Ogre::Image const*,Ogre::STLAllocator<Ogre::Image const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Image const* const&)")]
// type: int __fastcall(int, void *__src)
pub fn stub_0xf67194() { todo!("0xf67194 j___ZNSt6vectorIPKN4Ogre5ImageENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_") }

// 0xf671a4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::ParamDictionary> const&)")]
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf671a4() { todo!("0xf671a4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_") }

// 0xf671b4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamDictionary> const&)")]
// type: 
pub fn stub_0xf671b4() { todo!("0xf671b4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_") }

// 0xf671c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamCommand *> const&)")]
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf671c4() { todo!("0xf671c4 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_") }

// 0xf671d4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSH_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSH_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> const*,std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>>*)")]
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf671d4() { todo!("0xf671d4 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSH_") }

// 0xf671e4 — j___ZSt22__uninitialized_copy_aIPN4Ogre12ParameterDefES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
#[doc(alias="j___ZSt22__uninitialized_copy_aIPN4Ogre12ParameterDefES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_")]
#[doc(alias="Ogre::ParameterDef * std::__uninitialized_copy_a<Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf671e4() { todo!("0xf671e4 j___ZSt22__uninitialized_copy_aIPN4Ogre12ParameterDefES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_") }

// 0xf671f4 — j___ZN4Ogre7FontPtrD1Ev
#[doc(alias="j___ZN4Ogre7FontPtrD1Ev")]
#[doc(alias="Ogre::FontPtr::~FontPtr()")]
// type: void __fastcall(Ogre::FontPtr *__hidden this)
pub fn stub_0xf671f4() { todo!("0xf671f4 j___ZN4Ogre7FontPtrD1Ev") }

// 0xf67204 — j___ZN4Ogre9SharedPtrINS_4FontEE7destroyEv
#[doc(alias="j___ZN4Ogre9SharedPtrINS_4FontEE7destroyEv")]
#[doc(alias="Ogre::SharedPtr<Ogre::Font>::destroy(void)")]
// type: 
pub fn stub_0xf67204() { todo!("0xf67204 j___ZN4Ogre9SharedPtrINS_4FontEE7destroyEv") }

// 0xf67214 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKjN4Ogre4Font9GlyphInfoEEEjS8_NS_4hashIjEESt8equal_toIjEEEE12emplace_implIJS9_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEbERS5_DpOT_
#[doc(alias="j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKjN4Ogre4Font9GlyphInfoEEEjS8_NS_4hashIjEESt8equal_toIjEEEE12emplace_implIJS9_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEbERS5_DpOT_")]
#[doc(alias="std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>,unsigned int,Ogre::Font::GlyphInfo,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::emplace_impl<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>(unsigned int const&,std::pair<unsigned int const,Ogre::Font::GlyphInfo> &&)")]
// type: 
pub fn stub_0xf67214() { todo!("0xf67214 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKjN4Ogre4Font9GlyphInfoEEEjS8_NS_4hashIjEESt8equal_toIjEEEE12emplace_implIJS9_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEbERS5_DpOT_") }

// 0xf67224 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjN4Ogre4Font9GlyphInfoEEEjS8_NS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm
#[doc(alias="j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjN4Ogre4Font9GlyphInfoEEEjS8_NS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm")]
#[doc(alias="boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>,unsigned int,Ogre::Font::GlyphInfo,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::create_buckets(unsigned long)")]
// type: 
pub fn stub_0xf67224() { todo!("0xf67224 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjN4Ogre4Font9GlyphInfoEEEjS8_NS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm") }

// 0xf67234 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjN4Ogre4Font9GlyphInfoEEEjS8_NS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm
#[doc(alias="j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjN4Ogre4Font9GlyphInfoEEEjS8_NS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm")]
#[doc(alias="boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,Ogre::Font::GlyphInfo>>,unsigned int,Ogre::Font::GlyphInfo,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)")]
// type: 
pub fn stub_0xf67234() { todo!("0xf67234 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjN4Ogre4Font9GlyphInfoEEEjS8_NS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm") }

// 0xf67244 — j___ZNSt6vectorISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S8_EERKS1_
#[doc(alias="j___ZNSt6vectorISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S8_EERKS1_")]
#[doc(alias="std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<unsigned int,unsigned int>*,std::vector<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::pair<unsigned int,unsigned int> const&)")]
// type: 
pub fn stub_0xf67244() { todo!("0xf67244 j___ZNSt6vectorISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S8_EERKS1_") }

// 0xf67254 — j___ZSt22__uninitialized_copy_aIPSsS0_N4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEET0_T_S8_S7_T1_
#[doc(alias="j___ZSt22__uninitialized_copy_aIPSsS0_N4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEET0_T_S8_S7_T1_")]
#[doc(alias="std::string * std::__uninitialized_copy_a<std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::string *,std::string *,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf67254() { todo!("0xf67254 j___ZSt22__uninitialized_copy_aIPSsS0_N4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEET0_T_S8_S7_T1_") }

// 0xf67264 — j___ZNK4Ogre7Matrix4mlERKNS_5PlaneE
#[doc(alias="j___ZNK4Ogre7Matrix4mlERKNS_5PlaneE")]
#[doc(alias="Ogre::Matrix4::operator*(Ogre::Plane const&)const")]
// type: 
pub fn stub_0xf67264() { todo!("0xf67264 j___ZNK4Ogre7Matrix4mlERKNS_5PlaneE") }

// 0xf67274 — j___ZN4Ogre10GpuProgramD2Ev
#[doc(alias="j___ZN4Ogre10GpuProgramD2Ev")]
#[doc(alias="Ogre::GpuProgram::~GpuProgram()")]
// type: void __fastcall(Ogre::GpuProgram *__hidden this)
pub fn stub_0xf67274() { todo!("0xf67274 j___ZN4Ogre10GpuProgramD2Ev") }

// 0xf67284 — j___ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEEaSERKS2_
#[doc(alias="j___ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEEaSERKS2_")]
#[doc(alias="Ogre::SharedPtr<Ogre::GpuNamedConstants>::operator=(Ogre::SharedPtr<Ogre::GpuNamedConstants> const&)")]
// type: 
pub fn stub_0xf67284() { todo!("0xf67284 j___ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEEaSERKS2_") }

// 0xf67294 — j___ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEEaSERKS2_
#[doc(alias="j___ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEEaSERKS2_")]
#[doc(alias="Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::operator=(Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct> const&)")]
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xf67294() { todo!("0xf67294 j___ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEEaSERKS2_") }

// 0xf672a4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS4_EPSG_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS4_EPSG_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,Ogre::GpuConstantDefinition>> const*,std::_Rb_tree_node<std::pair<std::string const,Ogre::GpuConstantDefinition>>*)")]
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf672a4() { todo!("0xf672a4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS4_EPSG_") }

// 0xf672b4 — j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre18GpuLogicalIndexUseEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias="j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre18GpuLogicalIndexUseEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
#[doc(alias="std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,std::_Select1st<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long const,Ogre::GpuLogicalIndexUse> const&)")]
// type: int __fastcall(char *)
pub fn stub_0xf672b4() { todo!("0xf672b4 j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre18GpuLogicalIndexUseEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_") }

// 0xf672c4 — j___ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_16MemoryDataStreamEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_
#[doc(alias="j___ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_16MemoryDataStreamEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_")]
#[doc(alias="Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>*)")]
// type: 
pub fn stub_0xf672c4() { todo!("0xf672c4 j___ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_16MemoryDataStreamEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_") }

// 0xf672d4 — j___ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_19GpuSharedParametersEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_
#[doc(alias="j___ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_19GpuSharedParametersEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_")]
#[doc(alias="Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>*)")]
// type: 
pub fn stub_0xf672d4() { todo!("0xf672d4 j___ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_19GpuSharedParametersEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_") }

// 0xf672e4 — j___ZN4Ogre9SharedPtrINS_19GpuSharedParametersEEaSERKS2_
#[doc(alias="j___ZN4Ogre9SharedPtrINS_19GpuSharedParametersEEaSERKS2_")]
#[doc(alias="Ogre::SharedPtr<Ogre::GpuSharedParameters>::operator=(Ogre::SharedPtr<Ogre::GpuSharedParameters> const&)")]
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
pub fn stub_0xf672e4() { todo!("0xf672e4 j___ZN4Ogre9SharedPtrINS_19GpuSharedParametersEEaSERKS2_") }

// 0xf672f4 — j___ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias="j___ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// type: 
pub fn stub_0xf672f4() { todo!("0xf672f4 j___ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_") }

// 0xf67304 — j___ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias="j___ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// type: 
pub fn stub_0xf67304() { todo!("0xf67304 j___ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_") }

// 0xf67314 — j___ZNSt3mapISsN4Ogre9SharedPtrINS0_19GpuSharedParametersEEESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
#[doc(alias="j___ZNSt3mapISsN4Ogre9SharedPtrINS0_19GpuSharedParametersEEESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_")]
#[doc(alias="std::map<std::string,Ogre::SharedPtr<Ogre::GpuSharedParameters>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// type: 
pub fn stub_0xf67314() { todo!("0xf67314 j___ZNSt3mapISsN4Ogre9SharedPtrINS0_19GpuSharedParametersEEESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_") }

// 0xf67324 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS6_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS6_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>> const&)")]
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf67324() { todo!("0xf67324 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS6_") }

// 0xf67334 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>> const&)")]
// type: int __fastcall(int, int, int)
pub fn stub_0xf67334() { todo!("0xf67334 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_") }

// 0xf67344 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// type: 
pub fn stub_0xf67344() { todo!("0xf67344 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_") }

// 0xf67354 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>> *)")]
// type: 
pub fn stub_0xf67354() { todo!("0xf67354 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E") }

// 0xf67364 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>> const&)")]
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf67364() { todo!("0xf67364 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_") }

// 0xf67374 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS6_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS6_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> const&)")]
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf67374() { todo!("0xf67374 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS6_") }

// 0xf67384 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> const&)")]
// type: int __fastcall(int, int, int)
pub fn stub_0xf67384() { todo!("0xf67384 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_") }

// 0xf67394 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> const&)")]
// type: int __fastcall(int)
pub fn stub_0xf67394() { todo!("0xf67394 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_") }

// 0xf673a4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// type: 
pub fn stub_0xf673a4() { todo!("0xf673a4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_") }

// 0xf673b4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>> *)")]
// type: 
pub fn stub_0xf673b4() { todo!("0xf673b4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E") }

// 0xf673c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> const&)")]
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf673c4() { todo!("0xf673c4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_") }

// 0xf673d4 — j___ZN4Ogre24GpuSharedParametersUsageC2ERKS0_
#[doc(alias="j___ZN4Ogre24GpuSharedParametersUsageC2ERKS0_")]
#[doc(alias="Ogre::GpuSharedParametersUsage::GpuSharedParametersUsage(Ogre::GpuSharedParametersUsage const&)")]
// type: _DWORD __fastcall(Ogre::GpuSharedParametersUsage *__hidden this, const Ogre::GpuSharedParametersUsage *)
pub fn stub_0xf673d4() { todo!("0xf673d4 j___ZN4Ogre24GpuSharedParametersUsageC2ERKS0_") }

// 0xf673e4 — j___ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias="j___ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// type: 
pub fn stub_0xf673e4() { todo!("0xf673e4 j___ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_") }

// 0xf673f4 — j___ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN4Ogre24GpuSharedParametersUsageES4_EET0_T_S6_S5_
#[doc(alias="j___ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN4Ogre24GpuSharedParametersUsageES4_EET0_T_S6_S5_")]
#[doc(alias="Ogre::GpuSharedParametersUsage * std::__copy_backward_normal<false,false>::__copy_b_n<Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *>(Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *)")]
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xf673f4() { todo!("0xf673f4 j___ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN4Ogre24GpuSharedParametersUsageES4_EET0_T_S6_S5_") }

// 0xf67404 — j___ZNSt3mapISsN4Ogre21GpuConstantDefinitionESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
#[doc(alias="j___ZNSt3mapISsN4Ogre21GpuConstantDefinitionESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_")]
#[doc(alias="std::map<std::string,Ogre::GpuConstantDefinition,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// type: 
pub fn stub_0xf67404() { todo!("0xf67404 j___ZNSt3mapISsN4Ogre21GpuConstantDefinitionESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_") }

// 0xf67424 — j___ZNSt6vectorIN4Ogre20GpuProgramParameters17AutoConstantEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias="j___ZNSt6vectorIN4Ogre20GpuProgramParameters17AutoConstantEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
#[doc(alias="std::vector<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GpuProgramParameters::AutoConstantEntry*,std::vector<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GpuProgramParameters::AutoConstantEntry const&)")]
// type: 
pub fn stub_0xf67424() { todo!("0xf67424 j___ZNSt6vectorIN4Ogre20GpuProgramParameters17AutoConstantEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_") }

// 0xf67434 — j___ZNSt6vectorIN4Ogre20GpuProgramParameters17AutoConstantEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
#[doc(alias="j___ZNSt6vectorIN4Ogre20GpuProgramParameters17AutoConstantEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_")]
#[doc(alias="std::vector<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// type: 
pub fn stub_0xf67434() { todo!("0xf67434 j___ZNSt6vectorIN4Ogre20GpuProgramParameters17AutoConstantEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_") }

// 0xf67444 — j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias="j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
#[doc(alias="std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GpuSharedParametersUsage::CopyDataEntry*,std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GpuSharedParametersUsage::CopyDataEntry const&)")]
// type: 
pub fn stub_0xf67444() { todo!("0xf67444 j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_") }

// 0xf67454 — j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_
#[doc(alias="j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_")]
#[doc(alias="std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// type: 
pub fn stub_0xf67454() { todo!("0xf67454 j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_") }

// 0xf67464 — j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
#[doc(alias="j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_")]
#[doc(alias="std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// type: 
pub fn stub_0xf67464() { todo!("0xf67464 j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_") }

// 0xf67474 — j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias="j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
#[doc(alias="std::vector<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GpuSharedParametersUsage*,std::vector<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GpuSharedParametersUsage const&)")]
// type: int __fastcall(int, int, int)
pub fn stub_0xf67474() { todo!("0xf67474 j___ZNSt6vectorIN4Ogre24GpuSharedParametersUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_") }

// 0xf67484 — j___ZNSt6vectorIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_
#[doc(alias="j___ZNSt6vectorIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_")]
#[doc(alias="std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// type: 
pub fn stub_0xf67484() { todo!("0xf67484 j___ZNSt6vectorIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_") }

// 0xf67494 — j___ZNSt6vectorIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPiS6_EEmRKi
#[doc(alias="j___ZNSt6vectorIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPiS6_EEmRKi")]
#[doc(alias="std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<int *,std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,int const&)")]
// type: int __fastcall(int, void *__src)
pub fn stub_0xf67494() { todo!("0xf67494 j___ZNSt6vectorIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPiS6_EEmRKi") }

// 0xf674a4 — j___ZNSt6vectorIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_
#[doc(alias="j___ZNSt6vectorIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_")]
#[doc(alias="std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// type: 
pub fn stub_0xf674a4() { todo!("0xf674a4 j___ZNSt6vectorIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_") }

// 0xf674b4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::GpuConstantDefinition> const&)")]
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf674b4() { todo!("0xf674b4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_") }

// 0xf674c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::pair<std::string const,Ogre::GpuConstantDefinition> const&)")]
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf674c4() { todo!("0xf674c4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_") }

// 0xf674d4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// type: 
pub fn stub_0xf674d4() { todo!("0xf674d4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_") }

// 0xf674e4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
#[doc(alias="j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_")]
#[doc(alias="std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::GpuConstantDefinition> const&)")]
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf674e4() { todo!("0xf674e4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_") }

// 0xf67534 — j___ZSt22__uninitialized_copy_aIPN4Ogre24GpuSharedParametersUsageES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
#[doc(alias="j___ZSt22__uninitialized_copy_aIPN4Ogre24GpuSharedParametersUsageES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_")]
#[doc(alias="Ogre::GpuSharedParametersUsage * std::__uninitialized_copy_a<Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// type: int __fastcall(int, int, int, int, Ogre::GpuSharedParametersUsage *, void *, int, int, int, int)
pub fn stub_0xf67534() { todo!("0xf67534 j___ZSt22__uninitialized_copy_aIPN4Ogre24GpuSharedParametersUsageES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_") }

// 0xf67544 — j___ZN4Ogre9SharedPtrINS_10GpuProgramEEaSERKS2_
#[doc(alias="j___ZN4Ogre9SharedPtrINS_10GpuProgramEEaSERKS2_")]
#[doc(alias="Ogre::SharedPtr<Ogre::GpuProgram>::operator=(Ogre::SharedPtr<Ogre::GpuProgram> const&)")]
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
pub fn stub_0xf67544() { todo!("0xf67544 j___ZN4Ogre9SharedPtrINS_10GpuProgramEEaSERKS2_") }

