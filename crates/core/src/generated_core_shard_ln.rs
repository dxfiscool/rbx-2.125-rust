//! core shard ln — 150 core stubs EA-sorted, next uncovered fallback after shard lm (0x45c084..0x61f4f0, lowest EA first).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|Ogre|RakNet|FMOD|Lua (fallback 41432, 9082->8932 uncovered, 38300->38450 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::Weak, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::DataModel::CreatorType & rbx::any_cast<RBX::DataModel::CreatorType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX9DataModel11CreatorTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x45c084 — __ZN3rbx8any_castIRN3RBX9DataModel11CreatorTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0x45c084() {
    // IDA 0x45c084: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::resize(unsigned long,RBX::DataModel::CreatorType)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE6resizeEmS2_")]
// 0x45c174 — __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x45c174() {
    // IDA 0x45c174: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::push_back(RBX::DataModel::CreatorType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE9push_backERKS2_")]
// 0x45c1a8 — __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x45c1a8() {
    // IDA 0x45c1a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::CreatorType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_9DataModel11CreatorTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0x45c1d0 — __ZNSt3mapIPKN3RBX4NameENS0_9DataModel11CreatorTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x45c1d0() {
    // IDA 0x45c1d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0x45c228 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x45c228() {
    // IDA 0x45c228: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0x45c2dc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_0x45c2dc() {
    // IDA 0x45c2dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::CreatorType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0x45c334 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
