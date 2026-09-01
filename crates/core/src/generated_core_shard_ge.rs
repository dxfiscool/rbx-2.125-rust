//! core shard GE — 100 core stubs EA-sorted, 0xf49a54..0xf4b524 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf49a44).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf49a44.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "RBX::Controller::Button const& rbx::any_cast<RBX::Controller::Button const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf49a54 — j___ZN3rbx8any_castIRKN3RBX10Controller6ButtonENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f49a54() -> ! {
    todo!("0xf49a54 j___ZN3rbx8any_castIRKN3RBX10Controller6ButtonENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Controller::Button & rbx::any_cast<RBX::Controller::Button &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf49a64 — j___ZN3rbx8any_castIRN3RBX10Controller6ButtonENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f49a64() -> ! {
    todo!("0xf49a64 j___ZN3rbx8any_castIRN3RBX10Controller6ButtonENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>*>(boost::function<void ()(RBX::Controller::Button)> const&,rbx::signals::signal<void ()(RBX::Controller::Button)>*)")]
// 0xf49a74 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
pub fn stub_f49a74() -> ! {
    todo!("0xf49a74 j___ZN3rbx8callableINS_7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Controller>::shared_ptr<RBX::Controller>(rbx_core::WeakPtr<RBX::Controller> const&,boost::detail::sp_nothrow_tag)")]
// 0xf49a84 — j___ZN5boost10shared_ptrIN3RBX10ControllerEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::Controller>::shared_ptr<RBX::Controller>(boost::weak_ptr<RBX::Controller> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f49a84() -> ! {
    todo!("0xf49a84 j___ZN5boost10shared_ptrIN3RBX10ControllerEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VehicleSeat>::shared_ptr<RBX::VehicleSeat>(rbx_core::WeakPtr<RBX::VehicleSeat> const&,boost::detail::sp_nothrow_tag)")]
// 0xf49a94 — j___ZN5boost10shared_ptrIN3RBX11VehicleSeatEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::VehicleSeat>::shared_ptr<RBX::VehicleSeat>(boost::weak_ptr<RBX::VehicleSeat> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f49a94() -> ! {
    todo!("0xf49a94 j___ZN5boost10shared_ptrIN3RBX11VehicleSeatEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ButtonBindingWidget>::operator=(rbx_core::SharedPtr<RBX::ButtonBindingWidget> const&)")]
// 0xf49ae4 — j___ZN5boost10shared_ptrIN3RBX19ButtonBindingWidgetEEaSERKS3_
// was: boost::shared_ptr<RBX::ButtonBindingWidget>::operator=(boost::shared_ptr<RBX::ButtonBindingWidget> const&)
pub fn stub_f49ae4() -> ! {
    todo!("0xf49ae4 j___ZN5boost10shared_ptrIN3RBX19ButtonBindingWidgetEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot*)")]
// 0xf49af4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot*)
pub fn stub_f49af4() -> ! {
    todo!("0xf49af4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> const&)")]
// 0xf49b04 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> const&)
pub fn stub_f49b04() -> ! {
    todo!("0xf49b04 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEEaSERKSA_")
}

#[doc(alias = "boost::system::system_error::~system_error()")]
// 0xf49b74 — j___ZN5boost6system12system_errorD2Ev
pub fn stub_f49b74() -> ! {
    todo!("0xf49b74 j___ZN5boost6system12system_errorD2Ev")
}

#[doc(alias = "boost::function1<void,RBX::Controller::Button>::assign_to_own(boost::function1<void,RBX::Controller::Button> const&)")]
// 0xf49b94 — j___ZN5boost9function1IvN3RBX10Controller6ButtonEE13assign_to_ownERKS4_
pub fn stub_f49b94() -> ! {
    todo!("0xf49b94 j___ZN5boost9function1IvN3RBX10Controller6ButtonEE13assign_to_ownERKS4_")
}

#[doc(alias = "boost::function1<void,RBX::Controller::Button>::clear(void)")]
// 0xf49ba4 — j___ZN5boost9function1IvN3RBX10Controller6ButtonEE5clearEv
pub fn stub_f49ba4() -> ! {
    todo!("0xf49ba4 j___ZN5boost9function1IvN3RBX10Controller6ButtonEE5clearEv")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> *)")]
