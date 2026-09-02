//! core shard FY — 100 core stubs EA-sorted, 0xf41c54..0xf432f4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf41c34).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf41c34.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId)>::connect<boost::function<void ()(RBX::NormalId)>>(boost::function<void ()(RBX::NormalId)> const&)")]
// 0xf41c54 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
pub fn stub_f41c54() -> ! {
    todo!("0xf41c54 j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::on_error(std::exception &)")]
// 0xf41c64 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception
pub fn stub_f41c64() -> ! {
    todo!("0xf41c64 j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::disconnectAll(void)")]
// 0xf41c74 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv
pub fn stub_f41c74() -> ! {
    todo!("0xf41c74 j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::safe_static_do_get_mutex(void)")]
// 0xf41c84 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv
pub fn stub_f41c84() -> ! {
    todo!("0xf41c84 j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> &)")]
// 0xf41c94 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// was: rbx::signals::signal<void ()(RBX::NormalId,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> &)
pub fn stub_f41c94() -> ! {
    todo!("0xf41c94 j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::safe_static_do_get_mutex(void)")]
// 0xf41ca4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot24safe_static_do_get_mutexEv
pub fn stub_f41ca4() -> ! {
    todo!("0xf41ca4 j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::insert(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot *)")]
// 0xf41cb4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6insertEPNS5_4slotE
pub fn stub_f41cb4() -> ! {
    todo!("0xf41cb4 j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6insertEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::remove(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot *)")]
// 0xf41cc4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6removeEPNS5_4slotE
pub fn stub_f41cc4() -> ! {
    todo!("0xf41cc4 j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6removeEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<boost::function<void ()(RBX::NormalId,float)>>(boost::function<void ()(RBX::NormalId,float)> const&)")]
// 0xf41ce4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
pub fn stub_f41ce4() -> ! {
    todo!("0xf41ce4 j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::on_error(std::exception &)")]
// 0xf41cf4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception
pub fn stub_f41cf4() -> ! {
    todo!("0xf41cf4 j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::callable<rbx::signals::signal<void ()(RBX::NormalId)>*>(boost::function<void ()(RBX::NormalId)> const&,rbx::signals::signal<void ()(RBX::NormalId)>*)")]
// 0xf41d24 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
pub fn stub_f41d24() -> ! {
    todo!("0xf41d24 j___ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>*>(boost::function<void ()(RBX::NormalId,float)> const&,rbx::signals::signal<void ()(RBX::NormalId,float)>*)")]
// 0xf41d34 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_
pub fn stub_f41d34() -> ! {
    todo!("0xf41d34 j___ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId)>::slot*)")]
// 0xf41d54 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSEPS8_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId)>::slot*)
pub fn stub_f41d54() -> ! {
    todo!("0xf41d54 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSEPS8_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> const&)")]
// 0xf41d64 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSERKS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> const&)
pub fn stub_f41d64() -> ! {
    todo!("0xf41d64 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot*)")]
// 0xf41d74 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSEPS8_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot*)
pub fn stub_f41d74() -> ! {
    todo!("0xf41d74 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSEPS8_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> const&)")]
// 0xf41d84 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSERKS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> const&)
pub fn stub_f41d84() -> ! {
    todo!("0xf41d84 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSERKS9_")
}

#[doc(alias = "boost::function1<void,RBX::NormalId>::assign_to_own(boost::function1<void,RBX::NormalId> const&)")]
// 0xf41e64 — j___ZN5boost9function1IvN3RBX8NormalIdEE13assign_to_ownERKS3_
pub fn stub_f41e64() -> ! {
    todo!("0xf41e64 j___ZN5boost9function1IvN3RBX8NormalIdEE13assign_to_ownERKS3_")
}

#[doc(alias = "boost::function1<void,RBX::NormalId>::clear(void)")]
// 0xf41e74 — j___ZN5boost9function1IvN3RBX8NormalIdEE5clearEv
pub fn stub_f41e74() -> ! {
    todo!("0xf41e74 j___ZN5boost9function1IvN3RBX8NormalIdEE5clearEv")
}

