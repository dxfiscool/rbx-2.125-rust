//! Auto-generated skeletons for rbx-network — filler EA-sorted ascending global remaining
//! Filter: RakNet|RBX::Network (case-insensitive) -> 4479 funcs, 4479 already stubbed (0 remaining before batch); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0xc76aa0..0xc7cd90 | existing 13929 -> 14029 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xc76aa0 — __ZN4Ogre10ConfigFileD2Ev
// type: void __fastcall(Ogre::ConfigFile *__hidden this)
#[doc(alias = "Ogre::ConfigFile::~ConfigFile()")]
pub fn stub_c76aa0() -> ! {
    todo!("0xc76aa0 Ogre::ConfigFile::~ConfigFile()")
}

// 0xc76bac — __ZN4Ogre10ConfigFile5clearEv
// type: _DWORD __fastcall(Ogre::ConfigFile *__hidden this)
#[doc(alias = "Ogre::ConfigFile::clear(void)")]
pub fn stub_c76bac() -> ! {
    todo!("0xc76bac Ogre::ConfigFile::clear(void)")
}

// 0xc76c8c — __ZN4Ogre10ConfigFile4loadERKSsS2_b
// type: _DWORD __fastcall(Ogre::ConfigFile *__hidden this, const std::string *, const std::string *, bool)
#[doc(alias = "Ogre::ConfigFile::load(std::string const&,std::string const&,bool)")]
pub fn stub_c76c8c() -> ! {
    todo!("0xc76c8c Ogre::ConfigFile::load(std::string const&,std::string const&,bool)")
}

// 0xc76c98 — __ZN4Ogre10ConfigFile10loadDirectERKSsS2_b
// type: _DWORD __fastcall(Ogre::ConfigFile *__hidden this, const std::string *, const std::string *, bool)
#[doc(alias = "Ogre::ConfigFile::loadDirect(std::string const&,std::string const&,bool)")]
pub fn stub_c76c98() -> ! {
    todo!("0xc76c98 Ogre::ConfigFile::loadDirect(std::string const&,std::string const&,bool)")
}

// 0xc7727c — __ZN4Ogre10ConfigFile4loadERKNS_9SharedPtrINS_10DataStreamEEERKSsb
#[doc(alias = "Ogre::ConfigFile::load(Ogre::SharedPtr<Ogre::DataStream> const&,std::string const&,bool)")]
pub fn stub_c7727c() -> ! {
    todo!("0xc7727c Ogre::ConfigFile::load(Ogre::SharedPtr<Ogre::DataStream> const&,std::string const&,bool)")
}

// 0xc77cb4 — __ZNK4Ogre10ConfigFile10getSettingERKSsS2_S2_
// type: _DWORD __fastcall(Ogre::ConfigFile *__hidden this, const std::string *, const std::string *, const std::string *)
#[doc(alias = "Ogre::ConfigFile::getSetting(std::string const&,std::string const&,std::string const&)const")]
pub fn stub_c77cb4() -> ! {
    todo!("0xc77cb4 Ogre::ConfigFile::getSetting(std::string const&,std::string const&,std::string const&)const")
}

// 0xc77d00 — __ZNK4Ogre10ConfigFile15getMultiSettingERKSsS2_
// type: _DWORD __fastcall(Ogre::ConfigFile *__hidden this, const std::string *, const std::string *)
#[doc(alias = "Ogre::ConfigFile::getMultiSetting(std::string const&,std::string const&)const")]
pub fn stub_c77d00() -> ! {
    todo!("0xc77d00 Ogre::ConfigFile::getMultiSetting(std::string const&,std::string const&)const")
}

// 0xc77ee0 — __ZN4Ogre10ConfigFile19getSettingsIteratorERKSs
// type: _DWORD __fastcall(Ogre::ConfigFile *__hidden this, const std::string *)
#[doc(alias = "Ogre::ConfigFile::getSettingsIterator(std::string const&)")]
pub fn stub_c77ee0() -> ! {
    todo!("0xc77ee0 Ogre::ConfigFile::getSettingsIterator(std::string const&)")
}

// 0xc78128 — __ZNSt3mapISsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIKSsSsENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEES2_NS4_IS5_IS6_SD_ESA_EEEixERS6_
#[doc(alias = "std::map<std::string,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_c78128() -> ! {
    todo!("0xc78128 std::map<std::string,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")
}

// 0xc782e4 — __ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPSsS6_EERKSs
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::string const&)")]
pub fn stub_c782e4() -> ! {
    todo!("0xc782e4 std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::string const&)")
}

// 0xc78630 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE4findERS1_
// type: _DWORD *__fastcall(int, const void **)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_c78630() -> ! {
    todo!("0xc78630 std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")
}

// 0xc786d4 — __ZNKSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
pub fn stub_c786d4() -> ! {
    todo!("0xc786d4 std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")
}

