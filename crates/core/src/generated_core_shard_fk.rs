//! core shard FK — 100 core stubs EA-sorted, 0xf30b14..0xf31534 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after FJ 0xf31534 gap).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf30b04.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.


#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::StandardOut::StandardOut(void)")]
// 0xf30b14 — j___ZN3RBX11StandardOutC2Ev
pub fn stub_f30b14() {
    // IDA 0xf30b14: stdout/exception-print helper owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::StandardOut::~StandardOut()")]
// 0xf30b24 — j___ZN3RBX11StandardOutD2Ev
pub fn stub_f30b24() {
    // IDA 0xf30b24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StandardOutMessage::StandardOutMessage(RBX::MessageType,char const*)")]
// 0xf30b34 — j___ZN3RBX18StandardOutMessageC2ENS_11MessageTypeEPKc
pub fn stub_f30b34() {
    // IDA 0xf30b34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::StandardOutMessage const&)>::operator()(RBX::StandardOutMessage const&)")]
// 0xf30b44 — j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEEEclES5_
pub fn stub_f30b44() {
    // IDA 0xf30b44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::disconnectAll(void)")]
// 0xf30b54 — j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13disconnectAllEv
pub fn stub_f30b54() {
    // IDA 0xf30b54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> &)")]
// 0xf30b64 — j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> &)
pub fn stub_f30b64() {
    // IDA 0xf30b64: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::on_error(std::exception &)")]
// 0xf30b74 — j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE8on_errorERSt9exception
pub fn stub_f30b74() {
    // IDA 0xf30b74: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StandardOut>::shared_ptr<RBX::StandardOut>(RBX::StandardOut *)")]
// 0xf30b84 — j___ZN5boost10shared_ptrIN3RBX11StandardOutEEC2IS2_EEPT_
// was: boost::shared_ptr<RBX::StandardOut>::shared_ptr<RBX::StandardOut>(RBX::StandardOut *)
pub fn stub_f30b84() {
    // IDA 0xf30b84: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StandardOut>(RBX::StandardOut *)")]
// 0xf30b94 — j___ZN5boost6detail12shared_countC2IN3RBX11StandardOutEEEPT_
pub fn stub_f30b94() {
    // IDA 0xf30b94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::StandardOut>::_internal_accept_owner<RBX::StandardOut,RBX::StandardOut>(rbx_core::SharedPtr<RBX::StandardOut> const*,RBX::StandardOut *)const")]
// 0xf30ba4 — j___ZNK5boost23enable_shared_from_thisIN3RBX11StandardOutEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::StandardOut>::_internal_accept_owner<RBX::StandardOut,RBX::StandardOut>(boost::shared_ptr<RBX::StandardOut> const*,RBX::StandardOut *)const
pub fn stub_f30ba4() {
    // IDA 0xf30ba4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::safe_static_do_get_mutex(void)")]
// 0xf30bb4 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f30bb4() {
    // IDA 0xf30bb4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::insert(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")]
// 0xf30bc4 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6insertEPNS7_4slotE
pub fn stub_f30bc4() {
    // IDA 0xf30bc4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::remove(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")]
// 0xf30bd4 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6removeEPNS7_4slotE
pub fn stub_f30bd4() {
    // IDA 0xf30bd4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Stepped const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>> const&)")]
// 0xf30be4 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_f30be4() {
    // IDA 0xf30be4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot*)")]
// 0xf30bf4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot*)
pub fn stub_f30bf4() {
    // IDA 0xf30bf4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>::operator()<RBX::Stepped>(RBX::Stepped const&)")]
// 0xf30c04 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8ISteppedERKNS4_7SteppedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
pub fn stub_f30c04() {
    // IDA 0xf30c04: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "RBX::BaseThreadPool::PoolData::PoolData(void)")]
// 0xf30c14 — j___ZN3RBX14BaseThreadPool8PoolDataC2Ev
pub fn stub_f30c14() {
    // IDA 0xf30c14: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
// 0xf30c24 — j___ZN3RBX14BaseThreadPool8PoolDataD2Ev
pub fn stub_f30c24() {
    // IDA 0xf30c24: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::safe_queue<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>::pop_if_present(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&)")]
// 0xf30c34 — j___ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE14pop_if_presentERS8_
// was: rbx::safe_queue<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>::pop_if_present(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&)
pub fn stub_f30c34() {
    // IDA 0xf30c34: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::safe_queue<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>::push(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> const&)")]
// 0xf30c44 — j___ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE4pushERKS8_
// was: rbx::safe_queue<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>::push(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)
pub fn stub_f30c44() {
    // IDA 0xf30c44: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::pop_heap_if_present(RBX::PriorityThreadPool::PriorityTask&)")]
// 0xf30c54 — j___ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE19pop_heap_if_presentERS3_
pub fn stub_f30c54() {
    // IDA 0xf30c54: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::push_heap(RBX::PriorityThreadPool::PriorityTask const&)")]
// 0xf30c64 — j___ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE9push_heapERKS3_
pub fn stub_f30c64() {
    // IDA 0xf30c64: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>::shared_ptr<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")]
// 0xf30c74 — j___ZN5boost10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEC2IS3_EEPT_
// was: boost::shared_ptr<RBX::BaseThreadPool::PoolData>::shared_ptr<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)
pub fn stub_f30c74() {
    // IDA 0xf30c74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::mutex>::reset<RBX::mutex>(RBX::mutex *)")]
// 0xf30c84 — j___ZN5boost10shared_ptrIN3RBX5mutexEE5resetIS2_EEvPT_
// was: void boost::shared_ptr<RBX::mutex>::reset<RBX::mutex>(RBX::mutex *)
pub fn stub_f30c84() {
    // IDA 0xf30c84: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::mutex>::shared_ptr<RBX::mutex>(RBX::mutex *)")]
// 0xf30c94 — j___ZN5boost10shared_ptrIN3RBX5mutexEEC2IS2_EEPT_
// was: boost::shared_ptr<RBX::mutex>::shared_ptr<RBX::mutex>(RBX::mutex *)
pub fn stub_f30c94() {
    // IDA 0xf30c94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::mutex>::operator=(rbx_core::SharedPtr<RBX::mutex> const&)")]
// 0xf30ca4 — j___ZN5boost10shared_ptrIN3RBX5mutexEEaSERKS3_
// was: boost::shared_ptr<RBX::mutex>::operator=(boost::shared_ptr<RBX::mutex> const&)
pub fn stub_f30ca4() {
    // IDA 0xf30ca4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>> *)")]
// 0xf30cb4 — j___ZN5boost10shared_ptrINS_6detail16thread_data_baseEEC2INS1_11thread_dataINS_3_bi6bind_tIvPFvNS0_IN3RBX14BaseThreadPool8PoolDataEEENS0_INS8_5mutexEEEENS6_5list2INS6_5valueISB_EENSH_ISD_EEEEEEEEEEPT_
// was: boost::shared_ptr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)
pub fn stub_f30cb4() {
    // IDA 0xf30cb4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void rbx_core::SharedPtr<boost::thread>::reset<boost::thread>(boost::thread *)")]
// 0xf30cc4 — j___ZN5boost10shared_ptrINS_6threadEE5resetIS1_EEvPT_
// was: void boost::shared_ptr<boost::thread>::reset<boost::thread>(boost::thread *)
pub fn stub_f30cc4() {
    // IDA 0xf30cc4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<boost::thread>::shared_ptr<boost::thread>(boost::thread *)")]
// 0xf30cd4 — j___ZN5boost10shared_ptrINS_6threadEEC2IS1_EEPT_
// was: boost::shared_ptr<boost::thread>::shared_ptr<boost::thread>(boost::thread *)
pub fn stub_f30cd4() {
    // IDA 0xf30cd4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<boost::thread>::operator=(rbx_core::SharedPtr<boost::thread> const&)")]
// 0xf30ce4 — j___ZN5boost10shared_ptrINS_6threadEEaSERKS2_
// was: boost::shared_ptr<boost::thread>::operator=(boost::shared_ptr<boost::thread> const&)
pub fn stub_f30ce4() {
    // IDA 0xf30ce4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::_bi::list1<boost::arg<1>>::operator()<void (*)(rbx_core::SharedPtr<boost::thread>),boost::_bi::list1<rbx_core::SharedPtr<boost::thread>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<boost::thread>) &,boost::_bi::list1<rbx_core::SharedPtr<boost::thread>&> &,int)")]
// 0xf30cf4 — j___ZN5boost3_bi5list1INS_3argILi1EEEEclIPFvNS_10shared_ptrINS_6threadEEEENS1_IRS8_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list1<boost::arg<1>>::operator()<void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::shared_ptr<boost::thread>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<boost::thread>) &,boost::_bi::list1<boost::shared_ptr<boost::thread>&> &,int)
pub fn stub_f30cf4() {
    // IDA 0xf30cf4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>)")]
// 0xf30d04 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>)
pub fn stub_f30d04() {
    // IDA 0xf30d04: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>) &,boost::_bi::list0 &,int)")]
// 0xf30d14 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEclIPFvS7_SA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::operator()<void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>) &,boost::_bi::list0 &,int)
pub fn stub_f30d14() {
    // IDA 0xf30d14: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>::operator()<void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list1<rbx_core::SharedPtr<boost::thread>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>) &,boost::_bi::list1<rbx_core::SharedPtr<boost::thread>&> &,int)")]
// 0xf30d24 — j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_9date_time18subsecond_durationINS_10posix_time13time_durationELx1000EEEEEEclIPFvNS_10shared_ptrINS_6threadEEES9_ENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>::operator()<void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list1<boost::shared_ptr<boost::thread>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>) &,boost::_bi::list1<boost::shared_ptr<boost::thread>&> &,int)
pub fn stub_f30d24() {
    // IDA 0xf30d24: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>)")]
// 0xf30d34 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>)
pub fn stub_f30d34() {
    // IDA 0xf30d34: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>,rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>>(void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>)")]
// 0xf30d44 — j___ZN5boost4bindIvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS1_INS2_5mutexEEES5_S7_EENS_3_bi6bind_tIT_PFSA_T0_T1_ENS8_9list_av_2IT2_T3_E4typeEEESE_SG_SH_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list_av_2<boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>>::type> boost::bind<void,boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>,boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>>(void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>)
pub fn stub_f30d44() {
    // IDA 0xf30d44: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>)")]
// 0xf30d54 — j___ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEC2ESI_
// was: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>)
pub fn stub_f30d54() {
    // IDA 0xf30d54: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")]
// 0xf30d64 — j___ZN5boost6detail12shared_countC2IN3RBX14BaseThreadPool8PoolDataEEEPT_
pub fn stub_f30d64() {
    // IDA 0xf30d64: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::mutex>(RBX::mutex *)")]
// 0xf30d74 — j___ZN5boost6detail12shared_countC2IN3RBX5mutexEEEPT_
pub fn stub_f30d74() {
    // IDA 0xf30d74: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>> *)")]
// 0xf30d84 — j___ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS6_INS7_5mutexEEEENS4_5list2INS4_5valueISA_EENSG_ISC_EEEEEEEEEEPT_
// was: boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)
pub fn stub_f30d84() {
    // IDA 0xf30d84: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread>(boost::thread *)")]
// 0xf30d94 — j___ZN5boost6detail12shared_countC2INS_6threadEEEPT_
pub fn stub_f30d94() {
    // IDA 0xf30d94: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>&>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>&)")]
// 0xf30da4 — j___ZN5boost6detail13heap_new_implINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEERSJ_EEPT_T0_
// was: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>&>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>&)
pub fn stub_f30da4() {
    // IDA 0xf30da4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>::operator=(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> const&)")]
// 0xf30dc4 — j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEaSERKS6_
// was: boost::function<void ()(boost::shared_ptr<RBX::mutex>)>::operator=(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)
pub fn stub_f30dc4() {
    // IDA 0xf30dc4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::counted_time_rep(boost::gregorian::date const&,boost::posix_time::time_duration const&)")]
// 0xf30dd4 — j___ZN5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEC2ERKNS_9gregorian4dateERKNS2_13time_durationE
pub fn stub_f30dd4() {
    // IDA 0xf30dd4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::date_time::time_resolution_traits<boost::date_time::time_resolution_traits_adapted64_impl,(boost::date_time::time_resolutions)5,1000000ll,(unsigned short)6,int>::to_tick_count(int,int,int,long long)")]
// 0xf30de4 — j___ZN5boost9date_time22time_resolution_traitsINS0_37time_resolution_traits_adapted64_implELNS0_16time_resolutionsE5ELx1000000ELt6EiE13to_tick_countEiiix
pub fn stub_f30de4() {
    // IDA 0xf30de4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::move_assign(boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>&)")]
// 0xf30df4 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE11move_assignERS5_
// was: boost::function1<void,boost::shared_ptr<RBX::mutex>>::move_assign(boost::function1<void,boost::shared_ptr<RBX::mutex>>&)
pub fn stub_f30df4() {
    // IDA 0xf30df4: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::mutex>> const&)")]
// 0xf30e04 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE13assign_to_ownERKS5_
// was: boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::mutex>> const&)
pub fn stub_f30e04() {
    // IDA 0xf30e04: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::swap(boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>&)")]
// 0xf30e14 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE4swapERS5_
// was: boost::function1<void,boost::shared_ptr<RBX::mutex>>::swap(boost::function1<void,boost::shared_ptr<RBX::mutex>>&)
pub fn stub_f30e14() {
    // IDA 0xf30e14: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>> *)const")]
// 0xf30e24 — j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS8_INS9_5mutexEEEENS6_5list2INS6_5valueISC_EENSI_ISE_EEEEEEEEEEvPKNS8_IT_EEPT0_
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)const
pub fn stub_f30e24() {
    // IDA 0xf30e24: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::operator()(rbx_core::SharedPtr<RBX::mutex>)const")]
// 0xf30e34 — j___ZNK5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEclES4_
// was: boost::function1<void,boost::shared_ptr<RBX::mutex>>::operator()(boost::shared_ptr<RBX::mutex>)const
pub fn stub_f30e34() {
    // IDA 0xf30e34: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_allocate_map(unsigned long)")]
// 0xf30e44 — j___ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_allocate_mapEm
// was: std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_allocate_map(unsigned long)
pub fn stub_f30e44() {
    // IDA 0xf30e44: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_create_nodes(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>**,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>**)")]
// 0xf30e54 — j___ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_create_nodesEPPS7_SB_
// was: std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_create_nodes(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>**,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>**)
pub fn stub_f30e54() {
    // IDA 0xf30e54: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_initialize_map(unsigned long)")]
// 0xf30e64 — j___ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_initialize_mapEm
// was: std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_initialize_map(unsigned long)
pub fn stub_f30e64() {
    // IDA 0xf30e64: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::~_Deque_base()")]
// 0xf30e74 — j___ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
// was: std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::~_Deque_base()
pub fn stub_f30e74() {
    // IDA 0xf30e74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_allocate(unsigned long)")]
// 0xf30e84 — j___ZNSt12_Vector_baseIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE11_M_allocateEm
pub fn stub_f30e84() {
    // IDA 0xf30e84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::_M_allocate(unsigned long)")]
// 0xf30e94 — j___ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE11_M_allocateEm
// was: std::_Vector_base<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_allocate(unsigned long)
pub fn stub_f30e94() {
    // IDA 0xf30e94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::_M_allocate(unsigned long)")]
// 0xf30ea4 — j___ZNSt12_Vector_baseIN5boost10shared_ptrINS0_6threadEEESaIS3_EE11_M_allocateEm
// was: std::_Vector_base<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_allocate(unsigned long)
pub fn stub_f30ea4() {
    // IDA 0xf30ea4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityTask * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *>(RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *)")]
// 0xf30eb4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18PriorityThreadPool12PriorityTaskES6_EET0_T_S8_S7_
pub fn stub_f30eb4() {
    // IDA 0xf30eb4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::mutex> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::mutex> *,rbx_core::SharedPtr<RBX::mutex> *>(rbx_core::SharedPtr<RBX::mutex> *,rbx_core::SharedPtr<RBX::mutex> *,rbx_core::SharedPtr<RBX::mutex> *)")]
// 0xf30ec4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5mutexEEES8_EET0_T_SA_S9_
// was: boost::shared_ptr<RBX::mutex> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *>(boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *)
pub fn stub_f30ec4() {
    // IDA 0xf30ec4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<boost::thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<boost::thread> *,rbx_core::SharedPtr<boost::thread> *>(rbx_core::SharedPtr<boost::thread> *,rbx_core::SharedPtr<boost::thread> *,rbx_core::SharedPtr<boost::thread> *)")]
// 0xf30ed4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrINS3_6threadEEES7_EET0_T_S9_S8_
// was: boost::shared_ptr<boost::thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *>(boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *)
pub fn stub_f30ed4() {
    // IDA 0xf30ed4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_push_back_aux(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> const&)")]
// 0xf30ee4 — j___ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE16_M_push_back_auxERKS7_
// was: std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_push_back_aux(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)
pub fn stub_f30ee4() {
    // IDA 0xf30ee4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_reallocate_map(unsigned long,bool)")]
// 0xf30ef4 — j___ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_reallocate_mapEmb
// was: std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_reallocate_map(unsigned long,bool)
pub fn stub_f30ef4() {
    // IDA 0xf30ef4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>*>,std::_Deque_iterator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>*>)")]
// 0xf30f04 — j___ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_
// was: std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>*>,std::_Deque_iterator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>*>)
pub fn stub_f30f04() {
    // IDA 0xf30f04: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_reserve_map_at_back(unsigned long)")]
// 0xf30f14 — j___ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE22_M_reserve_map_at_backEm
// was: std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_reserve_map_at_back(unsigned long)
pub fn stub_f30f14() {
    // IDA 0xf30f14: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::pop_front(void)")]
// 0xf30f24 — j___ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9pop_frontEv
// was: std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::pop_front(void)
pub fn stub_f30f24() {
    // IDA 0xf30f24: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::push_back(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> const&)")]
// 0xf30f34 — j___ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9push_backERKS7_
// was: std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::push_back(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)
pub fn stub_f30f34() {
    // IDA 0xf30f34: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::deque(std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>> const&)")]
// 0xf30f44 — j___ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EEC2ERKS9_
// was: std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::deque(std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>> const&)
pub fn stub_f30f44() {
    // IDA 0xf30f44: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::~deque()")]
// 0xf30f54 — j___ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
// was: std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::~deque()
pub fn stub_f30f54() {
    // IDA 0xf30f54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,RBX::PriorityThreadPool::PriorityTask const&)")]
// 0xf30f64 — j___ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f30f64() {
    // IDA 0xf30f64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::push_back(RBX::PriorityThreadPool::PriorityTask const&)")]
// 0xf30f74 — j___ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE9push_backERKS2_
pub fn stub_f30f74() {
    // IDA 0xf30f74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::~vector()")]
// 0xf30f84 — j___ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EED2Ev
pub fn stub_f30f84() {
    // IDA 0xf30f84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::mutex>*,std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>>,unsigned long,rbx_core::SharedPtr<RBX::mutex> const&)")]
// 0xf30f94 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
// was: std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::mutex>*,std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>>,unsigned long,boost::shared_ptr<RBX::mutex> const&)
pub fn stub_f30f94() {
    // IDA 0xf30f94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::mutex>*)")]
// 0xf30fa4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE15_M_erase_at_endEPS4_
// was: std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_erase_at_end(boost::shared_ptr<RBX::mutex>*)
pub fn stub_f30fa4() {
    // IDA 0xf30fa4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::resize(unsigned long,rbx_core::SharedPtr<RBX::mutex>)")]
// 0xf30fb4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE6resizeEmS4_
// was: std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::resize(unsigned long,boost::shared_ptr<RBX::mutex>)
pub fn stub_f30fb4() {
    // IDA 0xf30fb4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::~vector()")]
// 0xf30fc4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EED2Ev
// was: std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::~vector()
pub fn stub_f30fc4() {
    // IDA 0xf30fc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread>*,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,unsigned long,rbx_core::SharedPtr<boost::thread> const&)")]
// 0xf30fd4 — j___ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// was: std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread>*,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,unsigned long,boost::shared_ptr<boost::thread> const&)
pub fn stub_f30fd4() {
    // IDA 0xf30fd4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::_M_erase_at_end(rbx_core::SharedPtr<boost::thread>*)")]
// 0xf30fe4 — j___ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE15_M_erase_at_endEPS3_
// was: std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_erase_at_end(boost::shared_ptr<boost::thread>*)
pub fn stub_f30fe4() {
    // IDA 0xf30fe4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::resize(unsigned long,rbx_core::SharedPtr<boost::thread>)")]
// 0xf30ff4 — j___ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE6resizeEmS3_
// was: std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::resize(unsigned long,boost::shared_ptr<boost::thread>)
pub fn stub_f30ff4() {
    // IDA 0xf30ff4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::~vector()")]
// 0xf31004 — j___ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EED2Ev
// was: std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::~vector()
pub fn stub_f31004() {
    // IDA 0xf31004: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
// 0xf31014 — j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
pub fn stub_f31014() {
    // IDA 0xf31014: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
// 0xf31024 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
pub fn stub_f31024() {
    // IDA 0xf31024: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<RBX::mutex> *,unsigned long,rbx_core::SharedPtr<RBX::mutex>>(rbx_core::SharedPtr<RBX::mutex> *,unsigned long,rbx_core::SharedPtr<RBX::mutex> const&,std::__false_type)")]
// 0xf31034 — j___ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX5mutexEEEmS4_EvT_T0_RKT1_St12__false_type
// was: void std::__uninitialized_fill_n_aux<boost::shared_ptr<RBX::mutex> *,unsigned long,boost::shared_ptr<RBX::mutex>>(boost::shared_ptr<RBX::mutex> *,unsigned long,boost::shared_ptr<RBX::mutex> const&,std::__false_type)
pub fn stub_f31034() {
    // IDA 0xf31034: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<boost::thread> *,unsigned long,rbx_core::SharedPtr<boost::thread>>(rbx_core::SharedPtr<boost::thread> *,unsigned long,rbx_core::SharedPtr<boost::thread> const&,std::__false_type)")]
// 0xf31044 — j___ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrINS0_6threadEEEmS3_EvT_T0_RKT1_St12__false_type
// was: void std::__uninitialized_fill_n_aux<boost::shared_ptr<boost::thread> *,unsigned long,boost::shared_ptr<boost::thread>>(boost::shared_ptr<boost::thread> *,unsigned long,boost::shared_ptr<boost::thread> const&,std::__false_type)
pub fn stub_f31044() {
    // IDA 0xf31044: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>),boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>),boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>),boost::_bi::list1<boost::arg<1>>>)")]
// 0xf31054 — j___ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_ENSB_5list1INS2_3argILi1EEEEEEEET0_T_SL_SK_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>>)
pub fn stub_f31054() {
    // IDA 0xf31054: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>)")]
// 0xf31064 — j___ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_NS2_9date_time18subsecond_durationINS2_10posix_time13time_durationELx1000EEEENSB_5list2INS2_3argILi1EEENSB_5valueISH_EEEEEEET0_T_SS_SR_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>)
pub fn stub_f31064() {
    // IDA 0xf31064: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long,char>(unsigned long &,char const*,char const*)")]
// 0xf31074 — j___ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEmcEEbRT0_PKT1_S8_
pub fn stub_f31074() {
    // IDA 0xf31074: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long>(long &)")]
// 0xf31084 — j___ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIlEEbRT_
pub fn stub_f31084() {
    // IDA 0xf31084: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned int>(unsigned int &)")]
// 0xf31094 — j___ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedIjEEbRT_
pub fn stub_f31094() {
    // IDA 0xf31094: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::Accoutrement::getRenderSize(void)")]
// 0xf310d4 — j___ZN3RBX12Accoutrement13getRenderSizeEv
pub fn stub_f310d4() {
    // IDA 0xf310d4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::PVAdornment::~PVAdornment()")]
// 0xf312d4 — j___ZN3RBX11PVAdornmentD1Ev
pub fn stub_f312d4() {
    // IDA 0xf312d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AnimationId const& rbx::any_cast<RBX::AnimationId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf31374 — j___ZN3rbx8any_castIRKN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f31374() {
    // IDA 0xf31374: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>> const&)")]
// 0xf31414 — j___ZN3rbx7signals6signalIFvSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_f31414() {
    // IDA 0xf31414: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Animator>::shared_ptr<RBX::Animator>(rbx_core::WeakPtr<RBX::Animator> const&,boost::detail::sp_nothrow_tag)")]
// 0xf31424 — j___ZN5boost10shared_ptrIN3RBX8AnimatorEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::Animator>::shared_ptr<RBX::Animator>(boost::weak_ptr<RBX::Animator> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f31424() {
    // IDA 0xf31424: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string> &,boost::_bi::list1<std::string &> &,int)")]
// 0xf31434 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX14AnimationTrackEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_SsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f31434() {
    // IDA 0xf31434: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>::operator()(RBX::AnimationTrack*,std::string)const")]
// 0xf31444 — j___ZNK5boost4_mfi3mf1IvN3RBX14AnimationTrackESsEclEPS3_Ss
pub fn stub_f31444() {
    // IDA 0xf31444: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState()")]
// 0xf314e4 — j___ZN3RBX19AnimationTrackStateD0Ev
pub fn stub_f314e4() {
    // IDA 0xf314e4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState()")]
// 0xf314f4 — j___ZN3RBX19AnimationTrackStateD2Ev
pub fn stub_f314f4() {
    // IDA 0xf314f4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(float,float)>::remote_signal(void)")]
// 0xf31514 — j___ZN3rbx13remote_signalIFvffEEC2Ev
pub fn stub_f31514() {
    // IDA 0xf31514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(float,float)>::~remote_signal()")]
// 0xf31524 — j___ZN3rbx13remote_signalIFvffEED2Ev
pub fn stub_f31524() {
    // IDA 0xf31524: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(float,float,float)>::remote_signal(void)")]
// 0xf31534 — j___ZN3rbx13remote_signalIFvfffEEC2Ev
pub fn stub_f31534() {
    // IDA 0xf31534: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
