//! core shard EF — 100 core stubs EA-sorted, lowest uncovered 0x8d05a0..0x8e4e70 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EE 0x8d049c).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "__gnu_cxx::__normal_iterator<char *,std::string> std::transform<__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::to_lowerF<char>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::to_lowerF<char>)")]
// 0x8d05a0 — __ZSt9transformIN9__gnu_cxx17__normal_iteratorIPcSsEES3_N5boost9algorithm6detail9to_lowerFIcEEET0_T_SA_S9_T1_
pub fn stub_8d05a0() -> ! {
    todo!("0x8d05a0 __ZSt9transformIN9__gnu_cxx17__normal_iteratorIPcSsEES3_N5boost9algorithm6detail9to_lowerFIcEEET0_T_SA_S9_T1_")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(int,int,bool)>::operator()(int,int,bool)")]
// 0x8d1ef0 — __ZN3rbx7signals16signal_with_argsILi3EFviibEEclEiib
pub fn stub_8d1ef0() -> ! {
    todo!("0x8d1ef0 __ZN3rbx7signals16signal_with_argsILi3EFviibEEclEiib")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,bool)>::slot> &)")]
// 0x8d2040 — __ZN3rbx7signals6signalIFviibEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(int,int,bool)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,bool)>::slot> &)
pub fn stub_8d2040() -> ! {
    todo!("0x8d2040 __ZN3rbx7signals6signalIFviibEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::on_error(std::exception &)")]
// 0x8d21a0 — __ZN3rbx7signals6signalIFviibEE8on_errorERSt9exception
pub fn stub_8d21a0() -> ! {
    todo!("0x8d21a0 __ZN3rbx7signals6signalIFviibEE8on_errorERSt9exception")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,bool)>::slot> const&)")]
// 0x8d21c8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviibEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,bool)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,bool)>::slot> const&)
pub fn stub_8d21c8() -> ! {
    todo!("0x8d21c8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviibEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::safe_static_init_mutex(void)")]
// 0x8d21ec — __ZN3rbx7signals6signalIFviibEE22safe_static_init_mutexEv
pub fn stub_8d21ec() -> ! {
    todo!("0x8d21ec __ZN3rbx7signals6signalIFviibEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::safe_static_do_get_mutex(void)")]
// 0x8d21f0 — __ZN3rbx7signals6signalIFviibEE24safe_static_do_get_mutexEv
pub fn stub_8d21f0() -> ! {
    todo!("0x8d21f0 __ZN3rbx7signals6signalIFviibEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "boost::function1<void,std::string>::dummy::nonnull(void)")]
// 0x8d3130 — __ZN5boost9function1IvSsE5dummy7nonnullEv
pub fn stub_8d3130() -> ! {
    todo!("0x8d3130 __ZN5boost9function1IvSsE5dummy7nonnullEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::disconnectAll(void)")]
// 0x8d3408 — __ZN3rbx7signals6signalIFvSsiiEE13disconnectAllEv
pub fn stub_8d3408() -> ! {
    todo!("0x8d3408 __ZN3rbx7signals6signalIFvSsiiEE13disconnectAllEv")
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,bool)>::remote_signal(void)")]
// 0x8d3854 — __ZN3rbx13remote_signalIFviibEEC2Ev
pub fn stub_8d3854() -> ! {
    todo!("0x8d3854 __ZN3rbx13remote_signalIFviibEEC2Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::disconnectAll(void)")]
// 0x8d39b0 — __ZN3rbx7signals6signalIFviibEE13disconnectAllEv
pub fn stub_8d39b0() -> ! {
    todo!("0x8d39b0 __ZN3rbx7signals6signalIFviibEE13disconnectAllEv")
}

#[doc(alias = "RBX::MarketplaceService::CurrencyType * rbx::any_cast<RBX::MarketplaceService::CurrencyType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x8d40f0 — __ZN3rbx8any_castIN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_8d40f0() -> ! {
    todo!("0x8d40f0 __ZN3rbx8any_castIN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::MarketplaceService::CurrencyType & rbx::any_cast<RBX::MarketplaceService::CurrencyType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x8d414c — __ZN3rbx8any_castIRN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_8d414c() -> ! {
    todo!("0x8d414c __ZN3rbx8any_castIRN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::resize(unsigned long,RBX::MarketplaceService::CurrencyType)")]
// 0x8d4240 — __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE6resizeEmS2_
pub fn stub_8d4240() -> ! {
    todo!("0x8d4240 __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::push_back(RBX::MarketplaceService::CurrencyType const&)")]
// 0x8d4278 — __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE9push_backERKS2_
pub fn stub_8d4278() -> ! {
    todo!("0x8d4278 __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::MarketplaceService::CurrencyType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::operator[](RBX::Name const* const&)")]
// 0x8d42a4 — __ZNSt3mapIPKN3RBX4NameENS0_18MarketplaceService12CurrencyTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_8d42a4() -> ! {
    todo!("0x8d42a4 __ZNSt3mapIPKN3RBX4NameENS0_18MarketplaceService12CurrencyTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType> const&)")]
// 0x8d42fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_8d42fc() -> ! {
    todo!("0x8d42fc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType> const&)")]