// 0xc78778 — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_c78778() -> ! {
    todo!("0xc78778 std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")
}

// 0xc7881c — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISE_ERKSE_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
pub fn stub_c7881c() -> ! {
    todo!("0xc7881c std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")
}

// 0xc789fc — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE9_M_insertEPSt18_Rb_tree_node_baseSK_RKSE_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
pub fn stub_c789fc() -> ! {
    todo!("0xc789fc std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")
}

// 0xc78b50 — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE16_M_insert_uniqueERKSE_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
pub fn stub_c78b50() -> ! {
    todo!("0xc78b50 std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")
}

// 0xc78c34 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
pub fn stub_c78c34() -> ! {
    todo!("0xc78c34 std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")
}

// 0xc78c38 — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")]
pub fn stub_c78c38() -> ! {
    todo!("0xc78c38 std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")
}

// 0xc78cb0 — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE13_Rb_tree_implIS4_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
pub fn stub_c78cb0() -> ! {
    todo!("0xc78cb0 std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")
}

// 0xc78cb4 — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE13_Rb_tree_implIS4_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
pub fn stub_c78cb4() -> ! {
    todo!("0xc78cb4 std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")
}

// 0xc78cc0 — __GLOBAL__I_a_669
#[doc(alias = "global constructor keyed to_a_669")]
pub fn stub_c78cc0() -> ! {
    todo!("0xc78cc0 global constructor keyed to_a_669")
}

// 0xc78cf4 — __ZN4Ogre17ControllerManager12getSingletonEv
// type: _DWORD __fastcall(Ogre::ControllerManager *__hidden this)
#[doc(alias = "Ogre::ControllerManager::getSingleton(void)")]
pub fn stub_c78cf4() -> ! {
    todo!("0xc78cf4 Ogre::ControllerManager::getSingleton(void)")
}

// 0xc78d04 — __ZN4Ogre17ControllerManagerC1Ev
// type: _DWORD __fastcall(Ogre::ControllerManager *__hidden this)
#[doc(alias = "Ogre::ControllerManager::ControllerManager(void)")]
pub fn stub_c78d04() -> ! {
    todo!("0xc78d04 Ogre::ControllerManager::ControllerManager(void)")
}

// 0xc78d10 — __ZN4Ogre17ControllerManagerC2Ev
// type: _DWORD __fastcall(Ogre::ControllerManager *__hidden this)
#[doc(alias = "Ogre::ControllerManager::ControllerManager(void)")]
pub fn stub_c78d10() -> ! {
    todo!("0xc78d10 Ogre::ControllerManager::ControllerManager(void)")
}

// 0xc78f70 — __ZN4Ogre17ControllerManagerD1Ev
// type: void __fastcall(Ogre::ControllerManager *__hidden this)
#[doc(alias = "Ogre::ControllerManager::~ControllerManager()")]
pub fn stub_c78f70() -> ! {
    todo!("0xc78f70 Ogre::ControllerManager::~ControllerManager()")
}

// 0xc78f7c — __ZN4Ogre17ControllerManagerD2Ev
// type: void __fastcall(Ogre::ControllerManager *__hidden this)
#[doc(alias = "Ogre::ControllerManager::~ControllerManager()")]
pub fn stub_c78f7c() -> ! {
    todo!("0xc78f7c Ogre::ControllerManager::~ControllerManager()")
}

// 0xc79268 — __ZN4Ogre17ControllerManager36createFrameTimePassthroughControllerERKNS_9SharedPtrINS_15ControllerValueIfEEEE
// type: int __fastcall(int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::ControllerManager::createFrameTimePassthroughController(Ogre::SharedPtr<Ogre::ControllerValue<float>> const&)")]
pub fn stub_c79268() -> ! {
    todo!("0xc79268 Ogre::ControllerManager::createFrameTimePassthroughController(Ogre::SharedPtr<Ogre::ControllerValue<float>> const&)")
}

// 0xc79344 — __ZNK4Ogre17ControllerManager18getFrameTimeSourceEv
// type: _DWORD __fastcall(Ogre::ControllerManager *__hidden this)
#[doc(alias = "Ogre::ControllerManager::getFrameTimeSource(void)const")]
pub fn stub_c79344() -> ! {
    todo!("0xc79344 Ogre::ControllerManager::getFrameTimeSource(void)const")
}

// 0xc79348 — __ZN4Ogre17ControllerManager20updateAllControllersEv
// type: _DWORD __fastcall(Ogre::ControllerManager *__hidden this)
#[doc(alias = "Ogre::ControllerManager::updateAllControllers(void)")]
pub fn stub_c79348() -> ! {
    todo!("0xc79348 Ogre::ControllerManager::updateAllControllers(void)")
}

