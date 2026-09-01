//! core shard FM — 100 core stubs EA-sorted, 0xf32504..0xf33364 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after FL 0xf324f4 gap).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf324f4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::map<int,std::set<int,std::less<int>,std::allocator<int>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::operator[](int const&)")]
// 0xf32504 — j___ZNSt3mapIiSt3setIiSt4lessIiESaIiEES2_SaISt4pairIKiS4_EEEixERS6_
pub fn stub_f32504() -> ! {
    todo!("0xf32504 j___ZNSt3mapIiSt3setIiSt4lessIiESaIiEES2_SaISt4pairIKiS4_EEEixERS6_")
}

#[doc(alias = "std::map<int,bool,std::less<int>,std::allocator<std::pair<int const,bool>>>::operator[](int const&)")]
// 0xf32514 — j___ZNSt3mapIibSt4lessIiESaISt4pairIKibEEEixERS3_
pub fn stub_f32514() -> ! {
    todo!("0xf32514 j___ZNSt3mapIibSt4lessIiESaISt4pairIKibEEEixERS3_")
}

#[doc(alias = "std::list<RBX::BadgeService::HotUserHasBadge,std::allocator<RBX::BadgeService::HotUserHasBadge>>::erase(std::_List_iterator<RBX::BadgeService::HotUserHasBadge>,std::_List_iterator<RBX::BadgeService::HotUserHasBadge>)")]
// 0xf32524 — j___ZNSt4listIN3RBX12BadgeService15HotUserHasBadgeESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_
pub fn stub_f32524() -> ! {
    todo!("0xf32524 j___ZNSt4listIN3RBX12BadgeService15HotUserHasBadgeESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_create_node(std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
// 0xf32534 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE14_M_create_nodeERKS7_
pub fn stub_f32534() -> ! {
    todo!("0xf32534 j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE14_M_create_nodeERKS7_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_insert_unique(std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
// 0xf32544 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_f32544() -> ! {
    todo!("0xf32544 j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
// 0xf32554 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_f32554() -> ! {
    todo!("0xf32554 j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>> *)")]
// 0xf32564 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_f32564() -> ! {
    todo!("0xf32564 j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
// 0xf32574 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS7_
pub fn stub_f32574() -> ! {
    todo!("0xf32574 j___ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS7_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,bool>,std::_Select1st<std::pair<int const,bool>>,std::less<int>,std::allocator<std::pair<int const,bool>>>::_M_insert_unique(std::pair<int const,bool> const&)")]
// 0xf32584 — j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_f32584() -> ! {
    todo!("0xf32584 j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,bool>,std::_Select1st<std::pair<int const,bool>>,std::less<int>,std::allocator<std::pair<int const,bool>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,bool>>,std::pair<int const,bool> const&)")]
// 0xf32594 — j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
pub fn stub_f32594() -> ! {
    todo!("0xf32594 j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,bool>,std::_Select1st<std::pair<int const,bool>>,std::less<int>,std::allocator<std::pair<int const,bool>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,bool>> *)")]
// 0xf325a4 — j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_f325a4() -> ! {
    todo!("0xf325a4 j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,bool>,std::_Select1st<std::pair<int const,bool>>,std::less<int>,std::allocator<std::pair<int const,bool>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,bool> const&)")]
// 0xf325b4 — j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_f325b4() -> ! {
    todo!("0xf325b4 j___ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_insert_unique(int const&)")]
// 0xf325c4 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE16_M_insert_uniqueERKi
pub fn stub_f325c4() -> ! {
    todo!("0xf325c4 j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE16_M_insert_uniqueERKi")
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_copy(std::_Rb_tree_node<int> const*,std::_Rb_tree_node<int>*)")]
// 0xf325d4 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE7_M_copyEPKSt13_Rb_tree_nodeIiEPS7_
pub fn stub_f325d4() -> ! {
    todo!("0xf325d4 j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE7_M_copyEPKSt13_Rb_tree_nodeIiEPS7_")
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_erase(std::_Rb_tree_node<int> *)")]
// 0xf325e4 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE8_M_eraseEPSt13_Rb_tree_nodeIiE
pub fn stub_f325e4() -> ! {
    todo!("0xf325e4 j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE8_M_eraseEPSt13_Rb_tree_nodeIiE")
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,int const&)")]
// 0xf325f4 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKi
pub fn stub_f325f4() -> ! {
    todo!("0xf325f4 j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKi")
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_Rb_tree(std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>> const&)")]
// 0xf32604 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEEC2ERKS5_
pub fn stub_f32604() -> ! {
    todo!("0xf32604 j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEEC2ERKS5_")
}

#[doc(alias = "RBX::BillboardGui::~BillboardGui()")]
// 0xf32754 — j___ZN3RBX12BillboardGuiD2Ev
pub fn stub_f32754() -> ! {
    todo!("0xf32754 j___ZN3RBX12BillboardGuiD2Ev")
}

#[doc(alias = "boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)>::operator=(boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)> const&)")]
// 0xf327a4 — j___ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEaSERKS7_
pub fn stub_f327a4() -> ! {
    todo!("0xf327a4 j___ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEaSERKS7_")
}

#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::move_assign(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&)")]
// 0xf327b4 — j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE11move_assignERS6_
pub fn stub_f327b4() -> ! {
    todo!("0xf327b4 j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE11move_assignERS6_")
}

#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to_own(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *> const&)")]
// 0xf327c4 — j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE13assign_to_ownERKS6_
pub fn stub_f327c4() -> ! {
    todo!("0xf327c4 j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE13assign_to_ownERKS6_")
}

#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::swap(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&)")]
// 0xf327d4 — j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE4swapERS6_
pub fn stub_f327d4() -> ! {
    todo!("0xf327d4 j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE4swapERS6_")
}

#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::operator()(RBX::BillboardGui *,RBX::Adorn *)const")]
// 0xf32804 — j___ZNK5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEclES3_S5_
pub fn stub_f32804() -> ! {
    todo!("0xf32804 j___ZNK5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEclES3_S5_")
}

#[doc(alias = "RBX::Camera::~Camera()")]
// 0xf32ab4 — j___ZN3RBX6CameraD0Ev
pub fn stub_f32ab4() -> ! {
    todo!("0xf32ab4 j___ZN3RBX6CameraD0Ev")
}

#[doc(alias = "RBX::Camera::~Camera()")]
// 0xf32ac4 — j___ZN3RBX6CameraD2Ev
pub fn stub_f32ac4() -> ! {
    todo!("0xf32ac4 j___ZN3RBX6CameraD2Ev")
}

#[doc(alias = "RBX::Tolerance::maxExtents(void)")]
// 0xf32ad4 — j___ZN3RBX9Tolerance10maxExtentsEv
pub fn stub_f32ad4() -> ! {
    todo!("0xf32ad4 j___ZN3RBX9Tolerance10maxExtentsEv")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraMode>(RBX::Camera::CameraMode const&)")]
// 0xf32ae4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraModeEEERS3_RKT_
pub fn stub_f32ae4() -> ! {
    todo!("0xf32ae4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraModeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraType>(RBX::Camera::CameraType const&)")]
// 0xf32af4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraTypeEEERS3_RKT_
pub fn stub_f32af4() -> ! {
    todo!("0xf32af4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraPanMode>(RBX::Camera::CameraPanMode const&)")]
// 0xf32b04 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera13CameraPanModeEEERS3_RKT_
pub fn stub_f32b04() -> ! {
    todo!("0xf32b04 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera13CameraPanModeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraMode>::singleton(void)")]
// 0xf32b14 — j___ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE9singletonEv
pub fn stub_f32b14() -> ! {
    todo!("0xf32b14 j___ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraType>::singleton(void)")]
// 0xf32b24 — j___ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE9singletonEv
pub fn stub_f32b24() -> ! {
    todo!("0xf32b24 j___ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraPanMode>::singleton(void)")]
// 0xf32b34 — j___ZN3rbx14implementation12typed_holderIN3RBX6Camera13CameraPanModeEE9singletonEv
pub fn stub_f32b34() -> ! {
    todo!("0xf32b34 j___ZN3rbx14implementation12typed_holderIN3RBX6Camera13CameraPanModeEE9singletonEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::safe_static_do_get_mutex(void)")]
// 0xf32b44 — j___ZN3rbx7signals6signalIFvbEE4slot24safe_static_do_get_mutexEv
pub fn stub_f32b44() -> ! {
    todo!("0xf32b44 j___ZN3rbx7signals6signalIFvbEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool)>::connect<boost::function<void ()(bool)>>(boost::function<void ()(bool)> const&)")]
// 0xf32b54 — j___ZN3rbx7signals6signalIFvbEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_f32b54() -> ! {
    todo!("0xf32b54 j___ZN3rbx7signals6signalIFvbEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "RBX::Camera::CameraPanMode * rbx::any_cast<RBX::Camera::CameraPanMode,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf32b64 — j___ZN3rbx8any_castIN3RBX6Camera13CameraPanModeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f32b64() -> ! {
    todo!("0xf32b64 j___ZN3rbx8any_castIN3RBX6Camera13CameraPanModeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Camera::CameraMode const& rbx::any_cast<RBX::Camera::CameraMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf32b74 — j___ZN3rbx8any_castIRKN3RBX6Camera10CameraModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f32b74() -> ! {
    todo!("0xf32b74 j___ZN3rbx8any_castIRKN3RBX6Camera10CameraModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Camera::CameraType const& rbx::any_cast<RBX::Camera::CameraType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf32b84 — j___ZN3rbx8any_castIRKN3RBX6Camera10CameraTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f32b84() -> ! {
    todo!("0xf32b84 j___ZN3rbx8any_castIRKN3RBX6Camera10CameraTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Camera::CameraPanMode const& rbx::any_cast<RBX::Camera::CameraPanMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf32b94 — j___ZN3rbx8any_castIRKN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f32b94() -> ! {
    todo!("0xf32b94 j___ZN3rbx8any_castIRKN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Camera::CameraPanMode & rbx::any_cast<RBX::Camera::CameraPanMode &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf32ba4 — j___ZN3rbx8any_castIRN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f32ba4() -> ! {
    todo!("0xf32ba4 j___ZN3rbx8any_castIRN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::NavKeys::navKeyDown(void)const")]
// 0xf32cc4 — j___ZNK3RBX7NavKeys10navKeyDownEv
pub fn stub_f32cc4() -> ! {
    todo!("0xf32cc4 j___ZNK3RBX7NavKeys10navKeyDownEv")
}

#[doc(alias = "std::_Vector_base<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_allocate(unsigned long)")]
// 0xf32cf4 — j___ZNSt12_Vector_baseIN3RBX6Camera10CameraModeESaIS2_EE11_M_allocateEm
pub fn stub_f32cf4() -> ! {
    todo!("0xf32cf4 j___ZNSt12_Vector_baseIN3RBX6Camera10CameraModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_allocate(unsigned long)")]
// 0xf32d04 — j___ZNSt12_Vector_baseIN3RBX6Camera10CameraTypeESaIS2_EE11_M_allocateEm
pub fn stub_f32d04() -> ! {
    todo!("0xf32d04 j___ZNSt12_Vector_baseIN3RBX6Camera10CameraTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_allocate(unsigned long)")]
// 0xf32d14 — j___ZNSt12_Vector_baseIN3RBX6Camera13CameraPanModeESaIS2_EE11_M_allocateEm
pub fn stub_f32d14() -> ! {
    todo!("0xf32d14 j___ZNSt12_Vector_baseIN3RBX6Camera13CameraPanModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Camera::CameraMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraMode *,RBX::Camera::CameraMode *>(RBX::Camera::CameraMode *,RBX::Camera::CameraMode *,RBX::Camera::CameraMode *)")]
// 0xf32d34 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraModeES6_EET0_T_S8_S7_
pub fn stub_f32d34() -> ! {
    todo!("0xf32d34 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::Camera::CameraType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraType *,RBX::Camera::CameraType *>(RBX::Camera::CameraType *,RBX::Camera::CameraType *,RBX::Camera::CameraType *)")]
// 0xf32d44 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraTypeES6_EET0_T_S8_S7_
pub fn stub_f32d44() -> ! {
    todo!("0xf32d44 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::Camera::CameraPanMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *>(RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *)")]
// 0xf32d54 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera13CameraPanModeES6_EET0_T_S8_S7_
pub fn stub_f32d54() -> ! {
    todo!("0xf32d54 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera13CameraPanModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::operator[](RBX::Name const* const&)")]
// 0xf32d74 — j___ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f32d74() -> ! {
    todo!("0xf32d74 j___ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::operator[](RBX::Name const* const&)")]
// 0xf32d84 — j___ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f32d84() -> ! {
    todo!("0xf32d84 j___ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraPanMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::operator[](RBX::Name const* const&)")]
// 0xf32d94 — j___ZNSt3mapIPKN3RBX4NameENS0_6Camera13CameraPanModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f32d94() -> ! {
    todo!("0xf32d94 j___ZNSt3mapIPKN3RBX4NameENS0_6Camera13CameraPanModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraMode*,std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>>,RBX::Camera::CameraMode const&)")]
// 0xf32db4 — j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f32db4() -> ! {
    todo!("0xf32db4 j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraMode*,std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>>,unsigned long,RBX::Camera::CameraMode const&)")]
// 0xf32dc4 — j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f32dc4() -> ! {
    todo!("0xf32dc4 j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::resize(unsigned long,RBX::Camera::CameraMode)")]
// 0xf32dd4 — j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE6resizeEmS2_
pub fn stub_f32dd4() -> ! {
    todo!("0xf32dd4 j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::push_back(RBX::Camera::CameraMode const&)")]
// 0xf32de4 — j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE9push_backERKS2_
pub fn stub_f32de4() -> ! {
    todo!("0xf32de4 j___ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraType*,std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>>,RBX::Camera::CameraType const&)")]
// 0xf32df4 — j___ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f32df4() -> ! {
    todo!("0xf32df4 j___ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraType*,std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>>,unsigned long,RBX::Camera::CameraType const&)")]
// 0xf32e04 — j___ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f32e04() -> ! {
    todo!("0xf32e04 j___ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::resize(unsigned long,RBX::Camera::CameraType)")]
// 0xf32e14 — j___ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE6resizeEmS2_
pub fn stub_f32e14() -> ! {
    todo!("0xf32e14 j___ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::push_back(RBX::Camera::CameraType const&)")]
// 0xf32e24 — j___ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE9push_backERKS2_
pub fn stub_f32e24() -> ! {
    todo!("0xf32e24 j___ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraPanMode*,std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>>,RBX::Camera::CameraPanMode const&)")]
// 0xf32e34 — j___ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f32e34() -> ! {
    todo!("0xf32e34 j___ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraPanMode*,std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>>,unsigned long,RBX::Camera::CameraPanMode const&)")]
// 0xf32e44 — j___ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f32e44() -> ! {
    todo!("0xf32e44 j___ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::resize(unsigned long,RBX::Camera::CameraPanMode)")]
// 0xf32e54 — j___ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE6resizeEmS2_
pub fn stub_f32e54() -> ! {
    todo!("0xf32e54 j___ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::push_back(RBX::Camera::CameraPanMode const&)")]
// 0xf32e64 — j___ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE9push_backERKS2_
pub fn stub_f32e64() -> ! {
    todo!("0xf32e64 j___ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")]
// 0xf32eb4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f32eb4() -> ! {
    todo!("0xf32eb4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")]
// 0xf32ec4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f32ec4() -> ! {
    todo!("0xf32ec4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>> *)")]
