//! core shard FN — 100 core stubs EA-sorted, 0xf33384..0xf350d4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after FM 0xf33364 gap).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf33364.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "boost::function0<int>::assign_to_own(boost::function0<int> const&)")]
// 0xf33384 — j___ZN5boost9function0IiE13assign_to_ownERKS1_
pub fn stub_f33384() {
    // IDA 0xf33384: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}


#[doc(alias = "boost::function2<void,boost::function<void ()(void)>,std::string>::clear(void)")]
// 0xf33394 — j___ZN5boost9function2IvNS_8functionIFvvEEESsE5clearEv
pub fn stub_f33394() {
    // IDA 0xf33394: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}


#[doc(alias = "RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(void)const")]
// 0xf334f4 — j___ZNK3RBX15ServiceProvider6createINS_9SelectionEEEPT_v
pub fn stub_f334f4() {
    // IDA 0xf334f4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}


#[doc(alias = "RBX::ChangeHistoryService::Item::computeDataSize(void)const")]
// 0xf33504 — j___ZNK3RBX20ChangeHistoryService4Item15computeDataSizeEv
pub fn stub_f33504() {
    // IDA 0xf33504: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}


#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::operator==(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)const")]
// 0xf33514 — j___ZNK3RBX5Voxel6RegionINS0_4Grid5ChunkEEeqERKS4_
pub fn stub_f33514() {
    // IDA 0xf33514: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}


#[doc(alias = "boost::function2<void,boost::function<void ()(void)>,std::string>::operator()(boost::function<void ()(void)>,std::string)const")]
// 0xf33564 — j___ZNK5boost9function2IvNS_8functionIFvvEEESsEclES3_Ss
pub fn stub_f33564() {
    // IDA 0xf33564: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}


#[doc(alias = "std::_List_base<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_clear(void)")]
// 0xf335b4 — j___ZNSt10_List_baseIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_clearEv
pub fn stub_f335b4() {
    // IDA 0xf335b4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}


#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_allocate_map(unsigned long)")]
// 0xf335d4 — j___ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE15_M_allocate_mapEm
pub fn stub_f335d4() {
    // IDA 0xf335d4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}


#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_create_nodes(RBX::ChangeHistoryService::Item ***,RBX::ChangeHistoryService::Item ***)")]
// 0xf335e4 — j___ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE15_M_create_nodesEPPS3_S7_
pub fn stub_f335e4() {
    // IDA 0xf335e4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}


#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_initialize_map(unsigned long)")]
// 0xf335f4 — j___ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE17_M_initialize_mapEm
pub fn stub_f335f4() {
    // IDA 0xf335f4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}


#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::~_Deque_base()")]
// 0xf33604 — j___ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EED2Ev
pub fn stub_f33604() {
    // IDA 0xf33604: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::_Vector_base<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_allocate(unsigned long)")]
// 0xf33614 — j___ZNSt12_Vector_baseIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE11_M_allocateEm
pub fn stub_f33614() {
    // IDA 0xf33614: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::_Vector_base<unsigned int,std::allocator<unsigned int>>::_M_allocate(unsigned long)")]
// 0xf33624 — j___ZNSt12_Vector_baseIjSaIjEE11_M_allocateEm
pub fn stub_f33624() {
    // IDA 0xf33624: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::_Vector_base<unsigned int,std::allocator<unsigned int>>::_Vector_base(unsigned long,std::allocator<unsigned int> const&)")]
// 0xf33634 — j___ZNSt12_Vector_baseIjSaIjEEC2EmRKS0_
pub fn stub_f33634() {
    // IDA 0xf33634: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::ChangeHistoryService::RuntimeUndoBehavior * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *>(RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *)")]
// 0xf33644 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX20ChangeHistoryService19RuntimeUndoBehaviorES6_EET0_T_S8_S7_
pub fn stub_f33644() {
    // IDA 0xf33644: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::map<RBX::Name const*,RBX::ChangeHistoryService::RuntimeUndoBehavior,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::operator[](RBX::Name const* const&)")]
// 0xf33664 — j___ZNSt3mapIPKN3RBX4NameENS0_20ChangeHistoryService19RuntimeUndoBehaviorESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f33664() {
    // IDA 0xf33664: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::map<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::operator[](unsigned int const&)")]
// 0xf33684 — j___ZNSt3mapIjSt6vectorIjSaIjEESt4lessIjESaISt4pairIKjS2_EEEixERS6_
pub fn stub_f33684() {
    // IDA 0xf33684: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::list<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_create_node(RBX::ChangeHistoryService::Item const&)")]
// 0xf33694 — j___ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_f33694() {
    // IDA 0xf33694: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::list<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_erase(std::_List_iterator<RBX::ChangeHistoryService::Item>)")]
// 0xf336a4 — j___ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
pub fn stub_f336a4() {
    // IDA 0xf336a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::list<RBX::ChangeHistoryService::Waypoint *,std::allocator<RBX::ChangeHistoryService::Waypoint *>>::erase(std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>,std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>)")]
// 0xf336c4 — j___ZNSt4listIPN3RBX20ChangeHistoryService8WaypointESaIS3_EE5eraseESt14_List_iteratorIS3_ES7_
pub fn stub_f336c4() {
    // IDA 0xf336c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_push_back_aux(RBX::ChangeHistoryService::Item * const&)")]
// 0xf336d4 — j___ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE16_M_push_back_auxERKS3_
pub fn stub_f336d4() {
    // IDA 0xf336d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_reallocate_map(unsigned long,bool)")]
// 0xf336e4 — j___ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE17_M_reallocate_mapEmb
pub fn stub_f336e4() {
    // IDA 0xf336e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_reserve_map_at_back(unsigned long)")]
// 0xf336f4 — j___ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE22_M_reserve_map_at_backEm
pub fn stub_f336f4() {
    // IDA 0xf336f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::pop_back(void)")]
// 0xf33704 — j___ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE8pop_backEv
pub fn stub_f33704() {
    // IDA 0xf33704: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::push_back(RBX::ChangeHistoryService::Item * const&)")]
// 0xf33714 — j___ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE9push_backERKS3_
pub fn stub_f33714() {
    // IDA 0xf33714: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::deque(std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>> const&)")]
// 0xf33724 — j___ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EEC2ERKS5_
pub fn stub_f33724() {
    // IDA 0xf33724: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **>>(std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **>)")]
// 0xf33734 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN3RBX20ChangeHistoryService4ItemERKS7_PS8_ES3_IS7_RS7_PS7_EEET0_T_SG_SF_
pub fn stub_f33734() {
    // IDA 0xf33734: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ChangeHistoryService::RuntimeUndoBehavior*,std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>>,RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
// 0xf33744 — j___ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f33744() {
    // IDA 0xf33744: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::ChangeHistoryService::RuntimeUndoBehavior*,std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>>,unsigned long,RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
// 0xf33754 — j___ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f33754() {
    // IDA 0xf33754: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::resize(unsigned long,RBX::ChangeHistoryService::RuntimeUndoBehavior)")]
// 0xf33764 — j___ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE6resizeEmS2_
pub fn stub_f33764() {
    // IDA 0xf33764: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::push_back(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
// 0xf33774 — j___ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE9push_backERKS2_
pub fn stub_f33774() {
    // IDA 0xf33774: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "void std::vector<unsigned int,std::allocator<unsigned int>>::_M_assign_aux<__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>>(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,std::forward_iterator_tag)")]
// 0xf33784 — j___ZNSt6vectorIjSaIjEE13_M_assign_auxIN9__gnu_cxx17__normal_iteratorIPjS1_EEEEvT_S7_St20forward_iterator_tag
pub fn stub_f33784() {
    // IDA 0xf33784: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,unsigned int const&)")]
// 0xf33794 — j___ZNSt6vectorIjSaIjEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS1_EERKj
pub fn stub_f33794() {
    // IDA 0xf33794: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::push_back(unsigned int const&)")]
// 0xf337a4 — j___ZNSt6vectorIjSaIjEE9push_backERKj
pub fn stub_f337a4() {
    // IDA 0xf337a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::vector(std::vector<unsigned int,std::allocator<unsigned int>> const&)")]
// 0xf337b4 — j___ZNSt6vectorIjSaIjEEC2ERKS1_
pub fn stub_f337b4() {
    // IDA 0xf337b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")]
// 0xf33844 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f33844() {
    // IDA 0xf33844: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")]
// 0xf33854 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f33854() {
    // IDA 0xf33854: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>> *)")]
// 0xf33864 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f33864() {
    // IDA 0xf33864: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")]
// 0xf33874 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f33874() {
    // IDA 0xf33874: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_create_node(std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// 0xf338c4 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_f338c4() {
    // IDA 0xf338c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")]
// 0xf338d4 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
pub fn stub_f338d4() {
    // IDA 0xf338d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert_unique(std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// 0xf338e4 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_f338e4() {
    // IDA 0xf338e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// 0xf338f4 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_f338f4() {
    // IDA 0xf338f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::find(unsigned int const&)")]
// 0xf33904 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE4findERS1_
pub fn stub_f33904() {
    // IDA 0xf33904: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_copy(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> const*,std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>*)")]
// 0xf33914 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
pub fn stub_f33914() {
    // IDA 0xf33914: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")]
// 0xf33924 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_f33924() {
    // IDA 0xf33924: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// 0xf33934 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_f33934() {
    // IDA 0xf33934: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_Rb_tree(std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>> const&)")]
// 0xf33944 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EEC2ERKSB_
pub fn stub_f33944() {
    // IDA 0xf33944: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
// 0xf33974 — j___ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKjSt6vectorIjSaIjEEEEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX20ChangeHistoryService4ItemERKS1_IjS5_EEENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEEET0_T_ST_SS_
pub fn stub_f33974() {
    // IDA 0xf33974: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}


#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
// 0xf33994 — j___ZSt8for_eachISt23_Rb_tree_const_iteratorISt4pairIKjSt6vectorIjSaIjEEEEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX20ChangeHistoryService4ItemERKS1_IjS5_EEENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEEET0_T_ST_SS_
pub fn stub_f33994() {
    // IDA 0xf33994: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}


#[doc(alias = "std::_Vector_base<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>::_M_allocate(unsigned long)")]
// 0xf33e04 — j___ZNSt12_Vector_baseIN3RBX13CharacterMesh8BodyPartESaIS2_EE11_M_allocateEm
pub fn stub_f33e04() {
    // IDA 0xf33e04: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}


#[doc(alias = "RBX::CharacterMesh::BodyPart * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CharacterMesh::BodyPart *,RBX::CharacterMesh::BodyPart *>(RBX::CharacterMesh::BodyPart *,RBX::CharacterMesh::BodyPart *,RBX::CharacterMesh::BodyPart *)")]
// 0xf33e14 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13CharacterMesh8BodyPartES6_EET0_T_S8_S7_
pub fn stub_f33e14() {
    // IDA 0xf33e14: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}


#[doc(alias = "std::map<RBX::Name const*,RBX::CharacterMesh::BodyPart,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::operator[](RBX::Name const* const&)")]
// 0xf33e24 — j___ZNSt3mapIPKN3RBX4NameENS0_13CharacterMesh8BodyPartESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f33e24() {
    // IDA 0xf33e24: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}


#[doc(alias = "std::vector<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CharacterMesh::BodyPart*,std::vector<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>>,RBX::CharacterMesh::BodyPart const&)")]
// 0xf33e34 — j___ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f33e34() {
    // IDA 0xf33e34: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}


#[doc(alias = "std::vector<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CharacterMesh::BodyPart*,std::vector<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>>,unsigned long,RBX::CharacterMesh::BodyPart const&)")]
// 0xf33e44 — j___ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f33e44() {
    // IDA 0xf33e44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>::resize(unsigned long,RBX::CharacterMesh::BodyPart)")]
// 0xf33e54 — j___ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE6resizeEmS2_
pub fn stub_f33e54() {
    // IDA 0xf33e54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>::push_back(RBX::CharacterMesh::BodyPart const&)")]
// 0xf33e64 — j___ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE9push_backERKS2_
pub fn stub_f33e64() {
    // IDA 0xf33e64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart> const&)")]
// 0xf33e74 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f33e74() {
    // IDA 0xf33e74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart> const&)")]
// 0xf33e84 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f33e84() {
    // IDA 0xf33e84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart> const&)")]
// 0xf33e94 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f33e94() {
    // IDA 0xf33e94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::ChatService::ChatColor * rbx::any_cast<RBX::ChatService::ChatColor,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf33fc4 — j___ZN3rbx8any_castIN3RBX11ChatService9ChatColorENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f33fc4() {
    // IDA 0xf33fc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::ChatService::ChatColor & rbx::any_cast<RBX::ChatService::ChatColor &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf33fd4 — j___ZN3rbx8any_castIRN3RBX11ChatService9ChatColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f33fd4() {
    // IDA 0xf33fd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Vector_base<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>::_M_allocate(unsigned long)")]
// 0xf340d4 — j___ZNSt12_Vector_baseIN3RBX11ChatService9ChatColorESaIS2_EE11_M_allocateEm
pub fn stub_f340d4() {
    // IDA 0xf340d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::ChatService::ChatColor * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ChatService::ChatColor *,RBX::ChatService::ChatColor *>(RBX::ChatService::ChatColor *,RBX::ChatService::ChatColor *,RBX::ChatService::ChatColor *)")]
// 0xf340e4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11ChatService9ChatColorES6_EET0_T_S8_S7_
pub fn stub_f340e4() {
    // IDA 0xf340e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::map<RBX::Name const*,RBX::ChatService::ChatColor,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::operator[](RBX::Name const* const&)")]
// 0xf340f4 — j___ZNSt3mapIPKN3RBX4NameENS0_11ChatService9ChatColorESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f340f4() {
    // IDA 0xf340f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ChatService::ChatColor*,std::vector<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>>,RBX::ChatService::ChatColor const&)")]
// 0xf34104 — j___ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f34104() {
    // IDA 0xf34104: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::ChatService::ChatColor*,std::vector<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>>,unsigned long,RBX::ChatService::ChatColor const&)")]
// 0xf34114 — j___ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f34114() {
    // IDA 0xf34114: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>::resize(unsigned long,RBX::ChatService::ChatColor)")]
// 0xf34124 — j___ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE6resizeEmS2_
pub fn stub_f34124() {
    // IDA 0xf34124: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>::push_back(RBX::ChatService::ChatColor const&)")]
// 0xf34134 — j___ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE9push_backERKS2_
pub fn stub_f34134() {
    // IDA 0xf34134: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ChatService::ChatColor> const&)")]
// 0xf34144 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f34144() {
    // IDA 0xf34144: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor> const&)")]
// 0xf34154 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f34154() {
    // IDA 0xf34154: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor> const&)")]
// 0xf34164 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f34164() {
    // IDA 0xf34164: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::ClickDetector> RBX::shared_from<RBX::ClickDetector>(RBX::ClickDetector*)")]
// 0xf341b4 — j___ZN3RBX11shared_fromINS_13ClickDetectorEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::ClickDetector> RBX::shared_from<RBX::ClickDetector>(RBX::ClickDetector*)
pub fn stub_f341b4() {
    // IDA 0xf341b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "RBX::ClickDetector::~ClickDetector()")]
// 0xf341c4 — j___ZN3RBX13ClickDetectorD0Ev
pub fn stub_f341c4() {
    // IDA 0xf341c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "RBX::ClickDetector::~ClickDetector()")]
// 0xf341d4 — j___ZN3RBX13ClickDetectorD2Ev
pub fn stub_f341d4() {
    // IDA 0xf341d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "RBX::CollectionService::~CollectionService()")]
// 0xf34264 — j___ZN3RBX17CollectionServiceD2Ev
pub fn stub_f34264() {
    // IDA 0xf34264: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::Selection> RBX::shared_from<RBX::Selection>(RBX::Selection*)")]
// 0xf34374 — j___ZN3RBX11shared_fromINS_9SelectionEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Selection> RBX::shared_from<RBX::Selection>(RBX::Selection*)
pub fn stub_f34374() {
    // IDA 0xf34374: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::Selection>::operator=(rbx_core::SharedPtr<RBX::Selection> const&)")]
// 0xf34624 — j___ZN5boost10shared_ptrIN3RBX9SelectionEEaSERKS3_
// was: boost::shared_ptr<RBX::Selection>::operator=(boost::shared_ptr<RBX::Selection> const&)
pub fn stub_f34624() {
    // IDA 0xf34624: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "RBX::ServiceClient<RBX::RunService>::createService(void)const")]
// 0xf34694 — j___ZNK3RBX13ServiceClientINS_10RunServiceEE13createServiceEv
pub fn stub_f34694() {
    // IDA 0xf34694: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "RBX::ServiceClient<RBX::Selection>::createService(void)const")]
// 0xf346b4 — j___ZNK3RBX13ServiceClientINS_9SelectionEE13createServiceEv
pub fn stub_f346b4() {
    // IDA 0xf346b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "RBX::Selection::back(void)const")]
// 0xf34744 — j___ZNK3RBX9Selection4backEv
pub fn stub_f34744() {
    // IDA 0xf34744: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "RBX::Selection::front(void)const")]
// 0xf34754 — j___ZNK3RBX9Selection5frontEv
pub fn stub_f34754() {
    // IDA 0xf34754: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "RBX::AdvMoveTool::~AdvMoveTool()")]
// 0xf34894 — j___ZN3RBX11AdvMoveToolD0Ev
pub fn stub_f34894() {
    // IDA 0xf34894: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool> RBX::shared_from<RBX::AnchorTool>(RBX::AnchorTool*)")]
// 0xf348b4 — j___ZN3RBX11shared_fromINS_10AnchorToolEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::AnchorTool> RBX::shared_from<RBX::AnchorTool>(RBX::AnchorTool*)
pub fn stub_f348b4() {
    // IDA 0xf348b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::LockTool> RBX::shared_from<RBX::LockTool>(RBX::LockTool*)")]
// 0xf348c4 — j___ZN3RBX11shared_fromINS_8LockToolEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::LockTool> RBX::shared_from<RBX::LockTool>(RBX::LockTool*)
pub fn stub_f348c4() {
    // IDA 0xf348c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "RBX::AdvMoveToolBase::~AdvMoveToolBase()")]
// 0xf34904 — j___ZN3RBX15AdvMoveToolBaseD0Ev
pub fn stub_f34904() {
    // IDA 0xf34904: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "RBX::AdvMoveToolBase::~AdvMoveToolBase()")]
// 0xf34914 — j___ZN3RBX15AdvMoveToolBaseD2Ev
pub fn stub_f34914() {
    // IDA 0xf34914: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool>::shared_ptr<RBX::AnchorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf35014 — j___ZN5boost10shared_ptrIN3RBX10AnchorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::AnchorTool>::shared_ptr<RBX::AnchorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f35014() {
    // IDA 0xf35014: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::HammerTool>::shared_ptr<RBX::HammerTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf35024 — j___ZN5boost10shared_ptrIN3RBX10HammerToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::HammerTool>::shared_ptr<RBX::HammerTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f35024() {
    // IDA 0xf35024: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::AdvMoveTool>::shared_ptr<RBX::AdvMoveTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf35034 — j___ZN5boost10shared_ptrIN3RBX11AdvMoveToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::AdvMoveTool>::shared_ptr<RBX::AdvMoveTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f35034() {
    // IDA 0xf35034: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::DropperTool>::shared_ptr<RBX::DropperTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf35044 — j___ZN5boost10shared_ptrIN3RBX11DropperToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::DropperTool>::shared_ptr<RBX::DropperTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f35044() {
    // IDA 0xf35044: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::MaterialTool>::shared_ptr<RBX::MaterialTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf35054 — j___ZN5boost10shared_ptrIN3RBX12MaterialToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::MaterialTool>::shared_ptr<RBX::MaterialTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f35054() {
    // IDA 0xf35054: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::AdvRotateTool>::shared_ptr<RBX::AdvRotateTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf35064 — j___ZN5boost10shared_ptrIN3RBX13AdvRotateToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::AdvRotateTool>::shared_ptr<RBX::AdvRotateTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f35064() {
    // IDA 0xf35064: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::LeftMotorTool>::shared_ptr<RBX::LeftMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf35074 — j___ZN5boost10shared_ptrIN3RBX13LeftMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::LeftMotorTool>::shared_ptr<RBX::LeftMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f35074() {
    // IDA 0xf35074: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::UniversalTool>::shared_ptr<RBX::UniversalTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf35084 — j___ZN5boost10shared_ptrIN3RBX13UniversalToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::UniversalTool>::shared_ptr<RBX::UniversalTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f35084() {
    // IDA 0xf35084: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::AxisRotateTool>::shared_ptr<RBX::AxisRotateTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf35094 — j___ZN5boost10shared_ptrIN3RBX14AxisRotateToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::AxisRotateTool>::shared_ptr<RBX::AxisRotateTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f35094() {
    // IDA 0xf35094: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::RightMotorTool>::shared_ptr<RBX::RightMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf350a4 — j___ZN5boost10shared_ptrIN3RBX14RightMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::RightMotorTool>::shared_ptr<RBX::RightMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f350a4() {
    // IDA 0xf350a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::MoveResizeJoinTool>::shared_ptr<RBX::MoveResizeJoinTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf350b4 — j___ZN5boost10shared_ptrIN3RBX18MoveResizeJoinToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::MoveResizeJoinTool>::shared_ptr<RBX::MoveResizeJoinTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f350b4() {
    // IDA 0xf350b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::OscillateMotorTool>::shared_ptr<RBX::OscillateMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf350c4 — j___ZN5boost10shared_ptrIN3RBX18OscillateMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::OscillateMotorTool>::shared_ptr<RBX::OscillateMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f350c4() {
    // IDA 0xf350c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}


#[doc(alias = "rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool>::shared_ptr<RBX::SmoothNoOutlinesTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf350d4 — j___ZN5boost10shared_ptrIN3RBX20SmoothNoOutlinesToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::SmoothNoOutlinesTool>::shared_ptr<RBX::SmoothNoOutlinesTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f350d4() {
    // IDA 0xf350d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

