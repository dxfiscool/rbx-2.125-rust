//! core shard ob — 120 core stubs EA-sorted asc gap filler global not yet in crates (global).
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 120 not yet in crates as stub_0x (global distinct 32922 before -> 32802 after, batch 0x600084..0x610524).
//! Filter: global EA-sorted asc next uncovered (no namespace filter), rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSERKSA_")]
// 0x600084 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSERKSA_
// type: int(void)
pub fn stub_0x600084() -> ! {
    todo!("0x600084 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSERKSA_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE22safe_static_init_mutexEv")]
// 0x6000ac — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE22safe_static_init_mutexEv
pub fn stub_0x6000ac() -> ! {
    todo!("0x6000ac __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv")]
// 0x6000b0 — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv
pub fn stub_0x6000b0() -> ! {
    todo!("0x6000b0 __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::operator[](RBX::StarterGuiService::CoreGuiType const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEixERS8_")]
// 0x6001a8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEixERS8_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
pub fn stub_0x6001a8() -> ! {
    todo!("0x6001a8 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEixERS8_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm")]
// 0x600324 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
// type: int(void)
pub fn stub_0x600324() -> ! {
    todo!("0x600324 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm")]
// 0x600378 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
pub fn stub_0x600378() -> ! {
    todo!("0x600378 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm")]
// 0x6004a0 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
// type: int(void)
pub fn stub_0x6004a0() -> ! {
    todo!("0x6004a0 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::rehash_impl(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm")]
// 0x600530 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
// type: int(void)
pub fn stub_0x600530() -> ! {
    todo!("0x600530 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE")]
// 0x60055c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE
// type: int(void)
pub fn stub_0x60055c() -> ! {
    todo!("0x60055c __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>>>::construct(void)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEEEEE9constructEv")]
// 0x6005b4 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEEEEE9constructEv
// type: int(void)
pub fn stub_0x6005b4() -> ! {
    todo!("0x6005b4 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::find_node_impl<RBX::StarterGuiService::CoreGuiType,std::equal_to<RBX::StarterGuiService::CoreGuiType>>(unsigned long,RBX::StarterGuiService::CoreGuiType const&,std::equal_to<RBX::StarterGuiService::CoreGuiType> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_")]
// 0x6005ec — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_
// type: int(void)
pub fn stub_0x6005ec() -> ! {
    todo!("0x6005ec __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::delete_buckets(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv")]
// 0x600658 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
// type: int(void)
pub fn stub_0x600658() -> ! {
    todo!("0x600658 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::table(unsigned long,boost::hash<RBX::StarterGuiService::CoreGuiType> const&,std::equal_to<RBX::StarterGuiService::CoreGuiType> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSC_RKSE_RKSaINS1_8ptr_nodeIS9_EEE")]