// 0xf32ed4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f32ed4() -> ! {
    todo!("0xf32ed4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")]
// 0xf32ee4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f32ee4() -> ! {
    todo!("0xf32ee4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)")]
// 0xf32ef4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f32ef4() -> ! {
    todo!("0xf32ef4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)")]
// 0xf32f04 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f32f04() -> ! {
    todo!("0xf32f04 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraType>> *)")]
// 0xf32f14 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f32f14() -> ! {
    todo!("0xf32f14 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)")]
// 0xf32f24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f32f24() -> ! {
    todo!("0xf32f24 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")]
// 0xf32f34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f32f34() -> ! {
    todo!("0xf32f34 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")]
// 0xf32f44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f32f44() -> ! {
    todo!("0xf32f44 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>> *)")]
// 0xf32f54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f32f54() -> ! {
    todo!("0xf32f54 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")]
// 0xf32f64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f32f64() -> ! {
    todo!("0xf32f64 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "void delete_helper<RBX::ChangeHistoryService::Waypoint>(RBX::ChangeHistoryService::Waypoint *)")]
// 0xf32f74 — j___Z13delete_helperIN3RBX20ChangeHistoryService8WaypointEEvPT_
pub fn stub_f32f74() -> ! {
    todo!("0xf32f74 j___Z13delete_helperIN3RBX20ChangeHistoryService8WaypointEEvPT_")
}

#[doc(alias = "ChangeHistoryStatsItem::create(RBX::ChangeHistoryService &)")]
// 0xf32f84 — j___ZN22ChangeHistoryStatsItem6createERN3RBX20ChangeHistoryServiceE
pub fn stub_f32f84() -> ! {
    todo!("0xf32f84 j___ZN22ChangeHistoryStatsItem6createERN3RBX20ChangeHistoryServiceE")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::getCellData(unsigned int,unsigned int,unsigned int &)")]
// 0xf33094 — j___ZN3RBX20ChangeHistoryService4Item11getCellDataEjjRj
pub fn stub_f33094() -> ! {
    todo!("0xf33094 j___ZN3RBX20ChangeHistoryService4Item11getCellDataEjjRj")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::recordCreate(void)")]
// 0xf330a4 — j___ZN3RBX20ChangeHistoryService4Item12recordCreateEv
pub fn stub_f330a4() -> ! {
    todo!("0xf330a4 j___ZN3RBX20ChangeHistoryService4Item12recordCreateEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::recordDelete(void)")]
// 0xf330b4 — j___ZN3RBX20ChangeHistoryService4Item12recordDeleteEv
pub fn stub_f330b4() -> ! {
    todo!("0xf330b4 j___ZN3RBX20ChangeHistoryService4Item12recordDeleteEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplayChange(void)")]
// 0xf330c4 — j___ZN3RBX20ChangeHistoryService4Item12unplayChangeEv
pub fn stub_f330c4() -> ! {
    todo!("0xf330c4 j___ZN3RBX20ChangeHistoryService4Item12unplayChangeEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::onSetWaypoint(void)")]
// 0xf330d4 — j___ZN3RBX20ChangeHistoryService4Item13onSetWaypointEv
pub fn stub_f330d4() -> ! {
    todo!("0xf330d4 j___ZN3RBX20ChangeHistoryService4Item13onSetWaypointEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplay_CFrame(void)")]
// 0xf330e4 — j___ZN3RBX20ChangeHistoryService4Item13unplay_CFrameEv
pub fn stub_f330e4() -> ! {
    todo!("0xf330e4 j___ZN3RBX20ChangeHistoryService4Item13unplay_CFrameEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::addClusterDataFast(RBX::Voxel::Grid const*)")]
// 0xf33114 — j___ZN3RBX20ChangeHistoryService4Item18addClusterDataFastEPKNS_5Voxel4GridE
pub fn stub_f33114() -> ! {
    todo!("0xf33114 j___ZN3RBX20ChangeHistoryService4Item18addClusterDataFastEPKNS_5Voxel4GridE")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::recordClusterDataGetChunk(int)")]
// 0xf33124 — j___ZN3RBX20ChangeHistoryService4Item25recordClusterDataGetChunkEi
pub fn stub_f33124() -> ! {
    todo!("0xf33124 j___ZN3RBX20ChangeHistoryService4Item25recordClusterDataGetChunkEi")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::play(void)")]
// 0xf33134 — j___ZN3RBX20ChangeHistoryService4Item4playEv
pub fn stub_f33134() -> ! {
    todo!("0xf33134 j___ZN3RBX20ChangeHistoryService4Item4playEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::absorb(RBX::ChangeHistoryService::Item const&)")]
// 0xf33154 — j___ZN3RBX20ChangeHistoryService4Item6absorbERKS1_
pub fn stub_f33154() -> ! {
    todo!("0xf33154 j___ZN3RBX20ChangeHistoryService4Item6absorbERKS1_")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplay(void)")]
// 0xf33164 — j___ZN3RBX20ChangeHistoryService4Item6unplayEv
pub fn stub_f33164() -> ! {
    todo!("0xf33164 j___ZN3RBX20ChangeHistoryService4Item6unplayEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::absorb(RBX::ChangeHistoryService::Waypoint const*)")]
// 0xf33194 — j___ZN3RBX20ChangeHistoryService8Waypoint6absorbEPKS1_
pub fn stub_f33194() -> ! {
    todo!("0xf33194 j___ZN3RBX20ChangeHistoryService8Waypoint6absorbEPKS1_")
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::addItem(RBX::ChangeHistoryService::Item const&)")]
// 0xf331a4 — j___ZN3RBX20ChangeHistoryService8Waypoint7addItemERKNS0_4ItemE
pub fn stub_f331a4() -> ! {
    todo!("0xf331a4 j___ZN3RBX20ChangeHistoryService8Waypoint7addItemERKNS0_4ItemE")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<int>::TypedStatsItem(boost::function0<int>)")]
// 0xf331d4 — j___ZN3RBX5Stats14TypedStatsItemIiEC2EN5boost9function0IiEE
pub fn stub_f331d4() -> ! {
    todo!("0xf331d4 j___ZN3RBX5Stats14TypedStatsItemIiEC2EN5boost9function0IiEE")
}

#[doc(alias = "RBX::Stats::Item::Item(void)")]
// 0xf331e4 — j___ZN3RBX5Stats4ItemC2Ev
pub fn stub_f331e4() -> ! {
    todo!("0xf331e4 j___ZN3RBX5Stats4ItemC2Ev")
}

#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::iterator::iterator(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)")]
// 0xf331f4 — j___ZN3RBX5Voxel6RegionINS0_4Grid5ChunkEE8iteratorC2ERKS4_
pub fn stub_f331f4() -> ! {
    todo!("0xf331f4 j___ZN3RBX5Voxel6RegionINS0_4Grid5ChunkEE8iteratorC2ERKS4_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)")]
// 0xf33224 — j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_f33224() -> ! {
    todo!("0xf33224 j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "boost::flyweights::static_holder_class<boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::holder_arg>::get(void)")]
// 0xf33264 — j___ZN5boost10flyweights19static_holder_classINS0_6detail14flyweight_coreINS2_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS9_S9_S9_Li0EEENS0_14simple_lockingENS0_13static_holderEE10holder_argEE3getEv
pub fn stub_f33264() -> ! {
    todo!("0xf33264 j___ZN5boost10flyweights19static_holder_classINS0_6detail14flyweight_coreINS2_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS9_S9_S9_Li0EEENS0_14simple_lockingENS0_13static_holderEE10holder_argEE3getEv")
}

#[doc(alias = "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::~multi_index_container()")]
// 0xf332a4 — j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EED2Ev
pub fn stub_f332a4() -> ! {
    todo!("0xf332a4 j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EED2Ev")
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::delete_all_nodes_(void)")]
// 0xf332b4 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE17delete_all_nodes_Ev
pub fn stub_f332b4() -> ! {
    todo!("0xf332b4 j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE17delete_all_nodes_Ev")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&>,boost::_bi::list1<std::pair const&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&> &,boost::_bi::list1<std::pair const&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> &,int)")]
// 0xf332f4 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIjSt6vectorIjSaIjEEEEENS0_5list1IRKSE_IKjSH_EEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f332f4() -> ! {
    todo!("0xf332f4 j___ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIjSt6vectorIjSaIjEEEEENS0_5list1IRKSE_IKjSH_EEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&>,boost::_bi::list1<std::pair&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&> &,boost::_bi::list1<std::pair&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> &,int)")]
// 0xf33304 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIjSt6vectorIjSaIjEEEEENS0_5list1IRSE_IKjSH_EEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f33304() -> ! {
    todo!("0xf33304 j___ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIjSt6vectorIjSaIjEEEEENS0_5list1IRSE_IKjSH_EEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)")]
// 0xf33314 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS3_13RunTransitionEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f33314() -> ! {
    todo!("0xf33314 j___ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS3_13RunTransitionEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>::operator()(void)")]
// 0xf33354 — j___ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
pub fn stub_f33354() -> ! {
    todo!("0xf33354 j___ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>::operator()(void)")]
// 0xf33364 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS0_5list1INS0_5valueIPS6_EEEEEclEv
pub fn stub_f33364() -> ! {
    todo!("0xf33364 j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS0_5list1INS0_5valueIPS6_EEEEEclEv")
}

