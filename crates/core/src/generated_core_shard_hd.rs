//! core shard HD — 100 core stubs EA-sorted, 0xf588e4..0xf590c4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HC 0xf588d4).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HC 0xf588d4 (0xf588e4..0xf590c4, 20714->20814 covered, 1104 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType> const&)")]
// 0xf588e4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf588e4() -> ! {
    todo!("0xf588e4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType> const&)")]
// 0xf588f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf588f4() -> ! {
    todo!("0xf588f4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Vector_base<std::vector<bool,std::allocator<bool>>,std::allocator<std::vector<bool,std::allocator<bool>>>>::_M_allocate(unsigned long)")]
// 0xf58914 — j___ZNSt12_Vector_baseISt6vectorIbSaIbEESaIS2_EE11_M_allocateEm
pub fn stub_0xf58914() -> ! {
    todo!("0xf58914 j___ZNSt12_Vector_baseISt6vectorIbSaIbEESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::vector<bool,std::allocator<bool>> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::vector<bool,std::allocator<bool>> *,std::vector<bool,std::allocator<bool>> *>(std::vector<bool,std::allocator<bool>> *,std::vector<bool,std::allocator<bool>> *,std::vector<bool,std::allocator<bool>> *)")]
// 0xf58934 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt6vectorIbSaIbEES6_EET0_T_S8_S7_
pub fn stub_0xf58934() -> ! {
    todo!("0xf58934 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt6vectorIbSaIbEES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<std::vector<bool,std::allocator<bool>>,std::allocator<std::vector<bool,std::allocator<bool>>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::vector<bool,std::allocator<bool>>*,std::vector<std::vector<bool,std::allocator<bool>>,std::allocator<std::vector<bool,std::allocator<bool>>>>>,unsigned long,std::vector<bool,std::allocator<bool>> const&)")]
// 0xf58964 — j___ZNSt6vectorIS_IbSaIbEESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_0xf58964() -> ! {
    todo!("0xf58964 j___ZNSt6vectorIS_IbSaIbEESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<std::vector<bool,std::allocator<bool>>,std::allocator<std::vector<bool,std::allocator<bool>>>>::_M_erase_at_end(std::vector<bool,std::allocator<bool>>*)")]
// 0xf58974 — j___ZNSt6vectorIS_IbSaIbEESaIS1_EE15_M_erase_at_endEPS1_
pub fn stub_0xf58974() -> ! {
    todo!("0xf58974 j___ZNSt6vectorIS_IbSaIbEESaIS1_EE15_M_erase_at_endEPS1_")
}

#[doc(alias = "std::vector<std::vector<bool,std::allocator<bool>>,std::allocator<std::vector<bool,std::allocator<bool>>>>::resize(unsigned long,std::vector<bool,std::allocator<bool>>)")]
// 0xf58984 — j___ZNSt6vectorIS_IbSaIbEESaIS1_EE6resizeEmS1_
pub fn stub_0xf58984() -> ! {
    todo!("0xf58984 j___ZNSt6vectorIS_IbSaIbEESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::_M_initialize(unsigned long)")]