// 0xc793b8 — __ZN4Ogre17ControllerManager21createTextureAnimatorEPNS_16TextureUnitStateEf
// type: _DWORD __fastcall(Ogre::ControllerManager *__hidden this, Ogre::TextureUnitState *, float)
#[doc(alias = "Ogre::ControllerManager::createTextureAnimator(Ogre::TextureUnitState *,float)")]
pub fn stub_c793b8() -> ! {
    todo!("0xc793b8 Ogre::ControllerManager::createTextureAnimator(Ogre::TextureUnitState *,float)")
}

// 0xc7967c — __ZN4Ogre17ControllerManager23createTextureUVScrollerEPNS_16TextureUnitStateEf
// type: _DWORD __fastcall(Ogre::ControllerManager *__hidden this, Ogre::TextureUnitState *, float)
#[doc(alias = "Ogre::ControllerManager::createTextureUVScroller(Ogre::TextureUnitState *,float)")]
pub fn stub_c7967c() -> ! {
    todo!("0xc7967c Ogre::ControllerManager::createTextureUVScroller(Ogre::TextureUnitState *,float)")
}

// 0xc7998c — __ZN4Ogre17ControllerManager22createTextureUScrollerEPNS_16TextureUnitStateEf
// type: _DWORD __fastcall(Ogre::ControllerManager *__hidden this, Ogre::TextureUnitState *, float)
#[doc(alias = "Ogre::ControllerManager::createTextureUScroller(Ogre::TextureUnitState *,float)")]
pub fn stub_c7998c() -> ! {
    todo!("0xc7998c Ogre::ControllerManager::createTextureUScroller(Ogre::TextureUnitState *,float)")
}

// 0xc79c9c — __ZN4Ogre17ControllerManager22createTextureVScrollerEPNS_16TextureUnitStateEf
// type: _DWORD __fastcall(Ogre::ControllerManager *__hidden this, Ogre::TextureUnitState *, float)
#[doc(alias = "Ogre::ControllerManager::createTextureVScroller(Ogre::TextureUnitState *,float)")]
pub fn stub_c79c9c() -> ! {
    todo!("0xc79c9c Ogre::ControllerManager::createTextureVScroller(Ogre::TextureUnitState *,float)")
}

// 0xc79fac — __ZN4Ogre17ControllerManager20createTextureRotaterEPNS_16TextureUnitStateEf
// type: _DWORD __fastcall(Ogre::ControllerManager *__hidden this, Ogre::TextureUnitState *, float)
#[doc(alias = "Ogre::ControllerManager::createTextureRotater(Ogre::TextureUnitState *,float)")]
pub fn stub_c79fac() -> ! {
    todo!("0xc79fac Ogre::ControllerManager::createTextureRotater(Ogre::TextureUnitState *,float)")
}

// 0xc7a2b8 — __ZN4Ogre17ControllerManager28createTextureWaveTransformerEPNS_16TextureUnitStateENS1_20TextureTransformTypeENS_12WaveformTypeEffff
// type: int __fastcall(int, int, int, int, float, float, float, struct _Unwind_Exception *lpuexcpt, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::ControllerManager::createTextureWaveTransformer(Ogre::TextureUnitState *,Ogre::TextureUnitState::TextureTransformType,Ogre::WaveformType,float,float,float,float)")]
pub fn stub_c7a2b8() -> ! {
    todo!("0xc7a2b8 Ogre::ControllerManager::createTextureWaveTransformer(Ogre::TextureUnitState *,Ogre::TextureUnitState::TextureTransformType,Ogre::WaveformType,float,float,float,float)")
}

// 0xc7a7a8 — __ZN4Ogre17ControllerManager17destroyControllerEPNS_10ControllerIfEE
#[doc(alias = "Ogre::ControllerManager::destroyController(Ogre::Controller<float> *)")]
pub fn stub_c7a7a8() -> ! {
    todo!("0xc7a7a8 Ogre::ControllerManager::destroyController(Ogre::Controller<float> *)")
}

// 0xc7a804 — __ZNK4Ogre17ControllerManager14getElapsedTimeEv
// type: _DWORD __fastcall(Ogre::ControllerManager *__hidden this)
#[doc(alias = "Ogre::ControllerManager::getElapsedTime(void)const")]
pub fn stub_c7a804() -> ! {
    todo!("0xc7a804 Ogre::ControllerManager::getElapsedTime(void)const")
}

// 0xc7a810 — __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::~SharedPtr()")]
pub fn stub_c7a810() -> ! {
    todo!("0xc7a810 Ogre::SharedPtr<Ogre::ControllerValue<float>>::~SharedPtr()")
}

// 0xc7a900 — __ZN4Ogre9SharedPtrINS_18ControllerFunctionIfEEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerFunction<float>>::~SharedPtr()")]
pub fn stub_c7a900() -> ! {
    todo!("0xc7a900 Ogre::SharedPtr<Ogre::ControllerFunction<float>>::~SharedPtr()")
}

