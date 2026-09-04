//! core bg28 — 100 core stubs EA-sorted asc distinct not in /tmp/global_eas.txt.
//! Source: ida/export.json (85545 funcs) next 100 uncovered 0xf67c04..0xf682d4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::vector<Ogre::LogListener *,Ogre::STLAllocator<Ogre::LogListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::LogListener **,std::vector<Ogre::LogListener *,Ogre::STLAllocator<Ogre::LogListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LogListener * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre11LogListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// 0xf67c04 — j___ZNSt6vectorIPN4Ogre11LogListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf67c04() {
    // IDA 0xf67c04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "OgreRbxMutex::OgreRbxMutex(int)")]
#[doc(alias = "j___ZN12OgreRbxMutexC2Ei")]
// 0xf67c14 — j___ZN12OgreRbxMutexC2Ei
// type: OgreRbxMutex *__fastcall(OgreRbxMutex *__hidden this, int)
pub fn stub_0xf67c14() {
    // IDA 0xf67c14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Log *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre3LogEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// 0xf67c24 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre3LogEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf67c24() {
    // IDA 0xf67c24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Log *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre3LogEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf67c34 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre3LogEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf67c34() {
    // IDA 0xf67c34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Log *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre3LogEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// 0xf67c44 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre3LogEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf67c44() {
    // IDA 0xf67c44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<Ogre::ManualObject::ManualObjectSection *,Ogre::STLAllocator<Ogre::ManualObject::ManualObjectSection *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ManualObject::ManualObjectSection **,std::vector<Ogre::ManualObject::ManualObjectSection *,Ogre::STLAllocator<Ogre::ManualObject::ManualObjectSection *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ManualObject::ManualObjectSection * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre12ManualObject19ManualObjectSectionENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_")]
// 0xf67c54 — j___ZNSt6vectorIPN4Ogre12ManualObject19ManualObjectSectionENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf67c54() {
    // IDA 0xf67c54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<Ogre::Technique *,Ogre::STLAllocator<Ogre::Technique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique **,std::vector<Ogre::Technique *,Ogre::STLAllocator<Ogre::Technique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre9TechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// 0xf67c64 — j___ZNSt6vectorIPN4Ogre9TechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf67c64() {
    // IDA 0xf67c64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Technique *>,std::_Select1st<std::pair<unsigned short const,Ogre::Technique *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::Technique *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre9TechniqueEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// 0xf67c74 — j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre9TechniqueEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(char *)
pub fn stub_0xf67c74() {
    // IDA 0xf67c74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Technique *>,std::_Select1st<std::pair<unsigned short const,Ogre::Technique *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::Technique *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre9TechniqueEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf67c84 — j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre9TechniqueEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
pub fn stub_0xf67c84() {
    // IDA 0xf67c84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE16_M_insert_uniqueERKSG_")]
// 0xf67c94 — j___ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE16_M_insert_uniqueERKSG_
// type: int __fastcall(char *)
pub fn stub_0xf67c94() {
    // IDA 0xf67c94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISG_ERKSG_")]
// 0xf67ca4 — j___ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISG_ERKSG_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf67ca4() {
    // IDA 0xf67ca4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE8_M_eraseEPSt13_Rb_tree_nodeISG_E")]
// 0xf67cb4 — j___ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE8_M_eraseEPSt13_Rb_tree_nodeISG_E
// type: 
pub fn stub_0xf67cb4() {
    // IDA 0xf67cb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<std::string,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsSt4listIPN4Ogre15MaterialManager8ListenerENS1_12STLAllocatorIS4_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEESt4lessISsESaISt4pairIKSsSA_EEEixERSE_")]
// 0xf67cc4 — j___ZNSt3mapISsSt4listIPN4Ogre15MaterialManager8ListenerENS1_12STLAllocatorIS4_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEESt4lessISsESaISt4pairIKSsSA_EEEixERSE_
// type: 
pub fn stub_0xf67cc4() {
    // IDA 0xf67cc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<std::string,unsigned short,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISstSt4lessISsEN4Ogre12STLAllocatorISt4pairIKSstENS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEixERS5_")]
// 0xf67cd4 — j___ZNSt3mapISstSt4lessISsEN4Ogre12STLAllocatorISt4pairIKSstENS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEixERS5_
// type: 
pub fn stub_0xf67cd4() {
    // IDA 0xf67cd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::pair(std::string const&,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "j___ZNSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEC2ERS0_RKSB_")]
// 0xf67ce4 — j___ZNSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEC2ERS0_RKSB_
// type: 
pub fn stub_0xf67ce4() {
    // IDA 0xf67ce4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_create_node(std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE14_M_create_nodeERKSD_")]
// 0xf67cf4 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE14_M_create_nodeERKSD_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf67cf4() {
    // IDA 0xf67cf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_insert_unique(std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE16_M_insert_uniqueERKSD_")]
// 0xf67d04 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE16_M_insert_uniqueERKSD_
// type: 
pub fn stub_0xf67d04() {
    // IDA 0xf67d04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISD_ERKSD_")]
// 0xf67d14 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISD_ERKSD_
// type: 
pub fn stub_0xf67d14() {
    // IDA 0xf67d14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE4findERS1_")]
// 0xf67d24 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE4findERS1_
// type: 
pub fn stub_0xf67d24() {
    // IDA 0xf67d24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE8_M_eraseEPSt13_Rb_tree_nodeISD_E")]
// 0xf67d34 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE8_M_eraseEPSt13_Rb_tree_nodeISD_E
// type: 
pub fn stub_0xf67d34() {
    // IDA 0xf67d34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE9_M_insertEPSt18_Rb_tree_node_baseSL_RKSD_")]
// 0xf67d44 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE9_M_insertEPSt18_Rb_tree_node_baseSL_RKSD_
// type: 
pub fn stub_0xf67d44() {
    // IDA 0xf67d44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,unsigned short> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
// 0xf67d54 — j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf67d54() {
    // IDA 0xf67d54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,unsigned short>>,std::pair<std::string const,unsigned short> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")]
// 0xf67d64 — j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf67d64() {
    // IDA 0xf67d64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE4findERS1_")]
// 0xf67d74 — j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
pub fn stub_0xf67d74() {
    // IDA 0xf67d74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned short>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xf67d84 — j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: 
pub fn stub_0xf67d84() {
    // IDA 0xf67d84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,unsigned short> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS2_")]
// 0xf67d94 — j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf67d94() {
    // IDA 0xf67d94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "Ogre::MaterialSerializer::~MaterialSerializer()")]
#[doc(alias = "j___ZN4Ogre18MaterialSerializerD2Ev")]
// 0xf67da4 — j___ZN4Ogre18MaterialSerializerD2Ev
// type: void __fastcall(Ogre::MaterialSerializer *__hidden this)
pub fn stub_0xf67da4() {
    // IDA 0xf67da4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "Ogre::MeshLodUsage::~MeshLodUsage()")]
#[doc(alias = "j___ZN4Ogre12MeshLodUsageD2Ev")]
// 0xf67df4 — j___ZN4Ogre12MeshLodUsageD2Ev
// type: void __fastcall(Ogre::MeshLodUsage *__hidden this)
pub fn stub_0xf67df4() {
    // IDA 0xf67df4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::construct(Ogre::MeshLodUsage*,Ogre::MeshLodUsage const&)")]
#[doc(alias = "j___ZN4Ogre12STLAllocatorINS_12MeshLodUsageENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE9constructEPS1_RKS1_")]
// 0xf67e04 — j___ZN4Ogre12STLAllocatorINS_12MeshLodUsageENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE9constructEPS1_RKS1_
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf67e04() {
    // IDA 0xf67e04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "Ogre::Log::Stream::~Stream()")]
#[doc(alias = "j___ZN4Ogre3Log6StreamD2Ev")]
// 0xf67e14 — j___ZN4Ogre3Log6StreamD2Ev
// type: void __fastcall(Ogre::Log::Stream *__hidden this)
pub fn stub_0xf67e14() {
    // IDA 0xf67e14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "Ogre::EdgeData::~EdgeData()")]
#[doc(alias = "j___ZN4Ogre8EdgeDataD2Ev")]
// 0xf67e24 — j___ZN4Ogre8EdgeDataD2Ev
// type: void __fastcall(Ogre::EdgeData *__hidden this)
pub fn stub_0xf67e24() {
    // IDA 0xf67e24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "Ogre::SharedPtr<Ogre::DataStream>::operator=(Ogre::SharedPtr<Ogre::DataStream> const&)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_10DataStreamEEaSERKS2_")]
// 0xf67e34 — j___ZN4Ogre9SharedPtrINS_10DataStreamEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
pub fn stub_0xf67e34() {
    // IDA 0xf67e34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::~SharedPtr()")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEED1Ev")]
// 0xf67e44 — j___ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEED1Ev
// type: 
pub fn stub_0xf67e44() {
    // IDA 0xf67e44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwareVertexBuffer> const&)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEEaSERKS2_")]
// 0xf67e54 — j___ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
pub fn stub_0xf67e54() {
    // IDA 0xf67e54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::operator=(Ogre::SharedPtr<Ogre::Mesh> const&)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_4MeshEEaSERKS2_")]
// 0xf67e64 — j___ZN4Ogre9SharedPtrINS_4MeshEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
pub fn stub_0xf67e64() {
    // IDA 0xf67e64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "Ogre::SharedPtr<Ogre::Skeleton>::operator=(Ogre::SharedPtr<Ogre::Skeleton> const&)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_8SkeletonEEaSERKS2_")]
// 0xf67e74 — j___ZN4Ogre9SharedPtrINS_8SkeletonEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
pub fn stub_0xf67e74() {
    // IDA 0xf67e74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<std::string,Ogre::Animation *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Animation *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsPN4Ogre9AnimationESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
// 0xf67ee4 — j___ZNSt3mapISsPN4Ogre9AnimationESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf67ee4() {
    // IDA 0xf67ee4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage*,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
// 0xf67ef4 — j___ZNSt6vectorIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: 
pub fn stub_0xf67ef4() {
    // IDA 0xf67ef4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Pose **,std::vector<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Pose * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre4PoseENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// 0xf67f04 — j___ZNSt6vectorIPN4Ogre4PoseENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf67f04() {
    // IDA 0xf67f04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SubMesh **,std::vector<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SubMesh * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre7SubMeshENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// 0xf67f14 — j___ZNSt6vectorIPN4Ogre7SubMeshENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf67f14() {
    // IDA 0xf67f14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<Ogre::IndexData *,Ogre::STLAllocator<Ogre::IndexData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::IndexData **,std::vector<Ogre::IndexData *,Ogre::STLAllocator<Ogre::IndexData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::IndexData * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre9IndexDataENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_")]
// 0xf67f24 — j___ZNSt6vectorIPN4Ogre9IndexDataENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf67f24() {
    // IDA 0xf67f24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned short *,std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,unsigned short const&)")]
#[doc(alias = "j___ZNSt6vectorItN4Ogre12STLAllocatorItNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPtS6_EEmRKt")]
// 0xf67f34 — j___ZNSt6vectorItN4Ogre12STLAllocatorItNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPtS6_EEmRKt
// type: int __fastcall(int, void *__src)
pub fn stub_0xf67f34() {
    // IDA 0xf67f34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "j___ZNSt6vectorItN4Ogre12STLAllocatorItNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_")]
// 0xf67f44 — j___ZNSt6vectorItN4Ogre12STLAllocatorItNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_
// type: 
pub fn stub_0xf67f44() {
    // IDA 0xf67f44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Animation *>,std::_Select1st<std::pair<std::string const,Ogre::Animation *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Animation *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Animation *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre9AnimationEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// 0xf67f54 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre9AnimationEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf67f54() {
    // IDA 0xf67f54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Animation *>,std::_Select1st<std::pair<std::string const,Ogre::Animation *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Animation *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre9AnimationEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// 0xf67f64 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre9AnimationEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf67f64() {
    // IDA 0xf67f64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,std::_Select1st<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIfSt4pairIKfSt17_Rb_tree_iteratorIS0_IKmN4Ogre22VertexBoneAssignment_sEEEESt10_Select1stIS8_ESt4lessIfENS4_12STLAllocatorIS8_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// 0xf67f74 — j___ZNSt8_Rb_treeIfSt4pairIKfSt17_Rb_tree_iteratorIS0_IKmN4Ogre22VertexBoneAssignment_sEEEESt10_Select1stIS8_ESt4lessIfENS4_12STLAllocatorIS8_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: 
pub fn stub_0xf67f74() {
    // IDA 0xf67f74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre22VertexBoneAssignment_sEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// 0xf67f84 — j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre22VertexBoneAssignment_sEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: 
pub fn stub_0xf67f84() {
    // IDA 0xf67f84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<unsigned short> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIttSt9_IdentityItESt4lessItEN4Ogre12STLAllocatorItNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeItE")]
// 0xf67f94 — j___ZNSt8_Rb_treeIttSt9_IdentityItESt4lessItEN4Ogre12STLAllocatorItNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeItE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf67f94() {
    // IDA 0xf67f94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::~SharedPtr()")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEED1Ev")]
// 0xf67fa4 — j___ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEED1Ev
// type: int __fastcall(_DWORD)
pub fn stub_0xf67fa4() {
    // IDA 0xf67fa4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwareIndexBuffer> const&)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEEaSERKS2_")]
// 0xf67fb4 — j___ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
pub fn stub_0xf67fb4() {
    // IDA 0xf67fb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::~SharedPtr()")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_4MeshEED1Ev")]
// 0xf67fc4 — j___ZN4Ogre9SharedPtrINS_4MeshEED1Ev
// type: int __fastcall(_DWORD)
pub fn stub_0xf67fc4() {
    // IDA 0xf67fc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "Ogre::Matrix4::concatenate(Ogre::Matrix4 const&)const")]
#[doc(alias = "j___ZNK4Ogre7Matrix411concatenateERKS0_")]
// 0xf67fd4 — j___ZNK4Ogre7Matrix411concatenateERKS0_
// type: _DWORD __fastcall(Ogre::Matrix4 *__hidden this, const Ogre::Matrix4 *)
pub fn stub_0xf67fd4() {
    // IDA 0xf67fd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<Ogre::Resource *,Ogre::MeshManager::MeshBuildParams,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](Ogre::Resource * const&)")]
#[doc(alias = "j___ZNSt3mapIPN4Ogre8ResourceENS0_11MeshManager15MeshBuildParamsESt4lessIS2_ENS0_12STLAllocatorISt4pairIKS2_S4_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS9_")]
// 0xf67fe4 — j___ZNSt3mapIPN4Ogre8ResourceENS0_11MeshManager15MeshBuildParamsESt4lessIS2_ENS0_12STLAllocatorISt4pairIKS2_S4_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS9_
// type: 
pub fn stub_0xf67fe4() {
    // IDA 0xf67fe4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_")]
// 0xf67ff4 — j___ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int)
pub fn stub_0xf67ff4() {
    // IDA 0xf67ff4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
// 0xf68004 — j___ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, int, int)
pub fn stub_0xf68004() {
    // IDA 0xf68004: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// 0xf68014 — j___ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: 
pub fn stub_0xf68014() {
    // IDA 0xf68014: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKS7_")]
// 0xf68024 — j___ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKS7_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf68024() {
    // IDA 0xf68024: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MeshSerializer::MeshVersionData **,std::vector<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshSerializer::MeshVersionData * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre14MeshSerializer15MeshVersionDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_")]
// 0xf68034 — j___ZNSt6vectorIPN4Ogre14MeshSerializer15MeshVersionDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf68034() {
    // IDA 0xf68034: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<unsigned short,std::string,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](unsigned short const&)")]
#[doc(alias = "j___ZNSt3mapItSsSt4lessItEN4Ogre12STLAllocatorISt4pairIKtSsENS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEixERS5_")]
// 0xf68044 — j___ZNSt3mapItSsSt4lessItEN4Ogre12STLAllocatorISt4pairIKtSsENS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEixERS5_
// type: 
pub fn stub_0xf68044() {
    // IDA 0xf68044: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::MeshLodUsage const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_")]
// 0xf68054 — j___ZNSt6vectorIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf68054() {
    // IDA 0xf68054: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector4*,std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>>,unsigned long,Ogre::Vector4 const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_")]
// 0xf68064 — j___ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_
// type: 
pub fn stub_0xf68064() {
    // IDA 0xf68064: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::EdgeData::Edge*,std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::EdgeData::Edge const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_")]
// 0xf68074 — j___ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: 
pub fn stub_0xf68074() {
    // IDA 0xf68074: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::EdgeData::Triangle*,std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::EdgeData::Triangle const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_")]
// 0xf68084 — j___ZNSt6vectorIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: 
pub fn stub_0xf68084() {
    // IDA 0xf68084: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::string> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
// 0xf68094 — j___ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf68094() {
    // IDA 0xf68094: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,std::string>>,std::pair<unsigned short const,std::string> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")]
// 0xf680a4 — j___ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf680a4() {
    // IDA 0xf680a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::string>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xf680b4 — j___ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: 
pub fn stub_0xf680b4() {
    // IDA 0xf680b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,std::string> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS2_")]
// 0xf680c4 — j___ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf680c4() {
    // IDA 0xf680c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::destroy(void)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_8MaterialEE7destroyEv")]
// 0xf680d4 — j___ZN4Ogre9SharedPtrINS_8MaterialEE7destroyEv
// type: 
pub fn stub_0xf680d4() {
    // IDA 0xf680d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::Node *>>,std::string,Ogre::Node *,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::Node *>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::Node *>> *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeIS9_EESJ_")]
// 0xf680e4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeIS9_EESJ_
// type: 
pub fn stub_0xf680e4() {
    // IDA 0xf680e4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::Node *>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::Node *>>,std::string,Ogre::Node *,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<std::pair<std::string const,Ogre::Node *>>(std::string const&,std::pair<std::string const,Ogre::Node *> &&)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEE12emplace_implIJS9_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEbERS5_DpOT_")]
// 0xf680f4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEE12emplace_implIJS9_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEbERS5_DpOT_
// type: int __fastcall(int, int, int, int, char, char, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf680f4() {
    // IDA 0xf680f4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::Node *>>>>::construct_with_value<std::pair<std::string const,Ogre::Node *>>(std::pair<std::string const,Ogre::Node *> &&)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN4Ogre4NodeEEEEEE20construct_with_valueIJS9_EEEvDpOT_")]
// 0xf68104 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN4Ogre4NodeEEEEEE20construct_with_valueIJS9_EEEvDpOT_
// type: int __fastcall(int, std::string *)
pub fn stub_0xf68104() {
    // IDA 0xf68104: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::Node *>>,std::string,Ogre::Node *,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")]
// 0xf68114 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// type: 
pub fn stub_0xf68114() {
    // IDA 0xf68114: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::Node *>>,std::string,Ogre::Node *,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")]
// 0xf68124 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: 
pub fn stub_0xf68124() {
    // IDA 0xf68124: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::Node *>>,std::string,Ogre::Node *,boost::hash<std::string>,std::equal_to<std::string>>>::~table()")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEED2Ev")]
// 0xf68134 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEED2Ev
// type: 
pub fn stub_0xf68134() {
    // IDA 0xf68134: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,Ogre::Node *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,Ogre::Node *>>,std::string,Ogre::Node *,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_")]
// 0xf68144 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4Ogre4NodeEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_
// type: 
pub fn stub_0xf68144() {
    // IDA 0xf68144: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<Ogre::Node *,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Node **,std::vector<Ogre::Node *,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Node * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre4NodeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// 0xf68154 — j___ZNSt6vectorIPN4Ogre4NodeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf68154() {
    // IDA 0xf68154: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<Ogre::Node *,Ogre::Node *,std::_Identity<Ogre::Node *>,std::less<Ogre::Node *>,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Node * const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre4NodeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
// 0xf68164 — j___ZNSt8_Rb_treeIPN4Ogre4NodeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(char *)
pub fn stub_0xf68164() {
    // IDA 0xf68164: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<Ogre::Node *,Ogre::Node *,std::_Identity<Ogre::Node *>,std::less<Ogre::Node *>,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Node *>,std::_Rb_tree_iterator<Ogre::Node *>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre4NodeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_")]
// 0xf68174 — j___ZNSt8_Rb_treeIPN4Ogre4NodeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf68174() {
    // IDA 0xf68174: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<Ogre::Node *,Ogre::Node *,std::_Identity<Ogre::Node *>,std::less<Ogre::Node *>,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Node *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre4NodeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xf68184 — j___ZNSt8_Rb_treeIPN4Ogre4NodeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: 
pub fn stub_0xf68184() {
    // IDA 0xf68184: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::Vector4>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// 0xf68194 — j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf68194() {
    // IDA 0xf68194: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::OverlayContainer *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// 0xf681a4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf681a4() {
    // IDA 0xf681a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// 0xf681b4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
pub fn stub_0xf681b4() {
    // IDA 0xf681b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::OverlayContainer *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf681c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
pub fn stub_0xf681c4() {
    // IDA 0xf681c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayContainer *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayContainer *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayContainer *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::OverlayContainer *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// 0xf681d4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre16OverlayContainerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf681d4() {
    // IDA 0xf681d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<std::string,Ogre::OverlayElementFactory *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayElementFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsPN4Ogre21OverlayElementFactoryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
// 0xf681e4 — j___ZNSt3mapISsPN4Ogre21OverlayElementFactoryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// type: 
pub fn stub_0xf681e4() {
    // IDA 0xf681e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<std::string,Ogre::Overlay *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Overlay *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsPN4Ogre7OverlayESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
// 0xf681f4 — j___ZNSt3mapISsPN4Ogre7OverlayESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// type: 
pub fn stub_0xf681f4() {
    // IDA 0xf681f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE4findERKSs")]
// 0xf68204 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE4findERKSs
// type: 
pub fn stub_0xf68204() {
    // IDA 0xf68204: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::string> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeISsE")]
// 0xf68214 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeISsE
// type: 
pub fn stub_0xf68214() {
    // IDA 0xf68214: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSC_RKSs")]
// 0xf68224 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsEN4Ogre12STLAllocatorISsNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSC_RKSs
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf68224() {
    // IDA 0xf68224: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayElement *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayElement *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayElement *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::OverlayElement *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14OverlayElementEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// 0xf68234 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14OverlayElementEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf68234() {
    // IDA 0xf68234: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayElement *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayElement *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayElement *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14OverlayElementEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// 0xf68244 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14OverlayElementEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
pub fn stub_0xf68244() {
    // IDA 0xf68244: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayElement *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayElement *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayElement *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::OverlayElement *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14OverlayElementEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf68254 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14OverlayElementEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
pub fn stub_0xf68254() {
    // IDA 0xf68254: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayElement *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayElement *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayElement *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::OverlayElement *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14OverlayElementEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// 0xf68264 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14OverlayElementEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf68264() {
    // IDA 0xf68264: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayElementFactory *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayElementFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayElementFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::OverlayElementFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21OverlayElementFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// 0xf68274 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21OverlayElementFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf68274() {
    // IDA 0xf68274: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayElementFactory *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayElementFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayElementFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::OverlayElementFactory *>>,std::pair<std::string const,Ogre::OverlayElementFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21OverlayElementFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// 0xf68284 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21OverlayElementFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf68284() {
    // IDA 0xf68284: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayElementFactory *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayElementFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayElementFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21OverlayElementFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// 0xf68294 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21OverlayElementFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
pub fn stub_0xf68294() {
    // IDA 0xf68294: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayElementFactory *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayElementFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayElementFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::OverlayElementFactory *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21OverlayElementFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf682a4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21OverlayElementFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
pub fn stub_0xf682a4() {
    // IDA 0xf682a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::OverlayElementFactory *>,std::_Select1st<std::pair<std::string const,Ogre::OverlayElementFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::OverlayElementFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::OverlayElementFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21OverlayElementFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// 0xf682b4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21OverlayElementFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
pub fn stub_0xf682b4() {
    // IDA 0xf682b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Overlay *>,std::_Select1st<std::pair<std::string const,Ogre::Overlay *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Overlay *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Overlay *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7OverlayEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// 0xf682c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7OverlayEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf682c4() {
    // IDA 0xf682c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Overlay *>,std::_Select1st<std::pair<std::string const,Ogre::Overlay *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Overlay *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Overlay *>>,std::pair<std::string const,Ogre::Overlay *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7OverlayEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// 0xf682d4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7OverlayEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf682d4() {
    // IDA 0xf682d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