// 0x8d43b0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_8d43b0() -> ! {
    todo!("0x8d43b0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType> const&)")]
// 0x8d4408 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_8d4408() -> ! {
    todo!("0x8d4408 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::MarketplaceService::CurrencyType*,std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>>,RBX::MarketplaceService::CurrencyType const&)")]
// 0x8d4474 — __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_8d4474() -> ! {
    todo!("0x8d4474 __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::_M_allocate(unsigned long)")]
// 0x8d4558 — __ZNSt12_Vector_baseIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE11_M_allocateEm
pub fn stub_8d4558() -> ! {
    todo!("0x8d4558 __ZNSt12_Vector_baseIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::MarketplaceService::CurrencyType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::MarketplaceService::CurrencyType *,RBX::MarketplaceService::CurrencyType *>(RBX::MarketplaceService::CurrencyType *,RBX::MarketplaceService::CurrencyType *,RBX::MarketplaceService::CurrencyType *)")]
// 0x8d4570 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18MarketplaceService12CurrencyTypeES6_EET0_T_S8_S7_
pub fn stub_8d4570() -> ! {
    todo!("0x8d4570 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18MarketplaceService12CurrencyTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::MarketplaceService::CurrencyType*,std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>>,unsigned long,RBX::MarketplaceService::CurrencyType const&)")]
// 0x8d45b0 — __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_8d45b0() -> ! {
    todo!("0x8d45b0 __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "boost::function3<void,int,int,bool>::clear(void)")]
// 0x8d4cdc — __ZN5boost9function3IviibE5clearEv
pub fn stub_8d4cdc() -> ! {
    todo!("0x8d4cdc __ZN5boost9function3IviibE5clearEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int,bool)>::connect<boost::function<void ()(int,int,bool)>>(boost::function<void ()(int,int,bool)> const&)")]
// 0x8d5508 — __ZN3rbx7signals6signalIFviibEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_8d5508() -> ! {
    todo!("0x8d5508 __ZN3rbx7signals6signalIFviibEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::insert(rbx::signals::signal<void ()(int,int,bool)>::slot *)")]
// 0x8d55fc — __ZN3rbx7signals6signalIFviibEE6insertEPNS3_4slotE
pub fn stub_8d55fc() -> ! {
    todo!("0x8d55fc __ZN3rbx7signals6signalIFviibEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,bool)>::slot>::operator=(rbx::signals::signal<void ()(int,int,bool)>::slot*)")]
// 0x8d5808 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviibEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,bool)>::slot>::operator=(rbx::signals::signal<void ()(int,int,bool)>::slot*)
pub fn stub_8d5808() -> ! {
    todo!("0x8d5808 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviibEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,bool)>::slot,boost::function<void ()(int,int,bool)>,3,void ()(int,int,bool)>::callable<rbx::signals::signal<void ()(int,int,bool)>*>(boost::function<void ()(int,int,bool)> const&,rbx::signals::signal<void ()(int,int,bool)>*)")]
// 0x8d582c — __ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_
pub fn stub_8d582c() -> ! {
    todo!("0x8d582c __ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::callable_slot<boost::function<void ()(int,int,bool)>>::~callable_slot()")]
// 0x8d5928 — __ZN3rbx7signals6signalIFviibEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_8d5928() -> ! {
    todo!("0x8d5928 __ZN3rbx7signals6signalIFviibEE13callable_slotIN5boost8functionIS2_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::callable_slot<boost::function<void ()(int,int,bool)>>::~callable_slot()")]
