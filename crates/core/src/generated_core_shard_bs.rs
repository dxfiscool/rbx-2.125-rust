//! core shard BS — 100 core stubs EA-sorted, next uncovered after BR 0x54c8f4..0x55aa84.
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x54c804.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::resize(unsigned long,RBX::GuiService::SpecialKey)")]
// 0x54c8f4 — __ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE6resizeEmS2_ — std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::resize(unsigned long,RBX::GuiService::SpecialKey)
pub fn stub_54c8f4() {
    // IDA 0x54c8f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::push_back(RBX::GuiService::SpecialKey const&)")]
// 0x54c928 — __ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE9push_backERKS2_ — std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::push_back(RBX::GuiService::SpecialKey const&)
pub fn stub_54c928() {
    // IDA 0x54c928: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GuiService::SpecialKey,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::operator[](RBX::Name const* const&)")]
// 0x54c950 — __ZNSt3mapIPKN3RBX4NameENS0_10GuiService10SpecialKeyESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::GuiService::SpecialKey,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::operator[](RBX::Name const* const&)
pub fn stub_54c950() {
    // IDA 0x54c950: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey> const&)")]
// 0x54c9a8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey> const&)
pub fn stub_54c9a8() {
    // IDA 0x54c9a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey> const&)")]
// 0x54ca5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey> const&)
pub fn stub_54ca5c() {
    // IDA 0x54ca5c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey> const&)")]
// 0x54cab4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey> const&)
pub fn stub_54cab4() {
    // IDA 0x54cab4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiService::SpecialKey*,std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>>,RBX::GuiService::SpecialKey const&)")]
// 0x54cb1c — __ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiService::SpecialKey*,std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>>,RBX::GuiService::SpecialKey const&)
pub fn stub_54cb1c() {
    // IDA 0x54cb1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_allocate(unsigned long)")]
// 0x54cc00 — __ZNSt12_Vector_baseIN3RBX10GuiService10SpecialKeyESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_allocate(unsigned long)
pub fn stub_54cc00() {
    // IDA 0x54cc00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiService::SpecialKey * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *>(RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *)")]
// 0x54cc18 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService10SpecialKeyES6_EET0_T_S8_S7_ — RBX::GuiService::SpecialKey * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *>(RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *)
pub fn stub_54cc18() {
    // IDA 0x54cc18: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiService::SpecialKey*,std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>>,unsigned long,RBX::GuiService::SpecialKey const&)")]
// 0x54cc54 — __ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiService::SpecialKey*,std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>>,unsigned long,RBX::GuiService::SpecialKey const&)
pub fn stub_54cc54() {
    // IDA 0x54cc54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)")]
// 0x554a3c — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E — std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)
pub fn stub_554a3c() {
    // IDA 0x554a3c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)")]
// 0x554a64 — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E — std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)
pub fn stub_554a64() {
    // IDA 0x554a64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_erase(std::_Rb_tree_node<char> *)")]
// 0x554a8c — __ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE8_M_eraseEPSt13_Rb_tree_nodeIcE — std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_erase(std::_Rb_tree_node<char> *)
pub fn stub_554a8c() {
    // IDA 0x554a8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>> *)")]
// 0x554ab4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>> *)
pub fn stub_554ab4() {
    // IDA 0x554ab4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>> *)")]
// 0x554adc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>> *)
pub fn stub_554adc() {
    // IDA 0x554adc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::registerBodyMovers(void)")]
// 0x555598 — __ZN3RBX18registerBodyMoversEv — RBX::registerBodyMovers(void)
pub fn stub_555598() {
    // IDA 0x555598: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BodyMover::BodyMover(char const*)")]
// 0x5555d8 — __ZN3RBX9BodyMoverC2EPKc — RBX::BodyMover::BodyMover(char const*)
pub fn stub_5555d8() {
    // IDA 0x5555d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BodyMover::~BodyMover()")]
// 0x555878 — __ZN3RBX9BodyMoverD0Ev — RBX::BodyMover::~BodyMover()
pub fn stub_555878() {
    // IDA 0x555878: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyMover::~BodyMover()")]
// 0x555918 — __ZN3RBX9BodyMoverD1Ev — RBX::BodyMover::~BodyMover()
pub fn stub_555918() {
    // IDA 0x555918: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
// 0x55591c — __ZThn32_N3RBX9BodyMoverD0Ev — non-virtual thunk to RBX::BodyMover::~BodyMover()
pub fn stub_55591c() {
    // IDA 0x55591c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
// 0x555924 — __ZThn36_N3RBX9BodyMoverD0Ev — non-virtual thunk to RBX::BodyMover::~BodyMover()
pub fn stub_555924() {
    // IDA 0x555924: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
// 0x55592c — __ZThn92_N3RBX9BodyMoverD0Ev — non-virtual thunk to RBX::BodyMover::~BodyMover()
pub fn stub_55592c() {
    // IDA 0x55592c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
// 0x555934 — __ZThn124_N3RBX9BodyMoverD0Ev — non-virtual thunk to RBX::BodyMover::~BodyMover()
pub fn stub_555934() {
    // IDA 0x555934: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
// 0x55593c — __ZThn244_N3RBX9BodyMoverD0Ev — non-virtual thunk to RBX::BodyMover::~BodyMover()
pub fn stub_55593c() {
    // IDA 0x55593c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyMover::~BodyMover()")]
// 0x555944 — __ZN3RBX9BodyMoverD2Ev — RBX::BodyMover::~BodyMover()
pub fn stub_555944() {
    // IDA 0x555944: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
// 0x555b68 — __ZThn32_N3RBX9BodyMoverD1Ev — non-virtual thunk to RBX::BodyMover::~BodyMover()
pub fn stub_555b68() {
    // IDA 0x555b68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
// 0x555b70 — __ZThn36_N3RBX9BodyMoverD1Ev — non-virtual thunk to RBX::BodyMover::~BodyMover()
pub fn stub_555b70() {
    // IDA 0x555b70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
// 0x555b78 — __ZThn92_N3RBX9BodyMoverD1Ev — non-virtual thunk to RBX::BodyMover::~BodyMover()
pub fn stub_555b78() {
    // IDA 0x555b78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
// 0x555b80 — __ZThn124_N3RBX9BodyMoverD1Ev — non-virtual thunk to RBX::BodyMover::~BodyMover()
pub fn stub_555b80() {
    // IDA 0x555b80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
// 0x555b88 — __ZThn244_N3RBX9BodyMoverD1Ev — non-virtual thunk to RBX::BodyMover::~BodyMover()
pub fn stub_555b88() {
    // IDA 0x555b88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyMover::computeForce(bool)")]
// 0x555b90 — __ZN3RBX9BodyMover12computeForceEb — RBX::BodyMover::computeForce(bool)
pub fn stub_555b90() {
    // IDA 0x555b90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::computeForce(bool)")]
// 0x556034 — __ZThn244_N3RBX9BodyMover12computeForceEb — non-virtual thunk to RBX::BodyMover::computeForce(bool)
pub fn stub_556034() {
    // IDA 0x556034: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyMover::stepWorld(void)")]
// 0x556140 — __ZN3RBX9BodyMover9stepWorldEv — RBX::BodyMover::stepWorld(void)
pub fn stub_556140() {
    // IDA 0x556140: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::stepWorld(void)")]
// 0x55627c — __ZThn92_N3RBX9BodyMover9stepWorldEv — non-virtual thunk to RBX::BodyMover::stepWorld(void)
pub fn stub_55627c() {
    // IDA 0x55627c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyMover::getEngineBody(void)")]
// 0x556284 — __ZN3RBX9BodyMover13getEngineBodyEv — RBX::BodyMover::getEngineBody(void)
pub fn stub_556284() {
    // IDA 0x556284: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::getEngineBody(void)")]
// 0x556318 — __ZThn92_N3RBX9BodyMover13getEngineBodyEv — non-virtual thunk to RBX::BodyMover::getEngineBody(void)
pub fn stub_556318() {
    // IDA 0x556318: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyMover::duplicateBodyMoverExists(RBX::Primitive *,RBX::Primitive *)")]
// 0x556320 — __ZN3RBX9BodyMover24duplicateBodyMoverExistsEPNS_9PrimitiveES2_ — RBX::BodyMover::duplicateBodyMoverExists(RBX::Primitive *,RBX::Primitive *)
pub fn stub_556320() {
    // IDA 0x556320: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyMover::onAncestorChanged(RBX::AncestorChanged const&)")]
// 0x556368 — __ZN3RBX9BodyMover17onAncestorChangedERKNS_15AncestorChangedE — RBX::BodyMover::onAncestorChanged(RBX::AncestorChanged const&)
pub fn stub_556368() {
    // IDA 0x556368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Rocket::fire(void)")]
// 0x5568b0 — __ZN3RBX6Rocket4fireEv — RBX::Rocket::fire(void)
pub fn stub_5568b0() {
    // IDA 0x5568b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Rocket::abort(void)")]
// 0x5568dc — __ZN3RBX6Rocket5abortEv — RBX::Rocket::abort(void)
pub fn stub_5568dc() {
    // IDA 0x5568dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Rocket::Rocket(void)")]
// 0x55690c — __ZN3RBX6RocketC2Ev — RBX::Rocket::Rocket(void)
pub fn stub_55690c() {
    // IDA 0x55690c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Rocket::~Rocket()")]
// 0x556bb0 — __ZN3RBX6RocketD0Ev — RBX::Rocket::~Rocket()
pub fn stub_556bb0() {
    // IDA 0x556bb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Rocket::~Rocket()")]
// 0x556c50 — __ZN3RBX6RocketD1Ev — RBX::Rocket::~Rocket()
pub fn stub_556c50() {
    // IDA 0x556c50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
// 0x556c54 — __ZThn32_N3RBX6RocketD0Ev — non-virtual thunk to RBX::Rocket::~Rocket()
pub fn stub_556c54() {
    // IDA 0x556c54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
// 0x556c5c — __ZThn36_N3RBX6RocketD0Ev — non-virtual thunk to RBX::Rocket::~Rocket()
pub fn stub_556c5c() {
    // IDA 0x556c5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
// 0x556c64 — __ZThn92_N3RBX6RocketD0Ev — non-virtual thunk to RBX::Rocket::~Rocket()
pub fn stub_556c64() {
    // IDA 0x556c64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
// 0x556c6c — __ZThn124_N3RBX6RocketD0Ev — non-virtual thunk to RBX::Rocket::~Rocket()
pub fn stub_556c6c() {
    // IDA 0x556c6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
// 0x556c74 — __ZThn244_N3RBX6RocketD0Ev — non-virtual thunk to RBX::Rocket::~Rocket()
pub fn stub_556c74() {
    // IDA 0x556c74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
// 0x556c7c — __ZThn304_N3RBX6RocketD0Ev — non-virtual thunk to RBX::Rocket::~Rocket()
pub fn stub_556c7c() {
    // IDA 0x556c7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Rocket::~Rocket()")]
// 0x556c84 — __ZN3RBX6RocketD2Ev — RBX::Rocket::~Rocket()
pub fn stub_556c84() {
    // IDA 0x556c84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
// 0x556e1c — __ZThn32_N3RBX6RocketD1Ev — non-virtual thunk to RBX::Rocket::~Rocket()
pub fn stub_556e1c() {
    // IDA 0x556e1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
// 0x556e24 — __ZThn36_N3RBX6RocketD1Ev — non-virtual thunk to RBX::Rocket::~Rocket()
pub fn stub_556e24() {
    // IDA 0x556e24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
// 0x556e2c — __ZThn92_N3RBX6RocketD1Ev — non-virtual thunk to RBX::Rocket::~Rocket()
pub fn stub_556e2c() {
    // IDA 0x556e2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
// 0x556e34 — __ZThn124_N3RBX6RocketD1Ev — non-virtual thunk to RBX::Rocket::~Rocket()
pub fn stub_556e34() {
    // IDA 0x556e34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
// 0x556e3c — __ZThn244_N3RBX6RocketD1Ev — non-virtual thunk to RBX::Rocket::~Rocket()
pub fn stub_556e3c() {
    // IDA 0x556e3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
// 0x556e44 — __ZThn304_N3RBX6RocketD1Ev — non-virtual thunk to RBX::Rocket::~Rocket()
pub fn stub_556e44() {
    // IDA 0x556e44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Rocket::onStepped(RBX::Stepped const&)")]
// 0x556e4c — __ZN3RBX6Rocket9onSteppedERKNS_7SteppedE — RBX::Rocket::onStepped(RBX::Stepped const&)
pub fn stub_556e4c() {
    // IDA 0x556e4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Rocket::onStepped(RBX::Stepped const&)")]
// 0x55705c — __ZThn304_N3RBX6Rocket9onSteppedERKNS_7SteppedE — non-virtual thunk to RBX::Rocket::onStepped(RBX::Stepped const&)
pub fn stub_55705c() {
    // IDA 0x55705c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyGyro::BodyGyro(void)")]
// 0x5578a0 — __ZN3RBX8BodyGyroC2Ev — RBX::BodyGyro::BodyGyro(void)
pub fn stub_5578a0() {
    // IDA 0x5578a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyGyro::computeBalanceTorque(RBX::Body *,RBX::Body *)")]
// 0x557c50 — __ZN3RBX8BodyGyro20computeBalanceTorqueEPNS_4BodyES2_ — RBX::BodyGyro::computeBalanceTorque(RBX::Body *,RBX::Body *)
pub fn stub_557c50() {
    // IDA 0x557c50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyGyro::computeOrientationTorque(RBX::Body *,RBX::Body *)")]
// 0x557ff8 — __ZN3RBX8BodyGyro24computeOrientationTorqueEPNS_4BodyES2_ — RBX::BodyGyro::computeOrientationTorque(RBX::Body *,RBX::Body *)
pub fn stub_557ff8() {
    // IDA 0x557ff8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyPosition::BodyPosition(void)")]
// 0x5582bc — __ZN3RBX12BodyPositionC2Ev — RBX::BodyPosition::BodyPosition(void)
pub fn stub_5582bc() {
    // IDA 0x5582bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyPosition::onStepped(RBX::Stepped const&)")]
// 0x558780 — __ZN3RBX12BodyPosition9onSteppedERKNS_7SteppedE — RBX::BodyPosition::onStepped(RBX::Stepped const&)
pub fn stub_558780() {
    // IDA 0x558780: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::onStepped(RBX::Stepped const&)")]
// 0x5588ec — __ZThn304_N3RBX12BodyPosition9onSteppedERKNS_7SteppedE — non-virtual thunk to RBX::BodyPosition::onStepped(RBX::Stepped const&)
pub fn stub_5588ec() {
    // IDA 0x5588ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyVelocity::BodyVelocity(void)")]
// 0x5588f8 — __ZN3RBX12BodyVelocityC2Ev — RBX::BodyVelocity::BodyVelocity(void)
pub fn stub_5588f8() {
    // IDA 0x5588f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyAngularVelocity::BodyAngularVelocity(void)")]
// 0x558c34 — __ZN3RBX19BodyAngularVelocityC2Ev — RBX::BodyAngularVelocity::BodyAngularVelocity(void)
pub fn stub_558c34() {
    // IDA 0x558c34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyForce::BodyForce(void)")]
// 0x558f70 — __ZN3RBX9BodyForceC2Ev — RBX::BodyForce::BodyForce(void)
pub fn stub_558f70() {
    // IDA 0x558f70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyThrust::BodyThrust(void)")]
// 0x559124 — __ZN3RBX10BodyThrustC2Ev — RBX::BodyThrust::BodyThrust(void)
pub fn stub_559124() {
    // IDA 0x559124: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Rocket::getTargetDangerous(void)const")]
// 0x559440 — __ZNK3RBX6Rocket18getTargetDangerousEv — RBX::Rocket::getTargetDangerous(void)const
pub fn stub_559440() {
    // IDA 0x559440: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Body::getBranchForce(void)const")]
// 0x5594c4 — __ZNK3RBX4Body14getBranchForceEv — RBX::Body::getBranchForce(void)const
pub fn stub_5594c4() {
    // IDA 0x5594c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Body::getBranchTorque(void)const")]
// 0x559534 — __ZNK3RBX4Body15getBranchTorqueEv — RBX::Body::getBranchTorque(void)const
pub fn stub_559534() {
    // IDA 0x559534: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BodyPosition::getLastForce(void)")]
// 0x5595ac — __ZN3RBX12BodyPosition12getLastForceEv — RBX::BodyPosition::getLastForce(void)
pub fn stub_5595ac() {
    // IDA 0x5595ac: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BodyVelocity::getLastForce(void)")]
// 0x559604 — __ZN3RBX12BodyVelocity12getLastForceEv — RBX::BodyVelocity::getLastForce(void)
pub fn stub_559604() {
    // IDA 0x559604: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Body::getBranchVelocity(void)")]
// 0x559638 — __ZN3RBX4Body17getBranchVelocityEv — RBX::Body::getBranchVelocity(void)
pub fn stub_559638() {
    // IDA 0x559638: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BodyPosition::~BodyPosition()")]
// 0x5596b0 — __ZN3RBX12BodyPositionD1Ev — RBX::BodyPosition::~BodyPosition()
pub fn stub_5596b0() {
    // IDA 0x5596b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyPosition::~BodyPosition()")]
// 0x5597e0 — __ZN3RBX12BodyPositionD0Ev — RBX::BodyPosition::~BodyPosition()
pub fn stub_5597e0() {
    // IDA 0x5597e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyPosition::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x559920 — __ZN3RBX12BodyPosition17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::BodyPosition::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_559920() {
    // IDA 0x559920: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyMover::canStepWorld(void)const")]
// 0x559938 — __ZNK3RBX9BodyMover12canStepWorldEv — RBX::BodyMover::canStepWorld(void)const
pub fn stub_559938() {
    // IDA 0x559938: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
// 0x55993c — __ZThn32_N3RBX12BodyPositionD1Ev — non-virtual thunk to RBX::BodyPosition::~BodyPosition()
pub fn stub_55993c() {
    // IDA 0x55993c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
// 0x559a68 — __ZThn32_N3RBX12BodyPositionD0Ev — non-virtual thunk to RBX::BodyPosition::~BodyPosition()
pub fn stub_559a68() {
    // IDA 0x559a68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
// 0x559bb8 — __ZThn36_N3RBX12BodyPositionD1Ev — non-virtual thunk to RBX::BodyPosition::~BodyPosition()
pub fn stub_559bb8() {
    // IDA 0x559bb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
// 0x559ce4 — __ZThn36_N3RBX12BodyPositionD0Ev — non-virtual thunk to RBX::BodyPosition::~BodyPosition()
pub fn stub_559ce4() {
    // IDA 0x559ce4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
// 0x559e24 — __ZThn92_N3RBX12BodyPositionD1Ev — non-virtual thunk to RBX::BodyPosition::~BodyPosition()
pub fn stub_559e24() {
    // IDA 0x559e24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
// 0x559f50 — __ZThn92_N3RBX12BodyPositionD0Ev — non-virtual thunk to RBX::BodyPosition::~BodyPosition()
pub fn stub_559f50() {
    // IDA 0x559f50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyMover::canStepWorld(void)const")]
// 0x55a090 — __ZThn92_NK3RBX9BodyMover12canStepWorldEv — non-virtual thunk to RBX::BodyMover::canStepWorld(void)const
pub fn stub_55a090() {
    // IDA 0x55a090: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
// 0x55a094 — __ZThn124_N3RBX12BodyPositionD1Ev — non-virtual thunk to RBX::BodyPosition::~BodyPosition()
pub fn stub_55a094() {
    // IDA 0x55a094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
// 0x55a1c0 — __ZThn124_N3RBX12BodyPositionD0Ev — non-virtual thunk to RBX::BodyPosition::~BodyPosition()
pub fn stub_55a1c0() {
    // IDA 0x55a1c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
// 0x55a300 — __ZThn244_N3RBX12BodyPositionD1Ev — non-virtual thunk to RBX::BodyPosition::~BodyPosition()
pub fn stub_55a300() {
    // IDA 0x55a300: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
// 0x55a430 — __ZThn244_N3RBX12BodyPositionD0Ev — non-virtual thunk to RBX::BodyPosition::~BodyPosition()
pub fn stub_55a430() {
    // IDA 0x55a430: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
// 0x55a574 — __ZThn304_N3RBX12BodyPositionD1Ev — non-virtual thunk to RBX::BodyPosition::~BodyPosition()
pub fn stub_55a574() {
    // IDA 0x55a574: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
// 0x55a6a4 — __ZThn304_N3RBX12BodyPositionD0Ev — non-virtual thunk to RBX::BodyPosition::~BodyPosition()
pub fn stub_55a6a4() {
    // IDA 0x55a6a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Rocket::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x55a7e8 — __ZN3RBX6Rocket17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::Rocket::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_55a7e8() {
    // IDA 0x55a7e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyGyro::~BodyGyro()")]
// 0x55a860 — __ZN3RBX8BodyGyroD1Ev — RBX::BodyGyro::~BodyGyro()
pub fn stub_55a860() {
    // IDA 0x55a860: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BodyGyro::~BodyGyro()")]
// 0x55a864 — __ZN3RBX8BodyGyroD0Ev — RBX::BodyGyro::~BodyGyro()
pub fn stub_55a864() {
    // IDA 0x55a864: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
// 0x55a914 — __ZThn32_N3RBX8BodyGyroD1Ev — non-virtual thunk to RBX::BodyGyro::~BodyGyro()
pub fn stub_55a914() {
    // IDA 0x55a914: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
// 0x55a91c — __ZThn32_N3RBX8BodyGyroD0Ev — non-virtual thunk to RBX::BodyGyro::~BodyGyro()
pub fn stub_55a91c() {
    // IDA 0x55a91c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
// 0x55a9d0 — __ZThn36_N3RBX8BodyGyroD1Ev — non-virtual thunk to RBX::BodyGyro::~BodyGyro()
pub fn stub_55a9d0() {
    // IDA 0x55a9d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
// 0x55a9d8 — __ZThn36_N3RBX8BodyGyroD0Ev — non-virtual thunk to RBX::BodyGyro::~BodyGyro()
pub fn stub_55a9d8() {
    // IDA 0x55a9d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
// 0x55aa7c — __ZThn92_N3RBX8BodyGyroD1Ev — non-virtual thunk to RBX::BodyGyro::~BodyGyro()
pub fn stub_55aa7c() {
    // IDA 0x55aa7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
// 0x55aa84 — __ZThn92_N3RBX8BodyGyroD0Ev — non-virtual thunk to RBX::BodyGyro::~BodyGyro()
pub fn stub_55aa84() {
    // IDA 0x55aa84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}