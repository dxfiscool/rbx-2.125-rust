//! core shard HL — 100 core stubs EA-sorted, 0xf62e14..0xf64b54 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HK 0xf62dc4 (21514->21614 covered, 304 remaining).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HK 0xf62dc4 (0xf62e14..0xf64b54, 21514->21614 covered, 304 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::vector<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>> *,std::allocator<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>> **,std::vector<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>> *,std::allocator<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>> *>>>,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>> * const&)")]
// 0xf62e14 — j___ZNSt6vectorIPN5boost13property_tree11basic_ptreeISsSsSt4lessISsEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_
pub fn stub_0xf62e14() -> ! {
    todo!("0xf62e14 j___ZNSt6vectorIPN5boost13property_tree11basic_ptreeISsSsSt4lessISsEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_")
}

#[doc(alias = "std::vector<char,std::allocator<char>>::_M_insert_aux(__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,char const&)")]
// 0xf62e24 — j___ZNSt6vectorIcSaIcEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPcS1_EERKc
pub fn stub_0xf62e24() -> ! {
    todo!("0xf62e24 j___ZNSt6vectorIcSaIcEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPcS1_EERKc")
}

#[doc(alias = "std::vector<unsigned long,std::allocator<unsigned long>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned long *,std::vector<unsigned long,std::allocator<unsigned long>>>,unsigned long const&)")]
// 0xf62e34 — j___ZNSt6vectorImSaImEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPmS1_EERKm
pub fn stub_0xf62e34() -> ! {
    todo!("0xf62e34 j___ZNSt6vectorImSaImEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPmS1_EERKm")
}

#[doc(alias = "void std::__heap_select<boost::multi_index::detail::copy_map_entry<boost::multi_index::detail::sequenced_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>>>> *>(boost::multi_index::detail::copy_map_entry<boost::multi_index::detail::sequenced_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>>>> *,boost::multi_index::detail::copy_map_entry<boost::multi_index::detail::sequenced_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>>>> *,boost::multi_index::detail::copy_map_entry<boost::multi_index::detail::sequenced_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>>>> *)")]
// 0xf62e84 — j___ZSt13__heap_selectIPN5boost11multi_index6detail14copy_map_entryINS2_20sequenced_index_nodeINS2_18ordered_index_nodeINS2_15index_node_baseISt4pairIKSsNS0_13property_tree11basic_ptreeISsSsSt4lessISsEEEESaISE_EEEEEEEEEEvT_SL_SL_
pub fn stub_0xf62e84() -> ! {
    todo!("0xf62e84 j___ZSt13__heap_selectIPN5boost11multi_index6detail14copy_map_entryINS2_20sequenced_index_nodeINS2_18ordered_index_nodeINS2_15index_node_baseISt4pairIKSsNS0_13property_tree11basic_ptreeISsSsSt4lessISsEEEESaISE_EEEEEEEEEEvT_SL_SL_")
}

#[doc(alias = "void std::__introsort_loop<boost::multi_index::detail::copy_map_entry<boost::multi_index::detail::sequenced_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>>>> *,int>(boost::multi_index::detail::copy_map_entry<boost::multi_index::detail::sequenced_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>>>> *,boost::multi_index::detail::copy_map_entry<boost::multi_index::detail::sequenced_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>>>> *,int)")]
// 0xf62e94 — j___ZSt16__introsort_loopIPN5boost11multi_index6detail14copy_map_entryINS2_20sequenced_index_nodeINS2_18ordered_index_nodeINS2_15index_node_baseISt4pairIKSsNS0_13property_tree11basic_ptreeISsSsSt4lessISsEEEESaISE_EEEEEEEEEiEvT_SL_T0_
pub fn stub_0xf62e94() -> ! {
    todo!("0xf62e94 j___ZSt16__introsort_loopIPN5boost11multi_index6detail14copy_map_entryINS2_20sequenced_index_nodeINS2_18ordered_index_nodeINS2_15index_node_baseISt4pairIKSsNS0_13property_tree11basic_ptreeISsSsSt4lessISsEEEESaISE_EEEEEEEEEiEvT_SL_T0_")
}

#[doc(alias = "void std::__final_insertion_sort<boost::multi_index::detail::copy_map_entry<boost::multi_index::detail::sequenced_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>>>> *>(boost::multi_index::detail::copy_map_entry<boost::multi_index::detail::sequenced_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>>>> *,boost::multi_index::detail::copy_map_entry<boost::multi_index::detail::sequenced_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>>>> *)")]
// 0xf62ea4 — j___ZSt22__final_insertion_sortIPN5boost11multi_index6detail14copy_map_entryINS2_20sequenced_index_nodeINS2_18ordered_index_nodeINS2_15index_node_baseISt4pairIKSsNS0_13property_tree11basic_ptreeISsSsSt4lessISsEEEESaISE_EEEEEEEEEEvT_SL_
pub fn stub_0xf62ea4() -> ! {
    todo!("0xf62ea4 j___ZSt22__final_insertion_sortIPN5boost11multi_index6detail14copy_map_entryINS2_20sequenced_index_nodeINS2_18ordered_index_nodeINS2_15index_node_baseISt4pairIKSsNS0_13property_tree11basic_ptreeISsSsSt4lessISsEEEESaISE_EEEEEEEEEEvT_SL_")
}