// 0xf58994 — j___ZNSt6vectorIbSaIbEE13_M_initializeEm
pub fn stub_0xf58994() -> ! {
    todo!("0xf58994 j___ZNSt6vectorIbSaIbEE13_M_initializeEm")
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::vector(std::vector<bool,std::allocator<bool>> const&)")]
// 0xf589a4 — j___ZNSt6vectorIbSaIbEEC2ERKS1_
pub fn stub_0xf589a4() -> ! {
    todo!("0xf589a4 j___ZNSt6vectorIbSaIbEEC2ERKS1_")
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::vector(unsigned long,bool const&,std::allocator<bool> const&)")]
// 0xf589b4 — j___ZNSt6vectorIbSaIbEEC2EmRKbRKS0_
pub fn stub_0xf589b4() -> ! {
    todo!("0xf589b4 j___ZNSt6vectorIbSaIbEEC2EmRKbRKS0_")
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::operator=(std::vector<bool,std::allocator<bool>> const&)")]
// 0xf589c4 — j___ZNSt6vectorIbSaIbEEaSERKS1_
pub fn stub_0xf589c4() -> ! {
    todo!("0xf589c4 j___ZNSt6vectorIbSaIbEEaSERKS1_")
}

#[doc(alias = "std::vector<short,std::allocator<short>>::resize(unsigned long,short)")]
// 0xf589d4 — j___ZNSt6vectorIsSaIsEE6resizeEms
pub fn stub_0xf589d4() -> ! {
    todo!("0xf589d4 j___ZNSt6vectorIsSaIsEE6resizeEms")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<std::vector<bool,std::allocator<bool>> *,unsigned long,std::vector<bool,std::allocator<bool>>>(std::vector<bool,std::allocator<bool>> *,unsigned long,std::vector<bool,std::allocator<bool>> const&,std::__false_type)")]
// 0xf58a04 — j___ZSt26__uninitialized_fill_n_auxIPSt6vectorIbSaIbEEmS2_EvT_T0_RKT1_St12__false_type
pub fn stub_0xf58a04() -> ! {
    todo!("0xf58a04 j___ZSt26__uninitialized_fill_n_auxIPSt6vectorIbSaIbEEmS2_EvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "RBX::ControlledLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::internalMakeEvictable(std::string const&,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)")]
// 0xf58a34 — j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE21internalMakeEvictableERKSsRKS5_m
pub fn stub_0xf58a34() -> ! {
    todo!("0xf58a34 j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE21internalMakeEvictableERKSsRKS5_m")
}

#[doc(alias = "RBX::ControlledLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::fetch(std::string const&,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>*,bool)")]
// 0xf58a44 — j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE5fetchERKSsPS5_b
pub fn stub_0xf58a44() -> ! {
    todo!("0xf58a44 j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE5fetchERKSsPS5_b")
}

#[doc(alias = "RBX::ControlledLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::remove(std::string const&)")]
// 0xf58a54 — j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6removeERKSs
pub fn stub_0xf58a54() -> ! {
    todo!("0xf58a54 j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6removeERKSs")
}

#[doc(alias = "RBX::ControlledLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::resize(unsigned long)")]
// 0xf58a64 — j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6resizeEm
pub fn stub_0xf58a64() -> ! {
    todo!("0xf58a64 j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6resizeEm")
}

#[doc(alias = "RBX::ControlledLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::ControlledLRUCache(unsigned long,RBX::CacheSizeEnforceMethod)")]
// 0xf58a74 — j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEEC2EmNS_22CacheSizeEnforceMethodE
pub fn stub_0xf58a74() -> ! {
    todo!("0xf58a74 j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEEC2EmNS_22CacheSizeEnforceMethodE")
}

#[doc(alias = "RBX::ControlledLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::~ControlledLRUCache()")]
// 0xf58a84 — j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEED2Ev
pub fn stub_0xf58a84() -> ! {
    todo!("0xf58a84 j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEED2Ev")
}

#[doc(alias = "RBX::CacheableContentProvider::CachedItem::~CachedItem()")]
// 0xf58a94 — j___ZN3RBX24CacheableContentProvider10CachedItemD2Ev
pub fn stub_0xf58a94() -> ! {
    todo!("0xf58a94 j___ZN3RBX24CacheableContentProvider10CachedItemD2Ev")
}

#[doc(alias = "RBX::ConcurrentControlledLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::ConcurrentControlledLRUCache(unsigned long,unsigned long,RBX::CacheSizeEnforceMethod)")]
// 0xf58aa4 — j___ZN3RBX28ConcurrentControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEEC2EmmNS_22CacheSizeEnforceMethodE
pub fn stub_0xf58aa4() -> ! {
    todo!("0xf58aa4 j___ZN3RBX28ConcurrentControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEEC2EmmNS_22CacheSizeEnforceMethodE")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::insert(std::string const&,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)")]
// 0xf58ac4 — j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6insertERKSsRKS5_m
pub fn stub_0xf58ac4() -> ! {
    todo!("0xf58ac4 j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6insertERKSsRKS5_m")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::LRUCache(void)")]
// 0xf58ad4 — j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEEC2Ev
pub fn stub_0xf58ad4() -> ! {
    todo!("0xf58ad4 j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEEC2Ev")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::~LRUCache()")]
// 0xf58ae4 — j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEED2Ev
pub fn stub_0xf58ae4() -> ! {
    todo!("0xf58ae4 j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEED2Ev")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::CacheableContentProvider> RBX::weak_from<RBX::CacheableContentProvider>(RBX::CacheableContentProvider*)")]
// 0xf58af4 — j___ZN3RBX9weak_fromINS_24CacheableContentProviderEEEN5boost8weak_ptrIT_EEPS4_
pub fn stub_0xf58af4() -> ! {
    todo!("0xf58af4 j___ZN3RBX9weak_fromINS_24CacheableContentProviderEEEN5boost8weak_ptrIT_EEPS4_")
}

#[doc(alias = "boost::scoped_ptr<RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>::~scoped_ptr()")]
// 0xf58b04 — j___ZN5boost10scoped_ptrIN3RBX8LRUCacheISsNS_10shared_ptrINS1_24CacheableContentProvider10CachedItemEEEEEED2Ev
pub fn stub_0xf58b04() -> ! {
    todo!("0xf58b04 j___ZN5boost10scoped_ptrIN3RBX8LRUCacheISsNS_10shared_ptrINS1_24CacheableContentProvider10CachedItemEEEEEED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContentProviderJob>::shared_ptr<RBX::ContentProviderJob>(RBX::ContentProviderJob *)")]
// 0xf58b14 — j___ZN5boost10shared_ptrIN3RBX18ContentProviderJobEEC2IS2_EEPT_
pub fn stub_0xf58b14() -> ! {
    todo!("0xf58b14 j___ZN5boost10shared_ptrIN3RBX18ContentProviderJobEEC2IS2_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContentProviderJob>::operator=(rbx_core::SharedPtr<RBX::ContentProviderJob> const&)")]
// 0xf58b24 — j___ZN5boost10shared_ptrIN3RBX18ContentProviderJobEEaSERKS3_
pub fn stub_0xf58b24() -> ! {
    todo!("0xf58b24 j___ZN5boost10shared_ptrIN3RBX18ContentProviderJobEEaSERKS3_")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>::reset<RBX::CacheableContentProvider::CachedItem>(RBX::CacheableContentProvider::CachedItem *)")]
// 0xf58b34 — j___ZN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEE5resetIS3_EEvPT_
pub fn stub_0xf58b34() -> ! {
    todo!("0xf58b34 j___ZN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEE5resetIS3_EEvPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>::shared_ptr<RBX::CacheableContentProvider::CachedItem>(RBX::CacheableContentProvider::CachedItem *)")]
// 0xf58b44 — j___ZN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEC2IS3_EEPT_
pub fn stub_0xf58b44() -> ! {
    todo!("0xf58b44 j___ZN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEC2IS3_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>::operator=(rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem> const&)")]
// 0xf58b54 — j___ZN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEaSERKS4_
pub fn stub_0xf58b54() -> ! {
    todo!("0xf58b54 j___ZN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEaSERKS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CacheableContentProvider>::shared_ptr<RBX::CacheableContentProvider>(rbx_core::WeakPtr<RBX::CacheableContentProvider> const&,boost::detail::sp_nothrow_tag)")]
// 0xf58b64 — j___ZN5boost10shared_ptrIN3RBX24CacheableContentProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
pub fn stub_0xf58b64() -> ! {
    todo!("0xf58b64 j___ZN5boost10shared_ptrIN3RBX24CacheableContentProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>)")]
// 0xf58b74 — j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEEEC2ES7_S9_
pub fn stub_0xf58b74() -> ! {
    todo!("0xf58b74 j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEEEC2ES7_S9_")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>>::operator()<void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&) &,boost::_bi::list1<std::string &> &,int)")]
// 0xf58b84 — j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEEEclIPFvS6_RKSsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0xf58b84() -> ! {
    todo!("0xf58b84 j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEEEclIPFvS6_RKSsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>)")]
// 0xf58b94 — j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
pub fn stub_0xf58b94() -> ! {
    todo!("0xf58b94 j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_")
}

#[doc(alias = "RBX::TaskScheduler::StepResult boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>::operator()<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<std::string &,rbx_core::SharedPtr<std::string const>&>>(boost::_bi::type<RBX::TaskScheduler::StepResult>,RBX::TaskScheduler::StepResult (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>) &,boost::_bi::list2<std::string &,rbx_core::SharedPtr<std::string const>&> &,long)")]
// 0xf58ba4 — j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEclINS4_13TaskScheduler10StepResultEPFSE_S6_RKSsNS_10shared_ptrISF_EEENS0_5list2IRSsRSI_EEEET_NS0_4typeISP_EERT0_RT1_l
pub fn stub_0xf58ba4() -> ! {
    todo!("0xf58ba4 j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEclINS4_13TaskScheduler10StepResultEPFSE_S6_RKSsNS_10shared_ptrISF_EEENS0_5list2IRSsRSI_EEEET_NS0_4typeISP_EERT0_RT1_l")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>)")]
// 0xf58bb4 — j___ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEC2ES7_S9_SA_SB_SC_
pub fn stub_0xf58bb4() -> ! {
    todo!("0xf58bb4 j___ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEC2ES7_S9_SA_SB_SC_")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>::operator()<void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&> &,int)")]
// 0xf58bc4 — j___ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS0_5list3IRSG_RSH_RSK_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0xf58bc4() -> ! {
    todo!("0xf58bc4 j___ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS0_5list3IRSG_RSH_RSK_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>)")]
// 0xf58bd4 — j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEEEC2ES7_S9_
pub fn stub_0xf58bd4() -> ! {
    todo!("0xf58bd4 j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEEEC2ES7_S9_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>)")]
// 0xf58be4 — j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
pub fn stub_0xf58be4() -> ! {
    todo!("0xf58be4 j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
// 0xf58bf4 — j___ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEEEC2ES7_S9_SA_SB_
pub fn stub_0xf58bf4() -> ! {
    todo!("0xf58bf4 j___ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEEEC2ES7_S9_SA_SB_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>)")]
// 0xf58c04 — j___ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEC2ES7_S9_SA_SB_SC_
pub fn stub_0xf58c04() -> ! {
    todo!("0xf58c04 j___ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEC2ES7_S9_SA_SB_SC_")
}

#[doc(alias = "boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>>::type> boost::bind<RBX::TaskScheduler::StepResult,rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::WeakPtr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>>(RBX::TaskScheduler::StepResult (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>),rbx_core::WeakPtr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>)")]
// 0xf58c14 — j___ZN5boost4bindIN3RBX13TaskScheduler10StepResultENS_8weak_ptrINS1_24CacheableContentProviderEEERKSsNS_10shared_ptrIS7_EES6_NS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_PFSG_T0_T1_T2_ENSE_9list_av_3IT3_T4_T5_E4typeEEESL_SN_SO_SP_
pub fn stub_0xf58c14() -> ! {
    todo!("0xf58c14 j___ZN5boost4bindIN3RBX13TaskScheduler10StepResultENS_8weak_ptrINS1_24CacheableContentProviderEEERKSsNS_10shared_ptrIS7_EES6_NS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_PFSG_T0_T1_T2_ENSE_9list_av_3IT3_T4_T5_E4typeEEESL_SN_SO_SP_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string>::type> boost::bind<void,rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string,rbx_core::WeakPtr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string>(void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string),rbx_core::WeakPtr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string)")]
// 0xf58c24 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS2_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsS4_NS_3argILi1EEENSB_ILi2EEENSB_ILi3EEESsEENS_3_bi6bind_tIT_PFSH_T0_T1_T2_T3_T4_ENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESO_SQ_SR_SS_ST_SU_
pub fn stub_0xf58c24() -> ! {
    todo!("0xf58c24 j___ZN5boost4bindIvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS2_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsS4_NS_3argILi1EEENSB_ILi2EEENSB_ILi3EEESsEENS_3_bi6bind_tIT_PFSH_T0_T1_T2_T3_T4_ENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESO_SQ_SR_SS_ST_SU_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::CacheableContentProvider>,boost::arg<1>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::WeakPtr<RBX::CacheableContentProvider>,boost::arg<1>>(void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&),rbx_core::WeakPtr<RBX::CacheableContentProvider>,boost::arg<1>)")]
// 0xf58c34 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsS4_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
pub fn stub_0xf58c34() -> ! {
    todo!("0xf58c34 j___ZN5boost4bindIvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsS4_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContentProviderJob>(RBX::ContentProviderJob *)")]
// 0xf58c44 — j___ZN5boost6detail12shared_countC2IN3RBX18ContentProviderJobEEEPT_
pub fn stub_0xf58c44() -> ! {
    todo!("0xf58c44 j___ZN5boost6detail12shared_countC2IN3RBX18ContentProviderJobEEEPT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf58c54 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0xf58c54() -> ! {
    todo!("0xf58c54 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0xf58c64 — j___ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
pub fn stub_0xf58c64() -> ! {
    todo!("0xf58c64 j___ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0xf58c74 — j___ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEE12manage_smallERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
pub fn stub_0xf58c74() -> ! {
    todo!("0xf58c74 j___ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEE12manage_smallERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::CacheableContentProvider>::weak_ptr<RBX::CacheableContentProvider>(rbx_core::SharedPtr<RBX::CacheableContentProvider> const&,boost::detail::sp_enable_if_convertible<RBX::CacheableContentProvider,RBX::CacheableContentProvider>::type)")]
// 0xf58cb4 — j___ZN5boost8weak_ptrIN3RBX24CacheableContentProviderEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
pub fn stub_0xf58cb4() -> ! {
    todo!("0xf58cb4 j___ZN5boost8weak_ptrIN3RBX24CacheableContentProviderEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")
}

#[doc(alias = "void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>>>)")]
// 0xf58cc4 — j___ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEEEvT_
pub fn stub_0xf58cc4() -> ! {
    todo!("0xf58cc4 j___ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEEEvT_")
}

#[doc(alias = "boost::function2<RBX::TaskScheduler::StepResult,std::string,rbx_core::SharedPtr<std::string const>>::clear(void)")]
// 0xf58ce4 — j___ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE5clearEv
pub fn stub_0xf58ce4() -> ! {
    todo!("0xf58ce4 j___ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE5clearEv")
}

#[doc(alias = "void boost::function2<RBX::TaskScheduler::StepResult,std::string,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>)")]
// 0xf58cf4 — j___ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS_8weak_ptrINS1_24CacheableContentProviderEEERS5_S6_ENS9_5list3INS9_5valueISD_EENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_
pub fn stub_0xf58cf4() -> ! {
    todo!("0xf58cf4 j___ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS_8weak_ptrINS1_24CacheableContentProviderEEERS5_S6_ENS9_5list3INS9_5valueISD_EENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_")
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>)")]
// 0xf58d14 — j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_24CacheableContentProviderEEES3_S4_S7_SsENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEEEvT_
pub fn stub_0xf58d14() -> ! {
    todo!("0xf58d14 j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_24CacheableContentProviderEEES3_S4_S7_SsENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEEEvT_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>> *)")]
// 0xf58d34 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISF_EESP_
pub fn stub_0xf58d34() -> ! {
    todo!("0xf58d34 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISF_EESP_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// 0xf58d44 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
pub fn stub_0xf58d44() -> ! {
    todo!("0xf58d44 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>> const&)")]
// 0xf58d54 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISF_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEbERS5_RKT_
pub fn stub_0xf58d54() -> ! {
    todo!("0xf58d54 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISF_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEbERS5_RKT_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0xf58d64 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISL_EEPNS1_10ptr_bucketE
pub fn stub_0xf58d64() -> ! {
    todo!("0xf58d64 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISL_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>> const&)")]
// 0xf58d74 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISF_EEEEvRKT_
pub fn stub_0xf58d74() -> ! {
    todo!("0xf58d74 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISF_EEEEvRKT_")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>>>::construct(void)")]
// 0xf58d84 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEEEEE9constructEv
pub fn stub_0xf58d84() -> ! {
    todo!("0xf58d84 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>>>::~node_constructor()")]
// 0xf58d94 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEEEEED2Ev
pub fn stub_0xf58d94() -> ! {
    todo!("0xf58d94 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEEEEED2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// 0xf58da4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
pub fn stub_0xf58da4() -> ! {
    todo!("0xf58da4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xf58db4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_0xf58db4() -> ! {
    todo!("0xf58db4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0xf58dc4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_0xf58dc4() -> ! {
    todo!("0xf58dc4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xf58dd4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_0xf58dd4() -> ! {
    todo!("0xf58dd4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// 0xf58de4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE5clearEv
pub fn stub_0xf58de4() -> ! {
    todo!("0xf58de4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE5clearEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>> const&)")]
// 0xf58df4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSI_RKSK_RKSaINS1_8ptr_nodeISF_EEE
pub fn stub_0xf58df4() -> ! {
    todo!("0xf58df4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSI_RKSK_RKSaINS1_8ptr_nodeISF_EEE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0xf58e04 — j___ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_0xf58e04() -> ! {
    todo!("0xf58e04 j___ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0xf58e14 — j___ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_0xf58e14() -> ! {
    todo!("0xf58e14 j___ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::ContentProviderJob,RBX::ContentProviderJob>(rbx_core::SharedPtr<RBX::ContentProviderJob> const*,RBX::ContentProviderJob *)const")]
// 0xf58e24 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_18ContentProviderJobES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_0xf58e24() -> ! {
    todo!("0xf58e24 j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_18ContentProviderJobES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0xf58e34 — j___ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS5_5list2INS5_5valueISA_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0xf58e34() -> ! {
    todo!("0xf58e34 j___ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS5_5list2INS5_5valueISA_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf58e44 — j___ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS5_5list2INS5_5valueISA_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0xf58e44() -> ! {
    todo!("0xf58e44 j___ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS5_5list2INS5_5valueISA_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::TaskScheduler::StepResult,std::string,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// 0xf58e54 — j___ZNK5boost6detail8function13basic_vtable2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_8weak_ptrINS3_24CacheableContentProviderEEERS7_S8_ENSB_5list3INSB_5valueISF_EENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0xf58e54() -> ! {
    todo!("0xf58e54 j___ZNK5boost6detail8function13basic_vtable2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_8weak_ptrINS3_24CacheableContentProviderEEERS7_S8_ENSB_5list3INSB_5valueISF_EENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::TaskScheduler::StepResult,std::string,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf58e64 — j___ZNK5boost6detail8function13basic_vtable2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_8weak_ptrINS3_24CacheableContentProviderEEERS7_S8_ENSB_5list3INSB_5valueISF_EENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0xf58e64() -> ! {
    todo!("0xf58e64 j___ZNK5boost6detail8function13basic_vtable2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_8weak_ptrINS3_24CacheableContentProviderEEERS7_S8_ENSB_5list3INSB_5valueISF_EENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf58e74 — j___ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_0xf58e74() -> ! {
    todo!("0xf58e74 j___ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// 0xf58e84 — j___ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0xf58e84() -> ! {
    todo!("0xf58e84 j___ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf58e94 — j___ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0xf58e94() -> ! {
    todo!("0xf58e94 j___ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// 0xf58ea4 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
pub fn stub_0xf58ea4() -> ! {
    todo!("0xf58ea4 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>::_M_clear(void)")]
// 0xf58eb4 — j___ZNSt10_List_baseISt4pairISsS0_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEESaIS8_EE8_M_clearEv
pub fn stub_0xf58eb4() -> ! {
    todo!("0xf58eb4 j___ZNSt10_List_baseISt4pairISsS0_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEESaIS8_EE8_M_clearEv")
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>> const&)")]
// 0xf58ec4 — j___ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEESaIS8_EE14_M_create_nodeERKS8_
pub fn stub_0xf58ec4() -> ! {
    todo!("0xf58ec4 j___ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEESaIS8_EE14_M_create_nodeERKS8_")
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>)")]
// 0xf58ed4 — j___ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEESaIS8_EE8_M_eraseESt14_List_iteratorIS8_E
pub fn stub_0xf58ed4() -> ! {
    todo!("0xf58ed4 j___ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEESaIS8_EE8_M_eraseESt14_List_iteratorIS8_E")
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>::pair(std::string const&,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>> const&)")]
// 0xf58ee4 — j___ZNSt4pairISsS_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEC2ERKSsRKS6_
pub fn stub_0xf58ee4() -> ! {
    todo!("0xf58ee4 j___ZNSt4pairISsS_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEC2ERKSsRKS6_")
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_pop_front_aux(void)")]
// 0xf58f64 — j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE16_M_pop_front_auxEv
pub fn stub_0xf58f64() -> ! {
    todo!("0xf58f64 j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE16_M_pop_front_auxEv")
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_push_back_aux(RBX::BindableFunction::Invocation const&)")]
// 0xf58f74 — j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE16_M_push_back_auxERKS2_
pub fn stub_0xf58f74() -> ! {
    todo!("0xf58f74 j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE16_M_push_back_auxERKS2_")
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_reallocate_map(unsigned long,bool)")]
// 0xf58f84 — j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE17_M_reallocate_mapEmb
pub fn stub_0xf58f84() -> ! {
    todo!("0xf58f84 j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_reserve_map_at_back(unsigned long)")]
// 0xf58f94 — j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE22_M_reserve_map_at_backEm
pub fn stub_0xf58f94() -> ! {
    todo!("0xf58f94 j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE22_M_reserve_map_at_backEm")
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::pop_front(void)")]
// 0xf58fa4 — j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE9pop_frontEv
pub fn stub_0xf58fa4() -> ! {
    todo!("0xf58fa4 j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE9pop_frontEv")
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::push_back(RBX::BindableFunction::Invocation const&)")]
// 0xf58fb4 — j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE9push_backERKS2_
pub fn stub_0xf58fb4() -> ! {
    todo!("0xf58fb4 j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "RBX::FixedArray<RBX::BuoyancyConnector *,8ul>::push_back(RBX::BuoyancyConnector * const&)")]
// 0xf58fc4 — j___ZN3RBX10FixedArrayIPNS_17BuoyancyConnectorELm8EE9push_backERKS2_
pub fn stub_0xf58fc4() -> ! {
    todo!("0xf58fc4 j___ZN3RBX10FixedArrayIPNS_17BuoyancyConnectorELm8EE9push_backERKS2_")
}

#[doc(alias = "RBX::FixedArray<RBX::BuoyancyConnector *,8ul>::operator[](unsigned long)")]
// 0xf58fd4 — j___ZN3RBX10FixedArrayIPNS_17BuoyancyConnectorELm8EEixEm
pub fn stub_0xf58fd4() -> ! {
    todo!("0xf58fd4 j___ZN3RBX10FixedArrayIPNS_17BuoyancyConnectorELm8EEixEm")
}

#[doc(alias = "RBX::Constants::getKmsGravity(void)")]
// 0xf58fe4 — j___ZN3RBX9Constants13getKmsGravityEv
pub fn stub_0xf58fe4() -> ! {
    todo!("0xf58fe4 j___ZN3RBX9Constants13getKmsGravityEv")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::maxSwimmingMoveForce(void)")]
// 0xf59014 — j___ZN3RBX5HUMAN13HumanoidState20maxSwimmingMoveForceEv
pub fn stub_0xf59014() -> ! {
    todo!("0xf59014 j___ZN3RBX5HUMAN13HumanoidState20maxSwimmingMoveForceEv")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::minSwimmingMoveForce(void)")]
// 0xf59024 — j___ZN3RBX5HUMAN13HumanoidState20minSwimmingMoveForceEv
pub fn stub_0xf59024() -> ! {
    todo!("0xf59024 j___ZN3RBX5HUMAN13HumanoidState20minSwimmingMoveForceEv")
}

#[doc(alias = "RBX::DoubleEndedVector<unsigned int>::grow(void)")]
// 0xf59034 — j___ZN3RBX17DoubleEndedVectorIjE4growEv
pub fn stub_0xf59034() -> ! {
    todo!("0xf59034 j___ZN3RBX17DoubleEndedVectorIjE4growEv")
}

#[doc(alias = "RBX::DoubleEndedVector<unsigned int>::pop_front(unsigned int *)")]
// 0xf59044 — j___ZN3RBX17DoubleEndedVectorIjE9pop_frontEPj
pub fn stub_0xf59044() -> ! {
    todo!("0xf59044 j___ZN3RBX17DoubleEndedVectorIjE9pop_frontEPj")
}

#[doc(alias = "RBX::ConstraintSurfacePair::~ConstraintSurfacePair()")]
// 0xf59054 — j___ZN3RBX21ConstraintSurfacePairD2Ev
pub fn stub_0xf59054() -> ! {
    todo!("0xf59054 j___ZN3RBX21ConstraintSurfacePairD2Ev")
}

#[doc(alias = "std::_Vector_base<RBX::ConstraintSurfacePair *,std::allocator<RBX::ConstraintSurfacePair *>>::_M_allocate(unsigned long)")]
// 0xf59094 — j___ZNSt12_Vector_baseIPN3RBX21ConstraintSurfacePairESaIS2_EE11_M_allocateEm
pub fn stub_0xf59094() -> ! {
    todo!("0xf59094 j___ZNSt12_Vector_baseIPN3RBX21ConstraintSurfacePairESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::vector<RBX::ConstraintSurfacePair *,std::allocator<RBX::ConstraintSurfacePair *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ConstraintSurfacePair **,std::vector<RBX::ConstraintSurfacePair *,std::allocator<RBX::ConstraintSurfacePair *>>>,RBX::ConstraintSurfacePair * const&)")]
// 0xf590a4 — j___ZNSt6vectorIPN3RBX21ConstraintSurfacePairESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf590a4() -> ! {
    todo!("0xf590a4 j___ZNSt6vectorIPN3RBX21ConstraintSurfacePairESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::ConstraintSurfacePair *,std::allocator<RBX::ConstraintSurfacePair *>>::push_back(RBX::ConstraintSurfacePair * const&)")]
// 0xf590b4 — j___ZNSt6vectorIPN3RBX21ConstraintSurfacePairESaIS2_EE9push_backERKS2_
pub fn stub_0xf590b4() -> ! {
    todo!("0xf590b4 j___ZNSt6vectorIPN3RBX21ConstraintSurfacePairESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Primitive **,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Primitive **,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>>,RBX::Primitive *>(__gnu_cxx::__normal_iterator<RBX::Primitive **,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>>,__gnu_cxx::__normal_iterator<RBX::Primitive **,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>>,RBX::Primitive * const&,std::random_access_iterator_tag)")]
// 0xf590c4 — j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX9PrimitiveESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
pub fn stub_0xf590c4() -> ! {
    todo!("0xf590c4 j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX9PrimitiveESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag")
}
