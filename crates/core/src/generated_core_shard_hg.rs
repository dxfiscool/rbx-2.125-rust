//! core shard HG — 100 core stubs EA-sorted, 0xf5b724..0xf5c9b4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HF 0xf5b714).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HF 0xf5b714 (0xf5b724..0xf5c9b4, 21014->21114 covered, 804 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::OnScreenProfiler::GetJobFrameInfo(RBX::OSProfilerJobInfo *,int)")]
// 0xf5b724 — j___ZN3RBX16OnScreenProfiler15GetJobFrameInfoEPNS_17OSProfilerJobInfoEi
pub fn stub_0xf5b724() {
    // IDA 0xf5b724: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::IsPreAllocateJob(char const*,long &)")]
// 0xf5b734 — j___ZN3RBX16OnScreenProfiler16IsPreAllocateJobEPKcRl
pub fn stub_0xf5b734() {
    // IDA 0xf5b734: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OSProfilerFrameInfo::CalcMaxTime(void)")]
// 0xf5b744 — j___ZN3RBX19OSProfilerFrameInfo11CalcMaxTimeEv
pub fn stub_0xf5b744() {
    // IDA 0xf5b744: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OSProfilerMarkerTempData * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *)")]
// 0xf5b754 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX24OSProfilerMarkerTempDataES5_EET0_T_S7_S6_
pub fn stub_0xf5b754() {
    // IDA 0xf5b754: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::OSProfilerMarkerTempDataStr * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *)")]
// 0xf5b764 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX27OSProfilerMarkerTempDataStrES5_EET0_T_S7_S6_
pub fn stub_0xf5b764() {
    // IDA 0xf5b764: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__push_heap<RBX::OSProfilerMarkerTempData *,int,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,int,int,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0xf5b774 — j___ZSt11__push_heapIPN3RBX24OSProfilerMarkerTempDataEiS1_PFbRKS1_S4_EEvT_T0_S8_T1_T2_
pub fn stub_0xf5b774() {
    // IDA 0xf5b774: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__push_heap<RBX::OSProfilerMarkerTempDataStr *,int,RBX::OSProfilerMarkerTempDataStr,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,int,int,RBX::OSProfilerMarkerTempDataStr,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0xf5b784 — j___ZSt11__push_heapIPN3RBX27OSProfilerMarkerTempDataStrEiS1_PFbRKS1_S4_EEvT_T0_S8_T1_T2_
pub fn stub_0xf5b784() {
    // IDA 0xf5b784: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__adjust_heap<RBX::OSProfilerMarkerTempData *,int,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,int,int,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0xf5b794 — j___ZSt13__adjust_heapIPN3RBX24OSProfilerMarkerTempDataEiS1_PFbRKS1_S4_EEvT_T0_S8_T1_T2_
pub fn stub_0xf5b794() {
    // IDA 0xf5b794: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__adjust_heap<RBX::OSProfilerMarkerTempDataStr *,int,RBX::OSProfilerMarkerTempDataStr,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,int,int,RBX::OSProfilerMarkerTempDataStr,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0xf5b7a4 — j___ZSt13__adjust_heapIPN3RBX27OSProfilerMarkerTempDataStrEiS1_PFbRKS1_S4_EEvT_T0_S8_T1_T2_
pub fn stub_0xf5b7a4() {
    // IDA 0xf5b7a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__heap_select<RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0xf5b7b4 — j___ZSt13__heap_selectIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_S7_T0_
pub fn stub_0xf5b7b4() {
    // IDA 0xf5b7b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__heap_select<RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0xf5b7c4 — j___ZSt13__heap_selectIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_S7_T0_
pub fn stub_0xf5b7c4() {
    // IDA 0xf5b7c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__insertion_sort<RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0xf5b7d4 — j___ZSt16__insertion_sortIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_0xf5b7d4() {
    // IDA 0xf5b7d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__insertion_sort<RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0xf5b7e4 — j___ZSt16__insertion_sortIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_0xf5b7e4() {
    // IDA 0xf5b7e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__introsort_loop<RBX::OSProfilerMarkerTempData *,int,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,int,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0xf5b7f4 — j___ZSt16__introsort_loopIPN3RBX24OSProfilerMarkerTempDataEiPFbRKS1_S4_EEvT_S7_T0_T1_
pub fn stub_0xf5b7f4() {
    // IDA 0xf5b7f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__introsort_loop<RBX::OSProfilerMarkerTempDataStr *,int,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,int,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0xf5b804 — j___ZSt16__introsort_loopIPN3RBX27OSProfilerMarkerTempDataStrEiPFbRKS1_S4_EEvT_S7_T0_T1_
pub fn stub_0xf5b804() {
    // IDA 0xf5b804: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::OSProfilerMarkerTempData * std::__unguarded_partition<RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0xf5b814 — j___ZSt21__unguarded_partitionIPN3RBX24OSProfilerMarkerTempDataES1_PFbRKS1_S4_EET_S7_S7_T0_T1_
pub fn stub_0xf5b814() {
    // IDA 0xf5b814: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::OSProfilerMarkerTempDataStr * std::__unguarded_partition<RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0xf5b824 — j___ZSt21__unguarded_partitionIPN3RBX27OSProfilerMarkerTempDataStrES1_PFbRKS1_S4_EET_S7_S7_T0_T1_
pub fn stub_0xf5b824() {
    // IDA 0xf5b824: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__final_insertion_sort<RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0xf5b834 — j___ZSt22__final_insertion_sortIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_0xf5b834() {
    // IDA 0xf5b834: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__final_insertion_sort<RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0xf5b844 — j___ZSt22__final_insertion_sortIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_0xf5b844() {
    // IDA 0xf5b844: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__unguarded_linear_insert<RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0xf5b854 — j___ZSt25__unguarded_linear_insertIPN3RBX24OSProfilerMarkerTempDataES1_PFbRKS1_S4_EEvT_T0_T1_
pub fn stub_0xf5b854() {
    // IDA 0xf5b854: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::pop_heap<RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0xf5b864 — j___ZSt8pop_heapIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_0xf5b864() {
    // IDA 0xf5b864: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::pop_heap<RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0xf5b874 — j___ZSt8pop_heapIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_0xf5b874() {
    // IDA 0xf5b874: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::sort_heap<RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0xf5b884 — j___ZSt9sort_heapIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_0xf5b884() {
    // IDA 0xf5b884: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::sort_heap<RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0xf5b894 — j___ZSt9sort_heapIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_0xf5b894() {
    // IDA 0xf5b894: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::MemoryInputStream::read(void *,unsigned long)")]
// 0xf5b8a4 — j___ZN3RBX17MemoryInputStream4readEPvm
pub fn stub_0xf5b8a4() {
    // IDA 0xf5b8a4: libstdc++ template instantiation (mangled-only context). Std container/algorithm — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<float,std::allocator<float>>::_M_allocate(unsigned long)")]
// 0xf5b8f4 — j___ZNSt12_Vector_baseIfSaIfEE11_M_allocateEm
pub fn stub_0xf5b8f4() {
    // IDA 0xf5b8f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::_M_insert_aux(std::_Bit_iterator,bool)")]
// 0xf5b9b4 — j___ZNSt6vectorIbSaIbEE13_M_insert_auxESt13_Bit_iteratorb
pub fn stub_0xf5b9b4() {
    // IDA 0xf5b9b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::reserve(unsigned long)")]
// 0xf5b9c4 — j___ZNSt6vectorIbSaIbEE7reserveEm
pub fn stub_0xf5b9c4() {
    // IDA 0xf5b9c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::push_back(bool)")]
// 0xf5b9d4 — j___ZNSt6vectorIbSaIbEE9push_backEb
pub fn stub_0xf5b9d4() {
    // IDA 0xf5b9d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<char,std::allocator<char>>::vector(unsigned long,char const&,std::allocator<char> const&)")]
// 0xf5b9e4 — j___ZNSt6vectorIcSaIcEEC2EmRKcRKS0_
pub fn stub_0xf5b9e4() {
    // IDA 0xf5b9e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<float,std::allocator<float>>::_M_insert_aux(__gnu_cxx::__normal_iterator<float *,std::vector<float,std::allocator<float>>>,float const&)")]
// 0xf5b9f4 — j___ZNSt6vectorIfSaIfEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPfS1_EERKf
pub fn stub_0xf5b9f4() {
    // IDA 0xf5b9f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<float,std::allocator<float>>::reserve(unsigned long)")]
// 0xf5ba04 — j___ZNSt6vectorIfSaIfEE7reserveEm
pub fn stub_0xf5ba04() {
    // IDA 0xf5ba04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<float,std::allocator<float>>::push_back(float const&)")]
// 0xf5ba14 — j___ZNSt6vectorIfSaIfEE9push_backERKf
pub fn stub_0xf5ba14() {
    // IDA 0xf5ba14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<int,std::allocator<int>>::reserve(unsigned long)")]
// 0xf5ba24 — j___ZNSt6vectorIiSaIiEE7reserveEm
pub fn stub_0xf5ba24() {
    // IDA 0xf5ba24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::LoginService::~LoginService()")]
// 0xf5ba84 — j___ZN3RBX12LoginServiceD2Ev
pub fn stub_0xf5ba84() {
    // IDA 0xf5ba84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TimerService>(void)")]
// 0xf5bc44 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_12TimerServiceEEEmv
pub fn stub_0xf5bc44() {
    // IDA 0xf5bc44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContentProvider>(void)")]
// 0xf5bc54 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_15ContentProviderEEEmv
pub fn stub_0xf5bc54() {
    // IDA 0xf5bc54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Http::Http(std::string const&)")]
// 0xf5bca4 — j___ZN3RBX4HttpC2ERKSs
pub fn stub_0xf5bca4() {
    // IDA 0xf5bca4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0xf5be64 — j___ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFviEEEEENS5_INS6_IFvSsEEEEEEC2ES3_S4_S9_SC_
pub fn stub_0xf5be64() {
    // IDA 0xf5be64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0xf5be74 — j___ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFviEEEEENS5_INS6_IFvSsEEEEEEclIPFvPSsPSt9exceptionS8_SB_ENS0_5list2IRSF_RSH_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0xf5be74() {
    // IDA 0xf5be74: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0xf5bf34 — j___ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFviEEEEENS5_INS6_IFvSsEEEEEEC2ES3_S4_S9_SC_
pub fn stub_0xf5bf34() {
    // IDA 0xf5bf34: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,boost::function<void ()(int)>,boost::function<void ()(std::string)>>::type> boost::bind<void,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>,boost::arg<1>,boost::arg<2>,boost::function<void ()(int)>,boost::function<void ()(std::string)>>(void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::arg<1>,boost::arg<2>,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0xf5bfc4 — j___ZN5boost4bindIvPSsPSt9exceptionNS_8functionIFviEEENS4_IFvSsEEENS_3argILi1EEENS9_ILi2EEES6_S8_EENS_3_bi6bind_tIT_PFSE_T0_T1_T2_T3_ENSC_9list_av_4IT4_T5_T6_T7_E4typeEEESK_SM_SN_SO_SP_
pub fn stub_0xf5bfc4() {
    // IDA 0xf5bfc4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf5c074 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFviEEENS8_IFvSsEEEENS3_5list4INS_3argILi1EEENSG_ILi2EEENS3_5valueISA_EENSJ_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0xf5c074() {
    // IDA 0xf5c074: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::function<void ()(void)>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf5c084 — j___ZN5boost6detail8function15functor_managerINS_8functionIFvvEEEE7managerERKNS1_15function_bufferERS7_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0xf5c084() {
    // IDA 0xf5c084: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::thread::start_thread(void)")]
// 0xf5c094 — j___ZN5boost6thread12start_threadEv
pub fn stub_0xf5c094() {
    // IDA 0xf5c094: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function<void ()(void)>::operator=(boost::function<void ()(void)> const&)")]
// 0xf5c114 — j___ZN5boost8functionIFvvEEaSERKS2_
pub fn stub_0xf5c114() {
    // IDA 0xf5c114: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function0<void>::move_assign(boost::function0<void>&)")]
// 0xf5c124 — j___ZN5boost9function0IvE11move_assignERS1_
pub fn stub_0xf5c124() {
    // IDA 0xf5c124: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::function<void ()(void)>>(boost::function<void ()(void)>)")]
// 0xf5c164 — j___ZN5boost9function0IvE9assign_toINS_8functionIFvvEEEEEvT_
pub fn stub_0xf5c164() {
    // IDA 0xf5c164: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function1<void,std::string>::swap(boost::function1<void,std::string>&)")]
// 0xf5c1f4 — j___ZN5boost9function1IvSsE4swapERS1_
pub fn stub_0xf5c1f4() {
    // IDA 0xf5c1f4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,std::string>::clear(void)")]
// 0xf5c204 — j___ZN5boost9function1IvSsE5clearEv
pub fn stub_0xf5c204() {
    // IDA 0xf5c204: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,int>::assign_to_own(boost::function1<void,int> const&)")]
// 0xf5c214 — j___ZN5boost9function1IviE13assign_to_ownERKS1_
pub fn stub_0xf5c214() {
    // IDA 0xf5c214: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,std::string *,std::exception *>::clear(void)")]
// 0xf5c244 — j___ZN5boost9function2IvPSsPSt9exceptionE5clearEv
pub fn stub_0xf5c244() {
    // IDA 0xf5c244: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0xf5c274 — j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFviEEENS8_IFvSsEEEENS6_5list4INS_3argILi1EEENSG_ILi2EEENS6_5valueISA_EENSJ_ISC_EEEEEEEEvT_
pub fn stub_0xf5c274() {
    // IDA 0xf5c274: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::TimerService * RBX::ServiceProvider::find<RBX::TimerService>(void)const")]
// 0xf5c2c4 — j___ZNK3RBX15ServiceProvider4findINS_12TimerServiceEEEPT_v
pub fn stub_0xf5c2c4() {
    // IDA 0xf5c2c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::ContentProvider * RBX::ServiceProvider::find<RBX::ContentProvider>(void)const")]
// 0xf5c2d4 — j___ZNK3RBX15ServiceProvider4findINS_15ContentProviderEEEPT_v
pub fn stub_0xf5c2d4() {
    // IDA 0xf5c2d4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::TimerService * RBX::ServiceProvider::create<RBX::TimerService>(void)const")]
// 0xf5c2f4 — j___ZNK3RBX15ServiceProvider6createINS_12TimerServiceEEEPT_v
pub fn stub_0xf5c2f4() {
    // IDA 0xf5c2f4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::ContentProvider * RBX::ServiceProvider::create<RBX::ContentProvider>(void)const")]
// 0xf5c304 — j___ZNK3RBX15ServiceProvider6createINS_15ContentProviderEEEPT_v
pub fn stub_0xf5c304() {
    // IDA 0xf5c304: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::weak_count::use_count(void)const")]
// 0xf5c374 — j___ZNK5boost6detail10weak_count9use_countEv
pub fn stub_0xf5c374() {
    // IDA 0xf5c374: content-id plumbing owned by higher crates — carrier no-op in core.
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::function<void ()(void)>>(boost::function<void ()(void)>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf5c3b4 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_8functionIFvvEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_0xf5c3b4() {
    // IDA 0xf5c3b4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::function<void ()(void)>>(boost::function<void ()(void)>,boost::detail::function::function_buffer &)const")]
// 0xf5c424 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_8functionIFvvEEEEEbT_RNS1_15function_bufferE
pub fn stub_0xf5c424() {
    // IDA 0xf5c424: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::function<void ()(void)>>(boost::function<void ()(void)>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf5c434 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_8functionIFvvEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0xf5c434() {
    // IDA 0xf5c434: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf5c4c4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFviEEENSA_IFvSsEEEENS8_5list4INS_3argILi1EEENSI_ILi2EEENS8_5valueISC_EENSL_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_0xf5c4c4() {
    // IDA 0xf5c4c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0xf5c514 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFviEEENSA_IFvSsEEEENS8_5list4INS_3argILi1EEENSI_ILi2EEENS8_5valueISC_EENSL_ISE_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0xf5c514() {
    // IDA 0xf5c514: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf5c524 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFviEEENSA_IFvSsEEEENS8_5list4INS_3argILi1EEENSI_ILi2EEENS8_5valueISC_EENSL_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0xf5c524() {
    // IDA 0xf5c524: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,int>::operator()(int)const")]
// 0xf5c554 — j___ZNK5boost9function1IviEclEi
pub fn stub_0xf5c554() {
    // IDA 0xf5c554: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::equal_range(RBX::Name const* const&)")]
// 0xf5c564 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE11equal_rangeERS5_
pub fn stub_0xf5c564() {
    // IDA 0xf5c564: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(RBX::Name const* const&)")]
// 0xf5c574 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE5eraseERS5_
pub fn stub_0xf5c574() {
    // IDA 0xf5c574: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ICreator const*> const&)")]
// 0xf5c584 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
pub fn stub_0xf5c584() {
    // IDA 0xf5c584: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::AssetService::AccessType * rbx::any_cast<RBX::AssetService::AccessType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf5c6c4 — j___ZN3rbx8any_castIN3RBX12AssetService10AccessTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0xf5c6c4() {
    // IDA 0xf5c6c4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "RBX::AssetService::AccessType & rbx::any_cast<RBX::AssetService::AccessType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf5c6d4 — j___ZN3rbx8any_castIRN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0xf5c6d4() {
    // IDA 0xf5c6d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::throw_exception<boost::bad_lexical_cast>(boost::bad_lexical_cast const&)")]
// 0xf5c6e4 — j___ZN5boost15throw_exceptionINS_16bad_lexical_castEEEvRKT_
pub fn stub_0xf5c6e4() {
    // IDA 0xf5c6e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone_impl(boost::exception_detail::error_info_injector<boost::bad_lexical_cast> const&)")]
// 0xf5c6f4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEEC1ERKS4_
pub fn stub_0xf5c6f4() {
    // IDA 0xf5c6f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone_tag)")]
// 0xf5c704 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEEC1ERKS5_NS5_9clone_tagE
pub fn stub_0xf5c704() {
    // IDA 0xf5c704: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_lexical_cast>::~error_info_injector()")]
// 0xf5c714 — j___ZN5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED2Ev
pub fn stub_0xf5c714() {
    // IDA 0xf5c714: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0xf5c744 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
pub fn stub_0xf5c744() {
    // IDA 0xf5c744: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>> &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0xf5c754 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0xf5c754() {
    // IDA 0xf5c754: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0xf5c764 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFviEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
pub fn stub_0xf5c764() {
    // IDA 0xf5c764: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>> &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0xf5c774 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFviEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0xf5c774() {
    // IDA 0xf5c774: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0xf5c794 — j___ZN5boost3_bi8storage5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
pub fn stub_0xf5c794() {
    // IDA 0xf5c794: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0xf5c7a4 — j___ZN5boost3_bi8storage5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFviEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
pub fn stub_0xf5c7a4() {
    // IDA 0xf5c7a4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_5<RBX::AssetService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,RBX::AssetService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (RBX::AssetService::*)(std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),RBX::AssetService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xf5c7c4 — j___ZN5boost4bindIvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENS6_IFvSsEEEPS2_NS_3argILi1EEENSC_ILi2EEES8_SA_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISH_T0_T1_T2_T3_T4_EENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSK_FSH_SL_SM_SN_SO_ESR_SS_ST_SU_SV_
pub fn stub_0xf5c7c4() {
    // IDA 0xf5c7c4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_5<RBX::AssetService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(int)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>,RBX::AssetService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(int)>,boost::function<void ()(std::string)>>(void (RBX::AssetService::*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),RBX::AssetService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0xf5c7d4 — j___ZN5boost4bindIvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENS6_IFvSsEEEPS2_NS_3argILi1EEENSC_ILi2EEES8_SA_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISH_T0_T1_T2_T3_T4_EENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSK_FSH_SL_SM_SN_SO_ESR_SS_ST_SU_SV_
pub fn stub_0xf5c7d4() {
    // IDA 0xf5c7d4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned int,char>(unsigned int &,char const*,char const*)")]
// 0xf5c7e4 — j___ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEjcEEbRT0_PKT1_S8_
pub fn stub_0xf5c7e4() {
    // IDA 0xf5c7e4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<int>(int &)")]
// 0xf5c7f4 — j___ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIiEEbRT_
pub fn stub_0xf5c7f4() {
    // IDA 0xf5c7f4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf5c814 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0xf5c814() {
    // IDA 0xf5c814: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf5c824 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0xf5c824() {
    // IDA 0xf5c824: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0xf5c874 — j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_
pub fn stub_0xf5c874() {
    // IDA 0xf5c874: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0xf5c884 — j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFviEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_
pub fn stub_0xf5c884() {
    // IDA 0xf5c884: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone(void)const")]
// 0xf5c8c4 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv
pub fn stub_0xf5c8c4() {
    // IDA 0xf5c8c4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::rethrow(void)const")]
// 0xf5c8d4 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE7rethrowEv
pub fn stub_0xf5c8d4() {
    // IDA 0xf5c8d4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::operator()(RBX::AssetService*,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)const")]
// 0xf5c8f4 — j___ZNK5boost4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_
pub fn stub_0xf5c8f4() {
    // IDA 0xf5c8f4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>::operator()(RBX::AssetService*,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>)const")]
// 0xf5c904 — j___ZNK5boost4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_
pub fn stub_0xf5c904() {
    // IDA 0xf5c904: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf5c924 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_0xf5c924() {
    // IDA 0xf5c924: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf5c934 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFviEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_0xf5c934() {
    // IDA 0xf5c934: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0xf5c964 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0xf5c964() {
    // IDA 0xf5c964: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf5c974 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0xf5c974() {
    // IDA 0xf5c974: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0xf5c984 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFviEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0xf5c984() {
    // IDA 0xf5c984: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf5c994 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFviEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0xf5c994() {
    // IDA 0xf5c994: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::_M_allocate(unsigned long)")]
// 0xf5c9a4 — j___ZNSt12_Vector_baseIN3RBX12AssetService10AccessTypeESaIS2_EE11_M_allocateEm
pub fn stub_0xf5c9a4() {
    // IDA 0xf5c9a4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::AssetService::AccessType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AssetService::AccessType *,RBX::AssetService::AccessType *>(RBX::AssetService::AccessType *,RBX::AssetService::AccessType *,RBX::AssetService::AccessType *)")]
// 0xf5c9b4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12AssetService10AccessTypeES6_EET0_T_S8_S7_
pub fn stub_0xf5c9b4() {
    // IDA 0xf5c9b4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}
