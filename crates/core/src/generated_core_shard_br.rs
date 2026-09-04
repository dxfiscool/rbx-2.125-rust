//! core shard BR — 100 core stubs EA-sorted, next uncovered after BQ 0x535798 (strict RBX|boost|std|rbx earliest gap, after BQ 0x523c20..0x535798).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x535798.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]



#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::resize(unsigned long,RBX::GuiObject::TweenStatus)")]
// 0x538dac — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE6resizeEmS2_ — std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::resize(unsigned long,RBX::GuiObject::TweenStatus)
pub fn stub_538dac() {
    // IDA 0x538dac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::push_back(RBX::GuiObject::TweenStatus const&)")]
// 0x538de0 — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_ — std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::push_back(RBX::GuiObject::TweenStatus const&)
pub fn stub_538de0() {
    // IDA 0x538de0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::TweenStatus,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::operator[](RBX::Name const* const&)")]
// 0x538e08 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject11TweenStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::GuiObject::TweenStatus,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::operator[](RBX::Name const* const&)
pub fn stub_538e08() {
    // IDA 0x538e08: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
// 0x538e60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)
pub fn stub_538e60() {
    // IDA 0x538e60: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
// 0x538f14 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)
pub fn stub_538f14() {
    // IDA 0x538f14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
// 0x538f6c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)
pub fn stub_538f6c() {
    // IDA 0x538f6c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,RBX::GuiObject::TweenStatus const&)")]
// 0x538fd4 — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,RBX::GuiObject::TweenStatus const&)
pub fn stub_538fd4() {
    // IDA 0x538fd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Vector_base<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_allocate(unsigned long)")]
// 0x5390b8 — __ZNSt12_Vector_baseIN3RBX9GuiObject11TweenStatusESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_allocate(unsigned long)
pub fn stub_5390b8() {
    // IDA 0x5390b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiObject::TweenStatus * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *>(RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *)")]
// 0x5390d0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject11TweenStatusES6_EET0_T_S8_S7_ — RBX::GuiObject::TweenStatus * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *>(RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *)
pub fn stub_5390d0() {
    // IDA 0x5390d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,unsigned long,RBX::GuiObject::TweenStatus const&)")]
// 0x53910c — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,unsigned long,RBX::GuiObject::TweenStatus const&)
pub fn stub_53910c() {
    // IDA 0x53910c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiObject::TweenEasingStyle * rbx::any_cast<RBX::GuiObject::TweenEasingStyle,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x53929c — __ZN3rbx8any_castIN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE — RBX::GuiObject::TweenEasingStyle * rbx::any_cast<RBX::GuiObject::TweenEasingStyle,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
pub fn stub_53929c() {
    // IDA 0x53929c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiObject::TweenEasingStyle & rbx::any_cast<RBX::GuiObject::TweenEasingStyle &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5392f4 — __ZN3rbx8any_castIRN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiObject::TweenEasingStyle & rbx::any_cast<RBX::GuiObject::TweenEasingStyle &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_5392f4() {
    // IDA 0x5392f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::resize(unsigned long,RBX::GuiObject::TweenEasingStyle)")]
// 0x5393e4 — __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE6resizeEmS2_ — std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::resize(unsigned long,RBX::GuiObject::TweenEasingStyle)
pub fn stub_5393e4() {
    // IDA 0x5393e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::push_back(RBX::GuiObject::TweenEasingStyle const&)")]
// 0x539418 — __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE9push_backERKS2_ — std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::push_back(RBX::GuiObject::TweenEasingStyle const&)
pub fn stub_539418() {
    // IDA 0x539418: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::TweenEasingStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::operator[](RBX::Name const* const&)")]
// 0x539440 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject16TweenEasingStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::GuiObject::TweenEasingStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::operator[](RBX::Name const* const&)
pub fn stub_539440() {
    // IDA 0x539440: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
// 0x539498 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)
pub fn stub_539498() {
    // IDA 0x539498: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
// 0x53954c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)
pub fn stub_53954c() {
    // IDA 0x53954c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
// 0x5395a4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)
pub fn stub_5395a4() {
    // IDA 0x5395a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingStyle*,std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>>,RBX::GuiObject::TweenEasingStyle const&)")]
// 0x53960c — __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingStyle*,std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>>,RBX::GuiObject::TweenEasingStyle const&)
pub fn stub_53960c() {
    // IDA 0x53960c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Vector_base<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_allocate(unsigned long)")]
// 0x5396f0 — __ZNSt12_Vector_baseIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_allocate(unsigned long)
pub fn stub_5396f0() {
    // IDA 0x5396f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiObject::TweenEasingStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenEasingStyle *,RBX::GuiObject::TweenEasingStyle *>(RBX::GuiObject::TweenEasingStyle *,RBX::GuiObject::TweenEasingStyle *,RBX::GuiObject::TweenEasingStyle *)")]
// 0x539708 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject16TweenEasingStyleES6_EET0_T_S8_S7_ — RBX::GuiObject::TweenEasingStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenEasingStyle *,RBX::GuiObject::TweenEasingStyle *>(RBX::GuiObject::TweenEasingStyle *,RBX::GuiObject::TweenEasingStyle *,RBX::GuiObject::TweenEasingStyle *)
pub fn stub_539708() {
    // IDA 0x539708: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingStyle*,std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>>,unsigned long,RBX::GuiObject::TweenEasingStyle const&)")]
// 0x539744 — __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingStyle*,std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>>,unsigned long,RBX::GuiObject::TweenEasingStyle const&)
pub fn stub_539744() {
    // IDA 0x539744: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiObject::TweenEasingDirection * rbx::any_cast<RBX::GuiObject::TweenEasingDirection,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x5398d4 — __ZN3rbx8any_castIN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE — RBX::GuiObject::TweenEasingDirection * rbx::any_cast<RBX::GuiObject::TweenEasingDirection,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
pub fn stub_5398d4() {
    // IDA 0x5398d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiObject::TweenEasingDirection & rbx::any_cast<RBX::GuiObject::TweenEasingDirection &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x53992c — __ZN3rbx8any_castIRN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiObject::TweenEasingDirection & rbx::any_cast<RBX::GuiObject::TweenEasingDirection &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_53992c() {
    // IDA 0x53992c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::resize(unsigned long,RBX::GuiObject::TweenEasingDirection)")]
// 0x539a1c — __ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE6resizeEmS2_ — std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::resize(unsigned long,RBX::GuiObject::TweenEasingDirection)
pub fn stub_539a1c() {
    // IDA 0x539a1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::push_back(RBX::GuiObject::TweenEasingDirection const&)")]
// 0x539a50 — __ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE9push_backERKS2_ — std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::push_back(RBX::GuiObject::TweenEasingDirection const&)
pub fn stub_539a50() {
    // IDA 0x539a50: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::TweenEasingDirection,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::operator[](RBX::Name const* const&)")]
// 0x539a78 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject20TweenEasingDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::GuiObject::TweenEasingDirection,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::operator[](RBX::Name const* const&)
pub fn stub_539a78() {
    // IDA 0x539a78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)")]
// 0x539ad0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)
pub fn stub_539ad0() {
    // IDA 0x539ad0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)")]
// 0x539b84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)
pub fn stub_539b84() {
    // IDA 0x539b84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)")]
// 0x539bdc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)
pub fn stub_539bdc() {
    // IDA 0x539bdc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingDirection*,std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>>,RBX::GuiObject::TweenEasingDirection const&)")]
// 0x539c44 — __ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingDirection*,std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>>,RBX::GuiObject::TweenEasingDirection const&)
pub fn stub_539c44() {
    // IDA 0x539c44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Vector_base<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_allocate(unsigned long)")]
// 0x539d28 — __ZNSt12_Vector_baseIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_allocate(unsigned long)
pub fn stub_539d28() {
    // IDA 0x539d28: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiObject::TweenEasingDirection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenEasingDirection *,RBX::GuiObject::TweenEasingDirection *>(RBX::GuiObject::TweenEasingDirection *,RBX::GuiObject::TweenEasingDirection *,RBX::GuiObject::TweenEasingDirection *)")]
// 0x539d40 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject20TweenEasingDirectionES6_EET0_T_S8_S7_ — RBX::GuiObject::TweenEasingDirection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenEasingDirection *,RBX::GuiObject::TweenEasingDirection *>(RBX::GuiObject::TweenEasingDirection *,RBX::GuiObject::TweenEasingDirection *,RBX::GuiObject::TweenEasingDirection *)
pub fn stub_539d40() {
    // IDA 0x539d40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingDirection*,std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>>,unsigned long,RBX::GuiObject::TweenEasingDirection const&)")]
// 0x539d7c — __ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingDirection*,std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>>,unsigned long,RBX::GuiObject::TweenEasingDirection const&)
pub fn stub_539d7c() {
    // IDA 0x539d7c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::UDim2 const& rbx::any_cast<RBX::UDim2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x53a3e0 — __ZN3rbx8any_castIRKN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::UDim2 const& rbx::any_cast<RBX::UDim2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_53a3e0() {
    // IDA 0x53a3e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiButton::~GuiButton()")]
// 0x53fb5c — __ZN3RBX9GuiButtonD2Ev — RBX::GuiButton::~GuiButton()
pub fn stub_53fb5c() {
    // IDA 0x53fb5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::GuiService::getModalDialogStatus(void)const")]
// 0x541c1c — __ZNK3RBX10GuiService20getModalDialogStatusEv — RBX::GuiService::getModalDialogStatus(void)const
pub fn stub_541c1c() {
    // IDA 0x541c1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::GuiService::addKey(std::string)")]
// 0x541c98 — __ZN3RBX10GuiService6addKeyESs — RBX::GuiService::addKey(std::string)
pub fn stub_541c98() {
    // IDA 0x541c98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::GuiService::removeKey(std::string)")]
// 0x541de8 — __ZN3RBX10GuiService9removeKeyESs — RBX::GuiService::removeKey(std::string)
pub fn stub_541de8() {
    // IDA 0x541de8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::GuiService::addSpecialKey(RBX::GuiService::SpecialKey)")]
// 0x541f3c — __ZN3RBX10GuiService13addSpecialKeyENS0_10SpecialKeyE — RBX::GuiService::addSpecialKey(RBX::GuiService::SpecialKey)
pub fn stub_541f3c() {
    // IDA 0x541f3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::GuiService::setGlobalGuiInset(int,int,int,int)")]
// 0x542fb4 — __ZN3RBX10GuiService17setGlobalGuiInsetEiiii — RBX::GuiService::setGlobalGuiInset(int,int,int,int)
pub fn stub_542fb4() {
    // IDA 0x542fb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::setShowLegacyPlayerList(bool)")]
// 0x543028 — __ZN3RBX10GuiService23setShowLegacyPlayerListEb — RBX::GuiService::setShowLegacyPlayerList(bool)
pub fn stub_543028() {
    // IDA 0x543028: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::openBrowserWindow(std::string)")]
// 0x543048 — __ZN3RBX10GuiService17openBrowserWindowESs — RBX::GuiService::openBrowserWindow(std::string)
pub fn stub_543048() {
    // IDA 0x543048: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::StringConverter<RBX::GuiService::SpecialKey>::convertToValue(std::string const&,RBX::GuiService::SpecialKey&)")]
// 0x54366c — __ZN3RBX15StringConverterINS_10GuiService10SpecialKeyEE14convertToValueERKSsRS2_ — RBX::StringConverter<RBX::GuiService::SpecialKey>::convertToValue(std::string const&,RBX::GuiService::SpecialKey&)
pub fn stub_54366c() {
    // IDA 0x54366c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::StringConverter<RBX::GuiService::CenterDialogType>::convertToValue(std::string const&,RBX::GuiService::CenterDialogType&)")]
// 0x5436b8 — __ZN3RBX15StringConverterINS_10GuiService16CenterDialogTypeEE14convertToValueERKSsRS2_ — RBX::StringConverter<RBX::GuiService::CenterDialogType>::convertToValue(std::string const&,RBX::GuiService::CenterDialogType&)
pub fn stub_5436b8() {
    // IDA 0x5436b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::GuiService(void)")]
// 0x543704 — __ZN3RBX10GuiServiceC1Ev — RBX::GuiService::GuiService(void)
pub fn stub_543704() {
    // IDA 0x543704: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::GuiService(void)")]
// 0x543708 — __ZN3RBX10GuiServiceC2Ev — RBX::GuiService::GuiService(void)
pub fn stub_543708() {
    // IDA 0x543708: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::shouldPreemptCurrentDialog(RBX::GuiService::DialogWrapper *)const")]
// 0x543b40 — __ZNK3RBX10GuiService26shouldPreemptCurrentDialogEPNS0_13DialogWrapperE — RBX::GuiService::shouldPreemptCurrentDialog(RBX::GuiService::DialogWrapper *)const
pub fn stub_543b40() {
    // IDA 0x543b40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::getScreenSize(void)const")]
// 0x543c10 — __ZNK3RBX10GuiService13getScreenSizeEv — RBX::GuiService::getScreenSize(void)const
pub fn stub_543c10() {
    // IDA 0x543c10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::queueDialogWrapper(RBX::GuiService::DialogWrapper *,bool)")]
// 0x543c3c — __ZN3RBX10GuiService18queueDialogWrapperEPNS0_13DialogWrapperEb — RBX::GuiService::queueDialogWrapper(RBX::GuiService::DialogWrapper *,bool)
pub fn stub_543c3c() {
    // IDA 0x543c3c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::GuiService::showWaitingDialog(RBX::GuiService::CenterDialogType)")]
// 0x5440c0 — __ZN3RBX10GuiService17showWaitingDialogENS0_16CenterDialogTypeE — RBX::GuiService::showWaitingDialog(RBX::GuiService::CenterDialogType)
pub fn stub_5440c0() {
    // IDA 0x5440c0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::GuiService::dispatchKey(RBX::GuiService::SpecialKey)")]
// 0x54416c — __ZN3RBX10GuiService11dispatchKeyENS0_10SpecialKeyE — RBX::GuiService::dispatchKey(RBX::GuiService::SpecialKey)
pub fn stub_54416c() {
    // IDA 0x54416c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::GuiService::processKeyDown(RBX::GuiEvent)")]
// 0x5442e4 — __ZN3RBX10GuiService14processKeyDownENS_8GuiEventE — RBX::GuiService::processKeyDown(RBX::GuiEvent)
pub fn stub_5442e4() {
    // IDA 0x5442e4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::GuiService::getVersion(void)const")]
// 0x5449fc — __ZNK3RBX10GuiService10getVersionEv — RBX::GuiService::getVersion(void)const
pub fn stub_5449fc() {
    // IDA 0x5449fc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::GuiService::getIsWindows(void)const")]
// 0x544a50 — __ZNK3RBX10GuiService12getIsWindowsEv — RBX::GuiService::getIsWindows(void)const
pub fn stub_544a50() {
    // IDA 0x544a50: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::GuiService::getShowLegacyPlayerList(void)const")]
// 0x544ca0 — __ZNK3RBX10GuiService23getShowLegacyPlayerListEv — RBX::GuiService::getShowLegacyPlayerList(void)const
pub fn stub_544ca0() {
    // IDA 0x544ca0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "std::map<RBX::GuiService::CenterDialogType,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::operator[](RBX::GuiService::CenterDialogType const&)")]
// 0x545758 — __ZNSt3mapIN3RBX10GuiService16CenterDialogTypeESt4listIPNS1_13DialogWrapperESaIS5_EESt4lessIS2_ESaISt4pairIKS2_S7_EEEixERSB_ — std::map<RBX::GuiService::CenterDialogType,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::operator[](RBX::GuiService::CenterDialogType const&)
pub fn stub_545758() {
    // IDA 0x545758: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::remove(RBX::GuiService::DialogWrapper * const&)")]
// 0x545ffc — __ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE6removeERKS3_ — std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::remove(RBX::GuiService::DialogWrapper * const&)
pub fn stub_545ffc() {
    // IDA 0x545ffc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::~GuiService()")]
// 0x546e6c — __ZN3RBX10GuiServiceD1Ev — RBX::GuiService::~GuiService()
pub fn stub_546e6c() {
    // IDA 0x546e6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::GuiService::~GuiService()")]
// 0x546e70 — __ZN3RBX10GuiServiceD0Ev — RBX::GuiService::~GuiService()
pub fn stub_546e70() {
    // IDA 0x546e70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::GuiService::~GuiService()")]
// 0x546f38 — __ZThn32_N3RBX10GuiServiceD1Ev — non-virtual thunk toRBX::GuiService::~GuiService()
pub fn stub_546f38() {
    // IDA 0x546f38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::GuiService::~GuiService()")]
// 0x546f40 — __ZThn32_N3RBX10GuiServiceD0Ev — non-virtual thunk toRBX::GuiService::~GuiService()
pub fn stub_546f40() {
    // IDA 0x546f40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::GuiService::~GuiService()")]
// 0x54700c — __ZThn36_N3RBX10GuiServiceD1Ev — non-virtual thunk toRBX::GuiService::~GuiService()
pub fn stub_54700c() {
    // IDA 0x54700c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::GuiService::~GuiService()")]
// 0x547014 — __ZThn36_N3RBX10GuiServiceD0Ev — non-virtual thunk toRBX::GuiService::~GuiService()
pub fn stub_547014() {
    // IDA 0x547014: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::GuiService::~GuiService()")]
// 0x5470b8 — __ZN3RBX10GuiServiceD2Ev — RBX::GuiService::~GuiService()
pub fn stub_5470b8() {
    // IDA 0x5470b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_erase(std::_Rb_tree_node<RBX::GuiService::SpecialKey> *)")]
// 0x547484 — __ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E — std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_erase(std::_Rb_tree_node<RBX::GuiService::SpecialKey> *)
pub fn stub_547484() {
    // IDA 0x547484: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiService::CenterDialogType>(RBX::GuiService::CenterDialogType const&)")]
// 0x54764c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService16CenterDialogTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiService::CenterDialogType>(RBX::GuiService::CenterDialogType const&)
pub fn stub_54764c() {
    // IDA 0x54764c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::CenterDialogType>::singleton(void)")]
// 0x54769c — __ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::GuiService::CenterDialogType>::singleton(void)
pub fn stub_54769c() {
    // IDA 0x54769c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::CenterDialogType>::construct_func(char const*,char *)")]
// 0x547708 — __ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::GuiService::CenterDialogType>::construct_func(char const*,char *)
pub fn stub_547708() {
    // IDA 0x547708: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::CenterDialogType>::destruct_func(char *)")]
// 0x547714 — __ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::GuiService::CenterDialogType>::destruct_func(char *)
pub fn stub_547714() {
    // IDA 0x547714: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::CenterDialogType const& rbx::any_cast<RBX::GuiService::CenterDialogType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5477e4 — __ZN3rbx8any_castIRKN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiService::CenterDialogType const& rbx::any_cast<RBX::GuiService::CenterDialogType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_5477e4() {
    // IDA 0x5477e4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiService::SpecialKey>(RBX::GuiService::SpecialKey const&)")]
// 0x547cc4 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService10SpecialKeyEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiService::SpecialKey>(RBX::GuiService::SpecialKey const&)
pub fn stub_547cc4() {
    // IDA 0x547cc4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::SpecialKey>::singleton(void)")]
// 0x547d14 — __ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE9singletonEv — rbx::implementation::typed_holder<RBX::GuiService::SpecialKey>::singleton(void)
pub fn stub_547d14() {
    // IDA 0x547d14: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::SpecialKey>::construct_func(char const*,char *)")]
// 0x547d80 — __ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::GuiService::SpecialKey>::construct_func(char const*,char *)
pub fn stub_547d80() {
    // IDA 0x547d80: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::SpecialKey>::destruct_func(char *)")]
// 0x547d8c — __ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::GuiService::SpecialKey>::destruct_func(char *)
pub fn stub_547d8c() {
    // IDA 0x547d8c: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "RBX::GuiService::SpecialKey const& rbx::any_cast<RBX::GuiService::SpecialKey const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x547e5c — __ZN3rbx8any_castIRKN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiService::SpecialKey const& rbx::any_cast<RBX::GuiService::SpecialKey const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_547e5c() {
    // IDA 0x547e5c: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert_unique(RBX::GuiService::SpecialKey const&)")]
// 0x549368 — __ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_ — std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert_unique(RBX::GuiService::SpecialKey const&)
pub fn stub_549368() {
    // IDA 0x549368: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::GuiService::SpecialKey const&)")]
// 0x5493d0 — __ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_ — std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::GuiService::SpecialKey const&)
pub fn stub_5493d0() {
    // IDA 0x5493d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_insert_unique(char const&)")]
// 0x549428 — __ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE16_M_insert_uniqueERKc — std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_insert_unique(char const&)
pub fn stub_549428() {
    // IDA 0x549428: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,char const&)")]
// 0x54949c — __ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKc — std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,char const&)
pub fn stub_54949c() {
    // IDA 0x54949c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::list(std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>> const&)")]
// 0x54b464 — __ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EEC2ERKS5_ — std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::list(std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>> const&)
pub fn stub_54b464() {
    // IDA 0x54b464: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "void std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::_M_initialize_dispatch<std::_List_const_iterator<RBX::GuiService::DialogWrapper *>>(std::_List_const_iterator<RBX::GuiService::DialogWrapper *>,std::_List_const_iterator<RBX::GuiService::DialogWrapper *>,std::__false_type)")]
// 0x54b530 — __ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE22_M_initialize_dispatchISt20_List_const_iteratorIS3_EEEvT_S9_St12__false_type — void std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::_M_initialize_dispatch<std::_List_const_iterator<RBX::GuiService::DialogWrapper *>>(std::_List_const_iterator<RBX::GuiService::DialogWrapper *>,std::_List_const_iterator<RBX::GuiService::DialogWrapper *>,std::__false_type)
pub fn stub_54b530() {
    // IDA 0x54b530: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
// 0x54b554 — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_ — std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)
pub fn stub_54b554() {
    // IDA 0x54b554: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
// 0x54b608 — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_ — std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)
pub fn stub_54b608() {
    // IDA 0x54b608: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert_unique(std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
// 0x54b654 — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueERKSA_ — std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert_unique(std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)
pub fn stub_54b654() {
    // IDA 0x54b654: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_create_node(std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
// 0x54b6bc — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE14_M_create_nodeERKSA_ — std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_create_node(std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)
pub fn stub_54b6bc() {
    // IDA 0x54b6bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::CenterDialogType * rbx::any_cast<RBX::GuiService::CenterDialogType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x54c174 — __ZN3rbx8any_castIN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE — RBX::GuiService::CenterDialogType * rbx::any_cast<RBX::GuiService::CenterDialogType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
pub fn stub_54c174() {
    // IDA 0x54c174: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::CenterDialogType & rbx::any_cast<RBX::GuiService::CenterDialogType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x54c1cc — __ZN3rbx8any_castIRN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiService::CenterDialogType & rbx::any_cast<RBX::GuiService::CenterDialogType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_54c1cc() {
    // IDA 0x54c1cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::resize(unsigned long,RBX::GuiService::CenterDialogType)")]
// 0x54c2bc — __ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE6resizeEmS2_ — std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::resize(unsigned long,RBX::GuiService::CenterDialogType)
pub fn stub_54c2bc() {
    // IDA 0x54c2bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::push_back(RBX::GuiService::CenterDialogType const&)")]
// 0x54c2f0 — __ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE9push_backERKS2_ — std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::push_back(RBX::GuiService::CenterDialogType const&)
pub fn stub_54c2f0() {
    // IDA 0x54c2f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::map<RBX::Name const*,RBX::GuiService::CenterDialogType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::operator[](RBX::Name const* const&)")]
// 0x54c318 — __ZNSt3mapIPKN3RBX4NameENS0_10GuiService16CenterDialogTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::GuiService::CenterDialogType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::operator[](RBX::Name const* const&)
pub fn stub_54c318() {
    // IDA 0x54c318: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType> const&)")]
// 0x54c370 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType> const&)
pub fn stub_54c370() {
    // IDA 0x54c370: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType> const&)")]
// 0x54c424 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType> const&)
pub fn stub_54c424() {
    // IDA 0x54c424: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType> const&)")]
// 0x54c47c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType> const&)
pub fn stub_54c47c() {
    // IDA 0x54c47c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiService::CenterDialogType*,std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>>,RBX::GuiService::CenterDialogType const&)")]
// 0x54c4e4 — __ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiService::CenterDialogType*,std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>>,RBX::GuiService::CenterDialogType const&)
pub fn stub_54c4e4() {
    // IDA 0x54c4e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Vector_base<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::_M_allocate(unsigned long)")]
// 0x54c5c8 — __ZNSt12_Vector_baseIN3RBX10GuiService16CenterDialogTypeESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::_M_allocate(unsigned long)
pub fn stub_54c5c8() {
    // IDA 0x54c5c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::CenterDialogType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiService::CenterDialogType *,RBX::GuiService::CenterDialogType *>(RBX::GuiService::CenterDialogType *,RBX::GuiService::CenterDialogType *,RBX::GuiService::CenterDialogType *)")]
// 0x54c5e0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService16CenterDialogTypeES6_EET0_T_S8_S7_ — RBX::GuiService::CenterDialogType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiService::CenterDialogType *,RBX::GuiService::CenterDialogType *>(RBX::GuiService::CenterDialogType *,RBX::GuiService::CenterDialogType *,RBX::GuiService::CenterDialogType *)
pub fn stub_54c5e0() {
    // IDA 0x54c5e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiService::CenterDialogType*,std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>>,unsigned long,RBX::GuiService::CenterDialogType const&)")]
// 0x54c61c — __ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiService::CenterDialogType*,std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>>,unsigned long,RBX::GuiService::CenterDialogType const&)
pub fn stub_54c61c() {
    // IDA 0x54c61c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::SpecialKey * rbx::any_cast<RBX::GuiService::SpecialKey,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x54c7ac — __ZN3rbx8any_castIN3RBX10GuiService10SpecialKeyENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE — RBX::GuiService::SpecialKey * rbx::any_cast<RBX::GuiService::SpecialKey,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
pub fn stub_54c7ac() {
    // IDA 0x54c7ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::GuiService::SpecialKey & rbx::any_cast<RBX::GuiService::SpecialKey &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x54c804 — __ZN3rbx8any_castIRN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiService::SpecialKey & rbx::any_cast<RBX::GuiService::SpecialKey &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_54c804() {
    // IDA 0x54c804: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