// 0xc7a9f0 — __ZN4Ogre9SharedPtrINS_18ControllerFunctionIfEEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerFunction<float>>::~SharedPtr()")]
pub fn stub_c7a9f0() -> ! {
    todo!("0xc7a9f0 Ogre::SharedPtr<Ogre::ControllerFunction<float>>::~SharedPtr()")
}

// 0xc7aae4 — __ZN4Ogre9SharedPtrINS_18ControllerFunctionIfEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerFunction<float>>::destroy(void)")]
pub fn stub_c7aae4() -> ! {
    todo!("0xc7aae4 Ogre::SharedPtr<Ogre::ControllerFunction<float>>::destroy(void)")
}

// 0xc7ab1c — __ZN4Ogre9SharedPtrINS_18ControllerFunctionIfEEE4swapERS3_
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerFunction<float>>::swap(Ogre::SharedPtr<Ogre::ControllerFunction<float>>&)")]
pub fn stub_c7ab1c() -> ! {
    todo!("0xc7ab1c Ogre::SharedPtr<Ogre::ControllerFunction<float>>::swap(Ogre::SharedPtr<Ogre::ControllerFunction<float>>&)")
}

// 0xc7ab38 — __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::~SharedPtr()")]
pub fn stub_c7ab38() -> ! {
    todo!("0xc7ab38 Ogre::SharedPtr<Ogre::ControllerValue<float>>::~SharedPtr()")
}

// 0xc7ac2c — __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::destroy(void)")]
pub fn stub_c7ac2c() -> ! {
    todo!("0xc7ac2c Ogre::SharedPtr<Ogre::ControllerValue<float>>::destroy(void)")
}

// 0xc7ac64 — __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEE4swapERS3_
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::swap(Ogre::SharedPtr<Ogre::ControllerValue<float>>&)")]
pub fn stub_c7ac64() -> ! {
    todo!("0xc7ac64 Ogre::SharedPtr<Ogre::ControllerValue<float>>::swap(Ogre::SharedPtr<Ogre::ControllerValue<float>>&)")
}

// 0xc7ac80 — __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E
#[doc(alias = "std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Controller<float> *> *)")]
pub fn stub_c7ac80() -> ! {
    todo!("0xc7ac80 std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Controller<float> *> *)")
}

// 0xc7aca8 — __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS3_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Controller<float> * const&)")]
pub fn stub_c7aca8() -> ! {
    todo!("0xc7aca8 std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Controller<float> * const&)")
}

// 0xc7ada0 — __ZN4Ogre10ControllerIfEC2ERKNS_9SharedPtrINS_15ControllerValueIfEEEES7_RKNS2_INS_18ControllerFunctionIfEEEE
#[doc(alias = "Ogre::Controller<float>::Controller(Ogre::SharedPtr<Ogre::ControllerValue<float>> const&,Ogre::SharedPtr<Ogre::ControllerValue<float>> const&,Ogre::SharedPtr<Ogre::ControllerFunction<float>> const&)")]
pub fn stub_c7ada0() -> ! {
    todo!("0xc7ada0 Ogre::Controller<float>::Controller(Ogre::SharedPtr<Ogre::ControllerValue<float>> const&,Ogre::SharedPtr<Ogre::ControllerValue<float>> const&,Ogre::SharedPtr<Ogre::ControllerFunction<float>> const&)")
}

// 0xc7ae5c — __ZN4Ogre10ControllerIfED1Ev
#[doc(alias = "Ogre::Controller<float>::~Controller()")]
pub fn stub_c7ae5c() -> ! {
    todo!("0xc7ae5c Ogre::Controller<float>::~Controller()")
}

// 0xc7ae68 — __ZN4Ogre10ControllerIfED0Ev
#[doc(alias = "Ogre::Controller<float>::~Controller()")]
pub fn stub_c7ae68() -> ! {
    todo!("0xc7ae68 Ogre::Controller<float>::~Controller()")
}

// 0xc7aef8 — __ZN4Ogre10ControllerIfED2Ev
#[doc(alias = "Ogre::Controller<float>::~Controller()")]
pub fn stub_c7aef8() -> ! {
    todo!("0xc7aef8 Ogre::Controller<float>::~Controller()")
}

// 0xc7b0cc — __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS7_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Controller<float> *>,false>::~_Rb_tree_impl()")]
pub fn stub_c7b0cc() -> ! {
    todo!("0xc7b0cc std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Controller<float> *>,false>::~_Rb_tree_impl()")
}

// 0xc7b0d0 — __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS7_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Controller<float> *>,false>::~_Rb_tree_impl()")]
pub fn stub_c7b0d0() -> ! {
    todo!("0xc7b0d0 std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Controller<float> *>,false>::~_Rb_tree_impl()")
}

