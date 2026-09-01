//! core shard HI — 100 core stubs EA-sorted, 0xf5e1b4..0xf5f254 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HH 0xf5e1a4).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HH 0xf5e1a4 (0xf5e1b4..0xf5f254, 21214->21314 covered, 604 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::GfxPart * const,RBX::ContentId>> *)")]
// 0xf5e1b4 — j___ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
pub fn stub_0xf5e1b4() -> ! {
    todo!("0xf5e1b4 j___ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")
}

#[doc(alias = "std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::GfxPart * const,RBX::ContentId> const&)")]
// 0xf5e1c4 — j___ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_
pub fn stub_0xf5e1c4() -> ! {
    todo!("0xf5e1c4 j___ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_")
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,int,RBX::ContentId>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,int,int,RBX::ContentId)")]
// 0xf5e1d4 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEiS3_EvT_T0_SA_T1_
pub fn stub_0xf5e1d4() -> ! {
    todo!("0xf5e1d4 j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEiS3_EvT_T0_SA_T1_")
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0xf5e1e4 — j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_S9_
pub fn stub_0xf5e1e4() -> ! {
    todo!("0xf5e1e4 j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_S9_")
}

#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0xf5e1f4 — j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_
pub fn stub_0xf5e1f4() -> ! {
    todo!("0xf5e1f4 j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_")
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,int>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,int)")]
// 0xf5e204 — j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEiEvT_S9_T0_
pub fn stub_0xf5e204() -> ! {
    todo!("0xf5e204 j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEiEvT_S9_T0_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>> std::__unguarded_partition<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,RBX::ContentId>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,RBX::ContentId)")]
// 0xf5e214 — j___ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEES3_ET_S9_S9_T0_
pub fn stub_0xf5e214() -> ! {
    todo!("0xf5e214 j___ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEES3_ET_S9_S9_T0_")
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0xf5e224 — j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_
pub fn stub_0xf5e224() -> ! {
    todo!("0xf5e224 j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_")
}

#[doc(alias = "void std::swap<boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>>>(boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &,boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &)")]
// 0xf5e244 — j___ZSt4swapIN5boost9unordered13unordered_setIPN3RBX13GfxAttachmentENS0_4hashIS5_EESt8equal_toIS5_ESaIS5_EEEEvRT_SD_
pub fn stub_0xf5e244() -> ! {
    todo!("0xf5e244 j___ZSt4swapIN5boost9unordered13unordered_setIPN3RBX13GfxAttachmentENS0_4hashIS5_EESt8equal_toIS5_ESaIS5_EEEEvRT_SD_")
}

#[doc(alias = "void std::swap<boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>>(boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &,boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &)")]
// 0xf5e254 — j___ZSt4swapIN5boost9unordered13unordered_setIPN3RBX7GfxPartENS0_4hashIS5_EESt8equal_toIS5_ENS0_19fast_pool_allocatorIS5_NS0_33default_user_allocator_new_deleteENS0_5mutexELj32ELj0EEEEEEvRT_SG_
pub fn stub_0xf5e254() -> ! {
    todo!("0xf5e254 j___ZSt4swapIN5boost9unordered13unordered_setIPN3RBX7GfxPartENS0_4hashIS5_EESt8equal_toIS5_ENS0_19fast_pool_allocatorIS5_NS0_33default_user_allocator_new_deleteENS0_5mutexELj32ELj0EEEEEEvRT_SG_")
}

#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0xf5e264 — j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_
pub fn stub_0xf5e264() -> ! {
    todo!("0xf5e264 j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_")
}

#[doc(alias = "void std::make_heap<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0xf5e274 — j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_
pub fn stub_0xf5e274() -> ! {
    todo!("0xf5e274 j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,PacketPriority>,std::_Select1st<std::pair<RBX::Name const* const,PacketPriority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,PacketPriority>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,PacketPriority>> *)")]
// 0xf5e4e4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_14PacketPriorityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_0xf5e4e4() -> ! {
    todo!("0xf5e4e4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_14PacketPriorityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,PacketReliability>,std::_Select1st<std::pair<RBX::Name const* const,PacketReliability>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,PacketReliability>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,PacketReliability>> *)")]
// 0xf5e4f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_17PacketReliabilityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_0xf5e4f4() -> ! {
    todo!("0xf5e4f4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_17PacketReliabilityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,int,std::string)>::fireItem(rbx::signals::signal<void ()(std::string,int,std::string)>::slot *,std::string,int,std::string)")]
