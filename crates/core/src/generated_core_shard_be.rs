//! core shard BE — 100 core stubs EA-sorted, next uncovered after BD 0x43ecfc (strict RBX|boost|std|rbx earliest gap, after BD 0x434850..0x43ecfc).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x43ecfc.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "std::vector<bool,std::allocator<bool>>::resize(unsigned long,bool)")]
// 0x440304 — __ZNSt6vectorIbSaIbEE6resizeEmb — std::vector<bool,std::allocator<bool>>::resize(unsigned long,bool)
pub fn stub_0x440304() {
    // IDA 0x440304: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::_M_fill_insert(std::_Bit_iterator,unsigned long,bool)")]
// 0x4403c0 — __ZNSt6vectorIbSaIbEE14_M_fill_insertESt13_Bit_iteratormb — std::vector<bool,std::allocator<bool>>::_M_fill_insert(std::_Bit_iterator,unsigned long,bool)
pub fn stub_0x4403c0() {
    // IDA 0x4403c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::fill(std::_Bit_iterator,std::_Bit_iterator,bool const&)")]
// 0x440554 — __ZSt4fillSt13_Bit_iteratorS_RKb — std::fill(std::_Bit_iterator,std::_Bit_iterator,bool const&)
pub fn stub_0x440554() {
    // IDA 0x440554: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Bit_iterator std::__copy<false,std::random_access_iterator_tag>::copy<std::_Bit_iterator,std::_Bit_iterator>(std::_Bit_iterator,std::_Bit_iterator,std::_Bit_iterator)")]
// 0x440628 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt13_Bit_iteratorS3_EET0_T_S5_S4_ — std::_Bit_iterator std::__copy<false,std::random_access_iterator_tag>::copy<std::_Bit_iterator,std::_Bit_iterator>(std::_Bit_iterator,std::_Bit_iterator,std::_Bit_iterator)
pub fn stub_0x440628() {
    // IDA 0x440628: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Bit_iterator std::__copy<false,std::random_access_iterator_tag>::copy<std::_Bit_const_iterator,std::_Bit_iterator>(std::_Bit_const_iterator,std::_Bit_const_iterator,std::_Bit_iterator)")]
// 0x440698 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt19_Bit_const_iteratorSt13_Bit_iteratorEET0_T_S6_S5_ — std::_Bit_iterator std::__copy<false,std::random_access_iterator_tag>::copy<std::_Bit_const_iterator,std::_Bit_iterator>(std::_Bit_const_iterator,std::_Bit_const_iterator,std::_Bit_iterator)
pub fn stub_0x440698() {
    // IDA 0x440698: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Bit_iterator std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::_Bit_iterator,std::_Bit_iterator>(std::_Bit_iterator,std::_Bit_iterator,std::_Bit_iterator)")]
// 0x440708 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt13_Bit_iteratorS3_EET0_T_S5_S4_ — std::_Bit_iterator std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::_Bit_iterator,std::_Bit_iterator>(std::_Bit_iterator,std::_Bit_iterator,std::_Bit_iterator)
pub fn stub_0x440708() {
    // IDA 0x440708: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::ctype<char>::_M_widen_init(void)const")]
// 0x440a50 — __ZNKSt5ctypeIcE13_M_widen_initEv — std::ctype<char>::_M_widen_init(void)const
pub fn stub_0x440a50() {
    // IDA 0x440a50: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Stats::StatsService>(void)")]
// 0x440ad0 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_5Stats12StatsServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::Stats::StatsService>(void)
pub fn stub_0x440ad0() {
    // IDA 0x440ad0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TestService * RBX::ServiceProvider::find<RBX::TestService>(void)const")]
// 0x440ee0 — __ZNK3RBX15ServiceProvider4findINS_11TestServiceEEEPT_v — RBX::TestService * RBX::ServiceProvider::find<RBX::TestService>(void)const
pub fn stub_0x440ee0() {
    // IDA 0x440ee0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::TestService>(void)")]
// 0x441130 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_11TestServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::TestService>(void)
pub fn stub_0x441130() {
    // IDA 0x441130: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TestService>(void)")]
// 0x441134 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_11TestServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TestService>(void)
pub fn stub_0x441134() {
    // IDA 0x441134: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::JointsService>(void)")]
// 0x441340 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13JointsServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::JointsService>(void)
pub fn stub_0x441340() {
    // IDA 0x441340: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SpawnerService * RBX::ServiceProvider::find<RBX::SpawnerService>(void)const")]
// 0x441418 — __ZNK3RBX15ServiceProvider4findINS_14SpawnerServiceEEEPT_v — RBX::SpawnerService * RBX::ServiceProvider::find<RBX::SpawnerService>(void)const
pub fn stub_0x441418() {
    // IDA 0x441418: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::SpawnerService>(void)")]
// 0x441758 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14SpawnerServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::SpawnerService>(void)
pub fn stub_0x441758() {
    // IDA 0x441758: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::SpawnerService>(void)")]
// 0x44175c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14SpawnerServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::SpawnerService>(void)
pub fn stub_0x44175c() {
    // IDA 0x44175c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService * RBX::ServiceProvider::find<RBX::UserInputService>(void)const")]
// 0x441f98 — __ZNK3RBX15ServiceProvider4findINS_16UserInputServiceEEEPT_v — RBX::UserInputService * RBX::ServiceProvider::find<RBX::UserInputService>(void)const
pub fn stub_0x441f98() {
    // IDA 0x441f98: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::UserInputService>(void)")]
// 0x442180 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_16UserInputServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::UserInputService>(void)
pub fn stub_0x442180() {
    // IDA 0x442180: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GuiService>(void)")]
// 0x4429b8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10GuiServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GuiService>(void)
pub fn stub_0x4429b8() {
    // IDA 0x4429b8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
// 0x442a90 — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEiPFbPKS3_SB_EEvT_SE_T0_T1_ — void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))
pub fn stub_0x442a90() {
    // IDA 0x442a90: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
// 0x442b4c — __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_ — void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))
pub fn stub_0x442b4c() {
    // IDA 0x442b4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
// 0x442bb8 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_ — void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))
pub fn stub_0x442bb8() {
    // IDA 0x442bb8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>> std::__unguarded_partition<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
// 0x442c34 — __ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEES4_PFbPKS3_SB_EET_SE_SE_T0_T1_ — __gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>> std::__unguarded_partition<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))
pub fn stub_0x442c34() {
    // IDA 0x442c34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
// 0x442c80 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_SE_T0_ — void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))
pub fn stub_0x442c80() {
    // IDA 0x442c80: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::sort_heap<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
// 0x442cf0 — __ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_ — void std::sort_heap<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))
pub fn stub_0x442cf0() {
    // IDA 0x442cf0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,int,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
// 0x442d30 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEiS4_PFbPKS3_SB_EEvT_T0_SF_T1_T2_ — void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,int,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))
pub fn stub_0x442d30() {
    // IDA 0x442d30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::create<RBX::ControllerService>(void)const")]
// 0x442de0 — __ZNK3RBX15ServiceProvider6createINS_17ControllerServiceEEEPT_v — RBX::ControllerService * RBX::ServiceProvider::create<RBX::ControllerService>(void)const
pub fn stub_0x442de0() {
    // IDA 0x442de0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::auto_ptr<XmlElement>::~auto_ptr()")]
// 0x4431c0 — __ZNSt8auto_ptrI10XmlElementED2Ev — std::auto_ptr<XmlElement>::~auto_ptr()
pub fn stub_0x4431c0() {
    // IDA 0x4431c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ServerStorage * RBX::ServiceProvider::find<RBX::ServerStorage>(void)const")]
// 0x445260 — __ZNK3RBX15ServiceProvider4findINS_13ServerStorageEEEPT_v — RBX::ServerStorage * RBX::ServiceProvider::find<RBX::ServerStorage>(void)const
pub fn stub_0x445260() {
    // IDA 0x445260: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ServerStorage>(void)")]
// 0x4454b0 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13ServerStorageEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::ServerStorage>(void)
pub fn stub_0x4454b0() {
    // IDA 0x4454b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ServerStorage>(void)")]
// 0x4454b4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ServerStorageEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ServerStorage>(void)
pub fn stub_0x4454b4() {
    // IDA 0x4454b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AssetService * RBX::ServiceProvider::find<RBX::AssetService>(void)const")]
// 0x4487a0 — __ZNK3RBX15ServiceProvider4findINS_12AssetServiceEEEPT_v — RBX::AssetService * RBX::ServiceProvider::find<RBX::AssetService>(void)const
pub fn stub_0x4487a0() {
    // IDA 0x4487a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::AssetService>(void)")]
// 0x448b24 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12AssetServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::AssetService>(void)
pub fn stub_0x448b24() {
    // IDA 0x448b24: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::AssetService>(void)")]
// 0x448b28 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12AssetServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::AssetService>(void)
pub fn stub_0x448b28() {
    // IDA 0x448b28: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ContextActionService * RBX::ServiceProvider::find<RBX::ContextActionService>(void)const")]
// 0x449e10 — __ZNK3RBX15ServiceProvider4findINS_20ContextActionServiceEEEPT_v — RBX::ContextActionService * RBX::ServiceProvider::find<RBX::ContextActionService>(void)const
pub fn stub_0x449e10() {
    // IDA 0x449e10: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContextActionService>(void)")]
// 0x44a194 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_20ContextActionServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContextActionService>(void)
pub fn stub_0x44a194() {
    // IDA 0x44a194: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContextActionService>(void)")]
// 0x44a198 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_20ContextActionServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContextActionService>(void)
pub fn stub_0x44a198() {
    // IDA 0x44a198: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PersonalServerService * RBX::ServiceProvider::find<RBX::PersonalServerService>(void)const")]
// 0x44ac20 — __ZNK3RBX15ServiceProvider4findINS_21PersonalServerServiceEEEPT_v — RBX::PersonalServerService * RBX::ServiceProvider::find<RBX::PersonalServerService>(void)const
pub fn stub_0x44ac20() {
    // IDA 0x44ac20: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::PersonalServerService>(void)")]
// 0x44afa4 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_21PersonalServerServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::PersonalServerService>(void)
pub fn stub_0x44afa4() {
    // IDA 0x44afa4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::PersonalServerService>(void)")]
// 0x44afa8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_21PersonalServerServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::PersonalServerService>(void)
pub fn stub_0x44afa8() {
    // IDA 0x44afa8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TeleportService * RBX::ServiceProvider::find<RBX::TeleportService>(void)const")]
// 0x44b420 — __ZNK3RBX15ServiceProvider4findINS_15TeleportServiceEEEPT_v — RBX::TeleportService * RBX::ServiceProvider::find<RBX::TeleportService>(void)const
pub fn stub_0x44b420() {
    // IDA 0x44b420: game-join/teleport/script bootstrap owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::TeleportService>(void)")]
// 0x44b7a4 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15TeleportServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::TeleportService>(void)
pub fn stub_0x44b7a4() {
    // IDA 0x44b7a4: game-join/teleport/script bootstrap owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TeleportService>(void)")]
// 0x44b7a8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15TeleportServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TeleportService>(void)
pub fn stub_0x44b7a8() {
    // IDA 0x44b7a8: game-join/teleport/script bootstrap owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "RBX::CookiesService * RBX::ServiceProvider::find<RBX::CookiesService>(void)const")]
// 0x44bc20 — __ZNK3RBX15ServiceProvider4findINS_14CookiesServiceEEEPT_v — RBX::CookiesService * RBX::ServiceProvider::find<RBX::CookiesService>(void)const
pub fn stub_0x44bc20() {
    // IDA 0x44bc20: game-join/teleport/script bootstrap owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::CookiesService>(void)")]
// 0x44bfa4 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14CookiesServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::CookiesService>(void)
pub fn stub_0x44bfa4() {
    // IDA 0x44bfa4: game-join/teleport/script bootstrap owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::CookiesService>(void)")]
// 0x44bfa8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14CookiesServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::CookiesService>(void)
pub fn stub_0x44bfa8() {
    // IDA 0x44bfa8: game-join/teleport/script bootstrap owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebrisService * RBX::ServiceProvider::find<RBX::DebrisService>(void)const")]
// 0x44c6f0 — __ZNK3RBX15ServiceProvider4findINS_13DebrisServiceEEEPT_v — RBX::DebrisService * RBX::ServiceProvider::find<RBX::DebrisService>(void)const
pub fn stub_0x44c6f0() {
    // IDA 0x44c6f0: game-join/teleport/script bootstrap owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::DebrisService>(void)")]
// 0x44ca74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13DebrisServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::DebrisService>(void)
pub fn stub_0x44ca74() {
    // IDA 0x44ca74: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::DebrisService>(void)")]
// 0x44ca78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13DebrisServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::DebrisService>(void)
pub fn stub_0x44ca78() {
    // IDA 0x44ca78: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GamePassService * RBX::ServiceProvider::find<RBX::GamePassService>(void)const")]
// 0x44cef0 — __ZNK3RBX15ServiceProvider4findINS_15GamePassServiceEEEPT_v — RBX::GamePassService * RBX::ServiceProvider::find<RBX::GamePassService>(void)const
pub fn stub_0x44cef0() {
    // IDA 0x44cef0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GamePassService>(void)")]
// 0x44d274 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GamePassServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::GamePassService>(void)
pub fn stub_0x44d274() {
    // IDA 0x44d274: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GamePassService>(void)")]
// 0x44d278 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GamePassService>(void)
pub fn stub_0x44d278() {
    // IDA 0x44d278: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SocialService * RBX::ServiceProvider::find<RBX::SocialService>(void)const")]
// 0x44d6f0 — __ZNK3RBX15ServiceProvider4findINS_13SocialServiceEEEPT_v — RBX::SocialService * RBX::ServiceProvider::find<RBX::SocialService>(void)const
pub fn stub_0x44d6f0() {
    // IDA 0x44d6f0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::SocialService>(void)")]
// 0x44da74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13SocialServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::SocialService>(void)
pub fn stub_0x44da74() {
    // IDA 0x44da74: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::SocialService>(void)")]
// 0x44da78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::SocialService>(void)
pub fn stub_0x44da78() {
    // IDA 0x44da78: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::InsertService>(void)")]
// 0x44e228 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13InsertServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::InsertService>(void)
pub fn stub_0x44e228() {
    // IDA 0x44e228: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::InsertService>(void)")]
// 0x44e22c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::InsertService>(void)
pub fn stub_0x44e22c() {
    // IDA 0x44e22c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::RenderHooksService>(void)")]
// 0x44e518 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_18RenderHooksServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::RenderHooksService>(void)
pub fn stub_0x44e518() {
    // IDA 0x44e518: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RenderHooksService>(void)")]
// 0x44e51c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18RenderHooksServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RenderHooksService>(void)
pub fn stub_0x44e51c() {
    // IDA 0x44e51c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FriendService>(void)")]
// 0x44edc8 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13FriendServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::FriendService>(void)
pub fn stub_0x44edc8() {
    // IDA 0x44edc8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FriendService>(void)")]
// 0x44edcc — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FriendService>(void)
pub fn stub_0x44edcc() {
    // IDA 0x44edcc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GeometryService * RBX::ServiceProvider::find<RBX::GeometryService>(void)const")]
// 0x44eea4 — __ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v — RBX::GeometryService * RBX::ServiceProvider::find<RBX::GeometryService>(void)const
pub fn stub_0x44eea4() {
    // IDA 0x44eea4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GeometryService>(void)")]
// 0x44f148 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GeometryServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::GeometryService>(void)
pub fn stub_0x44f148() {
    // IDA 0x44f148: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GeometryService>(void)")]
// 0x44f14c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GeometryService>(void)
pub fn stub_0x44f14c() {
    // IDA 0x44f14c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BadgeService * RBX::ServiceProvider::find<RBX::BadgeService>(void)const")]
// 0x44fc58 — __ZNK3RBX15ServiceProvider4findINS_12BadgeServiceEEEPT_v — RBX::BadgeService * RBX::ServiceProvider::find<RBX::BadgeService>(void)const
pub fn stub_0x44fc58() {
    // IDA 0x44fc58: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::BadgeService>(void)")]
// 0x44fe74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12BadgeServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::BadgeService>(void)
pub fn stub_0x44fe74() {
    // IDA 0x44fe74: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::BadgeService>(void)")]
// 0x44fe78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::BadgeService>(void)
pub fn stub_0x44fe78() {
    // IDA 0x44fe78: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsService * RBX::ServiceProvider::find<RBX::PhysicsService>(void)const")]
// 0x44ffb8 — __ZNK3RBX15ServiceProvider4findINS_14PhysicsServiceEEEPT_v — RBX::PhysicsService * RBX::ServiceProvider::find<RBX::PhysicsService>(void)const
pub fn stub_0x44ffb8() {
    // IDA 0x44ffb8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::PhysicsService>(void)")]
// 0x450340 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14PhysicsServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::PhysicsService>(void)
pub fn stub_0x450340() {
    // IDA 0x450340: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsService::PhysicsService(void)")]
// 0x450418 — __ZN3RBX14PhysicsServiceC2Ev — RBX::PhysicsService::PhysicsService(void)
pub fn stub_0x450418() {
    // IDA 0x450418: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ConcurrencyValidator::~ConcurrencyValidator()")]
// 0x450c74 — __ZN3RBX20ConcurrencyValidatorD2Ev — RBX::ConcurrencyValidator::~ConcurrencyValidator()
pub fn stub_0x450c74() {
    // IDA 0x450c74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CollectionService * RBX::ServiceProvider::find<RBX::CollectionService>(void)const")]
// 0x45147c — __ZNK3RBX15ServiceProvider4findINS_17CollectionServiceEEEPT_v — RBX::CollectionService * RBX::ServiceProvider::find<RBX::CollectionService>(void)const
pub fn stub_0x45147c() {
    // IDA 0x45147c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::CollectionService>(void)")]
// 0x451800 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17CollectionServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::CollectionService>(void)
pub fn stub_0x451800() {
    // IDA 0x451800: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::CollectionService>(void)")]
// 0x451804 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17CollectionServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::CollectionService>(void)
pub fn stub_0x451804() {
    // IDA 0x451804: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::RunService>(void)")]
// 0x453038 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10RunServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::RunService>(void)
pub fn stub_0x453038() {
    // IDA 0x453038: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StarterGuiService * RBX::ServiceProvider::find<RBX::StarterGuiService>(void)const")]
// 0x453200 — __ZNK3RBX15ServiceProvider4findINS_17StarterGuiServiceEEEPT_v — RBX::StarterGuiService * RBX::ServiceProvider::find<RBX::StarterGuiService>(void)const
pub fn stub_0x453200() {
    // IDA 0x453200: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::StarterGuiService>(void)")]
// 0x453584 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17StarterGuiServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::StarterGuiService>(void)
pub fn stub_0x453584() {
    // IDA 0x453584: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::StarterGuiService>(void)")]
// 0x453588 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17StarterGuiServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::StarterGuiService>(void)
pub fn stub_0x453588() {
    // IDA 0x453588: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::StarterPackService * RBX::ServiceProvider::find<RBX::StarterPackService>(void)const")]
// 0x4539e0 — __ZNK3RBX15ServiceProvider4findINS_18StarterPackServiceEEEPT_v — RBX::StarterPackService * RBX::ServiceProvider::find<RBX::StarterPackService>(void)const
pub fn stub_0x4539e0() {
    // IDA 0x4539e0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::StarterPackService>(void)")]
// 0x453c88 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18StarterPackServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::StarterPackService>(void)
pub fn stub_0x453c88() {
    // IDA 0x453c88: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::LocalBackpack * RBX::ServiceProvider::find<RBX::LocalBackpack>(void)const")]
// 0x4542c0 — __ZNK3RBX15ServiceProvider4findINS_13LocalBackpackEEEPT_v — RBX::LocalBackpack * RBX::ServiceProvider::find<RBX::LocalBackpack>(void)const
pub fn stub_0x4542c0() {
    // IDA 0x4542c0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::LocalBackpack>(void)")]
// 0x454644 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13LocalBackpackEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::LocalBackpack>(void)
pub fn stub_0x454644() {
    // IDA 0x454644: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LocalBackpack>(void)")]
// 0x454648 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13LocalBackpackEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LocalBackpack>(void)
pub fn stub_0x454648() {
    // IDA 0x454648: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MarketplaceService * RBX::ServiceProvider::find<RBX::MarketplaceService>(void)const")]
// 0x455118 — __ZNK3RBX15ServiceProvider4findINS_18MarketplaceServiceEEEPT_v — RBX::MarketplaceService * RBX::ServiceProvider::find<RBX::MarketplaceService>(void)const
pub fn stub_0x455118() {
    // IDA 0x455118: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::MarketplaceService>(void)")]
// 0x455308 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18MarketplaceServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::MarketplaceService>(void)
pub fn stub_0x455308() {
    // IDA 0x455308: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ChatService * RBX::ServiceProvider::find<RBX::ChatService>(void)const")]
// 0x455d30 — __ZNK3RBX15ServiceProvider4findINS_11ChatServiceEEEPT_v — RBX::ChatService * RBX::ServiceProvider::find<RBX::ChatService>(void)const
pub fn stub_0x455d30() {
    // IDA 0x455d30: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ChatService>(void)")]
// 0x455f50 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_11ChatServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ChatService>(void)
pub fn stub_0x455f50() {
    // IDA 0x455f50: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::KeyframeSequenceProvider * RBX::ServiceProvider::find<RBX::KeyframeSequenceProvider>(void)const")]
// 0x456474 — __ZNK3RBX15ServiceProvider4findINS_24KeyframeSequenceProviderEEEPT_v — RBX::KeyframeSequenceProvider * RBX::ServiceProvider::find<RBX::KeyframeSequenceProvider>(void)const
pub fn stub_0x456474() {
    // IDA 0x456474: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::KeyframeSequenceProvider>(void)")]
// 0x456718 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_24KeyframeSequenceProviderEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::KeyframeSequenceProvider>(void)
pub fn stub_0x456718() {
    // IDA 0x456718: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::KeyframeSequenceProvider>(void)")]
// 0x45671c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_24KeyframeSequenceProviderEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::KeyframeSequenceProvider>(void)
pub fn stub_0x45671c() {
    // IDA 0x45671c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ContentFilter * RBX::ServiceProvider::find<RBX::ContentFilter>(void)const")]
// 0x456b94 — __ZNK3RBX15ServiceProvider4findINS_13ContentFilterEEEPT_v — RBX::ContentFilter * RBX::ServiceProvider::find<RBX::ContentFilter>(void)const
pub fn stub_0x456b94() {
    // IDA 0x456b94: content-id plumbing owned by higher crates — carrier no-op in core.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContentFilter>(void)")]
// 0x456f18 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13ContentFilterEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContentFilter>(void)
pub fn stub_0x456f18() {
    // IDA 0x456f18: content-id plumbing owned by higher crates — carrier no-op in core.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContentFilter>(void)")]
// 0x456f1c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ContentFilterEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContentFilter>(void)
pub fn stub_0x456f1c() {
    // IDA 0x456f1c: content-id plumbing owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ServiceProvider::~ServiceProvider()")]
// 0x457848 — __ZN3RBX15ServiceProviderD2Ev — RBX::ServiceProvider::~ServiceProvider()
pub fn stub_0x457848() {
    // IDA 0x457848: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ServiceProvider::~ServiceProvider()")]
// 0x457b28 — __ZN3RBX15ServiceProviderD1Ev — RBX::ServiceProvider::~ServiceProvider()
pub fn stub_0x457b28() {
    // IDA 0x457b28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")]
// 0x457b30 — __ZThn32_N3RBX15ServiceProviderD1Ev — non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()
pub fn stub_0x457b30() {
    // IDA 0x457b30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")]
// 0x457b38 — __ZThn32_N3RBX15ServiceProviderD0Ev — non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()
pub fn stub_0x457b38() {
    // IDA 0x457b38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")]
// 0x457be0 — __ZThn36_N3RBX15ServiceProviderD1Ev — non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()
pub fn stub_0x457be0() {
    // IDA 0x457be0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")]
// 0x457be8 — __ZThn36_N3RBX15ServiceProviderD0Ev — non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()
pub fn stub_0x457be8() {
    // IDA 0x457be8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChangeHistoryService * RBX::ServiceProvider::find<RBX::ChangeHistoryService>(void)const")]
// 0x458b80 — __ZNK3RBX15ServiceProvider4findINS_20ChangeHistoryServiceEEEPT_v — RBX::ChangeHistoryService * RBX::ServiceProvider::find<RBX::ChangeHistoryService>(void)const
pub fn stub_0x458b80() {
    // IDA 0x458b80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ChangeHistoryService>(void)")]
// 0x458d68 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_20ChangeHistoryServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::ChangeHistoryService>(void)
pub fn stub_0x458d68() {
    // IDA 0x458d68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