// 0xf49bd4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_
pub fn stub_f49bd4() -> ! {
    todo!("0xf49bd4 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::rehash_impl(unsigned long)")]
// 0xf49be4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
pub fn stub_f49be4() -> ! {
    todo!("0xf49be4 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>(RBX::Controller::Button const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> const&)")]
// 0xf49bf4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS8_RKT_
pub fn stub_f49bf4() -> ! {
    todo!("0xf49bf4 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS8_RKT_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0xf49c04 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
pub fn stub_f49c04() -> ! {
    todo!("0xf49c04 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>>::construct(void)")]
// 0xf49c14 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEEEEE9constructEv
pub fn stub_f49c14() -> ! {
    todo!("0xf49c14 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>>::~node_constructor()")]
// 0xf49c24 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEEEEED2Ev
pub fn stub_f49c24() -> ! {
    todo!("0xf49c24 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEEEEED2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// 0xf49c34 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE10fix_bucketEmPNS1_10ptr_bucketE
pub fn stub_f49c34() -> ! {
    todo!("0xf49c34 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE10fix_bucketEmPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0xf49c44 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_f49c44() -> ! {
    todo!("0xf49c44 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::create_buckets(unsigned long)")]
// 0xf49c54 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
pub fn stub_f49c54() -> ! {
    todo!("0xf49c54 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::delete_buckets(void)")]
// 0xf49c64 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
pub fn stub_f49c64() -> ! {
    todo!("0xf49c64 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::reserve_for_insert(unsigned long)")]
// 0xf49c74 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
pub fn stub_f49c74() -> ! {
    todo!("0xf49c74 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::table(unsigned long,boost::hash<RBX::Controller::Button> const&,std::equal_to<RBX::Controller::Button> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>> const&)")]
// 0xf49c84 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
pub fn stub_f49c84() -> ! {
    todo!("0xf49c84 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE")
}

#[doc(alias = "boost::function1<void,RBX::Controller::Button>::operator()(RBX::Controller::Button)const")]
// 0xf49d64 — j___ZNK5boost9function1IvN3RBX10Controller6ButtonEEclES3_
pub fn stub_f49d64() -> ! {
    todo!("0xf49d64 j___ZNK5boost9function1IvN3RBX10Controller6ButtonEEclES3_")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::find_node_impl<RBX::Controller::Button,std::equal_to<RBX::Controller::Button>>(unsigned long,RBX::Controller::Button const&,std::equal_to<RBX::Controller::Button> const&)const")]
// 0xf49d74 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_
pub fn stub_f49d74() -> ! {
    todo!("0xf49d74 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::min_buckets_for_size(unsigned long)const")]
// 0xf49d84 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
pub fn stub_f49d84() -> ! {
    todo!("0xf49d84 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "std::_Vector_base<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::_M_allocate(unsigned long)")]
// 0xf49d94 — j___ZNSt12_Vector_baseIN3RBX10Controller6ButtonESaIS2_EE11_M_allocateEm
pub fn stub_f49d94() -> ! {
    todo!("0xf49d94 j___ZNSt12_Vector_baseIN3RBX10Controller6ButtonESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Name const*,std::allocator<RBX::Name const*>>::_M_allocate(unsigned long)")]
// 0xf49da4 — j___ZNSt12_Vector_baseIPKN3RBX4NameESaIS3_EE11_M_allocateEm
pub fn stub_f49da4() -> ! {
    todo!("0xf49da4 j___ZNSt12_Vector_baseIPKN3RBX4NameESaIS3_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<std::string,std::allocator<std::string>>::_M_allocate(unsigned long)")]
// 0xf49db4 — j___ZNSt12_Vector_baseISsSaISsEE11_M_allocateEm
pub fn stub_f49db4() -> ! {
    todo!("0xf49db4 j___ZNSt12_Vector_baseISsSaISsEE11_M_allocateEm")
}

#[doc(alias = "RBX::Controller::Button * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Controller::Button *,RBX::Controller::Button *>(RBX::Controller::Button *,RBX::Controller::Button *,RBX::Controller::Button *)")]
// 0xf49dc4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Controller6ButtonES6_EET0_T_S8_S7_
pub fn stub_f49dc4() -> ! {
    todo!("0xf49dc4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Controller6ButtonES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::string * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::string *,std::string *>(std::string *,std::string *,std::string *)")]
// 0xf49dd4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSsS3_EET0_T_S5_S4_
pub fn stub_f49dd4() -> ! {
    todo!("0xf49dd4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSsS3_EET0_T_S5_S4_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Controller::Button,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::operator[](RBX::Name const* const&)")]
// 0xf49de4 — j___ZNSt3mapIPKN3RBX4NameENS0_10Controller6ButtonESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f49de4() -> ! {
    todo!("0xf49de4 j___ZNSt3mapIPKN3RBX4NameENS0_10Controller6ButtonESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Controller::Button*,std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>>,RBX::Controller::Button const&)")]
// 0xf49df4 — j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f49df4() -> ! {
    todo!("0xf49df4 j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Controller::Button*,std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>>,unsigned long,RBX::Controller::Button const&)")]
// 0xf49e04 — j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f49e04() -> ! {
    todo!("0xf49e04 j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::resize(unsigned long,RBX::Controller::Button)")]
// 0xf49e14 — j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE6resizeEmS2_
pub fn stub_f49e14() -> ! {
    todo!("0xf49e14 j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::push_back(RBX::Controller::Button const&)")]
// 0xf49e24 — j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE9push_backERKS2_
pub fn stub_f49e24() -> ! {
    todo!("0xf49e24 j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Name const**,std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>>,unsigned long,RBX::Name const* const&)")]
// 0xf49e34 — j___ZNSt6vectorIPKN3RBX4NameESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
pub fn stub_f49e34() -> ! {
    todo!("0xf49e34 j___ZNSt6vectorIPKN3RBX4NameESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")
}

#[doc(alias = "std::vector<unsigned long,std::allocator<unsigned long>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned long *,std::vector<unsigned long,std::allocator<unsigned long>>>,unsigned long,unsigned long const&)")]
// 0xf49e44 — j___ZNSt6vectorImSaImEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPmS1_EEmRKm
pub fn stub_f49e44() -> ! {
    todo!("0xf49e44 j___ZNSt6vectorImSaImEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPmS1_EEmRKm")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Controller::Button> const&)")]
// 0xf49e54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f49e54() -> ! {
    todo!("0xf49e54 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::pair<RBX::Name const* const,RBX::Controller::Button> const&)")]
// 0xf49e64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f49e64() -> ! {
    todo!("0xf49e64 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Controller::Button>> *)")]
// 0xf49e74 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f49e74() -> ! {
    todo!("0xf49e74 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Controller::Button> const&)")]
// 0xf49e84 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f49e84() -> ! {
    todo!("0xf49e84 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<std::string *,unsigned long,std::string>(std::string *,unsigned long,std::string const&,std::__false_type)")]
// 0xf49e94 — j___ZSt26__uninitialized_fill_n_auxIPSsmSsEvT_T0_RKT1_St12__false_type
pub fn stub_f49e94() -> ! {
    todo!("0xf49e94 j___ZSt26__uninitialized_fill_n_auxIPSsmSsEvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "std::basic_string<char,std::char_traits<char>,std::allocator<char>> std::operator+<char,std::char_traits<char>,std::allocator<char>>(char const*,std::basic_string<char,std::char_traits<char>,std::allocator<char>> const&)")]
// 0xf49ea4 — j___ZStplIcSt11char_traitsIcESaIcEESbIT_T0_T1_EPKS3_RKS6_
pub fn stub_f49ea4() -> ! {
    todo!("0xf49ea4 j___ZStplIcSt11char_traitsIcESaIcEESbIT_T0_T1_EPKS3_RKS6_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::BrickColor)>::operator()(RBX::BrickColor)")]
// 0xf4a534 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10BrickColorEEEclES3_
pub fn stub_f4a534() -> ! {
    todo!("0xf4a534 j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10BrickColorEEEclES3_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::RbxRay)>::operator()(RBX::RbxRay)")]
// 0xf4a544 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX6RbxRayEEEclES3_
pub fn stub_f4a544() -> ! {
    todo!("0xf4a544 j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX6RbxRayEEEclES3_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(int)>::operator()(int)")]
// 0xf4a554 — j___ZN3rbx7signals16signal_with_argsILi1EFviEEclEi
pub fn stub_f4a554() -> ! {
    todo!("0xf4a554 j___ZN3rbx7signals16signal_with_argsILi1EFviEEclEi")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::disconnectAll(void)")]
// 0xf4a6d4 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE13disconnectAllEv
pub fn stub_f4a6d4() -> ! {
    todo!("0xf4a6d4 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::safe_static_do_get_mutex(void)")]
// 0xf4a6e4 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE24safe_static_do_get_mutexEv
pub fn stub_f4a6e4() -> ! {
    todo!("0xf4a6e4 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot> &)")]
// 0xf4a6f4 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// was: rbx::signals::signal<void ()(RBX::BrickColor)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot> &)
pub fn stub_f4a6f4() -> ! {
    todo!("0xf4a6f4 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::slot::safe_static_do_get_mutex(void)")]
// 0xf4a704 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f4a704() -> ! {
    todo!("0xf4a704 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::insert(rbx::signals::signal<void ()(RBX::BrickColor)>::slot *)")]
// 0xf4a714 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE6insertEPNS5_4slotE
pub fn stub_f4a714() -> ! {
    todo!("0xf4a714 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE6insertEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::remove(rbx::signals::signal<void ()(RBX::BrickColor)>::slot *)")]
// 0xf4a724 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE6removeEPNS5_4slotE
pub fn stub_f4a724() -> ! {
    todo!("0xf4a724 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE6removeEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::BrickColor)>::connect<boost::function<void ()(RBX::BrickColor)>>(boost::function<void ()(RBX::BrickColor)> const&)")]
// 0xf4a734 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
pub fn stub_f4a734() -> ! {
    todo!("0xf4a734 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::on_error(std::exception &)")]
// 0xf4a744 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE8on_errorERSt9exception
pub fn stub_f4a744() -> ! {
    todo!("0xf4a744 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RbxRay)>::disconnectAll(void)")]
// 0xf4a754 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE13disconnectAllEv
pub fn stub_f4a754() -> ! {
    todo!("0xf4a754 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RbxRay)>::safe_static_do_get_mutex(void)")]
// 0xf4a764 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE24safe_static_do_get_mutexEv
pub fn stub_f4a764() -> ! {
    todo!("0xf4a764 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RbxRay)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot> &)")]
// 0xf4a774 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// was: rbx::signals::signal<void ()(RBX::RbxRay)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot> &)
pub fn stub_f4a774() -> ! {
    todo!("0xf4a774 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RbxRay)>::slot::safe_static_do_get_mutex(void)")]
// 0xf4a784 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f4a784() -> ! {
    todo!("0xf4a784 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RbxRay)>::insert(rbx::signals::signal<void ()(RBX::RbxRay)>::slot *)")]
// 0xf4a794 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE6insertEPNS5_4slotE
pub fn stub_f4a794() -> ! {
    todo!("0xf4a794 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE6insertEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RbxRay)>::remove(rbx::signals::signal<void ()(RBX::RbxRay)>::slot *)")]
// 0xf4a7a4 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE6removeEPNS5_4slotE
pub fn stub_f4a7a4() -> ! {
    todo!("0xf4a7a4 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE6removeEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::RbxRay)>::connect<boost::function<void ()(RBX::RbxRay)>>(boost::function<void ()(RBX::RbxRay)> const&)")]
// 0xf4a7b4 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
pub fn stub_f4a7b4() -> ! {
    todo!("0xf4a7b4 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RbxRay)>::on_error(std::exception &)")]
// 0xf4a7c4 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE8on_errorERSt9exception
pub fn stub_f4a7c4() -> ! {
    todo!("0xf4a7c4 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot> &)")]
// 0xf4a7d4 — j___ZN3rbx7signals6signalIFviEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int)>::slot> &)
pub fn stub_f4a7d4() -> ! {
    todo!("0xf4a7d4 j___ZN3rbx7signals6signalIFviEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::on_error(std::exception &)")]
// 0xf4a7e4 — j___ZN3rbx7signals6signalIFviEE8on_errorERSt9exception
pub fn stub_f4a7e4() -> ! {
    todo!("0xf4a7e4 j___ZN3rbx7signals6signalIFviEE8on_errorERSt9exception")
}

#[doc(alias = "RBX::RbxRay const& rbx::any_cast<RBX::RbxRay const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf4a7f4 — j___ZN3rbx8any_castIRKN3RBX6RbxRayENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f4a7f4() -> ! {
    todo!("0xf4a7f4 j___ZN3rbx8any_castIRKN3RBX6RbxRayENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::callable<rbx::signals::signal<void ()(RBX::BrickColor)>*>(boost::function<void ()(RBX::BrickColor)> const&,rbx::signals::signal<void ()(RBX::BrickColor)>*)")]
// 0xf4a834 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX10BrickColorEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
pub fn stub_f4a834() -> ! {
    todo!("0xf4a834 j___ZN3rbx8callableINS_7signals6signalIFvN3RBX10BrickColorEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RbxRay)>::slot,boost::function<void ()(RBX::RbxRay)>,1,void ()(RBX::RbxRay)>::callable<rbx::signals::signal<void ()(RBX::RbxRay)>*>(boost::function<void ()(RBX::RbxRay)> const&,rbx::signals::signal<void ()(RBX::RbxRay)>*)")]
// 0xf4a844 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX6RbxRayEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
pub fn stub_f4a844() -> ! {
    todo!("0xf4a844 j___ZN3rbx8callableINS_7signals6signalIFvN3RBX6RbxRayEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::function<void ()(int)>,1,void ()(int)>::callable<rbx::signals::signal<void ()(int)>*>(boost::function<void ()(int)> const&,rbx::signals::signal<void ()(int)>*)")]
// 0xf4a854 — j___ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
pub fn stub_f4a854() -> ! {
    todo!("0xf4a854 j___ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot>::operator=(rbx::signals::signal<void ()(RBX::BrickColor)>::slot*)")]
// 0xf4a954 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotEEaSEPS8_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot>::operator=(rbx::signals::signal<void ()(RBX::BrickColor)>::slot*)
pub fn stub_f4a954() -> ! {
    todo!("0xf4a954 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotEEaSEPS8_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot> const&)")]
// 0xf4a964 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotEEaSERKS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot> const&)
pub fn stub_f4a964() -> ! {
    todo!("0xf4a964 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot>::operator=(rbx::signals::signal<void ()(RBX::RbxRay)>::slot*)")]
// 0xf4a974 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slotEEaSEPS8_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot>::operator=(rbx::signals::signal<void ()(RBX::RbxRay)>::slot*)
pub fn stub_f4a974() -> ! {
    todo!("0xf4a974 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slotEEaSEPS8_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot> const&)")]
// 0xf4a984 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slotEEaSERKS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot> const&)
pub fn stub_f4a984() -> ! {
    todo!("0xf4a984 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slotEEaSERKS9_")
}

#[doc(alias = "boost::function1<void,RBX::BrickColor>::assign_to_own(boost::function1<void,RBX::BrickColor> const&)")]
// 0xf4ac44 — j___ZN5boost9function1IvN3RBX10BrickColorEE13assign_to_ownERKS3_
pub fn stub_f4ac44() -> ! {
    todo!("0xf4ac44 j___ZN5boost9function1IvN3RBX10BrickColorEE13assign_to_ownERKS3_")
}

#[doc(alias = "boost::function1<void,RBX::BrickColor>::clear(void)")]
// 0xf4ac54 — j___ZN5boost9function1IvN3RBX10BrickColorEE5clearEv
pub fn stub_f4ac54() -> ! {
    todo!("0xf4ac54 j___ZN5boost9function1IvN3RBX10BrickColorEE5clearEv")
}

#[doc(alias = "boost::function1<void,RBX::RbxRay>::assign_to_own(boost::function1<void,RBX::RbxRay> const&)")]
// 0xf4ac84 — j___ZN5boost9function1IvN3RBX6RbxRayEE13assign_to_ownERKS3_
pub fn stub_f4ac84() -> ! {
    todo!("0xf4ac84 j___ZN5boost9function1IvN3RBX6RbxRayEE13assign_to_ownERKS3_")
}

#[doc(alias = "boost::function1<void,RBX::RbxRay>::clear(void)")]
// 0xf4ac94 — j___ZN5boost9function1IvN3RBX6RbxRayEE5clearEv
pub fn stub_f4ac94() -> ! {
    todo!("0xf4ac94 j___ZN5boost9function1IvN3RBX6RbxRayEE5clearEv")
}

#[doc(alias = "RBX::RbxRay::operator!=(RBX::RbxRay const&)const")]
// 0xf4ad74 — j___ZNK3RBX6RbxRayneERKS0_
pub fn stub_f4ad74() -> ! {
    todo!("0xf4ad74 j___ZNK3RBX6RbxRayneERKS0_")
}

#[doc(alias = "boost::function1<void,RBX::BrickColor>::operator()(RBX::BrickColor)const")]
// 0xf4af54 — j___ZNK5boost9function1IvN3RBX10BrickColorEEclES2_
pub fn stub_f4af54() -> ! {
    todo!("0xf4af54 j___ZNK5boost9function1IvN3RBX10BrickColorEEclES2_")
}

#[doc(alias = "boost::function1<void,RBX::RbxRay>::operator()(RBX::RbxRay)const")]
// 0xf4af64 — j___ZNK5boost9function1IvN3RBX6RbxRayEEclES2_
pub fn stub_f4af64() -> ! {
    todo!("0xf4af64 j___ZNK5boost9function1IvN3RBX6RbxRayEEclES2_")
}

#[doc(alias = "RBX::Primitive * RBX::IndexedTree::getTypedChild<RBX::Primitive>(int)")]
// 0xf4b054 — j___ZN3RBX11IndexedTree13getTypedChildINS_9PrimitiveEEEPT_i
pub fn stub_f4b054() -> ! {
    todo!("0xf4b054 j___ZN3RBX11IndexedTree13getTypedChildINS_9PrimitiveEEEPT_i")
}

#[doc(alias = "RBX::KernelJoint::getBody(RBX::Connector::BodyIndex)")]
// 0xf4b064 — j___ZN3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE
pub fn stub_f4b064() -> ! {
    todo!("0xf4b064 j___ZN3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE")
}

#[doc(alias = "RBX::KernelJoint::~KernelJoint()")]
// 0xf4b074 — j___ZN3RBX11KernelJointD0Ev
pub fn stub_f4b074() -> ! {
    todo!("0xf4b074 j___ZN3RBX11KernelJointD0Ev")
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>,RBX::Primitive *)")]
// 0xf4b104 — j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_11VehicleSeatEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_
pub fn stub_f4b104() -> ! {
    todo!("0xf4b104 j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_11VehicleSeatEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_")
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>,RBX::Primitive *)")]
// 0xf4b114 — j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERSt6vectorIPKS5_SaIS9_EEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISB_EEEEEEEEvT_S6_
pub fn stub_f4b114() -> ! {
    todo!("0xf4b114 j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERSt6vectorIPKS5_SaIS9_EEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISB_EEEEEEEEvT_S6_")
}

#[doc(alias = "RBX::Velocity::zero(void)")]
// 0xf4b194 — j___ZN3RBX8Velocity4zeroEv
pub fn stub_f4b194() -> ! {
    todo!("0xf4b194 j___ZN3RBX8Velocity4zeroEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VehicleController>::shared_ptr<RBX::VehicleController>(rbx_core::WeakPtr<RBX::VehicleController> const&,boost::detail::sp_nothrow_tag)")]
// 0xf4b1f4 — j___ZN5boost10shared_ptrIN3RBX17VehicleControllerEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::VehicleController>::shared_ptr<RBX::VehicleController>(boost::weak_ptr<RBX::VehicleController> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f4b1f4() -> ! {
    todo!("0xf4b1f4 j___ZN5boost10shared_ptrIN3RBX17VehicleControllerEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "RBX::IPipelined::inStage(RBX::IStage::StageType)const")]
// 0xf4b264 — j___ZNK3RBX10IPipelined7inStageENS_6IStage9StageTypeE
pub fn stub_f4b264() -> ! {
    todo!("0xf4b264 j___ZNK3RBX10IPipelined7inStageENS_6IStage9StageTypeE")
}

#[doc(alias = "RBX::VirtualUser::~VirtualUser()")]
// 0xf4b364 — j___ZN3RBX11VirtualUserD1Ev
pub fn stub_f4b364() -> ! {
    todo!("0xf4b364 j___ZN3RBX11VirtualUserD1Ev")
}

#[doc(alias = "RBX::UserInputBase::~UserInputBase()")]
// 0xf4b374 — j___ZN3RBX13UserInputBaseD2Ev
pub fn stub_f4b374() -> ! {
    todo!("0xf4b374 j___ZN3RBX13UserInputBaseD2Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot::safe_static_do_get_mutex(void)")]
// 0xf4b3d4 — j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f4b3d4() -> ! {
    todo!("0xf4b3d4 j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::insert(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot *)")]
// 0xf4b3e4 — j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE6insertEPNS7_4slotE
pub fn stub_f4b3e4() -> ! {
    todo!("0xf4b3e4 j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE6insertEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::remove(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot *)")]
// 0xf4b3f4 — j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE6removeEPNS7_4slotE
pub fn stub_f4b3f4() -> ! {
    todo!("0xf4b3f4 j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE6removeEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UIEvent const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>> const&)")]
// 0xf4b404 — j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_11VirtualUserES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_f4b404() -> ! {
    todo!("0xf4b404 j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_11VirtualUserES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "boost::scoped_ptr<RBX::VirtualHardwareDevice>::~scoped_ptr()")]
// 0xf4b414 — j___ZN5boost10scoped_ptrIN3RBX21VirtualHardwareDeviceEED2Ev
pub fn stub_f4b414() -> ! {
    todo!("0xf4b414 j___ZN5boost10scoped_ptrIN3RBX21VirtualHardwareDeviceEED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot*)")]
// 0xf4b434 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot*)
pub fn stub_f4b434() -> ! {
    todo!("0xf4b434 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slotEEaSEPSA_")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>::operator()<RBX::UIEvent>(RBX::UIEvent const&)")]
// 0xf4b444 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11VirtualUserERKNS4_7UIEventEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
pub fn stub_f4b444() -> ! {
    todo!("0xf4b444 j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11VirtualUserERKNS4_7UIEventEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_")
}

#[doc(alias = "boost::scoped_ptr<RBX::worker_thread>::~scoped_ptr()")]
// 0xf4b504 — j___ZN5boost10scoped_ptrIN3RBX13worker_threadEED2Ev
pub fn stub_f4b504() -> ! {
    todo!("0xf4b504 j___ZN5boost10scoped_ptrIN3RBX13worker_threadEED2Ev")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>::list2(boost::_bi::value<std::string>,boost::_bi::value<int>)")]
// 0xf4b514 — j___ZN5boost3_bi5list2INS0_5valueISsEENS2_IiEEEC2ES3_S4_
pub fn stub_f4b514() -> ! {
    todo!("0xf4b514 j___ZN5boost3_bi5list2INS0_5valueISsEENS2_IiEEEC2ES3_S4_")
}

#[doc(alias = "RBX::worker_thread::work_result boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>::operator()<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list0>(boost::_bi::type<RBX::worker_thread::work_result>,RBX::worker_thread::work_result (*)(std::string,int) &,boost::_bi::list0 &,long)")]
// 0xf4b524 — j___ZN5boost3_bi5list2INS0_5valueISsEENS2_IiEEEclIN3RBX13worker_thread11work_resultEPFS9_SsiENS0_5list0EEET_NS0_4typeISD_EERT0_RT1_l
pub fn stub_f4b524() -> ! {
    todo!("0xf4b524 j___ZN5boost3_bi5list2INS0_5valueISsEENS2_IiEEEclIN3RBX13worker_thread11work_resultEPFS9_SsiENS0_5list0EEET_NS0_4typeISD_EERT0_RT1_l")
}