#[doc(alias = "void (*)(boost::function<void ()(bool)>) std::for_each<std::_List_iterator<boost::function<void ()(bool)>>,void (*)(boost::function<void ()(bool)>)>(std::_List_iterator<boost::function<void ()(bool)>>,std::_List_iterator<boost::function<void ()(bool)>>,void (*)(boost::function<void ()(bool)>))")]
// 0xf62eb4 — j___ZSt8for_eachISt14_List_iteratorIN5boost8functionIFvbEEEEPFvS4_EET0_T_S9_S8_
pub fn stub_0xf62eb4() -> ! {
    todo!("0xf62eb4 j___ZSt8for_eachISt14_List_iteratorIN5boost8functionIFvbEEEEPFvS4_EET0_T_S9_S8_")
}

#[doc(alias = "RBX::RunningAverageDutyCycle<(RBX::Time::SampleMethod)1>::sample(RBX::Time::Interval)")]
// 0xf62ef4 — j___ZN3RBX23RunningAverageDutyCycleILNS_4Time12SampleMethodE1EE6sampleENS1_8IntervalE
pub fn stub_0xf62ef4() -> ! {
    todo!("0xf62ef4 j___ZN3RBX23RunningAverageDutyCycleILNS_4Time12SampleMethodE1EE6sampleENS1_8IntervalE")
}

#[doc(alias = "boost::multi_index::detail::ordered_index_node_impl<std::allocator<char>>::rebalance_for_erase(boost::multi_index::detail::ordered_index_node_impl<std::allocator<char>>*,boost::multi_index::detail::ordered_index_node_compressed_base<std::allocator<char>>::parent_ref,boost::multi_index::detail::ordered_index_node_impl<std::allocator<char>>*&,boost::multi_index::detail::ordered_index_node_impl<std::allocator<char>>*&)")]
// 0xf63064 — j___ZN5boost11multi_index6detail23ordered_index_node_implISaIcEE19rebalance_for_eraseEPS4_NS1_34ordered_index_node_compressed_baseIS3_E10parent_refERS5_S9_
pub fn stub_0xf63064() -> ! {
    todo!("0xf63064 j___ZN5boost11multi_index6detail23ordered_index_node_implISaIcEE19rebalance_for_eraseEPS4_NS1_34ordered_index_node_compressed_baseIS3_E10parent_refERS5_S9_")
}

#[doc(alias = "boost::multi_index::detail::ordered_index_node_impl<std::allocator<char>>::rebalance(boost::multi_index::detail::ordered_index_node_impl<std::allocator<char>>*,boost::multi_index::detail::ordered_index_node_compressed_base<std::allocator<char>>::parent_ref)")]
// 0xf63074 — j___ZN5boost11multi_index6detail23ordered_index_node_implISaIcEE9rebalanceEPS4_NS1_34ordered_index_node_compressed_baseIS3_E10parent_refE
pub fn stub_0xf63074() -> ! {
    todo!("0xf63074 j___ZN5boost11multi_index6detail23ordered_index_node_implISaIcEE9rebalanceEPS4_NS1_34ordered_index_node_compressed_baseIS3_E10parent_refE")
}

#[doc(alias = "RBX::ObjectValue::ObjectValue(void)")]
// 0xf63214 — j___ZN3RBX11ObjectValueC2Ev
pub fn stub_0xf63214() -> ! {
    todo!("0xf63214 j___ZN3RBX11ObjectValueC2Ev")
}

#[doc(alias = "RBX::ObjectValue::~ObjectValue()")]
// 0xf63224 — j___ZN3RBX11ObjectValueD2Ev
pub fn stub_0xf63224() -> ! {
    todo!("0xf63224 j___ZN3RBX11ObjectValueD2Ev")
}

#[doc(alias = "RBX::StringValue::StringValue(void)")]
// 0xf63234 — j___ZN3RBX11StringValueC2Ev
pub fn stub_0xf63234() -> ! {
    todo!("0xf63234 j___ZN3RBX11StringValueC2Ev")
}

#[doc(alias = "RBX::StringValue::~StringValue()")]
// 0xf63244 — j___ZN3RBX11StringValueD2Ev
pub fn stub_0xf63244() -> ! {
    todo!("0xf63244 j___ZN3RBX11StringValueD2Ev")
}

#[doc(alias = "RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
// 0xf63284 — j___ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEE
pub fn stub_0xf63284() -> ! {
    todo!("0xf63284 j___ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Region3>(RBX::Region3 const&)")]
// 0xf63584 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIS2_EERS3_RKT_
pub fn stub_0xf63584() -> ! {
    todo!("0xf63584 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIS2_EERS3_RKT_")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,bool)>::operator()(std::string,bool)")]
// 0xf635a4 — j___ZN3rbx7signals16signal_with_argsILi2EFvSsbEEclESsb
pub fn stub_0xf635a4() -> ! {
    todo!("0xf635a4 j___ZN3rbx7signals16signal_with_argsILi2EFvSsbEEclESsb")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,bool)>::disconnectAll(void)")]
// 0xf635f4 — j___ZN3rbx7signals6signalIFvSsbEE13disconnectAllEv
pub fn stub_0xf635f4() -> ! {
    todo!("0xf635f4 j___ZN3rbx7signals6signalIFvSsbEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,bool)>::slot> &)")]
