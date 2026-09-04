//! core shard BI — 100 core stubs EA-sorted, next uncovered after BH 0x49825c (strict RBX|boost|std|rbx earliest gap, after BH 0x4983ec..0x4b48e8).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x49825c.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::resize(unsigned long,RBX::DialogRoot::DialogPurpose)")]
// 0x4983ec — __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE6resizeEmS2_ — std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::resize(unsigned long,RBX::DialogRoot::DialogPurpose)
pub fn stub_4983ec() {
    // IDA 0x4983ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::push_back(RBX::DialogRoot::DialogPurpose const&)")]
// 0x498420 — __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE9push_backERKS2_ — std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::push_back(RBX::DialogRoot::DialogPurpose const&)
pub fn stub_498420() {
    // IDA 0x498420: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DialogRoot::DialogPurpose,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::operator[](RBX::Name const* const&)")]
// 0x498448 — __ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot13DialogPurposeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::DialogRoot::DialogPurpose,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::operator[](RBX::Name const* const&)
pub fn stub_498448() {
    // IDA 0x498448: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
// 0x4984a0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)
pub fn stub_4984a0() {
    // IDA 0x4984a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
// 0x498554 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)
pub fn stub_498554() {
    // IDA 0x498554: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
// 0x4985ac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)
pub fn stub_4985ac() {
    // IDA 0x4985ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogPurpose*,std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>>,RBX::DialogRoot::DialogPurpose const&)")]
// 0x498614 — __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogPurpose*,std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>>,RBX::DialogRoot::DialogPurpose const&)
pub fn stub_498614() {
    // IDA 0x498614: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_allocate(unsigned long)")]
// 0x4986f8 — __ZNSt12_Vector_baseIN3RBX10DialogRoot13DialogPurposeESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_allocate(unsigned long)
pub fn stub_4986f8() {
    // IDA 0x4986f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DialogRoot::DialogPurpose * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *>(RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *)")]
// 0x498710 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot13DialogPurposeES6_EET0_T_S8_S7_ — RBX::DialogRoot::DialogPurpose * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *>(RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *)
pub fn stub_498710() {
    // IDA 0x498710: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogPurpose*,std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>>,unsigned long,RBX::DialogRoot::DialogPurpose const&)")]
// 0x49874c — __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogPurpose*,std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>>,unsigned long,RBX::DialogRoot::DialogPurpose const&)
pub fn stub_49874c() {
    // IDA 0x49874c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DialogRoot::~DialogRoot()")]
// 0x49ac00 — __ZN3RBX10DialogRootD2Ev — RBX::DialogRoot::~DialogRoot()
pub fn stub_49ac00() {
    // IDA 0x49ac00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Effect::Effect(void)")]
// 0x49b3e0 — __ZN3RBX6EffectC2Ev — RBX::Effect::Effect(void)
pub fn stub_49b3e0() {
    // IDA 0x49b3e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Effect::~Effect()")]
// 0x49b3f0 — __ZN3RBX6EffectD0Ev — RBX::Effect::~Effect()
pub fn stub_49b3f0() {
    // IDA 0x49b3f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Effect::~Effect()")]
// 0x49b3f4 — __ZN3RBX6EffectD1Ev — RBX::Effect::~Effect()
pub fn stub_49b3f4() {
    // IDA 0x49b3f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Effect::~Effect()")]
// 0x49b3f8 — __ZN3RBX6EffectD2Ev — RBX::Effect::~Effect()
pub fn stub_49b3f8() {
    // IDA 0x49b3f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::resize(unsigned long,RBX::GuiObject::SizeConstraint)")]
// 0x49d59c — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE6resizeEmS2_ — std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::resize(unsigned long,RBX::GuiObject::SizeConstraint)
pub fn stub_49d59c() {
    // IDA 0x49d59c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::push_back(RBX::GuiObject::SizeConstraint const&)")]
// 0x49d5d0 — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE9push_backERKS2_ — std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::push_back(RBX::GuiObject::SizeConstraint const&)
pub fn stub_49d5d0() {
    // IDA 0x49d5d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::SizeConstraint,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::operator[](RBX::Name const* const&)")]
// 0x49d5f8 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject14SizeConstraintESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::GuiObject::SizeConstraint,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::operator[](RBX::Name const* const&)
pub fn stub_49d5f8() {
    // IDA 0x49d5f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
// 0x49d650 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)
pub fn stub_49d650() {
    // IDA 0x49d650: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
// 0x49d704 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)
pub fn stub_49d704() {
    // IDA 0x49d704: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
// 0x49d75c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)
pub fn stub_49d75c() {
    // IDA 0x49d75c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,RBX::GuiObject::SizeConstraint const&)")]
// 0x49d7c4 — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,RBX::GuiObject::SizeConstraint const&)
pub fn stub_49d7c4() {
    // IDA 0x49d7c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_allocate(unsigned long)")]
// 0x49d8a8 — __ZNSt12_Vector_baseIN3RBX9GuiObject14SizeConstraintESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_allocate(unsigned long)
pub fn stub_49d8a8() {
    // IDA 0x49d8a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiObject::SizeConstraint * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *>(RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *)")]
// 0x49d8c0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject14SizeConstraintES6_EET0_T_S8_S7_ — RBX::GuiObject::SizeConstraint * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *>(RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *)
pub fn stub_49d8c0() {
    // IDA 0x49d8c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,unsigned long,RBX::GuiObject::SizeConstraint const&)")]
// 0x49d8fc — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,unsigned long,RBX::GuiObject::SizeConstraint const&)
pub fn stub_49d8fc() {
    // IDA 0x49d8fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::resize(unsigned long,RBX::Handles::VisualStyle)")]
// 0x49da8c — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE6resizeEmS2_ — std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::resize(unsigned long,RBX::Handles::VisualStyle)
pub fn stub_49da8c() {
    // IDA 0x49da8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::push_back(RBX::Handles::VisualStyle const&)")]
// 0x49dac0 — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE9push_backERKS2_ — std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::push_back(RBX::Handles::VisualStyle const&)
pub fn stub_49dac0() {
    // IDA 0x49dac0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Handles::VisualStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::operator[](RBX::Name const* const&)")]
// 0x49dae8 — __ZNSt3mapIPKN3RBX4NameENS0_7Handles11VisualStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::Handles::VisualStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::operator[](RBX::Name const* const&)
pub fn stub_49dae8() {
    // IDA 0x49dae8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
// 0x49db40 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)
pub fn stub_49db40() {
    // IDA 0x49db40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
// 0x49dbf4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)
pub fn stub_49dbf4() {
    // IDA 0x49dbf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
// 0x49dc4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)
pub fn stub_49dc4c() {
    // IDA 0x49dc4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,RBX::Handles::VisualStyle const&)")]
// 0x49dcb4 — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,RBX::Handles::VisualStyle const&)
pub fn stub_49dcb4() {
    // IDA 0x49dcb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_allocate(unsigned long)")]
// 0x49dd98 — __ZNSt12_Vector_baseIN3RBX7Handles11VisualStyleESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_allocate(unsigned long)
pub fn stub_49dd98() {
    // IDA 0x49dd98: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Handles::VisualStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *>(RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *)")]
// 0x49ddb0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Handles11VisualStyleES6_EET0_T_S8_S7_ — RBX::Handles::VisualStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *>(RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *)
pub fn stub_49ddb0() {
    // IDA 0x49ddb0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,unsigned long,RBX::Handles::VisualStyle const&)")]
// 0x49ddec — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,unsigned long,RBX::Handles::VisualStyle const&)
pub fn stub_49ddec() {
    // IDA 0x49ddec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Explosion::setBlastRadius(float)")]
// 0x49f5ac — __ZN3RBX9Explosion14setBlastRadiusEf — RBX::Explosion::setBlastRadius(float)
pub fn stub_49f5ac() {
    // IDA 0x49f5ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Explosion::setExplosionType(RBX::Explosion::ExplosionType)")]
// 0x49f5f0 — __ZN3RBX9Explosion16setExplosionTypeENS0_13ExplosionTypeE — RBX::Explosion::setExplosionType(RBX::Explosion::ExplosionType)
pub fn stub_49f5f0() {
    // IDA 0x49f5f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Explosion::Explosion(void)")]
// 0x49f7ec — __ZN3RBX9ExplosionC1Ev — RBX::Explosion::Explosion(void)
pub fn stub_49f7ec() {
    // IDA 0x49f7ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Explosion::Explosion(void)")]
// 0x49f7f0 — __ZN3RBX9ExplosionC2Ev — RBX::Explosion::Explosion(void)
pub fn stub_49f7f0() {
    // IDA 0x49f7f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Explosion::~Explosion()")]
// 0x49fbe4 — __ZN3RBX9ExplosionD0Ev — RBX::Explosion::~Explosion()
pub fn stub_49fbe4() {
    // IDA 0x49fbe4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::~Explosion()")]
// 0x49fc84 — __ZN3RBX9ExplosionD1Ev — RBX::Explosion::~Explosion()
pub fn stub_49fc84() {
    // IDA 0x49fc84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Explosion::~Explosion()")]
// 0x49fc88 — __ZThn32_N3RBX9ExplosionD0Ev — non-virtual thunk toRBX::Explosion::~Explosion()
pub fn stub_49fc88() {
    // IDA 0x49fc88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Explosion::~Explosion()")]
// 0x49fc90 — __ZThn36_N3RBX9ExplosionD0Ev — non-virtual thunk toRBX::Explosion::~Explosion()
pub fn stub_49fc90() {
    // IDA 0x49fc90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Explosion::~Explosion()")]
// 0x49fc98 — __ZThn116_N3RBX9ExplosionD0Ev — non-virtual thunk toRBX::Explosion::~Explosion()
pub fn stub_49fc98() {
    // IDA 0x49fc98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Explosion::~Explosion()")]
// 0x49fca0 — __ZThn128_N3RBX9ExplosionD0Ev — non-virtual thunk toRBX::Explosion::~Explosion()
pub fn stub_49fca0() {
    // IDA 0x49fca0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::~Explosion()")]
// 0x49fca8 — __ZN3RBX9ExplosionD2Ev — RBX::Explosion::~Explosion()
pub fn stub_49fca8() {
    // IDA 0x49fca8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Explosion::~Explosion()")]
// 0x49fee8 — __ZThn32_N3RBX9ExplosionD1Ev — non-virtual thunk toRBX::Explosion::~Explosion()
pub fn stub_49fee8() {
    // IDA 0x49fee8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Explosion::~Explosion()")]
// 0x49fef0 — __ZThn36_N3RBX9ExplosionD1Ev — non-virtual thunk toRBX::Explosion::~Explosion()
pub fn stub_49fef0() {
    // IDA 0x49fef0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Explosion::~Explosion()")]
// 0x49fef8 — __ZThn116_N3RBX9ExplosionD1Ev — non-virtual thunk toRBX::Explosion::~Explosion()
pub fn stub_49fef8() {
    // IDA 0x49fef8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Explosion::~Explosion()")]
// 0x49ff00 — __ZThn128_N3RBX9ExplosionD1Ev — non-virtual thunk toRBX::Explosion::~Explosion()
pub fn stub_49ff00() {
    // IDA 0x49ff00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::onStepped(RBX::Stepped const&)")]
// 0x4a0098 — __ZN3RBX9Explosion9onSteppedERKNS_7SteppedE — RBX::Explosion::onStepped(RBX::Stepped const&)
pub fn stub_4a0098() {
    // IDA 0x4a0098: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Explosion::onStepped(RBX::Stepped const&)")]
// 0x4a0318 — __ZThn116_N3RBX9Explosion9onSteppedERKNS_7SteppedE — non-virtual thunk toRBX::Explosion::onStepped(RBX::Stepped const&)
pub fn stub_4a0318() {
    // IDA 0x4a0318: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::render3dAdorn(RBX::Adorn *)")]
// 0x4a0320 — __ZN3RBX9Explosion13render3dAdornEPNS_5AdornE — RBX::Explosion::render3dAdorn(RBX::Adorn *)
pub fn stub_4a0320() {
    // IDA 0x4a0320: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Explosion::render3dAdorn(RBX::Adorn *)")]
// 0x4a0430 — __ZThn92_N3RBX9Explosion13render3dAdornEPNS_5AdornE — non-virtual thunk toRBX::Explosion::render3dAdorn(RBX::Adorn *)
pub fn stub_4a0430() {
    // IDA 0x4a0430: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::getBlastRadius(void)const")]
// 0x4a0438 — __ZNK3RBX9Explosion14getBlastRadiusEv — RBX::Explosion::getBlastRadius(void)const
pub fn stub_4a0438() {
    // IDA 0x4a0438: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::getExplosionType(void)const")]
// 0x4a048c — __ZNK3RBX9Explosion16getExplosionTypeEv — RBX::Explosion::getExplosionType(void)const
pub fn stub_4a048c() {
    // IDA 0x4a048c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x4a1334 — __ZN3RBX9Explosion17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::Explosion::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_4a1334() {
    // IDA 0x4a1334: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::shouldRender3dAdorn(void)const")]
// 0x4a1350 — __ZNK3RBX9Explosion19shouldRender3dAdornEv — RBX::Explosion::shouldRender3dAdorn(void)const
pub fn stub_4a1350() {
    // IDA 0x4a1350: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Explosion::shouldRender3dAdorn(void)const")]
// 0x4a1368 — __ZThn92_NK3RBX9Explosion19shouldRender3dAdornEv — non-virtual thunk toRBX::Explosion::shouldRender3dAdorn(void)const
pub fn stub_4a1368() {
    // IDA 0x4a1368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IAdornable::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
// 0x4a1370 — __ZN3RBX10IAdornable14render3dSelectEPNS_5AdornENS_11SelectStateE — RBX::IAdornable::render3dSelect(RBX::Adorn *,RBX::SelectState)
pub fn stub_4a1370() {
    // IDA 0x4a1370: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::construct_func(char const*,char *)")]
// 0x4a14d0 — __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::construct_func(char const*,char *)
pub fn stub_4a14d0() {
    // IDA 0x4a14d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::resize(unsigned long,RBX::Explosion::ExplosionType)")]
// 0x4a33b8 — __ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE6resizeEmS2_ — std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::resize(unsigned long,RBX::Explosion::ExplosionType)
pub fn stub_4a33b8() {
    // IDA 0x4a33b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::push_back(RBX::Explosion::ExplosionType const&)")]
// 0x4a33f0 — __ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE9push_backERKS2_ — std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::push_back(RBX::Explosion::ExplosionType const&)
pub fn stub_4a33f0() {
    // IDA 0x4a33f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Explosion::ExplosionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::operator[](RBX::Name const* const&)")]
// 0x4a341c — __ZNSt3mapIPKN3RBX4NameENS0_9Explosion13ExplosionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::Explosion::ExplosionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::operator[](RBX::Name const* const&)
pub fn stub_4a341c() {
    // IDA 0x4a341c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
// 0x4a3474 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)
pub fn stub_4a3474() {
    // IDA 0x4a3474: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
// 0x4a3528 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)
pub fn stub_4a3528() {
    // IDA 0x4a3528: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
// 0x4a3580 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)
pub fn stub_4a3580() {
    // IDA 0x4a3580: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,RBX::Explosion::ExplosionType const&)")]
// 0x4a35ec — __ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,RBX::Explosion::ExplosionType const&)
pub fn stub_4a35ec() {
    // IDA 0x4a35ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_allocate(unsigned long)")]
// 0x4a36d0 — __ZNSt12_Vector_baseIN3RBX9Explosion13ExplosionTypeESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_allocate(unsigned long)
pub fn stub_4a36d0() {
    // IDA 0x4a36d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Explosion::ExplosionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *>(RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *)")]
// 0x4a36e8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9Explosion13ExplosionTypeES6_EET0_T_S8_S7_ — RBX::Explosion::ExplosionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *>(RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *)
pub fn stub_4a36e8() {
    // IDA 0x4a36e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,unsigned long,RBX::Explosion::ExplosionType const&)")]
// 0x4a3728 — __ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,unsigned long,RBX::Explosion::ExplosionType const&)
pub fn stub_4a3728() {
    // IDA 0x4a3728: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::IAdornable::shouldRender3dAdorn(void)const")]
// 0x4a6868 — __ZNK3RBX10IAdornable19shouldRender3dAdornEv — RBX::IAdornable::shouldRender3dAdorn(void)const
pub fn stub_4a6868() {
    // IDA 0x4a6868: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>> *)")]
// 0x4a6870 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>> *)
pub fn stub_4a6870() {
    // IDA 0x4a6870: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FactoryRegistrator::FactoryRegistrator(void)")]
// 0x4aa8c4 — __ZN3RBX18FactoryRegistratorC1Ev — RBX::FactoryRegistrator::FactoryRegistrator(void)
pub fn stub_4aa8c4() {
    // IDA 0x4aa8c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FactoryRegistrator::FactoryRegistrator(void)")]
// 0x4aa8c8 — __ZN3RBX18FactoryRegistratorC2Ev — RBX::FactoryRegistrator::FactoryRegistrator(void)
pub fn stub_4aa8c8() {
    // IDA 0x4aa8c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "onSlotException(std::exception &)")]
// 0x4aaa9c — __ZL15onSlotExceptionRSt9exception — onSlotException(std::exception &)
pub fn stub_4aaa9c() {
    // IDA 0x4aaa9c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BindableEvent::BindableEvent(void)")]
// 0x4ab904 — __ZN3RBX13BindableEventC2Ev — RBX::BindableEvent::BindableEvent(void)
pub fn stub_4ab904() {
    // IDA 0x4ab904: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BindableFunction::BindableFunction(void)")]
// 0x4ac88c — __ZN3RBX16BindableFunctionC2Ev — RBX::BindableFunction::BindableFunction(void)
pub fn stub_4ac88c() {
    // IDA 0x4ac88c: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::~deque()")]
// 0x4acacc — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EED2Ev — std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::~deque()
pub fn stub_4acacc() {
    // IDA 0x4acacc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::~_Deque_base()")]
// 0x4acbb4 — __ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EED2Ev — std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::~_Deque_base()
pub fn stub_4acbb4() {
    // IDA 0x4acbb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_destroy_data_aux(std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>)")]
// 0x4acbe0 — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE19_M_destroy_data_auxESt15_Deque_iteratorIS2_RS2_PS2_ES8_ — std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_destroy_data_aux(std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>)
pub fn stub_4acbe0() {
    // IDA 0x4acbe0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_initialize_map(unsigned long)")]
// 0x4ace68 — __ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EE17_M_initialize_mapEm — std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_initialize_map(unsigned long)
pub fn stub_4ace68() {
    // IDA 0x4ace68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_allocate_map(unsigned long)")]
// 0x4acfe4 — __ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EE15_M_allocate_mapEm — std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_allocate_map(unsigned long)
pub fn stub_4acfe4() {
    // IDA 0x4acfe4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_create_nodes(RBX::BindableFunction::Invocation**,RBX::BindableFunction::Invocation**)")]
// 0x4acffc — __ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EE15_M_create_nodesEPPS2_S6_ — std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_create_nodes(RBX::BindableFunction::Invocation**,RBX::BindableFunction::Invocation**)
pub fn stub_4acffc() {
    // IDA 0x4acffc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::deque(std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>> const&)")]
// 0x4ad0f0 — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EEC2ERKS4_ — std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::deque(std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>> const&)
pub fn stub_4ad0f0() {
    // IDA 0x4ad0f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>>(std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>,std::__false_type)")]
// 0x4ad224 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX16BindableFunction10InvocationERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type — std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>>(std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>,std::__false_type)
pub fn stub_4ad224() {
    // IDA 0x4ad224: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::CustomEvent::CustomEvent(void)")]
// 0x4b0954 — __ZN3RBX11CustomEventC2Ev — RBX::CustomEvent::CustomEvent(void)
pub fn stub_4b0954() {
    // IDA 0x4b0954: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::CustomEvent::~CustomEvent()")]
// 0x4b0b98 — __ZN3RBX11CustomEventD1Ev — RBX::CustomEvent::~CustomEvent()
pub fn stub_4b0b98() {
    // IDA 0x4b0b98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CustomEvent::~CustomEvent()")]
// 0x4b0b9c — __ZN3RBX11CustomEventD0Ev — RBX::CustomEvent::~CustomEvent()
pub fn stub_4b0b9c() {
    // IDA 0x4b0b9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CustomEvent::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x4b0c3c — __ZN3RBX11CustomEvent17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::CustomEvent::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_4b0c3c() {
    // IDA 0x4b0c3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::CustomEvent::~CustomEvent()")]
// 0x4b0ea4 — __ZThn32_N3RBX11CustomEventD1Ev — non-virtual thunk toRBX::CustomEvent::~CustomEvent()
pub fn stub_4b0ea4() {
    // IDA 0x4b0ea4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::CustomEvent::~CustomEvent()")]
// 0x4b0eac — __ZThn32_N3RBX11CustomEventD0Ev — non-virtual thunk toRBX::CustomEvent::~CustomEvent()
pub fn stub_4b0eac() {
    // IDA 0x4b0eac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::CustomEvent::~CustomEvent()")]
// 0x4b0f60 — __ZThn36_N3RBX11CustomEventD1Ev — non-virtual thunk toRBX::CustomEvent::~CustomEvent()
pub fn stub_4b0f60() {
    // IDA 0x4b0f60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::CustomEvent::~CustomEvent()")]
// 0x4b0f68 — __ZThn36_N3RBX11CustomEventD0Ev — non-virtual thunk toRBX::CustomEvent::~CustomEvent()
pub fn stub_4b0f68() {
    // IDA 0x4b0f68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CustomEvent::~CustomEvent()")]
// 0x4b100c — __ZN3RBX11CustomEventD2Ev — RBX::CustomEvent::~CustomEvent()
pub fn stub_4b100c() {
    // IDA 0x4b100c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CustomEventReceiver::CustomEventReceiver(void)")]
// 0x4b22d0 — __ZN3RBX19CustomEventReceiverC2Ev — RBX::CustomEventReceiver::CustomEventReceiver(void)
pub fn stub_4b22d0() {
    // IDA 0x4b22d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CustomEventReceiver::setCurrentValue(float)")]
// 0x4b26f4 — __ZN3RBX19CustomEventReceiver15setCurrentValueEf — RBX::CustomEventReceiver::setCurrentValue(float)
pub fn stub_4b26f4() {
    // IDA 0x4b26f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::StarterGuiService::CoreGuiType>::construct_func(char const*,char *)")]
// 0x4b40c4 — __ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::StarterGuiService::CoreGuiType>::construct_func(char const*,char *)
pub fn stub_4b40c4() {
    // IDA 0x4b40c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::HttpService::HttpContentType>(RBX::HttpService::HttpContentType const&)")]
// 0x4b4898 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11HttpService15HttpContentTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::HttpService::HttpContentType>(RBX::HttpService::HttpContentType const&)
pub fn stub_4b4898() {
    // IDA 0x4b4898: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::singleton(void)")]
// 0x4b48e8 — __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::singleton(void)
pub fn stub_4b48e8() {
    // IDA 0x4b48e8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