// 0xc7b0dc — __GLOBAL__I_a_670
#[doc(alias = "global constructor keyed to_a_670")]
pub fn stub_c7b0dc() -> ! {
    todo!("0xc7b0dc global constructor keyed to_a_670")
}

// 0xc7b110 — __ZN4Ogre10ConvexBody15_initialisePoolEv
// type: _DWORD __fastcall(Ogre::ConvexBody *__hidden this)
#[doc(alias = "Ogre::ConvexBody::_initialisePool(void)")]
pub fn stub_c7b110() -> ! {
    todo!("0xc7b110 Ogre::ConvexBody::_initialisePool(void)")
}

// 0xc7b16c — __ZN4Ogre10ConvexBody12_destroyPoolEv
// type: _DWORD __fastcall(Ogre::ConvexBody *__hidden this)
#[doc(alias = "Ogre::ConvexBody::_destroyPool(void)")]
pub fn stub_c7b16c() -> ! {
    todo!("0xc7b16c Ogre::ConvexBody::_destroyPool(void)")
}

// 0xc7b1a0 — __ZNSt6vectorIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(alias = "std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
pub fn stub_c7b1a0() -> ! {
    todo!("0xc7b1a0 std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")
}

// 0xc7b234 — __ZNSt6vectorIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Polygon **,std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Polygon * const&)")]
pub fn stub_c7b234() -> ! {
    todo!("0xc7b234 std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Polygon **,std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Polygon * const&)")
}

