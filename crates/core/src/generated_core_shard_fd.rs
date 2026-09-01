//! core shard FD — 100 core stubs EA-sorted, lowest uncovered 0xf27624..0xf28804 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after FC 0xf275f4).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf275f4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot*)")]
// 0xf27624 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot*)
pub fn stub_f27624() -> ! {
    todo!("0xf27624 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)")]
// 0xf27654 — j___ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX7TextBoxEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)
pub fn stub_f27654() -> ! {
    todo!("0xf27654 j___ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX7TextBoxEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>> const&)")]
// 0xf27664 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::TextBox>> const&)
pub fn stub_f27664() -> ! {
    todo!("0xf27664 j___ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::clear(void)")]
// 0xf27674 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::clear(void)
pub fn stub_f27674() -> ! {
    todo!("0xf27674 j___ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::operator()(rbx_core::SharedPtr<RBX::TextBox>)const")]
// 0xf276a4 — j___ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::operator()(boost::shared_ptr<RBX::TextBox>)const
pub fn stub_f276a4() -> ! {
    todo!("0xf276a4 j___ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox>&&)")]
// 0xf276d4 — j___ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_
// was: boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox>&&)
pub fn stub_f276d4() -> ! {
    todo!("0xf276d4 j___ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox> const&)")]
// 0xf276e4 — j___ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_
// was: boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox> const&)
pub fn stub_f276e4() -> ! {
    todo!("0xf276e4 j___ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string)>::slot> &)")]