// 0x8d5a38 — __ZN3rbx7signals6signalIFviibEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_8d5a38() -> ! {
    todo!("0x8d5a38 __ZN3rbx7signals6signalIFviibEE13callable_slotIN5boost8functionIS2_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::disconnect(void)")]
// 0x8d5b68 — __ZN3rbx7signals6signalIFviibEE4slot10disconnectEv
pub fn stub_8d5b68() -> ! {
    todo!("0x8d5b68 __ZN3rbx7signals6signalIFviibEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::connected(void)const")]
// 0x8d5c78 — __ZNK3rbx7signals6signalIFviibEE4slot9connectedEv
pub fn stub_8d5c78() -> ! {
    todo!("0x8d5c78 __ZNK3rbx7signals6signalIFviibEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,bool)>::slot,boost::function<void ()(int,int,bool)>,3,void ()(int,int,bool)>::call(int,int,bool)")]
// 0x8d5c84 — __ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_E4callEiib
pub fn stub_8d5c84() -> ! {
    todo!("0x8d5c84 __ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_E4callEiib")
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(int,int,bool)>::slot,boost::function<void ()(int,int,bool)>,3,void ()(int,int,bool)>::call(int,int,bool)")]
// 0x8d5c8c — __ZThn4_N3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_E4callEiib
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(int,int,bool)>::slot,boost::function<void ()(int,int,bool)>,3,void ()(int,int,bool)>::call(int,int,bool)
pub fn stub_8d5c8c() -> ! {
    todo!("0x8d5c8c __ZThn4_N3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_E4callEiib")
}

#[doc(alias = "boost::function3<void,int,int,bool>::operator()(int,int,bool)const")]
// 0x8d5c94 — __ZNK5boost9function3IviibEclEiib
pub fn stub_8d5c94() -> ! {
    todo!("0x8d5c94 __ZNK5boost9function3IviibEclEiib")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::remove(rbx::signals::signal<void ()(int,int,bool)>::slot *)")]
// 0x8d5d60 — __ZN3rbx7signals6signalIFviibEE6removeEPNS3_4slotE
pub fn stub_8d5d60() -> ! {
    todo!("0x8d5d60 __ZN3rbx7signals6signalIFviibEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::safe_static_init_mutex(void)")]
// 0x8d5e50 — __ZN3rbx7signals6signalIFviibEE4slot22safe_static_init_mutexEv
pub fn stub_8d5e50() -> ! {
    todo!("0x8d5e50 __ZN3rbx7signals6signalIFviibEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::safe_static_do_get_mutex(void)")]
// 0x8d5e54 — __ZN3rbx7signals6signalIFviibEE4slot24safe_static_do_get_mutexEv
pub fn stub_8d5e54() -> ! {
    todo!("0x8d5e54 __ZN3rbx7signals6signalIFviibEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,bool)>::slot,boost::function<void ()(int,int,bool)>,3,void ()(int,int,bool)>::~callable()")]
// 0x8d5f48 — __ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
pub fn stub_8d5f48() -> ! {
    todo!("0x8d5f48 __ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,bool)>::slot,boost::function<void ()(int,int,bool)>,3,void ()(int,int,bool)>::~callable()")]
// 0x8d6058 — __ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
pub fn stub_8d6058() -> ! {
    todo!("0x8d6058 __ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::~slot()")]
// 0x8d6188 — __ZN3rbx7signals6signalIFviibEE4slotD1Ev
pub fn stub_8d6188() -> ! {
    todo!("0x8d6188 __ZN3rbx7signals6signalIFviibEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::~slot()")]
// 0x8d61b4 — __ZN3rbx7signals6signalIFviibEE4slotD0Ev
pub fn stub_8d61b4() -> ! {
    todo!("0x8d61b4 __ZN3rbx7signals6signalIFviibEE4slotD0Ev")
}

#[doc(alias = "boost::function3<void,int,int,bool>::assign_to_own(boost::function3<void,int,int,bool> const&)")]
// 0x8d6288 — __ZN5boost9function3IviibE13assign_to_ownERKS1_
pub fn stub_8d6288() -> ! {
    todo!("0x8d6288 __ZN5boost9function3IviibE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::function3<void,std::string,int,int>::clear(void)")]
// 0x8dbfd0 — __ZN5boost9function3IvSsiiE5clearEv
pub fn stub_8dbfd0() -> ! {
    todo!("0x8dbfd0 __ZN5boost9function3IvSsiiE5clearEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::insert(rbx::signals::signal<void ()(std::string,int,int)>::slot *)")]
// 0x8dc2d8 — __ZN3rbx7signals6signalIFvSsiiEE6insertEPNS3_4slotE
pub fn stub_8dc2d8() -> ! {
    todo!("0x8dc2d8 __ZN3rbx7signals6signalIFvSsiiEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::callable<rbx::signals::signal<void ()(std::string,int,int)>*>(boost::function<void ()(std::string,int,int)> const&,rbx::signals::signal<void ()(std::string,int,int)>*)")]
// 0x8dc4e8 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_
pub fn stub_8dc4e8() -> ! {
    todo!("0x8dc4e8 __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
// 0x8dc5e8 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsii
pub fn stub_8dc5e8() -> ! {
    todo!("0x8dc5e8 __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsii")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::remove(rbx::signals::signal<void ()(std::string,int,int)>::slot *)")]
// 0x8dc710 — __ZN3rbx7signals6signalIFvSsiiEE6removeEPNS3_4slotE
pub fn stub_8dc710() -> ! {
    todo!("0x8dc710 __ZN3rbx7signals6signalIFvSsiiEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::safe_static_init_mutex(void)")]
// 0x8dc800 — __ZN3rbx7signals6signalIFvSsiiEE4slot22safe_static_init_mutexEv
pub fn stub_8dc800() -> ! {
    todo!("0x8dc800 __ZN3rbx7signals6signalIFvSsiiEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::~slot()")]
// 0x8dc808 — __ZN3rbx7signals6signalIFvSsiiEE4slotD0Ev
pub fn stub_8dc808() -> ! {
    todo!("0x8dc808 __ZN3rbx7signals6signalIFvSsiiEE4slotD0Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,bool)>::~remote_signal()")]
// 0x8e02d4 — __ZN3rbx13remote_signalIFviibEED2Ev
pub fn stub_8e02d4() -> ! {
    todo!("0x8e02d4 __ZN3rbx13remote_signalIFviibEED2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,int,int)>::~remote_signal()")]
// 0x8e06b8 — __ZN3rbx13remote_signalIFvSsiiEED2Ev
pub fn stub_8e06b8() -> ! {
    todo!("0x8e06b8 __ZN3rbx13remote_signalIFvSsiiEED2Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>> *)")]
// 0x8e0950 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_8e0950() -> ! {
    todo!("0x8e0950 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "RBX::GuiBase2d::GuiBase2d(char const*)")]
// 0x8e1134 — __ZN3RBX9GuiBase2dC2EPKc
pub fn stub_8e1134() -> ! {
    todo!("0x8e1134 __ZN3RBX9GuiBase2dC2EPKc")
}

#[doc(alias = "RBX::GuiBase2d::recursiveRender2d(RBX::Adorn *)")]
// 0x8e1480 — __ZN3RBX9GuiBase2d17recursiveRender2dEPNS_5AdornE
pub fn stub_8e1480() -> ! {
    todo!("0x8e1480 __ZN3RBX9GuiBase2d17recursiveRender2dEPNS_5AdornE")
}

#[doc(alias = "RBX::GuiBase2d::getRect2D(void)const")]
// 0x8e1764 — __ZNK3RBX9GuiBase2d9getRect2DEv
pub fn stub_8e1764() -> ! {
    todo!("0x8e1764 __ZNK3RBX9GuiBase2d9getRect2DEv")
}

#[doc(alias = "RBX::GuiBase2d::getAbsoluteSize(void)const")]
// 0x8e17d4 — __ZNK3RBX9GuiBase2d15getAbsoluteSizeEv
pub fn stub_8e17d4() -> ! {
    todo!("0x8e17d4 __ZNK3RBX9GuiBase2d15getAbsoluteSizeEv")
}

#[doc(alias = "RBX::GuiBase2d::getAbsolutePosition(void)const")]
// 0x8e17fc — __ZNK3RBX9GuiBase2d19getAbsolutePositionEv
pub fn stub_8e17fc() -> ! {
    todo!("0x8e17fc __ZNK3RBX9GuiBase2d19getAbsolutePositionEv")
}

#[doc(alias = "RBX::GuiBase2d::~GuiBase2d()")]
// 0x8e1800 — __ZN3RBX9GuiBase2dD1Ev
pub fn stub_8e1800() -> ! {
    todo!("0x8e1800 __ZN3RBX9GuiBase2dD1Ev")
}

#[doc(alias = "RBX::GuiBase2d::~GuiBase2d()")]
// 0x8e18bc — __ZN3RBX9GuiBase2dD0Ev
pub fn stub_8e18bc() -> ! {
    todo!("0x8e18bc __ZN3RBX9GuiBase2dD0Ev")
}

#[doc(alias = "RBX::GuiBase::process(RBX::GuiEvent const&)")]
// 0x8e19b0 — __ZN3RBX7GuiBase7processERKNS_8GuiEventE
pub fn stub_8e19b0() -> ! {
    todo!("0x8e19b0 __ZN3RBX7GuiBase7processERKNS_8GuiEventE")
}

#[doc(alias = "RBX::GuiBase2d::canProcessMeAndDescendants(void)const")]
// 0x8e19bc — __ZNK3RBX9GuiBase2d26canProcessMeAndDescendantsEv
pub fn stub_8e19bc() -> ! {
    todo!("0x8e19bc __ZNK3RBX9GuiBase2d26canProcessMeAndDescendantsEv")
}

#[doc(alias = "non-virtual thunk to RBX::GuiBase2d::~GuiBase2d()")]
// 0x8e19c0 — __ZThn32_N3RBX9GuiBase2dD1Ev
// was: non-virtual thunk to RBX::GuiBase2d::~GuiBase2d()
pub fn stub_8e19c0() -> ! {
    todo!("0x8e19c0 __ZThn32_N3RBX9GuiBase2dD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::GuiBase2d::~GuiBase2d()")]
// 0x8e1a78 — __ZThn32_N3RBX9GuiBase2dD0Ev
// was: non-virtual thunk to RBX::GuiBase2d::~GuiBase2d()
pub fn stub_8e1a78() -> ! {
    todo!("0x8e1a78 __ZThn32_N3RBX9GuiBase2dD0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::GuiBase2d::~GuiBase2d()")]
// 0x8e1b70 — __ZThn36_N3RBX9GuiBase2dD1Ev
// was: non-virtual thunk to RBX::GuiBase2d::~GuiBase2d()
pub fn stub_8e1b70() -> ! {
    todo!("0x8e1b70 __ZThn36_N3RBX9GuiBase2dD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::GuiBase2d::~GuiBase2d()")]
// 0x8e1c28 — __ZThn36_N3RBX9GuiBase2dD0Ev
// was: non-virtual thunk to RBX::GuiBase2d::~GuiBase2d()
pub fn stub_8e1c28() -> ! {
    todo!("0x8e1c28 __ZThn36_N3RBX9GuiBase2dD0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::GuiBase::process(RBX::GuiEvent const&)")]
// 0x8e1cf8 — __ZThn92_N3RBX7GuiBase7processERKNS_8GuiEventE
// was: non-virtual thunk to RBX::GuiBase::process(RBX::GuiEvent const&)
pub fn stub_8e1cf8() -> ! {
    todo!("0x8e1cf8 __ZThn92_N3RBX7GuiBase7processERKNS_8GuiEventE")
}

#[doc(alias = "RBX::GuiLayerCollector::GuiLayerCollector(char const*)")]
// 0x8e2920 — __ZN3RBX17GuiLayerCollectorC2EPKc
pub fn stub_8e2920() -> ! {
    todo!("0x8e2920 __ZN3RBX17GuiLayerCollectorC2EPKc")
}

#[doc(alias = "RBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2b9c — __ZN3RBX17GuiLayerCollectorD0Ev
pub fn stub_8e2b9c() -> ! {
    todo!("0x8e2b9c __ZN3RBX17GuiLayerCollectorD0Ev")
}

#[doc(alias = "RBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2c3c — __ZN3RBX17GuiLayerCollectorD1Ev
pub fn stub_8e2c3c() -> ! {
    todo!("0x8e2c3c __ZN3RBX17GuiLayerCollectorD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2c40 — __ZThn32_N3RBX17GuiLayerCollectorD0Ev
// was: non-virtual thunk to RBX::GuiLayerCollector::~GuiLayerCollector()
pub fn stub_8e2c40() -> ! {
    todo!("0x8e2c40 __ZThn32_N3RBX17GuiLayerCollectorD0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2c48 — __ZThn36_N3RBX17GuiLayerCollectorD0Ev
// was: non-virtual thunk to RBX::GuiLayerCollector::~GuiLayerCollector()
pub fn stub_8e2c48() -> ! {
    todo!("0x8e2c48 __ZThn36_N3RBX17GuiLayerCollectorD0Ev")
}

#[doc(alias = "RBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2c50 — __ZN3RBX17GuiLayerCollectorD2Ev
pub fn stub_8e2c50() -> ! {
    todo!("0x8e2c50 __ZN3RBX17GuiLayerCollectorD2Ev")
}

#[doc(alias = "non-virtual thunk to RBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2e20 — __ZThn32_N3RBX17GuiLayerCollectorD1Ev
// was: non-virtual thunk to RBX::GuiLayerCollector::~GuiLayerCollector()
pub fn stub_8e2e20() -> ! {
    todo!("0x8e2e20 __ZThn32_N3RBX17GuiLayerCollectorD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2e28 — __ZThn36_N3RBX17GuiLayerCollectorD1Ev
// was: non-virtual thunk to RBX::GuiLayerCollector::~GuiLayerCollector()
pub fn stub_8e2e28() -> ! {
    todo!("0x8e2e28 __ZThn36_N3RBX17GuiLayerCollectorD1Ev")
}

#[doc(alias = "RBX::GuiLayerCollector::loadZVectors(void)")]
// 0x8e30e0 — __ZN3RBX17GuiLayerCollector12loadZVectorsEv
pub fn stub_8e30e0() -> ! {
    todo!("0x8e30e0 __ZN3RBX17GuiLayerCollector12loadZVectorsEv")
}

#[doc(alias = "RBX::GuiLayerCollector::render2d(RBX::Adorn *)")]
// 0x8e32c8 — __ZN3RBX17GuiLayerCollector8render2dEPNS_5AdornE
pub fn stub_8e32c8() -> ! {
    todo!("0x8e32c8 __ZN3RBX17GuiLayerCollector8render2dEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk to RBX::GuiLayerCollector::render2d(RBX::Adorn *)")]
// 0x8e32d4 — __ZThn96_N3RBX17GuiLayerCollector8render2dEPNS_5AdornE
// was: non-virtual thunk to RBX::GuiLayerCollector::render2d(RBX::Adorn *)
pub fn stub_8e32d4() -> ! {
    todo!("0x8e32d4 __ZThn96_N3RBX17GuiLayerCollector8render2dEPNS_5AdornE")
}

#[doc(alias = "RBX::GuiLayerCollector::process(RBX::GuiEvent const&)")]
// 0x8e348c — __ZN3RBX17GuiLayerCollector7processERKNS_8GuiEventE
pub fn stub_8e348c() -> ! {
    todo!("0x8e348c __ZN3RBX17GuiLayerCollector7processERKNS_8GuiEventE")
}

#[doc(alias = "RBX::GuiLayerCollector::processDescendants(RBX::GuiEvent const&)")]
// 0x8e3534 — __ZN3RBX17GuiLayerCollector18processDescendantsERKNS_8GuiEventE
pub fn stub_8e3534() -> ! {
    todo!("0x8e3534 __ZN3RBX17GuiLayerCollector18processDescendantsERKNS_8GuiEventE")
}

#[doc(alias = "non-virtual thunk to RBX::GuiLayerCollector::process(RBX::GuiEvent const&)")]
// 0x8e365c — __ZThn92_N3RBX17GuiLayerCollector7processERKNS_8GuiEventE
// was: non-virtual thunk to RBX::GuiLayerCollector::process(RBX::GuiEvent const&)
pub fn stub_8e365c() -> ! {
    todo!("0x8e365c __ZThn92_N3RBX17GuiLayerCollector7processERKNS_8GuiEventE")
}

#[doc(alias = "std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>>::push_back(std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> const&)")]
// 0x8e3668 — __ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EE9push_backERKS6_
// was: std::vector<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>,std::allocator<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>>::push_back(std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> const&)
pub fn stub_8e3668() -> ! {
    todo!("0x8e3668 __ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EE9push_backERKS6_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::push_back(rbx_core::SharedPtr<RBX::GuiBase> const&)")]
// 0x8e36a8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE9push_backERKS4_
// was: std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::push_back(boost::shared_ptr<RBX::GuiBase> const&)
pub fn stub_8e36a8() -> ! {
    todo!("0x8e36a8 __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE9push_backERKS4_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::resize(unsigned long,rbx_core::SharedPtr<RBX::GuiBase>)")]
// 0x8e36f8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE6resizeEmS4_
// was: std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::resize(unsigned long,boost::shared_ptr<RBX::GuiBase>)
pub fn stub_8e36f8() -> ! {
    todo!("0x8e36f8 __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE6resizeEmS4_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::GuiBase>*)")]
// 0x8e3868 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE15_M_erase_at_endEPS4_
// was: std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::_M_erase_at_end(boost::shared_ptr<RBX::GuiBase>*)
pub fn stub_8e3868() -> ! {
    todo!("0x8e3868 __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE15_M_erase_at_endEPS4_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::GuiBase>*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>,unsigned long,rbx_core::SharedPtr<RBX::GuiBase> const&)")]
// 0x8e3898 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
// was: std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::GuiBase>*,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>,unsigned long,boost::shared_ptr<RBX::GuiBase> const&)
pub fn stub_8e3898() -> ! {
    todo!("0x8e3898 __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_")
}

#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::_M_allocate(unsigned long)")]
// 0x8e3e98 — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE11_M_allocateEm
// was: std::_Vector_base<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::_M_allocate(unsigned long)
pub fn stub_8e3e98() -> ! {
    todo!("0x8e3e98 __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE11_M_allocateEm")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<RBX::GuiBase> *,unsigned long,rbx_core::SharedPtr<RBX::GuiBase>>(rbx_core::SharedPtr<RBX::GuiBase> *,unsigned long,rbx_core::SharedPtr<RBX::GuiBase> const&,std::__false_type)")]
// 0x8e3eb0 — __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX7GuiBaseEEEmS4_EvT_T0_RKT1_St12__false_type
// was: void std::__uninitialized_fill_n_aux<boost::shared_ptr<RBX::GuiBase> *,unsigned long,boost::shared_ptr<RBX::GuiBase>>(boost::shared_ptr<RBX::GuiBase> *,unsigned long,boost::shared_ptr<RBX::GuiBase> const&,std::__false_type)
pub fn stub_8e3eb0() -> ! {
    todo!("0x8e3eb0 __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX7GuiBaseEEEmS4_EvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiBase>::operator=(rbx_core::SharedPtr<RBX::GuiBase> const&)")]
// 0x8e3fd8 — __ZN5boost10shared_ptrIN3RBX7GuiBaseEEaSERKS3_
// was: boost::shared_ptr<RBX::GuiBase>::operator=(boost::shared_ptr<RBX::GuiBase> const&)
pub fn stub_8e3fd8() -> ! {
    todo!("0x8e3fd8 __ZN5boost10shared_ptrIN3RBX7GuiBaseEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiBase> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::GuiBase> *,rbx_core::SharedPtr<RBX::GuiBase> *>(rbx_core::SharedPtr<RBX::GuiBase> *,rbx_core::SharedPtr<RBX::GuiBase> *,rbx_core::SharedPtr<RBX::GuiBase> *)")]
// 0x8e4010 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX7GuiBaseEEES8_EET0_T_SA_S9_
// was: boost::shared_ptr<RBX::GuiBase> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::GuiBase> *,boost::shared_ptr<RBX::GuiBase> *>(boost::shared_ptr<RBX::GuiBase> *,boost::shared_ptr<RBX::GuiBase> *,boost::shared_ptr<RBX::GuiBase> *)
pub fn stub_8e4010() -> ! {
    todo!("0x8e4010 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX7GuiBaseEEES8_EET0_T_SA_S9_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::GuiBase>*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>,rbx_core::SharedPtr<RBX::GuiBase> const&)")]
// 0x8e4134 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// was: std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::GuiBase>*,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>,boost::shared_ptr<RBX::GuiBase> const&)
pub fn stub_8e4134() -> ! {
    todo!("0x8e4134 __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_")
}

#[doc(alias = "std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>*,std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>>>,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> const&)")]
// 0x8e4500 — __ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_
// was: std::vector<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>,std::allocator<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>*,std::vector<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>,std::allocator<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>>>,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> const&)
pub fn stub_8e4500() -> ! {
    todo!("0x8e4500 __ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::operator=(std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> const&)")]
// 0x8e484c — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EEaSERKS6_
// was: std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::operator=(std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> const&)
pub fn stub_8e484c() -> ! {
    todo!("0x8e484c __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EEaSERKS6_")
}

#[doc(alias = "std::_Vector_base<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>>::_M_allocate(unsigned long)")]
// 0x8e4a74 — __ZNSt12_Vector_baseISt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS5_EESaIS7_EE11_M_allocateEm
// was: std::_Vector_base<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>,std::allocator<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>>::_M_allocate(unsigned long)
pub fn stub_8e4a74() -> ! {
    todo!("0x8e4a74 __ZNSt12_Vector_baseISt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS5_EESaIS7_EE11_M_allocateEm")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiBase>* std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::GuiBase> const*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>>(unsigned long,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::GuiBase> const*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::GuiBase> const*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>)")]
// 0x8e4a98 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS4_S6_EEEEPS4_mT_SE_
// was: boost::shared_ptr<RBX::GuiBase>* std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::GuiBase> const*,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>>(unsigned long,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::GuiBase> const*,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::GuiBase> const*,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>)
pub fn stub_8e4a98() -> ! {
    todo!("0x8e4a98 __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS4_S6_EEEEPS4_mT_SE_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiBase> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::GuiBase> *,rbx_core::SharedPtr<RBX::GuiBase> *>(rbx_core::SharedPtr<RBX::GuiBase> *,rbx_core::SharedPtr<RBX::GuiBase> *,rbx_core::SharedPtr<RBX::GuiBase> *)")]
// 0x8e4c10 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX7GuiBaseEEES8_EET0_T_SA_S9_
// was: boost::shared_ptr<RBX::GuiBase> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::GuiBase> *,boost::shared_ptr<RBX::GuiBase> *>(boost::shared_ptr<RBX::GuiBase> *,boost::shared_ptr<RBX::GuiBase> *,boost::shared_ptr<RBX::GuiBase> *)
pub fn stub_8e4c10() -> ! {
    todo!("0x8e4c10 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX7GuiBaseEEES8_EET0_T_SA_S9_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiBase>* std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::GuiBase> const*,rbx_core::SharedPtr<RBX::GuiBase>*>(rbx_core::SharedPtr<RBX::GuiBase> const*,rbx_core::SharedPtr<RBX::GuiBase> const*,rbx_core::SharedPtr<RBX::GuiBase>*)")]
// 0x8e4c5c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN5boost10shared_ptrIN3RBX7GuiBaseEEEPS7_EET0_T_SC_SB_
// was: boost::shared_ptr<RBX::GuiBase>* std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::GuiBase> const*,boost::shared_ptr<RBX::GuiBase>*>(boost::shared_ptr<RBX::GuiBase> const*,boost::shared_ptr<RBX::GuiBase> const*,boost::shared_ptr<RBX::GuiBase>*)
pub fn stub_8e4c5c() -> ! {
    todo!("0x8e4c5c __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN5boost10shared_ptrIN3RBX7GuiBaseEEEPS7_EET0_T_SC_SB_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> *,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> *>(std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> *,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> *,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> *)")]
// 0x8e4ca8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS8_EESB_EET0_T_SD_SC_
// was: std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> *,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> *>(std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> *,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> *,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> *)
pub fn stub_8e4ca8() -> ! {
    todo!("0x8e4ca8 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS8_EESB_EET0_T_SD_SC_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::vector(std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> const&)")]
// 0x8e4d04 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EEC2ERKS6_
// was: std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::vector(std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> const&)
pub fn stub_8e4d04() -> ! {
    todo!("0x8e4d04 __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EEC2ERKS6_")
}

#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::_Vector_base(unsigned long,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>> const&)")]
// 0x8e4e70 — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EEC2EmRKS5_
// was: std::_Vector_base<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::_Vector_base(unsigned long,std::allocator<boost::shared_ptr<RBX::GuiBase>> const&)
pub fn stub_8e4e70() -> ! {
    todo!("0x8e4e70 __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EEC2EmRKS5_")
}