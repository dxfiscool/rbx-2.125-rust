//! core shard BH — 100 core stubs EA-sorted, next uncovered after BG 0x48cdac (strict RBX|boost|std|rbx earliest gap, after BG 0x48cdc4..0x49825c).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x48cdac.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "RBX::TaskScheduler::Job::SleepAdjustMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *>(RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *)")]
// 0x48cdc4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler3Job17SleepAdjustMethodES7_EET0_T_S9_S8_ — RBX::TaskScheduler::Job::SleepAdjustMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *>(RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *)
pub fn stub_48cdc4() -> ! {
    todo!("0x48cdc4 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler3Job17SleepAdjustMethodES7_EET0_T_S9_S8_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
// 0x48ce00 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_ — std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod const&)
pub fn stub_48ce00() -> ! {
    todo!("0x48ce00 __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::resize(unsigned long,RBX::TaskScheduler::PriorityMethod)")]
// 0x48cf90 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE6resizeEmS2_ — std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::resize(unsigned long,RBX::TaskScheduler::PriorityMethod)
pub fn stub_48cf90() -> ! {
    todo!("0x48cf90 __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::push_back(RBX::TaskScheduler::PriorityMethod const&)")]
// 0x48cfc4 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE9push_backERKS2_ — std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::push_back(RBX::TaskScheduler::PriorityMethod const&)
pub fn stub_48cfc4() -> ! {
    todo!("0x48cfc4 __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::PriorityMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::operator[](RBX::Name const* const&)")]
// 0x48cfec — __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler14PriorityMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::TaskScheduler::PriorityMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::operator[](RBX::Name const* const&)
pub fn stub_48cfec() -> ! {
    todo!("0x48cfec __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler14PriorityMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
// 0x48d044 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)
pub fn stub_48d044() -> ! {
    todo!("0x48d044 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
// 0x48d0f8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)
pub fn stub_48d0f8() -> ! {
    todo!("0x48d0f8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
// 0x48d150 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)
pub fn stub_48d150() -> ! {
    todo!("0x48d150 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,RBX::TaskScheduler::PriorityMethod const&)")]
// 0x48d1b8 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,RBX::TaskScheduler::PriorityMethod const&)
pub fn stub_48d1b8() -> ! {
    todo!("0x48d1b8 __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_allocate(unsigned long)")]
// 0x48d29c — __ZNSt12_Vector_baseIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_allocate(unsigned long)
pub fn stub_48d29c() -> ! {
    todo!("0x48d29c __ZNSt12_Vector_baseIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::TaskScheduler::PriorityMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *>(RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *)")]
// 0x48d2b4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler14PriorityMethodES6_EET0_T_S8_S7_ — RBX::TaskScheduler::PriorityMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *>(RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *)
pub fn stub_48d2b4() -> ! {
    todo!("0x48d2b4 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler14PriorityMethodES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,unsigned long,RBX::TaskScheduler::PriorityMethod const&)")]
// 0x48d2f0 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,unsigned long,RBX::TaskScheduler::PriorityMethod const&)
pub fn stub_48d2f0() -> ! {
    todo!("0x48d2f0 __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::resize(unsigned long,RBX::TaskScheduler::ThreadPoolConfig)")]
// 0x48d480 — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE6resizeEmS2_ — std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::resize(unsigned long,RBX::TaskScheduler::ThreadPoolConfig)
pub fn stub_48d480() -> ! {
    todo!("0x48d480 __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::ThreadPoolConfig,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::operator[](RBX::Name const* const&)")]
// 0x48d4b4 — __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler16ThreadPoolConfigESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::TaskScheduler::ThreadPoolConfig,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::operator[](RBX::Name const* const&)
pub fn stub_48d4b4() -> ! {
    todo!("0x48d4b4 __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler16ThreadPoolConfigESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
// 0x48d50c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)
pub fn stub_48d50c() -> ! {
    todo!("0x48d50c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
// 0x48d5c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)
pub fn stub_48d5c0() -> ! {
    todo!("0x48d5c0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
// 0x48d618 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)
pub fn stub_48d618() -> ! {
    todo!("0x48d618 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,unsigned long,RBX::TaskScheduler::ThreadPoolConfig const&)")]
// 0x48d680 — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,unsigned long,RBX::TaskScheduler::ThreadPoolConfig const&)
pub fn stub_48d680() -> ! {
    todo!("0x48d680 __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_allocate(unsigned long)")]
// 0x48d810 — __ZNSt12_Vector_baseIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_allocate(unsigned long)
pub fn stub_48d810() -> ! {
    todo!("0x48d810 __ZNSt12_Vector_baseIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::TaskScheduler::ThreadPoolConfig * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *>(RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *)")]
// 0x48d828 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler16ThreadPoolConfigES6_EET0_T_S8_S7_ — RBX::TaskScheduler::ThreadPoolConfig * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *>(RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *)
pub fn stub_48d828() -> ! {
    todo!("0x48d828 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler16ThreadPoolConfigES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::push_back(RBX::TaskScheduler::ThreadPoolConfig const&)")]
// 0x48d864 — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE9push_backERKS2_ — std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::push_back(RBX::TaskScheduler::ThreadPoolConfig const&)
pub fn stub_48d864() -> ! {
    todo!("0x48d864 __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,RBX::TaskScheduler::ThreadPoolConfig const&)")]
// 0x48d88c — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,RBX::TaskScheduler::ThreadPoolConfig const&)
pub fn stub_48d88c() -> ! {
    todo!("0x48d88c __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "DummyJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x48dc2c — __ZN8DummyJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE — DummyJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
pub fn stub_48dc2c() -> ! {
    todo!("0x48dc2c __ZN8DummyJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE")
}

#[doc(alias = "DummyJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x48dc34 — __ZN8DummyJob5errorERKN3RBX13TaskScheduler3Job5StatsE — DummyJob::error(RBX::TaskScheduler::Job::Stats const&)
pub fn stub_48dc34() -> ! {
    todo!("0x48dc34 __ZN8DummyJob5errorERKN3RBX13TaskScheduler3Job5StatsE")
}

#[doc(alias = "DummyJob::step(RBX::TaskScheduler::Job::Stats const&)")]
// 0x48dc58 — __ZN8DummyJob4stepERKN3RBX13TaskScheduler3Job5StatsE — DummyJob::step(RBX::TaskScheduler::Job::Stats const&)
pub fn stub_48dc58() -> ! {
    todo!("0x48dc58 __ZN8DummyJob4stepERKN3RBX13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::rate(void)const")]
// 0x48dc60 — __ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv — RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::rate(void)const
pub fn stub_48dc60() -> ! {
    todo!("0x48dc60 __ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>> *)")]
// 0x48dcc0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>> *)
pub fn stub_48dcc0() -> ! {
    todo!("0x48dcc0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>> *)")]
// 0x48dce8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>> *)
pub fn stub_48dce8() -> ! {
    todo!("0x48dce8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>> *)")]
// 0x48dd10 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>> *)
pub fn stub_48dd10() -> ! {
    todo!("0x48dd10 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>> *)")]
// 0x48dd38 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>> *)
pub fn stub_48dd38() -> ! {
    todo!("0x48dd38 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>> *)")]
// 0x48dd60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>> *)
pub fn stub_48dd60() -> ! {
    todo!("0x48dd60 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>> *)")]
// 0x48dd88 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>> *)
pub fn stub_48dd88() -> ! {
    todo!("0x48dd88 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "RBX::Decal::setTexture(RBX::TextureId)")]
// 0x48f7f4 — __ZN3RBX5Decal10setTextureENS_9TextureIdE — RBX::Decal::setTexture(RBX::TextureId)
pub fn stub_48f7f4() -> ! {
    todo!("0x48f7f4 __ZN3RBX5Decal10setTextureENS_9TextureIdE")
}

#[doc(alias = "RBX::Decal::setSpecular(float)")]
// 0x48f82c — __ZN3RBX5Decal11setSpecularEf — RBX::Decal::setSpecular(float)
pub fn stub_48f82c() -> ! {
    todo!("0x48f82c __ZN3RBX5Decal11setSpecularEf")
}

#[doc(alias = "RBX::Decal::setShiny(float)")]
// 0x48f860 — __ZN3RBX5Decal8setShinyEf — RBX::Decal::setShiny(float)
pub fn stub_48f860() -> ! {
    todo!("0x48f860 __ZN3RBX5Decal8setShinyEf")
}

#[doc(alias = "RBX::Decal::setTransparency(float)")]
// 0x48f894 — __ZN3RBX5Decal15setTransparencyEf — RBX::Decal::setTransparency(float)
pub fn stub_48f894() -> ! {
    todo!("0x48f894 __ZN3RBX5Decal15setTransparencyEf")
}

#[doc(alias = "RBX::Decal::Decal(void)")]
// 0x48f8bc — __ZN3RBX5DecalC2Ev — RBX::Decal::Decal(void)
pub fn stub_48f8bc() -> ! {
    todo!("0x48f8bc __ZN3RBX5DecalC2Ev")
}

#[doc(alias = "RBX::StringConverter<RBX::TextureId>::convertToValue(std::string const&,RBX::TextureId&)")]
// 0x48fb04 — __ZN3RBX15StringConverterINS_9TextureIdEE14convertToValueERKSsRS1_ — RBX::StringConverter<RBX::TextureId>::convertToValue(std::string const&,RBX::TextureId&)
pub fn stub_48fb04() -> ! {
    todo!("0x48fb04 __ZN3RBX15StringConverterINS_9TextureIdEE14convertToValueERKSsRS1_")
}

#[doc(alias = "RBX::Texture::setStudsPerTileU(float)")]
// 0x49047c — __ZN3RBX7Texture16setStudsPerTileUEf — RBX::Texture::setStudsPerTileU(float)
pub fn stub_49047c() -> ! {
    todo!("0x49047c __ZN3RBX7Texture16setStudsPerTileUEf")
}

#[doc(alias = "RBX::Texture::setStudsPerTileV(float)")]
// 0x4904b0 — __ZN3RBX7Texture16setStudsPerTileVEf — RBX::Texture::setStudsPerTileV(float)
pub fn stub_4904b0() -> ! {
    todo!("0x4904b0 __ZN3RBX7Texture16setStudsPerTileVEf")
}

#[doc(alias = "RBX::Texture::Texture(void)")]
// 0x4904e4 — __ZN3RBX7TextureC2Ev — RBX::Texture::Texture(void)
pub fn stub_4904e4() -> ! {
    todo!("0x4904e4 __ZN3RBX7TextureC2Ev")
}

#[doc(alias = "RBX::Decal::getTexture(void)const")]
// 0x49076c — __ZNK3RBX5Decal10getTextureEv — RBX::Decal::getTexture(void)const
pub fn stub_49076c() -> ! {
    todo!("0x49076c __ZNK3RBX5Decal10getTextureEv")
}

#[doc(alias = "RBX::Decal::getSpecular(void)const")]
// 0x490794 — __ZNK3RBX5Decal11getSpecularEv — RBX::Decal::getSpecular(void)const
pub fn stub_490794() -> ! {
    todo!("0x490794 __ZNK3RBX5Decal11getSpecularEv")
}

#[doc(alias = "RBX::Decal::getShiny(void)const")]
// 0x4907c0 — __ZNK3RBX5Decal8getShinyEv — RBX::Decal::getShiny(void)const
pub fn stub_4907c0() -> ! {
    todo!("0x4907c0 __ZNK3RBX5Decal8getShinyEv")
}

#[doc(alias = "RBX::Decal::getTransparency(void)const")]
// 0x4907c8 — __ZNK3RBX5Decal15getTransparencyEv — RBX::Decal::getTransparency(void)const
pub fn stub_4907c8() -> ! {
    todo!("0x4907c8 __ZNK3RBX5Decal15getTransparencyEv")
}

#[doc(alias = "RBX::Texture::getStudsPerTileU(void)const")]
// 0x490a7c — __ZNK3RBX7Texture16getStudsPerTileUEv — RBX::Texture::getStudsPerTileU(void)const
pub fn stub_490a7c() -> ! {
    todo!("0x490a7c __ZNK3RBX7Texture16getStudsPerTileUEv")
}

#[doc(alias = "RBX::Texture::getStudsPerTileV(void)const")]
// 0x490aa8 — __ZNK3RBX7Texture16getStudsPerTileVEv — RBX::Texture::getStudsPerTileV(void)const
pub fn stub_490aa8() -> ! {
    todo!("0x490aa8 __ZNK3RBX7Texture16getStudsPerTileVEv")
}

#[doc(alias = "RBX::Decal::~Decal()")]
// 0x490ab8 — __ZN3RBX5DecalD1Ev — RBX::Decal::~Decal()
pub fn stub_490ab8() -> ! {
    todo!("0x490ab8 __ZN3RBX5DecalD1Ev")
}

#[doc(alias = "RBX::Decal::~Decal()")]
// 0x490af8 — __ZN3RBX5DecalD0Ev — RBX::Decal::~Decal()
pub fn stub_490af8() -> ! {
    todo!("0x490af8 __ZN3RBX5DecalD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Decal::~Decal()")]
// 0x490be4 — __ZThn32_N3RBX5DecalD1Ev — non-virtual thunk toRBX::Decal::~Decal()
pub fn stub_490be4() -> ! {
    todo!("0x490be4 __ZThn32_N3RBX5DecalD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Decal::~Decal()")]
// 0x490c28 — __ZThn32_N3RBX5DecalD0Ev — non-virtual thunk toRBX::Decal::~Decal()
pub fn stub_490c28() -> ! {
    todo!("0x490c28 __ZThn32_N3RBX5DecalD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Decal::~Decal()")]
// 0x490d14 — __ZThn36_N3RBX5DecalD1Ev — non-virtual thunk toRBX::Decal::~Decal()
pub fn stub_490d14() -> ! {
    todo!("0x490d14 __ZThn36_N3RBX5DecalD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Decal::~Decal()")]
// 0x490d58 — __ZThn36_N3RBX5DecalD0Ev — non-virtual thunk toRBX::Decal::~Decal()
pub fn stub_490d58() -> ! {
    todo!("0x490d58 __ZThn36_N3RBX5DecalD0Ev")
}

#[doc(alias = "RBX::Texture::~Texture()")]
// 0x490e34 — __ZN3RBX7TextureD1Ev — RBX::Texture::~Texture()
pub fn stub_490e34() -> ! {
    todo!("0x490e34 __ZN3RBX7TextureD1Ev")
}

#[doc(alias = "RBX::Texture::~Texture()")]
// 0x490e74 — __ZN3RBX7TextureD0Ev — RBX::Texture::~Texture()
pub fn stub_490e74() -> ! {
    todo!("0x490e74 __ZN3RBX7TextureD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Texture::~Texture()")]
// 0x490f60 — __ZThn32_N3RBX7TextureD1Ev — non-virtual thunk toRBX::Texture::~Texture()
pub fn stub_490f60() -> ! {
    todo!("0x490f60 __ZThn32_N3RBX7TextureD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Texture::~Texture()")]
// 0x490fa4 — __ZThn32_N3RBX7TextureD0Ev — non-virtual thunk toRBX::Texture::~Texture()
pub fn stub_490fa4() -> ! {
    todo!("0x490fa4 __ZThn32_N3RBX7TextureD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Texture::~Texture()")]
// 0x491090 — __ZThn36_N3RBX7TextureD1Ev — non-virtual thunk toRBX::Texture::~Texture()
pub fn stub_491090() -> ! {
    todo!("0x491090 __ZThn36_N3RBX7TextureD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Texture::~Texture()")]
// 0x4910d4 — __ZThn36_N3RBX7TextureD0Ev — non-virtual thunk toRBX::Texture::~Texture()
pub fn stub_4910d4() -> ! {
    todo!("0x4910d4 __ZThn36_N3RBX7TextureD0Ev")
}

#[doc(alias = "RBX::TextureId * rbx::any_cast<RBX::TextureId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x4929f8 — __ZN3rbx8any_castIN3RBX9TextureIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE — RBX::TextureId * rbx::any_cast<RBX::TextureId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
pub fn stub_4929f8() -> ! {
    todo!("0x4929f8 __ZN3rbx8any_castIN3RBX9TextureIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::TextureId & rbx::any_cast<RBX::TextureId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x492a50 — __ZN3rbx8any_castIRN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::TextureId & rbx::any_cast<RBX::TextureId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_492a50() -> ! {
    todo!("0x492a50 __ZN3rbx8any_castIRN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::DialogChoice::setUserDialog(std::string)")]
// 0x493660 — __ZN3RBX12DialogChoice13setUserDialogESs — RBX::DialogChoice::setUserDialog(std::string)
pub fn stub_493660() -> ! {
    todo!("0x493660 __ZN3RBX12DialogChoice13setUserDialogESs")
}

#[doc(alias = "RBX::DialogChoice::setResponseDialog(std::string)")]
// 0x4937d4 — __ZN3RBX12DialogChoice17setResponseDialogESs — RBX::DialogChoice::setResponseDialog(std::string)
pub fn stub_4937d4() -> ! {
    todo!("0x4937d4 __ZN3RBX12DialogChoice17setResponseDialogESs")
}

#[doc(alias = "RBX::DialogChoice::DialogChoice(void)")]
// 0x493810 — __ZN3RBX12DialogChoiceC2Ev — RBX::DialogChoice::DialogChoice(void)
pub fn stub_493810() -> ! {
    todo!("0x493810 __ZN3RBX12DialogChoiceC2Ev")
}

#[doc(alias = "RBX::DialogChoice::getUserDialog(void)const")]
// 0x493b28 — __ZNK3RBX12DialogChoice13getUserDialogEv — RBX::DialogChoice::getUserDialog(void)const
pub fn stub_493b28() -> ! {
    todo!("0x493b28 __ZNK3RBX12DialogChoice13getUserDialogEv")
}

#[doc(alias = "RBX::DialogChoice::getResponseDialog(void)const")]
// 0x493b58 — __ZNK3RBX12DialogChoice17getResponseDialogEv — RBX::DialogChoice::getResponseDialog(void)const
pub fn stub_493b58() -> ! {
    todo!("0x493b58 __ZNK3RBX12DialogChoice17getResponseDialogEv")
}

#[doc(alias = "RBX::DialogChoice::~DialogChoice()")]
// 0x493b64 — __ZN3RBX12DialogChoiceD1Ev — RBX::DialogChoice::~DialogChoice()
pub fn stub_493b64() -> ! {
    todo!("0x493b64 __ZN3RBX12DialogChoiceD1Ev")
}

#[doc(alias = "RBX::DialogChoice::~DialogChoice()")]
// 0x493cb8 — __ZN3RBX12DialogChoiceD0Ev — RBX::DialogChoice::~DialogChoice()
pub fn stub_493cb8() -> ! {
    todo!("0x493cb8 __ZN3RBX12DialogChoiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DialogChoice::~DialogChoice()")]
// 0x493d68 — __ZThn32_N3RBX12DialogChoiceD1Ev — non-virtual thunk toRBX::DialogChoice::~DialogChoice()
pub fn stub_493d68() -> ! {
    todo!("0x493d68 __ZThn32_N3RBX12DialogChoiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DialogChoice::~DialogChoice()")]
// 0x493ebc — __ZThn32_N3RBX12DialogChoiceD0Ev — non-virtual thunk toRBX::DialogChoice::~DialogChoice()
pub fn stub_493ebc() -> ! {
    todo!("0x493ebc __ZThn32_N3RBX12DialogChoiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DialogChoice::~DialogChoice()")]
// 0x494038 — __ZThn36_N3RBX12DialogChoiceD1Ev — non-virtual thunk toRBX::DialogChoice::~DialogChoice()
pub fn stub_494038() -> ! {
    todo!("0x494038 __ZThn36_N3RBX12DialogChoiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DialogChoice::~DialogChoice()")]
// 0x49418c — __ZThn36_N3RBX12DialogChoiceD0Ev — non-virtual thunk toRBX::DialogChoice::~DialogChoice()
pub fn stub_49418c() -> ! {
    todo!("0x49418c __ZThn36_N3RBX12DialogChoiceD0Ev")
}

#[doc(alias = "RBX::DialogRoot::setInitialPrompt(std::string)")]
// 0x495428 — __ZN3RBX10DialogRoot16setInitialPromptESs — RBX::DialogRoot::setInitialPrompt(std::string)
pub fn stub_495428() -> ! {
    todo!("0x495428 __ZN3RBX10DialogRoot16setInitialPromptESs")
}

#[doc(alias = "RBX::DialogRoot::setDialogPurpose(RBX::DialogRoot::DialogPurpose)")]
// 0x495464 — __ZN3RBX10DialogRoot16setDialogPurposeENS0_13DialogPurposeE — RBX::DialogRoot::setDialogPurpose(RBX::DialogRoot::DialogPurpose)
pub fn stub_495464() -> ! {
    todo!("0x495464 __ZN3RBX10DialogRoot16setDialogPurposeENS0_13DialogPurposeE")
}

#[doc(alias = "RBX::DialogRoot::setDialogTone(RBX::DialogRoot::DialogTone)")]
// 0x495480 — __ZN3RBX10DialogRoot13setDialogToneENS0_10DialogToneE — RBX::DialogRoot::setDialogTone(RBX::DialogRoot::DialogTone)
pub fn stub_495480() -> ! {
    todo!("0x495480 __ZN3RBX10DialogRoot13setDialogToneENS0_10DialogToneE")
}

#[doc(alias = "RBX::DialogRoot::setConversationDistance(float)")]
// 0x49549c — __ZN3RBX10DialogRoot23setConversationDistanceEf — RBX::DialogRoot::setConversationDistance(float)
pub fn stub_49549c() -> ! {
    todo!("0x49549c __ZN3RBX10DialogRoot23setConversationDistanceEf")
}

#[doc(alias = "RBX::DialogRoot::setInUse(bool)")]
// 0x4954c4 — __ZN3RBX10DialogRoot8setInUseEb — RBX::DialogRoot::setInUse(bool)
pub fn stub_4954c4() -> ! {
    todo!("0x4954c4 __ZN3RBX10DialogRoot8setInUseEb")
}

#[doc(alias = "RBX::DialogRoot::DialogRoot(void)")]
// 0x495c94 — __ZN3RBX10DialogRootC2Ev — RBX::DialogRoot::DialogRoot(void)
pub fn stub_495c94() -> ! {
    todo!("0x495c94 __ZN3RBX10DialogRootC2Ev")
}

#[doc(alias = "RBX::DialogRoot::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x495f88 — __ZN3RBX10DialogRoot17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::DialogRoot::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_495f88() -> ! {
    todo!("0x495f88 __ZN3RBX10DialogRoot17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "RBX::DialogRoot::getInitialPrompt(void)const")]
// 0x496158 — __ZNK3RBX10DialogRoot16getInitialPromptEv — RBX::DialogRoot::getInitialPrompt(void)const
pub fn stub_496158() -> ! {
    todo!("0x496158 __ZNK3RBX10DialogRoot16getInitialPromptEv")
}

#[doc(alias = "RBX::DialogRoot::getDialogPurpose(void)const")]
// 0x496188 — __ZNK3RBX10DialogRoot16getDialogPurposeEv — RBX::DialogRoot::getDialogPurpose(void)const
pub fn stub_496188() -> ! {
    todo!("0x496188 __ZNK3RBX10DialogRoot16getDialogPurposeEv")
}

#[doc(alias = "RBX::DialogRoot::getDialogTone(void)const")]
// 0x4961b0 — __ZNK3RBX10DialogRoot13getDialogToneEv — RBX::DialogRoot::getDialogTone(void)const
pub fn stub_4961b0() -> ! {
    todo!("0x4961b0 __ZNK3RBX10DialogRoot13getDialogToneEv")
}

#[doc(alias = "RBX::DialogRoot::getConversationDistance(void)const")]
// 0x4961d8 — __ZNK3RBX10DialogRoot23getConversationDistanceEv — RBX::DialogRoot::getConversationDistance(void)const
pub fn stub_4961d8() -> ! {
    todo!("0x4961d8 __ZNK3RBX10DialogRoot23getConversationDistanceEv")
}

#[doc(alias = "RBX::DialogRoot::getInUse(void)const")]
// 0x496200 — __ZNK3RBX10DialogRoot8getInUseEv — RBX::DialogRoot::getInUse(void)const
pub fn stub_496200() -> ! {
    todo!("0x496200 __ZNK3RBX10DialogRoot8getInUseEv")
}

#[doc(alias = "RBX::DialogRoot::~DialogRoot()")]
// 0x496d48 — __ZN3RBX10DialogRootD1Ev — RBX::DialogRoot::~DialogRoot()
pub fn stub_496d48() -> ! {
    todo!("0x496d48 __ZN3RBX10DialogRootD1Ev")
}

#[doc(alias = "RBX::DialogRoot::~DialogRoot()")]
// 0x496d4c — __ZN3RBX10DialogRootD0Ev — RBX::DialogRoot::~DialogRoot()
pub fn stub_496d4c() -> ! {
    todo!("0x496d4c __ZN3RBX10DialogRootD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DialogRoot::~DialogRoot()")]
// 0x496dfc — __ZThn32_N3RBX10DialogRootD1Ev — non-virtual thunk toRBX::DialogRoot::~DialogRoot()
pub fn stub_496dfc() -> ! {
    todo!("0x496dfc __ZThn32_N3RBX10DialogRootD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DialogRoot::~DialogRoot()")]
// 0x496e04 — __ZThn32_N3RBX10DialogRootD0Ev — non-virtual thunk toRBX::DialogRoot::~DialogRoot()
pub fn stub_496e04() -> ! {
    todo!("0x496e04 __ZThn32_N3RBX10DialogRootD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DialogRoot::~DialogRoot()")]
// 0x496eb8 — __ZThn36_N3RBX10DialogRootD1Ev — non-virtual thunk toRBX::DialogRoot::~DialogRoot()
pub fn stub_496eb8() -> ! {
    todo!("0x496eb8 __ZThn36_N3RBX10DialogRootD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DialogRoot::~DialogRoot()")]
// 0x496ec0 — __ZThn36_N3RBX10DialogRootD0Ev — non-virtual thunk toRBX::DialogRoot::~DialogRoot()
pub fn stub_496ec0() -> ! {
    todo!("0x496ec0 __ZThn36_N3RBX10DialogRootD0Ev")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::resize(unsigned long,RBX::DialogRoot::DialogTone)")]
// 0x497efc — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE6resizeEmS2_ — std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::resize(unsigned long,RBX::DialogRoot::DialogTone)
pub fn stub_497efc() -> ! {
    todo!("0x497efc __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::push_back(RBX::DialogRoot::DialogTone const&)")]
// 0x497f30 — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE9push_backERKS2_ — std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::push_back(RBX::DialogRoot::DialogTone const&)
pub fn stub_497f30() -> ! {
    todo!("0x497f30 __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DialogRoot::DialogTone,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::operator[](RBX::Name const* const&)")]
// 0x497f58 — __ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot10DialogToneESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::DialogRoot::DialogTone,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::operator[](RBX::Name const* const&)
pub fn stub_497f58() -> ! {
    todo!("0x497f58 __ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot10DialogToneESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
// 0x497fb0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)
pub fn stub_497fb0() -> ! {
    todo!("0x497fb0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
// 0x498064 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)
pub fn stub_498064() -> ! {
    todo!("0x498064 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
// 0x4980bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)
pub fn stub_4980bc() -> ! {
    todo!("0x4980bc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,RBX::DialogRoot::DialogTone const&)")]
// 0x498124 — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,RBX::DialogRoot::DialogTone const&)
pub fn stub_498124() -> ! {
    todo!("0x498124 __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_allocate(unsigned long)")]
// 0x498208 — __ZNSt12_Vector_baseIN3RBX10DialogRoot10DialogToneESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_allocate(unsigned long)
pub fn stub_498208() -> ! {
    todo!("0x498208 __ZNSt12_Vector_baseIN3RBX10DialogRoot10DialogToneESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::DialogRoot::DialogTone * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *>(RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *)")]
// 0x498220 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot10DialogToneES6_EET0_T_S8_S7_ — RBX::DialogRoot::DialogTone * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *>(RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *)
pub fn stub_498220() -> ! {
    todo!("0x498220 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot10DialogToneES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,unsigned long,RBX::DialogRoot::DialogTone const&)")]
// 0x49825c — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,unsigned long,RBX::DialogRoot::DialogTone const&)
pub fn stub_49825c() -> ! {
    todo!("0x49825c __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}
