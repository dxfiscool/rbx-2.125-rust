//! core shard HH — 100 core stubs EA-sorted, 0xf5c9c4..0xf5e1a4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HG 0xf5c9b4).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HG 0xf5c9b4 (0xf5c9c4..0xf5e1a4, 21114->21214 covered, 704 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::map<RBX::Name const*,RBX::AssetService::AccessType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::operator[](RBX::Name const* const&)")]
// 0xf5c9c4 — j___ZNSt3mapIPKN3RBX4NameENS0_12AssetService10AccessTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf5c9c4() -> ! {
    todo!("0xf5c9c4 j___ZNSt3mapIPKN3RBX4NameENS0_12AssetService10AccessTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AssetService::AccessType*,std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>>,RBX::AssetService::AccessType const&)")]
// 0xf5c9d4 — j___ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf5c9d4() -> ! {
    todo!("0xf5c9d4 j___ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::AssetService::AccessType*,std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>>,unsigned long,RBX::AssetService::AccessType const&)")]
// 0xf5c9e4 — j___ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf5c9e4() -> ! {
    todo!("0xf5c9e4 j___ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::resize(unsigned long,RBX::AssetService::AccessType)")]
// 0xf5c9f4 — j___ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE6resizeEmS2_
pub fn stub_0xf5c9f4() -> ! {
    todo!("0xf5c9f4 j___ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::push_back(RBX::AssetService::AccessType const&)")]
// 0xf5ca04 — j___ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE9push_backERKS2_
pub fn stub_0xf5ca04() -> ! {
    todo!("0xf5ca04 j___ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::AssetService::AccessType> const&)")]
// 0xf5ca14 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf5ca14() -> ! {
    todo!("0xf5ca14 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::pair<RBX::Name const* const,RBX::AssetService::AccessType> const&)")]
// 0xf5ca24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf5ca24() -> ! {
    todo!("0xf5ca24 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::AssetService::AccessType> const&)")]