#[doc(alias = "boost::function2<void,RBX::NormalId,float>::assign_to_own(boost::function2<void,RBX::NormalId,float> const&)")]
// 0xf41ea4 — j___ZN5boost9function2IvN3RBX8NormalIdEfE13assign_to_ownERKS3_
pub fn stub_f41ea4() -> ! {
    todo!("0xf41ea4 j___ZN5boost9function2IvN3RBX8NormalIdEfE13assign_to_ownERKS3_")
}

#[doc(alias = "boost::function2<void,RBX::NormalId,float>::clear(void)")]
// 0xf41eb4 — j___ZN5boost9function2IvN3RBX8NormalIdEfE5clearEv
pub fn stub_f41eb4() -> ! {
    todo!("0xf41eb4 j___ZN5boost9function2IvN3RBX8NormalIdEfE5clearEv")
}

#[doc(alias = "boost::function1<void,RBX::NormalId>::operator()(RBX::NormalId)const")]
// 0xf41f84 — j___ZNK5boost9function1IvN3RBX8NormalIdEEclES2_
pub fn stub_f41f84() -> ! {
    todo!("0xf41f84 j___ZNK5boost9function1IvN3RBX8NormalIdEEclES2_")
}

#[doc(alias = "boost::function2<void,RBX::NormalId,float>::operator()(RBX::NormalId,float)const")]
// 0xf41f94 — j___ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f
pub fn stub_f41f94() -> ! {
    todo!("0xf41f94 j___ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f")
}

#[doc(alias = "RBX::RelativePanel::RelativePanel(void)")]
// 0xf42044 — j___ZN3RBX13RelativePanelC2Ev
pub fn stub_f42044() -> ! {
    todo!("0xf42044 j___ZN3RBX13RelativePanelC2Ev")
}

#[doc(alias = "RBX::HopperBin::~HopperBin()")]
// 0xf42114 — j___ZN3RBX9HopperBinD2Ev
pub fn stub_f42114() -> ! {
    todo!("0xf42114 j___ZN3RBX9HopperBinD2Ev")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>> const&)")]
// 0xf42154 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_
pub fn stub_f42154() -> ! {
    todo!("0xf42154 j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>::operator()(void)")]
// 0xf42194 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9HopperBinEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
pub fn stub_f42194() -> ! {
    todo!("0xf42194 j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9HopperBinEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")
}

#[doc(alias = "std::_Vector_base<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_allocate(unsigned long)")]
// 0xf42244 — j___ZNSt12_Vector_baseIN3RBX9HopperBin7BinTypeESaIS2_EE11_M_allocateEm
pub fn stub_f42244() -> ! {
    todo!("0xf42244 j___ZNSt12_Vector_baseIN3RBX9HopperBin7BinTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::HopperBin::BinType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HopperBin::BinType *,RBX::HopperBin::BinType *>(RBX::HopperBin::BinType *,RBX::HopperBin::BinType *,RBX::HopperBin::BinType *)")]
// 0xf42254 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9HopperBin7BinTypeES6_EET0_T_S8_S7_
pub fn stub_f42254() -> ! {
    todo!("0xf42254 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9HopperBin7BinTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::HopperBin::BinType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::operator[](RBX::Name const* const&)")]
// 0xf42264 — j___ZNSt3mapIPKN3RBX4NameENS0_9HopperBin7BinTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f42264() -> ! {
    todo!("0xf42264 j___ZNSt3mapIPKN3RBX4NameENS0_9HopperBin7BinTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,RBX::HopperBin::BinType const&)")]
// 0xf42274 — j___ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f42274() -> ! {
    todo!("0xf42274 j___ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,unsigned long,RBX::HopperBin::BinType const&)")]
// 0xf42284 — j___ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f42284() -> ! {
    todo!("0xf42284 j___ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::resize(unsigned long,RBX::HopperBin::BinType)")]
// 0xf42294 — j___ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE6resizeEmS2_
pub fn stub_f42294() -> ! {
    todo!("0xf42294 j___ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::push_back(RBX::HopperBin::BinType const&)")]
