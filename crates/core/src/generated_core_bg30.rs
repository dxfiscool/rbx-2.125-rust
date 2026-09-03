//! core bg30 - 96 core stubs EA-sorted asc, final uncovered export batch (global dedup vs /tmp/global_eas.txt).
//! Source: ida/export.json (85545 funcs) uncovered 0xf68cd4..0xf692f4; export exhausted after this batch.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Capabilities>,std::_Select1st<std::pair<std::string const,Ogre::Capabilities>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Capabilities>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Capabilities>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre12CapabilitiesEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// 0xf68cd4 - j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre12CapabilitiesEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: 
pub fn stub_0xf68cd4() -> ! {
    todo!("0xf68cd4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre12CapabilitiesEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Capabilities>,std::_Select1st<std::pair<std::string const,Ogre::Capabilities>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Capabilities>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Capabilities> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre12CapabilitiesEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_")]
// 0xf68ce4 - j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre12CapabilitiesEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf68ce4() -> ! {
    todo!("0xf68ce4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre12CapabilitiesEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderSystemCapabilitiesSerializer::CapabilityKeywordType>,std::_Select1st<std::pair<std::string const,Ogre::RenderSystemCapabilitiesSerializer::CapabilityKeywordType>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderSystemCapabilitiesSerializer::CapabilityKeywordType>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::RenderSystemCapabilitiesSerializer::CapabilityKeywordType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre34RenderSystemCapabilitiesSerializer21CapabilityKeywordTypeEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// 0xf68cf4 - j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre34RenderSystemCapabilitiesSerializer21CapabilityKeywordTypeEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf68cf4() -> ! {
    todo!("0xf68cf4 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre34RenderSystemCapabilitiesSerializer21CapabilityKeywordTypeEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderSystemCapabilitiesSerializer::CapabilityKeywordType>,std::_Select1st<std::pair<std::string const,Ogre::RenderSystemCapabilitiesSerializer::CapabilityKeywordType>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderSystemCapabilitiesSerializer::CapabilityKeywordType>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::RenderSystemCapabilitiesSerializer::CapabilityKeywordType>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre34RenderSystemCapabilitiesSerializer21CapabilityKeywordTypeEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf68d04 - j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre34RenderSystemCapabilitiesSerializer21CapabilityKeywordTypeEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
pub fn stub_0xf68d04() -> ! {
    todo!("0xf68d04 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre34RenderSystemCapabilitiesSerializer21CapabilityKeywordTypeEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderSystemCapabilitiesSerializer::CapabilityKeywordType>,std::_Select1st<std::pair<std::string const,Ogre::RenderSystemCapabilitiesSerializer::CapabilityKeywordType>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderSystemCapabilitiesSerializer::CapabilityKeywordType>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::RenderSystemCapabilitiesSerializer::CapabilityKeywordType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre34RenderSystemCapabilitiesSerializer21CapabilityKeywordTypeEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// 0xf68d14 - j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre34RenderSystemCapabilitiesSerializer21CapabilityKeywordTypeEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf68d14() -> ! {
    todo!("0xf68d14 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre34RenderSystemCapabilitiesSerializer21CapabilityKeywordTypeEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")
}

#[doc(alias = "std::pair<std::string,int> * std::__uninitialized_copy_a<std::pair<std::string,int> *,std::pair<std::string,int> *,Ogre::STLAllocator<std::pair<std::string,int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::pair<std::string,int> *,std::pair<std::string,int> *,std::pair<std::string,int> *,Ogre::STLAllocator<std::pair<std::string,int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
#[doc(alias = "j___ZSt22__uninitialized_copy_aIPSt4pairISsiES2_N4Ogre12STLAllocatorIS1_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEET0_T_SA_S9_T1_")]
// 0xf68d24 - j___ZSt22__uninitialized_copy_aIPSt4pairISsiES2_N4Ogre12STLAllocatorIS1_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEET0_T_SA_S9_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf68d24() -> ! {
    todo!("0xf68d24 j___ZSt22__uninitialized_copy_aIPSt4pairISsiES2_N4Ogre12STLAllocatorIS1_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEET0_T_SA_S9_T1_")
}

#[doc(alias = "std::vector<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderTargetListener **,std::vector<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderTargetListener * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// 0xf68d34 - j___ZNSt6vectorIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf68d34() -> ! {
    todo!("0xf68d34 j___ZNSt6vectorIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")
}

#[doc(alias = "std::vector<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_")]
// 0xf68d44 - j___ZNSt6vectorIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_
// type: 
pub fn stub_0xf68d44() -> ! {
    todo!("0xf68d44 j___ZNSt6vectorIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<int const,Ogre::Viewport *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// 0xf68d54 - j___ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(char *)
pub fn stub_0xf68d54() -> ! {
    todo!("0xf68d54 j___ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<int const,Ogre::Viewport *>>,std::_Rb_tree_iterator<std::pair<int const,Ogre::Viewport *>>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS5_ESH_")]
// 0xf68d64 - j___ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS5_ESH_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf68d64() -> ! {
    todo!("0xf68d64 j___ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS5_ESH_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,Ogre::Viewport *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf68d74 - j___ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
pub fn stub_0xf68d74() -> ! {
    todo!("0xf68d74 j___ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<Ogre::Resource::Listener *,Ogre::Resource::Listener *,std::_Identity<Ogre::Resource::Listener *>,std::less<Ogre::Resource::Listener *>,Ogre::STLAllocator<Ogre::Resource::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Resource::Listener * const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS3_")]
// 0xf68d84 - j___ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS3_
// type: int __fastcall(char *)
pub fn stub_0xf68d84() -> ! {
    todo!("0xf68d84 j___ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS3_")
}

#[doc(alias = "std::_Rb_tree<Ogre::Resource::Listener *,Ogre::Resource::Listener *,std::_Identity<Ogre::Resource::Listener *>,std::less<Ogre::Resource::Listener *>,Ogre::STLAllocator<Ogre::Resource::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Resource::Listener *>,std::_Rb_tree_iterator<Ogre::Resource::Listener *>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS3_ESF_")]
// 0xf68d94 - j___ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS3_ESF_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf68d94() -> ! {
    todo!("0xf68d94 j___ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS3_ESF_")
}

#[doc(alias = "std::_Rb_tree<Ogre::Resource::Listener *,Ogre::Resource::Listener *,std::_Identity<Ogre::Resource::Listener *>,std::less<Ogre::Resource::Listener *>,Ogre::STLAllocator<Ogre::Resource::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Resource::Listener *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E")]
// 0xf68da4 - j___ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E
// type: 
pub fn stub_0xf68da4() -> ! {
    todo!("0xf68da4 j___ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E")
}

#[doc(alias = "Ogre::ResourceBackgroundQueue::ResourceRequest::ResourceRequest(Ogre::ResourceBackgroundQueue::ResourceRequest const&)")]
#[doc(alias = "j___ZN4Ogre23ResourceBackgroundQueue15ResourceRequestC2ERKS1_")]
// 0xf68db4 - j___ZN4Ogre23ResourceBackgroundQueue15ResourceRequestC2ERKS1_
// type: int __fastcall(int, int, int, int, char, char, char, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xf68db4() -> ! {
    todo!("0xf68db4 j___ZN4Ogre23ResourceBackgroundQueue15ResourceRequestC2ERKS1_")
}

#[doc(alias = "Ogre::ResourceBackgroundQueue::ResourceResponse::ResourceResponse(Ogre::SharedPtr<Ogre::Resource>,Ogre::ResourceBackgroundQueue::ResourceRequest const&)")]
#[doc(alias = "j___ZN4Ogre23ResourceBackgroundQueue16ResourceResponseC1ENS_9SharedPtrINS_8ResourceEEERKNS0_15ResourceRequestE")]
// 0xf68dc4 - j___ZN4Ogre23ResourceBackgroundQueue16ResourceResponseC1ENS_9SharedPtrINS_8ResourceEEERKNS0_15ResourceRequestE
// type: int __fastcall(char, int, int, int, int, int)
pub fn stub_0xf68dc4() -> ! {
    todo!("0xf68dc4 j___ZN4Ogre23ResourceBackgroundQueue16ResourceResponseC1ENS_9SharedPtrINS_8ResourceEEERKNS0_15ResourceRequestE")
}

#[doc(alias = "Ogre::ResourceBackgroundQueue::ResourceResponse::~ResourceResponse()")]
#[doc(alias = "j___ZN4Ogre23ResourceBackgroundQueue16ResourceResponseD2Ev")]
// 0xf68dd4 - j___ZN4Ogre23ResourceBackgroundQueue16ResourceResponseD2Ev
// type: void __fastcall(Ogre::ResourceBackgroundQueue::ResourceResponse *__hidden this)
pub fn stub_0xf68dd4() -> ! {
    todo!("0xf68dd4 j___ZN4Ogre23ResourceBackgroundQueue16ResourceResponseD2Ev")
}

#[doc(alias = "Ogre::Any::holder<Ogre::ResourceBackgroundQueue::ResourceResponse>::holder(Ogre::ResourceBackgroundQueue::ResourceResponse const&)")]
#[doc(alias = "j___ZN4Ogre3Any6holderINS_23ResourceBackgroundQueue16ResourceResponseEEC1ERKS3_")]
// 0xf68de4 - j___ZN4Ogre3Any6holderINS_23ResourceBackgroundQueue16ResourceResponseEEC1ERKS3_
// type: int __fastcall(char, int, int, int, int, int)
pub fn stub_0xf68de4() -> ! {
    todo!("0xf68de4 j___ZN4Ogre3Any6holderINS_23ResourceBackgroundQueue16ResourceResponseEEC1ERKS3_")
}

#[doc(alias = "Ogre::ResourceBackgroundQueue::ResourceRequest Ogre::any_cast<Ogre::ResourceBackgroundQueue::ResourceRequest>(Ogre::Any const&)")]
#[doc(alias = "j___ZN4Ogre8any_castINS_23ResourceBackgroundQueue15ResourceRequestEEET_RKNS_3AnyE")]
// 0xf68df4 - j___ZN4Ogre8any_castINS_23ResourceBackgroundQueue15ResourceRequestEEET_RKNS_3AnyE
// type: 
pub fn stub_0xf68df4() -> ! {
    todo!("0xf68df4 j___ZN4Ogre8any_castINS_23ResourceBackgroundQueue15ResourceRequestEEET_RKNS_3AnyE")
}

#[doc(alias = "Ogre::ResourceBackgroundQueue::ResourceResponse Ogre::any_cast<Ogre::ResourceBackgroundQueue::ResourceResponse>(Ogre::Any const&)")]
#[doc(alias = "j___ZN4Ogre8any_castINS_23ResourceBackgroundQueue16ResourceResponseEEET_RKNS_3AnyE")]
// 0xf68e04 - j___ZN4Ogre8any_castINS_23ResourceBackgroundQueue16ResourceResponseEEET_RKNS_3AnyE
// type: 
pub fn stub_0xf68e04() -> ! {
    todo!("0xf68e04 j___ZN4Ogre8any_castINS_23ResourceBackgroundQueue16ResourceResponseEEET_RKNS_3AnyE")
}

#[doc(alias = "std::_Rb_tree<unsigned long long,unsigned long long,std::_Identity<unsigned long long>,std::less<unsigned long long>,Ogre::STLAllocator<unsigned long long,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<unsigned long long>,std::_Rb_tree_iterator<unsigned long long>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIyySt9_IdentityIyESt4lessIyEN4Ogre12STLAllocatorIyNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIyESC_")]
// 0xf68e14 - j___ZNSt8_Rb_treeIyySt9_IdentityIyESt4lessIyEN4Ogre12STLAllocatorIyNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIyESC_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf68e14() -> ! {
    todo!("0xf68e14 j___ZNSt8_Rb_treeIyySt9_IdentityIyESt4lessIyEN4Ogre12STLAllocatorIyNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIyESC_")
}

#[doc(alias = "std::_Rb_tree<unsigned long long,unsigned long long,std::_Identity<unsigned long long>,std::less<unsigned long long>,Ogre::STLAllocator<unsigned long long,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<unsigned long long> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIyySt9_IdentityIyESt4lessIyEN4Ogre12STLAllocatorIyNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIyE")]
// 0xf68e24 - j___ZNSt8_Rb_treeIyySt9_IdentityIyESt4lessIyEN4Ogre12STLAllocatorIyNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIyE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf68e24() -> ! {
    todo!("0xf68e24 j___ZNSt8_Rb_treeIyySt9_IdentityIyESt4lessIyEN4Ogre12STLAllocatorIyNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIyE")
}