// 0xf5ca34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf5ca34() -> ! {
    todo!("0xf5ca34 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "RBX::RemoteEvent::~RemoteEvent()")]
// 0xf5cbb4 — j___ZN3RBX11RemoteEventD2Ev
pub fn stub_0xf5cbb4() -> ! {
    todo!("0xf5cbb4 j___ZN3RBX11RemoteEventD2Ev")
}

#[doc(alias = "RBX::RemoteFunction::~RemoteFunction()")]
// 0xf5cc84 — j___ZN3RBX14RemoteFunctionD2Ev
pub fn stub_0xf5cc84() -> ! {
    todo!("0xf5cc84 j___ZN3RBX14RemoteFunctionD2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(int,std::string)>::remote_signal(void)")]
// 0xf5cd64 — j___ZN3rbx13remote_signalIFviSsEEC2Ev
pub fn stub_0xf5cd64() -> ! {
    todo!("0xf5cd64 j___ZN3rbx13remote_signalIFviSsEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(int,std::string)>::~remote_signal()")]
// 0xf5cd74 — j___ZN3rbx13remote_signalIFviSsEED2Ev
pub fn stub_0xf5cd74() -> ! {
    todo!("0xf5cd74 j___ZN3rbx13remote_signalIFviSsEED2Ev")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(int,std::string)>::operator()(int,std::string)")]
// 0xf5cdb4 — j___ZN3rbx7signals16signal_with_argsILi2EFviSsEEclEiSs
pub fn stub_0xf5cdb4() -> ! {
    todo!("0xf5cdb4 j___ZN3rbx7signals16signal_with_argsILi2EFviSsEEclEiSs")
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::disconnectAll(void)")]
// 0xf5cfd4 — j___ZN3rbx7signals6signalIFviSsEE13disconnectAllEv
pub fn stub_0xf5cfd4() -> ! {
    todo!("0xf5cfd4 j___ZN3rbx7signals6signalIFviSsEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::safe_static_do_get_mutex(void)")]
// 0xf5cfe4 — j___ZN3rbx7signals6signalIFviSsEE24safe_static_do_get_mutexEv
pub fn stub_0xf5cfe4() -> ! {
    todo!("0xf5cfe4 j___ZN3rbx7signals6signalIFviSsEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,std::string)>::slot> &)")]
// 0xf5cff4 — j___ZN3rbx7signals6signalIFviSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_0xf5cff4() -> ! {
    todo!("0xf5cff4 j___ZN3rbx7signals6signalIFviSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::slot::safe_static_do_get_mutex(void)")]
// 0xf5d004 — j___ZN3rbx7signals6signalIFviSsEE4slot24safe_static_do_get_mutexEv
pub fn stub_0xf5d004() -> ! {
    todo!("0xf5d004 j___ZN3rbx7signals6signalIFviSsEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::insert(rbx::signals::signal<void ()(int,std::string)>::slot *)")]
// 0xf5d014 — j___ZN3rbx7signals6signalIFviSsEE6insertEPNS3_4slotE
pub fn stub_0xf5d014() -> ! {
    todo!("0xf5d014 j___ZN3rbx7signals6signalIFviSsEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::remove(rbx::signals::signal<void ()(int,std::string)>::slot *)")]
// 0xf5d024 — j___ZN3rbx7signals6signalIFviSsEE6removeEPNS3_4slotE
pub fn stub_0xf5d024() -> ! {
    todo!("0xf5d024 j___ZN3rbx7signals6signalIFviSsEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0xf5d034 — j___ZN3rbx7signals6signalIFviSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_0xf5d034() -> ! {
    todo!("0xf5d034 j___ZN3rbx7signals6signalIFviSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,std::string)>::connect<boost::function<void ()(int,std::string)>>(boost::function<void ()(int,std::string)> const&)")]
// 0xf5d044 — j___ZN3rbx7signals6signalIFviSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_0xf5d044() -> ! {
    todo!("0xf5d044 j___ZN3rbx7signals6signalIFviSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::on_error(std::exception &)")]
// 0xf5d054 — j___ZN3rbx7signals6signalIFviSsEE8on_errorERSt9exception
pub fn stub_0xf5d054() -> ! {
    todo!("0xf5d054 j___ZN3rbx7signals6signalIFviSsEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::function<void ()(int,std::string)>,2,void ()(int,std::string)>::call(int,std::string)")]
// 0xf5d0e4 — j___ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callEiSs
pub fn stub_0xf5d0e4() -> ! {
    todo!("0xf5d0e4 j___ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callEiSs")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::function<void ()(int,std::string)>,2,void ()(int,std::string)>::callable<rbx::signals::signal<void ()(int,std::string)>*>(boost::function<void ()(int,std::string)> const&,rbx::signals::signal<void ()(int,std::string)>*)")]
// 0xf5d0f4 — j___ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_
pub fn stub_0xf5d0f4() -> ! {
    todo!("0xf5d0f4 j___ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,std::string)>::slot>::operator=(rbx::signals::signal<void ()(int,std::string)>::slot*)")]
// 0xf5d1b4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviSsEE4slotEEaSEPS6_
pub fn stub_0xf5d1b4() -> ! {
    todo!("0xf5d1b4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviSsEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,std::string)>::slot> const&)")]
// 0xf5d1c4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviSsEE4slotEEaSERKS7_
pub fn stub_0xf5d1c4() -> ! {
    todo!("0xf5d1c4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviSsEE4slotEEaSERKS7_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list2<int &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string> &,boost::_bi::list2<int &,std::string &> &,int)")]
// 0xf5d254 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX14RemoteFunctionEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_iSsEENS0_5list2IRiRSsEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0xf5d254() -> ! {
    todo!("0xf5d254 j___ZN5boost3_bi5list3INS0_5valueIPN3RBX14RemoteFunctionEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_iSsEENS0_5list2IRiRSsEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<RBX::SystemAddress>,boost::_bi::value<int>,boost::arg<1>>::operator()<boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string> &,boost::_bi::list1<std::string &> &,int)")]
// 0xf5d284 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS3_13SystemAddressEEENS2_IiEENS_3argILi1EEEEclINS_4_mfi3mf3IvS4_S7_iSsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0xf5d284() -> ! {
    todo!("0xf5d284 j___ZN5boost3_bi5list4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS3_13SystemAddressEEENS2_IiEENS_3argILi1EEEEclINS_4_mfi3mf3IvS4_S7_iSsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::function2<void,int,std::string>::assign_to_own(boost::function2<void,int,std::string> const&)")]
// 0xf5d7d4 — j___ZN5boost9function2IviSsE13assign_to_ownERKS1_
pub fn stub_0xf5d7d4() -> ! {
    todo!("0xf5d7d4 j___ZN5boost9function2IviSsE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::function2<void,int,std::string>::clear(void)")]
// 0xf5d7e4 — j___ZN5boost9function2IviSsE5clearEv
pub fn stub_0xf5d7e4() -> ! {
    todo!("0xf5d7e4 j___ZN5boost9function2IviSsE5clearEv")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>::destroy(std::pair<int const,RBX::RemoteFunction::RemoteInvocation>*)")]
// 0xf5d854 — j___ZN9__gnu_cxx13new_allocatorISt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEEE7destroyEPS6_
pub fn stub_0xf5d854() -> ! {
    todo!("0xf5d854 j___ZN9__gnu_cxx13new_allocatorISt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEEE7destroyEPS6_")
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>::operator()(RBX::RemoteFunction*,int,std::string)const")]
// 0xf5d8d4 — j___ZNK5boost4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEclEPS3_iSs
pub fn stub_0xf5d8d4() -> ! {
    todo!("0xf5d8d4 j___ZNK5boost4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEclEPS3_iSs")
}

#[doc(alias = "boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string>::operator()(RBX::RemoteFunction*,RBX::SystemAddress,int,std::string)const")]
// 0xf5d8f4 — j___ZNK5boost4_mfi3mf3IvN3RBX14RemoteFunctionENS2_13SystemAddressEiSsEclEPS3_S4_iSs
pub fn stub_0xf5d8f4() -> ! {
    todo!("0xf5d8f4 j___ZNK5boost4_mfi3mf3IvN3RBX14RemoteFunctionENS2_13SystemAddressEiSsEclEPS3_S4_iSs")
}

#[doc(alias = "boost::function2<void,int,std::string>::operator()(int,std::string)const")]
// 0xf5db44 — j___ZNK5boost9function2IviSsEclEiSs
pub fn stub_0xf5db44() -> ! {
    todo!("0xf5db44 j___ZNK5boost9function2IviSsEclEiSs")
}

#[doc(alias = "std::_Vector_base<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::_M_allocate(unsigned long)")]
// 0xf5db64 — j___ZNSt12_Vector_baseIN5boost8functionIFvvEEESaIS3_EE11_M_allocateEm
pub fn stub_0xf5db64() -> ! {
    todo!("0xf5db64 j___ZNSt12_Vector_baseIN5boost8functionIFvvEEESaIS3_EE11_M_allocateEm")
}

#[doc(alias = "boost::function<void ()(void)> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::function<void ()(void)> *,boost::function<void ()(void)> *>(boost::function<void ()(void)> *,boost::function<void ()(void)> *,boost::function<void ()(void)> *)")]
// 0xf5db74 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost8functionIFvvEEES7_EET0_T_S9_S8_
pub fn stub_0xf5db74() -> ! {
    todo!("0xf5db74 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost8functionIFvvEEES7_EET0_T_S9_S8_")
}

#[doc(alias = "std::map<int,RBX::RemoteFunction::RemoteInvocation,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::operator[](int const&)")]
// 0xf5db84 — j___ZNSt3mapIiN3RBX14RemoteFunction16RemoteInvocationESt4lessIiESaISt4pairIKiS2_EEEixERS6_
pub fn stub_0xf5db84() -> ! {
    todo!("0xf5db84 j___ZNSt3mapIiN3RBX14RemoteFunction16RemoteInvocationESt4lessIiESaISt4pairIKiS2_EEEixERS6_")
}

#[doc(alias = "std::vector<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::function<void ()(void)>*,std::vector<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>>,boost::function<void ()(void)> const&)")]
// 0xf5db94 — j___ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
pub fn stub_0xf5db94() -> ! {
    todo!("0xf5db94 j___ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "std::vector<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::push_back(boost::function<void ()(void)> const&)")]
// 0xf5dba4 — j___ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE9push_backERKS3_
pub fn stub_0xf5dba4() -> ! {
    todo!("0xf5dba4 j___ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE9push_backERKS3_")
}

#[doc(alias = "std::vector<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::~vector()")]
// 0xf5dbb4 — j___ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EED2Ev
pub fn stub_0xf5dbb4() -> ! {
    todo!("0xf5dbb4 j___ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EED2Ev")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_create_node(std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
// 0xf5dbc4 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_0xf5dbc4() -> ! {
    todo!("0xf5dbc4 j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
// 0xf5dbd4 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_0xf5dbd4() -> ! {
    todo!("0xf5dbd4 j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
// 0xf5dbe4 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_0xf5dbe4() -> ! {
    todo!("0xf5dbe4 j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::erase(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>)")]
// 0xf5dbf4 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E
pub fn stub_0xf5dbf4() -> ! {
    todo!("0xf5dbf4 j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>> *)")]
// 0xf5dc04 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0xf5dc04() -> ! {
    todo!("0xf5dc04 j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
// 0xf5dc14 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_0xf5dc14() -> ! {
    todo!("0xf5dc14 j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "boost::unique_lock<boost::mutex>::unlock(void)")]
// 0xf5dc64 — j___ZN5boost11unique_lockINS_5mutexEE6unlockEv
pub fn stub_0xf5dc64() -> ! {
    todo!("0xf5dc64 j___ZN5boost11unique_lockINS_5mutexEE6unlockEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")]
// 0xf5dc74 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED0Ev
pub fn stub_0xf5dc74() -> ! {
    todo!("0xf5dc74 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::condition_error>::~error_info_injector()")]
// 0xf5dc84 — j___ZN5boost16exception_detail19error_info_injectorINS_15condition_errorEED0Ev
pub fn stub_0xf5dc84() -> ! {
    todo!("0xf5dc84 j___ZN5boost16exception_detail19error_info_injectorINS_15condition_errorEED0Ev")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,FLog::IValueGetSet *>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,FLog::IValueGetSet *>>,std::string,FLog::IValueGetSet *,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<std::pair<std::string const,FLog::IValueGetSet *>>(std::string const&,std::pair<std::string const,FLog::IValueGetSet *> &&)")]
// 0xf5dc94 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE12emplace_implIJS9_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEbERS5_DpOT_
pub fn stub_0xf5dc94() -> ! {
    todo!("0xf5dc94 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE12emplace_implIJS9_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEbERS5_DpOT_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>> *)")]
// 0xf5dca4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_
pub fn stub_0xf5dca4() -> ! {
    todo!("0xf5dca4 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)")]
// 0xf5dcb4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEixERS5_
pub fn stub_0xf5dcb4() -> ! {
    todo!("0xf5dcb4 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEixERS5_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,FLog::IValueGetSet *>>>>::construct_with_value<std::pair<std::string const,FLog::IValueGetSet *>>(std::pair<std::string const,FLog::IValueGetSet *> &&)")]
// 0xf5dcc4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN4FLog12IValueGetSetEEEEEE20construct_with_valueIJS9_EEEvDpOT_
pub fn stub_0xf5dcc4() -> ! {
    todo!("0xf5dcc4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN4FLog12IValueGetSetEEEEEE20construct_with_valueIJS9_EEEvDpOT_")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>>>::construct(void)")]
// 0xf5dcd4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSsEEEEE9constructEv
pub fn stub_0xf5dcd4() -> ! {
    todo!("0xf5dcd4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSsEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,FLog::IValueGetSet *>>,std::string,FLog::IValueGetSet *,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xf5dce4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_0xf5dce4() -> ! {
    todo!("0xf5dce4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,FLog::IValueGetSet *>>,std::string,FLog::IValueGetSet *,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xf5dcf4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_0xf5dcf4() -> ! {
    todo!("0xf5dcf4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xf5dd04 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_0xf5dd04() -> ! {
    todo!("0xf5dd04 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xf5dd14 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_0xf5dd14() -> ! {
    todo!("0xf5dd14 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::clone(void)const")]
// 0xf5dd24 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEE5cloneEv
pub fn stub_0xf5dd24() -> ! {
    todo!("0xf5dd24 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::rethrow(void)const")]
// 0xf5dd34 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEE7rethrowEv
pub fn stub_0xf5dd34() -> ! {
    todo!("0xf5dd34 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEE7rethrowEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,FLog::IValueGetSet *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,FLog::IValueGetSet *>>,std::string,FLog::IValueGetSet *,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xf5dd44 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_
pub fn stub_0xf5dd44() -> ! {
    todo!("0xf5dd44 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_")
}

#[doc(alias = "std::deque<FLog::LogEntry *,std::allocator<FLog::LogEntry *>>::_M_reallocate_map(unsigned long,bool)")]
// 0xf5dd54 — j___ZNSt5dequeIPN4FLog8LogEntryESaIS2_EE17_M_reallocate_mapEmb
pub fn stub_0xf5dd54() -> ! {
    todo!("0xf5dd54 j___ZNSt5dequeIPN4FLog8LogEntryESaIS2_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::vector<RBX::Accoutrement *,std::allocator<RBX::Accoutrement *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Accoutrement **,std::vector<RBX::Accoutrement *,std::allocator<RBX::Accoutrement *>>>,RBX::Accoutrement * const&)")]
// 0xf5dd64 — j___ZNSt6vectorIPN3RBX12AccoutrementESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf5dd64() -> ! {
    todo!("0xf5dd64 j___ZNSt6vectorIPN3RBX12AccoutrementESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "RBX::SpatialRegion::centerOfRegionInGlobalCoordStuds(RBX::SpatialRegion::Id const&)")]
// 0xf5dd94 — j___ZN3RBX13SpatialRegion32centerOfRegionInGlobalCoordStudsERKNS0_2IdE
pub fn stub_0xf5dd94() -> ! {
    todo!("0xf5dd94 j___ZN3RBX13SpatialRegion32centerOfRegionInGlobalCoordStudsERKNS0_2IdE")
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,12u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::malloc(void)")]
// 0xf5dda4 — j___ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj12ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_0xf5dda4() -> ! {
    todo!("0xf5dda4 j___ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj12ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,12u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::is_from(void *)")]
// 0xf5ddb4 — j___ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj12ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv
pub fn stub_0xf5ddb4() -> ! {
    todo!("0xf5ddb4 j___ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj12ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv")
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,4u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::free(void *,unsigned long)")]
// 0xf5ddc4 — j___ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj4ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE4freeEPvm
pub fn stub_0xf5ddc4() -> ! {
    todo!("0xf5ddc4 j___ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj4ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE4freeEPvm")
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,4u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::is_from(void *)")]
// 0xf5ddd4 — j___ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj4ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv
pub fn stub_0xf5ddd4() -> ! {
    todo!("0xf5ddd4 j___ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj4ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv")
}

#[doc(alias = "boost::detail::weak_count::operator=(boost::detail::weak_count const&)")]
// 0xf5de04 — j___ZN5boost6detail10weak_countaSERKS1_
pub fn stub_0xf5de04() -> ! {
    todo!("0xf5de04 j___ZN5boost6detail10weak_countaSERKS1_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxPart *> *,boost::unordered::detail::ptr_node<RBX::GfxPart *> *)")]
// 0xf5de94 — j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11erase_nodesEPNS1_8ptr_nodeIS7_EESJ_
pub fn stub_0xf5de94() -> ! {
    todo!("0xf5de94 j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11erase_nodesEPNS1_8ptr_nodeIS7_EESJ_")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxPart *>>(RBX::GfxPart * const&,boost::unordered::detail::emplace_args1<RBX::GfxPart *> const&)")]
// 0xf5dea4 — j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
pub fn stub_0xf5dea4() -> ! {
    todo!("0xf5dea4 j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_")
}

#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::fill_buckets<boost::unordered::detail::copy_nodes<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> &,boost::unordered::detail::copy_nodes<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &)")]
// 0xf5deb4 — j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_10copy_nodesINS4_INS1_8ptr_nodeIS7_EES8_S9_Lj32ELj0EEEEEEEvNS0_15iterator_detail8iteratorISK_EERNS1_5tableISF_EERT_
pub fn stub_0xf5deb4() -> ! {
    todo!("0xf5deb4 j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_10copy_nodesINS4_INS1_8ptr_nodeIS7_EES8_S9_Lj32ELj0EEEEEEEvNS0_15iterator_detail8iteratorISK_EERNS1_5tableISF_EERT_")
}

#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>> &)")]
// 0xf5dec4 — j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISF_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEERSK_RT_
pub fn stub_0xf5dec4() -> ! {
    todo!("0xf5dec4 j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISF_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEERSK_RT_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::erase_key(RBX::GfxPart * const&)")]
// 0xf5ded4 — j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE9erase_keyERKS7_
pub fn stub_0xf5ded4() -> ! {
    todo!("0xf5ded4 j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE9erase_keyERKS7_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *,boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *)")]
// 0xf5dee4 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_
pub fn stub_0xf5dee4() -> ! {
    todo!("0xf5dee4 j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxAttachment *>>(RBX::GfxAttachment * const&,boost::unordered::detail::emplace_args1<RBX::GfxAttachment *> const&)")]
// 0xf5def4 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
pub fn stub_0xf5def4() -> ! {
    todo!("0xf5def4 j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_")
}

#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>> &)")]
// 0xf5df04 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeIS6_EEEEEEEvNS0_15iterator_detail8iteratorISH_EERNS1_5tableISC_EERT_
pub fn stub_0xf5df04() -> ! {
    todo!("0xf5df04 j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeIS6_EEEEEEEvNS0_15iterator_detail8iteratorISH_EERNS1_5tableISC_EERT_")
}

#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>> &)")]
// 0xf5df14 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISC_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEERSH_RT_
pub fn stub_0xf5df14() -> ! {
    todo!("0xf5df14 j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISC_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEERSH_RT_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::table_impl(boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)")]
// 0xf5df24 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2ERKSD_
pub fn stub_0xf5df24() -> ! {
    todo!("0xf5df24 j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2ERKSD_")
}

#[doc(alias = "boost::unordered::detail::node_holder<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::~node_holder()")]
// 0xf5df34 — j___ZN5boost9unordered6detail11node_holderINS_19fast_pool_allocatorINS1_8ptr_nodeIPN3RBX7GfxPartEEENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED2Ev
pub fn stub_0xf5df34() -> ! {
    todo!("0xf5df34 j___ZN5boost9unordered6detail11node_holderINS_19fast_pool_allocatorINS1_8ptr_nodeIPN3RBX7GfxPartEEENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED2Ev")
}

#[doc(alias = "boost::unordered::detail::array_constructor<boost::fast_pool_allocator<boost::unordered::detail::ptr_bucket,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::~array_constructor()")]
// 0xf5df84 — j___ZN5boost9unordered6detail17array_constructorINS_19fast_pool_allocatorINS1_10ptr_bucketENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED2Ev
pub fn stub_0xf5df84() -> ! {
    todo!("0xf5df84 j___ZN5boost9unordered6detail17array_constructorINS_19fast_pool_allocatorINS1_10ptr_bucketENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::create_buckets(unsigned long)")]
// 0xf5e034 — j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
pub fn stub_0xf5e034() -> ! {
    todo!("0xf5e034 j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::destroy_buckets(void)")]
// 0xf5e044 — j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15destroy_bucketsEv
pub fn stub_0xf5e044() -> ! {
    todo!("0xf5e044 j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15destroy_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::reserve_for_insert(unsigned long)")]
// 0xf5e054 — j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
pub fn stub_0xf5e054() -> ! {
    todo!("0xf5e054 j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> const&)")]
// 0xf5e064 — j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE4initERKSG_
pub fn stub_0xf5e064() -> ! {
    todo!("0xf5e064 j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE4initERKSG_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")]
// 0xf5e074 — j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE6assignERKSG_NS1_17integral_constantIbLb0EEE
pub fn stub_0xf5e074() -> ! {
    todo!("0xf5e074 j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE6assignERKSG_NS1_17integral_constantIbLb0EEE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::~table()")]
// 0xf5e084 — j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEED2Ev
pub fn stub_0xf5e084() -> ! {
    todo!("0xf5e084 j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEED2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::create_buckets(unsigned long)")]
// 0xf5e094 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
pub fn stub_0xf5e094() -> ! {
    todo!("0xf5e094 j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::reserve_for_insert(unsigned long)")]
// 0xf5e0a4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
pub fn stub_0xf5e0a4() -> ! {
    todo!("0xf5e0a4 j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)")]
// 0xf5e0b4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE4initERKSD_
pub fn stub_0xf5e0b4() -> ! {
    todo!("0xf5e0b4 j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE4initERKSD_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")]
// 0xf5e0c4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE6assignERKSD_NS1_17integral_constantIbLb0EEE
pub fn stub_0xf5e0c4() -> ! {
    todo!("0xf5e0c4 j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE6assignERKSD_NS1_17integral_constantIbLb0EEE")
}

#[doc(alias = "void std::__iter_swap<true>::iter_swap<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0xf5e0f4 — j___ZNSt11__iter_swapILb1EE9iter_swapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS5_SaIS5_EEEESA_EEvT_T0_
pub fn stub_0xf5e0f4() -> ! {
    todo!("0xf5e0f4 j___ZNSt11__iter_swapILb1EE9iter_swapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS5_SaIS5_EEEESA_EEvT_T0_")
}

#[doc(alias = "std::vector<RBX::SceneUpdater::MegaClusterChunk,std::allocator<RBX::SceneUpdater::MegaClusterChunk>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SceneUpdater::MegaClusterChunk*,std::vector<RBX::SceneUpdater::MegaClusterChunk,std::allocator<RBX::SceneUpdater::MegaClusterChunk>>>,RBX::SceneUpdater::MegaClusterChunk const&)")]
// 0xf5e104 — j___ZNSt6vectorIN3RBX12SceneUpdater16MegaClusterChunkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf5e104() -> ! {
    todo!("0xf5e104 j___ZNSt6vectorIN3RBX12SceneUpdater16MegaClusterChunkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>::erase(__gnu_cxx::__normal_iterator<RBX::ContentId*,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId*,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0xf5e114 — j___ZNSt6vectorIN3RBX9ContentIdESaIS1_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS1_S3_EES7_
pub fn stub_0xf5e114() -> ! {
    todo!("0xf5e114 j___ZNSt6vectorIN3RBX9ContentIdESaIS1_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS1_S3_EES7_")
}

#[doc(alias = "std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>::vector(std::vector<RBX::ContentId,std::allocator<RBX::ContentId>> const&)")]
// 0xf5e124 — j___ZNSt6vectorIN3RBX9ContentIdESaIS1_EEC2ERKS3_
pub fn stub_0xf5e124() -> ! {
    todo!("0xf5e124 j___ZNSt6vectorIN3RBX9ContentIdESaIS1_EEC2ERKS3_")
}

#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::clear(void)")]
// 0xf5e134 — j___ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE5clearEv
pub fn stub_0xf5e134() -> ! {
    todo!("0xf5e134 j___ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE5clearEv")
}

#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::~vector()")]
// 0xf5e144 — j___ZNSt6vectorIN3rbx7signals10connectionESaIS2_EED1Ev
pub fn stub_0xf5e144() -> ! {
    todo!("0xf5e144 j___ZNSt6vectorIN3rbx7signals10connectionESaIS2_EED1Ev")
}

#[doc(alias = "std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxAttachment **,std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>>,RBX::GfxAttachment * const&)")]
// 0xf5e184 — j___ZNSt6vectorIPN3RBX13GfxAttachmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf5e184() -> ! {
    todo!("0xf5e184 j___ZNSt6vectorIPN3RBX13GfxAttachmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::GfxPart *,std::allocator<RBX::GfxPart *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxPart **,std::vector<RBX::GfxPart *,std::allocator<RBX::GfxPart *>>>,RBX::GfxPart * const&)")]
// 0xf5e194 — j___ZNSt6vectorIPN3RBX7GfxPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf5e194() -> ! {
    todo!("0xf5e194 j___ZNSt6vectorIPN3RBX7GfxPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::_Rb_tree_iterator<std::pair<RBX::GfxPart * const,RBX::ContentId>>)")]
// 0xf5e1a4 — j___ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE5eraseESt17_Rb_tree_iteratorIS6_ESE_
pub fn stub_0xf5e1a4() -> ! {
    todo!("0xf5e1a4 j___ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE5eraseESt17_Rb_tree_iteratorIS6_ESE_")
}