// 0xf422a4 — j___ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE9push_backERKS2_
pub fn stub_f422a4() -> ! {
    todo!("0xf422a4 j___ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
// 0xf422b4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f422b4() -> ! {
    todo!("0xf422b4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
// 0xf422c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f422c4() -> ! {
    todo!("0xf422c4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
// 0xf422d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f422d4() -> ! {
    todo!("0xf422d4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Weld>::operator=(rbx_core::SharedPtr<RBX::Weld> const&)")]
// 0xf422e4 — j___ZN5boost10shared_ptrIN3RBX4WeldEEaSERKS3_
// was: boost::shared_ptr<RBX::Weld>::operator=(boost::shared_ptr<RBX::Weld> const&)
pub fn stub_f422e4() -> ! {
    todo!("0xf422e4 j___ZN5boost10shared_ptrIN3RBX4WeldEEaSERKS3_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextureId>(RBX::TextureId const&)")]
// 0xf423c4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9TextureIdEEERS3_RKT_
pub fn stub_f423c4() -> ! {
    todo!("0xf423c4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9TextureIdEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TextureId>::singleton(void)")]
// 0xf423d4 — j___ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE9singletonEv
pub fn stub_f423d4() -> ! {
    todo!("0xf423d4 j___ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE9singletonEv")
}

#[doc(alias = "RBX::TextureId const& rbx::any_cast<RBX::TextureId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf423e4 — j___ZN3rbx8any_castIRKN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f423e4() -> ! {
    todo!("0xf423e4 j___ZN3rbx8any_castIRKN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::InsertService::~InsertService()")]
// 0xf425d4 — j___ZN3RBX13InsertServiceD2Ev
pub fn stub_f425d4() -> ! {
    todo!("0xf425d4 j___ZN3RBX13InsertServiceD2Ev")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::InsertService> RBX::weak_from<RBX::InsertService>(RBX::InsertService*)")]
// 0xf42604 — j___ZN3RBX9weak_fromINS_13InsertServiceEEEN5boost8weak_ptrIT_EEPS4_
// was: boost::weak_ptr<RBX::InsertService> RBX::weak_from<RBX::InsertService>(RBX::InsertService*)
pub fn stub_f42604() -> ! {
    todo!("0xf42604 j___ZN3RBX9weak_fromINS_13InsertServiceEEEN5boost8weak_ptrIT_EEPS4_")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,RBX::ContentId)>::remote_signal(void)")]
// 0xf42614 — j___ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEEC2Ev
pub fn stub_f42614() -> ! {
    todo!("0xf42614 j___ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,RBX::ContentId)>::~remote_signal()")]
// 0xf42624 — j___ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEED2Ev
pub fn stub_f42624() -> ! {
    todo!("0xf42624 j___ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEED2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string)>::remote_signal(void)")]
// 0xf42654 — j___ZN3rbx13remote_signalIFvSsSsEEC2Ev
pub fn stub_f42654() -> ! {
    todo!("0xf42654 j___ZN3rbx13remote_signalIFvSsSsEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string)>::~remote_signal()")]
// 0xf42664 — j___ZN3rbx13remote_signalIFvSsSsEED2Ev
pub fn stub_f42664() -> ! {
    todo!("0xf42664 j___ZN3rbx13remote_signalIFvSsSsEED2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,int,int)>::remote_signal(void)")]
// 0xf42674 — j___ZN3rbx13remote_signalIFvSsiiEEC2Ev
pub fn stub_f42674() -> ! {
    todo!("0xf42674 j___ZN3rbx13remote_signalIFvSsiiEEC2Ev")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,RBX::ContentId)>::fireItem(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *,std::string,RBX::ContentId)")]
// 0xf42684 — j___ZN3rbx7signals16signal_with_argsILi2EFvSsN3RBX9ContentIdEEE8fireItemEPNS0_6signalIS4_E4slotESsS3_
pub fn stub_f42684() -> ! {
    todo!("0xf42684 j___ZN3rbx7signals16signal_with_argsILi2EFvSsN3RBX9ContentIdEEE8fireItemEPNS0_6signalIS4_E4slotESsS3_")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,RBX::ContentId)>::operator()(std::string,RBX::ContentId)")]
// 0xf42694 — j___ZN3rbx7signals16signal_with_argsILi2EFvSsN3RBX9ContentIdEEEclESsS3_
pub fn stub_f42694() -> ! {
    todo!("0xf42694 j___ZN3rbx7signals16signal_with_argsILi2EFvSsN3RBX9ContentIdEEEclESsS3_")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,int,int)>::operator()(std::string,int,int)")]
// 0xf426a4 — j___ZN3rbx7signals16signal_with_argsILi3EFvSsiiEEclESsii
pub fn stub_f426a4() -> ! {
    todo!("0xf426a4 j___ZN3rbx7signals16signal_with_argsILi3EFvSsiiEEclESsii")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::disconnectAll(void)")]
// 0xf426b4 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13disconnectAllEv
pub fn stub_f426b4() -> ! {
    todo!("0xf426b4 j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::safe_static_do_get_mutex(void)")]
// 0xf426c4 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE24safe_static_do_get_mutexEv
pub fn stub_f426c4() -> ! {
    todo!("0xf426c4 j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot> &)")]
// 0xf426d4 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// was: rbx::signals::signal<void ()(std::string,RBX::ContentId)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot> &)
pub fn stub_f426d4() -> ! {
    todo!("0xf426d4 j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::safe_static_do_get_mutex(void)")]
// 0xf426e4 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f426e4() -> ! {
    todo!("0xf426e4 j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::insert(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *)")]
// 0xf426f4 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6insertEPNS5_4slotE
pub fn stub_f426f4() -> ! {
    todo!("0xf426f4 j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6insertEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::remove(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *)")]
// 0xf42704 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6removeEPNS5_4slotE
pub fn stub_f42704() -> ! {
    todo!("0xf42704 j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6removeEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,RBX::ContentId)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0xf42714 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_f42714() -> ! {
    todo!("0xf42714 j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,RBX::ContentId)>::connect<boost::function<void ()(std::string,RBX::ContentId)>>(boost::function<void ()(std::string,RBX::ContentId)> const&)")]
// 0xf42724 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
pub fn stub_f42724() -> ! {
    todo!("0xf42724 j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::on_error(std::exception &)")]
// 0xf42734 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE8on_errorERSt9exception
pub fn stub_f42734() -> ! {
    todo!("0xf42734 j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0xf42784 — j___ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_f42784() -> ! {
    todo!("0xf42784 j___ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::safe_static_do_get_mutex(void)")]
// 0xf42794 — j___ZN3rbx7signals6signalIFvSsiiEE24safe_static_do_get_mutexEv
pub fn stub_f42794() -> ! {
    todo!("0xf42794 j___ZN3rbx7signals6signalIFvSsiiEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot> &)")]
// 0xf427a4 — j___ZN3rbx7signals6signalIFvSsiiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(std::string,int,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot> &)
pub fn stub_f427a4() -> ! {
    todo!("0xf427a4 j___ZN3rbx7signals6signalIFvSsiiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::safe_static_do_get_mutex(void)")]
// 0xf427b4 — j___ZN3rbx7signals6signalIFvSsiiEE4slot24safe_static_do_get_mutexEv
pub fn stub_f427b4() -> ! {
    todo!("0xf427b4 j___ZN3rbx7signals6signalIFvSsiiEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
// 0xf427c4 — j___ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_
pub fn stub_f427c4() -> ! {
    todo!("0xf427c4 j___ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,int,int)>::connect<boost::function<void ()(std::string,int,int)>>(boost::function<void ()(std::string,int,int)> const&)")]
// 0xf427d4 — j___ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_f427d4() -> ! {
    todo!("0xf427d4 j___ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::on_error(std::exception &)")]
// 0xf427e4 — j___ZN3rbx7signals6signalIFvSsiiEE8on_errorERSt9exception
pub fn stub_f427e4() -> ! {
    todo!("0xf427e4 j___ZN3rbx7signals6signalIFvSsiiEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
// 0xf427f4 — j___ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_
pub fn stub_f427f4() -> ! {
    todo!("0xf427f4 j___ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>*>(boost::function<void ()(std::string,RBX::ContentId)> const&,rbx::signals::signal<void ()(std::string,RBX::ContentId)>*)")]
// 0xf42804 — j___ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_
pub fn stub_f42804() -> ! {
    todo!("0xf42804 j___ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::InsertService>::shared_ptr<RBX::InsertService>(rbx_core::WeakPtr<RBX::InsertService> const&,boost::detail::sp_nothrow_tag)")]
// 0xf42824 — j___ZN5boost10shared_ptrIN3RBX13InsertServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::InsertService>::shared_ptr<RBX::InsertService>(boost::weak_ptr<RBX::InsertService> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f42824() -> ! {
    todo!("0xf42824 j___ZN5boost10shared_ptrIN3RBX13InsertServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot*)")]
// 0xf42864 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSEPS8_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot*)
pub fn stub_f42864() -> ! {
    todo!("0xf42864 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSEPS8_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot> const&)")]
// 0xf42874 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSERKS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot> const&)
pub fn stub_f42874() -> ! {
    todo!("0xf42874 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(rbx::signals::signal<void ()(std::string,int,int)>::slot*)")]
// 0xf42894 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(rbx::signals::signal<void ()(std::string,int,int)>::slot*)
pub fn stub_f42894() -> ! {
    todo!("0xf42894 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot> const&)")]
// 0xf428a4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot> const&)
pub fn stub_f428a4() -> ! {
    todo!("0xf428a4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSERKS7_")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)")]
// 0xf428e4 — j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_
// was: boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::list3(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)
pub fn stub_f428e4() -> ! {
    todo!("0xf428e4 j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list2<std::string &,RBX::ContentId&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId> &,boost::_bi::list2<std::string &,RBX::ContentId&> &,int)")]
// 0xf42904 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS3_9ContentIdEEENS0_5list2IRSsRSE_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f42904() -> ! {
    todo!("0xf42904 j___ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS3_9ContentIdEEENS0_5list2IRSsRSE_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list2<std::string &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string> &,boost::_bi::list2<std::string &,std::string &> &,int)")]
// 0xf42924 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsSsEENS0_5list2IRSsSG_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f42924() -> ! {
    todo!("0xf42924 j___ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsSsEENS0_5list2IRSsSG_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list3<std::string &,int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int> &,boost::_bi::list3<std::string &,int &,int &> &,int)")]
// 0xf42974 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_SsiiEENS0_5list3IRSsRiSI_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f42974() -> ! {
    todo!("0xf42974 j___ZN5boost3_bi5list4INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_SsiiEENS0_5list3IRSsRiSI_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>)")]
// 0xf429e4 — j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEEEC2ES7_S8_
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>)
pub fn stub_f429e4() -> ! {
    todo!("0xf429e4 j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEEEC2ES7_S8_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>)")]
// 0xf429f4 — j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEEEC2ES7_S9_
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>)
pub fn stub_f429f4() -> ! {
    todo!("0xf429f4 j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEEEC2ES7_S9_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)")]
// 0xf42a14 — j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)
pub fn stub_f42a14() -> ! {
    todo!("0xf42a14 j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>)")]
// 0xf42a24 — j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>)
pub fn stub_f42a24() -> ! {
    todo!("0xf42a24 j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_")
}

#[doc(alias = "boost::function2<void,std::string,RBX::ContentId>::assign_to_own(boost::function2<void,std::string,RBX::ContentId> const&)")]
// 0xf42d84 — j___ZN5boost9function2IvSsN3RBX9ContentIdEE13assign_to_ownERKS3_
pub fn stub_f42d84() -> ! {
    todo!("0xf42d84 j___ZN5boost9function2IvSsN3RBX9ContentIdEE13assign_to_ownERKS3_")
}

#[doc(alias = "boost::function2<void,std::string,RBX::ContentId>::clear(void)")]
// 0xf42d94 — j___ZN5boost9function2IvSsN3RBX9ContentIdEE5clearEv
pub fn stub_f42d94() -> ! {
    todo!("0xf42d94 j___ZN5boost9function2IvSsN3RBX9ContentIdEE5clearEv")
}

#[doc(alias = "boost::function3<void,std::string,int,int>::assign_to_own(boost::function3<void,std::string,int,int> const&)")]
// 0xf42df4 — j___ZN5boost9function3IvSsiiE13assign_to_ownERKS1_
pub fn stub_f42df4() -> ! {
    todo!("0xf42df4 j___ZN5boost9function3IvSsiiE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>::operator()(RBX::InsertService*,std::string,RBX::ContentId)const")]
// 0xf42e54 — j___ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS2_9ContentIdEEclEPS3_SsS4_
pub fn stub_f42e54() -> ! {
    todo!("0xf42e54 j___ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS2_9ContentIdEEclEPS3_SsS4_")
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>::operator()(RBX::InsertService*,std::string,std::string)const")]
// 0xf42e74 — j___ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsSsEclEPS3_SsSs
pub fn stub_f42e74() -> ! {
    todo!("0xf42e74 j___ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsSsEclEPS3_SsSs")
}

#[doc(alias = "boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>::operator()(RBX::InsertService*,std::string,int,int)const")]
// 0xf42e84 — j___ZNK5boost4_mfi3mf3IvN3RBX13InsertServiceESsiiEclEPS3_Ssii
pub fn stub_f42e84() -> ! {
    todo!("0xf42e84 j___ZNK5boost4_mfi3mf3IvN3RBX13InsertServiceESsiiEclEPS3_Ssii")
}

#[doc(alias = "boost::function2<void,std::string,RBX::ContentId>::operator()(std::string,RBX::ContentId)const")]
// 0xf43034 — j___ZNK5boost9function2IvSsN3RBX9ContentIdEEclESsS2_
pub fn stub_f43034() -> ! {
    todo!("0xf43034 j___ZNK5boost9function2IvSsN3RBX9ContentIdEEclESsS2_")
}

#[doc(alias = "boost::function3<void,std::string,int,int>::operator()(std::string,int,int)const")]
// 0xf43044 — j___ZNK5boost9function3IvSsiiEclESsii
pub fn stub_f43044() -> ! {
    todo!("0xf43044 j___ZNK5boost9function3IvSsiiEclESsii")
}

#[doc(alias = "std::map<std::string,RBX::InsertService::Callback,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::operator[](std::string const&)")]
// 0xf43054 — j___ZNSt3mapISsN3RBX13InsertService8CallbackESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
pub fn stub_f43054() -> ! {
    todo!("0xf43054 j___ZNSt3mapISsN3RBX13InsertService8CallbackESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")
}

#[doc(alias = "std::pair<std::string const,RBX::InsertService::Callback>::pair(std::string const&,RBX::InsertService::Callback const&)")]
// 0xf43064 — j___ZNSt4pairIKSsN3RBX13InsertService8CallbackEEC2ERS0_RKS3_
pub fn stub_f43064() -> ! {
    todo!("0xf43064 j___ZNSt4pairIKSsN3RBX13InsertService8CallbackEEC2ERS0_RKS3_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::lower_bound(std::string const&)")]
// 0xf43074 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
pub fn stub_f43074() -> ! {
    todo!("0xf43074 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_create_node(std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// 0xf43084 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_f43084() -> ! {
    todo!("0xf43084 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::InsertService::Callback>> *)")]
// 0xf43094 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
pub fn stub_f43094() -> ! {
    todo!("0xf43094 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// 0xf430a4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_f430a4() -> ! {
    todo!("0xf430a4 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::InsertService::Callback>>,std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// 0xf430b4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_f430b4() -> ! {
    todo!("0xf430b4 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::find(std::string const&)")]
// 0xf430c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
pub fn stub_f430c4() -> ! {
    todo!("0xf430c4 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::InsertService::Callback>> *)")]
// 0xf430d4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_f430d4() -> ! {
    todo!("0xf430d4 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// 0xf430e4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_f430e4() -> ! {
    todo!("0xf430e4 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "RBX::ManualGlueJoint::~ManualGlueJoint()")]
// 0xf432f4 — j___ZN3RBX15ManualGlueJointD0Ev
pub fn stub_f432f4() -> ! {
    todo!("0xf432f4 j___ZN3RBX15ManualGlueJointD0Ev")
}