#[doc(alias = "Ogre::ResourceGroupManager::ResourceGroup::~ResourceGroup()")]
#[doc(alias = "j___ZN4Ogre20ResourceGroupManager13ResourceGroupD2Ev")]
// 0xf68e34 - j___ZN4Ogre20ResourceGroupManager13ResourceGroupD2Ev
// type: void __fastcall(Ogre::ResourceGroupManager::ResourceGroup *__hidden this)
pub fn stub_0xf68e34() -> ! {
    todo!("0xf68e34 j___ZN4Ogre20ResourceGroupManager13ResourceGroupD2Ev")
}

#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::operator=(Ogre::SharedPtr<Ogre::Resource> const&)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_8ResourceEEaSERKS2_")]
// 0xf68e44 - j___ZN4Ogre9SharedPtrINS_8ResourceEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
pub fn stub_0xf68e44() -> ! {
    todo!("0xf68e44 j___ZN4Ogre9SharedPtrINS_8ResourceEEaSERKS2_")
}

#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::STLAllocator<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrISt4listINS0_ISt6vectorINS_8FileInfoENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEEE7destroyEv")]
// 0xf68e54 - j___ZN4Ogre9SharedPtrISt4listINS0_ISt6vectorINS_8FileInfoENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEEE7destroyEv
// type: 
pub fn stub_0xf68e54() -> ! {
    todo!("0xf68e54 j___ZN4Ogre9SharedPtrISt4listINS0_ISt6vectorINS_8FileInfoENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEEE7destroyEv")
}

