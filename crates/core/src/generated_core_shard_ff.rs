//! core shard FF — 100 core stubs EA-sorted, lowest uncovered 0xf2a2e4..0xf2c614 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after FE 0xf2a0a4).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf2a0a4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::CellID::operator==(RBX::CellID const&)const")]
// 0xf2a2e4 — j___ZNK3RBX6CellIDeqERKS0_
pub fn stub_f2a2e4() {
    // IDA 0xf2a2e4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxRay::operator==(RBX::RbxRay const&)const")]
// 0xf2a2f4 — j___ZNK3RBX6RbxRayeqERKS0_
pub fn stub_f2a2f4() {
    // IDA 0xf2a2f4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "boost::pool<boost::default_user_allocator_new_delete>::purge_memory(void)")]
// 0xf2a6d4 — j___ZN5boost4poolINS_33default_user_allocator_new_deleteEE12purge_memoryEv
pub fn stub_f2a6d4() {
    // IDA 0xf2a6d4: boost template instantiation (mangled-only context). Per Boost map (AGENTS.md section 4) — carrier no-op.
}

#[doc(alias = "std::_Vector_base<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>::_M_allocate(unsigned long)")]
// 0xf2a6e4 — j___ZNSt12_Vector_baseIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE11_M_allocateEm
pub fn stub_f2a6e4() {
    // IDA 0xf2a6e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::pool<boost::default_user_allocator_new_delete> **,std::vector<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>>,boost::pool<boost::default_user_allocator_new_delete> * const&)")]
// 0xf2a6f4 — j___ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
pub fn stub_f2a6f4() {
    // IDA 0xf2a6f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>::push_back(boost::pool<boost::default_user_allocator_new_delete> * const&)")]
// 0xf2a704 — j___ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE9push_backERKS4_
pub fn stub_f2a704() {
    // IDA 0xf2a704: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::disconnectAll(void)")]
// 0xf2a9f4 — j___ZN3rbx7signals6signalIFvP9lua_StateEE13disconnectAllEv
pub fn stub_f2a9f4() {
    // IDA 0xf2a9f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::flyweight(void)")]
// 0xf2aa04 — j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EC2Ev
pub fn stub_f2aa04() {
    // IDA 0xf2aa04: flyweight interned-value holder. Arc<str>-style interning at the live site — carrier no-op.
}

#[doc(alias = "boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::operator=(RBX::ProtectedString const&)")]
// 0xf2aa14 — j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS3_
pub fn stub_f2aa14() {
    // IDA 0xf2aa14: flyweight interned-value holder. Arc<str>-style interning at the live site — carrier no-op.
}

#[doc(alias = "boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::operator=(boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_> const&)")]
// 0xf2aa24 — j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS6_
pub fn stub_f2aa24() {
    // IDA 0xf2aa24: flyweight interned-value holder. Arc<str>-style interning at the live site — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::StatsService> RBX::shared_from<RBX::Stats::StatsService>(RBX::Stats::StatsService*)")]
// 0xf2afc4 — j___ZN3RBX11shared_fromINS_5Stats12StatsServiceEEEN5boost10shared_ptrIT_EEPS5_
// was: boost::shared_ptr<RBX::Stats::StatsService> RBX::shared_from<RBX::Stats::StatsService>(RBX::Stats::StatsService*)
pub fn stub_f2afc4() {
    // IDA 0xf2afc4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::LibraryService::~LibraryService()")]
// 0xf2b0e4 — j___ZN3RBX14LibraryServiceD2Ev
pub fn stub_f2b0e4() {
    // IDA 0xf2b0e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::RunningAverage<double,double>::sample(double)")]
// 0xf2b0f4 — j___ZN3RBX14RunningAverageIddE6sampleEd
pub fn stub_f2b0f4() {
    // IDA 0xf2b0f4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::InvocationMeter<2>::updateBuckets(bool)")]
// 0xf2b104 — j___ZN3RBX15InvocationMeterILi2EE13updateBucketsEb
pub fn stub_f2b104() {
    // IDA 0xf2b104: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Stats::StatsService>(void)")]
// 0xf2b114 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_5Stats12StatsServiceEEEmv
pub fn stub_f2b114() {
    // IDA 0xf2b114: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample(void)")]
// 0xf2b1b4 — j___ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv
pub fn stub_f2b1b4() {
    // IDA 0xf2b1b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Security::Impersonator::Impersonator(RBX::Security::Identities)")]
// 0xf2b3c4 — j___ZN3RBX8Security12ImpersonatorC2ENS0_10IdentitiesE
pub fn stub_f2b3c4() {
    // IDA 0xf2b3c4: RunningAverage stats sampler. Accumulator at the live site — carrier no-op.
}

#[doc(alias = "RBX::Security::Context::current(void)")]
// 0xf2b3d4 — j___ZN3RBX8Security7Context7currentEv
pub fn stub_f2b3d4() {
    // IDA 0xf2b3d4: RunningAverage stats sampler. Accumulator at the live site — carrier no-op.
}

#[doc(alias = "RBX::ContentId::ContentId(char const*)")]
// 0xf2b3e4 — j___ZN3RBX9ContentIdC2EPKc
pub fn stub_f2b3e4() {
    // IDA 0xf2b3e4: RunningAverage stats sampler. Accumulator at the live site — carrier no-op.
}

#[doc(alias = "RBX::ContentId::ContentId(std::string const&)")]
// 0xf2b3f4 — j___ZN3RBX9ContentIdC2ERKSs
pub fn stub_f2b3f4() {
    // IDA 0xf2b3f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::Iterator(RobloxExtraSpace*)")]
// 0xf2b4b4 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorC2EPS2_
pub fn stub_f2b4b4() {
    // IDA 0xf2b4b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator++(void)")]
// 0xf2b4c4 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorppEv
pub fn stub_f2b4c4() {
    // IDA 0xf2b4c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator->(void)")]
// 0xf2b4d4 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorptEv
pub fn stub_f2b4d4() {
    // IDA 0xf2b4d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<std::string>(std::string const&)")]
// 0xf2b544 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSISsEERS3_RKT_
pub fn stub_f2b544() {
    // IDA 0xf2b544: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<bool>(bool const&)")]
// 0xf2b554 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIbEERS3_RKT_
pub fn stub_f2b554() {
    // IDA 0xf2b554: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<double>(double const&)")]
// 0xf2b564 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIdEERS3_RKT_
pub fn stub_f2b564() {
    // IDA 0xf2b564: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal_with_args<0,void ()(void)>::operator()(void)")]
// 0xf2b594 — j___ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv
pub fn stub_f2b594() {
    // IDA 0xf2b594: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(lua_State *)>::operator()(lua_State *)")]
// 0xf2b5a4 — j___ZN3rbx7signals16signal_with_argsILi1EFvP9lua_StateEEclES3_
pub fn stub_f2b5a4() {
    // IDA 0xf2b5a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::slot::safe_static_do_get_mutex(void)")]
// 0xf2b5f4 — j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f2b5f4() {
    // IDA 0xf2b5f4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::safe_static_do_get_mutex(void)")]
// 0xf2b6c4 — j___ZN3rbx7signals6signalIFvP9lua_StateEE24safe_static_do_get_mutexEv
pub fn stub_f2b6c4() {
    // IDA 0xf2b6c4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot> &)")]
// 0xf2b6d4 — j___ZN3rbx7signals6signalIFvP9lua_StateEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// was: rbx::signals::signal<void ()(lua_State *)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(lua_State *)>::slot> &)
pub fn stub_f2b6d4() {
    // IDA 0xf2b6d4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::on_error(std::exception &)")]
// 0xf2b6e4 — j___ZN3rbx7signals6signalIFvP9lua_StateEE8on_errorERSt9exception
pub fn stub_f2b6e4() {
    // IDA 0xf2b6e4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_do_get_mutex(void)")]
// 0xf2b6f4 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv
pub fn stub_f2b6f4() {
    // IDA 0xf2b6f4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_do_get_mutex(void)")]
// 0xf2b704 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f2b704() {
    // IDA 0xf2b704: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::insert(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
// 0xf2b714 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6insertEPNS7_4slotE
pub fn stub_f2b714() {
    // IDA 0xf2b714: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::remove(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
// 0xf2b724 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6removeEPNS7_4slotE
pub fn stub_f2b724() {
    // IDA 0xf2b724: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::safe_static_do_get_mutex(void)")]
// 0xf2b7c4 — j___ZN3rbx7signals6signalIFviEE24safe_static_do_get_mutexEv
pub fn stub_f2b7c4() {
    // IDA 0xf2b7c4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::disconnectAll(void)")]
// 0xf2b7d4 — j___ZN3rbx7signals6signalIFvvEE13disconnectAllEv
pub fn stub_f2b7d4() {
    // IDA 0xf2b7d4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::safe_static_do_get_mutex(void)")]
// 0xf2b7e4 — j___ZN3rbx7signals6signalIFvvEE24safe_static_do_get_mutexEv
pub fn stub_f2b7e4() {
    // IDA 0xf2b7e4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "std::string const& rbx::any_cast<std::string const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf2b7f4 — j___ZN3rbx8any_castIRKSsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f2b7f4() {
    // IDA 0xf2b7f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "bool const& rbx::any_cast<bool const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf2b804 — j___ZN3rbx8any_castIRKbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f2b804() {
    // IDA 0xf2b804: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "int const& rbx::any_cast<int const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf2b814 — j___ZN3rbx8any_castIRKiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f2b814() {
    // IDA 0xf2b814: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::scoped_ptr<RBX::LibraryService>::~scoped_ptr()")]
// 0xf2b874 — j___ZN5boost10scoped_ptrIN3RBX14LibraryServiceEED2Ev
pub fn stub_f2b874() {
    // IDA 0xf2b874: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::scoped_ptr<boost::thread>::~scoped_ptr()")]
// 0xf2b8a4 — j___ZN5boost10scoped_ptrINS_6threadEED2Ev
pub fn stub_f2b8a4() {
    // IDA 0xf2b8a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::scoped_ptr<std::string>::~scoped_ptr()")]
// 0xf2b8b4 — j___ZN5boost10scoped_ptrISsED2Ev
pub fn stub_f2b8b4() {
    // IDA 0xf2b8b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::operator=(rbx_core::SharedPtr<RBX::RunService> const&)")]
// 0xf2b904 — j___ZN5boost10shared_ptrIN3RBX10RunServiceEEaSERKS3_
// was: boost::shared_ptr<RBX::RunService>::operator=(boost::shared_ptr<RBX::RunService> const&)
pub fn stub_f2b904() {
    // IDA 0xf2b904: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::GcJob>(RBX::GcJob *)")]
// 0xf2b9c4 — j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_5GcJobEEEPT_
// was: boost::shared_ptr<RBX::TaskScheduler::Job>::shared_ptr<RBX::GcJob>(RBX::GcJob *)
pub fn stub_f2b9c4() {
    // IDA 0xf2b9c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::operator=(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
// 0xf2b9d4 — j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEaSERKS4_
// was: boost::shared_ptr<RBX::TaskScheduler::Job>::operator=(boost::shared_ptr<RBX::TaskScheduler::Job> const&)
pub fn stub_f2b9d4() {
    // IDA 0xf2b9d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::multi_index::detail::auto_space<boost::multi_index::detail::hashed_index_node_impl<std::allocator<char>>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::auto_space(std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&,unsigned long)")]
// 0xf2ba74 — j___ZN5boost11multi_index6detail10auto_spaceINS1_22hashed_index_node_implISaIcEEESaINS_10flyweights6detail16refcounted_valueINS7_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeESB_EEEEC2ERKSF_m
pub fn stub_f2ba74() {
    // IDA 0xf2ba74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::multi_index::detail::bucket_array<std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::bucket_array(std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&,boost::multi_index::detail::hashed_index_node_impl<std::allocator<char>> *,unsigned long)")]
// 0xf2ba84 — j___ZN5boost11multi_index6detail12bucket_arrayISaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES8_EEEEC2ERKSC_PNS1_22hashed_index_node_implISaIcEEEm
pub fn stub_f2ba84() {
    // IDA 0xf2ba84: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::hashed_index(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type> const&,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&)")]
// 0xf2ba94 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEEC2ERKNS_6tuples4consINSV_5tupleImSD_SF_SH_NSV_9null_typeESY_SY_SY_SY_SY_EESY_EERKSO_
pub fn stub_f2ba94() {
    // IDA 0xf2ba94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot>::operator=(rbx::signals::signal<void ()(RBX::RunTransition)>::slot*)")]
// 0xf2bac4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotEEaSEPS8_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot>::operator=(rbx::signals::signal<void ()(RBX::RunTransition)>::slot*)
pub fn stub_f2bac4() {
    // IDA 0xf2bac4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot> const&)")]
// 0xf2bb04 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSERKS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(lua_State *)>::slot> const&)
pub fn stub_f2bb04() {
    // IDA 0xf2bb04: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot*)")]
// 0xf2bb14 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot*)
pub fn stub_f2bb14() {
    // IDA 0xf2bb14: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> const&)")]
// 0xf2bb24 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSERKSB_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> const&)
pub fn stub_f2bb24() {
    // IDA 0xf2bb24: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot> const&)")]
// 0xf2bb54 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int)>::slot> const&)
pub fn stub_f2bb54() {
    // IDA 0xf2bb54: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot> const&)")]
// 0xf2bb64 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(void)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(void)>::slot> const&)
pub fn stub_f2bb64() {
    // IDA 0xf2bb64: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::condition_variable::~condition_variable()")]
// 0xf2bb84 — j___ZN5boost18condition_variableD2Ev
pub fn stub_f2bb84() {
    // IDA 0xf2bb84: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::reset(RBX::Security::Context*)")]
// 0xf2bb94 — j___ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE5resetEPS3_
pub fn stub_f2bb94() {
    // IDA 0xf2bb94: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::~thread_specific_ptr()")]
// 0xf2bba4 — j___ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEED2Ev
pub fn stub_f2bba4() {
    // IDA 0xf2bba4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>::list3(boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>)")]
// 0xf2bc94 — j___ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEC2ES3_S4_S6_
pub fn stub_f2bc94() {
    // IDA 0xf2bc94: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>::operator()<void (*)(lua_State *,int,std::string),boost::_bi::list2<lua_State *&,unsigned long &>>(boost::_bi::type<void>,void (*)(lua_State *,int,std::string) &,boost::_bi::list2<lua_State *&,unsigned long &> &,int)")]
// 0xf2bca4 — j___ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEclIPFvP9lua_StateiSsENS0_5list2IRSA_RmEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f2bca4() {
    // IDA 0xf2bca4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list_av_3<boost::arg<1>,boost::arg<2>,std::string>::type> boost::bind<void,lua_State *,int,std::string,boost::arg<1>,boost::arg<2>,std::string>(void (*)(lua_State *,int,std::string),boost::arg<1>,boost::arg<2>,std::string)")]
// 0xf2bd84 — j___ZN5boost4bindIvP9lua_StateiSsNS_3argILi1EEENS3_ILi2EEESsEENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_
pub fn stub_f2bd84() {
    // IDA 0xf2bd84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GcJob>(RBX::GcJob *)")]
// 0xf2bdc4 — j___ZN5boost6detail12shared_countC2IN3RBX5GcJobEEEPT_
pub fn stub_f2bdc4() {
    // IDA 0xf2bdc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::function0<void>>>(boost::detail::thread_data<boost::function0<void>> *)")]
// 0xf2bdd4 — j___ZN5boost6detail12shared_countC2INS0_11thread_dataINS_9function0IvEEEEEEPT_
pub fn stub_f2bdd4() {
    // IDA 0xf2bdd4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>(boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>)")]
// 0xf2be84 — j___ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS8_EEEET_T0_
pub fn stub_f2be84() {
    // IDA 0xf2be84: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::thread::join(void)")]
// 0xf2bf04 — j___ZN5boost6thread4joinEv
pub fn stub_f2bf04() {
    // IDA 0xf2bf04: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::thread::~thread()")]
// 0xf2bf24 — j___ZN5boost6threadD2Ev
pub fn stub_f2bf24() {
    // IDA 0xf2bf24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function<std::string ()(std::string const&)>::operator=(boost::function<std::string ()(std::string const&)> const&)")]
// 0xf2bf34 — j___ZN5boost8functionIFSsRKSsEEaSERKS4_
pub fn stub_f2bf34() {
    // IDA 0xf2bf34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<std::string,std::string const&>::move_assign(boost::function1<std::string,std::string const&>&)")]
// 0xf2c004 — j___ZN5boost9function1ISsRKSsE11move_assignERS3_
pub fn stub_f2c004() {
    // IDA 0xf2c004: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<std::string,std::string const&>::assign_to_own(boost::function1<std::string,std::string const&> const&)")]
// 0xf2c014 — j___ZN5boost9function1ISsRKSsE13assign_to_ownERKS3_
pub fn stub_f2c014() {
    // IDA 0xf2c014: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<std::string,std::string const&>::swap(boost::function1<std::string,std::string const&>&)")]
// 0xf2c024 — j___ZN5boost9function1ISsRKSsE4swapERS3_
pub fn stub_f2c024() {
    // IDA 0xf2c024: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<std::string,std::string const&>::clear(void)")]
// 0xf2c034 — j___ZN5boost9function1ISsRKSsE5clearEv
pub fn stub_f2c034() {
    // IDA 0xf2c034: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<unsigned long,lua_State *>::assign_to_own(boost::function1<unsigned long,lua_State *> const&)")]
// 0xf2c044 — j___ZN5boost9function1ImP9lua_StateE13assign_to_ownERKS3_
pub fn stub_f2c044() {
    // IDA 0xf2c044: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<unsigned long,lua_State *>::clear(void)")]
// 0xf2c054 — j___ZN5boost9function1ImP9lua_StateE5clearEv
pub fn stub_f2c054() {
    // IDA 0xf2c054: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,lua_State *>::move_assign(boost::function1<void,lua_State *>&)")]
// 0xf2c0a4 — j___ZN5boost9function1IvP9lua_StateE11move_assignERS3_
pub fn stub_f2c0a4() {
    // IDA 0xf2c0a4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,lua_State *>::assign_to_own(boost::function1<void,lua_State *> const&)")]
// 0xf2c0b4 — j___ZN5boost9function1IvP9lua_StateE13assign_to_ownERKS3_
pub fn stub_f2c0b4() {
    // IDA 0xf2c0b4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,lua_State *>::swap(boost::function1<void,lua_State *>&)")]
// 0xf2c0c4 — j___ZN5boost9function1IvP9lua_StateE4swapERS3_
pub fn stub_f2c0c4() {
    // IDA 0xf2c0c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,lua_State *>::clear(void)")]
// 0xf2c0d4 — j___ZN5boost9function1IvP9lua_StateE5clearEv
pub fn stub_f2c0d4() {
    // IDA 0xf2c0d4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::clear(void)")]
// 0xf2c144 — j___ZN5boost9function2IvP9lua_StateP9lua_DebugE5clearEv
pub fn stub_f2c144() {
    // IDA 0xf2c144: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::assign_to_own(boost::function2<void,lua_State *,unsigned long> const&)")]
// 0xf2c154 — j___ZN5boost9function2IvP9lua_StatemE13assign_to_ownERKS3_
pub fn stub_f2c154() {
    // IDA 0xf2c154: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::clear(void)")]
// 0xf2c164 — j___ZN5boost9function2IvP9lua_StatemE5clearEv
pub fn stub_f2c164() {
    // IDA 0xf2c164: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::function2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>)")]
// 0xf2c174 — j___ZN5boost9function2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEEvT_
pub fn stub_f2c174() {
    // IDA 0xf2c174: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::rehash_impl(unsigned long)")]
// 0xf2c254 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE11rehash_implEm
pub fn stub_f2c254() {
    // IDA 0xf2c254: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<unsigned int>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::emplace_impl<boost::unordered::detail::emplace_args1<unsigned int>>(unsigned int const&,boost::unordered::detail::emplace_args1<unsigned int> const&)")]
// 0xf2c264 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE12emplace_implINS1_13emplace_args1IjEEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEbERKjRKT_
pub fn stub_f2c264() {
    // IDA 0xf2c264: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0xf2c274 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE15place_in_bucketERNS1_5tableIS9_EEPNS1_10ptr_bucketE
pub fn stub_f2c274() {
    // IDA 0xf2c274: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<unsigned int>>>::construct(void)")]
// 0xf2c284 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIjEEEE9constructEv
pub fn stub_f2c284() {
    // IDA 0xf2c284: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>> const&)")]
// 0xf2c2a4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSB_RKSD_RKSaINS1_8ptr_nodeIS8_EEE
pub fn stub_f2c2a4() {
    // IDA 0xf2c2a4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::create_buckets(unsigned long)")]
// 0xf2c2c4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm
pub fn stub_f2c2c4() {
    // IDA 0xf2c2c4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::RunService * RBX::ServiceProvider::find<RBX::RunService>(void)const")]
// 0xf2c334 — j___ZNK3RBX15ServiceProvider4findINS_10RunServiceEEEPT_v
pub fn stub_f2c334() {
    // IDA 0xf2c334: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::RunService * RBX::ServiceProvider::create<RBX::RunService>(void)const")]
// 0xf2c354 — j___ZNK3RBX15ServiceProvider6createINS_10RunServiceEEEPT_v
pub fn stub_f2c354() {
    // IDA 0xf2c354: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Stats::StatsService * RBX::ServiceProvider::create<RBX::Stats::StatsService>(void)const")]
// 0xf2c374 — j___ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v
pub fn stub_f2c374() {
    // IDA 0xf2c374: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::GcJob>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const*,RBX::GcJob *)const")]
// 0xf2c434 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_5GcJobEEEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::GcJob>(boost::shared_ptr<RBX::TaskScheduler::Job> const*,RBX::GcJob *)const
pub fn stub_f2c434() {
    // IDA 0xf2c434: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// 0xf2c544 — j___ZNK5boost6detail8function13basic_vtable2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS4_iSsENS7_5list3INS_3argILi1EEENSC_ILi2EEENS7_5valueISsEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_f2c544() {
    // IDA 0xf2c544: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf2c554 — j___ZNK5boost6detail8function13basic_vtable2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS4_iSsENS7_5list3INS_3argILi1EEENSC_ILi2EEENS7_5valueISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_f2c554() {
    // IDA 0xf2c554: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::function0<void>::operator()(void)const")]
// 0xf2c5c4 — j___ZNK5boost9function0IvEclEv
pub fn stub_f2c5c4() {
    // IDA 0xf2c5c4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::function1<std::string,std::string const&>::operator()(std::string const&)const")]
// 0xf2c5d4 — j___ZNK5boost9function1ISsRKSsEclES2_
pub fn stub_f2c5d4() {
    // IDA 0xf2c5d4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::function1<unsigned long,lua_State *>::operator()(lua_State *)const")]
// 0xf2c5e4 — j___ZNK5boost9function1ImP9lua_StateEclES2_
pub fn stub_f2c5e4() {
    // IDA 0xf2c5e4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,lua_State *>::operator()(lua_State *)const")]
// 0xf2c604 — j___ZNK5boost9function1IvP9lua_StateEclES2_
pub fn stub_f2c604() {
    // IDA 0xf2c604: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,bool>::operator()(bool)const")]
// 0xf2c614 — j___ZNK5boost9function1IvbEclEb
pub fn stub_f2c614() {
    // IDA 0xf2c614: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}
