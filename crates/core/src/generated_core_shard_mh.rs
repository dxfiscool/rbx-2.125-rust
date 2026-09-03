//! core shard mh — 150 core stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 150 not yet in any crate (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 16073 uncovered before batch, batch 0x48caa0..0x4acbe0).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::resize(unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod)")]
// 0x48caa0 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE6resizeEmS3_ — std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::resize(unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod)
// type: int(void)
pub fn stub_0x48caa0() {
    // IDA 0x48caa0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::push_back(RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
// 0x48cad4 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE9push_backERKS3_ — std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::push_back(RBX::TaskScheduler::Job::SleepAdjustMethod const&)
// type: int(void)
pub fn stub_0x48cad4() {
    // IDA 0x48cad4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::Job::SleepAdjustMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::operator[](RBX::Name const* const&)")]
// 0x48cafc — __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler3Job17SleepAdjustMethodESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_ — std::map<RBX::Name const*,RBX::TaskScheduler::Job::SleepAdjustMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::operator[](RBX::Name const* const&)
// type: _Rb_tree_node_base **__fastcall(int, int *)
pub fn stub_0x48cafc() {
    // IDA 0x48cafc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
// 0x48cb54 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x48cb54() {
    // IDA 0x48cb54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
// 0x48cc08 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)
// type: int(void)
pub fn stub_0x48cc08() {
    // IDA 0x48cc08: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
// 0x48cc60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)
// type: int(void)
pub fn stub_0x48cc60() {
    // IDA 0x48cc60: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
// 0x48ccc8 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_ — std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,RBX::TaskScheduler::Job::SleepAdjustMethod const&)
// type: int(void)
pub fn stub_0x48ccc8() {
    // IDA 0x48ccc8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_allocate(unsigned long)")]
// 0x48cdac — __ZNSt12_Vector_baseIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE11_M_allocateEm — std::_Vector_base<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_allocate(unsigned long)
// type: int(void)
pub fn stub_0x48cdac() {
    // IDA 0x48cdac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TaskScheduler::Job::SleepAdjustMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *>(RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *)")]
// 0x48cdc4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler3Job17SleepAdjustMethodES7_EET0_T_S9_S8_ — RBX::TaskScheduler::Job::SleepAdjustMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *>(RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *)
// type: int(void)
pub fn stub_0x48cdc4() {
    // IDA 0x48cdc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
// 0x48ce00 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_ — std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod const&)
// type: int(void)
pub fn stub_0x48ce00() {
    // IDA 0x48ce00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::resize(unsigned long,RBX::TaskScheduler::PriorityMethod)")]
// 0x48cf90 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE6resizeEmS2_ — std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::resize(unsigned long,RBX::TaskScheduler::PriorityMethod)
// type: int(void)
pub fn stub_0x48cf90() {
    // IDA 0x48cf90: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::push_back(RBX::TaskScheduler::PriorityMethod const&)")]
// 0x48cfc4 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE9push_backERKS2_ — std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::push_back(RBX::TaskScheduler::PriorityMethod const&)
// type: int(void)
pub fn stub_0x48cfc4() {
    // IDA 0x48cfc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::PriorityMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::operator[](RBX::Name const* const&)")]
// 0x48cfec — __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler14PriorityMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::TaskScheduler::PriorityMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::operator[](RBX::Name const* const&)
// type: int(void)
pub fn stub_0x48cfec() {
    // IDA 0x48cfec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
// 0x48d044 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x48d044() {
    // IDA 0x48d044: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
// 0x48d0f8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)
// type: int(void)
pub fn stub_0x48d0f8() {
    // IDA 0x48d0f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
// 0x48d150 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)
// type: int(void)
pub fn stub_0x48d150() {
    // IDA 0x48d150: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,RBX::TaskScheduler::PriorityMethod const&)")]
// 0x48d1b8 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,RBX::TaskScheduler::PriorityMethod const&)
// type: int(void)
pub fn stub_0x48d1b8() {
    // IDA 0x48d1b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_allocate(unsigned long)")]
// 0x48d29c — __ZNSt12_Vector_baseIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_allocate(unsigned long)
// type: int __fastcall(int, unsigned int)
pub fn stub_0x48d29c() {
    // IDA 0x48d29c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TaskScheduler::PriorityMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *>(RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *)")]
// 0x48d2b4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler14PriorityMethodES6_EET0_T_S8_S7_ — RBX::TaskScheduler::PriorityMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *>(RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *)
// type: int(void)
pub fn stub_0x48d2b4() {
    // IDA 0x48d2b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,unsigned long,RBX::TaskScheduler::PriorityMethod const&)")]
// 0x48d2f0 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,unsigned long,RBX::TaskScheduler::PriorityMethod const&)
// type: int(void)
pub fn stub_0x48d2f0() {
    // IDA 0x48d2f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::resize(unsigned long,RBX::TaskScheduler::ThreadPoolConfig)")]
// 0x48d480 — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE6resizeEmS2_ — std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::resize(unsigned long,RBX::TaskScheduler::ThreadPoolConfig)
// type: int(void)
pub fn stub_0x48d480() {
    // IDA 0x48d480: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::ThreadPoolConfig,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::operator[](RBX::Name const* const&)")]
// 0x48d4b4 — __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler16ThreadPoolConfigESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::TaskScheduler::ThreadPoolConfig,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::operator[](RBX::Name const* const&)
// type: int(void)
pub fn stub_0x48d4b4() {
    // IDA 0x48d4b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
// 0x48d50c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x48d50c() {
    // IDA 0x48d50c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
// 0x48d5c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)
// type: int(void)
pub fn stub_0x48d5c0() {
    // IDA 0x48d5c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
// 0x48d618 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)
// type: int(void)
pub fn stub_0x48d618() {
    // IDA 0x48d618: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,unsigned long,RBX::TaskScheduler::ThreadPoolConfig const&)")]
// 0x48d680 — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,unsigned long,RBX::TaskScheduler::ThreadPoolConfig const&)
// type: int(void)
pub fn stub_0x48d680() {
    // IDA 0x48d680: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_allocate(unsigned long)")]
// 0x48d810 — __ZNSt12_Vector_baseIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_allocate(unsigned long)
// type: int(void)
pub fn stub_0x48d810() {
    // IDA 0x48d810: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TaskScheduler::ThreadPoolConfig * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *>(RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *)")]
// 0x48d828 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler16ThreadPoolConfigES6_EET0_T_S8_S7_ — RBX::TaskScheduler::ThreadPoolConfig * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *>(RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *)
// type: int(void)
pub fn stub_0x48d828() {
    // IDA 0x48d828: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::push_back(RBX::TaskScheduler::ThreadPoolConfig const&)")]
// 0x48d864 — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE9push_backERKS2_ — std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::push_back(RBX::TaskScheduler::ThreadPoolConfig const&)
// type: int(void)
pub fn stub_0x48d864() {
    // IDA 0x48d864: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,RBX::TaskScheduler::ThreadPoolConfig const&)")]
// 0x48d88c — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,RBX::TaskScheduler::ThreadPoolConfig const&)
// type: int(void)
pub fn stub_0x48d88c() {
    // IDA 0x48d88c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x48d970 — __ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv — boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
// type: int(void)
pub fn stub_0x48d970() {
    // IDA 0x48d970: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "DummyJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x48dc2c — __ZN8DummyJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE — DummyJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
pub fn stub_0x48dc2c() {
    // IDA 0x48dc2c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "DummyJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x48dc34 — __ZN8DummyJob5errorERKN3RBX13TaskScheduler3Job5StatsE — DummyJob::error(RBX::TaskScheduler::Job::Stats const&)
pub fn stub_0x48dc34() {
    // IDA 0x48dc34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "DummyJob::step(RBX::TaskScheduler::Job::Stats const&)")]
// 0x48dc58 — __ZN8DummyJob4stepERKN3RBX13TaskScheduler3Job5StatsE — DummyJob::step(RBX::TaskScheduler::Job::Stats const&)
pub fn stub_0x48dc58() {
    // IDA 0x48dc58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::rate(void)const")]
// 0x48dc60 — __ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv — RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::rate(void)const
// type: int(void)
pub fn stub_0x48dc60() {
    // IDA 0x48dc60: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>> *)")]
// 0x48dcc0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>> *)
// type: int(void)
pub fn stub_0x48dcc0() {
    // IDA 0x48dcc0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>> *)")]
// 0x48dce8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>> *)
// type: int(void)
pub fn stub_0x48dce8() {
    // IDA 0x48dce8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>> *)")]
// 0x48dd10 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>> *)
// type: int(void)
pub fn stub_0x48dd10() {
    // IDA 0x48dd10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>> *)")]
// 0x48dd38 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>> *)
// type: int(void)
pub fn stub_0x48dd38() {
    // IDA 0x48dd38: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>> *)")]
// 0x48dd60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>> *)
// type: int(void)
pub fn stub_0x48dd60() {
    // IDA 0x48dd60: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>> *)")]
// 0x48dd88 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>> *)
// type: int(void)
pub fn stub_0x48dd88() {
    // IDA 0x48dd88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Decal::setTexture(RBX::TextureId)")]
// 0x48f7f4 — __ZN3RBX5Decal10setTextureENS_9TextureIdE — RBX::Decal::setTexture(RBX::TextureId)
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x48f7f4() {
    // IDA 0x48f7f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Decal::setSpecular(float)")]
// 0x48f82c — __ZN3RBX5Decal11setSpecularEf — RBX::Decal::setSpecular(float)
// type: float *__fastcall(float *this, float)
pub fn stub_0x48f82c() {
    // IDA 0x48f82c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Decal::setShiny(float)")]
// 0x48f860 — __ZN3RBX5Decal8setShinyEf — RBX::Decal::setShiny(float)
// type: _DWORD __fastcall(RBX::Decal *__hidden this, float)
pub fn stub_0x48f860() {
    // IDA 0x48f860: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Decal::setTransparency(float)")]
// 0x48f894 — __ZN3RBX5Decal15setTransparencyEf — RBX::Decal::setTransparency(float)
// type: _DWORD __fastcall(RBX::Decal *__hidden this, float)
pub fn stub_0x48f894() {
    // IDA 0x48f894: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Decal::Decal(void)")]
// 0x48f8bc — __ZN3RBX5DecalC2Ev — RBX::Decal::Decal(void)
// type: _DWORD __fastcall(RBX::Decal *__hidden this)
pub fn stub_0x48f8bc() {
    // IDA 0x48f8bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<RBX::TextureId>::convertToValue(std::string const&,RBX::TextureId&)")]
// 0x48fb04 — __ZN3RBX15StringConverterINS_9TextureIdEE14convertToValueERKSsRS1_ — RBX::StringConverter<RBX::TextureId>::convertToValue(std::string const&,RBX::TextureId&)
// type: int __fastcall(std::string *)
pub fn stub_0x48fb04() {
    // IDA 0x48fb04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Texture::setStudsPerTileU(float)")]
// 0x49047c — __ZN3RBX7Texture16setStudsPerTileUEf — RBX::Texture::setStudsPerTileU(float)
// type: _DWORD __fastcall(RBX::Texture *__hidden this, float)
pub fn stub_0x49047c() {
    // IDA 0x49047c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Texture::setStudsPerTileV(float)")]
// 0x4904b0 — __ZN3RBX7Texture16setStudsPerTileVEf — RBX::Texture::setStudsPerTileV(float)
// type: _DWORD __fastcall(RBX::Texture *__hidden this, float)
pub fn stub_0x4904b0() {
    // IDA 0x4904b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Texture::Texture(void)")]
// 0x4904e4 — __ZN3RBX7TextureC2Ev — RBX::Texture::Texture(void)
// type: RBX::Decal *__fastcall(RBX::Texture *this)
pub fn stub_0x4904e4() {
    // IDA 0x4904e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Decal::getTexture(void)const")]
// 0x49076c — __ZNK3RBX5Decal10getTextureEv — RBX::Decal::getTexture(void)const
// type: _DWORD __fastcall(RBX::Decal *__hidden this)
pub fn stub_0x49076c() {
    // IDA 0x49076c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Decal::getSpecular(void)const")]
// 0x490794 — __ZNK3RBX5Decal11getSpecularEv — RBX::Decal::getSpecular(void)const
// type: _DWORD __fastcall(RBX::Decal *__hidden this)
pub fn stub_0x490794() {
    // IDA 0x490794: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Decal::getShiny(void)const")]
// 0x4907c0 — __ZNK3RBX5Decal8getShinyEv — RBX::Decal::getShiny(void)const
// type: _DWORD __fastcall(RBX::Decal *__hidden this)
pub fn stub_0x4907c0() {
    // IDA 0x4907c0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Decal::getTransparency(void)const")]
// 0x4907c8 — __ZNK3RBX5Decal15getTransparencyEv — RBX::Decal::getTransparency(void)const
// type: _DWORD __fastcall(RBX::Decal *__hidden this)
pub fn stub_0x4907c8() {
    // IDA 0x4907c8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Texture::getStudsPerTileU(void)const")]
// 0x490a7c — __ZNK3RBX7Texture16getStudsPerTileUEv — RBX::Texture::getStudsPerTileU(void)const
// type: _DWORD __fastcall(RBX::Texture *__hidden this)
pub fn stub_0x490a7c() {
    // IDA 0x490a7c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Texture::getStudsPerTileV(void)const")]
// 0x490aa8 — __ZNK3RBX7Texture16getStudsPerTileVEv — RBX::Texture::getStudsPerTileV(void)const
// type: _DWORD __fastcall(RBX::Texture *__hidden this)
pub fn stub_0x490aa8() {
    // IDA 0x490aa8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Decal::~Decal()")]
// 0x490ab8 — __ZN3RBX5DecalD1Ev — RBX::Decal::~Decal()
// type: void __fastcall(RBX::Decal *__hidden this)
pub fn stub_0x490ab8() {
    // IDA 0x490ab8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Decal::~Decal()")]
// 0x490af8 — __ZN3RBX5DecalD0Ev — RBX::Decal::~Decal()
// type: void __fastcall(RBX::Decal *__hidden this)
pub fn stub_0x490af8() {
    // IDA 0x490af8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Decal::~Decal()")]
// 0x490be4 — __ZThn32_N3RBX5DecalD1Ev — non-virtual thunk to RBX::Decal::~Decal()
// was: non-virtual thunk to RBX::Decal::~Decal()
// type: void __fastcall(RBX::Decal *__hidden this)
pub fn stub_0x490be4() {
    // IDA 0x490be4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Decal::~Decal()")]
// 0x490c28 — __ZThn32_N3RBX5DecalD0Ev — non-virtual thunk to RBX::Decal::~Decal()
// was: non-virtual thunk to RBX::Decal::~Decal()
// type: void __fastcall(RBX::Decal *__hidden this)
pub fn stub_0x490c28() {
    // IDA 0x490c28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Decal::~Decal()")]
// 0x490d14 — __ZThn36_N3RBX5DecalD1Ev — non-virtual thunk to RBX::Decal::~Decal()
// was: non-virtual thunk to RBX::Decal::~Decal()
// type: void __fastcall(RBX::Decal *__hidden this)
pub fn stub_0x490d14() {
    // IDA 0x490d14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Decal::~Decal()")]
// 0x490d58 — __ZThn36_N3RBX5DecalD0Ev — non-virtual thunk to RBX::Decal::~Decal()
// was: non-virtual thunk to RBX::Decal::~Decal()
// type: void __fastcall(RBX::Decal *__hidden this)
pub fn stub_0x490d58() {
    // IDA 0x490d58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Texture::~Texture()")]
// 0x490e34 — __ZN3RBX7TextureD1Ev — RBX::Texture::~Texture()
// type: void __fastcall(RBX::Texture *__hidden this)
pub fn stub_0x490e34() {
    // IDA 0x490e34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Texture::~Texture()")]
// 0x490e74 — __ZN3RBX7TextureD0Ev — RBX::Texture::~Texture()
// type: void __fastcall(RBX::Texture *__hidden this)
pub fn stub_0x490e74() {
    // IDA 0x490e74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Texture::~Texture()")]
// 0x490f60 — __ZThn32_N3RBX7TextureD1Ev — non-virtual thunk to RBX::Texture::~Texture()
// was: non-virtual thunk to RBX::Texture::~Texture()
// type: void __fastcall(RBX::Texture *__hidden this)
pub fn stub_0x490f60() {
    // IDA 0x490f60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Texture::~Texture()")]
// 0x490fa4 — __ZThn32_N3RBX7TextureD0Ev — non-virtual thunk to RBX::Texture::~Texture()
// was: non-virtual thunk to RBX::Texture::~Texture()
// type: void __fastcall(RBX::Texture *__hidden this)
pub fn stub_0x490fa4() {
    // IDA 0x490fa4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Texture::~Texture()")]
// 0x491090 — __ZThn36_N3RBX7TextureD1Ev — non-virtual thunk to RBX::Texture::~Texture()
// was: non-virtual thunk to RBX::Texture::~Texture()
// type: void __fastcall(RBX::Texture *__hidden this)
pub fn stub_0x491090() {
    // IDA 0x491090: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Texture::~Texture()")]
// 0x4910d4 — __ZThn36_N3RBX7TextureD0Ev — non-virtual thunk to RBX::Texture::~Texture()
// was: non-virtual thunk to RBX::Texture::~Texture()
// type: void __fastcall(RBX::Texture *__hidden this)
pub fn stub_0x4910d4() {
    // IDA 0x4910d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextureId * rbx::any_cast<RBX::TextureId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x4929f8 — __ZN3rbx8any_castIN3RBX9TextureIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE — RBX::TextureId * rbx::any_cast<RBX::TextureId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// type: int(void)
pub fn stub_0x4929f8() {
    // IDA 0x4929f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextureId & rbx::any_cast<RBX::TextureId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x492a50 — __ZN3rbx8any_castIRN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::TextureId & rbx::any_cast<RBX::TextureId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: int(void)
pub fn stub_0x492a50() {
    // IDA 0x492a50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogChoice::setUserDialog(std::string)")]
// 0x493660 — __ZN3RBX12DialogChoice13setUserDialogESs — RBX::DialogChoice::setUserDialog(std::string)
pub fn stub_0x493660() {
    // IDA 0x493660: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogChoice::setResponseDialog(std::string)")]
// 0x4937d4 — __ZN3RBX12DialogChoice17setResponseDialogESs — RBX::DialogChoice::setResponseDialog(std::string)
pub fn stub_0x4937d4() {
    // IDA 0x4937d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogChoice::DialogChoice(void)")]
// 0x493810 — __ZN3RBX12DialogChoiceC2Ev — RBX::DialogChoice::DialogChoice(void)
// type: _DWORD __fastcall(RBX::DialogChoice *__hidden this)
pub fn stub_0x493810() {
    // IDA 0x493810: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogChoice::getUserDialog(void)const")]
// 0x493b28 — __ZNK3RBX12DialogChoice13getUserDialogEv — RBX::DialogChoice::getUserDialog(void)const
// type: _DWORD __fastcall(RBX::DialogChoice *__hidden this)
pub fn stub_0x493b28() {
    // IDA 0x493b28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogChoice::getResponseDialog(void)const")]
// 0x493b58 — __ZNK3RBX12DialogChoice17getResponseDialogEv — RBX::DialogChoice::getResponseDialog(void)const
// type: _DWORD __fastcall(RBX::DialogChoice *__hidden this)
pub fn stub_0x493b58() {
    // IDA 0x493b58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DialogChoice::~DialogChoice()")]
// 0x493b64 — __ZN3RBX12DialogChoiceD1Ev — RBX::DialogChoice::~DialogChoice()
// type: void __fastcall(RBX::DialogChoice *__hidden this)
pub fn stub_0x493b64() {
    // IDA 0x493b64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogChoice::~DialogChoice()")]
// 0x493cb8 — __ZN3RBX12DialogChoiceD0Ev — RBX::DialogChoice::~DialogChoice()
// type: void __fastcall(RBX::DialogChoice *__hidden this)
pub fn stub_0x493cb8() {
    // IDA 0x493cb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::DialogChoice::~DialogChoice()")]
// 0x493d68 — __ZThn32_N3RBX12DialogChoiceD1Ev — non-virtual thunk to RBX::DialogChoice::~DialogChoice()
// was: non-virtual thunk to RBX::DialogChoice::~DialogChoice()
// type: void __fastcall(RBX::DialogChoice *this, int, int, int)
pub fn stub_0x493d68() {
    // IDA 0x493d68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::DialogChoice::~DialogChoice()")]
// 0x493ebc — __ZThn32_N3RBX12DialogChoiceD0Ev — non-virtual thunk to RBX::DialogChoice::~DialogChoice()
// was: non-virtual thunk to RBX::DialogChoice::~DialogChoice()
// type: void __fastcall(RBX::DialogChoice *__hidden this)
pub fn stub_0x493ebc() {
    // IDA 0x493ebc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::DialogChoice::~DialogChoice()")]
// 0x494038 — __ZThn36_N3RBX12DialogChoiceD1Ev — non-virtual thunk to RBX::DialogChoice::~DialogChoice()
// was: non-virtual thunk to RBX::DialogChoice::~DialogChoice()
// type: void __fastcall(RBX::DialogChoice *__hidden this)
pub fn stub_0x494038() {
    // IDA 0x494038: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::DialogChoice::~DialogChoice()")]
// 0x49418c — __ZThn36_N3RBX12DialogChoiceD0Ev — non-virtual thunk to RBX::DialogChoice::~DialogChoice()
// was: non-virtual thunk to RBX::DialogChoice::~DialogChoice()
// type: void __fastcall(RBX::DialogChoice *__hidden this)
pub fn stub_0x49418c() {
    // IDA 0x49418c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogRoot::setInitialPrompt(std::string)")]
// 0x495428 — __ZN3RBX10DialogRoot16setInitialPromptESs — RBX::DialogRoot::setInitialPrompt(std::string)
pub fn stub_0x495428() {
    // IDA 0x495428: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogRoot::setDialogPurpose(RBX::DialogRoot::DialogPurpose)")]
// 0x495464 — __ZN3RBX10DialogRoot16setDialogPurposeENS0_13DialogPurposeE — RBX::DialogRoot::setDialogPurpose(RBX::DialogRoot::DialogPurpose)
pub fn stub_0x495464() {
    // IDA 0x495464: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogRoot::setDialogTone(RBX::DialogRoot::DialogTone)")]
// 0x495480 — __ZN3RBX10DialogRoot13setDialogToneENS0_10DialogToneE — RBX::DialogRoot::setDialogTone(RBX::DialogRoot::DialogTone)
pub fn stub_0x495480() {
    // IDA 0x495480: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogRoot::setConversationDistance(float)")]
// 0x49549c — __ZN3RBX10DialogRoot23setConversationDistanceEf — RBX::DialogRoot::setConversationDistance(float)
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this, float)
pub fn stub_0x49549c() {
    // IDA 0x49549c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogRoot::setInUse(bool)")]
// 0x4954c4 — __ZN3RBX10DialogRoot8setInUseEb — RBX::DialogRoot::setInUse(bool)
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this, bool)
pub fn stub_0x4954c4() {
    // IDA 0x4954c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogRoot::DialogRoot(void)")]
// 0x495c94 — __ZN3RBX10DialogRootC2Ev — RBX::DialogRoot::DialogRoot(void)
// type: RBX::Instance *__fastcall(RBX::DialogRoot *this)
pub fn stub_0x495c94() {
    // IDA 0x495c94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DialogRoot::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x495f88 — __ZN3RBX10DialogRoot17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::DialogRoot::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
pub fn stub_0x495f88() {
    // IDA 0x495f88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DialogRoot::getInitialPrompt(void)const")]
// 0x496158 — __ZNK3RBX10DialogRoot16getInitialPromptEv — RBX::DialogRoot::getInitialPrompt(void)const
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this)
pub fn stub_0x496158() {
    // IDA 0x496158: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DialogRoot::getDialogPurpose(void)const")]
// 0x496188 — __ZNK3RBX10DialogRoot16getDialogPurposeEv — RBX::DialogRoot::getDialogPurpose(void)const
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this)
pub fn stub_0x496188() {
    // IDA 0x496188: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DialogRoot::getDialogTone(void)const")]
// 0x4961b0 — __ZNK3RBX10DialogRoot13getDialogToneEv — RBX::DialogRoot::getDialogTone(void)const
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this)
pub fn stub_0x4961b0() {
    // IDA 0x4961b0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DialogRoot::getConversationDistance(void)const")]
// 0x4961d8 — __ZNK3RBX10DialogRoot23getConversationDistanceEv — RBX::DialogRoot::getConversationDistance(void)const
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this)
pub fn stub_0x4961d8() {
    // IDA 0x4961d8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DialogRoot::getInUse(void)const")]
// 0x496200 — __ZNK3RBX10DialogRoot8getInUseEv — RBX::DialogRoot::getInUse(void)const
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this)
pub fn stub_0x496200() {
    // IDA 0x496200: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DialogRoot> RBX::shared_from<RBX::DialogRoot>(RBX::DialogRoot*)")]
// 0x496bd8 — __ZN3RBX11shared_fromINS_10DialogRootEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::DialogRoot> RBX::shared_from<RBX::DialogRoot>(RBX::DialogRoot*)
// was: boost::shared_ptr<RBX::DialogRoot> RBX::shared_from<RBX::DialogRoot>(RBX::DialogRoot*)
// type: int(void)
pub fn stub_0x496bd8() {
    // IDA 0x496bd8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::DialogRoot::~DialogRoot()")]
// 0x496d48 — __ZN3RBX10DialogRootD1Ev — RBX::DialogRoot::~DialogRoot()
// type: void __fastcall(RBX::DialogRoot *__hidden this)
pub fn stub_0x496d48() {
    // IDA 0x496d48: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::DialogRoot::~DialogRoot()")]
// 0x496d4c — __ZN3RBX10DialogRootD0Ev — RBX::DialogRoot::~DialogRoot()
// type: void __fastcall(RBX::DialogRoot *__hidden this)
pub fn stub_0x496d4c() {
    // IDA 0x496d4c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk to RBX::DialogRoot::~DialogRoot()")]
// 0x496dfc — __ZThn32_N3RBX10DialogRootD1Ev — non-virtual thunk to RBX::DialogRoot::~DialogRoot()
// was: non-virtual thunk to RBX::DialogRoot::~DialogRoot()
// type: void __fastcall(RBX::DialogRoot *__hidden this)
pub fn stub_0x496dfc() {
    // IDA 0x496dfc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::DialogRoot::~DialogRoot()")]
// 0x496e04 — __ZThn32_N3RBX10DialogRootD0Ev — non-virtual thunk to RBX::DialogRoot::~DialogRoot()
// was: non-virtual thunk to RBX::DialogRoot::~DialogRoot()
// type: void __fastcall(RBX::DialogRoot *__hidden this)
pub fn stub_0x496e04() {
    // IDA 0x496e04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::DialogRoot::~DialogRoot()")]
// 0x496eb8 — __ZThn36_N3RBX10DialogRootD1Ev — non-virtual thunk to RBX::DialogRoot::~DialogRoot()
// was: non-virtual thunk to RBX::DialogRoot::~DialogRoot()
// type: void __fastcall(RBX::DialogRoot *__hidden this)
pub fn stub_0x496eb8() {
    // IDA 0x496eb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::DialogRoot::~DialogRoot()")]
// 0x496ec0 — __ZThn36_N3RBX10DialogRootD0Ev — non-virtual thunk to RBX::DialogRoot::~DialogRoot()
// was: non-virtual thunk to RBX::DialogRoot::~DialogRoot()
// type: void __fastcall(RBX::DialogRoot *__hidden this)
pub fn stub_0x496ec0() {
    // IDA 0x496ec0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::resize(unsigned long,RBX::DialogRoot::DialogTone)")]
// 0x497efc — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE6resizeEmS2_ — std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::resize(unsigned long,RBX::DialogRoot::DialogTone)
// type: int(void)
pub fn stub_0x497efc() {
    // IDA 0x497efc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::push_back(RBX::DialogRoot::DialogTone const&)")]
// 0x497f30 — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE9push_backERKS2_ — std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::push_back(RBX::DialogRoot::DialogTone const&)
// type: int(void)
pub fn stub_0x497f30() {
    // IDA 0x497f30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DialogRoot::DialogTone,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::operator[](RBX::Name const* const&)")]
// 0x497f58 — __ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot10DialogToneESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::DialogRoot::DialogTone,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::operator[](RBX::Name const* const&)
// type: int(void)
pub fn stub_0x497f58() {
    // IDA 0x497f58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
// 0x497fb0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x497fb0() {
    // IDA 0x497fb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
// 0x498064 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)
// type: int(void)
pub fn stub_0x498064() {
    // IDA 0x498064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
// 0x4980bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)
// type: int(void)
pub fn stub_0x4980bc() {
    // IDA 0x4980bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,RBX::DialogRoot::DialogTone const&)")]
// 0x498124 — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,RBX::DialogRoot::DialogTone const&)
// type: int(void)
pub fn stub_0x498124() {
    // IDA 0x498124: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_allocate(unsigned long)")]
// 0x498208 — __ZNSt12_Vector_baseIN3RBX10DialogRoot10DialogToneESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_allocate(unsigned long)
// type: int __fastcall(int, unsigned int)
pub fn stub_0x498208() {
    // IDA 0x498208: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DialogRoot::DialogTone * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *>(RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *)")]
// 0x498220 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot10DialogToneES6_EET0_T_S8_S7_ — RBX::DialogRoot::DialogTone * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *>(RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *)
// type: int(void)
pub fn stub_0x498220() {
    // IDA 0x498220: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,unsigned long,RBX::DialogRoot::DialogTone const&)")]
// 0x49825c — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,unsigned long,RBX::DialogRoot::DialogTone const&)
// type: int(void)
pub fn stub_0x49825c() {
    // IDA 0x49825c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "non-virtual thunk to RBX::Explosion::~Explosion()")]
// 0x49fc88 — __ZThn32_N3RBX9ExplosionD0Ev — non-virtual thunk to RBX::Explosion::~Explosion()
// was: non-virtual thunk to RBX::Explosion::~Explosion()
// type: void __fastcall(RBX::Explosion *__hidden this)
pub fn stub_0x49fc88() {
    // IDA 0x49fc88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Explosion::~Explosion()")]
// 0x49fc90 — __ZThn36_N3RBX9ExplosionD0Ev — non-virtual thunk to RBX::Explosion::~Explosion()
// was: non-virtual thunk to RBX::Explosion::~Explosion()
// type: void __fastcall(RBX::Explosion *__hidden this)
pub fn stub_0x49fc90() {
    // IDA 0x49fc90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Explosion::~Explosion()")]
// 0x49fc98 — __ZThn116_N3RBX9ExplosionD0Ev — non-virtual thunk to RBX::Explosion::~Explosion()
// was: non-virtual thunk to RBX::Explosion::~Explosion()
// type: void __fastcall(RBX::Explosion *__hidden this)
pub fn stub_0x49fc98() {
    // IDA 0x49fc98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Explosion::~Explosion()")]
// 0x49fca0 — __ZThn128_N3RBX9ExplosionD0Ev — non-virtual thunk to RBX::Explosion::~Explosion()
// was: non-virtual thunk to RBX::Explosion::~Explosion()
// type: void __fastcall(RBX::Explosion *__hidden this)
pub fn stub_0x49fca0() {
    // IDA 0x49fca0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::~Explosion()")]
// 0x49fca8 — __ZN3RBX9ExplosionD2Ev — RBX::Explosion::~Explosion()
// type: void __fastcall(RBX::Explosion *this, int, int, const void *)
pub fn stub_0x49fca8() {
    // IDA 0x49fca8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Explosion::~Explosion()")]
// 0x49fee8 — __ZThn32_N3RBX9ExplosionD1Ev — non-virtual thunk to RBX::Explosion::~Explosion()
// was: non-virtual thunk to RBX::Explosion::~Explosion()
// type: void __fastcall(RBX::Explosion *__hidden this)
pub fn stub_0x49fee8() {
    // IDA 0x49fee8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Explosion::~Explosion()")]
// 0x49fef0 — __ZThn36_N3RBX9ExplosionD1Ev — non-virtual thunk to RBX::Explosion::~Explosion()
// was: non-virtual thunk to RBX::Explosion::~Explosion()
// type: void __fastcall(RBX::Explosion *__hidden this)
pub fn stub_0x49fef0() {
    // IDA 0x49fef0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Explosion::~Explosion()")]
// 0x49fef8 — __ZThn116_N3RBX9ExplosionD1Ev — non-virtual thunk to RBX::Explosion::~Explosion()
// was: non-virtual thunk to RBX::Explosion::~Explosion()
// type: void __fastcall(RBX::Explosion *__hidden this)
pub fn stub_0x49fef8() {
    // IDA 0x49fef8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Explosion::~Explosion()")]
// 0x49ff00 — __ZThn128_N3RBX9ExplosionD1Ev — non-virtual thunk to RBX::Explosion::~Explosion()
// was: non-virtual thunk to RBX::Explosion::~Explosion()
// type: void __fastcall(RBX::Explosion *__hidden this)
pub fn stub_0x49ff00() {
    // IDA 0x49ff00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::onStepped(RBX::Stepped const&)")]
// 0x4a0098 — __ZN3RBX9Explosion9onSteppedERKNS_7SteppedE — RBX::Explosion::onStepped(RBX::Stepped const&)
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, char, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x4a0098() {
    // IDA 0x4a0098: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Explosion::onStepped(RBX::Stepped const&)")]
// 0x4a0318 — __ZThn116_N3RBX9Explosion9onSteppedERKNS_7SteppedE — non-virtual thunk to RBX::Explosion::onStepped(RBX::Stepped const&)
// was: non-virtual thunk to RBX::Explosion::onStepped(RBX::Stepped const&)
pub fn stub_0x4a0318() {
    // IDA 0x4a0318: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::render3dAdorn(RBX::Adorn *)")]
// 0x4a0320 — __ZN3RBX9Explosion13render3dAdornEPNS_5AdornE — RBX::Explosion::render3dAdorn(RBX::Adorn *)
// type: _DWORD __fastcall(RBX::Explosion *__hidden this, RBX::Adorn *)
pub fn stub_0x4a0320() {
    // IDA 0x4a0320: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Explosion::render3dAdorn(RBX::Adorn *)")]
// 0x4a0430 — __ZThn92_N3RBX9Explosion13render3dAdornEPNS_5AdornE — non-virtual thunk to RBX::Explosion::render3dAdorn(RBX::Adorn *)
// was: non-virtual thunk to RBX::Explosion::render3dAdorn(RBX::Adorn *)
// type: _DWORD __fastcall(RBX::Explosion *__hidden this, RBX::Adorn *)
pub fn stub_0x4a0430() {
    // IDA 0x4a0430: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::getBlastRadius(void)const")]
// 0x4a0438 — __ZNK3RBX9Explosion14getBlastRadiusEv — RBX::Explosion::getBlastRadius(void)const
// type: _DWORD __fastcall(RBX::Explosion *__hidden this)
pub fn stub_0x4a0438() {
    // IDA 0x4a0438: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::getExplosionType(void)const")]
// 0x4a048c — __ZNK3RBX9Explosion16getExplosionTypeEv — RBX::Explosion::getExplosionType(void)const
// type: _DWORD __fastcall(RBX::Explosion *__hidden this)
pub fn stub_0x4a048c() {
    // IDA 0x4a048c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x4a1334 — __ZN3RBX9Explosion17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::Explosion::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
// type: _DWORD __fastcall(RBX::Explosion *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
pub fn stub_0x4a1334() {
    // IDA 0x4a1334: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Explosion::shouldRender3dAdorn(void)const")]
// 0x4a1350 — __ZNK3RBX9Explosion19shouldRender3dAdornEv — RBX::Explosion::shouldRender3dAdorn(void)const
// type: _DWORD __fastcall(RBX::Explosion *__hidden this)
pub fn stub_0x4a1350() {
    // IDA 0x4a1350: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::Explosion::shouldRender3dAdorn(void)const")]
// 0x4a1368 — __ZThn92_NK3RBX9Explosion19shouldRender3dAdornEv — non-virtual thunk to RBX::Explosion::shouldRender3dAdorn(void)const
// was: non-virtual thunk to RBX::Explosion::shouldRender3dAdorn(void)const
// type: _DWORD __fastcall(RBX::Explosion *__hidden this)
pub fn stub_0x4a1368() {
    // IDA 0x4a1368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IAdornable::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
// 0x4a1370 — __ZN3RBX10IAdornable14render3dSelectEPNS_5AdornENS_11SelectStateE — RBX::IAdornable::render3dSelect(RBX::Adorn *,RBX::SelectState)
pub fn stub_0x4a1370() {
    // IDA 0x4a1370: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::construct_func(char const*,char *)")]
// 0x4a14d0 — __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::construct_func(char const*,char *)
pub fn stub_0x4a14d0() {
    // IDA 0x4a14d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::resize(unsigned long,RBX::Explosion::ExplosionType)")]
// 0x4a33b8 — __ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE6resizeEmS2_ — std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::resize(unsigned long,RBX::Explosion::ExplosionType)
// type: int(void)
pub fn stub_0x4a33b8() {
    // IDA 0x4a33b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::push_back(RBX::Explosion::ExplosionType const&)")]
// 0x4a33f0 — __ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE9push_backERKS2_ — std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::push_back(RBX::Explosion::ExplosionType const&)
// type: int(void)
pub fn stub_0x4a33f0() {
    // IDA 0x4a33f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Explosion::ExplosionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::operator[](RBX::Name const* const&)")]
// 0x4a341c — __ZNSt3mapIPKN3RBX4NameENS0_9Explosion13ExplosionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::Explosion::ExplosionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::operator[](RBX::Name const* const&)
// type: int(void)
pub fn stub_0x4a341c() {
    // IDA 0x4a341c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
// 0x4a3474 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x4a3474() {
    // IDA 0x4a3474: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
// 0x4a3528 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)
// type: int(void)
pub fn stub_0x4a3528() {
    // IDA 0x4a3528: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
// 0x4a3580 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)
// type: int __fastcall(int, int, int *)
pub fn stub_0x4a3580() {
    // IDA 0x4a3580: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,RBX::Explosion::ExplosionType const&)")]
// 0x4a35ec — __ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,RBX::Explosion::ExplosionType const&)
// type: int(void)
pub fn stub_0x4a35ec() {
    // IDA 0x4a35ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_allocate(unsigned long)")]
// 0x4a36d0 — __ZNSt12_Vector_baseIN3RBX9Explosion13ExplosionTypeESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_allocate(unsigned long)
// type: int(void)
pub fn stub_0x4a36d0() {
    // IDA 0x4a36d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Explosion::ExplosionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *>(RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *)")]
// 0x4a36e8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9Explosion13ExplosionTypeES6_EET0_T_S8_S7_ — RBX::Explosion::ExplosionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *>(RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *)
// type: int(void)
pub fn stub_0x4a36e8() {
    // IDA 0x4a36e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,unsigned long,RBX::Explosion::ExplosionType const&)")]
// 0x4a3728 — __ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,unsigned long,RBX::Explosion::ExplosionType const&)
// type: int(void)
pub fn stub_0x4a3728() {
    // IDA 0x4a3728: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::IAdornable::shouldRender3dAdorn(void)const")]
// 0x4a6868 — __ZNK3RBX10IAdornable19shouldRender3dAdornEv — RBX::IAdornable::shouldRender3dAdorn(void)const
// type: _DWORD __fastcall(RBX::IAdornable *__hidden this)
pub fn stub_0x4a6868() {
    // IDA 0x4a6868: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>> *)")]
// 0x4a6870 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>> *)
// type: int(void)
pub fn stub_0x4a6870() {
    // IDA 0x4a6870: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FactoryRegistrator::FactoryRegistrator(void)")]
// 0x4aa8c4 — __ZN3RBX18FactoryRegistratorC1Ev — RBX::FactoryRegistrator::FactoryRegistrator(void)
// type: int __fastcall(RBX::FactoryRegistrator *this)
pub fn stub_0x4aa8c4() {
    // IDA 0x4aa8c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FactoryRegistrator::FactoryRegistrator(void)")]
// 0x4aa8c8 — __ZN3RBX18FactoryRegistratorC2Ev — RBX::FactoryRegistrator::FactoryRegistrator(void)
// type: _DWORD __fastcall(RBX::FactoryRegistrator *__hidden this)
pub fn stub_0x4aa8c8() {
    // IDA 0x4aa8c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "onSlotException(std::exception &)")]
// 0x4aaa9c — __ZL15onSlotExceptionRSt9exception — onSlotException(std::exception &)
// type: _DWORD __fastcall(std::exception *)
pub fn stub_0x4aaa9c() {
    // IDA 0x4aaa9c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BindableEvent::BindableEvent(void)")]
// 0x4ab904 — __ZN3RBX13BindableEventC2Ev — RBX::BindableEvent::BindableEvent(void)
// type: _DWORD __fastcall(RBX::BindableEvent *__hidden this)
pub fn stub_0x4ab904() {
    // IDA 0x4ab904: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BindableFunction::BindableFunction(void)")]
// 0x4ac88c — __ZN3RBX16BindableFunctionC2Ev — RBX::BindableFunction::BindableFunction(void)
// type: _DWORD __fastcall(RBX::BindableFunction *__hidden this)
pub fn stub_0x4ac88c() {
    // IDA 0x4ac88c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::~deque()")]
// 0x4acacc — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EED2Ev — std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::~deque()
// type: int __fastcall(int *, int, int, int)
pub fn stub_0x4acacc() {
    // IDA 0x4acacc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::~_Deque_base()")]
// 0x4acbb4 — __ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EED2Ev — std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::~_Deque_base()
pub fn stub_0x4acbb4() {
    // IDA 0x4acbb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_destroy_data_aux(std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>)")]
// 0x4acbe0 — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE19_M_destroy_data_auxESt15_Deque_iteratorIS2_RS2_PS2_ES8_ — std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_destroy_data_aux(std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
pub fn stub_0x4acbe0() {
    // IDA 0x4acbe0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