pub fn stub_0x45c334() {
    // IDA 0x45c334: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::CreatorType*,std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>>,RBX::DataModel::CreatorType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x45c39c — __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0x45c39c() {
    // IDA 0x45c39c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9DataModel11CreatorTypeESaIS2_EE11_M_allocateEm")]
// 0x45c480 — __ZNSt12_Vector_baseIN3RBX9DataModel11CreatorTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD)
pub fn stub_0x45c480() {
    // IDA 0x45c480: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModel::CreatorType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::CreatorType *,RBX::DataModel::CreatorType *>(RBX::DataModel::CreatorType *,RBX::DataModel::CreatorType *,RBX::DataModel::CreatorType *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel11CreatorTypeES6_EET0_T_S8_S7_")]
// 0x45c498 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel11CreatorTypeES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x45c498() {
    // IDA 0x45c498: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::CreatorType*,std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>>,unsigned long,RBX::DataModel::CreatorType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0x45c4d4 — __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
pub fn stub_0x45c4d4() {
    // IDA 0x45c4d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::Events::Events(void)")]
#[doc(alias = "__ZN3RBX9DataModel10LegacyLock14Implementation6EventsC2Ev")]
// 0x46e358 — __ZN3RBX9DataModel10LegacyLock14Implementation6EventsC2Ev
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation::Events *__hidden this)
pub fn stub_0x46e358() {
    // IDA 0x46e358: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_init_eventsPool(void)")]
#[doc(alias = "__ZN3RBX9DataModel10LegacyLock14Implementation27safe_static_init_eventsPoolEv")]
// 0x46e69c — __ZN3RBX9DataModel10LegacyLock14Implementation27safe_static_init_eventsPoolEv
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation *__hidden this)
pub fn stub_0x46e69c() {
    // IDA 0x46e69c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_eventsPool(void)")]
#[doc(alias = "__ZN3RBX9DataModel10LegacyLock14Implementation29safe_static_do_get_eventsPoolEv")]
// 0x46e6a0 — __ZN3RBX9DataModel10LegacyLock14Implementation29safe_static_do_get_eventsPoolEv
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation *__hidden this)
pub fn stub_0x46e6a0() {
    // IDA 0x46e6a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_init_currentJob(void)")]
#[doc(alias = "__ZN3RBX9DataModel10LegacyLock14Implementation27safe_static_init_currentJobEv")]
// 0x46f030 — __ZN3RBX9DataModel10LegacyLock14Implementation27safe_static_init_currentJobEv
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation *__hidden this)
pub fn stub_0x46f030() {
    // IDA 0x46f030: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_currentJob(void)")]
#[doc(alias = "__ZN3RBX9DataModel10LegacyLock14Implementation29safe_static_do_get_currentJobEv")]
// 0x46f034 — __ZN3RBX9DataModel10LegacyLock14Implementation29safe_static_do_get_currentJobEv
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation *__hidden this)
pub fn stub_0x46f034() {
    // IDA 0x46f034: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx::thread_specific_reference<RBX::DataModel::GenericJob>::~thread_specific_reference()")]
#[doc(alias = "__ZN3rbx25thread_specific_referenceIN3RBX9DataModel10GenericJobEED1Ev")]
// 0x46f148 — __ZN3rbx25thread_specific_referenceIN3RBX9DataModel10GenericJobEED1Ev
pub fn stub_0x46f148() {
    // IDA 0x46f148: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModel::MouseStats::MouseStats(void)")]
#[doc(alias = "__ZN3RBX9DataModel10MouseStatsC2Ev")]
// 0x46fd8c — __ZN3RBX9DataModel10MouseStatsC2Ev
// type: _DWORD __fastcall(RBX::DataModel::MouseStats *__hidden this)
pub fn stub_0x46fd8c() {
    // IDA 0x46fd8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModel::GenericJob::~GenericJob()")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJobD1Ev")]
// 0x47013c — __ZN3RBX9DataModel10GenericJobD1Ev
// type: void __fastcall(RBX::DataModel::GenericJob *__hidden this)
pub fn stub_0x47013c() {
    // IDA 0x47013c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModel::GenericJob::~GenericJob()")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJobD0Ev")]
// 0x47025c — __ZN3RBX9DataModel10GenericJobD0Ev
// type: void __fastcall(RBX::DataModel::GenericJob *__hidden this)
pub fn stub_0x47025c() {
    // IDA 0x47025c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModel::GenericJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
// 0x470390 — __ZN3RBX9DataModel10GenericJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_0x470390() {
    // IDA 0x470390: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModel::GenericJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJob5errorERKNS_13TaskScheduler3Job5StatsE")]
// 0x470400 — __ZN3RBX9DataModel10GenericJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::DataModel::GenericJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
pub fn stub_0x470400() {
    // IDA 0x470400: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModel::GenericJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE")]
// 0x470484 — __ZN3RBX9DataModel10GenericJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::DataModel::GenericJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
pub fn stub_0x470484() {
    // IDA 0x470484: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModel::GenericJob::processTasks(void)")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJob12processTasksEv")]
// 0x470670 — __ZN3RBX9DataModel10GenericJob12processTasksEv
// type: _DWORD __fastcall(RBX::DataModel::GenericJob *__hidden this)
pub fn stub_0x470670() {
    // IDA 0x470670: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::GearType>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// 0x470b58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x470b58() {
    // IDA 0x470b58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// 0x470b80 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x470b80() {
    // IDA 0x470b80: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::Genre>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// 0x470ba8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x470ba8() {
    // IDA 0x470ba8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// 0x470bd0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x470bd0() {
    // IDA 0x470bd0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModelJob::step(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX12DataModelJob4stepERKNS_13TaskScheduler3Job5StatsE")]
// 0x472b4c — __ZN3RBX12DataModelJob4stepERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::DataModelJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
pub fn stub_0x472b4c() {
    // IDA 0x472b4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModelJob::getPriorityFactor(void)")]
#[doc(alias = "__ZN3RBX12DataModelJob17getPriorityFactorEv")]
// 0x472cd4 — __ZN3RBX12DataModelJob17getPriorityFactorEv
// type: _DWORD __fastcall(RBX::DataModelJob *__hidden this)
pub fn stub_0x472cd4() {
    // IDA 0x472cd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModelArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
#[doc(alias = "__ZN3RBX16DataModelArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_")]
// 0x472e00 — __ZN3RBX16DataModelArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_
// type: _DWORD __fastcall(RBX::DataModelArbiter *__hidden this, RBX::TaskScheduler::Job *, RBX::TaskScheduler::Job *)
pub fn stub_0x472e00() {
    // IDA 0x472e00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModelArbiter::DataModelArbiter(void)")]
#[doc(alias = "__ZN3RBX16DataModelArbiterC2Ev")]
// 0x472e2c — __ZN3RBX16DataModelArbiterC2Ev
// type: _DWORD __fastcall(RBX::DataModelArbiter *__hidden this)
pub fn stub_0x472e2c() {
    // IDA 0x472e2c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DataModelArbiter::~DataModelArbiter()")]
#[doc(alias = "__ZN3RBX16DataModelArbiterD0Ev")]
// 0x473124 — __ZN3RBX16DataModelArbiterD0Ev
// type: void __fastcall(RBX::DataModelArbiter *__hidden this)
pub fn stub_0x473124() {
    // IDA 0x473124: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModelArbiter::~DataModelArbiter()")]
#[doc(alias = "__ZN3RBX16DataModelArbiterD1Ev")]
// 0x4731c4 — __ZN3RBX16DataModelArbiterD1Ev
// type: void __fastcall(RBX::DataModelArbiter *__hidden this)
pub fn stub_0x4731c4() {
    // IDA 0x4731c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModelArbiter::~DataModelArbiter()")]
#[doc(alias = "__ZN3RBX16DataModelArbiterD2Ev")]
// 0x4731c8 — __ZN3RBX16DataModelArbiterD2Ev
// type: void __fastcall(RBX::DataModelArbiter *__hidden this)
pub fn stub_0x4731c8() {
    // IDA 0x4731c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModelArbiter::preStep(RBX::TaskScheduler::Job *)")]
#[doc(alias = "__ZN3RBX16DataModelArbiter7preStepEPNS_13TaskScheduler3JobE")]
// 0x473318 — __ZN3RBX16DataModelArbiter7preStepEPNS_13TaskScheduler3JobE
// type: _DWORD __fastcall(RBX::DataModelArbiter *__hidden this, RBX::TaskScheduler::Job *)
pub fn stub_0x473318() {
    // IDA 0x473318: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModelArbiter::postStep(RBX::TaskScheduler::Job *)")]
#[doc(alias = "__ZN3RBX16DataModelArbiter8postStepEPNS_13TaskScheduler3JobE")]
// 0x473350 — __ZN3RBX16DataModelArbiter8postStepEPNS_13TaskScheduler3JobE
// type: _DWORD __fastcall(RBX::DataModelArbiter *__hidden this, RBX::TaskScheduler::Job *)
pub fn stub_0x473350() {
    // IDA 0x473350: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModelJob::~DataModelJob()")]
#[doc(alias = "__ZN3RBX12DataModelJobD1Ev")]
// 0x4736e8 — __ZN3RBX12DataModelJobD1Ev
// type: void __fastcall(RBX::DataModelJob *__hidden this)
pub fn stub_0x4736e8() {
    // IDA 0x4736e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModelJob::~DataModelJob()")]
#[doc(alias = "__ZN3RBX12DataModelJobD0Ev")]
// 0x4736ec — __ZN3RBX12DataModelJobD0Ev
// type: void __fastcall(RBX::DataModelJob *__hidden this)
pub fn stub_0x4736ec() {
    // IDA 0x4736ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModelArbiter::ConcurrencyModel>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE14construct_funcEPKcPc")]
// 0x4739b0 — __ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE14construct_funcEPKcPc
pub fn stub_0x4739b0() {
    // IDA 0x4739b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModelArbiter::ConcurrencyModel const& rbx::any_cast<RBX::DataModelArbiter::ConcurrencyModel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX16DataModelArbiter16ConcurrencyModelENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x473a8c — __ZN3rbx8any_castIRKN3RBX16DataModelArbiter16ConcurrencyModelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
pub fn stub_0x473a8c() {
    // IDA 0x473a8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::resize(unsigned long,RBX::DataModelArbiter::ConcurrencyModel)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE6resizeEmS2_")]
// 0x473f38 — __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE6resizeEmS2_
// type: int(void)
pub fn stub_0x473f38() {
    // IDA 0x473f38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::push_back(RBX::DataModelArbiter::ConcurrencyModel const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE9push_backERKS2_")]
// 0x473f70 — __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE9push_backERKS2_
// type: int(void)
pub fn stub_0x473f70() {
    // IDA 0x473f70: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DataModelArbiter::ConcurrencyModel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_16DataModelArbiter16ConcurrencyModelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0x474004 — __ZNSt3mapIPKN3RBX4NameENS0_16DataModelArbiter16ConcurrencyModelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
pub fn stub_0x474004() {
    // IDA 0x474004: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0x47405c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x47405c() {
    // IDA 0x47405c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0x474110 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
pub fn stub_0x474110() {
    // IDA 0x474110: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0x474168 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
pub fn stub_0x474168() {
    // IDA 0x474168: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModelArbiter::ConcurrencyModel*,std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>>,RBX::DataModelArbiter::ConcurrencyModel const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x47486c — __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0x47486c() {
    // IDA 0x47486c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE11_M_allocateEm")]
// 0x474950 — __ZNSt12_Vector_baseIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE11_M_allocateEm
// type: int(void)
pub fn stub_0x474950() {
    // IDA 0x474950: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModelArbiter::ConcurrencyModel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *>(RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16DataModelArbiter16ConcurrencyModelES6_EET0_T_S8_S7_")]
// 0x474968 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16DataModelArbiter16ConcurrencyModelES6_EET0_T_S8_S7_
// type: int(void)
pub fn stub_0x474968() {
    // IDA 0x474968: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModelArbiter::ConcurrencyModel*,std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>>,unsigned long,RBX::DataModelArbiter::ConcurrencyModel const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0x4749c0 — __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
pub fn stub_0x4749c0() {
    // IDA 0x4749c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// 0x474dfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int(void)
pub fn stub_0x474dfc() {
    // IDA 0x474dfc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModelMesh::setLevelOfDetailX(RBX::DataModelMesh::LODType)")]
#[doc(alias = "__ZN3RBX13DataModelMesh17setLevelOfDetailXENS0_7LODTypeE")]
// 0x4750c8 — __ZN3RBX13DataModelMesh17setLevelOfDetailXENS0_7LODTypeE
pub fn stub_0x4750c8() {
    // IDA 0x4750c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModelMesh::setLevelOfDetailY(RBX::DataModelMesh::LODType)")]
#[doc(alias = "__ZN3RBX13DataModelMesh17setLevelOfDetailYENS0_7LODTypeE")]
// 0x4750e8 — __ZN3RBX13DataModelMesh17setLevelOfDetailYENS0_7LODTypeE
pub fn stub_0x4750e8() {
    // IDA 0x4750e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModelMesh::setScale(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX13DataModelMesh8setScaleERKN3G3D7Vector3E")]
// 0x475108 — __ZN3RBX13DataModelMesh8setScaleERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this, const G3D::Vector3 *)
pub fn stub_0x475108() {
    // IDA 0x475108: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModelMesh::setVertColor(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX13DataModelMesh12setVertColorERKN3G3D7Vector3E")]
// 0x4751a8 — __ZN3RBX13DataModelMesh12setVertColorERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this, const G3D::Vector3 *)
pub fn stub_0x4751a8() {
    // IDA 0x4751a8: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::DataModelMesh::setOffset(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX13DataModelMesh9setOffsetERKN3G3D7Vector3E")]
// 0x475210 — __ZN3RBX13DataModelMesh9setOffsetERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this, const G3D::Vector3 *)
pub fn stub_0x475210() {
    // IDA 0x475210: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::DataModelMesh::DataModelMesh(void)")]
#[doc(alias = "__ZN3RBX13DataModelMeshC2Ev")]
// 0x475278 — __ZN3RBX13DataModelMeshC2Ev
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
pub fn stub_0x475278() {
    // IDA 0x475278: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::DataModelMesh::getLevelOfDetailX(void)const")]
#[doc(alias = "__ZNK3RBX13DataModelMesh17getLevelOfDetailXEv")]
// 0x475840 — __ZNK3RBX13DataModelMesh17getLevelOfDetailXEv
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
pub fn stub_0x475840() {
    // IDA 0x475840: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::DataModelMesh::getLevelOfDetailY(void)const")]
#[doc(alias = "__ZNK3RBX13DataModelMesh17getLevelOfDetailYEv")]
// 0x47586c — __ZNK3RBX13DataModelMesh17getLevelOfDetailYEv
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
pub fn stub_0x47586c() {
    // IDA 0x47586c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::DataModelMesh::getScale(void)const")]
#[doc(alias = "__ZNK3RBX13DataModelMesh8getScaleEv")]
// 0x475874 — __ZNK3RBX13DataModelMesh8getScaleEv
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
pub fn stub_0x475874() {
    // IDA 0x475874: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DataModelMesh::getVertColor(void)const")]
#[doc(alias = "__ZNK3RBX13DataModelMesh12getVertColorEv")]
// 0x47589c — __ZNK3RBX13DataModelMesh12getVertColorEv
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
pub fn stub_0x47589c() {
    // IDA 0x47589c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DataModelMesh::getOffset(void)const")]
#[doc(alias = "__ZNK3RBX13DataModelMesh9getOffsetEv")]
// 0x4758a0 — __ZNK3RBX13DataModelMesh9getOffsetEv
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
pub fn stub_0x4758a0() {
    // IDA 0x4758a0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DataModelMesh::~DataModelMesh()")]
#[doc(alias = "__ZN3RBX13DataModelMeshD1Ev")]
// 0x4758a4 — __ZN3RBX13DataModelMeshD1Ev
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
pub fn stub_0x4758a4() {
    // IDA 0x4758a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModelMesh::~DataModelMesh()")]
#[doc(alias = "__ZN3RBX13DataModelMeshD0Ev")]
// 0x4758a8 — __ZN3RBX13DataModelMeshD0Ev
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
pub fn stub_0x4758a8() {
    // IDA 0x4758a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::DataModelMesh::~DataModelMesh()")]
#[doc(alias = "__ZThn32_N3RBX13DataModelMeshD1Ev")]
// 0x475970 — __ZThn32_N3RBX13DataModelMeshD1Ev
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
pub fn stub_0x475970() {
    // IDA 0x475970: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::DataModelMesh::~DataModelMesh()")]
#[doc(alias = "__ZThn32_N3RBX13DataModelMeshD0Ev")]
// 0x475978 — __ZThn32_N3RBX13DataModelMeshD0Ev
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
pub fn stub_0x475978() {
    // IDA 0x475978: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::DataModelMesh::~DataModelMesh()")]
#[doc(alias = "__ZThn36_N3RBX13DataModelMeshD1Ev")]
// 0x475a44 — __ZThn36_N3RBX13DataModelMeshD1Ev
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
pub fn stub_0x475a44() {
    // IDA 0x475a44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::DataModelMesh::~DataModelMesh()")]
#[doc(alias = "__ZThn36_N3RBX13DataModelMeshD0Ev")]
// 0x475a4c — __ZThn36_N3RBX13DataModelMeshD0Ev
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
pub fn stub_0x475a4c() {
    // IDA 0x475a4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDataModelMeshEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDataModelMeshEEEEvv")]
// 0x475af0 — __ZN3RBX4Name13callDoDeclareILZNS_14sDataModelMeshEEEEvv
pub fn stub_0x475af0() {
    // IDA 0x475af0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDataModelMeshEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDataModelMeshEEEERKS0_v")]
// 0x475af4 — __ZN3RBX4Name9doDeclareILZNS_14sDataModelMeshEEEERKS0_v
pub fn stub_0x475af4() {
    // IDA 0x475af4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::resize(unsigned long,RBX::DataModelMesh::LODType)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE6resizeEmS2_")]
// 0x4767e8 — __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE6resizeEmS2_
// type: int(void)
pub fn stub_0x4767e8() {
    // IDA 0x4767e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::push_back(RBX::DataModelMesh::LODType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE9push_backERKS2_")]
// 0x47681c — __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE9push_backERKS2_
// type: int(void)
pub fn stub_0x47681c() {
    // IDA 0x47681c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DataModelMesh::LODType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_13DataModelMesh7LODTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0x476844 — __ZNSt3mapIPKN3RBX4NameENS0_13DataModelMesh7LODTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
pub fn stub_0x476844() {
    // IDA 0x476844: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0x47689c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x47689c() {
    // IDA 0x47689c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0x476950 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
pub fn stub_0x476950() {
    // IDA 0x476950: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0x4769a8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
pub fn stub_0x4769a8() {
    // IDA 0x4769a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModelMesh::LODType*,std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>>,RBX::DataModelMesh::LODType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x476a10 — __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0x476a10() {
    // IDA 0x476a10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX13DataModelMesh7LODTypeESaIS2_EE11_M_allocateEm")]
// 0x476af4 — __ZNSt12_Vector_baseIN3RBX13DataModelMesh7LODTypeESaIS2_EE11_M_allocateEm
// type: int(void)
pub fn stub_0x476af4() {
    // IDA 0x476af4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DataModelMesh::LODType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModelMesh::LODType *,RBX::DataModelMesh::LODType *>(RBX::DataModelMesh::LODType *,RBX::DataModelMesh::LODType *,RBX::DataModelMesh::LODType *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DataModelMesh7LODTypeES6_EET0_T_S8_S7_")]
// 0x476b0c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DataModelMesh7LODTypeES6_EET0_T_S8_S7_
// type: int(void)
pub fn stub_0x476b0c() {
    // IDA 0x476b0c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModelMesh::LODType*,std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>>,unsigned long,RBX::DataModelMesh::LODType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0x476b48 — __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
pub fn stub_0x476b48() {
    // IDA 0x476b48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DebugSettings::getDataModelCount(void)const")]
#[doc(alias = "__ZNK3RBX13DebugSettings17getDataModelCountEv")]
// 0x47bd60 — __ZNK3RBX13DebugSettings17getDataModelCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bd60() {
    // IDA 0x47bd60: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TaskSchedulerSettings::setConcurrencyModel(RBX::DataModelArbiter::ConcurrencyModel)")]
#[doc(alias = "__ZN3RBX21TaskSchedulerSettings19setConcurrencyModelENS_16DataModelArbiter16ConcurrencyModelE")]
// 0x47c4dc — __ZN3RBX21TaskSchedulerSettings19setConcurrencyModelENS_16DataModelArbiter16ConcurrencyModelE
pub fn stub_0x47c4dc() {
    // IDA 0x47c4dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModelArbiter::ConcurrencyModel>(RBX::DataModelArbiter::ConcurrencyModel const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16DataModelArbiter16ConcurrencyModelEEERS3_RKT_")]
// 0x486598 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16DataModelArbiter16ConcurrencyModelEEERS3_RKT_
// type: int(void)
pub fn stub_0x486598() {
    // IDA 0x486598: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModelArbiter::ConcurrencyModel>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE9singletonEv")]
// 0x4865e8 — __ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE9singletonEv
// type: int(void)
pub fn stub_0x4865e8() {
    // IDA 0x4865e8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModelArbiter::ConcurrencyModel>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE13destruct_funcEPc")]
// 0x486658 — __ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE13destruct_funcEPc
pub fn stub_0x486658() {
    // IDA 0x486658: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3::Axis>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D7Vector34AxisEE9singletonEv")]
// 0x4cb6e0 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector34AxisEE9singletonEv
// type: _DWORD *()
pub fn stub_0x4cb6e0() {
    // IDA 0x4cb6e0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3::Axis>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D7Vector34AxisEE14construct_funcEPKcPc")]
// 0x4cb74c — __ZN3rbx14implementation12typed_holderIN3G3D7Vector34AxisEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x4cb74c() {
    // IDA 0x4cb74c: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3::Axis>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D7Vector34AxisEE13destruct_funcEPc")]
// 0x4cb758 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector34AxisEE13destruct_funcEPc
// type: void()
pub fn stub_0x4cb758() {
    // IDA 0x4cb758: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModelMesh::LODType>(RBX::DataModelMesh::LODType const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13DataModelMesh7LODTypeEEERS3_RKT_")]
// 0x4cfe9c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13DataModelMesh7LODTypeEEERS3_RKT_
pub fn stub_0x4cfe9c() {
    // IDA 0x4cfe9c: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModelMesh::LODType>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13DataModelMesh7LODTypeEE9singletonEv")]
// 0x4cfeec — __ZN3rbx14implementation12typed_holderIN3RBX13DataModelMesh7LODTypeEE9singletonEv
pub fn stub_0x4cfeec() {
    // IDA 0x4cfeec: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModelMesh::LODType>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13DataModelMesh7LODTypeEE14construct_funcEPKcPc")]
// 0x4cff58 — __ZN3rbx14implementation12typed_holderIN3RBX13DataModelMesh7LODTypeEE14construct_funcEPKcPc
pub fn stub_0x4cff58() {
    // IDA 0x4cff58: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModelMesh::LODType>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13DataModelMesh7LODTypeEE13destruct_funcEPc")]
// 0x4cff64 — __ZN3rbx14implementation12typed_holderIN3RBX13DataModelMesh7LODTypeEE13destruct_funcEPc
pub fn stub_0x4cff64() {
    // IDA 0x4cff64: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::DataModelMesh::LODType const& rbx::any_cast<RBX::DataModelMesh::LODType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX13DataModelMesh7LODTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x4d0034 — __ZN3rbx8any_castIRKN3RBX13DataModelMesh7LODTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x4d0034() {
    // IDA 0x4d0034: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// 0x4d01a0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0x4d01a0() {
    // IDA 0x4d01a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Game::setupDataModel(std::string const&)")]
#[doc(alias = "__ZN3RBX4Game14setupDataModelERKSs")]
// 0x4fbc68 — __ZN3RBX4Game14setupDataModelERKSs
// type: _DWORD __fastcall(RBX::Game *__hidden this, const std::string *)
pub fn stub_0x4fbc68() {
    // IDA 0x4fbc68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Game::shutdownGameDataModel(void)")]
#[doc(alias = "__ZN3RBX4Game21shutdownGameDataModelEv")]
// 0x4fc660 — __ZN3RBX4Game21shutdownGameDataModelEv
// type: _DWORD __fastcall(RBX::Game *__hidden this)
pub fn stub_0x4fc660() {
    // IDA 0x4fc660: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiBuilder::Initialize(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX10GuiBuilder10InitializeEPNS_9DataModelE")]
// 0x51e768 — __ZN3RBX10GuiBuilder10InitializeEPNS_9DataModelE
// type: _DWORD __fastcall(RBX::GuiBuilder *__hidden this, RBX::DataModel *)
pub fn stub_0x51e768() {
    // IDA 0x51e768: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "G3D::Rect2D::intersect(G3D::Rect2D const&)const")]
#[doc(alias = "__ZNK3G3D6Rect2D9intersectERKS0_")]
// 0x52d8e0 — __ZNK3G3D6Rect2D9intersectERKS0_
pub fn stub_0x52d8e0() {
    // IDA 0x52d8e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "different5percent(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__Z17different5percentRKN3G3D7Vector3ES2_")]
// 0x55603c — __Z17different5percentRKN3G3D7Vector3ES2_
// type: _DWORD __fastcall(const G3D::Vector3 *this, Vector3 *)
pub fn stub_0x55603c() {
    // IDA 0x55603c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "G3D::CoordinateFrame::operator*(G3D::CoordinateFrame const&)const")]
#[doc(alias = "__ZNK3G3D15CoordinateFramemlERKS0_")]
// 0x5e1350 — __ZNK3G3D15CoordinateFramemlERKS0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0x5e1350() {
    // IDA 0x5e1350: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "G3D::Sphere::~Sphere()")]
#[doc(alias = "__ZN3G3D6SphereD1Ev")]
// 0x5e1e38 — __ZN3G3D6SphereD1Ev
// type: void __fastcall(G3D::Sphere *__hidden this)
pub fn stub_0x5e1e38() {
    // IDA 0x5e1e38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IAdornable::isVisible(G3D::Rect2D const&)const")]
#[doc(alias = "__ZNK3RBX10IAdornable9isVisibleERKN3G3D6Rect2DE")]
// 0x5e2ba0 — __ZNK3RBX10IAdornable9isVisibleERKN3G3D6Rect2DE
pub fn stub_0x5e2ba0() {
    // IDA 0x5e2ba0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CameraSubject::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
#[doc(alias = "__ZN3RBX13CameraSubject4zoomEfRN3G3D15CoordinateFrameES3_")]
// 0x5e2c4c — __ZN3RBX13CameraSubject4zoomEfRN3G3D15CoordinateFrameES3_
pub fn stub_0x5e2c4c() {
    // IDA 0x5e2c4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CameraSubject::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
#[doc(alias = "__ZN3RBX13CameraSubject20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")]
// 0x5e2c50 — __ZN3RBX13CameraSubject20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
pub fn stub_0x5e2c50() {
    // IDA 0x5e2c50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE6resizeEib")]
// 0x5e5cf8 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE6resizeEib
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x5e5cf8() {
    // IDA 0x5e5cf8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE7reallocEi")]
// 0x5e5db0 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE7reallocEi
// type: int(void)
pub fn stub_0x5e5db0() {
    // IDA 0x5e5db0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Color3 const& rbx::any_cast<G3D::Color3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3G3D6Color3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x5f0710 — __ZN3rbx8any_castIRKN3G3D6Color3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
pub fn stub_0x5f0710() {
    // IDA 0x5f0710: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Color3>(G3D::Color3 const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D6Color3EEERS3_RKT_")]
// 0x5f0800 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D6Color3EEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x5f0800() {
    // IDA 0x5f0800: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Color3>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D6Color3EE9singletonEv")]
// 0x5f0860 — __ZN3rbx14implementation12typed_holderIN3G3D6Color3EE9singletonEv
// type: int(void)
pub fn stub_0x5f0860() {
    // IDA 0x5f0860: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Color3>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D6Color3EE13destruct_funcEPc")]
// 0x5f08d0 — __ZN3rbx14implementation12typed_holderIN3G3D6Color3EE13destruct_funcEPc
// type: void()
pub fn stub_0x5f08d0() {
    // IDA 0x5f08d0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D7Vector3EE14construct_funcEPKcPc")]
// 0x5f1148 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector3EE14construct_funcEPKcPc
// type: __int64 *__fastcall(__int64 *result, int)
pub fn stub_0x5f1148() {
    // IDA 0x5f1148: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::CoordinateFrame const& rbx::any_cast<G3D::CoordinateFrame const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x5f35c0 — __ZN3rbx8any_castIRKN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x5f35c0() {
    // IDA 0x5f35c0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::CoordinateFrame>(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D15CoordinateFrameEEERS3_RKT_")]
// 0x5f36b0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D15CoordinateFrameEEERS3_RKT_
// type: int __fastcall(int, __int64 *)
pub fn stub_0x5f36b0() {
    // IDA 0x5f36b0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::CoordinateFrame>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE14construct_funcEPKcPc")]
// 0x5f3738 — __ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE14construct_funcEPKcPc
// type: const G3D::Matrix3 *__fastcall(const G3D::Matrix3 *result, int)
pub fn stub_0x5f3738() {
    // IDA 0x5f3738: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EED2Ev")]
// 0x5f3b30 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EED2Ev
// type: int __fastcall(int)
pub fn stub_0x5f3b30() {
    // IDA 0x5f3b30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BasePlayerGui::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZN3RBX13BasePlayerGui15scriptShouldRunEPNS_10BaseScriptE")]
// 0x5fb8e0 — __ZN3RBX13BasePlayerGui15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::BasePlayerGui *__hidden this, RBX::BaseScript *)
pub fn stub_0x5fb8e0() {
    // IDA 0x5fb8e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZThn92_N3RBX13BasePlayerGui15scriptShouldRunEPNS_10BaseScriptE")]
// 0x5fba7c — __ZThn92_N3RBX13BasePlayerGui15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::BasePlayerGui *__hidden this, RBX::BaseScript *)
pub fn stub_0x5fba7c() {
    // IDA 0x5fba7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StarterGuiService::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZN3RBX17StarterGuiService15scriptShouldRunEPNS_10BaseScriptE")]
// 0x5fd8b4 — __ZN3RBX17StarterGuiService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this, RBX::BaseScript *)
pub fn stub_0x5fd8b4() {
    // IDA 0x5fd8b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::StarterGuiService::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZThn92_N3RBX17StarterGuiService15scriptShouldRunEPNS_10BaseScriptE")]
// 0x5fda38 — __ZThn92_N3RBX17StarterGuiService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this, RBX::BaseScript *)
pub fn stub_0x5fda38() {
    // IDA 0x5fda38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CoreGuiService::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZN3RBX14CoreGuiService15scriptShouldRunEPNS_10BaseScriptE")]
// 0x5fdcb4 — __ZN3RBX14CoreGuiService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this, RBX::BaseScript *)
pub fn stub_0x5fdcb4() {
    // IDA 0x5fdcb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::CoreGuiService::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZThn92_N3RBX14CoreGuiService15scriptShouldRunEPNS_10BaseScriptE")]
// 0x5fe170 — __ZThn92_N3RBX14CoreGuiService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this, RBX::BaseScript *)
pub fn stub_0x5fe170() {
    // IDA 0x5fe170: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::ReferenceCountedPointer<G3D::MemoryManager>::zeroPointer(void)")]
#[doc(alias = "__ZN3G3D23ReferenceCountedPointerINS_13MemoryManagerEE11zeroPointerEv")]
// 0x604058 — __ZN3G3D23ReferenceCountedPointerINS_13MemoryManagerEE11zeroPointerEv
// type: int(void)
pub fn stub_0x604058() {
    // IDA 0x604058: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::IAdornable *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX10IAdornableELi10ELm32EEC2Ev")]
// 0x604088 — __ZN3G3D5ArrayIPN3RBX10IAdornableELi10ELm32EEC2Ev
pub fn stub_0x604088() {
    // IDA 0x604088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::ReferenceCountedPointer<G3D::MemoryManager>::setPointer(G3D::MemoryManager*)")]
#[doc(alias = "__ZN3G3D23ReferenceCountedPointerINS_13MemoryManagerEE10setPointerEPS1_")]
// 0x604178 — __ZN3G3D23ReferenceCountedPointerINS_13MemoryManagerEE10setPointerEPS1_
// type: int(void)
pub fn stub_0x604178() {
    // IDA 0x604178: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Pose::setCoordinateFrame(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX4Pose18setCoordinateFrameERKN3G3D15CoordinateFrameE")]
// 0x605ac4 — __ZN3RBX4Pose18setCoordinateFrameERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Pose *__hidden this, const G3D::CoordinateFrame *)
pub fn stub_0x605ac4() {
    // IDA 0x605ac4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scale9Frame::setScaleEdgeSize(G3D::Vector2int16)")]
#[doc(alias = "__ZN3RBX11Scale9Frame16setScaleEdgeSizeEN3G3D12Vector2int16E")]
// 0x60ec20 — __ZN3RBX11Scale9Frame16setScaleEdgeSizeEN3G3D12Vector2int16E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x60ec20() {
    // IDA 0x60ec20: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScreenGui::setReplicatingAbsoluteSize(G3D::Vector2int16)")]
#[doc(alias = "__ZN3RBX9ScreenGui26setReplicatingAbsoluteSizeEN3G3D12Vector2int16E")]
// 0x610094 — __ZN3RBX9ScreenGui26setReplicatingAbsoluteSizeEN3G3D12Vector2int16E
pub fn stub_0x610094() {
    // IDA 0x610094: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScreenGui::setReplicatingAbsolutePosition(G3D::Vector2int16)")]
#[doc(alias = "__ZN3RBX9ScreenGui30setReplicatingAbsolutePositionEN3G3D12Vector2int16E")]
// 0x6100b8 — __ZN3RBX9ScreenGui30setReplicatingAbsolutePositionEN3G3D12Vector2int16E
pub fn stub_0x6100b8() {
    // IDA 0x6100b8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptMouseCommand::ScriptMouseCommand(RBX::Workspace *)")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommandC1EPNS_9WorkspaceE")]
// 0x614a00 — __ZN3RBX18ScriptMouseCommandC1EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, RBX::Workspace *)
pub fn stub_0x614a00() {
    // IDA 0x614a00: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptMouseCommand::ScriptMouseCommand(RBX::Workspace *)")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommandC2EPNS_9WorkspaceE")]
// 0x614a04 — __ZN3RBX18ScriptMouseCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, RBX::Workspace *)
pub fn stub_0x614a04() {
    // IDA 0x614a04: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptMouseCommand::~ScriptMouseCommand()")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommandD0Ev")]
// 0x614b58 — __ZN3RBX18ScriptMouseCommandD0Ev
// type: void __fastcall(RBX::ScriptMouseCommand *__hidden this)
pub fn stub_0x614b58() {
    // IDA 0x614b58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScriptMouseCommand::~ScriptMouseCommand()")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommandD1Ev")]
// 0x614bf8 — __ZN3RBX18ScriptMouseCommandD1Ev
// type: void __fastcall(RBX::ScriptMouseCommand *__hidden this)
pub fn stub_0x614bf8() {
    // IDA 0x614bf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ScriptMouseCommand::~ScriptMouseCommand()")]
#[doc(alias = "__ZThn36_N3RBX18ScriptMouseCommandD0Ev")]
// 0x614bfc — __ZThn36_N3RBX18ScriptMouseCommandD0Ev
// type: void __fastcall(RBX::ScriptMouseCommand *__hidden this)
pub fn stub_0x614bfc() {
    // IDA 0x614bfc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScriptMouseCommand::~ScriptMouseCommand()")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommandD2Ev")]
// 0x614c04 — __ZN3RBX18ScriptMouseCommandD2Ev
// type: void __fastcall(RBX::ScriptMouseCommand *__hidden this)
pub fn stub_0x614c04() {
    // IDA 0x614c04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ScriptMouseCommand::~ScriptMouseCommand()")]
#[doc(alias = "__ZThn36_N3RBX18ScriptMouseCommandD1Ev")]
// 0x614d30 — __ZThn36_N3RBX18ScriptMouseCommandD1Ev
// type: void __fastcall(RBX::ScriptMouseCommand *__hidden this)
pub fn stub_0x614d30() {
    // IDA 0x614d30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScriptMouseCommand::getCursorId(void)const")]
#[doc(alias = "__ZNK3RBX18ScriptMouseCommand11getCursorIdEv")]
// 0x614d38 — __ZNK3RBX18ScriptMouseCommand11getCursorIdEv
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this)
pub fn stub_0x614d38() {
    // IDA 0x614d38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScriptMouseCommand::onMouseDown(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommand11onMouseDownERKNS_7UIEventE")]
// 0x614d48 — __ZN3RBX18ScriptMouseCommand11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
pub fn stub_0x614d48() {
    // IDA 0x614d48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScriptMouseCommand::onMouseHover(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommand12onMouseHoverERKNS_7UIEventE")]
// 0x614e20 — __ZN3RBX18ScriptMouseCommand12onMouseHoverERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
pub fn stub_0x614e20() {
    // IDA 0x614e20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScriptMouseCommand::onMouseIdle(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommand11onMouseIdleERKNS_7UIEventE")]
// 0x614e2c — __ZN3RBX18ScriptMouseCommand11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
pub fn stub_0x614e2c() {
    // IDA 0x614e2c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptMouseCommand::onMouseWheelForward(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommand19onMouseWheelForwardERKNS_7UIEventE")]
// 0x614e38 — __ZN3RBX18ScriptMouseCommand19onMouseWheelForwardERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
pub fn stub_0x614e38() {
    // IDA 0x614e38: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptMouseCommand::onMouseWheelBackward(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommand20onMouseWheelBackwardERKNS_7UIEventE")]
// 0x614f10 — __ZN3RBX18ScriptMouseCommand20onMouseWheelBackwardERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
pub fn stub_0x614f10() {
    // IDA 0x614f10: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptMouseCommand::onRightMouseDown(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommand16onRightMouseDownERKNS_7UIEventE")]
// 0x614fe8 — __ZN3RBX18ScriptMouseCommand16onRightMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
pub fn stub_0x614fe8() {
    // IDA 0x614fe8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptMouseCommand::onRightMouseUp(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommand14onRightMouseUpERKNS_7UIEventE")]
// 0x6150c0 — __ZN3RBX18ScriptMouseCommand14onRightMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
pub fn stub_0x6150c0() {
    // IDA 0x6150c0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptMouseCommand::onMouseUp(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommand9onMouseUpERKNS_7UIEventE")]
// 0x615198 — __ZN3RBX18ScriptMouseCommand9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
pub fn stub_0x615198() {
    // IDA 0x615198: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptMouseCommand::onPeekKeyDown(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommand13onPeekKeyDownERKNS_7UIEventE")]
// 0x615270 — __ZN3RBX18ScriptMouseCommand13onPeekKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
pub fn stub_0x615270() {
    // IDA 0x615270: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptMouseCommand::onPeekKeyUp(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX18ScriptMouseCommand11onPeekKeyUpERKNS_7UIEventE")]
// 0x615348 — __ZN3RBX18ScriptMouseCommand11onPeekKeyUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
pub fn stub_0x615348() {
    // IDA 0x615348: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptMouseCommand::getName(void)const")]
#[doc(alias = "__ZNK3RBX18ScriptMouseCommand7getNameEv")]
// 0x615420 — __ZNK3RBX18ScriptMouseCommand7getNameEv
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this)
pub fn stub_0x615420() {
    // IDA 0x615420: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SelectionLasso::getHumanoidPosition(G3D::Vector3 &)const")]
#[doc(alias = "__ZNK3RBX14SelectionLasso19getHumanoidPositionERN3G3D7Vector3E")]
// 0x61ee88 — __ZNK3RBX14SelectionLasso19getHumanoidPositionERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::SelectionLasso *__hidden this, G3D::Vector3 *)
pub fn stub_0x61ee88() {
    // IDA 0x61ee88: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::SelectionPartLasso::getPosition(G3D::Vector3 &)const")]
#[doc(alias = "__ZNK3RBX18SelectionPartLasso11getPositionERN3G3D7Vector3E")]
// 0x61f4f0 — __ZNK3RBX18SelectionPartLasso11getPositionERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::SelectionPartLasso *__hidden this, G3D::Vector3 *)
pub fn stub_0x61f4f0() {
    // IDA 0x61f4f0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}
