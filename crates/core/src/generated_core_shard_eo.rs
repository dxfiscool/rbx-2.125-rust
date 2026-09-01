//! core shard EO — 100 core stubs EA-sorted, lowest uncovered 0x9a3014..0x9ef0cc (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EN 0x9a3014).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned char>,std::_Select1st<std::pair<std::string const,unsigned char>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned char>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,unsigned char>>,std::_Rb_tree_iterator<std::pair<std::string const,unsigned char>>)")]
// 0x9a3014 — __ZNSt8_Rb_treeISsSt4pairIKSshESt10_Select1stIS2_ESt4lessISsESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
pub fn stub_9a3014() -> ! {
    todo!("0x9a3014 __ZNSt8_Rb_treeISsSt4pairIKSshESt10_Select1stIS2_ESt4lessISsESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned char>,std::_Select1st<std::pair<std::string const,unsigned char>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned char>>>::_M_insert_unique(std::pair<std::string const,unsigned char> const&)")]
// 0x9a30cc — __ZNSt8_Rb_treeISsSt4pairIKSshESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_9a30cc() -> ! {
    todo!("0x9a30cc __ZNSt8_Rb_treeISsSt4pairIKSshESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned char>,std::_Select1st<std::pair<std::string const,unsigned char>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned char>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,unsigned char> const&)")]