// 0xf63604 — j___ZN3rbx7signals6signalIFvSsbEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_0xf63604() -> ! {
    todo!("0xf63604 j___ZN3rbx7signals6signalIFvSsbEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,bool)>::insert(rbx::signals::signal<void ()(std::string,bool)>::slot *)")]
// 0xf63614 — j___ZN3rbx7signals6signalIFvSsbEE6insertEPNS3_4slotE
pub fn stub_0xf63614() -> ! {
    todo!("0xf63614 j___ZN3rbx7signals6signalIFvSsbEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,bool)>::remove(rbx::signals::signal<void ()(std::string,bool)>::slot *)")]
// 0xf63624 — j___ZN3rbx7signals6signalIFvSsbEE6removeEPNS3_4slotE
pub fn stub_0xf63624() -> ! {
    todo!("0xf63624 j___ZN3rbx7signals6signalIFvSsbEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,bool)>::slot,boost::function<void ()(std::string,bool)>,2,void ()(std::string,bool)>::~callable()")]
// 0xf63654 — j___ZN3rbx8callableINS_7signals6signalIFvSsbEE4slotEN5boost8functionIS3_EELi2ES3_ED2Ev
pub fn stub_0xf63654() -> ! {
    todo!("0xf63654 j___ZN3rbx8callableINS_7signals6signalIFvSsbEE4slotEN5boost8functionIS3_EELi2ES3_ED2Ev")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::parse(std::string const&)")]
// 0xf63724 — j___ZN5boost12basic_formatIcSt11char_traitsIcESaIcEE5parseERKSs
pub fn stub_0xf63724() -> ! {
    todo!("0xf63724 j___ZN5boost12basic_formatIcSt11char_traitsIcESaIcEE5parseERKSs")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::~basic_format()")]
// 0xf63734 — j___ZN5boost12basic_formatIcSt11char_traitsIcESaIcEED2Ev
pub fn stub_0xf63734() -> ! {
    todo!("0xf63734 j___ZN5boost12basic_formatIcSt11char_traitsIcESaIcEED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,bool)>::slot>::operator=(rbx::signals::signal<void ()(std::string,bool)>::slot*)")]
// 0xf63774 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsbEE4slotEEaSEPS6_
pub fn stub_0xf63774() -> ! {
    todo!("0xf63774 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsbEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,bool)>::slot> const&)")]
// 0xf63784 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsbEE4slotEEaSERKS7_
pub fn stub_0xf63784() -> ! {
    todo!("0xf63784 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsbEE4slotEEaSERKS7_")
}

#[doc(alias = "void boost::throw_exception<boost::io::too_few_args>(boost::io::too_few_args const&)")]
// 0xf63794 — j___ZN5boost15throw_exceptionINS_2io12too_few_argsEEEvRKT_
pub fn stub_0xf63794() -> ! {
    todo!("0xf63794 j___ZN5boost15throw_exceptionINS_2io12too_few_argsEEEvRKT_")
}

#[doc(alias = "void boost::throw_exception<boost::io::bad_format_string>(boost::io::bad_format_string const&)")]
// 0xf637a4 — j___ZN5boost15throw_exceptionINS_2io17bad_format_stringEEEvRKT_
pub fn stub_0xf637a4() -> ! {
    todo!("0xf637a4 j___ZN5boost15throw_exceptionINS_2io17bad_format_stringEEEvRKT_")
}

#[doc(alias = "void boost::throw_exception<std::bad_alloc>(std::bad_alloc const&)")]
// 0xf637b4 — j___ZN5boost15throw_exceptionISt9bad_allocEEvRKT_
pub fn stub_0xf637b4() -> ! {
    todo!("0xf637b4 j___ZN5boost15throw_exceptionISt9bad_allocEEvRKT_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_tag)")]
// 0xf637c4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEEC1ERKS6_NS6_9clone_tagE
pub fn stub_0xf637c4() -> ! {
    todo!("0xf637c4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone_impl(boost::exception_detail::error_info_injector<std::bad_alloc> const&)")]
// 0xf637d4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEEC1ERKS4_
pub fn stub_0xf637d4() -> ! {
    todo!("0xf637d4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEEC1ERKS4_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone_tag)")]
// 0xf637e4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEEC1ERKS5_NS5_9clone_tagE
pub fn stub_0xf637e4() -> ! {
    todo!("0xf637e4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "int boost::io::detail::upper_bound_from_fstring<std::string,std::ctype<char>>(std::string const&,std::string::value_type,std::ctype<char> const&,unsigned char)")]
// 0xf637f4 — j___ZN5boost2io6detail24upper_bound_from_fstringISsSt5ctypeIcEEEiRKT_NS5_10value_typeERKT0_h
pub fn stub_0xf637f4() -> ! {
    todo!("0xf637f4 j___ZN5boost2io6detail24upper_bound_from_fstringISsSt5ctypeIcEEEiRKT_NS5_10value_typeERKT0_h")
}

#[doc(alias = "boost::pool<boost::default_user_allocator_new_delete>::ordered_malloc(unsigned long)")]
// 0xf63914 — j___ZN5boost4poolINS_33default_user_allocator_new_deleteEE14ordered_mallocEm
pub fn stub_0xf63914() -> ! {
    todo!("0xf63914 j___ZN5boost4poolINS_33default_user_allocator_new_deleteEE14ordered_mallocEm")
}

#[doc(alias = "boost::pool<boost::default_user_allocator_new_delete>::malloc_need_resize(void)")]
// 0xf63924 — j___ZN5boost4poolINS_33default_user_allocator_new_deleteEE18malloc_need_resizeEv
pub fn stub_0xf63924() -> ! {
    todo!("0xf63924 j___ZN5boost4poolINS_33default_user_allocator_new_deleteEE18malloc_need_resizeEv")
}

#[doc(alias = "boost::detail::shared_count::~shared_count()")]
// 0xf63934 — j___ZN5boost6detail12shared_countD1Ev
pub fn stub_0xf63934() -> ! {
    todo!("0xf63934 j___ZN5boost6detail12shared_countD1Ev")
}

#[doc(alias = "void boost::unordered::detail::array_constructor<boost::fast_pool_allocator<boost::unordered::detail::ptr_bucket,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::construct<boost::unordered::detail::ptr_bucket>(boost::unordered::detail::ptr_bucket const&,unsigned long)")]
// 0xf63b44 — j___ZN5boost9unordered6detail17array_constructorINS_19fast_pool_allocatorINS1_10ptr_bucketENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEE9constructIS4_EEvRKT_m
pub fn stub_0xf63b44() -> ! {
    todo!("0xf63b44 j___ZN5boost9unordered6detail17array_constructorINS_19fast_pool_allocatorINS1_10ptr_bucketENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEE9constructIS4_EEvRKT_m")
}

#[doc(alias = "RBX::ReplicatedStorage * RBX::ServiceProvider::find<RBX::ReplicatedStorage>(void)const")]
// 0xf63ce4 — j___ZNK3RBX15ServiceProvider4findINS_17ReplicatedStorageEEEPT_v
pub fn stub_0xf63ce4() -> ! {
    todo!("0xf63ce4 j___ZNK3RBX15ServiceProvider4findINS_17ReplicatedStorageEEEPT_v")
}

#[doc(alias = "RBX::Stats::StatsService * RBX::ServiceProvider::find<RBX::Stats::StatsService>(void)const")]
// 0xf63cf4 — j___ZNK3RBX15ServiceProvider4findINS_5Stats12StatsServiceEEEPT_v
pub fn stub_0xf63cf4() -> ! {
    todo!("0xf63cf4 j___ZNK3RBX15ServiceProvider4findINS_5Stats12StatsServiceEEEPT_v")
}

#[doc(alias = "RBX::Lighting * RBX::ServiceProvider::find<RBX::Lighting>(void)const")]
// 0xf63d24 — j___ZNK3RBX15ServiceProvider4findINS_8LightingEEEPT_v
pub fn stub_0xf63d24() -> ! {
    todo!("0xf63d24 j___ZNK3RBX15ServiceProvider4findINS_8LightingEEEPT_v")
}

#[doc(alias = "RBX::TestService * RBX::ServiceProvider::create<RBX::TestService>(void)const")]
// 0xf63d34 — j___ZNK3RBX15ServiceProvider6createINS_11TestServiceEEEPT_v
pub fn stub_0xf63d34() -> ! {
    todo!("0xf63d34 j___ZNK3RBX15ServiceProvider6createINS_11TestServiceEEEPT_v")
}

#[doc(alias = "RBX::ReplicatedStorage * RBX::ServiceProvider::create<RBX::ReplicatedStorage>(void)const")]
// 0xf63d44 — j___ZNK3RBX15ServiceProvider6createINS_17ReplicatedStorageEEEPT_v
pub fn stub_0xf63d44() -> ! {
    todo!("0xf63d44 j___ZNK3RBX15ServiceProvider6createINS_17ReplicatedStorageEEEPT_v")
}

#[doc(alias = "RBX::MarketplaceService * RBX::ServiceProvider::create<RBX::MarketplaceService>(void)const")]
// 0xf63d54 — j___ZNK3RBX15ServiceProvider6createINS_18MarketplaceServiceEEEPT_v
pub fn stub_0xf63d54() -> ! {
    todo!("0xf63d54 j___ZNK3RBX15ServiceProvider6createINS_18MarketplaceServiceEEEPT_v")
}

#[doc(alias = "RBX::Teams * RBX::ServiceProvider::create<RBX::Teams>(void)const")]
// 0xf63d64 — j___ZNK3RBX15ServiceProvider6createINS_5TeamsEEEPT_v
pub fn stub_0xf63d64() -> ! {
    todo!("0xf63d64 j___ZNK3RBX15ServiceProvider6createINS_5TeamsEEEPT_v")
}

#[doc(alias = "RBX::Lighting * RBX::ServiceProvider::create<RBX::Lighting>(void)const")]
// 0xf63d84 — j___ZNK3RBX15ServiceProvider6createINS_8LightingEEEPT_v
pub fn stub_0xf63d84() -> ! {
    todo!("0xf63d84 j___ZNK3RBX15ServiceProvider6createINS_8LightingEEEPT_v")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone(void)const")]
// 0xf63ed4 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE5cloneEv
pub fn stub_0xf63ed4() -> ! {
    todo!("0xf63ed4 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE5cloneEv")
}

#[doc(alias = "boost::function2<void,std::string,bool>::operator()(std::string,bool)const")]
// 0xf63fe4 — j___ZNK5boost9function2IvSsbEclESsb
pub fn stub_0xf63fe4() -> ! {
    todo!("0xf63fe4 j___ZNK5boost9function2IvSsbEclESsb")
}

#[doc(alias = "std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>::operator=(std::vector<RBX::UintSet,std::allocator<RBX::UintSet>> const&)")]
// 0xf640a4 — j___ZNSt6vectorIN3RBX7UintSetESaIS1_EEaSERKS3_
pub fn stub_0xf640a4() -> ! {
    todo!("0xf640a4 j___ZNSt6vectorIN3RBX7UintSetESaIS1_EEaSERKS3_")
}

#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>*,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&)")]
// 0xf640b4 — j___ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS7_S9_EEmRKS7_
pub fn stub_0xf640b4() -> ! {
    todo!("0xf640b4 j___ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS7_S9_EEmRKS7_")
}

#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::~vector()")]
// 0xf640c4 — j___ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EED1Ev
pub fn stub_0xf640c4() -> ! {
    todo!("0xf640c4 j___ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EED1Ev")
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::operator=(std::vector<unsigned int,std::allocator<unsigned int>> const&)")]
// 0xf64124 — j___ZNSt6vectorIjSaIjEEaSERKS1_
pub fn stub_0xf64124() -> ! {
    todo!("0xf64124 j___ZNSt6vectorIjSaIjEEaSERKS1_")
}

#[doc(alias = "std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,unsigned char>,std::_Select1st<std::pair<RBX::SystemAddress const,unsigned char>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,unsigned char>>>::_M_insert_unique(std::pair<RBX::SystemAddress const,unsigned char> const&)")]
// 0xf64134 — j___ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_hESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE16_M_insert_uniqueERKS4_
pub fn stub_0xf64134() -> ! {
    todo!("0xf64134 j___ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_hESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE16_M_insert_uniqueERKS4_")
}

#[doc(alias = "std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,unsigned char>,std::_Select1st<std::pair<RBX::SystemAddress const,unsigned char>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,unsigned char>>>::erase(RBX::SystemAddress const&)")]
// 0xf64144 — j___ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_hESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE5eraseERS3_
pub fn stub_0xf64144() -> ! {
    todo!("0xf64144 j___ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_hESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE5eraseERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,unsigned char>,std::_Select1st<std::pair<RBX::SystemAddress const,unsigned char>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,unsigned char>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SystemAddress const,unsigned char>> *)")]
// 0xf64154 — j___ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_hESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
pub fn stub_0xf64154() -> ! {
    todo!("0xf64154 j___ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_hESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned char>,std::_Select1st<std::pair<std::string const,unsigned char>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned char>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned char>> *)")]
// 0xf642d4 — j___ZNSt8_Rb_treeISsSt4pairIKSshESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_0xf642d4() -> ! {
    todo!("0xf642d4 j___ZNSt8_Rb_treeISsSt4pairIKSshESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> * std::__uninitialized_copy_a<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>)")]
// 0xf642e4 — j___ZSt22__uninitialized_copy_aIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEES8_S7_ET0_T_SA_S9_SaIT1_E
pub fn stub_0xf642e4() -> ! {
    todo!("0xf642e4 j___ZSt22__uninitialized_copy_aIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEES8_S7_ET0_T_SA_S9_SaIT1_E")
}

#[doc(alias = "RBX::UintSet* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::UintSet const*,std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>>,RBX::UintSet*>(__gnu_cxx::__normal_iterator<RBX::UintSet const*,std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>>,__gnu_cxx::__normal_iterator<RBX::UintSet const*,std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>>,RBX::UintSet*,std::__false_type)")]
// 0xf642f4 — j___ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX7UintSetESt6vectorIS3_SaIS3_EEEEPS3_ET0_T_SC_SB_St12__false_type
pub fn stub_0xf642f4() -> ! {
    todo!("0xf642f4 j___ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX7UintSetESt6vectorIS3_SaIS3_EEEEPS3_ET0_T_SC_SB_St12__false_type")
}

#[doc(alias = "void std::__uninitialized_fill_n_a<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>)")]
// 0xf64314 — j___ZSt24__uninitialized_fill_n_aIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEEmS7_S7_EvT_T0_RKT1_SaIT2_E
pub fn stub_0xf64314() -> ! {
    todo!("0xf64314 j___ZSt24__uninitialized_fill_n_aIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEEmS7_S7_EvT_T0_RKT1_SaIT2_E")
}

#[doc(alias = "void boost::checked_delete<XmlElement>(XmlElement *)")]
// 0xf64324 — j___ZN5boost14checked_deleteI10XmlElementEEvPT_
pub fn stub_0xf64324() -> ! {
    todo!("0xf64324 j___ZN5boost14checked_deleteI10XmlElementEEvPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsService> RBX::shared_from<RBX::PhysicsService>(RBX::PhysicsService*)")]
// 0xf643a4 — j___ZN3RBX11shared_fromINS_14PhysicsServiceEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_0xf643a4() -> ! {
    todo!("0xf643a4 j___ZN3RBX11shared_fromINS_14PhysicsServiceEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "boost::shared_lock<boost::shared_mutex>::lock(void)")]
// 0xf64404 — j___ZN5boost11shared_lockINS_12shared_mutexEE4lockEv
pub fn stub_0xf64404() -> ! {
    todo!("0xf64404 j___ZN5boost11shared_lockINS_12shared_mutexEE4lockEv")
}

#[doc(alias = "boost::unique_lock<boost::shared_mutex>::lock(void)")]
// 0xf64414 — j___ZN5boost11unique_lockINS_12shared_mutexEE4lockEv
pub fn stub_0xf64414() -> ! {
    todo!("0xf64414 j___ZN5boost11unique_lockINS_12shared_mutexEE4lockEv")
}

#[doc(alias = "boost::shared_mutex::lock_upgrade(void)")]
// 0xf64424 — j___ZN5boost12shared_mutex12lock_upgradeEv
pub fn stub_0xf64424() -> ! {
    todo!("0xf64424 j___ZN5boost12shared_mutex12lock_upgradeEv")
}

#[doc(alias = "boost::shared_mutex::unlock_shared(void)")]
// 0xf64434 — j___ZN5boost12shared_mutex13unlock_sharedEv
pub fn stub_0xf64434() -> ! {
    todo!("0xf64434 j___ZN5boost12shared_mutex13unlock_sharedEv")
}

#[doc(alias = "boost::shared_mutex::unlock_upgrade(void)")]
// 0xf64444 — j___ZN5boost12shared_mutex14unlock_upgradeEv
pub fn stub_0xf64444() -> ! {
    todo!("0xf64444 j___ZN5boost12shared_mutex14unlock_upgradeEv")
}

#[doc(alias = "boost::shared_mutex::unlock_upgrade_and_lock(void)")]
// 0xf64454 — j___ZN5boost12shared_mutex23unlock_upgrade_and_lockEv
pub fn stub_0xf64454() -> ! {
    todo!("0xf64454 j___ZN5boost12shared_mutex23unlock_upgrade_and_lockEv")
}

#[doc(alias = "boost::shared_mutex::~shared_mutex()")]
// 0xf64464 — j___ZN5boost12shared_mutexD2Ev
pub fn stub_0xf64464() -> ! {
    todo!("0xf64464 j___ZN5boost12shared_mutexD2Ev")
}

#[doc(alias = "boost::upgrade_lock<boost::shared_mutex>::lock(void)")]
// 0xf64474 — j___ZN5boost12upgrade_lockINS_12shared_mutexEE4lockEv
pub fn stub_0xf64474() -> ! {
    todo!("0xf64474 j___ZN5boost12upgrade_lockINS_12shared_mutexEE4lockEv")
}

#[doc(alias = "boost::upgrade_to_unique_lock<boost::shared_mutex>::~upgrade_to_unique_lock()")]
// 0xf64484 — j___ZN5boost22upgrade_to_unique_lockINS_12shared_mutexEED2Ev
pub fn stub_0xf64484() -> ! {
    todo!("0xf64484 j___ZN5boost22upgrade_to_unique_lockINS_12shared_mutexEED2Ev")
}

#[doc(alias = "std::_List_base<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_clear(void)")]
// 0xf645a4 — j___ZNSt10_List_baseIN3rbx7signals10connectionESaIS2_EE8_M_clearEv
pub fn stub_0xf645a4() -> ! {
    todo!("0xf645a4 j___ZNSt10_List_baseIN3rbx7signals10connectionESaIS2_EE8_M_clearEv")
}

#[doc(alias = "std::list<rbx::signals::connection,std::allocator<rbx::signals::connection>>::erase(std::_List_iterator<rbx::signals::connection>)")]
// 0xf645b4 — j___ZNSt4listIN3rbx7signals10connectionESaIS2_EE5eraseESt14_List_iteratorIS2_E
pub fn stub_0xf645b4() -> ! {
    todo!("0xf645b4 j___ZNSt4listIN3rbx7signals10connectionESaIS2_EE5eraseESt14_List_iteratorIS2_E")
}

#[doc(alias = "std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>::vector(unsigned long,RBX::UintSet const&,std::allocator<RBX::UintSet> const&)")]
// 0xf646b4 — j___ZNSt6vectorIN3RBX7UintSetESaIS1_EEC2EmRKS1_RKS2_
pub fn stub_0xf646b4() -> ! {
    todo!("0xf646b4 j___ZNSt6vectorIN3RBX7UintSetESaIS1_EEC2EmRKS1_RKS2_")
}

#[doc(alias = "boost::shared_mutex::lock_shared(void)")]
// 0xf646f4 — j___ZN5boost12shared_mutex11lock_sharedEv
pub fn stub_0xf646f4() -> ! {
    todo!("0xf646f4 j___ZN5boost12shared_mutex11lock_sharedEv")
}

#[doc(alias = "boost::shared_mutex::release_waiters(void)")]
// 0xf64704 — j___ZN5boost12shared_mutex15release_waitersEv
pub fn stub_0xf64704() -> ! {
    todo!("0xf64704 j___ZN5boost12shared_mutex15release_waitersEv")
}

#[doc(alias = "boost::shared_mutex::lock(void)")]
// 0xf64714 — j___ZN5boost12shared_mutex4lockEv
pub fn stub_0xf64714() -> ! {
    todo!("0xf64714 j___ZN5boost12shared_mutex4lockEv")
}

#[doc(alias = "boost::shared_mutex::shared_mutex(void)")]
// 0xf64724 — j___ZN5boost12shared_mutexC2Ev
pub fn stub_0xf64724() -> ! {
    todo!("0xf64724 j___ZN5boost12shared_mutexC2Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>> const&)")]
// 0xf64734 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEEC1ERKS5_
pub fn stub_0xf64734() -> ! {
    todo!("0xf64734 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEEC1ERKS5_")
}

#[doc(alias = "boost::condition_variable::wait(boost::unique_lock<boost::mutex> &)")]
// 0xf64744 — j___ZN5boost18condition_variable4waitERNS_11unique_lockINS_5mutexEEE
pub fn stub_0xf64744() -> ! {
    todo!("0xf64744 j___ZN5boost18condition_variable4waitERNS_11unique_lockINS_5mutexEEE")
}

#[doc(alias = "boost::detail::interruption_checker::interruption_checker(_opaque_pthread_mutex_t *,_opaque_pthread_cond_t *)")]
// 0xf64754 — j___ZN5boost6detail20interruption_checkerC2EP23_opaque_pthread_mutex_tP22_opaque_pthread_cond_t
pub fn stub_0xf64754() -> ! {
    todo!("0xf64754 j___ZN5boost6detail20interruption_checkerC2EP23_opaque_pthread_mutex_tP22_opaque_pthread_cond_t")
}

#[doc(alias = "void RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getPrimitivesOverlapping<RBX::DenseHashSet<RBX::Primitive*,boost::hash<RBX::Primitive*>,std::allocator<RBX::Primitive*>>>(RBX::Extents const&,RBX::DenseHashSet<RBX::Primitive*,boost::hash<RBX::Primitive*>,std::allocator<RBX::Primitive*>> &)")]
// 0xf647c4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE24getPrimitivesOverlappingINS_12DenseHashSetIPS1_N5boost4hashIS7_EESaIS7_EEEEEvRKNS_7ExtentsERT_
pub fn stub_0xf647c4() -> ! {
    todo!("0xf647c4 j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE24getPrimitivesOverlappingINS_12DenseHashSetIPS1_N5boost4hashIS7_EESaIS7_EEEEEvRKNS_7ExtentsERT_")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::unregisterCoarseMovementCallback(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback *)")]
// 0xf647d4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE32unregisterCoarseMovementCallbackEPNS4_22CoarseMovementCallbackE
pub fn stub_0xf647d4() -> ! {
    todo!("0xf647d4 j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE32unregisterCoarseMovementCallbackEPNS4_22CoarseMovementCallbackE")
}

#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::insert(RBX::Primitive * const&)")]
// 0xf647e4 — j___ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE6insertERKS2_
pub fn stub_0xf647e4() -> ! {
    todo!("0xf647e4 j___ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE6insertERKS2_")
}

#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::rehash(void)")]
// 0xf647f4 — j___ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE6rehashEv
pub fn stub_0xf647f4() -> ! {
    todo!("0xf647f4 j___ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE6rehashEv")
}

#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::DenseHashSet(RBX::Primitive * const&,unsigned long,boost::hash<RBX::Primitive *> const&)")]
// 0xf64804 — j___ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EEC2ERKS2_mRKS5_
pub fn stub_0xf64804() -> ! {
    todo!("0xf64804 j___ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EEC2ERKS2_mRKS5_")
}

#[doc(alias = "RBX::SpatialHashStatic::safeExtents(RBX::Extents const&)")]
// 0xf64814 — j___ZN3RBX17SpatialHashStatic11safeExtentsERKNS_7ExtentsE
pub fn stub_0xf64814() -> ! {
    todo!("0xf64814 j___ZN3RBX17SpatialHashStatic11safeExtentsERKNS_7ExtentsE")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::StreamRegion::Id>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::StreamRegion::Id>,RBX::StreamRegion::Id,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::StreamRegion::Id>>(RBX::StreamRegion::Id const&,boost::unordered::detail::emplace_args1<RBX::StreamRegion::Id> const&)")]
// 0xf64874 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
pub fn stub_0xf64874() -> ! {
    todo!("0xf64874 j___ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::StreamRegion::Id>,RBX::StreamRegion::Id,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::erase_key(RBX::StreamRegion::Id const&)")]
// 0xf64884 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE9erase_keyERKS6_
pub fn stub_0xf64884() -> ! {
    todo!("0xf64884 j___ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE9erase_keyERKS6_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::StreamRegion::Id>,RBX::StreamRegion::Id,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::create_buckets(unsigned long)")]
// 0xf64894 — j___ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14create_bucketsEm
pub fn stub_0xf64894() -> ! {
    todo!("0xf64894 j___ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::StreamRegion::Id>,RBX::StreamRegion::Id,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::reserve_for_insert(unsigned long)")]
// 0xf648a4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE18reserve_for_insertEm
pub fn stub_0xf648a4() -> ! {
    todo!("0xf648a4 j___ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE18reserve_for_insertEm")
}

#[doc(alias = "bool RBX::StreamRegion::IdExtents::intersectsContainer<boost::unordered::unordered_set<RBX::StreamRegion::Id,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>,std::allocator<RBX::StreamRegion::Id>>>(boost::unordered::unordered_set<RBX::StreamRegion::Id,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>,std::allocator<RBX::StreamRegion::Id>> const&,RBX::StreamRegion::Id*)const")]
// 0xf648b4 — j___ZNK3RBX12StreamRegion9IdExtents19intersectsContainerIN5boost9unordered13unordered_setINS0_2IdENS6_27boost_compatible_hash_valueESt8equal_toIS6_ESaIS6_EEEEEbRKT_PS6_
pub fn stub_0xf648b4() -> ! {
    todo!("0xf648b4 j___ZNK3RBX12StreamRegion9IdExtents19intersectsContainerIN5boost9unordered13unordered_setINS0_2IdENS6_27boost_compatible_hash_valueESt8equal_toIS6_ESaIS6_EEEEEbRKT_PS6_")
}

#[doc(alias = "std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback *,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback **,std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback *,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback *>>>,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback * const&)")]
// 0xf64954 — j___ZNSt6vectorIPN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE22CoarseMovementCallbackESaIS7_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_S9_EERKS7_
pub fn stub_0xf64954() -> ! {
    todo!("0xf64954 j___ZNSt6vectorIPN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE22CoarseMovementCallbackESaIS7_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_S9_EERKS7_")
}

#[doc(alias = "std::_Rb_tree<float,std::pair<float const,RBX::StreamRegion::Id>,std::_Select1st<std::pair<float const,RBX::StreamRegion::Id>>,std::less<float>,std::allocator<std::pair<float const,RBX::StreamRegion::Id>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,RBX::StreamRegion::Id>> *)")]
// 0xf64964 — j___ZNSt8_Rb_treeIfSt4pairIKfN3RBX12StreamRegion2IdEESt10_Select1stIS5_ESt4lessIfESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0xf64964() -> ! {
    todo!("0xf64964 j___ZNSt8_Rb_treeIfSt4pairIKfN3RBX12StreamRegion2IdEESt10_Select1stIS5_ESt4lessIfESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "RBX::JointsService * RBX::ServiceProvider::find<RBX::JointsService>(void)const")]
// 0xf649f4 — j___ZNK3RBX15ServiceProvider4findINS_13JointsServiceEEEPT_v
pub fn stub_0xf649f4() -> ! {
    todo!("0xf649f4 j___ZNK3RBX15ServiceProvider4findINS_13JointsServiceEEEPT_v")
}

#[doc(alias = "std::vector<RBX::Guid::Data,std::allocator<RBX::Guid::Data>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Guid::Data*,std::vector<RBX::Guid::Data,std::allocator<RBX::Guid::Data>>>,RBX::Guid::Data const&)")]
// 0xf64a74 — j___ZNSt6vectorIN3RBX4Guid4DataESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf64a74() -> ! {
    todo!("0xf64a74 j___ZNSt6vectorIN3RBX4Guid4DataESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "RBX::MaterialGenerator::~MaterialGenerator()")]
// 0xf64ae4 — j___ZN3RBX17MaterialGeneratorD2Ev
pub fn stub_0xf64ae4() -> ! {
    todo!("0xf64ae4 j___ZN3RBX17MaterialGeneratorD2Ev")
}

#[doc(alias = "RBX::FastClusterShadowData::~FastClusterShadowData()")]
// 0xf64af4 — j___ZN3RBX21FastClusterShadowDataD2Ev
pub fn stub_0xf64af4() -> ! {
    todo!("0xf64af4 j___ZN3RBX21FastClusterShadowDataD2Ev")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::MaterialGroup::MaterialGroup(RBX::FastClusterMeshGenerator::MaterialGroup const&)")]
// 0xf64b24 — j___ZN3RBX24FastClusterMeshGenerator13MaterialGroupC2ERKS1_
pub fn stub_0xf64b24() -> ! {
    todo!("0xf64b24 j___ZN3RBX24FastClusterMeshGenerator13MaterialGroupC2ERKS1_")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::MaterialGroup::~MaterialGroup()")]
// 0xf64b34 — j___ZN3RBX24FastClusterMeshGenerator13MaterialGroupD1Ev
pub fn stub_0xf64b34() -> ! {
    todo!("0xf64b34 j___ZN3RBX24FastClusterMeshGenerator13MaterialGroupD1Ev")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::finalizeMerged(RBX::FastCluster *,RBX::eShadowCullingPriority,RBX::FastClusterSharedGeometry &)")]
// 0xf64b44 — j___ZN3RBX24FastClusterMeshGenerator14finalizeMergedEPNS_11FastClusterENS_22eShadowCullingPriorityERNS_25FastClusterSharedGeometryE
pub fn stub_0xf64b44() -> ! {
    todo!("0xf64b44 j___ZN3RBX24FastClusterMeshGenerator14finalizeMergedEPNS_11FastClusterENS_22eShadowCullingPriorityERNS_25FastClusterSharedGeometryE")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::createIndexData(unsigned int)")]
// 0xf64b54 — j___ZN3RBX24FastClusterMeshGenerator15createIndexDataEj
pub fn stub_0xf64b54() -> ! {
    todo!("0xf64b54 j___ZN3RBX24FastClusterMeshGenerator15createIndexDataEj")
}
