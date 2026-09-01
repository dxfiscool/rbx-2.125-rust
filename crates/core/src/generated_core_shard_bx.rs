//! core shard BX — 100 core stubs EA-sorted, next uncovered after BW 0x253ecc (earliest gap 0x364e58).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::bad_placement_any_cast::~bad_placement_any_cast()")]
// 0x364e58 — __ZN3rbx22bad_placement_any_castD1Ev — rbx::bad_placement_any_cast::~bad_placement_any_cast()
pub fn stub_364e58() -> ! {
    todo!("0x364e58 __ZN3rbx22bad_placement_any_castD1Ev")
}

#[doc(alias = "rbx::bad_placement_any_cast::what(void)const")]
// 0x364e60 — __ZNK3rbx22bad_placement_any_cast4whatEv — rbx::bad_placement_any_cast::what(void)const
pub fn stub_364e60() -> ! {
    todo!("0x364e60 __ZNK3rbx22bad_placement_any_cast4whatEv")
}

#[doc(alias = "rbx::implementation::typed_holder<double>::singleton(void)")]
// 0x365550 — __ZN3rbx14implementation12typed_holderIdE9singletonEv — rbx::implementation::typed_holder<double>::singleton(void)
pub fn stub_365550() -> ! {
    todo!("0x365550 __ZN3rbx14implementation12typed_holderIdE9singletonEv")
}

#[doc(alias = "RBX::Camera::CameraMode const& rbx::any_cast<RBX::Camera::CameraMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x3caec0 — __ZN3rbx8any_castIRKN3RBX6Camera10CameraModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Camera::CameraMode const& rbx::any_cast<RBX::Camera::CameraMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_3caec0() -> ! {
    todo!("0x3caec0 __ZN3rbx8any_castIRKN3RBX6Camera10CameraModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraType>(RBX::Camera::CameraType const&)")]
// 0x3cb3a0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraType>(RBX::Camera::CameraType const&)
pub fn stub_3cb3a0() -> ! {
    todo!("0x3cb3a0 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraType>::singleton(void)")]
// 0x3cb3f0 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::Camera::CameraType>::singleton(void)
pub fn stub_3cb3f0() -> ! {
    todo!("0x3cb3f0 __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraType>::construct_func(char const*,char *)")]
// 0x3cb45c — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Camera::CameraType>::construct_func(char const*,char *)
pub fn stub_3cb45c() -> ! {
    todo!("0x3cb45c __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraType>::destruct_func(char *)")]