// 0xf5e8a4 — j___ZN3rbx7signals16signal_with_argsILi3EFvSsiSsEE8fireItemEPNS0_6signalIS2_E4slotESsiSs
pub fn stub_0xf5e8a4() -> ! {
    todo!("0xf5e8a4 j___ZN3rbx7signals16signal_with_argsILi3EFvSsiSsEE8fireItemEPNS0_6signalIS2_E4slotESsiSs")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,int,std::string)>::operator()(std::string,int,std::string)")]
// 0xf5e8b4 — j___ZN3rbx7signals16signal_with_argsILi3EFvSsiSsEEclESsiSs
pub fn stub_0xf5e8b4() -> ! {
    todo!("0xf5e8b4 j___ZN3rbx7signals16signal_with_argsILi3EFvSsiSsEEclESsiSs")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::disconnectAll(void)")]
// 0xf5e904 — j___ZN3rbx7signals6signalIFvSsiSsEE13disconnectAllEv
pub fn stub_0xf5e904() -> ! {
    todo!("0xf5e904 j___ZN3rbx7signals6signalIFvSsiSsEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::insert(rbx::signals::signal<void ()(std::string,int,std::string)>::slot *)")]
// 0xf5e914 — j___ZN3rbx7signals6signalIFvSsiSsEE6insertEPNS3_4slotE
pub fn stub_0xf5e914() -> ! {
    todo!("0xf5e914 j___ZN3rbx7signals6signalIFvSsiSsEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::remove(rbx::signals::signal<void ()(std::string,int,std::string)>::slot *)")]
// 0xf5e924 — j___ZN3rbx7signals6signalIFvSsiSsEE6removeEPNS3_4slotE
pub fn stub_0xf5e924() -> ! {
    todo!("0xf5e924 j___ZN3rbx7signals6signalIFvSsiSsEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,std::string)>::slot,boost::function<void ()(std::string,int,std::string)>,3,void ()(std::string,int,std::string)>::call(std::string,int,std::string)")]
// 0xf5e954 — j___ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsiSs
pub fn stub_0xf5e954() -> ! {
    todo!("0xf5e954 j___ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsiSs")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,std::string)>::slot,boost::function<void ()(std::string,int,std::string)>,3,void ()(std::string,int,std::string)>::~callable()")]
// 0xf5e964 — j___ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_ED2Ev
pub fn stub_0xf5e964() -> ! {
    todo!("0xf5e964 j___ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_ED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,int,std::string)>::slot*)")]
// 0xf5e974 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiSsEE4slotEEaSEPS6_
pub fn stub_0xf5e974() -> ! {
    todo!("0xf5e974 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiSsEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,std::string)>::slot> const&)")]
// 0xf5e984 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiSsEE4slotEEaSERKS7_
pub fn stub_0xf5e984() -> ! {
    todo!("0xf5e984 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiSsEE4slotEEaSERKS7_")
}

#[doc(alias = "void boost::throw_exception<boost::gregorian::bad_day_of_month>(boost::gregorian::bad_day_of_month const&)")]
// 0xf5e994 — j___ZN5boost15throw_exceptionINS_9gregorian16bad_day_of_monthEEEvRKT_
pub fn stub_0xf5e994() -> ! {
    todo!("0xf5e994 j___ZN5boost15throw_exceptionINS_9gregorian16bad_day_of_monthEEEvRKT_")
}

#[doc(alias = "void boost::throw_exception<boost::gregorian::bad_year>(boost::gregorian::bad_year const&)")]
// 0xf5e9a4 — j___ZN5boost15throw_exceptionINS_9gregorian8bad_yearEEEvRKT_
pub fn stub_0xf5e9a4() -> ! {
    todo!("0xf5e9a4 j___ZN5boost15throw_exceptionINS_9gregorian8bad_yearEEEvRKT_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_month> const&)")]
// 0xf5e9b4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS5_
pub fn stub_0xf5e9b4() -> ! {
    todo!("0xf5e9b4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS5_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_impl(boost::exception_detail::error_info_injector<std::runtime_error> const&)")]
// 0xf5e9c4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEEC1ERKS4_
pub fn stub_0xf5e9c4() -> ! {
    todo!("0xf5e9c4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEEC1ERKS4_")
}

#[doc(alias = "boost::date_time::microsec_clock<boost::posix_time::ptime>::create_time(tm * (*)(long const*,tm *))")]
// 0xf5eac4 — j___ZN5boost9date_time14microsec_clockINS_10posix_time5ptimeEE11create_timeEPFP2tmPKlS6_E
pub fn stub_0xf5eac4() -> ! {
    todo!("0xf5eac4 j___ZN5boost9date_time14microsec_clockINS_10posix_time5ptimeEE11create_timeEPFP2tmPKlS6_E")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::rethrow(void)const")]
// 0xf5eb34 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE7rethrowEv
pub fn stub_0xf5eb34() -> ! {
    todo!("0xf5eb34 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE7rethrowEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::rethrow(void)const")]
// 0xf5eb44 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE7rethrowEv
pub fn stub_0xf5eb44() -> ! {
    todo!("0xf5eb44 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE7rethrowEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::rethrow(void)const")]
// 0xf5eb54 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE7rethrowEv
pub fn stub_0xf5eb54() -> ! {
    todo!("0xf5eb54 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE7rethrowEv")
}

#[doc(alias = "boost::function3<void,std::string,int,std::string>::operator()(std::string,int,std::string)const")]
// 0xf5ebc4 — j___ZNK5boost9function3IvSsiSsEclESsiSs
pub fn stub_0xf5ebc4() -> ! {
    todo!("0xf5ebc4 j___ZNK5boost9function3IvSsiSsEclESsiSs")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<float>::~TypedStatsItem()")]
// 0xf5ec04 — j___ZN3RBX5Stats14TypedStatsItemIfED2Ev
pub fn stub_0xf5ec04() -> ! {
    todo!("0xf5ec04 j___ZN3RBX5Stats14TypedStatsItemIfED2Ev")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<float>(char const*,float const&)")]
// 0xf5ec14 — j___ZN3RBX5Stats4Item20createBoundChildItemIfEEPS1_PKcRKT_
pub fn stub_0xf5ec14() -> ! {
    todo!("0xf5ec14 j___ZN3RBX5Stats4Item20createBoundChildItemIfEEPS1_PKcRKT_")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<unsigned long long>(char const*,unsigned long long const&)")]
// 0xf5ec24 — j___ZN3RBX5Stats4Item20createBoundChildItemIyEEPS1_PKcRKT_
pub fn stub_0xf5ec24() -> ! {
    todo!("0xf5ec24 j___ZN3RBX5Stats4Item20createBoundChildItemIyEEPS1_PKcRKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,std::string)>::slot> &)")]
// 0xf5ecd4 — j___ZN3rbx7signals6signalIFvSsiSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_0xf5ecd4() -> ! {
    todo!("0xf5ecd4 j___ZN3rbx7signals6signalIFvSsiSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::mutex(void)")]
// 0xf5ece4 — j___ZN3rbx7signals6signalIFvSsiSsEE5mutexEv
pub fn stub_0xf5ece4() -> ! {
    todo!("0xf5ece4 j___ZN3rbx7signals6signalIFvSsiSsEE5mutexEv")
}

#[doc(alias = "boost::unique_lock<boost::mutex>::~unique_lock()")]
// 0xf5ee74 — j___ZN5boost11unique_lockINS_5mutexEED1Ev
pub fn stub_0xf5ee74() -> ! {
    todo!("0xf5ee74 j___ZN5boost11unique_lockINS_5mutexEED1Ev")
}

#[doc(alias = "void boost::throw_exception<boost::iostreams::gzip_error>(boost::iostreams::gzip_error const&)")]
// 0xf5ee84 — j___ZN5boost15throw_exceptionINS_9iostreams10gzip_errorEEEvRKT_
pub fn stub_0xf5ee84() -> ! {
    todo!("0xf5ee84 j___ZN5boost15throw_exceptionINS_9iostreams10gzip_errorEEEvRKT_")
}

#[doc(alias = "void boost::throw_exception<std::logic_error>(std::logic_error const&)")]
// 0xf5ee94 — j___ZN5boost15throw_exceptionISt11logic_errorEEvRKT_
pub fn stub_0xf5ee94() -> ! {
    todo!("0xf5ee94 j___ZN5boost15throw_exceptionISt11logic_errorEEvRKT_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::iostreams::gzip_error> const&)")]
// 0xf5eea4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEEC1ERKS5_
pub fn stub_0xf5eea4() -> ! {
    todo!("0xf5eea4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEEC1ERKS5_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>> const&)")]
// 0xf5eeb4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEEC1ERKS6_
pub fn stub_0xf5eeb4() -> ! {
    todo!("0xf5eeb4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEEC1ERKS6_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone_tag)")]
// 0xf5eec4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEEC1ERKS6_NS6_9clone_tagE
pub fn stub_0xf5eec4() -> ! {
    todo!("0xf5eec4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_impl(boost::exception_detail::error_info_injector<std::ios_base::failure> const&)")]
// 0xf5eed4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEEC1ERKS5_
pub fn stub_0xf5eed4() -> ! {
    todo!("0xf5eed4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEEC1ERKS5_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone_tag)")]
// 0xf5eee4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEEC1ERKS5_NS5_9clone_tagE
pub fn stub_0xf5eee4() -> ! {
    todo!("0xf5eee4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl,boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>(rbx_core::SharedPtr<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl> *,boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl *,boost::detail::shared_count &)")]
// 0xf5eef4 — j___ZN5boost6detail20sp_pointer_constructINS_9iostreams16symmetric_filterINS2_6detail22zlib_decompressor_implISaIcEEES6_E4implES9_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
pub fn stub_0xf5eef4() -> ! {
    todo!("0xf5eef4 j___ZN5boost6detail20sp_pointer_constructINS_9iostreams16symmetric_filterINS2_6detail22zlib_decompressor_implISaIcEEES6_E4implES9_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>(rbx_core::SharedPtr<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl> *,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl *,boost::detail::shared_count &)")]
// 0xf5ef04 — j___ZN5boost6detail20sp_pointer_constructINS_9iostreams6detail10chain_baseINS2_5chainINS2_5inputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implESC_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
pub fn stub_0xf5ef04() -> ! {
    todo!("0xf5ef04 j___ZN5boost6detail20sp_pointer_constructINS_9iostreams6detail10chain_baseINS2_5chainINS2_5inputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implESC_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open_impl(boost::iostreams::basic_null_device<char,boost::iostreams::input> const&,int,int)")]
// 0xf5ef14 — j___ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES3_E9open_implERKS4_ii
pub fn stub_0xf5ef14() -> ! {
    todo!("0xf5ef14 j___ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES3_E9open_implERKS4_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0xf5ef24 — j___ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES3_ED2Ev
pub fn stub_0xf5ef24() -> ! {
    todo!("0xf5ef24 j___ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES3_ED2Ev")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input_seekable>::open_impl(boost::iostreams::basic_array_source<char> const&,int,int)")]
// 0xf5ef34 — j___ZN5boost9iostreams13stream_bufferINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcENS0_14input_seekableEE9open_implERKS3_ii
pub fn stub_0xf5ef34() -> ! {
    todo!("0xf5ef34 j___ZN5boost9iostreams13stream_bufferINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcENS0_14input_seekableEE9open_implERKS3_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input_seekable>::~stream_buffer()")]
// 0xf5ef44 — j___ZN5boost9iostreams13stream_bufferINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcENS0_14input_seekableEED2Ev
pub fn stub_0xf5ef44() -> ! {
    todo!("0xf5ef44 j___ZN5boost9iostreams13stream_bufferINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcENS0_14input_seekableEED2Ev")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open_impl(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0xf5ef54 — j___ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_5inputEE9open_implERKS4_ii
pub fn stub_0xf5ef54() -> ! {
    todo!("0xf5ef54 j___ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_5inputEE9open_implERKS4_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::stream_buffer(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0xf5ef64 — j___ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_5inputEEC2ERKS4_ii
pub fn stub_0xf5ef64() -> ! {
    todo!("0xf5ef64 j___ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_5inputEEC2ERKS4_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0xf5ef74 — j___ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_5inputEED2Ev
pub fn stub_0xf5ef74() -> ! {
    todo!("0xf5ef74 j___ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_5inputEED2Ev")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open_impl(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> const&,int,int)")]
// 0xf5ef84 — j___ZN5boost9iostreams13stream_bufferINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES3_NS0_5inputEE9open_implERKS4_ii
pub fn stub_0xf5ef84() -> ! {
    todo!("0xf5ef84 j___ZN5boost9iostreams13stream_bufferINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES3_NS0_5inputEE9open_implERKS4_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0xf5ef94 — j___ZN5boost9iostreams13stream_bufferINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES3_NS0_5inputEED2Ev
pub fn stub_0xf5ef94() -> ! {
    todo!("0xf5ef94 j___ZN5boost9iostreams13stream_bufferINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES3_NS0_5inputEED2Ev")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open_impl(boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream> const&,int,int)")]
// 0xf5efa4 — j___ZN5boost9iostreams13stream_bufferINS0_6detail12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E9open_implERKS5_ii
pub fn stub_0xf5efa4() -> ! {
    todo!("0xf5efa4 j___ZN5boost9iostreams13stream_bufferINS0_6detail12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E9open_implERKS5_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0xf5efb4 — j___ZN5boost9iostreams13stream_bufferINS0_6detail12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_ED2Ev
pub fn stub_0xf5efb4() -> ! {
    todo!("0xf5efb4 j___ZN5boost9iostreams13stream_bufferINS0_6detail12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_ED2Ev")
}

#[doc(alias = "int boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char *,int)")]
// 0xf5efc4 — j___ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E4readINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_Pci
pub fn stub_0xf5efc4() -> ! {
    todo!("0xf5efc4 j___ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E4readINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_Pci")
}

#[doc(alias = "int boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char const*,int)")]
// 0xf5efd4 — j___ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5writeINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci
pub fn stub_0xf5efd4() -> ! {
    todo!("0xf5efd4 j___ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5writeINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci")
}

#[doc(alias = "int boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::read<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::peekable_source<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::peekable_source<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,char *,int)")]
// 0xf5efe4 — j___ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E4readINS0_23basic_gzip_decompressorIS4_E15peekable_sourceINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEiRT_Pci
pub fn stub_0xf5efe4() -> ! {
    todo!("0xf5efe4 j___ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E4readINS0_23basic_gzip_decompressorIS4_E15peekable_sourceINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEiRT_Pci")
}

#[doc(alias = "void boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::close<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,std::_Ios_Openmode)")]
// 0xf5eff4 — j___ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E5closeINS0_20non_blocking_adapterINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode
pub fn stub_0xf5eff4() -> ! {
    todo!("0xf5eff4 j___ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E5closeINS0_20non_blocking_adapterINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode")
}

#[doc(alias = "void boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::close<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,std::_Ios_Openmode)")]
// 0xf5f004 — j___ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E5closeINS2_16linked_streambufIcSt11char_traitsIcEEEEEvRT_St13_Ios_Openmode
pub fn stub_0xf5f004() -> ! {
    todo!("0xf5f004 j___ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E5closeINS2_16linked_streambufIcSt11char_traitsIcEEEEEvRT_St13_Ios_Openmode")
}

#[doc(alias = "int boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char const*,int)")]
// 0xf5f014 — j___ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E5writeINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci
pub fn stub_0xf5f014() -> ! {
    todo!("0xf5f014 j___ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E5writeINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci")
}

#[doc(alias = "boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_streambuf()")]
// 0xf5f024 — j___ZN5boost9iostreams19filtering_streambufINS0_5inputEcSt11char_traitsIcESaIcENS0_7public_EED2Ev
pub fn stub_0xf5f024() -> ! {
    todo!("0xf5f024 j___ZN5boost9iostreams19filtering_streambufINS0_5inputEcSt11char_traitsIcESaIcENS0_7public_EED2Ev")
}

#[doc(alias = "void boost::iostreams::basic_gzip_compressor<std::allocator<char>>::write_long<boost::iostreams::back_insert_device<std::string>>(long,boost::iostreams::back_insert_device<std::string> &,mpl_::bool_<true>)")]
// 0xf5f034 — j___ZN5boost9iostreams21basic_gzip_compressorISaIcEE10write_longINS0_18back_insert_deviceISsEEEEvlRT_N4mpl_5bool_ILb1EEE
pub fn stub_0xf5f034() -> ! {
    todo!("0xf5f034 j___ZN5boost9iostreams21basic_gzip_compressorISaIcEE10write_longINS0_18back_insert_deviceISsEEEEvlRT_N4mpl_5bool_ILb1EEE")
}

#[doc(alias = "int boost::iostreams::basic_gzip_compressor<std::allocator<char>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char *,int)")]
// 0xf5f044 — j___ZN5boost9iostreams21basic_gzip_compressorISaIcEE4readINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_Pci
pub fn stub_0xf5f044() -> ! {
    todo!("0xf5f044 j___ZN5boost9iostreams21basic_gzip_compressorISaIcEE4readINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_Pci")
}

#[doc(alias = "boost::iostreams::basic_gzip_compressor<std::allocator<char>>::basic_gzip_compressor(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&)")]
// 0xf5f054 — j___ZN5boost9iostreams21basic_gzip_compressorISaIcEEC2ERKS3_
pub fn stub_0xf5f054() -> ! {
    todo!("0xf5f054 j___ZN5boost9iostreams21basic_gzip_compressorISaIcEEC2ERKS3_")
}

#[doc(alias = "boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::make_params(int)")]
// 0xf5f064 — j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEE11make_paramsEi
pub fn stub_0xf5f064() -> ! {
    todo!("0xf5f064 j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEE11make_paramsEi")
}

#[doc(alias = "boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::peekable_source<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>::putback(char)")]
// 0xf5f074 — j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEE15peekable_sourceINS0_6detail16linked_streambufIcSt11char_traitsIcEEEE7putbackEc
pub fn stub_0xf5f074() -> ! {
    todo!("0xf5f074 j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEE15peekable_sourceINS0_6detail16linked_streambufIcSt11char_traitsIcEEEE7putbackEc")
}

#[doc(alias = "int boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char *,int)")]
// 0xf5f084 — j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEE4readINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_Pci
pub fn stub_0xf5f084() -> ! {
    todo!("0xf5f084 j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEE4readINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_Pci")
}

#[doc(alias = "void boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::close<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,std::_Ios_Openmode)")]
// 0xf5f094 — j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEE5closeINS0_20non_blocking_adapterINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode
pub fn stub_0xf5f094() -> ! {
    todo!("0xf5f094 j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEE5closeINS0_20non_blocking_adapterINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode")
}

#[doc(alias = "int boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char const*,int)")]
// 0xf5f0a4 — j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEE5writeINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci
pub fn stub_0xf5f0a4() -> ! {
    todo!("0xf5f0a4 j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEE5writeINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci")
}

#[doc(alias = "boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::basic_gzip_decompressor(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> const&)")]
// 0xf5f0b4 — j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEEC2ERKS3_
pub fn stub_0xf5f0b4() -> ! {
    todo!("0xf5f0b4 j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEEC2ERKS3_")
}

#[doc(alias = "boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::basic_gzip_decompressor(int,int)")]
// 0xf5f0c4 — j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEEC2Eii
pub fn stub_0xf5f0c4() -> ! {
    todo!("0xf5f0c4 j___ZN5boost9iostreams23basic_gzip_decompressorISaIcEEC2Eii")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl::close(void)")]
// 0xf5f0d4 — j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_impl5closeEv
pub fn stub_0xf5f0d4() -> ! {
    todo!("0xf5f0d4 j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_impl5closeEv")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl::~chain_impl()")]
// 0xf5f0e4 — j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_implD2Ev
pub fn stub_0xf5f0e4() -> ! {
    todo!("0xf5f0e4 j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_implD2Ev")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::pop(void)")]
// 0xf5f0f4 — j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E3popEv
pub fn stub_0xf5f0f4() -> ! {
    todo!("0xf5f0f4 j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E3popEv")
}

#[doc(alias = "void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::push_impl<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0xf5f104 — j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_21basic_gzip_compressorIS7_EEEEvRKT_ii
pub fn stub_0xf5f104() -> ! {
    todo!("0xf5f104 j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_21basic_gzip_compressorIS7_EEEEvRKT_ii")
}

#[doc(alias = "void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::push_impl<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>>(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> const&,int,int)")]
// 0xf5f114 — j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_23basic_gzip_decompressorIS7_EEEEvRKT_ii
pub fn stub_0xf5f114() -> ! {
    todo!("0xf5f114 j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_23basic_gzip_decompressorIS7_EEEEvRKT_ii")
}

#[doc(alias = "void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::push_impl<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>(boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream> const&,int,int)")]
// 0xf5f124 — j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS1_12mode_adapterIS4_SiEEEEvRKT_ii
pub fn stub_0xf5f124() -> ! {
    todo!("0xf5f124 j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS1_12mode_adapterIS4_SiEEEEvRKT_ii")
}

#[doc(alias = "void boost::iostreams::detail::close_impl<boost::iostreams::detail::two_sequence>::close<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,std::_Ios_Openmode)")]
// 0xf5f134 — j___ZN5boost9iostreams6detail10close_implINS1_12two_sequenceEE5closeINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_St13_Ios_Openmode
pub fn stub_0xf5f134() -> ! {
    todo!("0xf5f134 j___ZN5boost9iostreams6detail10close_implINS1_12two_sequenceEE5closeINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_St13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>,String_sink>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>,String_sink> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>,String_sink>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>,String_sink>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>)")]
// 0xf5f144 — j___ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperINS0_19filtering_streambufINS0_5inputEcSt11char_traitsIcESaIcENS0_7public_EEEEE11String_sinkEENS1_26device_close_all_operationISC_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_
pub fn stub_0xf5f144() -> ! {
    todo!("0xf5f144 j___ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperINS0_19filtering_streambufINS0_5inputEcSt11char_traitsIcESaIcENS0_7public_EEEEE11String_sinkEENS1_26device_close_all_operationISC_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>)")]
// 0xf5f154 — j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_EENS1_14execute_traitsIT_NS_9result_ofIFSA_vEE4typeEE11result_typeESA_T0_
pub fn stub_0xf5f154() -> ! {
    todo!("0xf5f154 j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_EENS1_14execute_traitsIT_NS_9result_ofIFSA_vEE4typeEE11result_typeESA_T0_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::input>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::input>>>>,boost::iostreams::detail::clear_flags_operation<int>)")]
// 0xf5f164 — j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_5inputEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
pub fn stub_0xf5f164() -> ! {
    todo!("0xf5f164 j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_5inputEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>>>>)")]
// 0xf5f174 — j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_23basic_gzip_decompressorISaIcEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
pub fn stub_0xf5f174() -> ! {
    todo!("0xf5f174 j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_23basic_gzip_decompressorISaIcEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>>>,boost::iostreams::detail::clear_flags_operation<int>)")]
// 0xf5f184 — j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS1_12mode_adapterINS0_5inputESiEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
pub fn stub_0xf5f184() -> ! {
    todo!("0xf5f184 j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS1_12mode_adapterINS0_5inputESiEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::input>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0xf5f194 — j___ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_5inputEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
pub fn stub_0xf5f194() -> ! {
    todo!("0xf5f194 j___ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_5inputEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_")
}

#[doc(alias = "int boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::input>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char const*,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0xf5f1a4 — j___ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_5inputEEEE5writeINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPKciPT_
pub fn stub_0xf5f1a4() -> ! {
    todo!("0xf5f1a4 j___ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_5inputEEEE5writeINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPKciPT_")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0xf5f1b4 — j___ZN5boost9iostreams6detail15concept_adapterINS0_21basic_gzip_compressorISaIcEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
pub fn stub_0xf5f1b4() -> ! {
    todo!("0xf5f1b4 j___ZN5boost9iostreams6detail15concept_adapterINS0_21basic_gzip_compressorISaIcEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0xf5f1c4 — j___ZN5boost9iostreams6detail15concept_adapterINS0_23basic_gzip_decompressorISaIcEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
pub fn stub_0xf5f1c4() -> ! {
    todo!("0xf5f1c4 j___ZN5boost9iostreams6detail15concept_adapterINS0_23basic_gzip_decompressorISaIcEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0xf5f1d4 — j___ZN5boost9iostreams6detail15concept_adapterINS1_12mode_adapterINS0_5inputESiEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
pub fn stub_0xf5f1d4() -> ! {
    todo!("0xf5f1d4 j___ZN5boost9iostreams6detail15concept_adapterINS1_12mode_adapterINS0_5inputESiEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_")
}

#[doc(alias = "int boost::iostreams::detail::concept_adapter<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char const*,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0xf5f1e4 — j___ZN5boost9iostreams6detail15concept_adapterINS1_12mode_adapterINS0_5inputESiEEE5writeINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPKciPT_
pub fn stub_0xf5f1e4() -> ! {
    todo!("0xf5f1e4 j___ZN5boost9iostreams6detail15concept_adapterINS1_12mode_adapterINS0_5inputESiEEE5writeINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPKciPT_")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer boost::iostreams::detail::execute_foreach<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer>(std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer)")]
// 0xf5f1f4 — j___ZN5boost9iostreams6detail15execute_foreachISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEENS1_10chain_baseINS0_5chainINS0_5inputEcS6_SaIcEEEcS6_SD_SC_E6closerEEET0_T_SI_SH_
pub fn stub_0xf5f1f4() -> ! {
    todo!("0xf5f1f4 j___ZN5boost9iostreams6detail15execute_foreachISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEENS1_10chain_baseINS0_5chainINS0_5inputEcS6_SaIcEEEcS6_SD_SC_E6closerEEET0_T_SI_SH_")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer boost::iostreams::detail::execute_foreach<std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer>(std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer)")]
// 0xf5f204 — j___ZN5boost9iostreams6detail15execute_foreachISt16reverse_iteratorISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEEENS1_10chain_baseINS0_5chainINS0_5inputEcS7_SaIcEEEcS7_SF_SE_E6closerEEET0_T_SK_SJ_
pub fn stub_0xf5f204() -> ! {
    todo!("0xf5f204 j___ZN5boost9iostreams6detail15execute_foreachISt16reverse_iteratorISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEEENS1_10chain_baseINS0_5chainINS0_5inputEcS7_SaIcEEEcS7_SF_SE_E6closerEEET0_T_SK_SJ_")
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0xf5f214 — j___ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
pub fn stub_0xf5f214() -> ! {
    todo!("0xf5f214 j___ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::flt_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")]
// 0xf5f224 — j___ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
pub fn stub_0xf5f224() -> ! {
    todo!("0xf5f224 j___ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::flt_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0xf5f234 — j___ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_23basic_gzip_decompressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
pub fn stub_0xf5f234() -> ! {
    todo!("0xf5f234 j___ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_23basic_gzip_decompressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::flt_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")]
// 0xf5f244 — j___ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_23basic_gzip_decompressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
pub fn stub_0xf5f244() -> ! {
    todo!("0xf5f244 j___ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_23basic_gzip_decompressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0xf5f254 — j___ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
pub fn stub_0xf5f254() -> ! {
    todo!("0xf5f254 j___ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode")
}