// 0x9a31b0 — __ZNSt8_Rb_treeISsSt4pairIKSshESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_9a31b0() -> ! {
    todo!("0x9a31b0 __ZNSt8_Rb_treeISsSt4pairIKSshESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "RBX::PhysicsService::begin(void)")]
// 0x9a8ee0 — __ZN3RBX14PhysicsService5beginEv
pub fn stub_9a8ee0() -> ! {
    todo!("0x9a8ee0 __ZN3RBX14PhysicsService5beginEv")
}

#[doc(alias = "boost::intrusive::rbtree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::rebalance_after_insertion(boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> *)")]
// 0x9a9348 — __ZN5boost9intrusive17rbtree_algorithmsINS0_18rbtree_node_traitsIPvLb0EEEE25rebalance_after_insertionERKPNS0_11rbtree_nodeIS3_EES8_
// was: boost::intrusive::rbtree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::rebalance_after_insertion(boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> *)
pub fn stub_9a9348() -> ! {
    todo!("0x9a9348 __ZN5boost9intrusive17rbtree_algorithmsINS0_18rbtree_node_traitsIPvLb0EEEE25rebalance_after_insertionERKPNS0_11rbtree_nodeIS3_EES8_")
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::~clone_impl()")]
// 0x9aa0d8 — __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEED1Ev
// was: non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::~clone_impl()
pub fn stub_9aa0d8() -> ! {
    todo!("0x9aa0d8 __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::~clone_impl()")]
// 0x9aa190 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEED0Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::~clone_impl()
pub fn stub_9aa190() -> ! {
    todo!("0x9aa190 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::rethrow(void)const")]
// 0x9aa250 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEE7rethrowEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::rethrow(void)const
pub fn stub_9aa250() -> ! {
    todo!("0x9aa250 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEE7rethrowEv")
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::~clone_impl()")]
// 0x9aa378 — __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEED0Ev
// was: non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::~clone_impl()
pub fn stub_9aa378() -> ! {
    todo!("0x9aa378 __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEED0Ev")
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::rethrow(void)const")]
// 0x9aa438 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEE7rethrowEv
// was: virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::rethrow(void)const
pub fn stub_9aa438() -> ! {
    todo!("0x9aa438 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEE7rethrowEv")
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::~clone_impl()")]
// 0x9aa448 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEED0Ev
// was: virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::~clone_impl()
pub fn stub_9aa448() -> ! {
    todo!("0x9aa448 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEED0Ev")
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::error_info_injector<std::bad_alloc>::~error_info_injector()")]
// 0x9aa520 — __ZThn4_N5boost16exception_detail19error_info_injectorISt9bad_allocED0Ev
// was: non-virtual thunk to boost::exception_detail::error_info_injector<std::bad_alloc>::~error_info_injector()
pub fn stub_9aa520() -> ! {
    todo!("0x9aa520 __ZThn4_N5boost16exception_detail19error_info_injectorISt9bad_allocED0Ev")
}

#[doc(alias = "boost::intrusive::rbtree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::rebalance_after_erasure(boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> *,boost::intrusive::rbtree_node<void *> *)")]
// 0x9aa5dc — __ZN5boost9intrusive17rbtree_algorithmsINS0_18rbtree_node_traitsIPvLb0EEEE23rebalance_after_erasureERKPNS0_11rbtree_nodeIS3_EES8_S8_
// was: boost::intrusive::rbtree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::rebalance_after_erasure(boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> *,boost::intrusive::rbtree_node<void *> *)
pub fn stub_9aa5dc() -> ! {
    todo!("0x9aa5dc __ZN5boost9intrusive17rbtree_algorithmsINS0_18rbtree_node_traitsIPvLb0EEEE23rebalance_after_erasureERKPNS0_11rbtree_nodeIS3_EES8_S8_")
}

#[doc(alias = "boost::intrusive::detail::tree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::erase_impl(boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::detail::tree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::data_for_rebalance &)")]
// 0x9aa7d8 — __ZN5boost9intrusive6detail15tree_algorithmsINS0_18rbtree_node_traitsIPvLb0EEEE10erase_implERKPNS0_11rbtree_nodeIS4_EESB_RNS6_18data_for_rebalanceE
// was: boost::intrusive::detail::tree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::erase_impl(boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::detail::tree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::data_for_rebalance &)
pub fn stub_9aa7d8() -> ! {
    todo!("0x9aa7d8 __ZN5boost9intrusive6detail15tree_algorithmsINS0_18rbtree_node_traitsIPvLb0EEEE10erase_implERKPNS0_11rbtree_nodeIS4_EESB_RNS6_18data_for_rebalanceE")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::PhysicsService>(void)")]
// 0x9ab3d8 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14PhysicsServiceEEEvv
pub fn stub_9ab3d8() -> ! {
    todo!("0x9ab3d8 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14PhysicsServiceEEEvv")
}

#[doc(alias = "RBX::MechanismItem::~MechanismItem()")]
// 0x9ae8a8 — __ZN3RBX13MechanismItemD1Ev
pub fn stub_9ae8a8() -> ! {
    todo!("0x9ae8a8 __ZN3RBX13MechanismItemD1Ev")
}

#[doc(alias = "RBX::MechanismItem::~MechanismItem()")]
// 0x9ae8b4 — __ZN3RBX13MechanismItemD2Ev
pub fn stub_9ae8b4() -> ! {
    todo!("0x9ae8b4 __ZN3RBX13MechanismItemD2Ev")
}

#[doc(alias = "RBX::MechanismItem::reset(int)")]
// 0x9ae9d0 — __ZN3RBX13MechanismItem5resetEi
pub fn stub_9ae9d0() -> ! {
    todo!("0x9ae9d0 __ZN3RBX13MechanismItem5resetEi")
}

#[doc(alias = "RBX::MechanismItem::appendAssembly(void)")]
// 0x9aeaa8 — __ZN3RBX13MechanismItem14appendAssemblyEv
pub fn stub_9aeaa8() -> ! {
    todo!("0x9aeaa8 __ZN3RBX13MechanismItem14appendAssemblyEv")
}

#[doc(alias = "RBX::MechanismItem::consistent(RBX::MechanismItem const*,RBX::MechanismItem const*)")]
// 0x9aecb8 — __ZN3RBX13MechanismItem10consistentEPKS0_S2_
pub fn stub_9aecb8() -> ! {
    todo!("0x9aecb8 __ZN3RBX13MechanismItem10consistentEPKS0_S2_")
}

#[doc(alias = "RBX::MechanismItem::lerp(RBX::MechanismItem const*,RBX::MechanismItem const*,RBX::MechanismItem*,float)")]
// 0x9aee00 — __ZN3RBX13MechanismItem4lerpEPKS0_S2_PS0_f
pub fn stub_9aee00() -> ! {
    todo!("0x9aee00 __ZN3RBX13MechanismItem4lerpEPKS0_S2_PS0_f")
}

#[doc(alias = "RBX::AssemblyItem::AssemblyItem(void)")]
// 0x9af8d0 — __ZN3RBX12AssemblyItemC2Ev
pub fn stub_9af8d0() -> ! {
    todo!("0x9af8d0 __ZN3RBX12AssemblyItemC2Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,PacketPriority>,std::_Select1st<std::pair<RBX::Name const* const,PacketPriority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,PacketPriority>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,PacketPriority>>,std::pair<RBX::Name const* const,PacketPriority> const&)")]
// 0x9b8ea0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_14PacketPriorityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_9b8ea0() -> ! {
    todo!("0x9b8ea0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_14PacketPriorityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,PacketPriority>,std::_Select1st<std::pair<RBX::Name const* const,PacketPriority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,PacketPriority>>>::_M_insert_unique(std::pair<RBX::Name const* const,PacketPriority> const&)")]
// 0x9b9054 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_14PacketPriorityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_9b9054() -> ! {
    todo!("0x9b9054 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_14PacketPriorityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::vector<PacketPriority,std::allocator<PacketPriority>>::_M_insert_aux(__gnu_cxx::__normal_iterator<PacketPriority*,std::vector<PacketPriority,std::allocator<PacketPriority>>>,PacketPriority const&)")]
// 0x9b9144 — __ZNSt6vectorI14PacketPrioritySaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_
pub fn stub_9b9144() -> ! {
    todo!("0x9b9144 __ZNSt6vectorI14PacketPrioritySaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_")
}

#[doc(alias = "std::vector<PacketPriority,std::allocator<PacketPriority>>::_M_fill_insert(__gnu_cxx::__normal_iterator<PacketPriority*,std::vector<PacketPriority,std::allocator<PacketPriority>>>,unsigned long,PacketPriority const&)")]
// 0x9b9254 — __ZNSt6vectorI14PacketPrioritySaIS0_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS0_S2_EEmRKS0_
pub fn stub_9b9254() -> ! {
    todo!("0x9b9254 __ZNSt6vectorI14PacketPrioritySaIS0_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS0_S2_EEmRKS0_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,PacketReliability>,std::_Select1st<std::pair<RBX::Name const* const,PacketReliability>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,PacketReliability>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,PacketReliability>>,std::pair<RBX::Name const* const,PacketReliability> const&)")]
// 0x9b93fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_17PacketReliabilityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_9b93fc() -> ! {
    todo!("0x9b93fc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_17PacketReliabilityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,PacketReliability>,std::_Select1st<std::pair<RBX::Name const* const,PacketReliability>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,PacketReliability>>>::_M_insert_unique(std::pair<RBX::Name const* const,PacketReliability> const&)")]
// 0x9b95b0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_17PacketReliabilityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_9b95b0() -> ! {
    todo!("0x9b95b0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_17PacketReliabilityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::vector<PacketReliability,std::allocator<PacketReliability>>::_M_insert_aux(__gnu_cxx::__normal_iterator<PacketReliability*,std::vector<PacketReliability,std::allocator<PacketReliability>>>,PacketReliability const&)")]
// 0x9b96a0 — __ZNSt6vectorI17PacketReliabilitySaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_
pub fn stub_9b96a0() -> ! {
    todo!("0x9b96a0 __ZNSt6vectorI17PacketReliabilitySaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_")
}

#[doc(alias = "std::vector<PacketReliability,std::allocator<PacketReliability>>::_M_fill_insert(__gnu_cxx::__normal_iterator<PacketReliability*,std::vector<PacketReliability,std::allocator<PacketReliability>>>,unsigned long,PacketReliability const&)")]
// 0x9b97b0 — __ZNSt6vectorI17PacketReliabilitySaIS0_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS0_S2_EEmRKS0_
pub fn stub_9b97b0() -> ! {
    todo!("0x9b97b0 __ZNSt6vectorI17PacketReliabilitySaIS0_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS0_S2_EEmRKS0_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Primitive *)>::operator()(RBX::Primitive *)")]
// 0x9bec1c — __ZN3rbx7signals16signal_with_argsILi1EFvPN3RBX9PrimitiveEEEclES4_
pub fn stub_9bec1c() -> ! {
    todo!("0x9bec1c __ZN3rbx7signals16signal_with_argsILi1EFvPN3RBX9PrimitiveEEEclES4_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> &)")]
// 0x9bf028 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(RBX::Primitive *)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> &)
pub fn stub_9bf028() -> ! {
    todo!("0x9bf028 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::mutex(void)")]
// 0x9bf22c — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE5mutexEv
pub fn stub_9bf22c() -> ! {
    todo!("0x9bf22c __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE5mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::insert(rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot *)")]
// 0x9c4168 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE6insertEPNS7_4slotE
pub fn stub_9c4168() -> ! {
    todo!("0x9c4168 __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE6insertEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::mutex(void)")]
// 0x9c4420 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE5mutexEv
pub fn stub_9c4420() -> ! {
    todo!("0x9c4420 __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE5mutexEv")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot*)")]
// 0x9c4534 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot*)
pub fn stub_9c4534() -> ! {
    todo!("0x9c4534 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotEEaSEPSA_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot> const&)")]
// 0x9c45e8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotEEaSERKSB_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot> const&)
pub fn stub_9c45e8() -> ! {
    todo!("0x9c45e8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotEEaSERKSB_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot::disconnect(void)")]
// 0x9c4800 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slot10disconnectEv
pub fn stub_9c4800() -> ! {
    todo!("0x9c4800 __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot::connected(void)const")]
// 0x9c4974 — __ZNK3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slot9connectedEv
pub fn stub_9c4974() -> ! {
    todo!("0x9c4974 __ZNK3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slot9connectedEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::remove(rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot *)")]
// 0x9c49b8 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE6removeEPNS7_4slotE
pub fn stub_9c49b8() -> ! {
    todo!("0x9c49b8 __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE6removeEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot::safe_static_init_mutex(void)")]
// 0x9c4aa4 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slot22safe_static_init_mutexEv
pub fn stub_9c4aa4() -> ! {
    todo!("0x9c4aa4 __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot::~slot()")]
// 0x9c4b8c — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotD1Ev
pub fn stub_9c4b8c() -> ! {
    todo!("0x9c4b8c __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot::~slot()")]
// 0x9c4be8 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotD0Ev
pub fn stub_9c4be8() -> ! {
    todo!("0x9c4be8 __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotD0Ev")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::TouchPair>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::TouchPair>,RBX::TouchPair,boost::hash<RBX::TouchPair>,std::equal_to<RBX::TouchPair>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::TouchPair>>(RBX::TouchPair const&,boost::unordered::detail::emplace_args1<RBX::TouchPair> const&)")]
// 0x9c4cf0 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEE12emplace_implINS1_13emplace_args1IS5_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS5_EEEEbERKS5_RKT_
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::TouchPair>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::TouchPair>,RBX::TouchPair,boost::hash<RBX::TouchPair>,std::equal_to<RBX::TouchPair>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::TouchPair>>(RBX::TouchPair const&,boost::unordered::detail::emplace_args1<RBX::TouchPair> const&)
pub fn stub_9c4cf0() -> ! {
    todo!("0x9c4cf0 __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEE12emplace_implINS1_13emplace_args1IS5_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS5_EEEEbERKS5_RKT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::TouchPair>,RBX::TouchPair,boost::hash<RBX::TouchPair>,std::equal_to<RBX::TouchPair>>>::reserve_for_insert(unsigned long)")]
// 0x9c5058 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEE18reserve_for_insertEm
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::TouchPair>,RBX::TouchPair,boost::hash<RBX::TouchPair>,std::equal_to<RBX::TouchPair>>>::reserve_for_insert(unsigned long)
pub fn stub_9c5058() -> ! {
    todo!("0x9c5058 __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::TouchPair>,RBX::TouchPair,boost::hash<RBX::TouchPair>,std::equal_to<RBX::TouchPair>>>::create_buckets(unsigned long)")]
// 0x9c5200 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEE14create_bucketsEm
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::TouchPair>,RBX::TouchPair,boost::hash<RBX::TouchPair>,std::equal_to<RBX::TouchPair>>>::create_buckets(unsigned long)
pub fn stub_9c5200() -> ! {
    todo!("0x9c5200 __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::TouchPair>,RBX::TouchPair,boost::hash<RBX::TouchPair>,std::equal_to<RBX::TouchPair>>>::~table()")]
// 0x9c53d0 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEED2Ev
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::TouchPair>,RBX::TouchPair,boost::hash<RBX::TouchPair>,std::equal_to<RBX::TouchPair>>>::~table()
pub fn stub_9c53d0() -> ! {
    todo!("0x9c53d0 __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEED2Ev")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::~vector()")]
// 0x9cb070 — __ZNSt6vectorISsSaISsEED1Ev
pub fn stub_9cb070() -> ! {
    todo!("0x9cb070 __ZNSt6vectorISsSaISsEED1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::reset(void)")]
// 0x9cb36c — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEE5resetEv
// was: boost::shared_ptr<RBX::TaskScheduler::Job>::reset(void)
pub fn stub_9cb36c() -> ! {
    todo!("0x9cb36c __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEE5resetEv")
}

#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,long,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::right>,std::less<long>,boost::multi_index::detail::nth_layer<1,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::right,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::insert_(boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true> const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>> *)")]
// 0x9ce4b4 — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE7insert_ERKSP_PNS1_18ordered_index_nodeINS15_INS1_15index_node_baseISP_SU_EEEEEE
// was: boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,long,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::right>,std::less<long>,boost::multi_index::detail::nth_layer<1,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::right,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::insert_(boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true> const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>> *)
pub fn stub_9ce4b4() -> ! {
    todo!("0x9ce4b4 __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE7insert_ERKSP_PNS1_18ordered_index_nodeINS15_INS1_15index_node_baseISP_SU_EEEEEE")
}

#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,long,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::right>,std::less<long>,boost::multi_index::detail::nth_layer<1,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::right,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::link_point(long,boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,long,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::right>,std::less<long>,boost::multi_index::detail::nth_layer<1,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::right,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::link_info &,boost::multi_index::detail::ordered_unique_tag)")]
// 0x9ce548 — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE10link_pointElRNS12_9link_infoES11_
// was: boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,long,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::right>,std::less<long>,boost::multi_index::detail::nth_layer<1,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::right,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::link_point(long,boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,long,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::right>,std::less<long>,boost::multi_index::detail::nth_layer<1,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::right,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::link_info &,boost::multi_index::detail::ordered_unique_tag)
pub fn stub_9ce548() -> ! {
    todo!("0x9ce548 __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE10link_pointElRNS12_9link_infoES11_")
}

#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::insert_(boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true> const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>> *)")]
// 0x9ce620 — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEESsXadL_ZNSH_4leftEEEEESt4lessISsENS1_9nth_layerILi2ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISC_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE7insert_ERKSP_PNS1_18ordered_index_nodeINS1_15index_node_baseISP_SU_EEEE
// was: boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::insert_(boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true> const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>> *)
pub fn stub_9ce620() -> ! {
    todo!("0x9ce620 __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEESsXadL_ZNSH_4leftEEEEESt4lessISsENS1_9nth_layerILi2ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISC_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE7insert_ERKSP_PNS1_18ordered_index_nodeINS1_15index_node_baseISP_SU_EEEE")
}

#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::link_point(std::string const&,boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::link_info &,boost::multi_index::detail::ordered_unique_tag)")]
// 0x9ce6b8 — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEESsXadL_ZNSH_4leftEEEEESt4lessISsENS1_9nth_layerILi2ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISC_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE10link_pointERSA_RNS12_9link_infoES11_
// was: boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::link_point(std::string const&,boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::link_info &,boost::multi_index::detail::ordered_unique_tag)
pub fn stub_9ce6b8() -> ! {
    todo!("0x9ce6b8 __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEESsXadL_ZNSH_4leftEEEEESt4lessISsENS1_9nth_layerILi2ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISC_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE10link_pointERSA_RNS12_9link_infoES11_")
}

#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,long,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::right>,std::less<long>,boost::multi_index::detail::nth_layer<1,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::right,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::delete_all_nodes(boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>> *)")]
// 0x9ce810 — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE16delete_all_nodesEPNS1_18ordered_index_nodeINS13_INS1_15index_node_baseISP_SU_EEEEEE
// was: boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,long,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::right>,std::less<long>,boost::multi_index::detail::nth_layer<1,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::right,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::delete_all_nodes(boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>> *)
pub fn stub_9ce810() -> ! {
    todo!("0x9ce810 __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE16delete_all_nodesEPNS1_18ordered_index_nodeINS13_INS1_15index_node_baseISP_SU_EEEEEE")
}

#[doc(alias = "serializeSFFlag(std::string const&,std::string const&,void *)")]
// 0x9e2054 — __ZL15serializeSFFlagRKSsS0_Pv
pub fn stub_9e2054() -> ! {
    todo!("0x9e2054 __ZL15serializeSFFlagRKSsS0_Pv")
}

#[doc(alias = "boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>> boost::bimaps::container_adaptor::associative_container_adaptor<boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>,boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,std::string const,boost::bimaps::container_adaptor::support::iterator_facade_to_base<boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,mpl_::na,boost::bimaps::relation::detail::pair_to_relation_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,boost::bimaps::relation::support::get_pair_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,mpl_::na,boost::mpl::v_item<boost::bimaps::container_adaptor::detail::iterator_from_base_identity<boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::const_reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,boost::mpl::vector<mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,1>>::find<std::string>(std::string const&)")]
// 0x9e438c — __ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_
// was: boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>> boost::bimaps::container_adaptor::associative_container_adaptor<boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>,boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,std::string const,boost::bimaps::container_adaptor::support::iterator_facade_to_base<boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,mpl_::na,boost::bimaps::relation::detail::pair_to_relation_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,boost::bimaps::relation::support::get_pair_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,mpl_::na,boost::mpl::v_item<boost::bimaps::container_adaptor::detail::iterator_from_base_identity<boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::const_reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,boost::mpl::vector<mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,1>>::find<std::string>(std::string const&)
pub fn stub_9e438c() -> ! {
    todo!("0x9e438c __ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(int,bool,int)>::operator()(int,bool,int)")]
// 0x9e4de0 — __ZN3rbx7signals16signal_with_argsILi3EFvibiEEclEibi
pub fn stub_9e4de0() -> ! {
    todo!("0x9e4de0 __ZN3rbx7signals16signal_with_argsILi3EFvibiEEclEibi")
}

#[doc(alias = "RBX::ObjectValue::~ObjectValue()")]
// 0x9e63e0 — __ZN3RBX11ObjectValueD1Ev
pub fn stub_9e63e0() -> ! {
    todo!("0x9e63e0 __ZN3RBX11ObjectValueD1Ev")
}

#[doc(alias = "boost::bad_function_call::bad_function_call(void)")]
// 0x9e6a30 — __ZN5boost17bad_function_callC1Ev
// was: boost::bad_function_call::bad_function_call(void)
pub fn stub_9e6a30() -> ! {
    todo!("0x9e6a30 __ZN5boost17bad_function_callC1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,bool,int)>::slot> &)")]
// 0x9e6b98 — __ZN3rbx7signals6signalIFvibiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(int,bool,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int,bool,int)>::slot> &)
pub fn stub_9e6b98() -> ! {
    todo!("0x9e6b98 __ZN3rbx7signals6signalIFvibiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::mutex(void)")]
// 0x9e6da0 — __ZN3rbx7signals6signalIFvibiEE5mutexEv
pub fn stub_9e6da0() -> ! {
    todo!("0x9e6da0 __ZN3rbx7signals6signalIFvibiEE5mutexEv")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,bool,int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,bool,int)>::slot> const&)")]
// 0x9e6eb4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvibiEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,bool,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,bool,int)>::slot> const&)
pub fn stub_9e6eb4() -> ! {
    todo!("0x9e6eb4 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvibiEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::safe_static_init_mutex(void)")]
// 0x9e6f68 — __ZN3rbx7signals6signalIFvibiEE22safe_static_init_mutexEv
pub fn stub_9e6f68() -> ! {
    todo!("0x9e6f68 __ZN3rbx7signals6signalIFvibiEE22safe_static_init_mutexEv")
}

#[doc(alias = "boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>::split_iterator<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>)")]
// 0x9e7050 — __ZN5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2INS0_6detail13token_finderFINS8_10is_any_ofFIcEEEEEES5_S5_T_
// was: boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>::split_iterator<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>)
pub fn stub_9e7050() -> ! {
    todo!("0x9e7050 __ZN5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2INS0_6detail13token_finderFINS8_10is_any_ofFIcEEEEEES5_S5_T_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x9e71f0 — __ZNK5boost6detail8function13basic_vtable2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES7_S7_E9assign_toINS_9algorithm6detail13token_finderFINSC_10is_any_ofFIcEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_9e71f0() -> ! {
    todo!("0x9e71f0 __ZNK5boost6detail8function13basic_vtable2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES7_S7_E9assign_toINS_9algorithm6detail13token_finderFINSC_10is_any_ofFIcEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x9e7320 — __ZN5boost6detail8function15functor_managerINS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEEE7managerERKNS1_15function_bufferERSA_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_9e7320() -> ! {
    todo!("0x9e7320 __ZN5boost6detail8function15functor_managerINS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEEE7managerERKNS1_15function_bufferERSA_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::_M_insert_unique(std::string const&)")]
// 0x9e7470 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE16_M_insert_uniqueERKSs
pub fn stub_9e7470() -> ! {
    todo!("0x9e7470 __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE16_M_insert_uniqueERKSs")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<unsigned int>(char const*,unsigned int const&)")]
// 0x9e8e08 — __ZN3RBX5Stats4Item20createBoundChildItemIjEEPS1_PKcRKT_
pub fn stub_9e8e08() -> ! {
    todo!("0x9e8e08 __ZN3RBX5Stats4Item20createBoundChildItemIjEEPS1_PKcRKT_")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<int>::deref(int const*)")]
// 0x9ea4c8 — __ZN3RBX5Stats14TypedStatsItemIiE5derefEPKi
pub fn stub_9ea4c8() -> ! {
    todo!("0x9ea4c8 __ZN3RBX5Stats14TypedStatsItemIiE5derefEPKi")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x9ea4d0 — __ZN3RBX5Stats14TypedStatsItemIjED1Ev
pub fn stub_9ea4d0() -> ! {
    todo!("0x9ea4d0 __ZN3RBX5Stats14TypedStatsItemIjED1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x9ea4e0 — __ZThn32_N3RBX5Stats14TypedStatsItemIjED1Ev
pub fn stub_9ea4e0() -> ! {
    todo!("0x9ea4e0 __ZThn32_N3RBX5Stats14TypedStatsItemIjED1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x9ea4f0 — __ZThn36_N3RBX5Stats14TypedStatsItemIjED1Ev
pub fn stub_9ea4f0() -> ! {
    todo!("0x9ea4f0 __ZThn36_N3RBX5Stats14TypedStatsItemIjED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x9ea500 — __ZN3RBX5Stats14TypedStatsItemIjED2Ev
pub fn stub_9ea500() -> ! {
    todo!("0x9ea500 __ZN3RBX5Stats14TypedStatsItemIjED2Ev")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<unsigned int const&,unsigned int const& (*)(unsigned int const*),boost::_bi::list1<boost::_bi::value<unsigned int const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x9ea6d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKjPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<unsigned int const&,unsigned int const& (*)(unsigned int const*),boost::_bi::list1<boost::_bi::value<unsigned int const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_9ea6d8() -> ! {
    todo!("0x9ea6d8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKjPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<unsigned int const&,unsigned int const& (*)(unsigned int const*),boost::_bi::list1<boost::_bi::value<unsigned int const*>>>,unsigned int>::invoke(boost::detail::function::function_buffer &)")]
// 0x9ea738 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKjPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEjE6invokeERNS1_15function_bufferE
// was: boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<unsigned int const&,unsigned int const& (*)(unsigned int const*),boost::_bi::list1<boost::_bi::value<unsigned int const*>>>,unsigned int>::invoke(boost::detail::function::function_buffer &)
pub fn stub_9ea738() -> ! {
    todo!("0x9ea738 __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKjPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEjE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<float>::~TypedStatsItem()")]
// 0x9eac40 — __ZN3RBX5Stats14TypedStatsItemIfED0Ev
pub fn stub_9eac40() -> ! {
    todo!("0x9eac40 __ZN3RBX5Stats14TypedStatsItemIfED0Ev")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<double>(char const*,double const&)")]
// 0x9eace0 — __ZN3RBX5Stats4Item20createBoundChildItemIdEEPS1_PKcRKT_
pub fn stub_9eace0() -> ! {
    todo!("0x9eace0 __ZN3RBX5Stats4Item20createBoundChildItemIdEEPS1_PKcRKT_")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
// 0x9eb338 — __ZThn36_N3RBX5Stats14TypedStatsItemIdED1Ev
pub fn stub_9eb338() -> ! {
    todo!("0x9eb338 __ZThn36_N3RBX5Stats14TypedStatsItemIdED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
// 0x9eb348 — __ZN3RBX5Stats14TypedStatsItemIdED2Ev
pub fn stub_9eb348() -> ! {
    todo!("0x9eb348 __ZN3RBX5Stats14TypedStatsItemIdED2Ev")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<double const&,double const& (*)(double const*),boost::_bi::list1<boost::_bi::value<double const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x9eb520 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKdPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<double const&,double const& (*)(double const*),boost::_bi::list1<boost::_bi::value<double const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_9eb520() -> ! {
    todo!("0x9eb520 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKdPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<double const&,double const& (*)(double const*),boost::_bi::list1<boost::_bi::value<double const*>>>,double>::invoke(boost::detail::function::function_buffer &)")]
// 0x9eb580 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKdPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEdE6invokeERNS1_15function_bufferE
// was: boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<double const&,double const& (*)(double const*),boost::_bi::list1<boost::_bi::value<double const*>>>,double>::invoke(boost::detail::function::function_buffer &)
pub fn stub_9eb580() -> ! {
    todo!("0x9eb580 __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKdPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEdE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()")]
// 0x9eb5b8 — __ZThn32_N3RBX5Stats14TypedStatsItemIbED1Ev
pub fn stub_9eb5b8() -> ! {
    todo!("0x9eb5b8 __ZThn32_N3RBX5Stats14TypedStatsItemIbED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()")]
// 0x9eb5c8 — __ZN3RBX5Stats14TypedStatsItemIbED2Ev
pub fn stub_9eb5c8() -> ! {
    todo!("0x9eb5c8 __ZN3RBX5Stats14TypedStatsItemIbED2Ev")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool const&,bool const& (*)(bool const*),boost::_bi::list1<boost::_bi::value<bool const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x9eb7a0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKbPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<bool const&,bool const& (*)(bool const*),boost::_bi::list1<boost::_bi::value<bool const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_9eb7a0() -> ! {
    todo!("0x9eb7a0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKbPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<bool const&,bool const& (*)(bool const*),boost::_bi::list1<boost::_bi::value<bool const*>>>,bool>::invoke(boost::detail::function::function_buffer &)")]
// 0x9eb800 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKbPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEbE6invokeERNS1_15function_bufferE
// was: boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<bool const&,bool const& (*)(bool const*),boost::_bi::list1<boost::_bi::value<bool const*>>>,bool>::invoke(boost::detail::function::function_buffer &)
pub fn stub_9eb800() -> ! {
    todo!("0x9eb800 __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKbPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEbE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x9ebaf0 — __ZN3RBX5Stats14TypedStatsItemIyED1Ev
pub fn stub_9ebaf0() -> ! {
    todo!("0x9ebaf0 __ZN3RBX5Stats14TypedStatsItemIyED1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x9ebb00 — __ZThn32_N3RBX5Stats14TypedStatsItemIyED1Ev
pub fn stub_9ebb00() -> ! {
    todo!("0x9ebb00 __ZThn32_N3RBX5Stats14TypedStatsItemIyED1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x9ebb10 — __ZThn36_N3RBX5Stats14TypedStatsItemIyED1Ev
pub fn stub_9ebb10() -> ! {
    todo!("0x9ebb10 __ZThn36_N3RBX5Stats14TypedStatsItemIyED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x9ebb20 — __ZN3RBX5Stats14TypedStatsItemIyED2Ev
pub fn stub_9ebb20() -> ! {
    todo!("0x9ebb20 __ZN3RBX5Stats14TypedStatsItemIyED2Ev")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<unsigned long long const&,unsigned long long const& (*)(unsigned long long const*),boost::_bi::list1<boost::_bi::value<unsigned long long const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x9ebcf8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKyPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<unsigned long long const&,unsigned long long const& (*)(unsigned long long const*),boost::_bi::list1<boost::_bi::value<unsigned long long const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_9ebcf8() -> ! {
    todo!("0x9ebcf8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKyPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<unsigned long long const&,unsigned long long const& (*)(unsigned long long const*),boost::_bi::list1<boost::_bi::value<unsigned long long const*>>>,unsigned long long>::invoke(boost::detail::function::function_buffer &)")]
// 0x9ebd58 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKyPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEyE6invokeERNS1_15function_bufferE
// was: boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<unsigned long long const&,unsigned long long const& (*)(unsigned long long const*),boost::_bi::list1<boost::_bi::value<unsigned long long const*>>>,unsigned long long>::invoke(boost::detail::function::function_buffer &)
pub fn stub_9ebd58() -> ! {
    todo!("0x9ebd58 __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKyPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEyE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::disconnectAll(void)")]
// 0x9ecf0c — __ZN3rbx7signals6signalIFvibiEE13disconnectAllEv
pub fn stub_9ecf0c() -> ! {
    todo!("0x9ecf0c __ZN3rbx7signals6signalIFvibiEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::insert(rbx::signals::signal<void ()(int,bool,int)>::slot *)")]
// 0x9eea54 — __ZN3rbx7signals6signalIFvibiEE6insertEPNS3_4slotE
pub fn stub_9eea54() -> ! {
    todo!("0x9eea54 __ZN3rbx7signals6signalIFvibiEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,bool,int)>::slot>::operator=(rbx::signals::signal<void ()(int,bool,int)>::slot*)")]
// 0x9eed0c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvibiEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,bool,int)>::slot>::operator=(rbx::signals::signal<void ()(int,bool,int)>::slot*)
pub fn stub_9eed0c() -> ! {
    todo!("0x9eed0c __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvibiEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::callable_slot<boost::function<void ()(int,bool,int)>>::~callable_slot()")]
// 0x9eedc0 — __ZN3rbx7signals6signalIFvibiEE13callable_slotIN5boost8functionIS2_EEED1Ev
// was: rbx::signals::signal<void ()(int,bool,int)>::callable_slot<boost::function<void ()(int,bool,int)>>::~callable_slot()
pub fn stub_9eedc0() -> ! {
    todo!("0x9eedc0 __ZN3rbx7signals6signalIFvibiEE13callable_slotIN5boost8functionIS2_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::callable_slot<boost::function<void ()(int,bool,int)>>::~callable_slot()")]
// 0x9eedcc — __ZN3rbx7signals6signalIFvibiEE13callable_slotIN5boost8functionIS2_EEED0Ev
// was: rbx::signals::signal<void ()(int,bool,int)>::callable_slot<boost::function<void ()(int,bool,int)>>::~callable_slot()
pub fn stub_9eedcc() -> ! {
    todo!("0x9eedcc __ZN3rbx7signals6signalIFvibiEE13callable_slotIN5boost8functionIS2_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::slot::disconnect(void)")]
// 0x9eee80 — __ZN3rbx7signals6signalIFvibiEE4slot10disconnectEv
pub fn stub_9eee80() -> ! {
    todo!("0x9eee80 __ZN3rbx7signals6signalIFvibiEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::slot::connected(void)const")]
// 0x9eeff4 — __ZNK3rbx7signals6signalIFvibiEE4slot9connectedEv
pub fn stub_9eeff4() -> ! {
    todo!("0x9eeff4 __ZNK3rbx7signals6signalIFvibiEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,bool,int)>::slot,boost::function<void ()(int,bool,int)>,3,void ()(int,bool,int)>::call(int,bool,int)")]
// 0x9ef000 — __ZN3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_E4callEibi
// was: rbx::callable<rbx::signals::signal<void ()(int,bool,int)>::slot,boost::function<void ()(int,bool,int)>,3,void ()(int,bool,int)>::call(int,bool,int)
pub fn stub_9ef000() -> ! {
    todo!("0x9ef000 __ZN3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_E4callEibi")
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(int,bool,int)>::slot,boost::function<void ()(int,bool,int)>,3,void ()(int,bool,int)>::call(int,bool,int)")]
// 0x9ef0cc — __ZThn4_N3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_E4callEibi
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(int,bool,int)>::slot,boost::function<void ()(int,bool,int)>,3,void ()(int,bool,int)>::call(int,bool,int)
pub fn stub_9ef0cc() -> ! {
    todo!("0x9ef0cc __ZThn4_N3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_E4callEibi")
}
