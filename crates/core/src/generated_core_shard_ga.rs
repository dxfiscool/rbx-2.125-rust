//! core shard GA — 100 core stubs EA-sorted, 0xf442c4..0xf455f4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf442b4).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf442b4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// 0xf442c4 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const
pub fn stub_f442c4() {
    // IDA 0xf442c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0xf442d4 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const
pub fn stub_f442d4() {
    // IDA 0xf442d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::_M_clear(void)")]
// 0xf442e4 — j___ZNSt10_List_baseISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE8_M_clearEv
// was: std::_List_base<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>::_M_clear(void)
pub fn stub_f442e4() {
    // IDA 0xf442e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Deque_base<XmlElement *,std::allocator<XmlElement *>>::~_Deque_base()")]
// 0xf442f4 — j___ZNSt11_Deque_baseIP10XmlElementSaIS1_EED2Ev
pub fn stub_f442f4() {
    // IDA 0xf442f4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::operator[](std::string const&)")]
// 0xf44304 — j___ZNSt3mapISsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_
// was: std::map<std::string,boost::shared_ptr<RBX::KeyframeSequence>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::operator[](std::string const&)
pub fn stub_f44304() {
    // IDA 0xf44304: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>> const&)")]
// 0xf44314 — j___ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE14_M_create_nodeERKS7_
// was: std::list<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>> const&)
pub fn stub_f44314() {
    // IDA 0xf44314: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>::pair(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&)")]
// 0xf44324 — j___ZNSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEC2ERS0_RKS5_
// was: std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>::pair(std::string const&,boost::shared_ptr<RBX::KeyframeSequence> const&)
pub fn stub_f44324() {
    // IDA 0xf44324: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>::pair(std::string const&,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
// 0xf44334 — j___ZNSt4pairISsS_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEC2ERKSsRKS5_
// was: std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>::pair(std::string const&,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>> const&)
pub fn stub_f44334() {
    // IDA 0xf44334: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::lower_bound(std::string const&)")]
// 0xf44344 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::lower_bound(std::string const&)
pub fn stub_f44344() {
    // IDA 0xf44344: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
// 0xf44354 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>> const&)
pub fn stub_f44354() {
    // IDA 0xf44354: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
// 0xf44364 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>> const&)
pub fn stub_f44364() {
    // IDA 0xf44364: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
// 0xf44374 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>> const&)
pub fn stub_f44374() {
    // IDA 0xf44374: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::find(std::string const&)")]
// 0xf44384 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::find(std::string const&)
pub fn stub_f44384() {
    // IDA 0xf44384: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>> *)")]
// 0xf44394 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>> *)
pub fn stub_f44394() {
    // IDA 0xf44394: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
// 0xf443a4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>> const&)
pub fn stub_f443a4() {
    // IDA 0xf443a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::_M_allocate(unsigned long)")]
// 0xf443c4 — j___ZNSt12_Vector_baseIN3RBX6Legacy17SurfaceConstraintESaIS2_EE11_M_allocateEm
pub fn stub_f443c4() {
    // IDA 0xf443c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Legacy::SurfaceConstraint * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Legacy::SurfaceConstraint *,RBX::Legacy::SurfaceConstraint *>(RBX::Legacy::SurfaceConstraint *,RBX::Legacy::SurfaceConstraint *,RBX::Legacy::SurfaceConstraint *)")]
// 0xf443d4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Legacy17SurfaceConstraintES6_EET0_T_S8_S7_
pub fn stub_f443d4() {
    // IDA 0xf443d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Legacy::SurfaceConstraint,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::operator[](RBX::Name const* const&)")]
// 0xf443e4 — j___ZNSt3mapIPKN3RBX4NameENS0_6Legacy17SurfaceConstraintESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f443e4() {
    // IDA 0xf443e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Legacy::SurfaceConstraint*,std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>>,RBX::Legacy::SurfaceConstraint const&)")]
// 0xf443f4 — j___ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f443f4() {
    // IDA 0xf443f4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Legacy::SurfaceConstraint*,std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>>,unsigned long,RBX::Legacy::SurfaceConstraint const&)")]
// 0xf44404 — j___ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f44404() {
    // IDA 0xf44404: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::resize(unsigned long,RBX::Legacy::SurfaceConstraint)")]
// 0xf44414 — j___ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE6resizeEmS2_
pub fn stub_f44414() {
    // IDA 0xf44414: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::push_back(RBX::Legacy::SurfaceConstraint const&)")]
// 0xf44424 — j___ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE9push_backERKS2_
pub fn stub_f44424() {
    // IDA 0xf44424: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint> const&)")]
// 0xf44434 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f44434() {
    // IDA 0xf44434: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint> const&)")]
// 0xf44444 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f44444() {
    // IDA 0xf44444: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint> const&)")]
// 0xf44454 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f44454() {
    // IDA 0xf44454: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Sky> RBX::shared_from<RBX::Sky>(RBX::Sky*)")]
// 0xf44564 — j___ZN3RBX11shared_fromINS_3SkyEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Sky> RBX::shared_from<RBX::Sky>(RBX::Sky*)
pub fn stub_f44564() {
    // IDA 0xf44564: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lighting::~Lighting()")]
// 0xf44574 — j___ZN3RBX8LightingD2Ev
pub fn stub_f44574() {
    // IDA 0xf44574: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::basic_string<char,std::char_traits<char>,std::allocator<char>> boost::posix_time::to_simple_string_type<char>(boost::posix_time::time_duration)")]
// 0xf44584 — j___ZN5boost10posix_time21to_simple_string_typeIcEESbIT_St11char_traitsIS2_ESaIS2_EENS0_13time_durationE
pub fn stub_f44584() {
    // IDA 0xf44584: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Sky>::operator=(rbx_core::SharedPtr<RBX::Sky> const&)")]
// 0xf44594 — j___ZN5boost10shared_ptrIN3RBX3SkyEEaSERKS3_
// was: boost::shared_ptr<RBX::Sky>::operator=(boost::shared_ptr<RBX::Sky> const&)
pub fn stub_f44594() {
    // IDA 0xf44594: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::tokenizer_detail::traits_extension_details<std::char_traits<char>,1>::ispunct(char)")]
// 0xf445a4 — j___ZN5boost16tokenizer_detail24traits_extension_detailsISt11char_traitsIcELi1EE7ispunctEc
pub fn stub_f445a4() {
    // IDA 0xf445a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::tokenizer_detail::traits_extension_details<std::char_traits<char>,1>::isspace(char)")]
// 0xf445b4 — j___ZN5boost16tokenizer_detail24traits_extension_detailsISt11char_traitsIcELi1EE7isspaceEc
pub fn stub_f445b4() {
    // IDA 0xf445b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned short,char>(unsigned short &,char const*,char const*)")]
// 0xf445c4 — j___ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEtcEEbRT0_PKT1_S8_
pub fn stub_f445c4() {
    // IDA 0xf445c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long long,char>(unsigned long long &,char const*,char const*)")]
// 0xf445d4 — j___ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEycEEbRT0_PKT1_S8_
pub fn stub_f445d4() {
    // IDA 0xf445d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long long>(long long &)")]
// 0xf445e4 — j___ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIxEEbRT_
pub fn stub_f445e4() {
    // IDA 0xf445e4: boost tokenizer/lexical_cast/iostream buf. str::parse/Iterator/String — carrier no-op.
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned short>(unsigned short &)")]
// 0xf445f4 — j___ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedItEEbRT_
pub fn stub_f445f4() {
    // IDA 0xf445f4: boost tokenizer/lexical_cast/iostream buf. str::parse/Iterator/String — carrier no-op.
}

#[doc(alias = "boost::posix_time::time_duration boost::date_time::str_from_delimited_time_duration<boost::posix_time::time_duration,char>(std::basic_string<char,std::char_traits<char>,std::allocator<char>> const&)")]
// 0xf44604 — j___ZN5boost9date_time32str_from_delimited_time_durationINS_10posix_time13time_durationEcEET_RKSbIT0_St11char_traitsIS5_ESaIS5_EE
pub fn stub_f44604() {
    // IDA 0xf44604: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::char_separator<char,std::char_traits<char>>::is_kept(char)const")]
// 0xf44624 — j___ZNK5boost14char_separatorIcSt11char_traitsIcEE7is_keptEc
pub fn stub_f44624() {
    // IDA 0xf44624: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::date_time::int_adapter<long long>::mult_div_specials(int const&)const")]
// 0xf44634 — j___ZNK5boost9date_time11int_adapterIxE17mult_div_specialsERKi
pub fn stub_f44634() {
    // IDA 0xf44634: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::date_time::int_adapter<long long>::compare(boost::date_time::int_adapter<long long> const&)const")]
// 0xf44644 — j___ZNK5boost9date_time11int_adapterIxE7compareERKS2_
pub fn stub_f44644() {
    // IDA 0xf44644: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::date_time::int_adapter<long long>::operator*(int)const")]
// 0xf44654 — j___ZNK5boost9date_time11int_adapterIxEmlEi
pub fn stub_f44654() {
    // IDA 0xf44654: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>::_M_allocate(unsigned long)")]
// 0xf44934 — j___ZNSt12_Vector_baseIPN3RBX14IModelModifierESaIS2_EE11_M_allocateEm
pub fn stub_f44934() {
    // IDA 0xf44934: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>>,RBX::IModelModifier * const&)")]
// 0xf44954 — j___ZNSt6vectorIPN3RBX14IModelModifierESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f44954() {
    // IDA 0xf44954: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>::push_back(RBX::IModelModifier * const&)")]
// 0xf44964 — j___ZNSt6vectorIPN3RBX14IModelModifierESaIS2_EE9push_backERKS2_
pub fn stub_f44964() {
    // IDA 0xf44964: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>>,RBX::IModelModifier *>(__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>>,__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>>,RBX::IModelModifier * const&,std::random_access_iterator_tag)")]
// 0xf44974 — j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX14IModelModifierESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
pub fn stub_f44974() {
    // IDA 0xf44974: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "XmlElement::XmlElement<float>(RBX::Name const&,float)")]
// 0xf44af4 — j___ZN10XmlElementC2IfEERKN3RBX4NameET_
pub fn stub_f44af4() {
    // IDA 0xf44af4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SurfaceData::empty(void)")]
// 0xf44e94 — j___ZN3RBX11SurfaceData5emptyEv
pub fn stub_f44e94() {
    // IDA 0xf44e94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FWService>(void)")]
// 0xf44f64 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_9FWServiceEEEmv
pub fn stub_f44f64() {
    // IDA 0xf44f64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::InterpolatedCFrame::InterpolatedCFrame(void)")]
// 0xf44fa4 — j___ZN3RBX18InterpolatedCFrameC2Ev
pub fn stub_f44fa4() {
    // IDA 0xf44fa4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Body::getMeInParent(void)")]
// 0xf44fd4 — j___ZN3RBX4Body13getMeInParentEv
pub fn stub_f44fd4() {
    // IDA 0xf44fd4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Body::updatePV(void)")]
// 0xf44fe4 — j___ZN3RBX4Body8updatePVEv
pub fn stub_f44fe4() {
    // IDA 0xf44fe4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Dragger::dragSnap(void)")]
// 0xf45034 — j___ZN3RBX7Dragger8dragSnapEv
pub fn stub_f45034() {
    // IDA 0xf45034: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FWValue<RBX::BrickColor>::set(RBX::BrickColor const&,RBX::FWRef *)")]
// 0xf45054 — j___ZN3RBX7FWValueINS_10BrickColorEE3setERKS1_PNS_5FWRefE
pub fn stub_f45054() {
    // IDA 0xf45054: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FWValue<RBX::Material>::set(RBX::Material const&,RBX::FWRef *)")]
// 0xf45064 — j___ZN3RBX7FWValueINS_8MaterialEE3setERKS1_PNS_5FWRefE
pub fn stub_f45064() {
    // IDA 0xf45064: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FWValue<RBX::World *>::set(RBX::World * const&,RBX::FWRef *)")]
// 0xf45074 — j___ZN3RBX7FWValueIPNS_5WorldEE3setERKS2_PNS_5FWRefE
pub fn stub_f45074() {
    // IDA 0xf45074: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FWValue<float>::set(float const&,RBX::FWRef *)")]
// 0xf45084 — j___ZN3RBX7FWValueIfE3setERKfPNS_5FWRefE
pub fn stub_f45084() {
    // IDA 0xf45084: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IMoving::~IMoving()")]
// 0xf45094 — j___ZN3RBX7IMovingD2Ev
pub fn stub_f45094() {
    // IDA 0xf45094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Security::Context::ptr(void)")]
// 0xf45104 — j___ZN3RBX8Security7Context3ptrEv
pub fn stub_f45104() {
    // IDA 0xf45104: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::BrickColor>(RBX::BrickColor const&)")]
// 0xf451a4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10BrickColorEEERS3_RKT_
pub fn stub_f451a4() {
    // IDA 0xf451a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SystemAddress>(RBX::SystemAddress const&)")]
// 0xf451c4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SystemAddressEEERS3_RKT_
pub fn stub_f451c4() {
    // IDA 0xf451c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Faces>(RBX::Faces const&)")]
// 0xf451d4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5FacesEEERS3_RKT_
pub fn stub_f451d4() {
    // IDA 0xf451d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Material>(RBX::Material const&)")]
// 0xf451e4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8MaterialEEERS3_RKT_
pub fn stub_f451e4() {
    // IDA 0xf451e4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::NormalId>(RBX::NormalId const&)")]
// 0xf451f4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8NormalIdEEERS3_RKT_
pub fn stub_f451f4() {
    // IDA 0xf451f4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::BrickColor>::singleton(void)")]
// 0xf45214 — j___ZN3rbx14implementation12typed_holderIN3RBX10BrickColorEE9singletonEv
pub fn stub_f45214() {
    // IDA 0xf45214: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SystemAddress>::singleton(void)")]
// 0xf45234 — j___ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE9singletonEv
pub fn stub_f45234() {
    // IDA 0xf45234: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Faces>::singleton(void)")]
// 0xf45244 — j___ZN3rbx14implementation12typed_holderIN3RBX5FacesEE9singletonEv
pub fn stub_f45244() {
    // IDA 0xf45244: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Material>::singleton(void)")]
// 0xf45254 — j___ZN3rbx14implementation12typed_holderIN3RBX8MaterialEE9singletonEv
pub fn stub_f45254() {
    // IDA 0xf45254: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::NormalId>::singleton(void)")]
// 0xf45264 — j___ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE9singletonEv
pub fn stub_f45264() {
    // IDA 0xf45264: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<bool>::singleton(void)")]
// 0xf45284 — j___ZN3rbx14implementation12typed_holderIbE9singletonEv
pub fn stub_f45284() {
    // IDA 0xf45284: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::signals::connection::flogPrint(void)")]
// 0xf45294 — j___ZN3rbx7signals10connection9flogPrintEv
pub fn stub_f45294() {
    // IDA 0xf45294: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(bool)>::operator()(bool)")]
// 0xf452a4 — j___ZN3rbx7signals16signal_with_argsILi1EFvbEEclEb
pub fn stub_f452a4() {
    // IDA 0xf452a4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::disconnectAll(void)")]
// 0xf45304 — j___ZN3rbx7signals6signalIFvbEE13disconnectAllEv
pub fn stub_f45304() {
    // IDA 0xf45304: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::safe_static_do_get_mutex(void)")]
// 0xf45314 — j___ZN3rbx7signals6signalIFvbEE24safe_static_do_get_mutexEv
pub fn stub_f45314() {
    // IDA 0xf45314: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool)>::slot> &)")]
// 0xf45324 — j___ZN3rbx7signals6signalIFvbEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(bool)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot> &)
pub fn stub_f45324() {
    // IDA 0xf45324: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::on_error(std::exception &)")]
// 0xf45334 — j___ZN3rbx7signals6signalIFvbEE8on_errorERSt9exception
pub fn stub_f45334() {
    // IDA 0xf45334: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot> &)")]
// 0xf45344 — j___ZN3rbx7signals6signalIFvvEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(void)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(void)>::slot> &)
pub fn stub_f45344() {
    // IDA 0xf45344: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::safe_static_do_get_mutex(void)")]
// 0xf45354 — j___ZN3rbx7signals6signalIFvvEE4slot24safe_static_do_get_mutexEv
pub fn stub_f45354() {
    // IDA 0xf45354: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::function<void ()(void)>>(boost::function<void ()(void)> const&)")]
// 0xf45364 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_f45364() {
    // IDA 0xf45364: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::on_error(std::exception &)")]
// 0xf45374 — j___ZN3rbx7signals6signalIFvvEE8on_errorERSt9exception
pub fn stub_f45374() {
    // IDA 0xf45374: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "RBX::BrickColor const& rbx::any_cast<RBX::BrickColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf453a4 — j___ZN3rbx8any_castIRKN3RBX10BrickColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f453a4() {
    // IDA 0xf453a4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "RBX::SystemAddress const& rbx::any_cast<RBX::SystemAddress const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf453c4 — j___ZN3rbx8any_castIRKN3RBX13SystemAddressENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f453c4() {
    // IDA 0xf453c4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Faces const& rbx::any_cast<RBX::Faces const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf453d4 — j___ZN3rbx8any_castIRKN3RBX5FacesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f453d4() {
    // IDA 0xf453d4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Material const& rbx::any_cast<RBX::Material const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf453e4 — j___ZN3rbx8any_castIRKN3RBX8MaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f453e4() {
    // IDA 0xf453e4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::NormalId const& rbx::any_cast<RBX::NormalId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf453f4 — j___ZN3rbx8any_castIRKN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f453f4() {
    // IDA 0xf453f4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::IFWHolder>::operator=(rbx_core::SharedPtr<RBX::IFWHolder> const&)")]
// 0xf45474 — j___ZN5boost10shared_ptrIN3RBX9IFWHolderEEaSERKS3_
// was: boost::shared_ptr<RBX::IFWHolder>::operator=(boost::shared_ptr<RBX::IFWHolder> const&)
pub fn stub_f45474() {
    // IDA 0xf45474: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::unique_lock<boost::mutex>::lock(void)")]
// 0xf45494 — j___ZN5boost11unique_lockINS_5mutexEE4lockEv
pub fn stub_f45494() {
    // IDA 0xf45494: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool)>::slot> const&)")]
// 0xf454a4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot> const&)
pub fn stub_f454a4() {
    // IDA 0xf454a4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::RevoluteLink,208u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf454e4 — j___ZN5boost14singleton_poolIN3RBX12RevoluteLinkELj208ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f454e4() {
    // IDA 0xf454e4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf45514 — j___ZN5boost14singleton_poolIN3RBX16BallBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f45514() {
    // IDA 0xf45514: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::GeoPairConnector,264u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf45524 — j___ZN5boost14singleton_poolIN3RBX16GeoPairConnectorELj264ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f45524() {
    // IDA 0xf45524: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf45534 — j___ZN5boost14singleton_poolIN3RBX17BallBallConnectorELj272ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f45534() {
    // IDA 0xf45534: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf45544 — j___ZN5boost14singleton_poolIN3RBX18BallBlockConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f45544() {
    // IDA 0xf45544: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::destroy(void)")]
// 0xf45574 — j___ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE7destroyEv
pub fn stub_f45574() {
    // IDA 0xf45574: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::allocate(unsigned long)")]
// 0xf45584 — j___ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE8allocateEm
pub fn stub_f45584() {
    // IDA 0xf45584: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::throw_exception<rbx::bad_placement_any_cast>(rbx::bad_placement_any_cast const&)")]
// 0xf45594 — j___ZN5boost15throw_exceptionIN3rbx22bad_placement_any_castEEEvRKT_
pub fn stub_f45594() {
    // IDA 0xf45594: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::throw_exception<std::length_error>(std::length_error const&)")]
// 0xf455a4 — j___ZN5boost15throw_exceptionISt12length_errorEEvRKT_
pub fn stub_f455a4() {
    // IDA 0xf455a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_impl(boost::exception_detail::error_info_injector<boost::bad_weak_ptr> const&)")]
// 0xf455b4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEEC1ERKS4_
pub fn stub_f455b4() {
    // IDA 0xf455b4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::error_info_injector<std::length_error> const&)")]
// 0xf455c4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS4_
pub fn stub_f455c4() {
    // IDA 0xf455c4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
// 0xf455d4 — j___ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED2Ev
pub fn stub_f455d4() {
    // IDA 0xf455d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")]
// 0xf455e4 — j___ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED2Ev
pub fn stub_f455e4() {
    // IDA 0xf455e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::error_info_injector(std::length_error const&)")]
// 0xf455f4 — j___ZN5boost16exception_detail19error_info_injectorISt12length_errorEC2ERKS2_
pub fn stub_f455f4() {
    // IDA 0xf455f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