// 0xc7b3dc — __ZNSt12_Vector_baseIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
pub fn stub_c7b3dc() -> ! {
    todo!("0xc7b3dc std::_Vector_base<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xc7b3e0 — __ZNSt12_Vector_baseIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
pub fn stub_c7b3e0() -> ! {
    todo!("0xc7b3e0 std::_Vector_base<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xc7b3ec — __GLOBAL__I_a_671
#[doc(alias = "global constructor keyed to_a_671")]
pub fn stub_c7b3ec() -> ! {
    todo!("0xc7b3ec global constructor keyed to_a_671")
}

// 0xc7b458 — __ZN4Ogre10DataStream7getLineEb
// type: _DWORD __fastcall(Ogre::DataStream *__hidden this, bool)
#[doc(alias = "Ogre::DataStream::getLine(bool)")]
pub fn stub_c7b458() -> ! {
    todo!("0xc7b458 Ogre::DataStream::getLine(bool)")
}

// 0xc7b630 — __ZN4Ogre10DataStream8readLineEPcmRKSs
// type: _DWORD __fastcall(Ogre::DataStream *__hidden this, char *, unsigned int, const std::string *)
#[doc(alias = "Ogre::DataStream::readLine(char *,unsigned long,std::string const&)")]
pub fn stub_c7b630() -> ! {
    todo!("0xc7b630 Ogre::DataStream::readLine(char *,unsigned long,std::string const&)")
}

// 0xc7b710 — __ZN4Ogre10DataStream8skipLineERKSs
// type: _DWORD __fastcall(Ogre::DataStream *__hidden this, const std::string *)
#[doc(alias = "Ogre::DataStream::skipLine(std::string const&)")]
pub fn stub_c7b710() -> ! {
    todo!("0xc7b710 Ogre::DataStream::skipLine(std::string const&)")
}

// 0xc7b78c — __ZN4Ogre10DataStream11getAsStringEv
// type: _DWORD __fastcall(Ogre::DataStream *__hidden this)
#[doc(alias = "Ogre::DataStream::getAsString(void)")]
pub fn stub_c7b78c() -> ! {
    todo!("0xc7b78c Ogre::DataStream::getAsString(void)")
}

// 0xc7b8c8 — __ZN4Ogre16MemoryDataStreamC1EPvmbb
// type: _DWORD __fastcall(Ogre::MemoryDataStream *__hidden this, void *, unsigned int, bool, bool)
#[doc(alias = "Ogre::MemoryDataStream::MemoryDataStream(void *,unsigned long,bool,bool)")]
pub fn stub_c7b8c8() -> ! {
    todo!("0xc7b8c8 Ogre::MemoryDataStream::MemoryDataStream(void *,unsigned long,bool,bool)")
}

// 0xc7b918 — __ZN4Ogre16MemoryDataStreamC1ERNS_9SharedPtrINS_10DataStreamEEEbb
#[doc(alias = "Ogre::MemoryDataStream::MemoryDataStream(Ogre::SharedPtr<Ogre::DataStream> &,bool,bool)")]
pub fn stub_c7b918() -> ! {
    todo!("0xc7b918 Ogre::MemoryDataStream::MemoryDataStream(Ogre::SharedPtr<Ogre::DataStream> &,bool,bool)")
}

// 0xc7b924 — __ZN4Ogre16MemoryDataStreamC2ERNS_9SharedPtrINS_10DataStreamEEEbb
#[doc(alias = "Ogre::MemoryDataStream::MemoryDataStream(Ogre::SharedPtr<Ogre::DataStream> &,bool,bool)")]
pub fn stub_c7b924() -> ! {
    todo!("0xc7b924 Ogre::MemoryDataStream::MemoryDataStream(Ogre::SharedPtr<Ogre::DataStream> &,bool,bool)")
}

// 0xc7bb48 — __ZN4Ogre16MemoryDataStreamC1ERKSsRKNS_9SharedPtrINS_10DataStreamEEEbb
#[doc(alias = "Ogre::MemoryDataStream::MemoryDataStream(std::string const&,Ogre::SharedPtr<Ogre::DataStream> const&,bool,bool)")]
pub fn stub_c7bb48() -> ! {
    todo!("0xc7bb48 Ogre::MemoryDataStream::MemoryDataStream(std::string const&,Ogre::SharedPtr<Ogre::DataStream> const&,bool,bool)")
}

// 0xc7bb60 — __ZN4Ogre16MemoryDataStreamC2ERKSsRKNS_9SharedPtrINS_10DataStreamEEEbb
#[doc(alias = "Ogre::MemoryDataStream::MemoryDataStream(std::string const&,Ogre::SharedPtr<Ogre::DataStream> const&,bool,bool)")]
pub fn stub_c7bb60() -> ! {
    todo!("0xc7bb60 Ogre::MemoryDataStream::MemoryDataStream(std::string const&,Ogre::SharedPtr<Ogre::DataStream> const&,bool,bool)")
}

// 0xc7bd9c — __ZN4Ogre16MemoryDataStreamC1Embb
// type: _DWORD __fastcall(Ogre::MemoryDataStream *__hidden this, unsigned int, bool, bool)
#[doc(alias = "Ogre::MemoryDataStream::MemoryDataStream(unsigned long,bool,bool)")]
pub fn stub_c7bd9c() -> ! {
    todo!("0xc7bd9c Ogre::MemoryDataStream::MemoryDataStream(unsigned long,bool,bool)")
}

// 0xc7bed4 — __ZN4Ogre16MemoryDataStreamC1ERKSsmbb
// type: _DWORD __fastcall(Ogre::MemoryDataStream *__hidden this, const std::string *, unsigned int, bool, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "Ogre::MemoryDataStream::MemoryDataStream(std::string const&,unsigned long,bool,bool)")]
pub fn stub_c7bed4() -> ! {
    todo!("0xc7bed4 Ogre::MemoryDataStream::MemoryDataStream(std::string const&,unsigned long,bool,bool)")
}

// 0xc7c010 — __ZN4Ogre16MemoryDataStreamD0Ev
// type: void __fastcall(Ogre::MemoryDataStream *__hidden this)
#[doc(alias = "Ogre::MemoryDataStream::~MemoryDataStream()")]
pub fn stub_c7c010() -> ! {
    todo!("0xc7c010 Ogre::MemoryDataStream::~MemoryDataStream()")
}

// 0xc7c0a0 — __ZN4Ogre16MemoryDataStreamD1Ev
// type: void __fastcall(Ogre::MemoryDataStream *__hidden this)
#[doc(alias = "Ogre::MemoryDataStream::~MemoryDataStream()")]
pub fn stub_c7c0a0() -> ! {
    todo!("0xc7c0a0 Ogre::MemoryDataStream::~MemoryDataStream()")
}

// 0xc7c0ac — __ZN4Ogre16MemoryDataStreamD2Ev
// type: void __fastcall(Ogre::MemoryDataStream *__hidden this)
#[doc(alias = "Ogre::MemoryDataStream::~MemoryDataStream()")]
pub fn stub_c7c0ac() -> ! {
    todo!("0xc7c0ac Ogre::MemoryDataStream::~MemoryDataStream()")
}

// 0xc7c200 — __ZN4Ogre16MemoryDataStream4readEPvm
// type: _DWORD __fastcall(Ogre::MemoryDataStream *__hidden this, void *__dst, unsigned int)
#[doc(alias = "Ogre::MemoryDataStream::read(void *,unsigned long)")]
pub fn stub_c7c200() -> ! {
    todo!("0xc7c200 Ogre::MemoryDataStream::read(void *,unsigned long)")
}

// 0xc7c234 — __ZN4Ogre16MemoryDataStream5writeEPKvm
// type: _DWORD __fastcall(Ogre::MemoryDataStream *__hidden this, const void *, unsigned int)
#[doc(alias = "Ogre::MemoryDataStream::write(void const*,unsigned long)")]
pub fn stub_c7c234() -> ! {
    todo!("0xc7c234 Ogre::MemoryDataStream::write(void const*,unsigned long)")
}

// 0xc7c278 — __ZN4Ogre16MemoryDataStream8readLineEPcmRKSs
// type: _DWORD __fastcall(Ogre::MemoryDataStream *__hidden this, char *, unsigned int, const std::string *)
#[doc(alias = "Ogre::MemoryDataStream::readLine(char *,unsigned long,std::string const&)")]
pub fn stub_c7c278() -> ! {
    todo!("0xc7c278 Ogre::MemoryDataStream::readLine(char *,unsigned long,std::string const&)")
}

// 0xc7c308 — __ZN4Ogre16MemoryDataStream8skipLineERKSs
// type: _DWORD __fastcall(Ogre::MemoryDataStream *__hidden this, const std::string *)
#[doc(alias = "Ogre::MemoryDataStream::skipLine(std::string const&)")]
pub fn stub_c7c308() -> ! {
    todo!("0xc7c308 Ogre::MemoryDataStream::skipLine(std::string const&)")
}

// 0xc7c338 — __ZN4Ogre16MemoryDataStream4skipEl
// type: _DWORD __fastcall(Ogre::MemoryDataStream *__hidden this, int)
#[doc(alias = "Ogre::MemoryDataStream::skip(long)")]
pub fn stub_c7c338() -> ! {
    todo!("0xc7c338 Ogre::MemoryDataStream::skip(long)")
}

// 0xc7c340 — __ZN4Ogre16MemoryDataStream4seekEm
// type: _DWORD __fastcall(Ogre::MemoryDataStream *__hidden this, unsigned int)
#[doc(alias = "Ogre::MemoryDataStream::seek(unsigned long)")]
pub fn stub_c7c340() -> ! {
    todo!("0xc7c340 Ogre::MemoryDataStream::seek(unsigned long)")
}

// 0xc7c348 — __ZNK4Ogre16MemoryDataStream4tellEv
// type: _DWORD __fastcall(Ogre::MemoryDataStream *__hidden this)
#[doc(alias = "Ogre::MemoryDataStream::tell(void)const")]
pub fn stub_c7c348() -> ! {
    todo!("0xc7c348 Ogre::MemoryDataStream::tell(void)const")
}

// 0xc7c350 — __ZNK4Ogre16MemoryDataStream3eofEv
// type: _DWORD __fastcall(Ogre::MemoryDataStream *__hidden this)
#[doc(alias = "Ogre::MemoryDataStream::eof(void)const")]
pub fn stub_c7c350() -> ! {
    todo!("0xc7c350 Ogre::MemoryDataStream::eof(void)const")
}

// 0xc7c360 — __ZN4Ogre16MemoryDataStream5closeEv
// type: _DWORD __fastcall(Ogre::MemoryDataStream *__hidden this)
#[doc(alias = "Ogre::MemoryDataStream::close(void)")]
pub fn stub_c7c360() -> ! {
    todo!("0xc7c360 Ogre::MemoryDataStream::close(void)")
}

// 0xc7c37c — __ZN4Ogre20FileStreamDataStreamC1ERKSsPSt14basic_ifstreamIcSt11char_traitsIcEEb
#[doc(alias = "Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_ifstream<char,std::char_traits<char>> *,bool)")]
pub fn stub_c7c37c() -> ! {
    todo!("0xc7c37c Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_ifstream<char,std::char_traits<char>> *,bool)")
}

// 0xc7c388 — __ZN4Ogre20FileStreamDataStreamC2ERKSsPSt14basic_ifstreamIcSt11char_traitsIcEEb
#[doc(alias = "Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_ifstream<char,std::char_traits<char>> *,bool)")]
pub fn stub_c7c388() -> ! {
    todo!("0xc7c388 Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_ifstream<char,std::char_traits<char>> *,bool)")
}

// 0xc7c520 — __ZN4Ogre20FileStreamDataStreamC1ERKSsPSt14basic_ifstreamIcSt11char_traitsIcEEmb
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_ifstream<char,std::char_traits<char>> *,unsigned long,bool)")]
pub fn stub_c7c520() -> ! {
    todo!("0xc7c520 Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_ifstream<char,std::char_traits<char>> *,unsigned long,bool)")
}

// 0xc7c56c — __ZN4Ogre20FileStreamDataStreamC1ERKSsPSt13basic_fstreamIcSt11char_traitsIcEEmb
#[doc(alias = "Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_fstream<char,std::char_traits<char>> *,unsigned long,bool)")]
pub fn stub_c7c56c() -> ! {
    todo!("0xc7c56c Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_fstream<char,std::char_traits<char>> *,unsigned long,bool)")
}

// 0xc7c5b8 — __ZN4Ogre20FileStreamDataStreamD0Ev
// type: void __fastcall(Ogre::FileStreamDataStream *__hidden this)
#[doc(alias = "Ogre::FileStreamDataStream::~FileStreamDataStream()")]
pub fn stub_c7c5b8() -> ! {
    todo!("0xc7c5b8 Ogre::FileStreamDataStream::~FileStreamDataStream()")
}

// 0xc7c648 — __ZN4Ogre20FileStreamDataStreamD1Ev
// type: void __fastcall(Ogre::FileStreamDataStream *__hidden this)
#[doc(alias = "Ogre::FileStreamDataStream::~FileStreamDataStream()")]
pub fn stub_c7c648() -> ! {
    todo!("0xc7c648 Ogre::FileStreamDataStream::~FileStreamDataStream()")
}

// 0xc7c654 — __ZN4Ogre20FileStreamDataStreamD2Ev
// type: void __fastcall(Ogre::FileStreamDataStream *__hidden this)
#[doc(alias = "Ogre::FileStreamDataStream::~FileStreamDataStream()")]
pub fn stub_c7c654() -> ! {
    todo!("0xc7c654 Ogre::FileStreamDataStream::~FileStreamDataStream()")
}

// 0xc7c798 — __ZN4Ogre20FileStreamDataStream4readEPvm
// type: _DWORD __fastcall(Ogre::FileStreamDataStream *__hidden this, void *, unsigned int)
#[doc(alias = "Ogre::FileStreamDataStream::read(void *,unsigned long)")]
pub fn stub_c7c798() -> ! {
    todo!("0xc7c798 Ogre::FileStreamDataStream::read(void *,unsigned long)")
}

// 0xc7c7ac — __ZN4Ogre20FileStreamDataStream5writeEPKvm
// type: _DWORD __fastcall(Ogre::FileStreamDataStream *__hidden this, const void *, unsigned int)
#[doc(alias = "Ogre::FileStreamDataStream::write(void const*,unsigned long)")]
pub fn stub_c7c7ac() -> ! {
    todo!("0xc7c7ac Ogre::FileStreamDataStream::write(void const*,unsigned long)")
}

// 0xc7c7e0 — __ZN4Ogre20FileStreamDataStream8readLineEPcmRKSs
// type: _DWORD __fastcall(Ogre::FileStreamDataStream *__hidden this, char *, unsigned int, const std::string *)
#[doc(alias = "Ogre::FileStreamDataStream::readLine(char *,unsigned long,std::string const&)")]
pub fn stub_c7c7e0() -> ! {
    todo!("0xc7c7e0 Ogre::FileStreamDataStream::readLine(char *,unsigned long,std::string const&)")
}

// 0xc7cc5c — __ZN4Ogre20FileStreamDataStream4skipEl
// type: _DWORD __fastcall(Ogre::FileStreamDataStream *__hidden this, int)
#[doc(alias = "Ogre::FileStreamDataStream::skip(long)")]
pub fn stub_c7cc5c() -> ! {
    todo!("0xc7cc5c Ogre::FileStreamDataStream::skip(long)")
}

// 0xc7cc84 — __ZN4Ogre20FileStreamDataStream4seekEm
// type: _DWORD __fastcall(Ogre::FileStreamDataStream *__hidden this, unsigned int)
#[doc(alias = "Ogre::FileStreamDataStream::seek(unsigned long)")]
pub fn stub_c7cc84() -> ! {
    todo!("0xc7cc84 Ogre::FileStreamDataStream::seek(unsigned long)")
}

// 0xc7ccac — __ZNK4Ogre20FileStreamDataStream4tellEv
// type: _DWORD __fastcall(Ogre::FileStreamDataStream *__hidden this)
#[doc(alias = "Ogre::FileStreamDataStream::tell(void)const")]
pub fn stub_c7ccac() -> ! {
    todo!("0xc7ccac Ogre::FileStreamDataStream::tell(void)const")
}

// 0xc7ccf0 — __ZNK4Ogre20FileStreamDataStream3eofEv
// type: _DWORD __fastcall(Ogre::FileStreamDataStream *__hidden this)
#[doc(alias = "Ogre::FileStreamDataStream::eof(void)const")]
pub fn stub_c7ccf0() -> ! {
    todo!("0xc7ccf0 Ogre::FileStreamDataStream::eof(void)const")
}

// 0xc7cd04 — __ZN4Ogre20FileStreamDataStream5closeEv
// type: _DWORD __fastcall(Ogre::FileStreamDataStream *__hidden this)
#[doc(alias = "Ogre::FileStreamDataStream::close(void)")]
pub fn stub_c7cd04() -> ! {
    todo!("0xc7cd04 Ogre::FileStreamDataStream::close(void)")
}

// 0xc7cd90 — __GLOBAL__I_a_672
#[doc(alias = "global constructor keyed to_a_672")]
pub fn stub_c7cd90() -> ! {
    todo!("0xc7cd90 global constructor keyed to_a_672")
}