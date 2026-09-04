//! core shard DT — 100 core stubs EA-sorted, next uncovered after DS 0x83e29c (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "non-virtual thunk toRBX::FriendService::~FriendService()")]
// 0x83e34c — __ZThn32_N3RBX13FriendServiceD1Ev
pub fn stub_83e34c() {
    // IDA 0x83e34c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FriendService::~FriendService()")]
// 0x83e354 — __ZThn32_N3RBX13FriendServiceD0Ev
pub fn stub_83e354() {
    // IDA 0x83e354: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FriendService::~FriendService()")]
// 0x83e408 — __ZThn36_N3RBX13FriendServiceD1Ev
pub fn stub_83e408() {
    // IDA 0x83e408: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FriendService::~FriendService()")]
// 0x83e410 — __ZThn36_N3RBX13FriendServiceD0Ev
pub fn stub_83e410() {
    // IDA 0x83e410: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::resize(unsigned long,RBX::FriendService::FriendEventType)")]
// 0x83e4b4 — __ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE6resizeEmS2_
pub fn stub_83e4b4() {
    // IDA 0x83e4b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::push_back(RBX::FriendService::FriendEventType const&)")]
// 0x83e4e8 — __ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE9push_backERKS2_
pub fn stub_83e4e8() {
    // IDA 0x83e4e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::FriendService::FriendEventType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::operator[](RBX::Name const* const&)")]
// 0x83e510 — __ZNSt3mapIPKN3RBX4NameENS0_13FriendService15FriendEventTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_83e510() {
    // IDA 0x83e510: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType> const&)")]
// 0x83e568 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_83e568() {
    // IDA 0x83e568: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType> const&)")]
// 0x83e61c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_83e61c() {
    // IDA 0x83e61c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType> const&)")]
// 0x83e674 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_83e674() {
    // IDA 0x83e674: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FriendService::FriendEventType*,std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>>,RBX::FriendService::FriendEventType const&)")]
// 0x83e6dc — __ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_83e6dc() {
    // IDA 0x83e6dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::_M_allocate(unsigned long)")]
// 0x83e7c0 — __ZNSt12_Vector_baseIN3RBX13FriendService15FriendEventTypeESaIS2_EE11_M_allocateEm
pub fn stub_83e7c0() {
    // IDA 0x83e7c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FriendService::FriendEventType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::FriendService::FriendEventType *,RBX::FriendService::FriendEventType *>(RBX::FriendService::FriendEventType *,RBX::FriendService::FriendEventType *,RBX::FriendService::FriendEventType *)")]
// 0x83e7d8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13FriendService15FriendEventTypeES6_EET0_T_S8_S7_
pub fn stub_83e7d8() {
    // IDA 0x83e7d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FriendService::FriendEventType*,std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>>,unsigned long,RBX::FriendService::FriendEventType const&)")]
// 0x83e814 — __ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_83e814() {
    // IDA 0x83e814: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::resize(unsigned long,RBX::FriendService::FriendStatus)")]
// 0x83e9a4 — __ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE6resizeEmS2_
pub fn stub_83e9a4() {
    // IDA 0x83e9a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::push_back(RBX::FriendService::FriendStatus const&)")]
// 0x83e9d8 — __ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE9push_backERKS2_
pub fn stub_83e9d8() {
    // IDA 0x83e9d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::FriendService::FriendStatus,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::operator[](RBX::Name const* const&)")]
// 0x83ea00 — __ZNSt3mapIPKN3RBX4NameENS0_13FriendService12FriendStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_83ea00() {
    // IDA 0x83ea00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus> const&)")]
// 0x83ea58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_83ea58() {
    // IDA 0x83ea58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus> const&)")]
// 0x83eb0c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_83eb0c() {
    // IDA 0x83eb0c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus> const&)")]
// 0x83eb64 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_83eb64() {
    // IDA 0x83eb64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FriendService::FriendStatus*,std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>>,RBX::FriendService::FriendStatus const&)")]
// 0x83ebcc — __ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_83ebcc() {
    // IDA 0x83ebcc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::_M_allocate(unsigned long)")]
// 0x83ecb0 — __ZNSt12_Vector_baseIN3RBX13FriendService12FriendStatusESaIS2_EE11_M_allocateEm
pub fn stub_83ecb0() {
    // IDA 0x83ecb0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FriendService::FriendStatus * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::FriendService::FriendStatus *,RBX::FriendService::FriendStatus *>(RBX::FriendService::FriendStatus *,RBX::FriendService::FriendStatus *,RBX::FriendService::FriendStatus *)")]
// 0x83ecc8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13FriendService12FriendStatusES6_EET0_T_S8_S7_
pub fn stub_83ecc8() {
    // IDA 0x83ecc8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FriendService::FriendStatus*,std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>>,unsigned long,RBX::FriendService::FriendStatus const&)")]
// 0x83ed04 — __ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_83ed04() {
    // IDA 0x83ed04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(int,int,RBX::FriendService::FriendStatus)>::operator()(int,int,RBX::FriendService::FriendStatus)")]
// 0x83f028 — __ZN3rbx7signals16signal_with_argsILi3EFviiN3RBX13FriendService12FriendStatusEEEclEiiS4_
pub fn stub_83f028() {
    // IDA 0x83f028: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot> &)")]
// 0x83f174 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot> &)
pub fn stub_83f174() {
    // IDA 0x83f174: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::on_error(std::exception &)")]
// 0x83f2d4 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE8on_errorERSt9exception
pub fn stub_83f2d4() {
    // IDA 0x83f2d4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot> const&)")]
// 0x83f2fc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot> const&)
pub fn stub_83f2fc() {
    // IDA 0x83f2fc: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::safe_static_init_mutex(void)")]
// 0x83f320 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE22safe_static_init_mutexEv
pub fn stub_83f320() {
    // IDA 0x83f320: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::safe_static_do_get_mutex(void)")]
// 0x83f324 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE24safe_static_do_get_mutexEv
pub fn stub_83f324() {
    // IDA 0x83f324: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,RBX::FriendService::FriendStatus>> *)")]
// 0x83f41c — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX13FriendService12FriendStatusEESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_83f41c() {
    // IDA 0x83f41c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::_Rb_tree(std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>> const&)")]
// 0x83f444 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX13FriendService12FriendStatusEESt10_Select1stIS5_ESt4lessIiESaIS5_EEC2ERKSB_
pub fn stub_83f444() {
    // IDA 0x83f444: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::_M_copy(std::_Rb_tree_node<std::pair<int const,RBX::FriendService::FriendStatus>> const*,std::_Rb_tree_node<std::pair<int const,RBX::FriendService::FriendStatus>>*)")]
// 0x83f488 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX13FriendService12FriendStatusEESt10_Select1stIS5_ESt4lessIiESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
pub fn stub_83f488() {
    // IDA 0x83f488: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,std::_Select1st<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>> const&)")]
// 0x83f5dc — __ZNSt8_Rb_treeIiSt4pairIKiSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaIS0_IS1_S5_EEEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
pub fn stub_83f5dc() {
    // IDA 0x83f5dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,std::_Select1st<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>> const&)")]
// 0x83f690 — __ZNSt8_Rb_treeIiSt4pairIKiSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaIS0_IS1_S5_EEEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
pub fn stub_83f690() {
    // IDA 0x83f690: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,std::_Select1st<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::_M_insert_unique(std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>> const&)")]
// 0x83f6dc — __ZNSt8_Rb_treeIiSt4pairIKiSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaIS0_IS1_S5_EEEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
pub fn stub_83f6dc() {
    // IDA 0x83f6dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,std::_Select1st<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::_M_create_node(std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>> const&)")]
// 0x83f744 — __ZNSt8_Rb_treeIiSt4pairIKiSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaIS0_IS1_S5_EEEESt10_Select1stISB_ES7_SaISB_EE14_M_create_nodeERKSB_
pub fn stub_83f744() {
    // IDA 0x83f744: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::erase(int const&)")]
// 0x83f828 — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE5eraseERKi
pub fn stub_83f828() {
    // IDA 0x83f828: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::equal_range(int const&)")]
// 0x83f850 — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE11equal_rangeERKi
pub fn stub_83f850() {
    // IDA 0x83f850: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::erase(std::_Rb_tree_iterator<int>,std::_Rb_tree_iterator<int>)")]
// 0x83f89c — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE5eraseESt17_Rb_tree_iteratorIiES7_
pub fn stub_83f89c() {
    // IDA 0x83f89c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,std::_Select1st<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::erase(std::_Rb_tree_iterator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>)")]
// 0x83f8fc — __ZNSt8_Rb_treeIiSt4pairIKiSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaIS0_IS1_S5_EEEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_E
pub fn stub_83f8fc() {
    // IDA 0x83f8fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>)")]
// 0x83fbe4 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEES1_S3_ENS6_5list5INS6_5valueISB_EENSK_IiEENSK_ISG_EENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_
// was: void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_83fbe4() {
    // IDA 0x83fbe4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x83fd54 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEEPSsPSt9exceptionENS3_5list5INS3_5valueIS8_EENSK_IiEENSK_ISD_EENS_3argILi1EEENSO_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_83fd54() {
    // IDA 0x83fd54: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// 0x83fd70 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEEPSsPSt9exceptionENS3_5list5INS3_5valueIS8_EENSK_IiEENSK_ISD_EENS_3argILi1EEENSO_ILi2EEEEEEEvSE_SG_E6invokeERNS1_15function_bufferESE_SG_
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)
pub fn stub_83fd70() {
    // IDA 0x83fd70: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// 0x83fd90 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEES3_S5_ENS8_5list5INS8_5valueISD_EENSM_IiEENSM_ISI_EENS_3argILi1EEENSQ_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_83fd90() {
    // IDA 0x83fd90: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x83fef0 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEES3_S5_ENS8_5list5INS8_5valueISD_EENSM_IiEENSM_ISI_EENS_3argILi1EEENSQ_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_83fef0() {
    // IDA 0x83fef0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x84004c — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEES3_S5_ENS8_5list5INS8_5valueISD_EENSM_IiEENSM_ISI_EENS_3argILi1EEENSQ_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_84004c() {
    // IDA 0x84004c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>::operator()<void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0x840194 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_ISt3setIiSt4lessIiESaIiEEEENS_3argILi1EEENSF_ILi2EEEEclIPFvS6_iSD_PSsPSt9exceptionENS0_5list2IRSK_RSM_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>::operator()<void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)
pub fn stub_840194() {
    // IDA 0x840194: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x8402e4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEEPSsPSt9exceptionENS3_5list5INS3_5valueIS8_EENSK_IiEENSK_ISD_EENS_3argILi1EEENSO_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_8402e4() {
    // IDA 0x8402e4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>)")]
// 0x8404c8 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_ISt3setIiSt4lessIiESaIiEEEENS_3argILi1EEENSF_ILi2EEEEC2ES7_S8_SE_SG_SH_
// was: boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>::list5(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>)
pub fn stub_8404c8() {
    // IDA 0x8404c8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>)")]
// 0x840614 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_ISt3setIiSt4lessIiESaIiEEEENS_3argILi1EEENSF_ILi2EEEEC2ES7_S8_SE_SG_SH_
// was: boost::_bi::storage5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>)
pub fn stub_840614() {
    // IDA 0x840614: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>)")]
// 0x840760 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_ISt3setIiSt4lessIiESaIiEEEENS_3argILi1EEEEC2ES7_S8_SE_SG_
// was: boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>)
pub fn stub_840760() {
    // IDA 0x840760: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>)")]
// 0x8408ac — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_ISt3setIiSt4lessIiESaIiEEEEEC2ES7_S8_SE_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>)
pub fn stub_8408ac() {
    // IDA 0x8408ac: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>)")]
// 0x8409f0 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEEEC2ES7_S8_
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>)
pub fn stub_8409f0() {
    // IDA 0x8409f0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::FriendService>::weak_ptr<RBX::FriendService>(rbx_core::SharedPtr<RBX::FriendService> const&,boost::detail::sp_enable_if_convertible<RBX::FriendService,RBX::FriendService>::type)")]
// 0x840b40 — __ZN5boost8weak_ptrIN3RBX13FriendServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// was: boost::weak_ptr<RBX::FriendService>::weak_ptr<RBX::FriendService>(boost::shared_ptr<RBX::FriendService> const&,boost::detail::sp_enable_if_convertible<RBX::FriendService,RBX::FriendService>::type)
pub fn stub_840b40() {
    // IDA 0x840b40: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x841098 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiNS_10shared_ptrISt3mapIiNS7_12FriendStatusESt4lessIiESaISt4pairIKiSB_EEEEEENS3_5list3INS3_5valueIS8_EENSN_IiEENSN_ISJ_EEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_841098() {
    // IDA 0x841098: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x8416cc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiNS_10shared_ptrISt3mapIiNS7_12FriendStatusESt4lessIiESaISt4pairIKiSB_EEEEEENS3_5list3INS3_5valueIS8_EENSN_IiEENSN_ISJ_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_8416cc() {
    // IDA 0x8416cc: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>)")]
// 0x8418d0 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_INS_10shared_ptrISt3mapIiNS5_12FriendStatusESt4lessIiESaISt4pairIKiSB_EEEEEEEEC2ES7_S8_SK_
// was: boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::list3(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>)
pub fn stub_8418d0() {
    // IDA 0x8418d0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>)")]
// 0x841a30 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_INS_10shared_ptrISt3mapIiNS5_12FriendStatusESt4lessIiESaISt4pairIKiSB_EEEEEEEEC2ES7_S8_SK_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>)
pub fn stub_841a30() {
    // IDA 0x841a30: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,RBX::FriendService::FriendStatus>>,std::pair<int const,RBX::FriendService::FriendStatus> const&)")]
// 0x841b84 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX13FriendService12FriendStatusEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_841b84() {
    // IDA 0x841b84: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,RBX::FriendService::FriendStatus> const&)")]
// 0x841c38 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX13FriendService12FriendStatusEESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_841c38() {
    // IDA 0x841c38: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::_M_insert_unique(std::pair<int const,RBX::FriendService::FriendStatus> const&)")]
// 0x841c90 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX13FriendService12FriendStatusEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_841c90() {
    // IDA 0x841c90: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>(std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>> *)")]
// 0x841cf8 — __ZN5boost10shared_ptrISt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS4_EEEEC2ISB_EEPT_
// was: boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>(std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>> *)
pub fn stub_841cf8() {
    // IDA 0x841cf8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>(std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>> *)")]
// 0x841dcc — __ZN5boost6detail12shared_countC2ISt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS6_EEEEEPT_
pub fn stub_841dcc() {
    // IDA 0x841dcc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>::~sp_counted_impl_p()")]
// 0x841ec8 — __ZN5boost6detail17sp_counted_impl_pISt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS5_EEEED1Ev
pub fn stub_841ec8() {
    // IDA 0x841ec8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>::~sp_counted_impl_p()")]
// 0x841ecc — __ZN5boost6detail17sp_counted_impl_pISt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS5_EEEED0Ev
pub fn stub_841ecc() {
    // IDA 0x841ecc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>::dispose(void)")]
// 0x841ed0 — __ZN5boost6detail17sp_counted_impl_pISt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS5_EEEE7disposeEv
pub fn stub_841ed0() {
    // IDA 0x841ed0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>::get_deleter(std::type_info const&)")]
// 0x841ef0 — __ZN5boost6detail17sp_counted_impl_pISt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS5_EEEE11get_deleterERKSt9type_info
pub fn stub_841ef0() {
    // IDA 0x841ef0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>::get_untyped_deleter(void)")]
// 0x841ef4 — __ZN5boost6detail17sp_counted_impl_pISt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS5_EEEE19get_untyped_deleterEv
pub fn stub_841ef4() {
    // IDA 0x841ef4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FriendService>::shared_ptr<RBX::FriendService>(rbx_core::WeakPtr<RBX::FriendService> const&,boost::detail::sp_nothrow_tag)")]
// 0x841ef8 — __ZN5boost10shared_ptrIN3RBX13FriendServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::FriendService>::shared_ptr<RBX::FriendService>(boost::weak_ptr<RBX::FriendService> const&,boost::detail::sp_nothrow_tag)
pub fn stub_841ef8() {
    // IDA 0x841ef8: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::_M_insert_unique(std::pair<int,int> const&)")]
// 0x841f74 — __ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE16_M_insert_uniqueERKS1_
pub fn stub_841f74() {
    // IDA 0x841f74: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int,int> const&)")]
// 0x841ffc — __ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE9_M_insertEPSt18_Rb_tree_node_baseS9_RKS1_
pub fn stub_841ffc() {
    // IDA 0x841ffc: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::erase(std::pair<int,int> const&)")]
// 0x842064 — __ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE5eraseERKS1_
pub fn stub_842064() {
    // IDA 0x842064: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::erase(std::_Rb_tree_iterator<std::pair<int,int>>,std::_Rb_tree_iterator<std::pair<int,int>>)")]
// 0x84208c — __ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE5eraseESt17_Rb_tree_iteratorIS1_ES9_
pub fn stub_84208c() {
    // IDA 0x84208c: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::_M_erase(std::_Rb_tree_node<std::pair<int,int>> *)")]
// 0x8420ec — __ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE8_M_eraseEPSt13_Rb_tree_nodeIS1_E
pub fn stub_8420ec() {
    // IDA 0x8420ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::lower_bound(std::pair<int,int> const&)")]
// 0x842114 — __ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE11lower_boundERKS1_
pub fn stub_842114() {
    // IDA 0x842114: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::upper_bound(std::pair<int,int> const&)")]
// 0x84214c — __ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE11upper_boundERKS1_
pub fn stub_84214c() {
    // IDA 0x84214c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(int,int,RBX::FriendService::FriendEventType)>::operator()(int,int,RBX::FriendService::FriendEventType)")]
// 0x842318 — __ZN3rbx7signals16signal_with_argsILi3EFviiN3RBX13FriendService15FriendEventTypeEEEclEiiS4_
pub fn stub_842318() {
    // IDA 0x842318: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot> &)")]
// 0x842464 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot> &)
pub fn stub_842464() {
    // IDA 0x842464: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::on_error(std::exception &)")]
// 0x8425c4 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE8on_errorERSt9exception
pub fn stub_8425c4() {
    // IDA 0x8425c4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot> const&)")]
// 0x8425ec — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot> const&)
pub fn stub_8425ec() {
    // IDA 0x8425ec: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::safe_static_init_mutex(void)")]
// 0x842610 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE22safe_static_init_mutexEv
pub fn stub_842610() {
    // IDA 0x842610: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::safe_static_do_get_mutex(void)")]
// 0x842614 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE24safe_static_do_get_mutexEv
pub fn stub_842614() {
    // IDA 0x842614: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::find(std::pair<int,int> const&)")]
// 0x84270c — __ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE4findERKS1_
pub fn stub_84270c() {
    // IDA 0x84270c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
// 0x842778 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvS3_iiS4_EENS9_5list4INS9_5valueIPS3_EENS8_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEEENS0_10connectionERKT_
pub fn stub_842778() {
    // IDA 0x842778: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::insert(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot *)")]
// 0x8427ec — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE6insertEPNS6_4slotE
pub fn stub_8427ec() {
    // IDA 0x8427ec: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot>::operator=(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot*)")]
// 0x8429f8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot>::operator=(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot*)
pub fn stub_8429f8() {
    // IDA 0x8429f8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// 0x842a1c — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvS3_iiS4_EENS9_5list4INS9_5valueIPS3_EENS8_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEED1Ev
pub fn stub_842a1c() {
    // IDA 0x842a1c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// 0x842a48 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvS3_iiS4_EENS9_5list4INS9_5valueIPS3_EENS8_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEED0Ev
pub fn stub_842a48() {
    // IDA 0x842a48: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot::disconnect(void)")]
// 0x842b1c — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slot10disconnectEv
pub fn stub_842b1c() {
    // IDA 0x842b1c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot::connected(void)const")]
// 0x842c2c — __ZNK3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slot9connectedEv
pub fn stub_842c2c() {
    // IDA 0x842c2c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(int,int,RBX::FriendService::FriendEventType)>::call(int,int,RBX::FriendService::FriendEventType)")]
// 0x842c38 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvS4_iiS5_EENSA_5list4INSA_5valueIPS4_EENS9_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEELi3ES6_E4callEiiS5_
pub fn stub_842c38() {
    // IDA 0x842c38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(int,int,RBX::FriendService::FriendEventType)>::call(int,int,RBX::FriendService::FriendEventType)")]
// 0x842c64 — __ZThn4_N3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvS4_iiS5_EENSA_5list4INSA_5valueIPS4_EENS9_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEELi3ES6_E4callEiiS5_
pub fn stub_842c64() {
    // IDA 0x842c64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::FriendService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType>,boost::_bi::list3<int &,int &,RBX::FriendService::FriendEventType&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType> &,boost::_bi::list3<int &,int &,RBX::FriendService::FriendEventType&> &,int)")]
// 0x842c90 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX13FriendServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_iiNS4_15FriendEventTypeEEENS0_5list3IRiSI_RSF_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_842c90() {
    // IDA 0x842c90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::remove(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot *)")]
// 0x842cc4 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE6removeEPNS6_4slotE
pub fn stub_842cc4() {
    // IDA 0x842cc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot::safe_static_init_mutex(void)")]
// 0x842db4 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slot22safe_static_init_mutexEv
pub fn stub_842db4() {
    // IDA 0x842db4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot::safe_static_do_get_mutex(void)")]
// 0x842db8 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slot24safe_static_do_get_mutexEv
pub fn stub_842db8() {
    // IDA 0x842db8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot::~slot()")]
// 0x842ea8 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotD1Ev
pub fn stub_842ea8() {
    // IDA 0x842ea8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot::~slot()")]
// 0x842ed4 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotD0Ev
pub fn stub_842ed4() {
    // IDA 0x842ed4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(int,int,RBX::FriendService::FriendEventType)>::~callable()")]
// 0x842fa8 — __ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvS4_iiS5_EENSA_5list4INSA_5valueIPS4_EENS9_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEELi3ES6_ED1Ev
pub fn stub_842fa8() {
    // IDA 0x842fa8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