// 0x6006a4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSC_RKSE_RKSaINS1_8ptr_nodeIS9_EEE
// type: int(void)
pub fn stub_0x6006a4() -> ! {
    todo!("0x6006a4 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSC_RKSE_RKSaINS1_8ptr_nodeIS9_EEE")
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType * rbx::any_cast<RBX::StarterGuiService::CoreGuiType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x600914 — __ZN3rbx8any_castIN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int(void)
pub fn stub_0x600914() -> ! {
    todo!("0x600914 __ZN3rbx8any_castIN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType & rbx::any_cast<RBX::StarterGuiService::CoreGuiType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x600970 — __ZN3rbx8any_castIRN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
pub fn stub_0x600970() -> ! {
    todo!("0x600970 __ZN3rbx8any_castIRN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::resize(unsigned long,RBX::StarterGuiService::CoreGuiType)")]
#[doc(alias = "__ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE6resizeEmS2_")]
// 0x600a64 — __ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE6resizeEmS2_
// type: int(void)
pub fn stub_0x600a64() -> ! {
    todo!("0x600a64 __ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::push_back(RBX::StarterGuiService::CoreGuiType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE9push_backERKS2_")]
// 0x600a9c — __ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE9push_backERKS2_
// type: int(void)
pub fn stub_0x600a9c() -> ! {
    todo!("0x600a9c __ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::StarterGuiService::CoreGuiType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_17StarterGuiService11CoreGuiTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0x600ac8 — __ZNSt3mapIPKN3RBX4NameENS0_17StarterGuiService11CoreGuiTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
pub fn stub_0x600ac8() -> ! {
    todo!("0x600ac8 __ZNSt3mapIPKN3RBX4NameENS0_17StarterGuiService11CoreGuiTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0x600b20 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x600b20() -> ! {
    todo!("0x600b20 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0x600bd4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
pub fn stub_0x600bd4() -> ! {
    todo!("0x600bd4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0x600c2c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
pub fn stub_0x600c2c() -> ! {
    todo!("0x600c2c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::StarterGuiService::CoreGuiType*,std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>>,RBX::StarterGuiService::CoreGuiType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x600c98 — __ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0x600c98() -> ! {
    todo!("0x600c98 __ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE11_M_allocateEm")]
// 0x600d7c — __ZNSt12_Vector_baseIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE11_M_allocateEm
// type: int(void)
pub fn stub_0x600d7c() -> ! {
    todo!("0x600d7c __ZNSt12_Vector_baseIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::StarterGuiService::CoreGuiType *,RBX::StarterGuiService::CoreGuiType *>(RBX::StarterGuiService::CoreGuiType *,RBX::StarterGuiService::CoreGuiType *,RBX::StarterGuiService::CoreGuiType *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17StarterGuiService11CoreGuiTypeES6_EET0_T_S8_S7_")]
// 0x600d94 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17StarterGuiService11CoreGuiTypeES6_EET0_T_S8_S7_
// type: int(void)
pub fn stub_0x600d94() -> ! {
    todo!("0x600d94 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17StarterGuiService11CoreGuiTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::StarterGuiService::CoreGuiType*,std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>>,unsigned long,RBX::StarterGuiService::CoreGuiType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0x600dd4 — __ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
pub fn stub_0x600dd4() -> ! {
    todo!("0x600dd4 __ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13disconnectAllEv")]
// 0x60140c — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x60140c() -> ! {
    todo!("0x60140c __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13disconnectAllEv")
}

#[doc(alias = "boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool>::clear(void)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE5clearEv")]
// 0x601808 — __ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE5clearEv
// type: int(void)
pub fn stub_0x601808() -> ! {
    todo!("0x601808 __ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE5clearEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::connect<boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)>>(boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")]
// 0x601f44 — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x601f44() -> ! {
    todo!("0x601f44 __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::insert(rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6insertEPNS6_4slotE")]
// 0x602038 — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x602038() -> ! {
    todo!("0x602038 __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSEPS9_")]
// 0x602244 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSEPS9_
// type: int(void)
pub fn stub_0x602244() -> ! {
    todo!("0x602244 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot,boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,2,void ()(RBX::StarterGuiService::CoreGuiType,bool)>::callable<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>*>(boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)> const&,rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_")]
// 0x602268 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_
pub fn stub_0x602268() -> ! {
    todo!("0x602268 __ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::callable_slot<boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13callable_slotIN5boost8functionIS5_EEED1Ev")]
// 0x602364 — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13callable_slotIN5boost8functionIS5_EEED1Ev
pub fn stub_0x602364() -> ! {
    todo!("0x602364 __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13callable_slotIN5boost8functionIS5_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::callable_slot<boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13callable_slotIN5boost8functionIS5_EEED0Ev")]
// 0x602474 — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13callable_slotIN5boost8functionIS5_EEED0Ev
pub fn stub_0x602474() -> ! {
    todo!("0x602474 __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13callable_slotIN5boost8functionIS5_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot10disconnectEv")]
// 0x6025a4 — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot10disconnectEv
pub fn stub_0x6025a4() -> ! {
    todo!("0x6025a4 __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot9connectedEv")]
// 0x6026b4 — __ZNK3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot9connectedEv
pub fn stub_0x6026b4() -> ! {
    todo!("0x6026b4 __ZNK3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot,boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,2,void ()(RBX::StarterGuiService::CoreGuiType,bool)>::call(RBX::StarterGuiService::CoreGuiType,bool)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_b")]
// 0x6026c0 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_b
pub fn stub_0x6026c0() -> ! {
    todo!("0x6026c0 __ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_b")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot,boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,2,void ()(RBX::StarterGuiService::CoreGuiType,bool)>::call(RBX::StarterGuiService::CoreGuiType,bool)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_b")]
// 0x6026c8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_b
pub fn stub_0x6026c8() -> ! {
    todo!("0x6026c8 __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_b")
}

#[doc(alias = "boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool>::operator()(RBX::StarterGuiService::CoreGuiType,bool)const")]
#[doc(alias = "__ZNK5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbEclES3_b")]
// 0x6026d0 — __ZNK5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbEclES3_b
// type: int(void)
pub fn stub_0x6026d0() -> ! {
    todo!("0x6026d0 __ZNK5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbEclES3_b")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::remove(rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6removeEPNS6_4slotE")]
// 0x602798 — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x602798() -> ! {
    todo!("0x602798 __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot22safe_static_init_mutexEv")]
// 0x602888 — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot22safe_static_init_mutexEv
pub fn stub_0x602888() -> ! {
    todo!("0x602888 __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot24safe_static_do_get_mutexEv")]
// 0x60288c — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot24safe_static_do_get_mutexEv
// type: void *()
pub fn stub_0x60288c() -> ! {
    todo!("0x60288c __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot,boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,2,void ()(RBX::StarterGuiService::CoreGuiType,bool)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_ED1Ev")]
// 0x602980 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_ED1Ev
pub fn stub_0x602980() -> ! {
    todo!("0x602980 __ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot,boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,2,void ()(RBX::StarterGuiService::CoreGuiType,bool)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_ED0Ev")]
// 0x602a90 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_ED0Ev
pub fn stub_0x602a90() -> ! {
    todo!("0x602a90 __ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotD1Ev")]
// 0x602bc0 — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotD1Ev
pub fn stub_0x602bc0() -> ! {
    todo!("0x602bc0 __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotD0Ev")]
// 0x602bec — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotD0Ev
pub fn stub_0x602bec() -> ! {
    todo!("0x602bec __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotD0Ev")
}

#[doc(alias = "boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_to_own(boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool> const&)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE13assign_to_ownERKS4_")]
// 0x602cc0 — __ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE13assign_to_ownERKS4_
// type: int(void)
pub fn stub_0x602cc0() -> ! {
    todo!("0x602cc0 __ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE13assign_to_ownERKS4_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::IAdornableCollector>(RBX::IAdornableCollector *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX19IAdornableCollectorEEEPT_")]
// 0x6039f0 — __ZN5boost6detail12shared_countC2IN3RBX19IAdornableCollectorEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x6039f0() -> ! {
    todo!("0x6039f0 __ZN5boost6detail12shared_countC2IN3RBX19IAdornableCollectorEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::IAdornableCollector>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX19IAdornableCollectorEED0Ev")]
// 0x603b00 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19IAdornableCollectorEED0Ev
pub fn stub_0x603b00() -> ! {
    todo!("0x603b00 __ZN5boost6detail17sp_counted_impl_pIN3RBX19IAdornableCollectorEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::IAdornableCollector>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX19IAdornableCollectorEE11get_deleterERKSt9type_info")]
// 0x603b08 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19IAdornableCollectorEE11get_deleterERKSt9type_info
pub fn stub_0x603b08() -> ! {
    todo!("0x603b08 __ZN5boost6detail17sp_counted_impl_pIN3RBX19IAdornableCollectorEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::multi_index_container(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type> const&,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&)")]
#[doc(alias = "__ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EEC2ERKNS_6tuples4consINSI_5tupleImNS0_8identityISA_EENS_4hashIS7_EESt8equal_toIS7_ENSI_9null_typeESR_SR_SR_SR_SR_EESR_EERKSG_")]
// 0x603d58 — __ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EEC2ERKNS_6tuples4consINSI_5tupleImNS0_8identityISA_EENS_4hashIS7_EESt8equal_toIS7_ENSI_9null_typeESR_SR_SR_SR_SR_EESR_EERKSG_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x603d58() -> ! {
    todo!("0x603d58 __ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EEC2ERKNS_6tuples4consINSI_5tupleImNS0_8identityISA_EENS_4hashIS7_EESt8equal_toIS7_ENSI_9null_typeESR_SR_SR_SR_SR_EESR_EERKSG_")
}

#[doc(alias = "RBX::StarterGuiService::~StarterGuiService()")]
#[doc(alias = "__ZN3RBX17StarterGuiServiceD2Ev")]
// 0x603e1c — __ZN3RBX17StarterGuiServiceD2Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
pub fn stub_0x603e1c() -> ! {
    todo!("0x603e1c __ZN3RBX17StarterGuiServiceD2Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// 0x603f50 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int(void)
pub fn stub_0x603f50() -> ! {
    todo!("0x603f50 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "RBX::IAdornableCollector::IAdornableCollector(void)")]
#[doc(alias = "__ZN3RBX19IAdornableCollectorC2Ev")]
// 0x603f78 — __ZN3RBX19IAdornableCollectorC2Ev
// type: _DWORD __fastcall(RBX::IAdornableCollector *__hidden this)
pub fn stub_0x603f78() -> ! {
    todo!("0x603f78 __ZN3RBX19IAdornableCollectorC2Ev")
}

#[doc(alias = "RBX::PlayerHUD::PlayerHUD(void)")]
#[doc(alias = "__ZN3RBX9PlayerHUDC1Ev")]
// 0x604694 — __ZN3RBX9PlayerHUDC1Ev
// type: _DWORD __fastcall(RBX::PlayerHUD *__hidden this)
pub fn stub_0x604694() -> ! {
    todo!("0x604694 __ZN3RBX9PlayerHUDC1Ev")
}

#[doc(alias = "RBX::PlayerHUD::PlayerHUD(void)")]
#[doc(alias = "__ZN3RBX9PlayerHUDC2Ev")]
// 0x604698 — __ZN3RBX9PlayerHUDC2Ev
// type: _DWORD __fastcall(RBX::PlayerHUD *__hidden this)
pub fn stub_0x604698() -> ! {
    todo!("0x604698 __ZN3RBX9PlayerHUDC2Ev")
}

#[doc(alias = "RBX::PlayerHUD::~PlayerHUD()")]
#[doc(alias = "__ZN3RBX9PlayerHUDD1Ev")]
// 0x604820 — __ZN3RBX9PlayerHUDD1Ev
// type: void __fastcall(RBX::PlayerHUD *__hidden this)
pub fn stub_0x604820() -> ! {
    todo!("0x604820 __ZN3RBX9PlayerHUDD1Ev")
}

#[doc(alias = "RBX::PlayerHUD::~PlayerHUD()")]
#[doc(alias = "__ZN3RBX9PlayerHUDD0Ev")]
// 0x604824 — __ZN3RBX9PlayerHUDD0Ev
// type: void __fastcall(RBX::PlayerHUD *__hidden this)
pub fn stub_0x604824() -> ! {
    todo!("0x604824 __ZN3RBX9PlayerHUDD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PlayerHUD::~PlayerHUD()")]
#[doc(alias = "__ZThn32_N3RBX9PlayerHUDD1Ev")]
// 0x6048ec — __ZThn32_N3RBX9PlayerHUDD1Ev
// type: void __fastcall(RBX::PlayerHUD *__hidden this)
pub fn stub_0x6048ec() -> ! {
    todo!("0x6048ec __ZThn32_N3RBX9PlayerHUDD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PlayerHUD::~PlayerHUD()")]
#[doc(alias = "__ZThn32_N3RBX9PlayerHUDD0Ev")]
// 0x6048f4 — __ZThn32_N3RBX9PlayerHUDD0Ev
// type: void __fastcall(RBX::PlayerHUD *__hidden this)
pub fn stub_0x6048f4() -> ! {
    todo!("0x6048f4 __ZThn32_N3RBX9PlayerHUDD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PlayerHUD::~PlayerHUD()")]
#[doc(alias = "__ZThn36_N3RBX9PlayerHUDD1Ev")]
// 0x6049c0 — __ZThn36_N3RBX9PlayerHUDD1Ev
// type: void __fastcall(RBX::PlayerHUD *__hidden this)
pub fn stub_0x6049c0() -> ! {
    todo!("0x6049c0 __ZThn36_N3RBX9PlayerHUDD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PlayerHUD::~PlayerHUD()")]
#[doc(alias = "__ZThn36_N3RBX9PlayerHUDD0Ev")]
// 0x6049c8 — __ZThn36_N3RBX9PlayerHUDD0Ev
// type: void __fastcall(RBX::PlayerHUD *__hidden this)
pub fn stub_0x6049c8() -> ! {
    todo!("0x6049c8 __ZThn36_N3RBX9PlayerHUDD0Ev")
}

#[doc(alias = "RBX::PlayerMouse::PlayerMouse(void)")]
#[doc(alias = "__ZN3RBX11PlayerMouseC1Ev")]
// 0x604f54 — __ZN3RBX11PlayerMouseC1Ev
// type: _DWORD __fastcall(RBX::PlayerMouse *__hidden this)
pub fn stub_0x604f54() -> ! {
    todo!("0x604f54 __ZN3RBX11PlayerMouseC1Ev")
}

#[doc(alias = "RBX::PlayerMouse::PlayerMouse(void)")]
#[doc(alias = "__ZN3RBX11PlayerMouseC2Ev")]
// 0x604f58 — __ZN3RBX11PlayerMouseC2Ev
// type: _DWORD __fastcall(RBX::PlayerMouse *__hidden this)
pub fn stub_0x604f58() -> ! {
    todo!("0x604f58 __ZN3RBX11PlayerMouseC2Ev")
}

#[doc(alias = "RBX::PlayerMouse::~PlayerMouse()")]
#[doc(alias = "__ZN3RBX11PlayerMouseD0Ev")]
// 0x605088 — __ZN3RBX11PlayerMouseD0Ev
// type: void __fastcall(RBX::PlayerMouse *__hidden this)
pub fn stub_0x605088() -> ! {
    todo!("0x605088 __ZN3RBX11PlayerMouseD0Ev")
}

#[doc(alias = "RBX::PlayerMouse::~PlayerMouse()")]
#[doc(alias = "__ZN3RBX11PlayerMouseD1Ev")]
// 0x605128 — __ZN3RBX11PlayerMouseD1Ev
// type: void __fastcall(RBX::PlayerMouse *__hidden this)
pub fn stub_0x605128() -> ! {
    todo!("0x605128 __ZN3RBX11PlayerMouseD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PlayerMouse::~PlayerMouse()")]
#[doc(alias = "__ZThn32_N3RBX11PlayerMouseD0Ev")]
// 0x60512c — __ZThn32_N3RBX11PlayerMouseD0Ev
// type: void __fastcall(RBX::PlayerMouse *__hidden this)
pub fn stub_0x60512c() -> ! {
    todo!("0x60512c __ZThn32_N3RBX11PlayerMouseD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PlayerMouse::~PlayerMouse()")]
#[doc(alias = "__ZThn36_N3RBX11PlayerMouseD0Ev")]
// 0x605134 — __ZThn36_N3RBX11PlayerMouseD0Ev
// type: void __fastcall(RBX::PlayerMouse *__hidden this)
pub fn stub_0x605134() -> ! {
    todo!("0x605134 __ZThn36_N3RBX11PlayerMouseD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PlayerMouse::~PlayerMouse()")]
#[doc(alias = "__ZThn32_N3RBX11PlayerMouseD1Ev")]
// 0x60513c — __ZThn32_N3RBX11PlayerMouseD1Ev
// type: void __fastcall(RBX::PlayerMouse *__hidden this)
pub fn stub_0x60513c() -> ! {
    todo!("0x60513c __ZThn32_N3RBX11PlayerMouseD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PlayerMouse::~PlayerMouse()")]
#[doc(alias = "__ZThn36_N3RBX11PlayerMouseD1Ev")]
// 0x605144 — __ZThn36_N3RBX11PlayerMouseD1Ev
// type: void __fastcall(RBX::PlayerMouse *__hidden this)
pub fn stub_0x605144() -> ! {
    todo!("0x605144 __ZThn36_N3RBX11PlayerMouseD1Ev")
}

#[doc(alias = "RBX::PlayerMouse::getIcon(void)const")]
#[doc(alias = "__ZNK3RBX11PlayerMouse7getIconEv")]
// 0x60514c — __ZNK3RBX11PlayerMouse7getIconEv
// type: _DWORD __fastcall(RBX::PlayerMouse *__hidden this)
pub fn stub_0x60514c() -> ! {
    todo!("0x60514c __ZNK3RBX11PlayerMouse7getIconEv")
}

#[doc(alias = "RBX::PlayerMouse::setIcon(RBX::TextureId const&)")]
#[doc(alias = "__ZN3RBX11PlayerMouse7setIconERKNS_9TextureIdE")]
// 0x605214 — __ZN3RBX11PlayerMouse7setIconERKNS_9TextureIdE
// type: _DWORD __fastcall(RBX::PlayerMouse *__hidden this, const RBX::TextureId *)
pub fn stub_0x605214() -> ! {
    todo!("0x605214 __ZN3RBX11PlayerMouse7setIconERKNS_9TextureIdE")
}

#[doc(alias = "RBX::Mouse::checkActive(void)const")]
#[doc(alias = "__ZNK3RBX5Mouse11checkActiveEv")]
// 0x6053ec — __ZNK3RBX5Mouse11checkActiveEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
pub fn stub_0x6053ec() -> ! {
    todo!("0x6053ec __ZNK3RBX5Mouse11checkActiveEv")
}

#[doc(alias = "RBX::Pose::getSubPoses(void)")]
#[doc(alias = "__ZN3RBX4Pose11getSubPosesEv")]
// 0x605a90 — __ZN3RBX4Pose11getSubPosesEv
// type: _DWORD __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x605a90() -> ! {
    todo!("0x605a90 __ZN3RBX4Pose11getSubPosesEv")
}

#[doc(alias = "RBX::Pose::setWeight(float)")]
#[doc(alias = "__ZN3RBX4Pose9setWeightEf")]
// 0x605b7c — __ZN3RBX4Pose9setWeightEf
// type: _DWORD __fastcall(RBX::Pose *__hidden this, float)
pub fn stub_0x605b7c() -> ! {
    todo!("0x605b7c __ZN3RBX4Pose9setWeightEf")
}

#[doc(alias = "RBX::Pose::setMaskWeight(float)")]
#[doc(alias = "__ZN3RBX4Pose13setMaskWeightEf")]
// 0x605bb8 — __ZN3RBX4Pose13setMaskWeightEf
// type: _DWORD __fastcall(RBX::Pose *__hidden this, float)
pub fn stub_0x605bb8() -> ! {
    todo!("0x605bb8 __ZN3RBX4Pose13setMaskWeightEf")
}

#[doc(alias = "RBX::Pose::Pose(void)")]
#[doc(alias = "__ZN3RBX4PoseC2Ev")]
// 0x605bf4 — __ZN3RBX4PoseC2Ev
// type: _DWORD __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x605bf4() -> ! {
    todo!("0x605bf4 __ZN3RBX4PoseC2Ev")
}

#[doc(alias = "RBX::Pose::findKeyframeParent(void)")]
#[doc(alias = "__ZN3RBX4Pose18findKeyframeParentEv")]
// 0x605e10 — __ZN3RBX4Pose18findKeyframeParentEv
// type: _DWORD __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x605e10() -> ! {
    todo!("0x605e10 __ZN3RBX4Pose18findKeyframeParentEv")
}

#[doc(alias = "RBX::Pose::invalidate(void)")]
#[doc(alias = "__ZN3RBX4Pose10invalidateEv")]
// 0x605e9c — __ZN3RBX4Pose10invalidateEv
// type: _DWORD __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x605e9c() -> ! {
    todo!("0x605e9c __ZN3RBX4Pose10invalidateEv")
}

#[doc(alias = "RBX::Pose::getCoordinateFrame(void)const")]
#[doc(alias = "__ZNK3RBX4Pose18getCoordinateFrameEv")]
// 0x6060d4 — __ZNK3RBX4Pose18getCoordinateFrameEv
// type: _DWORD __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x6060d4() -> ! {
    todo!("0x6060d4 __ZNK3RBX4Pose18getCoordinateFrameEv")
}

#[doc(alias = "RBX::Pose::getWeight(void)const")]
#[doc(alias = "__ZNK3RBX4Pose9getWeightEv")]
// 0x6060fc — __ZNK3RBX4Pose9getWeightEv
// type: _DWORD __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x6060fc() -> ! {
    todo!("0x6060fc __ZNK3RBX4Pose9getWeightEv")
}

#[doc(alias = "RBX::Pose::getMaskWeight(void)const")]
#[doc(alias = "__ZNK3RBX4Pose13getMaskWeightEv")]
// 0x606128 — __ZNK3RBX4Pose13getMaskWeightEv
// type: _DWORD __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x606128() -> ! {
    todo!("0x606128 __ZNK3RBX4Pose13getMaskWeightEv")
}

#[doc(alias = "RBX::Pose::~Pose()")]
#[doc(alias = "__ZN3RBX4PoseD1Ev")]
// 0x606130 — __ZN3RBX4PoseD1Ev
// type: void __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x606130() -> ! {
    todo!("0x606130 __ZN3RBX4PoseD1Ev")
}

#[doc(alias = "RBX::Pose::~Pose()")]
#[doc(alias = "__ZN3RBX4PoseD0Ev")]
// 0x606134 — __ZN3RBX4PoseD0Ev
// type: void __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x606134() -> ! {
    todo!("0x606134 __ZN3RBX4PoseD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Pose::~Pose()")]
#[doc(alias = "__ZThn32_N3RBX4PoseD1Ev")]
// 0x606228 — __ZThn32_N3RBX4PoseD1Ev
// type: void __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x606228() -> ! {
    todo!("0x606228 __ZThn32_N3RBX4PoseD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Pose::~Pose()")]
#[doc(alias = "__ZThn32_N3RBX4PoseD0Ev")]
// 0x606230 — __ZThn32_N3RBX4PoseD0Ev
// type: void __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x606230() -> ! {
    todo!("0x606230 __ZThn32_N3RBX4PoseD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Pose::~Pose()")]
#[doc(alias = "__ZThn36_N3RBX4PoseD1Ev")]
// 0x6062e4 — __ZThn36_N3RBX4PoseD1Ev
// type: void __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x6062e4() -> ! {
    todo!("0x6062e4 __ZThn36_N3RBX4PoseD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Pose::~Pose()")]
#[doc(alias = "__ZThn36_N3RBX4PoseD0Ev")]
// 0x6062ec — __ZThn36_N3RBX4PoseD0Ev
// type: void __fastcall(RBX::Pose *__hidden this)
pub fn stub_0x6062ec() -> ! {
    todo!("0x6062ec __ZThn36_N3RBX4PoseD0Ev")
}

#[doc(alias = "std::auto_ptr<RBX::World>::~auto_ptr()")]
#[doc(alias = "__ZNSt8auto_ptrIN3RBX5WorldEED2Ev")]
// 0x60d9b4 — __ZNSt8auto_ptrIN3RBX5WorldEED2Ev
pub fn stub_0x60d9b4() -> ! {
    todo!("0x60d9b4 __ZNSt8auto_ptrIN3RBX5WorldEED2Ev")
}

#[doc(alias = "RBX::ICameraOwner::~ICameraOwner()")]
#[doc(alias = "__ZN3RBX12ICameraOwnerD1Ev")]
// 0x60da5c — __ZN3RBX12ICameraOwnerD1Ev
// type: void __fastcall(RBX::ICameraOwner *__hidden this)
pub fn stub_0x60da5c() -> ! {
    todo!("0x60da5c __ZN3RBX12ICameraOwnerD1Ev")
}

#[doc(alias = "RBX::ICameraOwner::~ICameraOwner()")]
#[doc(alias = "__ZN3RBX12ICameraOwnerD0Ev")]
// 0x60da60 — __ZN3RBX12ICameraOwnerD0Ev
// type: void __fastcall(RBX::ICameraOwner *__hidden this)
pub fn stub_0x60da60() -> ! {
    todo!("0x60da60 __ZN3RBX12ICameraOwnerD0Ev")
}

#[doc(alias = "RBX::SafeChat::singleton(void)")]
#[doc(alias = "__ZN3RBX8SafeChat9singletonEv")]
// 0x60ddb8 — __ZN3RBX8SafeChat9singletonEv
// type: _DWORD __fastcall(RBX::SafeChat *__hidden this)
pub fn stub_0x60ddb8() -> ! {
    todo!("0x60ddb8 __ZN3RBX8SafeChat9singletonEv")
}

#[doc(alias = "RBX::SafeChat::loadChildren(RBX::ChatOption *,XmlElement const*)")]
#[doc(alias = "__ZN3RBX8SafeChat12loadChildrenEPNS_10ChatOptionEPK10XmlElement")]
// 0x60deb4 — __ZN3RBX8SafeChat12loadChildrenEPNS_10ChatOptionEPK10XmlElement
// type: _DWORD __fastcall(RBX::SafeChat *__hidden this, RBX::ChatOption *, const XmlElement *)
pub fn stub_0x60deb4() -> ! {
    todo!("0x60deb4 __ZN3RBX8SafeChat12loadChildrenEPNS_10ChatOptionEPK10XmlElement")
}

#[doc(alias = "RBX::SafeChat::loadChatTree(void)")]
#[doc(alias = "__ZN3RBX8SafeChat12loadChatTreeEv")]
// 0x60e178 — __ZN3RBX8SafeChat12loadChatTreeEv
// type: _DWORD __fastcall(RBX::SafeChat *__hidden this)
pub fn stub_0x60e178() -> ! {
    todo!("0x60e178 __ZN3RBX8SafeChat12loadChatTreeEv")
}

#[doc(alias = "RBX::SafeChat::getMessage(std::vector<std::string,std::allocator<std::string>>)")]
#[doc(alias = "__ZN3RBX8SafeChat10getMessageESt6vectorISsSaISsEE")]
// 0x60e4e8 — __ZN3RBX8SafeChat10getMessageESt6vectorISsSaISsEE
// type: int __fastcall(std::string *this)
pub fn stub_0x60e4e8() -> ! {
    todo!("0x60e4e8 __ZN3RBX8SafeChat10getMessageESt6vectorISsSaISsEE")
}

#[doc(alias = "RBX::ChatOption::~ChatOption()")]
#[doc(alias = "__ZN3RBX10ChatOptionD2Ev")]
// 0x60e5f0 — __ZN3RBX10ChatOptionD2Ev
// type: void __fastcall(RBX::ChatOption *__hidden this)
pub fn stub_0x60e5f0() -> ! {
    todo!("0x60e5f0 __ZN3RBX10ChatOptionD2Ev")
}

#[doc(alias = "boost::scoped_ptr<RBX::SafeChat>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX8SafeChatEED1Ev")]
// 0x60e6fc — __ZN5boost10scoped_ptrIN3RBX8SafeChatEED1Ev
pub fn stub_0x60e6fc() -> ! {
    todo!("0x60e6fc __ZN5boost10scoped_ptrIN3RBX8SafeChatEED1Ev")
}

#[doc(alias = "std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::push_back(RBX::ChatOption * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE9push_backERKS2_")]
// 0x60e700 — __ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE9push_backERKS2_
// type: int(void)
pub fn stub_0x60e700() -> ! {
    todo!("0x60e700 __ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "boost::scoped_ptr<RBX::ChatOption>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX10ChatOptionEED2Ev")]
// 0x60e72c — __ZN5boost10scoped_ptrIN3RBX10ChatOptionEED2Ev
pub fn stub_0x60e72c() -> ! {
    todo!("0x60e72c __ZN5boost10scoped_ptrIN3RBX10ChatOptionEED2Ev")
}

#[doc(alias = "std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ChatOption **,std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>>,RBX::ChatOption * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x60e7d4 — __ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0x60e7d4() -> ! {
    todo!("0x60e7d4 __ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX10ChatOptionESaIS2_EE11_M_allocateEm")]
// 0x60e8b4 — __ZNSt12_Vector_baseIPN3RBX10ChatOptionESaIS2_EE11_M_allocateEm
// type: int(void)
pub fn stub_0x60e8b4() -> ! {
    todo!("0x60e8b4 __ZNSt12_Vector_baseIPN3RBX10ChatOptionESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "boost::scoped_ptr<RBX::SafeChat>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX8SafeChatEED2Ev")]
// 0x60e8cc — __ZN5boost10scoped_ptrIN3RBX8SafeChatEED2Ev
pub fn stub_0x60e8cc() -> ! {
    todo!("0x60e8cc __ZN5boost10scoped_ptrIN3RBX8SafeChatEED2Ev")
}

#[doc(alias = "RBX::ChatOption::ChatOption(std::string)")]
#[doc(alias = "__ZN3RBX10ChatOptionC2ESs")]
// 0x60e974 — __ZN3RBX10ChatOptionC2ESs
// type: int(void)
pub fn stub_0x60e974() -> ! {
    todo!("0x60e974 __ZN3RBX10ChatOptionC2ESs")
}

#[doc(alias = "RBX::Scale9Frame::setSlicePrefix(std::string)")]
#[doc(alias = "__ZN3RBX11Scale9Frame14setSlicePrefixESs")]
// 0x60ec58 — __ZN3RBX11Scale9Frame14setSlicePrefixESs
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x60ec58() -> ! {
    todo!("0x60ec58 __ZN3RBX11Scale9Frame14setSlicePrefixESs")
}

#[doc(alias = "RBX::Scale9Frame::Scale9Frame(void)")]
#[doc(alias = "__ZN3RBX11Scale9FrameC1Ev")]
// 0x60ec94 — __ZN3RBX11Scale9FrameC1Ev
// type: _DWORD __fastcall(RBX::Scale9Frame *__hidden this)
pub fn stub_0x60ec94() -> ! {
    todo!("0x60ec94 __ZN3RBX11Scale9FrameC1Ev")
}

#[doc(alias = "RBX::Scale9Frame::Scale9Frame(void)")]
#[doc(alias = "__ZN3RBX11Scale9FrameC2Ev")]
// 0x60ec98 — __ZN3RBX11Scale9FrameC2Ev
// type: _DWORD __fastcall(RBX::Scale9Frame *__hidden this)
pub fn stub_0x60ec98() -> ! {
    todo!("0x60ec98 __ZN3RBX11Scale9FrameC2Ev")
}

#[doc(alias = "RBX::Scale9Frame::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX11Scale9Frame8render2dEPNS_5AdornE")]
// 0x60ee44 — __ZN3RBX11Scale9Frame8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Scale9Frame *__hidden this, RBX::Adorn *)
pub fn stub_0x60ee44() -> ! {
    todo!("0x60ee44 __ZN3RBX11Scale9Frame8render2dEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::Scale9Frame::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX11Scale9Frame8render2dEPNS_5AdornE")]
// 0x60efb8 — __ZThn96_N3RBX11Scale9Frame8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Scale9Frame *__hidden this, RBX::Adorn *)
pub fn stub_0x60efb8() -> ! {
    todo!("0x60efb8 __ZThn96_N3RBX11Scale9Frame8render2dEPNS_5AdornE")
}

#[doc(alias = "RBX::Scale9Frame::getScaleEdgeSize(void)const")]
#[doc(alias = "__ZNK3RBX11Scale9Frame16getScaleEdgeSizeEv")]
// 0x60efc0 — __ZNK3RBX11Scale9Frame16getScaleEdgeSizeEv
// type: _DWORD __fastcall(RBX::Scale9Frame *__hidden this)
pub fn stub_0x60efc0() -> ! {
    todo!("0x60efc0 __ZNK3RBX11Scale9Frame16getScaleEdgeSizeEv")
}

#[doc(alias = "RBX::Scale9Frame::getSlicePrefix(void)const")]
#[doc(alias = "__ZNK3RBX11Scale9Frame14getSlicePrefixEv")]
// 0x60efec — __ZNK3RBX11Scale9Frame14getSlicePrefixEv
// type: _DWORD __fastcall(RBX::Scale9Frame *__hidden this)
pub fn stub_0x60efec() -> ! {
    todo!("0x60efec __ZNK3RBX11Scale9Frame14getSlicePrefixEv")
}

#[doc(alias = "RBX::Scale9Frame::~Scale9Frame()")]
#[doc(alias = "__ZN3RBX11Scale9FrameD1Ev")]
// 0x60f020 — __ZN3RBX11Scale9FrameD1Ev
// type: void __fastcall(RBX::Scale9Frame *__hidden this)
pub fn stub_0x60f020() -> ! {
    todo!("0x60f020 __ZN3RBX11Scale9FrameD1Ev")
}

#[doc(alias = "RBX::Scale9Frame::~Scale9Frame()")]
#[doc(alias = "__ZN3RBX11Scale9FrameD0Ev")]
// 0x60f128 — __ZN3RBX11Scale9FrameD0Ev
// type: void __fastcall(RBX::Scale9Frame *__hidden this)
pub fn stub_0x60f128() -> ! {
    todo!("0x60f128 __ZN3RBX11Scale9FrameD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Scale9Frame::~Scale9Frame()")]
#[doc(alias = "__ZThn32_N3RBX11Scale9FrameD1Ev")]
// 0x60f26c — __ZThn32_N3RBX11Scale9FrameD1Ev
// type: void __fastcall(RBX::Scale9Frame *__hidden this)
pub fn stub_0x60f26c() -> ! {
    todo!("0x60f26c __ZThn32_N3RBX11Scale9FrameD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Scale9Frame::~Scale9Frame()")]
#[doc(alias = "__ZThn32_N3RBX11Scale9FrameD0Ev")]
// 0x60f374 — __ZThn32_N3RBX11Scale9FrameD0Ev
// type: void __fastcall(RBX::Scale9Frame *__hidden this)
pub fn stub_0x60f374() -> ! {
    todo!("0x60f374 __ZThn32_N3RBX11Scale9FrameD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Scale9Frame::~Scale9Frame()")]
#[doc(alias = "__ZThn36_N3RBX11Scale9FrameD1Ev")]
// 0x60f4b8 — __ZThn36_N3RBX11Scale9FrameD1Ev
// type: void __fastcall(RBX::Scale9Frame *__hidden this)
pub fn stub_0x60f4b8() -> ! {
    todo!("0x60f4b8 __ZThn36_N3RBX11Scale9FrameD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Scale9Frame::~Scale9Frame()")]
#[doc(alias = "__ZThn36_N3RBX11Scale9FrameD0Ev")]
// 0x60f5c0 — __ZThn36_N3RBX11Scale9FrameD0Ev
// type: void __fastcall(RBX::Scale9Frame *__hidden this)
pub fn stub_0x60f5c0() -> ! {
    todo!("0x60f5c0 __ZThn36_N3RBX11Scale9FrameD0Ev")
}

#[doc(alias = "RBX::ScreenGui::ScreenGui(void)")]
#[doc(alias = "__ZN3RBX9ScreenGuiC1Ev")]
// 0x6100dc — __ZN3RBX9ScreenGuiC1Ev
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this)
pub fn stub_0x6100dc() -> ! {
    todo!("0x6100dc __ZN3RBX9ScreenGuiC1Ev")
}

#[doc(alias = "RBX::ScreenGui::ScreenGui(void)")]
#[doc(alias = "__ZN3RBX9ScreenGuiC2Ev")]
// 0x6100e0 — __ZN3RBX9ScreenGuiC2Ev
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this)
pub fn stub_0x6100e0() -> ! {
    todo!("0x6100e0 __ZN3RBX9ScreenGuiC2Ev")
}

#[doc(alias = "RBX::ScreenGui::ScreenGui(char const*)")]
#[doc(alias = "__ZN3RBX9ScreenGuiC2EPKc")]
// 0x6102ac — __ZN3RBX9ScreenGuiC2EPKc
// type: _DWORD __fastcall(RBX::ScreenGui *__hidden this, const char *)
pub fn stub_0x6102ac() -> ! {
    todo!("0x6102ac __ZN3RBX9ScreenGuiC2EPKc")
}

#[doc(alias = "RBX::ScreenGui::onHeartbeat(RBX::Heartbeat const&)")]
#[doc(alias = "__ZN3RBX9ScreenGui11onHeartbeatERKNS_9HeartbeatE")]
// 0x6104fc — __ZN3RBX9ScreenGui11onHeartbeatERKNS_9HeartbeatE
pub fn stub_0x6104fc() -> ! {
    todo!("0x6104fc __ZN3RBX9ScreenGui11onHeartbeatERKNS_9HeartbeatE")
}

#[doc(alias = "non-virtual thunk toRBX::ScreenGui::onHeartbeat(RBX::Heartbeat const&)")]
#[doc(alias = "__ZThn168_N3RBX9ScreenGui11onHeartbeatERKNS_9HeartbeatE")]
// 0x610524 — __ZThn168_N3RBX9ScreenGui11onHeartbeatERKNS_9HeartbeatE
pub fn stub_0x610524() -> ! {
    todo!("0x610524 __ZThn168_N3RBX9ScreenGui11onHeartbeatERKNS_9HeartbeatE")
}