// 0xf276f4 — j___ZN3rbx7signals6signalIFvSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot> &)
pub fn stub_f276f4() -> ! {
    todo!("0xf276f4 j___ZN3rbx7signals6signalIFvSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "std::_Vector_base<void *,std::allocator<void *>>::_M_allocate(unsigned long)")]
// 0xf27714 — j___ZNSt12_Vector_baseIPvSaIS0_EE11_M_allocateEm
pub fn stub_f27714() -> ! {
    todo!("0xf27714 j___ZNSt12_Vector_baseIPvSaIS0_EE11_M_allocateEm")
}

#[doc(alias = "std::vector<void *,std::allocator<void *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<void **,std::vector<void *,std::allocator<void *>>>,void * const&)")]
// 0xf27724 — j___ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_
pub fn stub_f27724() -> ! {
    todo!("0xf27724 j___ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_")
}

#[doc(alias = "std::vector<void *,std::allocator<void *>>::push_back(void * const&)")]
// 0xf27734 — j___ZNSt6vectorIPvSaIS0_EE9push_backERKS0_
pub fn stub_f27734() -> ! {
    todo!("0xf27734 j___ZNSt6vectorIPvSaIS0_EE9push_backERKS0_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_do_get_mutex(void)")]
// 0xf27744 — j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv
pub fn stub_f27744() -> ! {
    todo!("0xf27744 j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_do_get_mutex(void)")]
// 0xf27754 — j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f27754() -> ! {
    todo!("0xf27754 j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::insert(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
// 0xf27764 — j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6insertEPNS7_4slotE
pub fn stub_f27764() -> ! {
    todo!("0xf27764 j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6insertEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::remove(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
// 0xf27774 — j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6removeEPNS7_4slotE
pub fn stub_f27774() -> ! {
    todo!("0xf27774 j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6removeEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::connect<boost::function<void ()(RBX::StandardOutMessage const&)>>(boost::function<void ()(RBX::StandardOutMessage const&)> const&)")]
// 0xf27784 — j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE7connectIN5boost8functionIS6_EEEENS0_10connectionERKT_
pub fn stub_f27784() -> ! {
    todo!("0xf27784 j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE7connectIN5boost8functionIS6_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*>(boost::function<void ()(RBX::StandardOutMessage const&)> const&,rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*)")]
// 0xf27794 — j___ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_EC2IPS8_EERKSC_T_
pub fn stub_f27794() -> ! {
    todo!("0xf27794 j___ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_EC2IPS8_EERKSC_T_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)")]
// 0xf277a4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)
pub fn stub_f277a4() -> ! {
    todo!("0xf277a4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSEPSA_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)")]
// 0xf277b4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSERKSB_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)
pub fn stub_f277b4() -> ! {
    todo!("0xf277b4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSERKSB_")
}

#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::assign_to_own(boost::function1<void,RBX::StandardOutMessage const&> const&)")]
// 0xf277c4 — j___ZN5boost9function1IvRKN3RBX18StandardOutMessageEE13assign_to_ownERKS5_
pub fn stub_f277c4() -> ! {
    todo!("0xf277c4 j___ZN5boost9function1IvRKN3RBX18StandardOutMessageEE13assign_to_ownERKS5_")
}

#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::clear(void)")]
// 0xf277d4 — j___ZN5boost9function1IvRKN3RBX18StandardOutMessageEE5clearEv
pub fn stub_f277d4() -> ! {
    todo!("0xf277d4 j___ZN5boost9function1IvRKN3RBX18StandardOutMessageEE5clearEv")
}

#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::operator()(RBX::StandardOutMessage const&)const")]
// 0xf277e4 — j___ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_
pub fn stub_f277e4() -> ! {
    todo!("0xf277e4 j___ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::find(std::string const&)")]
// 0xf28324 — j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_
pub fn stub_f28324() -> ! {
    todo!("0xf28324 j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_")
}

#[doc(alias = "rbx_core::SharedPtr<boost::detail::thread_data_base>::operator=(rbx_core::SharedPtr<boost::detail::thread_data_base> const&)")]
// 0xf28334 — j___ZN5boost10shared_ptrINS_6detail16thread_data_baseEEaSERKS3_
// was: boost::shared_ptr<boost::detail::thread_data_base>::operator=(boost::shared_ptr<boost::detail::thread_data_base> const&)
pub fn stub_f28334() -> ! {
    todo!("0xf28334 j___ZN5boost10shared_ptrINS_6detail16thread_data_baseEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<boost::detail::tss_cleanup_function>::operator=(rbx_core::SharedPtr<boost::detail::tss_cleanup_function> const&)")]
// 0xf28344 — j___ZN5boost10shared_ptrINS_6detail20tss_cleanup_functionEEaSERKS3_
// was: boost::shared_ptr<boost::detail::tss_cleanup_function>::operator=(boost::shared_ptr<boost::detail::tss_cleanup_function> const&)
pub fn stub_f28344() -> ! {
    todo!("0xf28344 j___ZN5boost10shared_ptrINS_6detail20tss_cleanup_functionEEaSERKS3_")
}

#[doc(alias = "boost::detail::future_object_base::mark_finished_internal(boost::unique_lock<boost::mutex> &)")]
// 0xf28354 — j___ZN5boost6detail18future_object_base22mark_finished_internalERNS_11unique_lockINS_5mutexEEE
pub fn stub_f28354() -> ! {
    todo!("0xf28354 j___ZN5boost6detail18future_object_base22mark_finished_internalERNS_11unique_lockINS_5mutexEEE")
}

#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data_base>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data_base *)const")]
// 0xf28364 — j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data_base>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data_base *)const
pub fn stub_f28364() -> ! {
    todo!("0xf28364 j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_create_node(std::pair<void const* const,boost::detail::tss_data_node> const&)")]
// 0xf28374 — j___ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE14_M_create_nodeERKS7_
pub fn stub_f28374() -> ! {
    todo!("0xf28374 j___ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE14_M_create_nodeERKS7_")
}

#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_insert_unique(std::pair<void const* const,boost::detail::tss_data_node> const&)")]
// 0xf28384 — j___ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_f28384() -> ! {
    todo!("0xf28384 j___ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::erase(std::_Rb_tree_iterator<std::pair<void const* const,boost::detail::tss_data_node>>,std::_Rb_tree_iterator<std::pair<void const* const,boost::detail::tss_data_node>>)")]
// 0xf28394 — j___ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_
pub fn stub_f28394() -> ! {
    todo!("0xf28394 j___ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_")
}

#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_erase(std::_Rb_tree_node<std::pair<void const* const,boost::detail::tss_data_node>> *)")]
// 0xf283a4 — j___ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_f283a4() -> ! {
    todo!("0xf283a4 j___ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::system::error_code)")]
// 0xf283b4 — j___ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE
pub fn stub_f283b4() -> ! {
    todo!("0xf283b4 j___ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE")
}

#[doc(alias = "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::filesystem::path const&,boost::system::error_code)")]
// 0xf283c4 — j___ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE
pub fn stub_f283c4() -> ! {
    todo!("0xf283c4 j___ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<boost::filesystem::filesystem_error::m_imp,boost::filesystem::filesystem_error::m_imp>(rbx_core::SharedPtr<boost::filesystem::filesystem_error::m_imp> *,boost::filesystem::filesystem_error::m_imp *,boost::detail::shared_count &)")]
// 0xf283d4 — j___ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// was: void boost::detail::sp_pointer_construct<boost::filesystem::filesystem_error::m_imp,boost::filesystem::filesystem_error::m_imp>(boost::shared_ptr<boost::filesystem::filesystem_error::m_imp> *,boost::filesystem::filesystem_error::m_imp *,boost::detail::shared_count &)
pub fn stub_f283d4() -> ! {
    todo!("0xf283d4 j___ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::filesystem::path::path<char const*>(char const*,char const*)")]
// 0xf283e4 — j___ZN5boost10filesystem4pathC2IPKcEET_S5_
pub fn stub_f283e4() -> ! {
    todo!("0xf283e4 j___ZN5boost10filesystem4pathC2IPKcEET_S5_")
}

#[doc(alias = "std::locale::locale<boost::filesystem::detail::utf8_codecvt_facet>(std::locale const&,boost::filesystem::detail::utf8_codecvt_facet *)")]
// 0xf283f4 — j___ZNSt6localeC2IN5boost10filesystem6detail18utf8_codecvt_facetEEERKS_PT_
pub fn stub_f283f4() -> ! {
    todo!("0xf283f4 j___ZNSt6localeC2IN5boost10filesystem6detail18utf8_codecvt_facetEEERKS_PT_")
}

#[doc(alias = "void boost::throw_exception<boost::iostreams::zlib_error>(boost::iostreams::zlib_error const&)")]
// 0xf28404 — j___ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_
pub fn stub_f28404() -> ! {
    todo!("0xf28404 j___ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::iostreams::zlib_error> const&)")]
// 0xf28414 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS5_
pub fn stub_f28414() -> ! {
    todo!("0xf28414 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS5_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&)")]
// 0xf28424 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_
pub fn stub_f28424() -> ! {
    todo!("0xf28424 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_tag)")]
// 0xf28434 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_NS6_9clone_tagE
pub fn stub_f28434() -> ! {
    todo!("0xf28434 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const")]
// 0xf28444 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv
pub fn stub_f28444() -> ! {
    todo!("0xf28444 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv")
}

#[doc(alias = "boost::date_time::second_clock<boost::posix_time::ptime>::create_time(tm *)")]
// 0xf28464 — j___ZN5boost9date_time12second_clockINS_10posix_time5ptimeEE11create_timeEP2tm
pub fn stub_f28464() -> ! {
    todo!("0xf28464 j___ZN5boost9date_time12second_clockINS_10posix_time5ptimeEE11create_timeEP2tm")
}

#[doc(alias = "boost::date_time::gregorian_calendar_base<boost::date_time::year_month_day_base<boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day>,unsigned int>::from_day_number(unsigned int)")]
// 0xf28474 — j___ZN5boost9date_time23gregorian_calendar_baseINS0_19year_month_day_baseINS_9gregorian9greg_yearENS3_10greg_monthENS3_8greg_dayEEEjE15from_day_numberEj
pub fn stub_f28474() -> ! {
    todo!("0xf28474 j___ZN5boost9date_time23gregorian_calendar_baseINS0_19year_month_day_baseINS_9gregorian9greg_yearENS3_10greg_monthENS3_8greg_dayEEEjE15from_day_numberEj")
}

#[doc(alias = "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::date(void)const")]
// 0xf28484 — j___ZNK5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEE4dateEv
pub fn stub_f28484() -> ! {
    todo!("0xf28484 j___ZNK5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEE4dateEv")
}

#[doc(alias = "void boost::throw_exception<boost::condition_error>(boost::condition_error const&)")]
// 0xf28494 — j___ZN5boost15throw_exceptionINS_15condition_errorEEEvRKT_
pub fn stub_f28494() -> ! {
    todo!("0xf28494 j___ZN5boost15throw_exceptionINS_15condition_errorEEEvRKT_")
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::reset(std::string *)")]
// 0xf284a4 — j___ZN5boost19thread_specific_ptrISsE5resetEPSs
pub fn stub_f284a4() -> ! {
    todo!("0xf284a4 j___ZN5boost19thread_specific_ptrISsE5resetEPSs")
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::~thread_specific_ptr()")]
// 0xf284b4 — j___ZN5boost19thread_specific_ptrISsED2Ev
pub fn stub_f284b4() -> ! {
    todo!("0xf284b4 j___ZN5boost19thread_specific_ptrISsED2Ev")
}

#[doc(alias = "void boost::condition_variable_any::wait<boost::unique_lock<boost::mutex>>(boost::unique_lock<boost::mutex> &)")]
// 0xf284c4 — j___ZN5boost22condition_variable_any4waitINS_11unique_lockINS_5mutexEEEEEvRT_
pub fn stub_f284c4() -> ! {
    todo!("0xf284c4 j___ZN5boost22condition_variable_any4waitINS_11unique_lockINS_5mutexEEEEEvRT_")
}

#[doc(alias = "boost::condition_variable_any::condition_variable_any(void)")]
// 0xf284d4 — j___ZN5boost22condition_variable_anyC2Ev
pub fn stub_f284d4() -> ! {
    todo!("0xf284d4 j___ZN5boost22condition_variable_anyC2Ev")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
// 0xf284e4 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)
pub fn stub_f284e4() -> ! {
    todo!("0xf284e4 j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) &,boost::_bi::list0 &,int)")]
// 0xf284f4 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEclIPFvS7_RKSB_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::operator()<void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) &,boost::_bi::list0 &,int)
pub fn stub_f284f4() -> ! {
    todo!("0xf284f4 j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEclIPFvS7_RKSB_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")]
// 0xf28504 — j___ZN5boost3_bi5list2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_
pub fn stub_f28504() -> ! {
    todo!("0xf28504 j___ZN5boost3_bi5list2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>::bind_t(void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>> const&)")]
// 0xf28514 — j___ZN5boost3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS4_11work_resultEEEENS0_5list2INS0_5valueIS6_EENSF_IS9_EEEEEC2ESD_RKSI_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>::bind_t(void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>> const&)
pub fn stub_f28514() -> ! {
    todo!("0xf28514 j___ZN5boost3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS4_11work_resultEEEENS0_5list2INS0_5valueIS6_EENSF_IS9_EEEEEC2ESD_RKSI_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
// 0xf28524 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)
pub fn stub_f28524() -> ! {
    todo!("0xf28524 j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")]
// 0xf28534 — j___ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_
pub fn stub_f28534() -> ! {
    todo!("0xf28534 j___ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&,rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>(void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>)")]
// 0xf28544 — j___ZN5boost4bindIvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS3_11work_resultEEES5_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list_av_2<boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>::type> boost::bind<void,boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&,boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>(void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>)
pub fn stub_f28544() -> ! {
    todo!("0xf28544 j___ZN5boost4bindIvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS3_11work_resultEEES5_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list_av_2<boost::function0<void>,std::string>::type> boost::bind<void,boost::function0<void> const&,std::string,boost::function0<void>,std::string>(void (*)(boost::function0<void> const&,std::string),boost::function0<void>,std::string)")]
// 0xf28554 — j___ZN5boost4bindIvRKNS_9function0IvEESsS2_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
pub fn stub_f28554() -> ! {
    todo!("0xf28554 j___ZN5boost4bindIvRKNS_9function0IvEESsS2_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::worker_thread::data,RBX::worker_thread::data>(rbx_core::SharedPtr<RBX::worker_thread::data> *,RBX::worker_thread::data *,boost::detail::shared_count &)")]
// 0xf28564 — j___ZN5boost6detail20sp_pointer_constructIN3RBX13worker_thread4dataES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// was: void boost::detail::sp_pointer_construct<RBX::worker_thread::data,RBX::worker_thread::data>(boost::shared_ptr<RBX::worker_thread::data> *,RBX::worker_thread::data *,boost::detail::shared_count &)
pub fn stub_f28564() -> ! {
    todo!("0xf28564 j___ZN5boost6detail20sp_pointer_constructIN3RBX13worker_thread4dataES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf28574 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_f28574() -> ! {
    todo!("0xf28574 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf28584 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_f28584() -> ! {
    todo!("0xf28584 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>)")]
// 0xf28594 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS0_INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSH_ISB_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>)
pub fn stub_f28594() -> ! {
    todo!("0xf28594 j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS0_INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSH_ISB_EEEEEEEEvT_")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>)")]
// 0xf285a4 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_SsENS3_5list2INS3_5valueIS1_EENSA_ISsEEEEEEEEvT_
pub fn stub_f285a4() -> ! {
    todo!("0xf285a4 j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_SsENS3_5list2INS3_5valueIS1_EENSA_ISsEEEEEEEEvT_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf285b4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS9_11work_resultEEEENS5_5list2INS5_5valueISB_EENSK_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_f285b4() -> ! {
    todo!("0xf285b4 j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS9_11work_resultEEEENS5_5list2INS5_5valueISB_EENSK_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf285c4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS5_5list2INS5_5valueIS8_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_f285c4() -> ! {
    todo!("0xf285c4 j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS5_5list2INS5_5valueIS8_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::condition_variable::do_wait_until(boost::unique_lock<boost::mutex> &,timespec const&)")]
// 0xf285d4 — j___ZN5boost18condition_variable13do_wait_untilERNS_11unique_lockINS_5mutexEEERK8timespec
pub fn stub_f285d4() -> ! {
    todo!("0xf285d4 j___ZN5boost18condition_variable13do_wait_untilERNS_11unique_lockINS_5mutexEEERK8timespec")
}

#[doc(alias = "RBX::Limits::Counter::safe_static_do_get_current(void)")]
// 0xf285e4 — j___ZN3RBX6Limits7Counter26safe_static_do_get_currentEv
pub fn stub_f285e4() -> ! {
    todo!("0xf285e4 j___ZN3RBX6Limits7Counter26safe_static_do_get_currentEv")
}

#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::reset(rbx_core::SharedPtr<RBX::Limits::Counter>*)")]
// 0xf285f4 — j___ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE5resetEPS5_
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::reset(boost::shared_ptr<RBX::Limits::Counter>*)
pub fn stub_f285f4() -> ! {
    todo!("0xf285f4 j___ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE5resetEPS5_")
}

#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::~thread_specific_ptr()")]
// 0xf28604 — j___ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEED2Ev
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::~thread_specific_ptr()
pub fn stub_f28604() -> ! {
    todo!("0xf28604 j___ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEED2Ev")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job **,std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>>,RBX::TaskScheduler::Job * const&)")]
// 0xf28614 — j___ZNSt6vectorIPN3RBX13TaskScheduler3JobESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
pub fn stub_f28614() -> ! {
    todo!("0xf28614 j___ZNSt6vectorIPN3RBX13TaskScheduler3JobESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "RBX::RunningAverage<int,double>::sample(int)")]
// 0xf28624 — j___ZN3RBX14RunningAverageIidE6sampleEi
pub fn stub_f28624() -> ! {
    todo!("0xf28624 j___ZN3RBX14RunningAverageIidE6sampleEi")
}

#[doc(alias = "RBX::mutex::mutex(void)")]
// 0xf28634 — j___ZN3RBX5mutexC2Ev
pub fn stub_f28634() -> ! {
    todo!("0xf28634 j___ZN3RBX5mutexC2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(rbx_core::WeakPtr<RBX::TaskScheduler::Job> const&)")]
// 0xf28644 — j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE
// was: boost::shared_ptr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(boost::weak_ptr<RBX::TaskScheduler::Job> const&)
pub fn stub_f28644() -> ! {
    todo!("0xf28644 j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&)")]
// 0xf28654 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_
pub fn stub_f28654() -> ! {
    todo!("0xf28654 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_")
}

#[doc(alias = "boost::condition_variable::condition_variable(void)")]
// 0xf28664 — j___ZN5boost18condition_variableC2Ev
pub fn stub_f28664() -> ! {
    todo!("0xf28664 j___ZN5boost18condition_variableC2Ev")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::~thread_specific_ptr()")]
// 0xf28674 — j___ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev
pub fn stub_f28674() -> ! {
    todo!("0xf28674 j___ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CEvent>(RBX::CEvent *)")]
// 0xf28684 — j___ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_
pub fn stub_f28684() -> ! {
    todo!("0xf28684 j___ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_")
}

#[doc(alias = "RBX::TaskScheduler::Job::getDebugName(void)const")]
// 0xf28694 — j___ZNK3RBX13TaskScheduler3Job12getDebugNameEv
pub fn stub_f28694() -> ! {
    todo!("0xf28694 j___ZNK3RBX13TaskScheduler3Job12getDebugNameEv")
}

#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const")]
// 0xf286a4 — j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_9function0IvEEEEEEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const
pub fn stub_f286a4() -> ! {
    todo!("0xf286a4 j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_9function0IvEEEEEEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_create_node(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
// 0xf286b4 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_create_node(boost::shared_ptr<RBX::TaskScheduler::Job> const&)
pub fn stub_f286b4() -> ! {
    todo!("0xf286b4 j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
// 0xf286c4 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_unique(boost::shared_ptr<RBX::TaskScheduler::Job> const&)
pub fn stub_f286c4() -> ! {
    todo!("0xf286c4 j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>)")]
// 0xf286d4 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>)
pub fn stub_f286d4() -> ! {
    todo!("0xf286d4 j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::TaskScheduler::Job>> *)")]
// 0xf286e4 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::TaskScheduler::Job>> *)
pub fn stub_f286e4() -> ! {
    todo!("0xf286e4 j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "void RBX::WindowAverage<double,double>::sample<RBX::FOnBeforeDrop>(double,RBX::FOnBeforeDrop &)")]
// 0xf286f4 — j___ZN3RBX13WindowAverageIddE6sampleINS_13FOnBeforeDropEEEvdRT_
pub fn stub_f286f4() -> ! {
    todo!("0xf286f4 j___ZN3RBX13WindowAverageIddE6sampleINS_13FOnBeforeDropEEEvdRT_")
}

#[doc(alias = "RBX::RunningAverage<double,double>::RunningAverage(double,double,unsigned int)")]
// 0xf28704 — j___ZN3RBX14RunningAverageIddEC2Eddj
pub fn stub_f28704() -> ! {
    todo!("0xf28704 j___ZN3RBX14RunningAverageIddEC2Eddj")
}

#[doc(alias = "RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::sample(RBX::Time::Interval)")]
// 0xf28714 — j___ZN3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE6sampleENS1_8IntervalE
pub fn stub_f28714() -> ! {
    todo!("0xf28714 j___ZN3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE6sampleENS1_8IntervalE")
}

#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample(void)")]
// 0xf28724 — j___ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv
pub fn stub_f28724() -> ! {
    todo!("0xf28724 j___ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv")
}

#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::WindowAverageTimeInterval(RBX::Time::Interval)")]
// 0xf28734 — j___ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EEC2ENS1_8IntervalE
pub fn stub_f28734() -> ! {
    todo!("0xf28734 j___ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EEC2ENS1_8IntervalE")
}

#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::allocate(unsigned long)")]
// 0xf28744 — j___ZN5boost15circular_bufferIdSaIdEE8allocateEm
pub fn stub_f28744() -> ! {
    todo!("0xf28744 j___ZN5boost15circular_bufferIdSaIdEE8allocateEm")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::circular_buffer<double,std::allocator<double>>>(boost::circular_buffer<double,std::allocator<double>> *)")]
// 0xf28754 — j___ZN5boost6detail12shared_countC2INS_15circular_bufferIdSaIdEEEEEPT_
pub fn stub_f28754() -> ! {
    todo!("0xf28754 j___ZN5boost6detail12shared_countC2INS_15circular_bufferIdSaIdEEEEEPT_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const")]
// 0xf28764 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv
pub fn stub_f28764() -> ! {
    todo!("0xf28764 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *)")]
// 0xf28774 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::Tasks::Coordinator> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *>(boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *)
pub fn stub_f28774() -> ! {
    todo!("0xf28774 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *)")]
// 0xf28784 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::Tasks::Coordinator> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *>(boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *)
pub fn stub_f28784() -> ! {
    todo!("0xf28784 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>*,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&)")]
// 0xf28794 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// was: std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator>*,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::shared_ptr<RBX::Tasks::Coordinator> const&)
pub fn stub_f28794() -> ! {
    todo!("0xf28794 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::push_back(rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&)")]
// 0xf287a4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE9push_backERKS5_
// was: std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>::push_back(boost::shared_ptr<RBX::Tasks::Coordinator> const&)
pub fn stub_f287a4() -> ! {
    todo!("0xf287a4 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE9push_backERKS5_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>> std::__find_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>,std::random_access_iterator_tag)")]
// 0xf287b4 — j___ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESt6vectorIS7_SaIS7_EEEENS2_3_bi6bind_tIbNS2_4_mfi3mf1IbS6_PNS4_13TaskScheduler3JobEEENSD_5list2INS2_3argILi1EEENSD_5valueISJ_EEEEEEET_SS_SS_T0_St26random_access_iterator_tag
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>> std::__find_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>,std::random_access_iterator_tag)
pub fn stub_f287b4() -> ! {
    todo!("0xf287b4 j___ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESt6vectorIS7_SaIS7_EEEENS2_3_bi6bind_tIbNS2_4_mfi3mf1IbS6_PNS4_13TaskScheduler3JobEEENSD_5list2INS2_3argILi1EEENSD_5valueISJ_EEEEEEET_SS_SS_T0_St26random_access_iterator_tag")
}

#[doc(alias = "RBX::TaskScheduler::Thread::join(void)")]
// 0xf287c4 — j___ZN3RBX13TaskScheduler6Thread4joinEv
pub fn stub_f287c4() -> ! {
    todo!("0xf287c4 j___ZN3RBX13TaskScheduler6Thread4joinEv")
}

#[doc(alias = "RBX::TaskScheduler::Thread::create(RBX::TaskScheduler*)")]
// 0xf287d4 — j___ZN3RBX13TaskScheduler6Thread6createEPS0_
pub fn stub_f287d4() -> ! {
    todo!("0xf287d4 j___ZN3RBX13TaskScheduler6Thread6createEPS0_")
}

#[doc(alias = "RBX::TaskScheduler::Thread::~Thread()")]
// 0xf287e4 — j___ZN3RBX13TaskScheduler6ThreadD2Ev
pub fn stub_f287e4() -> ! {
    todo!("0xf287e4 j___ZN3RBX13TaskScheduler6ThreadD2Ev")
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::sample(void)")]
// 0xf287f4 — j___ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE6sampleEv
pub fn stub_f287f4() -> ! {
    todo!("0xf287f4 j___ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE6sampleEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Thread>::shared_ptr<RBX::TaskScheduler::Thread>(rbx_core::WeakPtr<RBX::TaskScheduler::Thread> const&)")]
// 0xf28804 — j___ZN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEC2IS3_EERKNS_8weak_ptrIT_EE
// was: boost::shared_ptr<RBX::TaskScheduler::Thread>::shared_ptr<RBX::TaskScheduler::Thread>(boost::weak_ptr<RBX::TaskScheduler::Thread> const&)
pub fn stub_f28804() -> ! {
    todo!("0xf28804 j___ZN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEC2IS3_EERKNS_8weak_ptrIT_EE")
}