#[doc(alias = "std::_List_base<Ogre::ResourceGroupManager::ResourceDeclaration,Ogre::STLAllocator<Ogre::ResourceGroupManager::ResourceDeclaration,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~_List_base()")]
#[doc(alias = "j___ZNSt10_List_baseIN4Ogre20ResourceGroupManager19ResourceDeclarationENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED2Ev")]
// 0xf68e64 - j___ZNSt10_List_baseIN4Ogre20ResourceGroupManager19ResourceDeclarationENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED2Ev
// type: 
pub fn stub_0xf68e64() -> ! {
    todo!("0xf68e64 j___ZNSt10_List_baseIN4Ogre20ResourceGroupManager19ResourceDeclarationENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED2Ev")
}

#[doc(alias = "std::map<std::string,Ogre::ResourceManager *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsPN4Ogre15ResourceManagerESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
// 0xf68e74 - j___ZNSt3mapISsPN4Ogre15ResourceManagerESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// type: 
pub fn stub_0xf68e74() -> ! {
    todo!("0xf68e74 j___ZNSt3mapISsPN4Ogre15ResourceManagerESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")
}

#[doc(alias = "void std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_range_insert<__gnu_cxx::__normal_iterator<Ogre::FileInfo*,std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>(__gnu_cxx::__normal_iterator<Ogre::FileInfo*,std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::FileInfo*,std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::FileInfo*,std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::forward_iterator_tag)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre8FileInfoENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_range_insertIN9__gnu_cxx17__normal_iteratorIPS1_S7_EEEEvSC_T_SD_St20forward_iterator_tag")]
// 0xf68e84 - j___ZNSt6vectorIN4Ogre8FileInfoENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_range_insertIN9__gnu_cxx17__normal_iteratorIPS1_S7_EEEEvSC_T_SD_St20forward_iterator_tag
// type: 
pub fn stub_0xf68e84() -> ! {
    todo!("0xf68e84 j___ZNSt6vectorIN4Ogre8FileInfoENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_range_insertIN9__gnu_cxx17__normal_iteratorIPS1_S7_EEEEvSC_T_SD_St20forward_iterator_tag")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceManager *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ResourceManager *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// 0xf68e94 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf68e94() -> ! {
    todo!("0xf68e94 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceManager *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::ResourceManager *>>,std::pair<std::string const,Ogre::ResourceManager *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// 0xf68ea4 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf68ea4() -> ! {
    todo!("0xf68ea4 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceManager *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// 0xf68eb4 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
pub fn stub_0xf68eb4() -> ! {
    todo!("0xf68eb4 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceManager *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ResourceManager *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf68ec4 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
pub fn stub_0xf68ec4() -> ! {
    todo!("0xf68ec4 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceManager *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::ResourceManager *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// 0xf68ed4 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf68ed4() -> ! {
    todo!("0xf68ed4 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20ResourceGroupManager13ResourceGroupEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
// 0xf68ee4 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20ResourceGroupManager13ResourceGroupEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf68ee4() -> ! {
    todo!("0xf68ee4 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20ResourceGroupManager13ResourceGroupEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20ResourceGroupManager13ResourceGroupEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// 0xf68ef4 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20ResourceGroupManager13ResourceGroupEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
pub fn stub_0xf68ef4() -> ! {
    todo!("0xf68ef4 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20ResourceGroupManager13ResourceGroupEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20ResourceGroupManager13ResourceGroupEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
// 0xf68f04 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20ResourceGroupManager13ResourceGroupEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
// type: 
pub fn stub_0xf68f04() -> ! {
    todo!("0xf68f04 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20ResourceGroupManager13ResourceGroupEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::ResourceGroupManager::ResourceGroup *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20ResourceGroupManager13ResourceGroupEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_")]
// 0xf68f14 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20ResourceGroupManager13ResourceGroupEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf68f14() -> ! {
    todo!("0xf68f14 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20ResourceGroupManager13ResourceGroupEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_")
}

#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE16_M_insert_uniqueERKSE_")]
// 0xf68f34 - j___ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE16_M_insert_uniqueERKSE_
// type: int __fastcall(char *)
pub fn stub_0xf68f34() -> ! {
    todo!("0xf68f34 j___ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE16_M_insert_uniqueERKSE_")
}

#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISE_ERKSE_")]
// 0xf68f44 - j___ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISE_ERKSE_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf68f44() -> ! {
    todo!("0xf68f44 j___ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISE_ERKSE_")
}

#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E")]
// 0xf68f54 - j___ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E
// type: 
pub fn stub_0xf68f54() -> ! {
    todo!("0xf68f54 j___ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E")
}

#[doc(alias = "Ogre::FileInfo * std::__uninitialized_copy_a<__gnu_cxx::__normal_iterator<Ogre::FileInfo *,std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::FileInfo *,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(__gnu_cxx::__normal_iterator<Ogre::FileInfo *,std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::FileInfo *,std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::FileInfo *,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
#[doc(alias = "j___ZSt22__uninitialized_copy_aIN9__gnu_cxx17__normal_iteratorIPN4Ogre8FileInfoESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES4_SA_ET0_T_SE_SD_T1_")]
// 0xf68f64 - j___ZSt22__uninitialized_copy_aIN9__gnu_cxx17__normal_iteratorIPN4Ogre8FileInfoESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES4_SA_ET0_T_SE_SD_T1_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, char, int, int, int, int, void *, int)
pub fn stub_0xf68f64() -> ! {
    todo!("0xf68f64 j___ZSt22__uninitialized_copy_aIN9__gnu_cxx17__normal_iteratorIPN4Ogre8FileInfoESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES4_SA_ET0_T_SE_SD_T1_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>,std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>> *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_")]
// 0xf68f74 - j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_
// type: 
pub fn stub_0xf68f74() -> ! {
    todo!("0xf68f74 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>,std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>(std::string const&,std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>> &&)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12emplace_implIJSA_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS5_DpOT_")]
// 0xf68f84 - j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12emplace_implIJSA_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS5_DpOT_
// type: int __fastcall(int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf68f84() -> ! {
    todo!("0xf68f84 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12emplace_implIJSA_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS5_DpOT_")
}

#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>,std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>,boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>,std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>> &)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeISA_EEEEEEEvNS0_15iterator_detail8iteratorISL_EERNS1_5tableISG_EERT_")]
// 0xf68f94 - j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeISA_EEEEEEEvNS0_15iterator_detail8iteratorISL_EERNS1_5tableISG_EERT_
// type: int __fastcall(std::string *, int, int)
pub fn stub_0xf68f94() -> ! {
    todo!("0xf68f94 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeISA_EEEEEEEvNS0_15iterator_detail8iteratorISL_EERNS1_5tableISG_EERT_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>>,std::string,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>> *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE11erase_nodesEPNS1_8ptr_nodeISI_EESO_")]
// 0xf68fa4 - j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE11erase_nodesEPNS1_8ptr_nodeISI_EESO_
// type: 
pub fn stub_0xf68fa4() -> ! {
    todo!("0xf68fa4 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE11erase_nodesEPNS1_8ptr_nodeISI_EESO_")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>>,std::string,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>>(std::string const&,std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>> &&)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE12emplace_implIJSI_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISI_EEEEbERS5_DpOT_")]
// 0xf68fb4 - j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE12emplace_implIJSI_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISI_EEEEbERS5_DpOT_
// type: int __fastcall(int, int, int, int, int, int, int, void *, char, char, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf68fb4() -> ! {
    todo!("0xf68fb4 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE12emplace_implIJSI_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISI_EEEEbERS5_DpOT_")
}

#[doc(alias = "j___ZN5boost9unordered6detail16allocator_traitsISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE7destroyISA_EENS_12disable_if_cIXsr5boost9unordered6detail11has_destroyISC_T_EE5valueEvE4typeERSC_PSG_")]
#[doc(alias = "j___ZN5boost9unordered6detail16allocator_traitsISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE7destroyISA_EENS_12disable_if_cIXsr5boost9unordered6detail11has_destroyISC_T_EE5valueEvE4typeERSC_PSG_")]
// 0xf68fc4 - j___ZN5boost9unordered6detail16allocator_traitsISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE7destroyISA_EENS_12disable_if_cIXsr5boost9unordered6detail11has_destroyISC_T_EE5valueEvE4typeERSC_PSG_
// type: 
pub fn stub_0xf68fc4() -> ! {
    todo!("0xf68fc4 j___ZN5boost9unordered6detail16allocator_traitsISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE7destroyISA_EENS_12disable_if_cIXsr5boost9unordered6detail11has_destroyISC_T_EE5valueEvE4typeERSC_PSG_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>::construct_with_value<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>(std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>> &&)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE20construct_with_valueIJSA_EEEvDpOT_")]
// 0xf68fd4 - j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE20construct_with_valueIJSA_EEEvDpOT_
// type: int __fastcall(int, std::string *)
pub fn stub_0xf68fd4() -> ! {
    todo!("0xf68fd4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE20construct_with_valueIJSA_EEEvDpOT_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>::construct_with_value2<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>> const&>(std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE21construct_with_value2IRKSA_EEvOT_")]
// 0xf68fe4 - j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE21construct_with_value2IRKSA_EEvOT_
// type: int __fastcall(int, std::string *)
pub fn stub_0xf68fe4() -> ! {
    todo!("0xf68fe4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE21construct_with_value2IRKSA_EEvOT_")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>>>>::construct(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEEEEE9constructEv")]
// 0xf68ff4 - j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEEEEE9constructEv
// type: 
pub fn stub_0xf68ff4() -> ! {
    todo!("0xf68ff4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>,std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")]
// 0xf69004 - j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// type: 
pub fn stub_0xf69004() -> ! {
    todo!("0xf69004 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>,std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")]
// 0xf69014 - j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// type: 
pub fn stub_0xf69014() -> ! {
    todo!("0xf69014 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>,std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")]
// 0xf69024 - j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: 
pub fn stub_0xf69024() -> ! {
    todo!("0xf69024 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>,std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>>>::init(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>,std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE4initERKSH_")]
// 0xf69034 - j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE4initERKSH_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf69034() -> ! {
    todo!("0xf69034 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE4initERKSH_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>>,std::string,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE14create_bucketsEm")]
// 0xf69044 - j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE14create_bucketsEm
// type: 
pub fn stub_0xf69044() -> ! {
    todo!("0xf69044 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>>,std::string,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE18reserve_for_insertEm")]
// 0xf69054 - j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE18reserve_for_insertEm
// type: 
pub fn stub_0xf69054() -> ! {
    todo!("0xf69054 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>>,std::string,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::~table()")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEED2Ev")]
// 0xf69064 - j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEED2Ev
// type: 
pub fn stub_0xf69064() -> ! {
    todo!("0xf69064 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEED2Ev")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>,std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_")]
// 0xf69074 - j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_
// type: 
pub fn stub_0xf69074() -> ! {
    todo!("0xf69074 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>>,std::string,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISI_EEEEmRKT_RKT0_")]
// 0xf69084 - j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISI_EEEEmRKT_RKT0_
// type: 
pub fn stub_0xf69084() -> ! {
    todo!("0xf69084 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsNS0_13unordered_mapISsN4Ogre9SharedPtrINS7_8ResourceEEENS_4hashISsEESt8equal_toISsESaIS4_IS5_SA_EEEEEESsSH_SC_SE_EEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISI_EEEEmRKT_RKT0_")
}

#[doc(alias = "std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>>::pair(std::pair<std::string const,boost::unordered::unordered_map<std::string,Ogre::SharedPtr<Ogre::Resource>,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::Resource>>>>> const&)")]
#[doc(alias = "j___ZNSt4pairIKSsN5boost9unordered13unordered_mapISsN4Ogre9SharedPtrINS4_8ResourceEEENS1_4hashISsEESt8equal_toISsESaIS_IS0_S7_EEEEEC2ERKSF_")]
// 0xf69094 - j___ZNSt4pairIKSsN5boost9unordered13unordered_mapISsN4Ogre9SharedPtrINS4_8ResourceEEENS1_4hashISsEESt8equal_toISsESaIS_IS0_S7_EEEEEC2ERKSF_
// type: 
pub fn stub_0xf69094() -> ! {
    todo!("0xf69094 j___ZNSt4pairIKSsN5boost9unordered13unordered_mapISsN4Ogre9SharedPtrINS4_8ResourceEEENS1_4hashISsEESt8equal_toISsESaIS_IS0_S7_EEEEEC2ERKSF_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManager12ResourcePoolEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
// 0xf690a4 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManager12ResourcePoolEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
// type: 
pub fn stub_0xf690a4() -> ! {
    todo!("0xf690a4 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManager12ResourcePoolEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,std::_Select1st<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS6_E")]
// 0xf690b4 - j___ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS6_E
// type: 
pub fn stub_0xf690b4() -> ! {
    todo!("0xf690b4 j___ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS6_E")
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,std::_Select1st<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
// 0xf690c4 - j___ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: 
pub fn stub_0xf690c4() -> ! {
    todo!("0xf690c4 j___ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,std::_Select1st<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
// 0xf690d4 - j___ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
// type: 
pub fn stub_0xf690d4() -> ! {
    todo!("0xf690d4 j___ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,std::_Select1st<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_")]
// 0xf690e4 - j___ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf690e4() -> ! {
    todo!("0xf690e4 j___ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_")
}

#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::operator=(Ogre::SharedPtr<Ogre::ControllerValue<float>> const&)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_15ControllerValueIfEEEaSERKS3_")]
// 0xf690f4 - j___ZN4Ogre9SharedPtrINS_15ControllerValueIfEEEaSERKS3_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
pub fn stub_0xf690f4() -> ! {
    todo!("0xf690f4 j___ZN4Ogre9SharedPtrINS_15ControllerValueIfEEEaSERKS3_")
}

#[doc(alias = "std::vector<Ogre::ColourValue,Ogre::STLAllocator<Ogre::ColourValue,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::ColourValue*,std::vector<Ogre::ColourValue,Ogre::STLAllocator<Ogre::ColourValue,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::ColourValue const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre11ColourValueENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_")]
// 0xf69104 - j___ZNSt6vectorIN4Ogre11ColourValueENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_
// type: 
pub fn stub_0xf69104() -> ! {
    todo!("0xf69104 j___ZNSt6vectorIN4Ogre11ColourValueENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_")
}

#[doc(alias = "std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<float *,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,float const&)")]
#[doc(alias = "j___ZNSt6vectorIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPfS6_EEmRKf")]
// 0xf69114 - j___ZNSt6vectorIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPfS6_EEmRKf
// type: int __fastcall(int, void *__src)
pub fn stub_0xf69114() -> ! {
    todo!("0xf69114 j___ZNSt6vectorIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPfS6_EEmRKf")
}

#[doc(alias = "std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Node const* const,unsigned long> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
// 0xf69124 - j___ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: int __fastcall(char *)
pub fn stub_0xf69124() -> ! {
    todo!("0xf69124 j___ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")
}

#[doc(alias = "std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::Node const* const,unsigned long>>,std::pair<Ogre::Node const* const,unsigned long> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_")]
// 0xf69134 - j___ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf69134() -> ! {
    todo!("0xf69134 j___ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_")
}

#[doc(alias = "std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Node const* const,unsigned long>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
// 0xf69144 - j___ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
// type: 
pub fn stub_0xf69144() -> ! {
    todo!("0xf69144 j___ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MovableObjectFactory *>,std::_Select1st<std::pair<std::string const,Ogre::MovableObjectFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObjectFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
#[doc(alias = "j___ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// 0xf69154 - j___ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
pub fn stub_0xf69154() -> ! {
    todo!("0xf69154 j___ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")
}

#[doc(alias = "std::_Deque_base<Ogre::SceneManager *,Ogre::STLAllocator<Ogre::SceneManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "j___ZNSt11_Deque_baseIPN4Ogre12SceneManagerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_initialize_mapEm")]
// 0xf69164 - j___ZNSt11_Deque_baseIPN4Ogre12SceneManagerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf69164() -> ! {
    todo!("0xf69164 j___ZNSt11_Deque_baseIPN4Ogre12SceneManagerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<unsigned long,Ogre::STLAllocator<unsigned long,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "j___ZNSt11_Deque_baseImN4Ogre12STLAllocatorImNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_initialize_mapEm")]
// 0xf69174 - j___ZNSt11_Deque_baseImN4Ogre12STLAllocatorImNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf69174() -> ! {
    todo!("0xf69174 j___ZNSt11_Deque_baseImN4Ogre12STLAllocatorImNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_initialize_mapEm")
}

#[doc(alias = "std::map<std::string,Ogre::MovableObjectFactory *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObjectFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsPN4Ogre20MovableObjectFactoryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
// 0xf69194 - j___ZNSt3mapISsPN4Ogre20MovableObjectFactoryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// type: 
pub fn stub_0xf69194() -> ! {
    todo!("0xf69194 j___ZNSt3mapISsPN4Ogre20MovableObjectFactoryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")
}

#[doc(alias = "std::deque<Ogre::SceneManager *,Ogre::STLAllocator<Ogre::SceneManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_reallocate_map(unsigned long,bool)")]
#[doc(alias = "j___ZNSt5dequeIPN4Ogre12SceneManagerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_reallocate_mapEmb")]
// 0xf691a4 - j___ZNSt5dequeIPN4Ogre12SceneManagerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_reallocate_mapEmb
// type: 
pub fn stub_0xf691a4() -> ! {
    todo!("0xf691a4 j___ZNSt5dequeIPN4Ogre12SceneManagerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::deque<unsigned long,Ogre::STLAllocator<unsigned long,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_reallocate_map(unsigned long,bool)")]
#[doc(alias = "j___ZNSt5dequeImN4Ogre12STLAllocatorImNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_reallocate_mapEmb")]
// 0xf691b4 - j___ZNSt5dequeImN4Ogre12STLAllocatorImNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_reallocate_mapEmb
// type: 
pub fn stub_0xf691b4() -> ! {
    todo!("0xf691b4 j___ZNSt5dequeImN4Ogre12STLAllocatorImNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::deque<unsigned long,Ogre::STLAllocator<unsigned long,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>)")]
#[doc(alias = "j___ZNSt5dequeImN4Ogre12STLAllocatorImNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt15_Deque_iteratorImRmPmESA_")]
// 0xf691c4 - j___ZNSt5dequeImN4Ogre12STLAllocatorImNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt15_Deque_iteratorImRmPmESA_
// type: 
pub fn stub_0xf691c4() -> ! {
    todo!("0xf691c4 j___ZNSt5dequeImN4Ogre12STLAllocatorImNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt15_Deque_iteratorImRmPmESA_")
}

#[doc(alias = "std::vector<Ogre::RenderSystem *,Ogre::STLAllocator<Ogre::RenderSystem *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderSystem **,std::vector<Ogre::RenderSystem *,Ogre::STLAllocator<Ogre::RenderSystem *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderSystem * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre12RenderSystemENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// 0xf691e4 - j___ZNSt6vectorIPN4Ogre12RenderSystemENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf691e4() -> ! {
    todo!("0xf691e4 j___ZNSt6vectorIPN4Ogre12RenderSystemENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")
}

#[doc(alias = "std::vector<Ogre::DynLib *,Ogre::STLAllocator<Ogre::DynLib *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::DynLib **,std::vector<Ogre::DynLib *,Ogre::STLAllocator<Ogre::DynLib *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::DynLib * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre6DynLibENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// 0xf691f4 - j___ZNSt6vectorIPN4Ogre6DynLibENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf691f4() -> ! {
    todo!("0xf691f4 j___ZNSt6vectorIPN4Ogre6DynLibENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")
}

#[doc(alias = "std::vector<Ogre::Plugin *,Ogre::STLAllocator<Ogre::Plugin *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Plugin **,std::vector<Ogre::Plugin *,Ogre::STLAllocator<Ogre::Plugin *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Plugin * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre6PluginENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// 0xf69204 - j___ZNSt6vectorIPN4Ogre6PluginENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf69204() -> ! {
    todo!("0xf69204 j___ZNSt6vectorIPN4Ogre6PluginENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")
}

#[doc(alias = "std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "j___ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_")]
// 0xf69214 - j___ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_
// type: int __fastcall(int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf69214() -> ! {
    todo!("0xf69214 j___ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_")
}

#[doc(alias = "std::_Rb_tree<Ogre::FrameListener *,Ogre::FrameListener *,std::_Identity<Ogre::FrameListener *>,std::less<Ogre::FrameListener *>,Ogre::STLAllocator<Ogre::FrameListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::FrameListener * const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre13FrameListenerES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
// 0xf69224 - j___ZNSt8_Rb_treeIPN4Ogre13FrameListenerES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(char *)
pub fn stub_0xf69224() -> ! {
    todo!("0xf69224 j___ZNSt8_Rb_treeIPN4Ogre13FrameListenerES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<Ogre::FrameListener *,Ogre::FrameListener *,std::_Identity<Ogre::FrameListener *>,std::less<Ogre::FrameListener *>,Ogre::STLAllocator<Ogre::FrameListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::FrameListener *>,std::_Rb_tree_iterator<Ogre::FrameListener *>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre13FrameListenerES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_")]
// 0xf69234 - j___ZNSt8_Rb_treeIPN4Ogre13FrameListenerES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf69234() -> ! {
    todo!("0xf69234 j___ZNSt8_Rb_treeIPN4Ogre13FrameListenerES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_")
}

#[doc(alias = "std::_Rb_tree<Ogre::FrameListener *,Ogre::FrameListener *,std::_Identity<Ogre::FrameListener *>,std::less<Ogre::FrameListener *>,Ogre::STLAllocator<Ogre::FrameListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::FrameListener *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre13FrameListenerES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xf69244 - j___ZNSt8_Rb_treeIPN4Ogre13FrameListenerES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: 
pub fn stub_0xf69244() -> ! {
    todo!("0xf69244 j___ZNSt8_Rb_treeIPN4Ogre13FrameListenerES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MovableObjectFactory *>,std::_Select1st<std::pair<std::string const,Ogre::MovableObjectFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObjectFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::MovableObjectFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// 0xf69254 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf69254() -> ! {
    todo!("0xf69254 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MovableObjectFactory *>,std::_Select1st<std::pair<std::string const,Ogre::MovableObjectFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObjectFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::MovableObjectFactory *>>,std::pair<std::string const,Ogre::MovableObjectFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// 0xf69264 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf69264() -> ! {
    todo!("0xf69264 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MovableObjectFactory *>,std::_Select1st<std::pair<std::string const,Ogre::MovableObjectFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObjectFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// 0xf69274 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
pub fn stub_0xf69274() -> ! {
    todo!("0xf69274 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MovableObjectFactory *>,std::_Select1st<std::pair<std::string const,Ogre::MovableObjectFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObjectFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::MovableObjectFactory *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf69284 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
pub fn stub_0xf69284() -> ! {
    todo!("0xf69284 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MovableObjectFactory *>,std::_Select1st<std::pair<std::string const,Ogre::MovableObjectFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObjectFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::MovableObjectFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// 0xf69294 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf69294() -> ! {
    todo!("0xf69294 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre20MovableObjectFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderQueueInvocationSequence *>,std::_Select1st<std::pair<std::string const,Ogre::RenderQueueInvocationSequence *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderQueueInvocationSequence *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre29RenderQueueInvocationSequenceEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// 0xf692a4 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre29RenderQueueInvocationSequenceEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
pub fn stub_0xf692a4() -> ! {
    todo!("0xf692a4 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre29RenderQueueInvocationSequenceEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderQueueInvocationSequence *>,std::_Select1st<std::pair<std::string const,Ogre::RenderQueueInvocationSequence *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderQueueInvocationSequence *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::RenderQueueInvocationSequence *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre29RenderQueueInvocationSequenceEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf692b4 - j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre29RenderQueueInvocationSequenceEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
pub fn stub_0xf692b4() -> ! {
    todo!("0xf692b4 j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre29RenderQueueInvocationSequenceEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::string * std::__uninitialized_copy_a<__gnu_cxx::__normal_iterator<std::string const*,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(__gnu_cxx::__normal_iterator<std::string const*,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<std::string const*,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::string *,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
#[doc(alias = "j___ZSt22__uninitialized_copy_aIN9__gnu_cxx17__normal_iteratorIPKSsSt6vectorISsN4Ogre12STLAllocatorISsNS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEEEPSsSA_ET0_T_SF_SE_T1_")]
// 0xf692c4 - j___ZSt22__uninitialized_copy_aIN9__gnu_cxx17__normal_iteratorIPKSsSt6vectorISsN4Ogre12STLAllocatorISsNS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEEEPSsSA_ET0_T_SF_SE_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf692c4() -> ! {
    todo!("0xf692c4 j___ZSt22__uninitialized_copy_aIN9__gnu_cxx17__normal_iteratorIPKSsSt6vectorISsN4Ogre12STLAllocatorISsNS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEEEPSsSA_ET0_T_SF_SE_T1_")
}

#[doc(alias = "std::vector<Ogre::Quaternion,Ogre::STLAllocator<Ogre::Quaternion,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Quaternion*,std::vector<Ogre::Quaternion,Ogre::STLAllocator<Ogre::Quaternion,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Quaternion const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre10QuaternionENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
// 0xf692d4 - j___ZNSt6vectorIN4Ogre10QuaternionENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: 
pub fn stub_0xf692d4() -> ! {
    todo!("0xf692d4 j___ZNSt6vectorIN4Ogre10QuaternionENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")
}

#[doc(alias = "std::vector<Ogre::Quaternion,Ogre::STLAllocator<Ogre::Quaternion,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Quaternion*,std::vector<Ogre::Quaternion,Ogre::STLAllocator<Ogre::Quaternion,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Quaternion const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre10QuaternionENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_")]
// 0xf692e4 - j___ZNSt6vectorIN4Ogre10QuaternionENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_
// type: 
pub fn stub_0xf692e4() -> ! {
    todo!("0xf692e4 j___ZNSt6vectorIN4Ogre10QuaternionENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_")
}

#[doc(alias = "Ogre::NameGenerator::generate(void)")]
#[doc(alias = "j___ZN4Ogre13NameGenerator8generateEv")]
// 0xf692f4 - j___ZN4Ogre13NameGenerator8generateEv
// type: _DWORD __fastcall(Ogre::NameGenerator *__hidden this)
pub fn stub_0xf692f4() -> ! {
    todo!("0xf692f4 j___ZN4Ogre13NameGenerator8generateEv")
}
