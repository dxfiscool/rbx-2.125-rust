//! core shard EV — 100 core stubs EA-sorted, lowest uncovered 0xbdb320..0xbeeab8 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EU 0xbdadb8).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xbdadb8.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::TextureCompositor::isQueueEmpty(void)const")]
// 0xbdb320 — __ZNK3RBX17TextureCompositor12isQueueEmptyEv
pub fn stub_bdb320() {
    // IDA 0xbdb320: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>> std::remove_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,anonymous namespace::ExistsInSetPredicate<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,anonymous namespace::ExistsInSetPredicate<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>)")]
// 0xbdb8d8 — __ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_120ExistsInSetPredicateIS7_EEET_SG_SG_T0_
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>> std::remove_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::ExistsInSetPredicate<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::ExistsInSetPredicate<boost::shared_ptr<RBX::TextureCompositor::Job>>)
pub fn stub_bdb8d8() {
    // IDA 0xbdb8d8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositor::garbageCollectOrphanedJobs(void)")]
// 0xbdbb84 — __ZN3RBX17TextureCompositor26garbageCollectOrphanedJobsEv
pub fn stub_bdbb84() {
    // IDA 0xbdbb84: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositor::findRebakeTargetAndEnqueue(void)")]
// 0xbdc080 — __ZN3RBX17TextureCompositor26findRebakeTargetAndEnqueueEv
pub fn stub_bdc080() {
    // IDA 0xbdc080: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::sort<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>)")]
// 0xbdc398 — __ZSt4sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_
// was: void std::sort<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)
pub fn stub_bdc398() {
    // IDA 0xbdc398: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositor::updateJob(RBX::TextureCompositor::Job &)")]
// 0xbdc888 — __ZN3RBX17TextureCompositor9updateJobERNS0_3JobE
pub fn stub_bdc888() {
    // IDA 0xbdc888: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositor::renderJobFinalize(RBX::TextureCompositor::Job &)")]
// 0xbdc9e8 — __ZN3RBX17TextureCompositor17renderJobFinalizeERNS0_3JobE
pub fn stub_bdc9e8() {
    // IDA 0xbdc9e8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositor::renderJobIfNecessary(RBX::TextureCompositor::Job &,unsigned long)")]
// 0xbdd154 — __ZN3RBX17TextureCompositor20renderJobIfNecessaryERNS0_3JobEm
pub fn stub_bdd154() {
    // IDA 0xbdd154: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositor::orphanTextureFromJob(RBX::TextureCompositor::Job &)")]
// 0xbde4fc — __ZN3RBX17TextureCompositor20orphanTextureFromJobERNS0_3JobE
pub fn stub_bde4fc() {
    // IDA 0xbde4fc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositor::getOrCreateTexture(unsigned int)")]
// 0xbde708 — __ZN3RBX17TextureCompositor18getOrCreateTextureEj
pub fn stub_bde708() {
    // IDA 0xbde708: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextureCompositor::getStatistics(void)const")]
// 0xbdf32c — __ZNK3RBX17TextureCompositor13getStatisticsEv
pub fn stub_bdf32c() {
    // IDA 0xbdf32c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>)")]
// 0xbdf498 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_SG_T0_
// was: void std::__heap_select<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)
pub fn stub_bdf498() {
    // IDA 0xbdf498: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>)")]
// 0xbdf798 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_
// was: void std::__insertion_sort<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)
pub fn stub_bdf798() {
    // IDA 0xbdf798: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,rbx_core::SharedPtr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,rbx_core::SharedPtr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>)")]
// 0xbdf9e4 — __ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEES7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_T1_
// was: void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)
pub fn stub_bdf9e4() {
    // IDA 0xbdf9e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,int,rbx_core::SharedPtr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,int,int,rbx_core::SharedPtr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>)")]
// 0xbdfae0 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiS7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_SH_T1_T2_
// was: void std::__adjust_heap<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,boost::shared_ptr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,int,boost::shared_ptr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)
pub fn stub_bdfae0() {
    // IDA 0xbdfae0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,int,anonymous namespace::PriorityComparator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,int,anonymous namespace::PriorityComparator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>)")]
// 0xbdfe68 — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_T1_
// was: void std::__introsort_loop<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)
pub fn stub_bdfe68() {
    // IDA 0xbdfe68: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::operator[](std::string const&)")]
// 0xbe04f0 — __ZNSt3mapISsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt4lessISsESaISt4pairIKSsS5_EEEixERS9_
// was: std::map<std::string,boost::shared_ptr<RBX::TextureCompositor::Job>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::operator[](std::string const&)
pub fn stub_be04f0() {
    // IDA 0xbe04f0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::operator=(std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&)")]
// 0xbe075c — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EEaSERKS3_
pub fn stub_be075c() {
    // IDA 0xbe075c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::push_back(rbx_core::SharedPtr<RBX::TextureCompositor::Job> const&)")]
// 0xbe09c8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE9push_backERKS5_
// was: std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::push_back(boost::shared_ptr<RBX::TextureCompositor::Job> const&)
pub fn stub_be09c8() {
    // IDA 0xbe09c8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::reserve(unsigned long)")]
// 0xbe0b20 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE7reserveEm
// was: std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::reserve(unsigned long)
pub fn stub_be0b20() {
    // IDA 0xbe0b20: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextureCompositor::Job>(RBX::TextureCompositor::Job *)")]
// 0xbe1144 — __ZN5boost6detail12shared_countC2IN3RBX17TextureCompositor3JobEEEPT_
pub fn stub_be1144() {
    // IDA 0xbe1144: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositor::Job::~Job()")]
// 0xbe1250 — __ZN3RBX17TextureCompositor3JobD2Ev
pub fn stub_be1250() {
    // IDA 0xbe1250: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::~vector()")]
// 0xbe1550 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EED2Ev
pub fn stub_be1550() {
    // IDA 0xbe1550: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::~sp_counted_impl_p()")]
// 0xbe15fc — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED1Ev
pub fn stub_be15fc() {
    // IDA 0xbe15fc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::~sp_counted_impl_p()")]
// 0xbe1600 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED0Ev
pub fn stub_be1600() {
    // IDA 0xbe1600: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::dispose(void)")]
// 0xbe1604 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE7disposeEv
pub fn stub_be1604() {
    // IDA 0xbe1604: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::get_deleter(std::type_info const&)")]
// 0xbe16a8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE11get_deleterERKSt9type_info
pub fn stub_be16a8() {
    // IDA 0xbe16a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::get_untyped_deleter(void)")]
// 0xbe16ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE19get_untyped_deleterEv
pub fn stub_be16ac() {
    // IDA 0xbe16ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextureCompositorJob>(RBX::TextureCompositorJob *)")]
// 0xbe16b0 — __ZN5boost6detail12shared_countC2IN3RBX20TextureCompositorJobEEEPT_
pub fn stub_be16b0() {
    // IDA 0xbe16b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::~sp_counted_impl_p()")]
// 0xbe1818 — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED1Ev
pub fn stub_be1818() {
    // IDA 0xbe1818: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::~sp_counted_impl_p()")]
// 0xbe181c — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED0Ev
pub fn stub_be181c() {
    // IDA 0xbe181c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::dispose(void)")]
// 0xbe1820 — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE7disposeEv
pub fn stub_be1820() {
    // IDA 0xbe1820: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::get_deleter(std::type_info const&)")]
// 0xbe192c — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE11get_deleterERKSt9type_info
pub fn stub_be192c() {
    // IDA 0xbe192c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::get_untyped_deleter(void)")]
// 0xbe1930 — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE19get_untyped_deleterEv
pub fn stub_be1930() {
    // IDA 0xbe1930: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_range_insert<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,std::forward_iterator_tag)")]
// 0xbe1934 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertIN9__gnu_cxx17__normal_iteratorIPS5_S7_EEEEvSC_T_SD_St20forward_iterator_tag
// was: void std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_range_insert<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,std::forward_iterator_tag)
pub fn stub_be1934() {
    // IDA 0xbe1934: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextureCompositor::Job> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,rbx_core::SharedPtr<RBX::TextureCompositor::Job> *>(rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,rbx_core::SharedPtr<RBX::TextureCompositor::Job> *)")]
// 0xbe21d4 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::TextureCompositor::Job> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *>(boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *)
pub fn stub_be21d4() {
    // IDA 0xbe21d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextureCompositor::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,rbx_core::SharedPtr<RBX::TextureCompositor::Job> *>(rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,rbx_core::SharedPtr<RBX::TextureCompositor::Job> *)")]
// 0xbe2288 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::TextureCompositor::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *>(boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *)
pub fn stub_be2288() {
    // IDA 0xbe2288: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextureCompositor::Job>* std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_allocate_and_copy<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*>(unsigned long,rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,rbx_core::SharedPtr<RBX::TextureCompositor::Job>*)")]
// 0xbe2340 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE20_M_allocate_and_copyIPS5_EES9_mT_SA_
// was: boost::shared_ptr<RBX::TextureCompositor::Job>* std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_allocate_and_copy<boost::shared_ptr<RBX::TextureCompositor::Job>*>(unsigned long,boost::shared_ptr<RBX::TextureCompositor::Job>*,boost::shared_ptr<RBX::TextureCompositor::Job>*)
pub fn stub_be2340() {
    // IDA 0xbe2340: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_range_insert<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::forward_iterator_tag)")]
// 0xbe2524 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertISt23_Rb_tree_const_iteratorIS5_EEEvN9__gnu_cxx17__normal_iteratorIPS5_S7_EET_SF_St20forward_iterator_tag
// was: void std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_range_insert<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::forward_iterator_tag)
pub fn stub_be2524() {
    // IDA 0xbe2524: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>> std::__copy_normal<false,true>::__copy_n<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>)")]
// 0xbe2dec — __ZNSt13__copy_normalILb0ELb1EE8__copy_nISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEN9__gnu_cxx17__normal_iteratorIPS8_St6vectorIS8_SaIS8_EEEEEET0_T_SI_SH_
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>> std::__copy_normal<false,true>::__copy_n<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>)
pub fn stub_be2dec() {
    // IDA 0xbe2dec: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>)")]
// 0xbe2e74 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE5eraseESt17_Rb_tree_iteratorIS8_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>)
pub fn stub_be2e74() {
    // IDA 0xbe2e74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::less<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::TextureCompositor::Job> const&)")]
// 0xbe2fa8 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// was: std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_insert_unique(boost::shared_ptr<RBX::TextureCompositor::Job> const&)
pub fn stub_be2fa8() {
    // IDA 0xbe2fa8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::less<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_create_node(rbx_core::SharedPtr<RBX::TextureCompositor::Job> const&)")]
// 0xbe305c — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// was: std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_create_node(boost::shared_ptr<RBX::TextureCompositor::Job> const&)
pub fn stub_be305c() {
    // IDA 0xbe305c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<unsigned long long,std::allocator<unsigned long long>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned long long *,std::vector<unsigned long long,std::allocator<unsigned long long>>>,unsigned long long const&)")]
// 0xbe317c — __ZNSt6vectorIySaIyEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPyS1_EERKy
pub fn stub_be317c() {
    // IDA 0xbe317c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,rbx_core::SharedPtr<RBX::TextureCompositor::Job> const&)")]
// 0xbe3278 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// was: std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job> const&)
pub fn stub_be3278() {
    // IDA 0xbe3278: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositorLayer* std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>)")]
// 0xbe380c — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS1_S3_EEEEPS1_mT_SB_
pub fn stub_be380c() {
    // IDA 0xbe380c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositorLayer * std::__uninitialized_copy_aux<RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *>(RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *,std::__false_type)")]
// 0xbe38f8 — __ZSt24__uninitialized_copy_auxIPN3RBX22TextureCompositorLayerES2_ET0_T_S4_S3_St12__false_type
pub fn stub_be38f8() {
    // IDA 0xbe38f8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositorLayer* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer*>(__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer*,std::__false_type)")]
// 0xbe3b44 — __ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX22TextureCompositorLayerESt6vectorIS3_SaIS3_EEEEPS3_ET0_T_SC_SB_St12__false_type
pub fn stub_be3b44() {
    // IDA 0xbe3b44: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>> const&)")]
// 0xbe3d90 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)
pub fn stub_be3d90() {
    // IDA 0xbe3d90: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>> const&)")]
// 0xbe40d8 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)
pub fn stub_be40d8() {
    // IDA 0xbe40d8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>> const&)")]
// 0xbe414c — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueERKS8_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)
pub fn stub_be414c() {
    // IDA 0xbe414c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>> const&)")]
// 0xbe4230 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE14_M_create_nodeERKS8_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)
pub fn stub_be4230() {
    // IDA 0xbe4230: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<RBX::TextureCompositorJob::LayerData,std::allocator<RBX::TextureCompositorJob::LayerData>>::vector(unsigned long,RBX::TextureCompositorJob::LayerData const&,std::allocator<RBX::TextureCompositorJob::LayerData> const&)")]
// 0xbe43b4 — __ZNSt6vectorIN3RBX20TextureCompositorJob9LayerDataESaIS2_EEC2EmRKS2_RKS3_
pub fn stub_be43b4() {
    // IDA 0xbe43b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositorJob::LayerData::LayerData(RBX::TextureCompositorJob::LayerData const&)")]
// 0xbe4530 — __ZN3RBX20TextureCompositorJob9LayerDataC2ERKS1_
pub fn stub_be4530() {
    // IDA 0xbe4530: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::less<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::TextureCompositor::Job>> *)")]
// 0xbe46e4 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// was: std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::TextureCompositor::Job>> *)
pub fn stub_be46e4() {
    // IDA 0xbe46e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>> *)")]
// 0xbe4714 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>> *)
pub fn stub_be4714() {
    // IDA 0xbe4714: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositorJob::LayerData::~LayerData()")]
// 0xbe4854 — __ZN3RBX20TextureCompositorJob9LayerDataD2Ev
pub fn stub_be4854() {
    // IDA 0xbe4854: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositorJob::LayerData::LayerData(void)")]
// 0xbe4b50 — __ZN3RBX20TextureCompositorJob9LayerDataC2Ev
pub fn stub_be4b50() {
    // IDA 0xbe4b50: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::RbxTextureProxy::~RbxTextureProxy()")]
// 0xbe57cc — __ZN3RBX15RbxTextureProxyD0Ev
pub fn stub_be57cc() {
    // IDA 0xbe57cc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::RbxTextureProxy::~RbxTextureProxy()")]
// 0xbe586c — __ZN3RBX15RbxTextureProxyD1Ev
pub fn stub_be586c() {
    // IDA 0xbe586c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::RbxTextureProxy::~RbxTextureProxy()")]
// 0xbe5870 — __ZN3RBX15RbxTextureProxyD2Ev
pub fn stub_be5870() {
    // IDA 0xbe5870: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::RbxTextureProxy::getOriginalSize(void)")]
// 0xbe5a84 — __ZN3RBX15RbxTextureProxy15getOriginalSizeEv
pub fn stub_be5a84() {
    // IDA 0xbe5a84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx_InitModule(void)")]
// 0xbe6d70 — __ZN3RBX21ViewRbxGfx_InitModuleEv
pub fn stub_be6d70() {
    // IDA 0xbe6d70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::ViewRbxGfx(RBX::CRenderSettings::GraphicsMode,RBX::OSContext *,RBX::CRenderSettings*)")]
// 0xbe6e54 — __ZN3RBX10ViewRbxGfxC2ENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_
pub fn stub_be6e54() {
    // IDA 0xbe6e54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::enableAdorns(bool)")]
// 0xbe75bc — __ZN3RBX10ViewRbxGfx12enableAdornsEb
pub fn stub_be75bc() {
    // IDA 0xbe75bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::ViewRbxGfx::enableAdorns(bool)")]
// 0xbe75c4 — __ZThn4_N3RBX10ViewRbxGfx12enableAdornsEb
// was: non-virtual thunk to RBX::ViewRbxGfx::enableAdorns(bool)
pub fn stub_be75c4() {
    // IDA 0xbe75c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::initResources(void)")]
// 0xbe75cc — __ZN3RBX10ViewRbxGfx13initResourcesEv
pub fn stub_be75cc() {
    // IDA 0xbe75cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::onTakeScreenshot(void)")]
// 0xbe83d8 — __ZN3RBX10ViewRbxGfx16onTakeScreenshotEv
pub fn stub_be83d8() {
    // IDA 0xbe83d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::reloadShaders(void)")]
// 0xbe83e0 — __ZN3RBX10ViewRbxGfx13reloadShadersEv
pub fn stub_be83e0() {
    // IDA 0xbe83e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::enableQueue(int)")]
// 0xbe85f4 — __ZN3RBX10ViewRbxGfx11enableQueueEi
pub fn stub_be85f4() {
    // IDA 0xbe85f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::disableQueue(int)")]
// 0xbe8674 — __ZN3RBX10ViewRbxGfx12disableQueueEi
pub fn stub_be8674() {
    // IDA 0xbe8674: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ViewRbxGfx::~ViewRbxGfx()")]
// 0xbe86f4 — __ZN3RBX10ViewRbxGfxD0Ev
pub fn stub_be86f4() {
    // IDA 0xbe86f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::~ViewRbxGfx()")]
// 0xbe8794 — __ZN3RBX10ViewRbxGfxD1Ev
pub fn stub_be8794() {
    // IDA 0xbe8794: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::ViewRbxGfx::~ViewRbxGfx()")]
// 0xbe8798 — __ZThn8_N3RBX10ViewRbxGfxD0Ev
// was: non-virtual thunk to RBX::ViewRbxGfx::~ViewRbxGfx()
pub fn stub_be8798() {
    // IDA 0xbe8798: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::~ViewRbxGfx()")]
// 0xbe883c — __ZN3RBX10ViewRbxGfxD2Ev
pub fn stub_be883c() {
    // IDA 0xbe883c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::ViewRbxGfx::~ViewRbxGfx()")]
// 0xbe8e20 — __ZThn8_N3RBX10ViewRbxGfxD1Ev
// was: non-virtual thunk to RBX::ViewRbxGfx::~ViewRbxGfx()
pub fn stub_be8e20() {
    // IDA 0xbe8e20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::onResize(int,int)")]
// 0xbe8e28 — __ZN3RBX10ViewRbxGfx8onResizeEii
pub fn stub_be8e28() {
    // IDA 0xbe8e28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::getFrameRateManager(void)")]
// 0xbe8f48 — __ZN3RBX10ViewRbxGfx19getFrameRateManagerEv
pub fn stub_be8f48() {
    // IDA 0xbe8f48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::suppressSkybox(void)")]
// 0xbe8f50 — __ZN3RBX10ViewRbxGfx14suppressSkyboxEv
pub fn stub_be8f50() {
    // IDA 0xbe8f50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::invalidateLighting(bool)")]
// 0xbe9628 — __ZN3RBX10ViewRbxGfx18invalidateLightingEb
pub fn stub_be9628() {
    // IDA 0xbe9628: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::getAndClearDoScreenshot(void)")]
// 0xbe9638 — __ZN3RBX10ViewRbxGfx23getAndClearDoScreenshotEv
pub fn stub_be9638() {
    // IDA 0xbe9638: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ViewRbxGfx::loadSkyBox(bool &)")]
// 0xbe9648 — __ZN3RBX10ViewRbxGfx10loadSkyBoxERb
pub fn stub_be9648() {
    // IDA 0xbe9648: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ViewRbxGfx::updateFog(void)")]
// 0xbeb168 — __ZN3RBX10ViewRbxGfx9updateFogEv
pub fn stub_beb168() {
    // IDA 0xbeb168: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ViewRbxGfx::updateLighting(void)")]
// 0xbeb548 — __ZN3RBX10ViewRbxGfx14updateLightingEv
pub fn stub_beb548() {
    // IDA 0xbeb548: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ViewRbxGfx::isPreRenderNeeded(void)")]
// 0xbeb9a0 — __ZN3RBX10ViewRbxGfx17isPreRenderNeededEv
pub fn stub_beb9a0() {
    // IDA 0xbeb9a0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ViewRbxGfx::preRender(void)")]
// 0xbeb9a4 — __ZN3RBX10ViewRbxGfx9preRenderEv
pub fn stub_beb9a4() {
    // IDA 0xbeb9a4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ViewRbxGfx::getMetricValue(std::string const&)")]
// 0xbeba18 — __ZN3RBX10ViewRbxGfx14getMetricValueERKSs
pub fn stub_beba18() {
    // IDA 0xbeba18: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ViewRbxGfx::getRenderStatsMetric(std::string const&)")]
// 0xbebb08 — __ZN3RBX10ViewRbxGfx20getRenderStatsMetricERKSs
pub fn stub_bebb08() {
    // IDA 0xbebb08: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ViewRbxGfx::captureMetrics(RBX::RenderMetrics &)")]
// 0xbec1a0 — __ZN3RBX10ViewRbxGfx14captureMetricsERNS_13RenderMetricsE
pub fn stub_bec1a0() {
    // IDA 0xbec1a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "non-virtual thunk to RBX::ViewRbxGfx::captureMetrics(RBX::RenderMetrics &)")]
// 0xbec240 — __ZThn4_N3RBX10ViewRbxGfx14captureMetricsERNS_13RenderMetricsE
// was: non-virtual thunk to RBX::ViewRbxGfx::captureMetrics(RBX::RenderMetrics &)
pub fn stub_bec240() {
    // IDA 0xbec240: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::printScene(void)")]
// 0xbec65c — __ZN3RBX10ViewRbxGfx10printSceneEv
pub fn stub_bec65c() {
    // IDA 0xbec65c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::ViewRbxGfx::printScene(void)")]
// 0xbec7fc — __ZThn4_N3RBX10ViewRbxGfx10printSceneEv
// was: non-virtual thunk to RBX::ViewRbxGfx::printScene(void)
pub fn stub_bec7fc() {
    // IDA 0xbec7fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)")]
// 0xbec808 — __ZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricE
pub fn stub_bec808() {
    // IDA 0xbec808: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::~ProxyMetric()")]
// 0xbed5b8 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD1Ev
pub fn stub_bed5b8() {
    // IDA 0xbed5b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::renderPerform(double)")]
// 0xbed5c0 — __ZN3RBX10ViewRbxGfx13renderPerformEd
pub fn stub_bed5c0() {
    // IDA 0xbed5c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::saveScreenshotToFile(std::string &)")]
// 0xbee4c0 — __ZN3RBX10ViewRbxGfx20saveScreenshotToFileERSs
pub fn stub_bee4c0() {
    // IDA 0xbee4c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::update(void)")]
// 0xbee96c — __ZN3RBX10ViewRbxGfx6updateEv
pub fn stub_bee96c() {
    // IDA 0xbee96c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::buildGui(bool)")]
// 0xbeea8c — __ZN3RBX10ViewRbxGfx8buildGuiEb
pub fn stub_beea8c() {
    // IDA 0xbeea8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::getRenderStats(void)")]
// 0xbeeaac — __ZN3RBX10ViewRbxGfx14getRenderStatsEv
pub fn stub_beeaac() {
    // IDA 0xbeeaac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ViewRbxGfx::renderThumb(void)")]
// 0xbeeab8 — __ZN3RBX10ViewRbxGfx11renderThumbEv
pub fn stub_beeab8() {
    // IDA 0xbeeab8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}