// 0x3cb468 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Camera::CameraType>::destruct_func(char *)
pub fn stub_3cb468() -> ! {
    todo!("0x3cb468 __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Camera::CameraPanMode * rbx::any_cast<RBX::Camera::CameraPanMode,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x3cc14c — __ZN3rbx8any_castIN3RBX6Camera13CameraPanModeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE — RBX::Camera::CameraPanMode * rbx::any_cast<RBX::Camera::CameraPanMode,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
pub fn stub_3cc14c() -> ! {
    todo!("0x3cc14c __ZN3rbx8any_castIN3RBX6Camera13CameraPanModeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Camera::CameraPanMode & rbx::any_cast<RBX::Camera::CameraPanMode &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x3cc1a4 — __ZN3rbx8any_castIRN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Camera::CameraPanMode & rbx::any_cast<RBX::Camera::CameraPanMode &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_3cc1a4() -> ! {
    todo!("0x3cc1a4 __ZN3rbx8any_castIRN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::resize(unsigned long,RBX::Camera::CameraPanMode)")]
// 0x3cc294 — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE6resizeEmS2_ — std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::resize(unsigned long,RBX::Camera::CameraPanMode)
pub fn stub_3cc294() -> ! {
    todo!("0x3cc294 __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::push_back(RBX::Camera::CameraPanMode const&)")]
// 0x3cc2c8 — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE9push_backERKS2_ — std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::push_back(RBX::Camera::CameraPanMode const&)
pub fn stub_3cc2c8() -> ! {
    todo!("0x3cc2c8 __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraPanMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::operator[](RBX::Name const* const&)")]
// 0x3cc2f0 — __ZNSt3mapIPKN3RBX4NameENS0_6Camera13CameraPanModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::Camera::CameraPanMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::operator[](RBX::Name const* const&)
pub fn stub_3cc2f0() -> ! {
    todo!("0x3cc2f0 __ZNSt3mapIPKN3RBX4NameENS0_6Camera13CameraPanModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")]
// 0x3cc348 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)
pub fn stub_3cc348() -> ! {
    todo!("0x3cc348 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")]
// 0x3cc3fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)
pub fn stub_3cc3fc() -> ! {
    todo!("0x3cc3fc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")]
// 0x3cc454 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)
pub fn stub_3cc454() -> ! {
    todo!("0x3cc454 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraPanMode*,std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>>,RBX::Camera::CameraPanMode const&)")]
// 0x3cc4bc — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraPanMode*,std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>>,RBX::Camera::CameraPanMode const&)
pub fn stub_3cc4bc() -> ! {
    todo!("0x3cc4bc __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_allocate(unsigned long)")]
// 0x3cc5a0 — __ZNSt12_Vector_baseIN3RBX6Camera13CameraPanModeESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_allocate(unsigned long)
pub fn stub_3cc5a0() -> ! {
    todo!("0x3cc5a0 __ZNSt12_Vector_baseIN3RBX6Camera13CameraPanModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Camera::CameraPanMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *>(RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *)")]
// 0x3cc5b8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera13CameraPanModeES6_EET0_T_S8_S7_ — RBX::Camera::CameraPanMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *>(RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *)
pub fn stub_3cc5b8() -> ! {
    todo!("0x3cc5b8 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera13CameraPanModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraPanMode*,std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>>,unsigned long,RBX::Camera::CameraPanMode const&)")]
// 0x3cc5f4 — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraPanMode*,std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>>,unsigned long,RBX::Camera::CameraPanMode const&)
pub fn stub_3cc5f4() -> ! {
    todo!("0x3cc5f4 __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::resize(unsigned long,RBX::Camera::CameraMode)")]
// 0x3cc784 — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE6resizeEmS2_ — std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::resize(unsigned long,RBX::Camera::CameraMode)
pub fn stub_3cc784() -> ! {
    todo!("0x3cc784 __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::push_back(RBX::Camera::CameraMode const&)")]
// 0x3cc7b8 — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE9push_backERKS2_ — std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::push_back(RBX::Camera::CameraMode const&)
pub fn stub_3cc7b8() -> ! {
    todo!("0x3cc7b8 __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::operator[](RBX::Name const* const&)")]
// 0x3cc7e0 — __ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::Camera::CameraMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::operator[](RBX::Name const* const&)
pub fn stub_3cc7e0() -> ! {
    todo!("0x3cc7e0 __ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")]
// 0x3cc838 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)
pub fn stub_3cc838() -> ! {
    todo!("0x3cc838 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")]
// 0x3cc8ec — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)
pub fn stub_3cc8ec() -> ! {
    todo!("0x3cc8ec __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")]
// 0x3cc944 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)
pub fn stub_3cc944() -> ! {
    todo!("0x3cc944 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraMode*,std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>>,RBX::Camera::CameraMode const&)")]
// 0x3cc9ac — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraMode*,std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>>,RBX::Camera::CameraMode const&)
pub fn stub_3cc9ac() -> ! {
    todo!("0x3cc9ac __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_allocate(unsigned long)")]
// 0x3cca90 — __ZNSt12_Vector_baseIN3RBX6Camera10CameraModeESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_allocate(unsigned long)
pub fn stub_3cca90() -> ! {
    todo!("0x3cca90 __ZNSt12_Vector_baseIN3RBX6Camera10CameraModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Camera::CameraMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraMode *,RBX::Camera::CameraMode *>(RBX::Camera::CameraMode *,RBX::Camera::CameraMode *,RBX::Camera::CameraMode *)")]
// 0x3ccaa8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraModeES6_EET0_T_S8_S7_ — RBX::Camera::CameraMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraMode *,RBX::Camera::CameraMode *>(RBX::Camera::CameraMode *,RBX::Camera::CameraMode *,RBX::Camera::CameraMode *)
pub fn stub_3ccaa8() -> ! {
    todo!("0x3ccaa8 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraMode*,std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>>,unsigned long,RBX::Camera::CameraMode const&)")]
// 0x3ccae4 — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraMode*,std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>>,unsigned long,RBX::Camera::CameraMode const&)
pub fn stub_3ccae4() -> ! {
    todo!("0x3ccae4 __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::resize(unsigned long,RBX::Camera::CameraType)")]
// 0x3ccc74 — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE6resizeEmS2_ — std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::resize(unsigned long,RBX::Camera::CameraType)
pub fn stub_3ccc74() -> ! {
    todo!("0x3ccc74 __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::push_back(RBX::Camera::CameraType const&)")]
// 0x3ccca8 — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE9push_backERKS2_ — std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::push_back(RBX::Camera::CameraType const&)
pub fn stub_3ccca8() -> ! {
    todo!("0x3ccca8 __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::operator[](RBX::Name const* const&)")]
// 0x3cccd0 — __ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::Camera::CameraType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::operator[](RBX::Name const* const&)
pub fn stub_3cccd0() -> ! {
    todo!("0x3cccd0 __ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)")]
// 0x3ccd28 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)
pub fn stub_3ccd28() -> ! {
    todo!("0x3ccd28 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)")]
// 0x3ccddc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)
pub fn stub_3ccddc() -> ! {
    todo!("0x3ccddc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)")]
// 0x3cce34 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)
pub fn stub_3cce34() -> ! {
    todo!("0x3cce34 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraType*,std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>>,RBX::Camera::CameraType const&)")]
// 0x3cce9c — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraType*,std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>>,RBX::Camera::CameraType const&)
pub fn stub_3cce9c() -> ! {
    todo!("0x3cce9c __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_allocate(unsigned long)")]
// 0x3ccf80 — __ZNSt12_Vector_baseIN3RBX6Camera10CameraTypeESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_allocate(unsigned long)
pub fn stub_3ccf80() -> ! {
    todo!("0x3ccf80 __ZNSt12_Vector_baseIN3RBX6Camera10CameraTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Camera::CameraType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraType *,RBX::Camera::CameraType *>(RBX::Camera::CameraType *,RBX::Camera::CameraType *,RBX::Camera::CameraType *)")]
// 0x3ccf98 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraTypeES6_EET0_T_S8_S7_ — RBX::Camera::CameraType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraType *,RBX::Camera::CameraType *>(RBX::Camera::CameraType *,RBX::Camera::CameraType *,RBX::Camera::CameraType *)
pub fn stub_3ccf98() -> ! {
    todo!("0x3ccf98 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraType*,std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>>,unsigned long,RBX::Camera::CameraType const&)")]
// 0x3ccfd4 — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraType*,std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>>,unsigned long,RBX::Camera::CameraType const&)
pub fn stub_3ccfd4() -> ! {
    todo!("0x3ccfd4 __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::Camera::~Camera()")]
// 0x3d16d8 — __ZN3RBX6CameraD2Ev — RBX::Camera::~Camera()
pub fn stub_3d16d8() -> ! {
    todo!("0x3d16d8 __ZN3RBX6CameraD2Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>> *)")]
// 0x3d194c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>> *)
pub fn stub_3d194c() -> ! {
    todo!("0x3d194c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>> *)")]
// 0x3d1974 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>> *)
pub fn stub_3d1974() -> ! {
    todo!("0x3d1974 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraType>> *)")]
// 0x3d199c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraType>> *)
pub fn stub_3d199c() -> ! {
    todo!("0x3d199c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "RBX::ChangeHistoryService::setEnabled(bool)")]
// 0x3d249c — __ZN3RBX20ChangeHistoryService10setEnabledEb — RBX::ChangeHistoryService::setEnabled(bool)
pub fn stub_3d249c() -> ! {
    todo!("0x3d249c __ZN3RBX20ChangeHistoryService10setEnabledEb")
}

#[doc(alias = "RBX::ChangeHistoryService::resetBaseWaypoint(void)")]
// 0x3d24b8 — __ZN3RBX20ChangeHistoryService17resetBaseWaypointEv — RBX::ChangeHistoryService::resetBaseWaypoint(void)
pub fn stub_3d24b8() -> ! {
    todo!("0x3d24b8 __ZN3RBX20ChangeHistoryService17resetBaseWaypointEv")
}

#[doc(alias = "RBX::ChangeHistoryService::play(void)")]
// 0x3d250c — __ZN3RBX20ChangeHistoryService4playEv — RBX::ChangeHistoryService::play(void)
pub fn stub_3d250c() -> ! {
    todo!("0x3d250c __ZN3RBX20ChangeHistoryService4playEv")
}

#[doc(alias = "RBX::ChangeHistoryService::unplay(void)")]
// 0x3d28cc — __ZN3RBX20ChangeHistoryService6unplayEv — RBX::ChangeHistoryService::unplay(void)
pub fn stub_3d28cc() -> ! {
    todo!("0x3d28cc __ZN3RBX20ChangeHistoryService6unplayEv")
}

#[doc(alias = "RBX::ChangeHistoryService::canUnplay2(void)")]
// 0x3d2c28 — __ZN3RBX20ChangeHistoryService10canUnplay2Ev — RBX::ChangeHistoryService::canUnplay2(void)
pub fn stub_3d2c28() -> ! {
    todo!("0x3d2c28 __ZN3RBX20ChangeHistoryService10canUnplay2Ev")
}

#[doc(alias = "RBX::ChangeHistoryService::canPlay2(void)")]
// 0x3d2ea0 — __ZN3RBX20ChangeHistoryService8canPlay2Ev — RBX::ChangeHistoryService::canPlay2(void)
pub fn stub_3d2ea0() -> ! {
    todo!("0x3d2ea0 __ZN3RBX20ChangeHistoryService8canPlay2Ev")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplayDelete(void)")]
// 0x3d3120 — __ZN3RBX20ChangeHistoryService4Item12unplayDeleteEv — RBX::ChangeHistoryService::Item::unplayDelete(void)
pub fn stub_3d3120() -> ! {
    todo!("0x3d3120 __ZN3RBX20ChangeHistoryService4Item12unplayDeleteEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplayClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// 0x3d3518 — __ZN3RBX20ChangeHistoryService4Item17unplayClusterDataERKSt4pairIjSt6vectorIjSaIjEEE — RBX::ChangeHistoryService::Item::unplayClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)
pub fn stub_3d3518() -> ! {
    todo!("0x3d3518 __ZN3RBX20ChangeHistoryService4Item17unplayClusterDataERKSt4pairIjSt6vectorIjSaIjEEE")
}

#[doc(alias = "RBX::ChangeHistoryService::ChangeHistoryService(void)")]
// 0x3d367c — __ZN3RBX20ChangeHistoryServiceC1Ev — RBX::ChangeHistoryService::ChangeHistoryService(void)
pub fn stub_3d367c() -> ! {
    todo!("0x3d367c __ZN3RBX20ChangeHistoryServiceC1Ev")
}

#[doc(alias = "RBX::ChangeHistoryService::ChangeHistoryService(void)")]
// 0x3d3680 — __ZN3RBX20ChangeHistoryServiceC2Ev — RBX::ChangeHistoryService::ChangeHistoryService(void)
pub fn stub_3d3680() -> ! {
    todo!("0x3d3680 __ZN3RBX20ChangeHistoryServiceC2Ev")
}

#[doc(alias = "RBX::ChangeHistoryService::~ChangeHistoryService()")]
// 0x3d39cc — __ZN3RBX20ChangeHistoryServiceD0Ev — RBX::ChangeHistoryService::~ChangeHistoryService()
pub fn stub_3d39cc() -> ! {
    todo!("0x3d39cc __ZN3RBX20ChangeHistoryServiceD0Ev")
}

#[doc(alias = "RBX::ChangeHistoryService::~ChangeHistoryService()")]
// 0x3d3a6c — __ZN3RBX20ChangeHistoryServiceD1Ev — RBX::ChangeHistoryService::~ChangeHistoryService()
pub fn stub_3d3a6c() -> ! {
    todo!("0x3d3a6c __ZN3RBX20ChangeHistoryServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ChangeHistoryService::~ChangeHistoryService()")]
// 0x3d3a70 — __ZThn32_N3RBX20ChangeHistoryServiceD0Ev — non-virtual thunk toRBX::ChangeHistoryService::~ChangeHistoryService()
pub fn stub_3d3a70() -> ! {
    todo!("0x3d3a70 __ZThn32_N3RBX20ChangeHistoryServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ChangeHistoryService::~ChangeHistoryService()")]
// 0x3d3a78 — __ZThn36_N3RBX20ChangeHistoryServiceD0Ev — non-virtual thunk toRBX::ChangeHistoryService::~ChangeHistoryService()
pub fn stub_3d3a78() -> ! {
    todo!("0x3d3a78 __ZThn36_N3RBX20ChangeHistoryServiceD0Ev")
}

#[doc(alias = "RBX::ChangeHistoryService::~ChangeHistoryService()")]
// 0x3d3a80 — __ZN3RBX20ChangeHistoryServiceD2Ev — RBX::ChangeHistoryService::~ChangeHistoryService()
pub fn stub_3d3a80() -> ! {
    todo!("0x3d3a80 __ZN3RBX20ChangeHistoryServiceD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ChangeHistoryService::~ChangeHistoryService()")]
// 0x3d3f08 — __ZThn32_N3RBX20ChangeHistoryServiceD1Ev — non-virtual thunk toRBX::ChangeHistoryService::~ChangeHistoryService()
pub fn stub_3d3f08() -> ! {
    todo!("0x3d3f08 __ZThn32_N3RBX20ChangeHistoryServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ChangeHistoryService::~ChangeHistoryService()")]
// 0x3d3f10 — __ZThn36_N3RBX20ChangeHistoryServiceD1Ev — non-virtual thunk toRBX::ChangeHistoryService::~ChangeHistoryService()
pub fn stub_3d3f10() -> ! {
    todo!("0x3d3f10 __ZThn36_N3RBX20ChangeHistoryServiceD1Ev")
}

#[doc(alias = "RBX::ChangeHistoryService::attach(void)")]
// 0x3d3f18 — __ZN3RBX20ChangeHistoryService6attachEv — RBX::ChangeHistoryService::attach(void)
pub fn stub_3d3f18() -> ! {
    todo!("0x3d3f18 __ZN3RBX20ChangeHistoryService6attachEv")
}

#[doc(alias = "RBX::ChangeHistoryService::dettach(void)")]
// 0x3d429c — __ZN3RBX20ChangeHistoryService7dettachEv — RBX::ChangeHistoryService::dettach(void)
pub fn stub_3d429c() -> ! {
    todo!("0x3d429c __ZN3RBX20ChangeHistoryService7dettachEv")
}

#[doc(alias = "RBX::ChangeHistoryService::getUnplayWaypoint(std::string &,int)const")]
// 0x3d434c — __ZNK3RBX20ChangeHistoryService17getUnplayWaypointERSsi — RBX::ChangeHistoryService::getUnplayWaypoint(std::string &,int)const
pub fn stub_3d434c() -> ! {
    todo!("0x3d434c __ZNK3RBX20ChangeHistoryService17getUnplayWaypointERSsi")
}

#[doc(alias = "RBX::ChangeHistoryService::requestWaypoint(char const*)")]
// 0x3d43c0 — __ZN3RBX20ChangeHistoryService15requestWaypointEPKc — RBX::ChangeHistoryService::requestWaypoint(char const*)
pub fn stub_3d43c0() -> ! {
    todo!("0x3d43c0 __ZN3RBX20ChangeHistoryService15requestWaypointEPKc")
}

#[doc(alias = "RBX::ChangeHistoryService::setWaypoint(char const*)")]
// 0x3d43e0 — __ZN3RBX20ChangeHistoryService11setWaypointEPKc — RBX::ChangeHistoryService::setWaypoint(char const*)
pub fn stub_3d43e0() -> ! {
    todo!("0x3d43e0 __ZN3RBX20ChangeHistoryService11setWaypointEPKc")
}

#[doc(alias = "RBX::ChangeHistoryService::mergeFirstTwoWaypoints(void)")]
// 0x3d45f0 — __ZN3RBX20ChangeHistoryService22mergeFirstTwoWaypointsEv — RBX::ChangeHistoryService::mergeFirstTwoWaypoints(void)
pub fn stub_3d45f0() -> ! {
    todo!("0x3d45f0 __ZN3RBX20ChangeHistoryService22mergeFirstTwoWaypointsEv")
}

#[doc(alias = "RBX::ChangeHistoryService::computeDataSize(void)")]
// 0x3d4de0 — __ZN3RBX20ChangeHistoryService15computeDataSizeEv — RBX::ChangeHistoryService::computeDataSize(void)
pub fn stub_3d4de0() -> ! {
    todo!("0x3d4de0 __ZN3RBX20ChangeHistoryService15computeDataSizeEv")
}

#[doc(alias = "RBX::ChangeHistoryService::trimWaypoints(void)")]
// 0x3d4e30 — __ZN3RBX20ChangeHistoryService13trimWaypointsEv — RBX::ChangeHistoryService::trimWaypoints(void)
pub fn stub_3d4e30() -> ! {
    todo!("0x3d4e30 __ZN3RBX20ChangeHistoryService13trimWaypointsEv")
}

#[doc(alias = "RBX::ChangeHistoryService::checkSettingWaypoint(void)")]
// 0x3d4f20 — __ZN3RBX20ChangeHistoryService20checkSettingWaypointEv — RBX::ChangeHistoryService::checkSettingWaypoint(void)
pub fn stub_3d4f20() -> ! {
    todo!("0x3d4f20 __ZN3RBX20ChangeHistoryService20checkSettingWaypointEv")
}

#[doc(alias = "RBX::ChangeHistoryService::clearWaypoints(void)")]
// 0x3d4fc4 — __ZN3RBX20ChangeHistoryService14clearWaypointsEv — RBX::ChangeHistoryService::clearWaypoints(void)
pub fn stub_3d4fc4() -> ! {
    todo!("0x3d4fc4 __ZN3RBX20ChangeHistoryService14clearWaypointsEv")
}

#[doc(alias = "RBX::ChangeHistoryService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x3d511c — __ZN3RBX20ChangeHistoryService17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::ChangeHistoryService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_3d511c() -> ! {
    todo!("0x3d511c __ZN3RBX20ChangeHistoryService17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "RBX::ChangeHistoryService::onRunTransition(RBX::RunTransition)")]
// 0x3d5358 — __ZN3RBX20ChangeHistoryService15onRunTransitionENS_13RunTransitionE — RBX::ChangeHistoryService::onRunTransition(RBX::RunTransition)
pub fn stub_3d5358() -> ! {
    todo!("0x3d5358 __ZN3RBX20ChangeHistoryService15onRunTransitionENS_13RunTransitionE")
}

#[doc(alias = "RBX::ChangeHistoryService::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0x3d582c — __ZN3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE — RBX::ChangeHistoryService::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)
pub fn stub_3d582c() -> ! {
    todo!("0x3d582c __ZN3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "non-virtual thunk toRBX::ChangeHistoryService::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0x3d59b8 — __ZThn96_N3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE — non-virtual thunk toRBX::ChangeHistoryService::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)
pub fn stub_3d59b8() -> ! {
    todo!("0x3d59b8 __ZThn96_N3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::play(void)")]
// 0x3d5fc0 — __ZN3RBX20ChangeHistoryService8Waypoint4playEv — RBX::ChangeHistoryService::Waypoint::play(void)
pub fn stub_3d5fc0() -> ! {
    todo!("0x3d5fc0 __ZN3RBX20ChangeHistoryService8Waypoint4playEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::selectModifiedParts(bool)")]
// 0x3d60f4 — __ZN3RBX20ChangeHistoryService8Waypoint19selectModifiedPartsEb — RBX::ChangeHistoryService::Waypoint::selectModifiedParts(bool)
pub fn stub_3d60f4() -> ! {
    todo!("0x3d60f4 __ZN3RBX20ChangeHistoryService8Waypoint19selectModifiedPartsEb")
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::unplay(void)")]
// 0x3d63dc — __ZN3RBX20ChangeHistoryService8Waypoint6unplayEv — RBX::ChangeHistoryService::Waypoint::unplay(void)
pub fn stub_3d63dc() -> ! {
    todo!("0x3d63dc __ZN3RBX20ChangeHistoryService8Waypoint6unplayEv")
}

#[doc(alias = "RBX::ChangeHistoryService::setRunWaypoint(void)")]
// 0x3d65c4 — __ZN3RBX20ChangeHistoryService14setRunWaypointEv — RBX::ChangeHistoryService::setRunWaypoint(void)
pub fn stub_3d65c4() -> ! {
    todo!("0x3d65c4 __ZN3RBX20ChangeHistoryService14setRunWaypointEv")
}

#[doc(alias = "RBX::ChangeHistoryService::requestWaypoint2(std::string)")]
// 0x3d6b10 — __ZN3RBX20ChangeHistoryService16requestWaypoint2ESs — RBX::ChangeHistoryService::requestWaypoint2(std::string)
pub fn stub_3d6b10() -> ! {
    todo!("0x3d6b10 __ZN3RBX20ChangeHistoryService16requestWaypoint2ESs")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::play(void)")]
// 0x3d6c14 — __ZN3RBX20ChangeHistoryService4Item4playEv — RBX::ChangeHistoryService::Item::play(void)
pub fn stub_3d6c14() -> ! {
    todo!("0x3d6c14 __ZN3RBX20ChangeHistoryService4Item4playEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::getCellData(unsigned int,unsigned int,unsigned int &)")]
// 0x3d6f18 — __ZN3RBX20ChangeHistoryService4Item11getCellDataEjjRj — RBX::ChangeHistoryService::Item::getCellData(unsigned int,unsigned int,unsigned int &)
pub fn stub_3d6f18() -> ! {
    todo!("0x3d6f18 __ZN3RBX20ChangeHistoryService4Item11getCellDataEjjRj")
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::absorb(RBX::ChangeHistoryService::Waypoint const*)")]
// 0x3d7150 — __ZN3RBX20ChangeHistoryService8Waypoint6absorbEPKS1_ — RBX::ChangeHistoryService::Waypoint::absorb(RBX::ChangeHistoryService::Waypoint const*)
pub fn stub_3d7150() -> ! {
    todo!("0x3d7150 __ZN3RBX20ChangeHistoryService8Waypoint6absorbEPKS1_")
}

#[doc(alias = "void delete_helper<RBX::ChangeHistoryService::Waypoint>(RBX::ChangeHistoryService::Waypoint *)")]
// 0x3d7214 — __Z13delete_helperIN3RBX20ChangeHistoryService8WaypointEEvPT_ — void delete_helper<RBX::ChangeHistoryService::Waypoint>(RBX::ChangeHistoryService::Waypoint *)
pub fn stub_3d7214() -> ! {
    todo!("0x3d7214 __Z13delete_helperIN3RBX20ChangeHistoryService8WaypointEEvPT_")
}

#[doc(alias = "std::list<RBX::ChangeHistoryService::Waypoint *,std::allocator<RBX::ChangeHistoryService::Waypoint *>>::erase(std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>,std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>)")]
// 0x3d72cc — __ZNSt4listIPN3RBX20ChangeHistoryService8WaypointESaIS3_EE5eraseESt14_List_iteratorIS3_ES7_ — std::list<RBX::ChangeHistoryService::Waypoint *,std::allocator<RBX::ChangeHistoryService::Waypoint *>>::erase(std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>,std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>)
pub fn stub_3d72cc() -> ! {
    todo!("0x3d72cc __ZNSt4listIPN3RBX20ChangeHistoryService8WaypointESaIS3_EE5eraseESt14_List_iteratorIS3_ES7_")
}

#[doc(alias = "ChangeHistoryStatsItem::create(RBX::ChangeHistoryService &)")]
// 0x3d758c — __ZN22ChangeHistoryStatsItem6createERN3RBX20ChangeHistoryServiceE — ChangeHistoryStatsItem::create(RBX::ChangeHistoryService &)
pub fn stub_3d758c() -> ! {
    todo!("0x3d758c __ZN22ChangeHistoryStatsItem6createERN3RBX20ChangeHistoryServiceE")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::recordCreate(void)")]
// 0x3d79c4 — __ZN3RBX20ChangeHistoryService4Item12recordCreateEv — RBX::ChangeHistoryService::Item::recordCreate(void)
pub fn stub_3d79c4() -> ! {
    todo!("0x3d79c4 __ZN3RBX20ChangeHistoryService4Item12recordCreateEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::addClusterDataFast(RBX::Voxel::Grid const*)")]
// 0x3d7b64 — __ZN3RBX20ChangeHistoryService4Item18addClusterDataFastEPKNS_5Voxel4GridE — RBX::ChangeHistoryService::Item::addClusterDataFast(RBX::Voxel::Grid const*)
pub fn stub_3d7b64() -> ! {
    todo!("0x3d7b64 __ZN3RBX20ChangeHistoryService4Item18addClusterDataFastEPKNS_5Voxel4GridE")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::recordDelete(void)")]
// 0x3d8108 — __ZN3RBX20ChangeHistoryService4Item12recordDeleteEv — RBX::ChangeHistoryService::Item::recordDelete(void)
pub fn stub_3d8108() -> ! {
    todo!("0x3d8108 __ZN3RBX20ChangeHistoryService4Item12recordDeleteEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplay(void)")]
// 0x3d8144 — __ZN3RBX20ChangeHistoryService4Item6unplayEv — RBX::ChangeHistoryService::Item::unplay(void)
pub fn stub_3d8144() -> ! {
    todo!("0x3d8144 __ZN3RBX20ChangeHistoryService4Item6unplayEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplay_CFrame(void)")]
// 0x3d8168 — __ZN3RBX20ChangeHistoryService4Item13unplay_CFrameEv — RBX::ChangeHistoryService::Item::unplay_CFrame(void)
pub fn stub_3d8168() -> ! {
    todo!("0x3d8168 __ZN3RBX20ChangeHistoryService4Item13unplay_CFrameEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ChangeHistoryService::RuntimeUndoBehavior>::construct_func(char const*,char *)")]
// 0x3d8460 — __ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::ChangeHistoryService::RuntimeUndoBehavior>::construct_func(char const*,char *)
pub fn stub_3d8460() -> ! {
    todo!("0x3d8460 __ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE14construct_funcEPKcPc")
}

#[doc(alias = "RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(void)const")]
// 0x3d95b0 — __ZNK3RBX15ServiceProvider6createINS_9SelectionEEEPT_v — RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(void)const
pub fn stub_3d95b0() -> ! {
    todo!("0x3d95b0 __ZNK3RBX15ServiceProvider6createINS_9SelectionEEEPT_v")
}

#[doc(alias = "non-virtual thunk toRBX::Motor::~Motor()")]
// 0x5a355c — __ZThn36_N3RBX5MotorD0Ev — non-virtual thunk toRBX::Motor::~Motor()
pub fn stub_5a355c() -> ! {
    todo!("0x5a355c __ZThn36_N3RBX5MotorD0Ev")
}

#[doc(alias = "RBX::Weld::~Weld()")]
// 0x5a3600 — __ZN3RBX4WeldD1Ev — RBX::Weld::~Weld()
pub fn stub_5a3600() -> ! {
    todo!("0x5a3600 __ZN3RBX4WeldD1Ev")
}

#[doc(alias = "RBX::Weld::~Weld()")]
// 0x5a3604 — __ZN3RBX4WeldD0Ev — RBX::Weld::~Weld()
pub fn stub_5a3604() -> ! {
    todo!("0x5a3604 __ZN3RBX4WeldD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Weld::~Weld()")]
// 0x5a36b4 — __ZThn32_N3RBX4WeldD1Ev — non-virtual thunk toRBX::Weld::~Weld()
pub fn stub_5a36b4() -> ! {
    todo!("0x5a36b4 __ZThn32_N3RBX4WeldD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Weld::~Weld()")]
// 0x5a36bc — __ZThn32_N3RBX4WeldD0Ev — non-virtual thunk toRBX::Weld::~Weld()
pub fn stub_5a36bc() -> ! {
    todo!("0x5a36bc __ZThn32_N3RBX4WeldD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Weld::~Weld()")]
// 0x5a3770 — __ZThn36_N3RBX4WeldD1Ev — non-virtual thunk toRBX::Weld::~Weld()
pub fn stub_5a3770() -> ! {
    todo!("0x5a3770 __ZThn36_N3RBX4WeldD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Weld::~Weld()")]
// 0x5a3778 — __ZThn36_N3RBX4WeldD0Ev — non-virtual thunk toRBX::Weld::~Weld()
pub fn stub_5a3778() -> ! {
    todo!("0x5a3778 __ZThn36_N3RBX4WeldD0Ev")
}
